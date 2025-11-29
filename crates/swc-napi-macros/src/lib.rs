use napi::bindgen_prelude::*;
use napi_derive::napi;
use swc_core::{
    common::{FileName, GLOBALS, Globals, SourceMap, errors::Handler, sync::Lrc},
    ecma::{
        ast::{EsVersion, Program},
        codegen::{Emitter, text_writer::JsWriter},
        parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer},
    },
};
use ts_macro_abi::{Diagnostic, DiagnosticLevel};
use ts_macro_host::derived;

mod builtin;
mod macro_host;
use crate::macro_host::MacroHostIntegration;

#[napi(object)]
pub struct TransformResult {
    pub code: String,
    pub map: Option<String>,
    pub types: Option<String>,
    pub metadata: Option<String>,
}

#[napi(object)]
pub struct MacroDiagnostic {
    pub level: String,
    pub message: String,
    pub start: Option<u32>,
    pub end: Option<u32>,
}

#[napi(object)]
pub struct MappingSegmentResult {
    pub original_start: u32,
    pub original_end: u32,
    pub expanded_start: u32,
    pub expanded_end: u32,
}

#[napi(object)]
pub struct GeneratedRegionResult {
    pub start: u32,
    pub end: u32,
    pub source_macro: String,
}

#[napi(object)]
pub struct SourceMappingResult {
    pub segments: Vec<MappingSegmentResult>,
    pub generated_regions: Vec<GeneratedRegionResult>,
}

#[napi(object)]
pub struct ExpandResult {
    pub code: String,
    pub types: Option<String>,
    pub metadata: Option<String>,
    pub diagnostics: Vec<MacroDiagnostic>,
    pub source_mapping: Option<SourceMappingResult>,
}

#[napi(
    js_name = "Derive",
    ts_return_type = "ClassDecorator",
    ts_args_type = "...features: Array<string | ClassDecorator | PropertyDecorator | ((...args:\n  any[]) => unknown) | Record<string, unknown>>"
)]
pub fn derive_decorator() {}

/// Transform TypeScript code to JavaScript with macro expansion
#[napi]
pub fn transform_sync(code: String, filepath: String) -> Result<TransformResult> {
    // Initialize SWC globals
    let globals = Globals::default();
    GLOBALS.set(&globals, || {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            transform_inner(&code, &filepath)
        }))
        .map_err(|_| Error::new(Status::GenericFailure, "Macro transformation panicked"))?
    })
}

/// Expand macros in TypeScript code and return the transformed TS (types) and diagnostics
#[napi]
pub fn expand_sync(code: String, filepath: String) -> Result<ExpandResult> {
    let globals = Globals::default();
    GLOBALS.set(&globals, || {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            expand_inner(&code, &filepath)
        }))
        .map_err(|_| Error::new(Status::GenericFailure, "Macro expansion panicked"))?
    })
}

fn expand_inner(code: &str, filepath: &str) -> Result<ExpandResult> {
    let macro_host = MacroHostIntegration::new().map_err(|err| {
        Error::new(
            Status::GenericFailure,
            format!("Failed to initialize macro host: {err:?}"),
        )
    })?;

    let (program, _) = parse_program(code, filepath)?;

    let expansion = macro_host.expand(code, &program, filepath).map_err(|err| {
        Error::new(
            Status::GenericFailure,
            format!("Macro expansion failed: {err:?}"),
        )
    })?;

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

    // Convert SourceMapping to NAPI-compatible SourceMappingResult
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

    let mut types_output = expansion.type_output;
    if let Some(types) = &mut types_output
        && expansion.code.contains("toJSON(")
        && !types.contains("toJSON(")
        && let Some(insert_at) = types.rfind('}')
    {
        types.insert_str(insert_at, "  toJSON(): Record<string, unknown>;\n");
    }

    Ok(ExpandResult {
        code: expansion.code,
        types: types_output,
        metadata: if expansion.classes.is_empty() {
            None
        } else {
            serde_json::to_string(&expansion.classes).ok()
        },
        diagnostics,
        source_mapping,
    })
}

