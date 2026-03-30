use napi::bindgen_prelude::*;
use swc_core::{
    common::{FileName, SourceMap, errors::Handler, sync::Lrc},
    ecma::{
        ast::{EsVersion, Program},
        codegen::{Emitter, text_writer::JsWriter},
        parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer},
    },
};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::LazyLock;

use crate::host::CONFIG_CACHE;
use crate::host::MacroExpander;
use crate::napi_types::{
    ExpandOptions, ExpandResult, GeneratedRegionResult, MacroDiagnostic, MappingSegmentResult,
    SourceMappingResult, TransformResult,
};
use crate::ts_syn::abi::ir::type_registry::TypeRegistry;
use crate::ts_syn::{Diagnostic, DiagnosticLevel};

// ============================================================================
// Type Registry Cache
// ============================================================================
// Caches deserialized TypeRegistry by hash of the JSON string to avoid
// re-parsing the registry on every expand_sync call.

pub(crate) static REGISTRY_CACHE: LazyLock<dashmap::DashMap<u64, TypeRegistry>> =
    LazyLock::new(dashmap::DashMap::new);

pub(crate) fn get_or_parse_registry(json: &str) -> Option<TypeRegistry> {
    let mut hasher = DefaultHasher::new();
    json.hash(&mut hasher);
    let key = hasher.finish();

    if let Some(entry) = REGISTRY_CACHE.get(&key) {
        return Some(entry.clone());
    }

    match serde_json::from_str::<TypeRegistry>(json) {
        Ok(registry) => {
            REGISTRY_CACHE.insert(key, registry.clone());
            Some(registry)
        }
        Err(_) => None,
    }
}

// ============================================================================
// Inner Logic (Optimized)
// ============================================================================

/// Check if source code contains `@derive(` as a standalone JSDoc directive.
///
/// Only matches `@derive(` when it appears at the start of a JSDoc line (after
/// stripping `/**`, `*/`, `*`, and whitespace). Skips `@derive` embedded in prose
/// (e.g., `"result from @derive(Deserialize)"`) and inside fenced code blocks.
pub(crate) fn has_macro_annotations(source: &str) -> bool {
    if !source.contains("@derive") {
        return false;
    }
    let mut in_code_block = false;
    for line in source.lines() {
        // Strip JSDoc comment syntax: /**, */, leading *, and whitespace
        let trimmed = line
            .trim()
            .trim_start_matches('/')
            .trim_start_matches('*')
            .trim_end_matches('/')
            .trim_end_matches('*')
            .trim();
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block {
            continue;
        }
        // A line must START with @derive( to be a real directive.
        // This rejects prose like "result from @derive(Deserialize)".
        if trimmed.starts_with("@derive(") {
            return true;
        }
    }
    false
}

