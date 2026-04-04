use anyhow::{Result, anyhow};
#[cfg(feature = "swc")]
use swc_core::{
    common::{FileName, SourceMap, errors::Handler, sync::Lrc},
    ecma::{
        ast::{EsVersion, Program},
        codegen::{Emitter, text_writer::JsWriter},
        parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer},
    },
};

#[cfg(feature = "oxc")]
use oxc_allocator::Allocator;
#[cfg(feature = "oxc")]
use oxc_codegen::Codegen as OxcCodegen;
#[cfg(feature = "oxc")]
use oxc_parser::Parser as OxcParser;
#[cfg(feature = "oxc")]
use oxc_span::SourceType;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::LazyLock;

use crate::api_types::{
    ExpandOptions, ExpandResult, GeneratedRegionResult, MacroDiagnostic, MappingSegmentResult,
    SourceMappingResult, TransformResult,
};
use crate::host::CONFIG_CACHE;
use crate::host::MacroExpander;
#[cfg(feature = "oxc")]
use crate::host::expand::LoweredItems;
use crate::ts_syn::abi::ir::type_registry::TypeRegistry;
#[cfg(feature = "swc")]
use crate::ts_syn::{Diagnostic, DiagnosticLevel};

// ============================================================================
// MacroExpander Cache
// ============================================================================

/// Cached config discovery result to avoid filesystem walks on every expand call.
/// On native, `MacroExpander::new()` calls `MacroConfig::find_with_root()` which
/// walks up directories looking for config files. Caching the result here means
/// the filesystem is only touched once.
#[cfg(not(target_arch = "wasm32"))]
static DISCOVERED_CONFIG: LazyLock<
    std::sync::Mutex<Option<(crate::host::config::MacroConfig, std::path::PathBuf)>>,
> = LazyLock::new(|| std::sync::Mutex::new(None));

/// Create a MacroExpander using cached config discovery.
fn create_expander() -> Result<MacroExpander> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::host::MacroConfig;
        let mut guard = DISCOVERED_CONFIG
            .lock()
            .map_err(|e| anyhow!("Lock poisoned: {e}"))?;
        let (config, root) = if let Some((c, r)) = guard.as_ref() {
            (c.clone(), r.clone())
        } else {
            let discovered = MacroConfig::find_with_root()
                .map_err(|e| anyhow!("Config discovery failed: {e}"))?
                .unwrap_or_else(|| {
                    (
                        MacroConfig::default(),
                        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
                    )
                });
            *guard = Some(discovered.clone());
            discovered
        };
        MacroExpander::with_config(config, root)
            .map_err(|err| anyhow!("Failed to initialize macro host: {err:?}"))
    }
    #[cfg(target_arch = "wasm32")]
    {
        MacroExpander::new().map_err(|err| anyhow!("Failed to initialize macro host: {err:?}"))
    }
}

// ============================================================================
// Type Registry Cache
// ============================================================================

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
// Backend Abstraction
// ============================================================================

pub(crate) trait CompilerBackend {
    fn expand(
        &self,
        code: &str,
        filepath: &str,
        options: &Option<ExpandOptions>,
    ) -> Result<ExpandResult>;
    fn transform(&self, code: &str, filepath: &str) -> Result<TransformResult>;
}

#[cfg(feature = "swc")]
pub(crate) struct SwcBackend;

#[cfg(feature = "swc")]
impl CompilerBackend for SwcBackend {
    fn expand(
        &self,
        code: &str,
        filepath: &str,
        options: &Option<ExpandOptions>,
    ) -> Result<ExpandResult> {
        let mut macro_host = create_expander()?;

        apply_options(&mut macro_host, options);

        let (program, _) = match parse_program(code, filepath) {
            Ok(p) => p,
            Err(e) => return Ok(make_syntax_error_result(code, &e.to_string())),
        };

        let expansion_result = macro_host.expand(code, &program, filepath);
        crate::host::import_registry::clear_registry();

        let expansion =
            expansion_result.map_err(|err| anyhow!("Macro expansion failed: {err:?}"))?;

        Ok(finalize_expansion(expansion))
    }