fn transform_inner(code: &str, filepath: &str) -> Result<TransformResult> {
    let macro_host = MacroHostIntegration::new().map_err(|err| {
        Error::new(
            Status::GenericFailure,
            format!("Failed to initialize macro host: {err:?}"),
        )
    })?;

    let (mut program, mut cm) = parse_program(code, filepath)?;

    let expansion = macro_host.expand(code, &program, filepath).map_err(|err| {
        Error::new(
            Status::GenericFailure,
            format!("Macro expansion failed: {err:?}"),
        )
    })?;

    handle_macro_diagnostics(&expansion.diagnostics, filepath)?;

    if expansion.changed {
        let (new_program, new_cm) = parse_program(&expansion.code, filepath)?;
        program = new_program;
        cm = new_cm;
    }

    let generated = emit_program(&program, &cm)?;
    let metadata = if expansion.classes.is_empty() {
        None
    } else {
        serde_json::to_string(&expansion.classes).ok()
    };

    Ok(TransformResult {
        code: generated,
        map: None,
        types: expansion.type_output,
        metadata,
    })
}

fn parse_program(code: &str, filepath: &str) -> Result<(Program, Lrc<SourceMap>)> {
    let cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
    let fm = cm.new_source_file(
        FileName::Custom(filepath.to_string()).into(),
        code.to_string(),
    );

    // Use a memory buffer instead of stderr to avoid polluting the LSP stream
    let handler =
        Handler::with_emitter_writer(Box::new(std::io::Cursor::new(Vec::new())), Some(cm.clone()));

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: filepath.ends_with(".tsx"),
            decorators: true,
            dts: false,
            no_early_errors: true,
            ..Default::default()
        }),
        EsVersion::latest(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);
    match parser.parse_program() {
        Ok(program) => Ok((program, cm)),
        Err(error) => {
            let message = format!("Failed to parse TypeScript: {error:?}");
            error.into_diagnostic(&handler).emit();
            Err(Error::new(Status::GenericFailure, message))
        }
    }
}

fn emit_program(program: &Program, cm: &Lrc<SourceMap>) -> Result<String> {
    let mut buf = vec![];
    let mut emitter = Emitter {
        cfg: swc_core::ecma::codegen::Config::default(),
        cm: cm.clone(),
        comments: None,
        wr: Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None)),
    };

    emitter.emit_program(program).map_err(|error| {
        Error::new(
            Status::GenericFailure,
            format!("Failed to generate JavaScript: {error:?}"),
        )
    })?;

    Ok(String::from_utf8_lossy(&buf).to_string())
}

fn handle_macro_diagnostics(diags: &[Diagnostic], file: &str) -> Result<()> {
    for diag in diags {
        match diag.level {
            DiagnosticLevel::Error => {
                let location = diag
                    .span
                    .map(|span| format!("{file}:{}-{}", span.start, span.end))
                    .unwrap_or_else(|| file.to_string());
                return Err(Error::new(
                    Status::GenericFailure,
                    format!("Macro error at {location}: {}", diag.message),
                ));
            }
            DiagnosticLevel::Warning => {
                // eprintln!("[ts-macros] warning: {}", diag.message);
            }
            DiagnosticLevel::Info => {
                // eprintln!("[ts-macros] info: {}", diag.message);
            }
        }
    }
    Ok(())
}

// ============================================================================
// Macro Package Manifest API
// ============================================================================

/// Entry for a single macro in the manifest
#[napi(object)]
pub struct MacroManifestEntry {
    /// The macro name (e.g., "Debug", "JSON")
    pub name: String,
    /// The macro kind ("derive", "attribute", or "call")
    pub kind: String,
    /// Description of what the macro does
    pub description: String,
    /// The Rust package that provides this macro
    pub package: String,
}