/// Core macro expansion logic, decoupled from NAPI Env to allow threading.
///
/// This function contains the actual expansion implementation and is called
/// from a separate thread with a large stack size to prevent stack overflow.
///
/// # Arguments
///
/// * `code` - The TypeScript source code to expand
/// * `filepath` - The file path (used for TSX detection and error reporting)
/// * `options` - Optional expansion configuration
///
/// # Returns
///
/// An [`ExpandResult`] containing the expanded code and metadata.
///
/// # Errors
///
/// Returns an error if:
/// - The macro host fails to initialize
/// - Macro expansion fails internally
///
/// # Algorithm
///
/// 1. **Early bailout**: If code doesn't contain `@derive`, return unchanged
/// 2. **Parse**: Convert code to SWC AST
/// 3. **Expand**: Run all registered macros on decorated classes
/// 4. **Collect**: Gather diagnostics and source mapping
/// 5. **Post-process**: Inject type declarations for generated methods
pub(crate) fn expand_inner(
    code: &str,
    filepath: &str,
    options: Option<ExpandOptions>,
) -> Result<ExpandResult> {
    // Early bailout: Skip files without @derive decorator.
    // This optimization avoids expensive parsing for files that don't use macros
    // and prevents issues with Svelte runes ($state, $derived, etc.) that use
    // similar syntax but aren't macroforge decorators.
    // Uses has_macro_annotations to skip @derive inside fenced code blocks in docs.
    if !has_macro_annotations(code) {
        return Ok(ExpandResult::unchanged(code));
    }

    // Create a new macro host for this thread.
    // Each thread needs its own MacroExpander because the expansion process
    // is stateful and cannot be safely shared across threads.
    let mut macro_host = MacroExpander::new().map_err(|err| {
        Error::new(
            Status::GenericFailure,
            format!("Failed to initialize macro host: {err:?}"),
        )
    })?;

    // Apply options if provided
    if let Some(ref opts) = options {
        if let Some(keep) = opts.keep_decorators {
            macro_host.set_keep_decorators(keep);
        }
        if let Some(ref modules) = opts.external_decorator_modules {
            macro_host.set_external_decorator_modules(modules.clone());
        }
        // Deserialize and attach the type registry for type awareness
        if let Some(ref json) = opts.type_registry_json {
            let registry = get_or_parse_registry(json);
            macro_host.set_type_registry(registry);
        }
    }

    // Set up foreign types and config imports on the registry (from config file).
    // The registry's source imports will be built from the AST later in prepare_expansion_context.
    let config_path = options.as_ref().and_then(|o| o.config_path.as_ref());
    if let Some(path) = config_path
        && let Some(config) = CONFIG_CACHE.get(path)
    {
        crate::host::import_registry::set_foreign_types(config.foreign_types.clone());
        crate::host::import_registry::with_registry_mut(|r| {
            r.config_imports = config
                .config_imports
                .iter()
                .map(|(name, info)| (name.clone(), info.source.clone()))
                .collect();
        });
    }

    // Parse the code into an AST.
    // On parse errors, we return a graceful "no-op" result instead of failing,
    // because parse errors can happen frequently during typing in an IDE.
    let (program, _) = match parse_program(code, filepath) {
        Ok(p) => p,
        Err(e) => {
            let error_msg = e.to_string();

            // Clean up registry before returning
            crate::host::import_registry::clear_registry();
            crate::host::import_registry::clear_foreign_types();

            return Ok(ExpandResult {
                code: code.to_string(),
                types: None,
                metadata: None,
                diagnostics: vec![MacroDiagnostic {
                    level: "info".to_string(),
                    message: format!("Macro expansion skipped due to syntax error: {}", error_msg),
                    start: None,
                    end: None,
                }],
                source_mapping: None,
            });
        }
    };

    // Run macro expansion on the parsed AST.
    // expand() internally calls prepare_expansion_context which builds the full registry from the AST.
    let expansion_result = macro_host.expand(code, &program, filepath);

    // Single cleanup — replaces 7 separate clear_* calls
    crate::host::import_registry::clear_registry();

    // Now propagate any error
    let expansion = expansion_result.map_err(|err| {
        Error::new(
            Status::GenericFailure,
            format!("Macro expansion failed: {err:?}"),
        )
    })?;

    // Convert internal diagnostics to NAPI-compatible format
    let diagnostics = expansion
        .diagnostics
        .into_iter()
        .map(|d| MacroDiagnostic {
            level: format!("{:?}", d.level).to_lowercase(),
            message: d.message,
            start: d.span.map(|s| s.start),
            end: d.span.map(|s| s.end),
        })
        .collect();

    // Convert internal source mapping to NAPI-compatible format
    let source_mapping = expansion.source_mapping.map(|mapping| SourceMappingResult {
        segments: mapping
            .segments
            .into_iter()
            .map(|seg| MappingSegmentResult {
                original_start: seg.original_start,
                original_end: seg.original_end,
                expanded_start: seg.expanded_start,
                expanded_end: seg.expanded_end,
            })
            .collect(),
        generated_regions: mapping
            .generated_regions
            .into_iter()
            .map(|region| GeneratedRegionResult {
                start: region.start,
                end: region.end,
                source_macro: region.source_macro,
            })
            .collect(),
    });

    // Post-process type declarations.
    // Heuristic fix: If the expanded code contains toJSON() but the type
    // declarations don't, inject the type signature. This ensures IDEs
    // provide proper type information for serialized objects.
    let mut types_output = expansion.type_output;
    if let Some(types) = &mut types_output
        && expansion.code.contains("toJSON(")
        && !types.contains("toJSON(")
    {
        // Find the last closing brace and insert before it.
        // This is a heuristic that works for simple cases.
        if let Some(insert_at) = types.rfind('}') {
            types.insert_str(insert_at, "  toJSON(): Record<string, unknown>;\n");
        }
    }

    Ok(ExpandResult {
        code: expansion.code,
        types: types_output,
        // Only include metadata if there were classes processed
        metadata: if expansion.classes.is_empty() {
            None
        } else {
            serde_json::to_string(&expansion.classes).ok()
        },
        diagnostics,
        source_mapping,
    })
}