    fn transform(&self, code: &str, filepath: &str) -> Result<TransformResult> {
        let macro_host = create_expander()?;
        let (program, cm) = parse_program(code, filepath)?;

        let expansion = macro_host
            .expand(code, &program, filepath)
            .map_err(|err| anyhow!("Expansion failed: {err:?}"))?;

        handle_macro_diagnostics(&expansion.diagnostics, filepath).map_err(|e| anyhow!(e))?;

        let generated = if expansion.changed {
            expansion.code
        } else {
            emit_program(&program, &cm)?
        };

        Ok(TransformResult {
            code: generated,
            map: None,
            types: expansion.type_output,
            metadata: serialize_metadata(&expansion.classes),
        })
    }
}

#[cfg(feature = "oxc")]
pub(crate) struct OxcBackend;

#[cfg(feature = "oxc")]
impl CompilerBackend for OxcBackend {
    fn expand(
        &self,
        code: &str,
        filepath: &str,
        options: &Option<ExpandOptions>,
    ) -> Result<ExpandResult> {
        let allocator = Allocator::default();
        let source_type = SourceType::ts().with_jsx(filepath.ends_with(".tsx"));

        let ret = OxcParser::new(&allocator, code, source_type).parse();

        if !ret.errors.is_empty() {
            return Err(anyhow!("Oxc parse errors: {:?}", ret.errors));
        }

        #[cfg(feature = "swc")]
        {
            SwcBackend.expand(code, filepath, options)
        }
        #[cfg(not(feature = "swc"))]
        {
            let mut macro_host = create_expander()?;
            apply_options(&mut macro_host, options);

            let classes = crate::ts_syn::lower_classes_oxc(&ret.program, code, None)?;
            let interfaces = crate::ts_syn::lower_interfaces_oxc(&ret.program, code, None)?;
            let enums = crate::ts_syn::lower_enums_oxc(&ret.program, code, None)?;
            let type_aliases = crate::ts_syn::lower_type_aliases_oxc(&ret.program, code, None)?;
            let imports = crate::ts_syn::ImportRegistry::from_oxc_program(&ret.program, code);

            let items = LoweredItems {
                classes,
                interfaces,
                enums,
                type_aliases,
                imports,
            };

            if items.is_empty() {
                return Ok(ExpandResult::unchanged(code));
            }

            let items_clone = items.clone();
            let (mut collector, mut diagnostics) =
                macro_host.collect_macro_patches_oxc(items, filepath, code);

            let expansion = macro_host
                .apply_and_finalize_expansion(code, &mut collector, &mut diagnostics, items_clone)
                .map_err(anyhow::Error::from)?;

            let mut result = ExpandResult {
                code: expansion.code,
                types: expansion.type_output,
                metadata: serialize_metadata(&expansion.classes),
                diagnostics: expansion
                    .diagnostics
                    .into_iter()
                    .map(|d| MacroDiagnostic {
                        level: format!("{:?}", d.level).to_lowercase(),
                        message: d.message,
                        start: d.span.map(|s| s.start),
                        end: d.span.map(|s| s.end),
                    })
                    .collect(),
                source_mapping: expansion.source_mapping.map(|mapping| SourceMappingResult {
                    segments: mapping
                        .segments
                        .into_iter()
                        .map(|segment| MappingSegmentResult {
                            original_start: segment.original_start,
                            original_end: segment.original_end,
                            expanded_start: segment.expanded_start,
                            expanded_end: segment.expanded_end,
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
                }),
            };
            inject_log_comments(&mut result);
            Ok(result)
        }
    }

    fn transform(&self, code: &str, filepath: &str) -> Result<TransformResult> {
        let allocator = Allocator::default();
        let source_type = SourceType::ts().with_jsx(filepath.ends_with(".tsx"));

        let ret = OxcParser::new(&allocator, code, source_type).parse();

        if !ret.errors.is_empty() {
            return Err(anyhow!("Oxc parse errors: {:?}", ret.errors));
        }

        let generated = OxcCodegen::new().build(&ret.program).code;

        Ok(TransformResult {
            code: generated,
            map: None,
            types: None,
            metadata: None,
        })
    }
}

fn apply_options(macro_host: &mut MacroExpander, options: &Option<ExpandOptions>) {
    if let Some(opts) = options {
        if let Some(keep) = opts.keep_decorators {
            macro_host.set_keep_decorators(keep);
        }
        if let Some(modules) = &opts.external_decorator_modules {
            macro_host.set_external_decorator_modules(modules.clone());
        }
        if let Some(json) = &opts.type_registry_json {
            let registry = get_or_parse_registry(json);
            if registry.is_some() {
                eprintln!(
                    "[macroforge:expand] Found type registry with {} types",
                    registry.as_ref().unwrap().len()
                );
            } else {
                eprintln!("[macroforge:expand] Failed to parse type registry JSON");
            }
            macro_host.set_type_registry(registry);
        }
        if let Some(path) = opts.config_path.as_ref() {
            if let Some(config) = CONFIG_CACHE.get(path) {
                eprintln!("[macroforge:expand] Applying config from cache: {}", path);
                crate::host::import_registry::set_foreign_types(config.foreign_types.clone());
                crate::host::import_registry::with_registry_mut(|r| {
                    r.config_imports = config
                        .config_imports
                        .iter()
                        .map(|(name, info)| (name.clone(), info.source.clone()))
                        .collect();
                });
            } else {
                eprintln!(
                    "[macroforge:expand] Config path provided but NOT FOUND in cache: {}",
                    path
                );
            }
        }
    }
}

#[cfg(feature = "swc")]
fn make_syntax_error_result(code: &str, error_msg: &str) -> ExpandResult {
    crate::host::import_registry::clear_registry();
    crate::host::import_registry::clear_foreign_types();
    ExpandResult {
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
    }
}

fn serialize_metadata(classes: &Vec<crate::ts_syn::abi::ir::ClassIR>) -> Option<String> {
    if classes.is_empty() {
        None
    } else {
        serde_json::to_string(classes).ok()
    }
}

#[cfg(feature = "swc")]
fn finalize_expansion(expansion: crate::host::expand::MacroExpansion) -> ExpandResult {
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
        let types: &mut String = types;
        types.insert_str(insert_at, "  toJSON(): Record<string, unknown>;\n");
    }

    let mut result = ExpandResult {
        code: expansion.code,
        types: types_output,
        metadata: serialize_metadata(&expansion.classes),
        diagnostics,
        source_mapping,
    };
    inject_log_comments(&mut result);
    result
}

// ============================================================================
// Public Interface
// ============================================================================

pub(crate) fn get_backend() -> Box<dyn CompilerBackend> {
    #[cfg(feature = "oxc")]
    {
        Box::new(OxcBackend)
    }
    #[cfg(all(not(feature = "oxc"), feature = "swc"))]
    {
        Box::new(SwcBackend)
    }
}

pub(crate) fn expand_inner(
    code: &str,
    filepath: &str,
    options: Option<ExpandOptions>,
) -> Result<ExpandResult> {
    if !has_macro_annotations(code) {
        return Ok(ExpandResult::unchanged(code));
    }

    get_backend().expand(code, filepath, &options)
}

pub(crate) fn transform_inner(code: &str, filepath: &str) -> Result<TransformResult> {
    get_backend().transform(code, filepath)
}

// ============================================================================
// Log Level Support
// ============================================================================

/// Log levels for MF_LOG env var, ordered by verbosity.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum LogLevel {
    Off = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

fn parse_log_level() -> LogLevel {
    match std::env::var("MF_LOG").ok().as_deref() {
        Some("error") => LogLevel::Error,
        Some("warn") => LogLevel::Warn,
        Some("info") => LogLevel::Info,
        Some("debug") => LogLevel::Debug,
        Some("trace") => LogLevel::Trace,
        Some("1" | "true") => LogLevel::Info,
        _ => LogLevel::Off,
    }
}

/// Inject trace/debug diagnostics as comments into expanded code.
/// Diagnostics with spans are inserted above the relevant line;
/// those without spans go into a block comment at the top.
fn inject_log_comments(result: &mut ExpandResult) {
    let level = parse_log_level();
    if level == LogLevel::Off {
        return;
    }

    let trace_diags: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|d| {
            d.message.starts_with("[trace]") && level >= LogLevel::Trace
                || d.level == "error" && level >= LogLevel::Error
                || d.level == "warning" && level >= LogLevel::Warn
                || d.level == "info" && level >= LogLevel::Info
        })
        .collect();