/// Decorator metadata for TypeScript type stubs
#[napi(object)]
pub struct DecoratorManifestEntry {
    /// Module this decorator is exported from
    pub module: String,
    /// Export name
    pub export: String,
    /// Decorator kind ("class", "property", "method", etc.)
    pub kind: String,
    /// Documentation
    pub docs: String,
}

/// Complete manifest for this macro package
#[napi(object)]
pub struct MacroManifest {
    /// Manifest format version
    pub version: u32,
    /// List of macros provided by this package
    pub macros: Vec<MacroManifestEntry>,
    /// List of decorator exports
    pub decorators: Vec<DecoratorManifestEntry>,
}

/// Get the manifest of all macros available in this package
/// This is used by the TypeScript plugin to auto-discover macro packages
#[napi(js_name = "__tsMacrosGetManifest")]
pub fn get_macro_manifest() -> MacroManifest {
    let manifest = derived::get_manifest();

    MacroManifest {
        version: manifest.version,
        macros: manifest
            .macros
            .into_iter()
            .map(|m| MacroManifestEntry {
                name: m.name.to_string(),
                kind: format!("{:?}", m.kind).to_lowercase(),
                description: m.description.to_string(),
                package: m.package.to_string(),
            })
            .collect(),
        decorators: manifest
            .decorators
            .into_iter()
            .map(|d| DecoratorManifestEntry {
                module: d.module.to_string(),
                export: d.export.to_string(),
                kind: format!("{:?}", d.kind).to_lowercase(),
                docs: d.docs.to_string(),
            })
            .collect(),
    }
}

/// Check if this package exports macros (quick probe)
#[napi(js_name = "__tsMacrosIsMacroPackage")]
pub fn is_macro_package() -> bool {
    !derived::macro_names().is_empty()
}

/// Get the names of all macros in this package
#[napi(js_name = "__tsMacrosGetMacroNames")]
pub fn get_macro_names() -> Vec<String> {
    derived::macro_names()
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

/// Debug: Get all registered module paths from inventory
#[napi(js_name = "__tsMacrosDebugGetModules")]
pub fn debug_get_modules() -> Vec<String> {
    let modules = ts_macro_host::derived::modules();
    modules.into_iter().map(|s| s.to_string()).collect()
}

/// Debug: Try to look up a macro by module and name
#[napi(js_name = "__tsMacrosDebugLookup")]
pub fn debug_lookup(module: String, name: String) -> String {
    // Create a fresh host to test lookup
    match MacroHostIntegration::new() {
        Ok(host) => {
            let registry = host.dispatcher.registry();
            match registry.lookup(&module, &name) {
                Ok(_) => format!("Found: ({}, {})", module, name),
                Err(_e) => {
                    // Try fallback
                    match registry.lookup_with_fallback(&module, &name) {
                        Ok(_) => format!("Found via fallback: ({}, {})", module, name),
                        Err(_) => {
                            // List all macros in registry
                            let all = registry.all_macros();
                            let keys: Vec<String> = all
                                .iter()
                                .map(|(k, _)| format!("({}, {})", k.module, k.name))
                                .collect();
                            format!("Not found: ({}, {}). Available: {:?}", module, name, keys)
                        }
                    }
                }
            }
        }
        Err(e) => format!("Failed to create host: {}", e),
    }
}

/// Debug: List all descriptors from inventory
#[napi(js_name = "__tsMacrosDebugDescriptors")]
pub fn debug_descriptors() -> Vec<String> {
    use ts_macro_host::derived::DerivedMacroRegistration;

    inventory::iter::<DerivedMacroRegistration>
        .into_iter()
        .map(|entry| {
            let d = entry.descriptor;
            format!(
                "name={}, module={}, package={}",
                d.name, d.module, d.package
            )
        })
        .collect()
}