/// Core transform logic, decoupled from NAPI Env to allow threading.
///
/// Similar to [`expand_inner`] but returns a [`TransformResult`] and fails
/// on any error-level diagnostics.
///
/// # Arguments
///
/// * `code` - The TypeScript source code to transform
/// * `filepath` - The file path (used for TSX detection)
///
/// # Returns
///
/// A [`TransformResult`] containing the transformed code.
///
/// # Errors
///
/// Returns an error if:
/// - The macro host fails to initialize
/// - Parsing fails
/// - Expansion fails
/// - Any error-level diagnostic is emitted
pub(crate) fn transform_inner(code: &str, filepath: &str) -> Result<TransformResult> {
    let macro_host = MacroExpander::new().map_err(|err| {
        Error::new(
            Status::GenericFailure,
            format!("Failed to init host: {err:?}"),
        )
    })?;

    let (program, cm) = parse_program(code, filepath)?;

    let expansion = macro_host
        .expand(code, &program, filepath)
        .map_err(|err| Error::new(Status::GenericFailure, format!("Expansion failed: {err:?}")))?;

    // Unlike expand_inner, transform_inner treats errors as fatal
    handle_macro_diagnostics(&expansion.diagnostics, filepath)?;

    // Optimization: Only re-emit if we didn't change anything.
    // If expansion.changed is true, we already have the string from the expander.
    // Otherwise, emit the original AST to string (no changes made).
    let generated = if expansion.changed {
        expansion.code
    } else {
        emit_program(&program, &cm)?
    };

    let metadata = if expansion.classes.is_empty() {
        None
    } else {
        serde_json::to_string(&expansion.classes).ok()
    };

    Ok(TransformResult {
        code: generated,
        map: None, // Source mapping handled separately via SourceMappingResult
        types: expansion.type_output,
        metadata,
    })
}

/// Parses TypeScript source code into an SWC AST.
///
/// # Arguments
///
/// * `code` - The TypeScript source code
/// * `filepath` - The file path (used to determine TSX mode from extension)
///
/// # Returns
///
/// A tuple of `(Program, SourceMap)` on success.
///
/// # Errors
///
/// Returns an error if the code contains syntax errors.
///
/// # Configuration
///
/// - TSX mode is enabled for `.tsx` files
/// - Decorators are always enabled
/// - Uses latest ES version
/// - `no_early_errors` is enabled for better error recovery
pub(crate) fn parse_program(code: &str, filepath: &str) -> Result<(Program, Lrc<SourceMap>)> {
    let cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
    let fm = cm.new_source_file(
        FileName::Custom(filepath.to_string()).into(),
        code.to_string(),
    );
    // Create a handler that captures errors to a buffer (we don't use its output directly)
    let handler =
        Handler::with_emitter_writer(Box::new(std::io::Cursor::new(Vec::new())), Some(cm.clone()));

    // Configure the lexer for TypeScript with decorator support
    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: filepath.ends_with(".tsx"), // Enable TSX for .tsx files
            decorators: true,                // Required for @derive decorators
            dts: false,                      // Not parsing .d.ts files
            no_early_errors: true,           // Better error recovery during typing
            ..Default::default()
        }),
        EsVersion::latest(),
        StringInput::from(&*fm),
        None, // No comments collection
    );

    let mut parser = Parser::new_from(lexer);
    match parser.parse_program() {
        Ok(program) => Ok((program, cm)),
        Err(error) => {
            // Format and emit the error for debugging purposes
            let msg = format!("Failed to parse TypeScript: {:?}", error);
            error.into_diagnostic(&handler).emit();
            Err(Error::new(Status::GenericFailure, msg))
        }
    }
}

/// Emits an SWC AST back to JavaScript/TypeScript source code.
///
/// # Arguments
///
/// * `program` - The AST to emit
/// * `cm` - The source map (used for line/column tracking)
///
/// # Returns
///
/// The generated source code as a string.
///
/// # Errors
///
/// Returns an error if code generation fails (rare).
pub(crate) fn emit_program(program: &Program, cm: &Lrc<SourceMap>) -> Result<String> {
    let mut buf = vec![];
    let mut emitter = Emitter {
        cfg: swc_core::ecma::codegen::Config::default(),
        cm: cm.clone(),
        comments: None,
        wr: Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None)),
    };
    emitter
        .emit_program(program)
        .map_err(|e| Error::new(Status::GenericFailure, format!("{:?}", e)))?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}

/// Checks diagnostics for errors and returns the first error as a Result.
///
/// This is used by [`transform_inner`] which treats errors as fatal,
/// unlike [`expand_inner`] which allows non-fatal errors in diagnostics.
///
/// # Arguments
///
/// * `diags` - The diagnostics to check
/// * `file` - The file path for error location reporting
///
/// # Returns
///
/// `Ok(())` if no errors, `Err` with the first error message otherwise.
pub(crate) fn handle_macro_diagnostics(diags: &[Diagnostic], file: &str) -> Result<()> {
    for diag in diags {
        if matches!(diag.level, DiagnosticLevel::Error) {
            // Format error location for helpful error messages
            let loc = diag
                .span
                .map(|s| format!("{}:{}-{}", file, s.start, s.end))
                .unwrap_or_else(|| file.to_string());
            return Err(Error::new(
                Status::GenericFailure,
                format!("Macro error at {}: {}", loc, diag.message),
            ));
        }
    }
    Ok(())
}