    if trace_diags.is_empty() {
        return;
    }

    // Separate positioned vs unpositioned
    let mut positioned: Vec<(u32, &str)> = Vec::new();
    let mut top_lines: Vec<String> = Vec::new();

    for d in &trace_diags {
        if let Some(start) = d.start {
            positioned.push((start, &d.message));
        } else {
            top_lines.push(format!("// {}", d.message));
        }
    }

    // Build the top block
    let mut header = String::new();
    if !top_lines.is_empty() {
        header.push_str("/*\n * MF_LOG output\n");
        for line in &top_lines {
            header.push_str(" * ");
            header.push_str(line.trim_start_matches("// "));
            header.push('\n');
        }
        header.push_str(" */\n");
    }

    // Insert positioned comments (process in reverse order to preserve offsets)
    let mut code = result.code.clone();
    positioned.sort_by(|a, b| b.0.cmp(&a.0));
    for (offset, msg) in &positioned {
        let offset = *offset as usize;
        if offset <= code.len() {
            // Find the start of the line containing this offset
            let line_start = code[..offset].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let indent = &code[line_start..offset]
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>();
            let comment = format!("{}// {}\n", indent, msg);
            code.insert_str(line_start, &comment);
        }
    }

    if !header.is_empty() {
        code.insert_str(0, &header);
    }

    result.code = code;
}

// ============================================================================
// Inner Logic (Optimized)
// ============================================================================

/// Check if source code contains `@derive(` as a standalone JSDoc directive.
pub(crate) fn has_macro_annotations(source: &str) -> bool {
    if !source.contains("@derive") {
        return false;
    }
    let mut in_code_block = false;
    for line in source.lines() {
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
        if trimmed.starts_with("@derive(") {
            return true;
        }
    }
    false
}

#[cfg(feature = "swc")]
pub(crate) fn parse_program(code: &str, filepath: &str) -> Result<(Program, Lrc<SourceMap>)> {
    let cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
    let fm = cm.new_source_file(
        FileName::Custom(filepath.to_string()).into(),
        code.to_string(),
    );
    let handler =
        Handler::with_emitter_writer(Box::new(std::io::Cursor::new(Vec::new())), Some(cm.clone()));

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: filepath.ends_with(".tsx"),
            decorators: true,
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
            let msg = format!("Failed to parse TypeScript: {:?}", error);
            error.into_diagnostic(&handler).emit();
            Err(anyhow!(msg))
        }
    }
}

#[cfg(feature = "swc")]
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
        .map_err(|e| anyhow!("{:?}", e))?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}

#[cfg(feature = "swc")]
pub(crate) fn handle_macro_diagnostics(diags: &[Diagnostic], file: &str) -> Result<(), String> {
    for diag in diags {
        if matches!(diag.level, DiagnosticLevel::Error) {
            let loc = diag
                .span
                .map(|s| format!("{}:{}-{}", file, s.start, s.end))
                .unwrap_or_else(|| file.to_string());
            return Err(format!("Macro error at {}: {}", loc, diag.message));
        }
    }
    Ok(())
}
