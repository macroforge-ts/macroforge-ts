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

// Allow the crate to reference itself as `macroforge_ts`
// This is needed for the ts_macro_derive generated code
extern crate self as macroforge_ts;

// Internal modules
pub mod host;

// Re-export abi types from ts_syn
pub use ts_syn::abi;

use ts_syn::{Diagnostic, DiagnosticLevel};
use host::derived;

mod builtin;

#[cfg(test)]
mod test;

use crate::host::MacroHostIntegration;

// ============================================================================
// Data Structures
// ============================================================================

#[napi(object)]
#[derive(Clone)]
pub struct TransformResult {
    pub code: String,
    pub map: Option<String>,
    pub types: Option<String>,
    pub metadata: Option<String>,
}

#[napi(object)]
#[derive(Clone)]
pub struct MacroDiagnostic {
    pub level: String,
    pub message: String,
    pub start: Option<u32>,
    pub end: Option<u32>,
}

#[napi(object)]
#[derive(Clone)]
pub struct MappingSegmentResult {
    pub original_start: u32,
    pub original_end: u32,
    pub expanded_start: u32,
    pub expanded_end: u32,
}

#[napi(object)]
#[derive(Clone)]
pub struct GeneratedRegionResult {
    pub start: u32,
    pub end: u32,
    pub source_macro: String,
}

#[napi(object)]
#[derive(Clone)]
pub struct SourceMappingResult {
    pub segments: Vec<MappingSegmentResult>,
    pub generated_regions: Vec<GeneratedRegionResult>,
}

#[napi(object)]
#[derive(Clone)]
pub struct ExpandResult {
    pub code: String,
    pub types: Option<String>,
    pub metadata: Option<String>,
    pub diagnostics: Vec<MacroDiagnostic>,
    pub source_mapping: Option<SourceMappingResult>,
}

#[napi(object)]
#[derive(Clone)]
pub struct ImportSourceResult {
    /// Local identifier name in the import statement
    pub local: String,
    /// Module specifier this identifier was imported from
    pub module: String,
}

#[napi(object)]
#[derive(Clone)]
pub struct SyntaxCheckResult {
    pub ok: bool,
    pub error: Option<String>,
}

#[napi(object)]
#[derive(Clone)]
pub struct SpanResult {
    pub start: u32,
    pub length: u32,
}

#[napi(object)]
#[derive(Clone)]
pub struct JsDiagnostic {
    pub start: Option<u32>,
    pub length: Option<u32>,
    pub message: Option<String>,
    pub code: Option<u32>,
    pub category: Option<String>,
}

// ============================================================================
// Position Mapper (Optimized with Binary Search)
// ============================================================================

#[napi(js_name = "PositionMapper")]
pub struct NativePositionMapper {
    segments: Vec<MappingSegmentResult>,
    generated_regions: Vec<GeneratedRegionResult>,
}

#[napi(js_name = "NativeMapper")]
pub struct NativeMapper {
    inner: NativePositionMapper,
}

#[napi]
impl NativePositionMapper {
    #[napi(constructor)]
    pub fn new(mapping: SourceMappingResult) -> Self {
        Self {
            segments: mapping.segments,
            generated_regions: mapping.generated_regions,
        }
    }

    #[napi(js_name = "isEmpty")]
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty() && self.generated_regions.is_empty()
    }

    #[napi]
    pub fn original_to_expanded(&self, pos: u32) -> u32 {
        // OPTIMIZATION: Binary search instead of linear scan
        let idx = self.segments.partition_point(|seg| seg.original_end <= pos);

        if let Some(seg) = self.segments.get(idx) {
            // Check if pos is actually inside this segment (it might be in a gap)
            if pos >= seg.original_start && pos < seg.original_end {
                let offset = pos - seg.original_start;
                return seg.expanded_start + offset;
            }
        }

        // Handle case where position is extrapolated after the last segment
        if let Some(last) = self.segments.last()
            && pos >= last.original_end
        {
            let delta = pos - last.original_end;
            return last.expanded_end + delta;
        }

        // Fallback for positions before first segment or in gaps
        pos
    }

    #[napi]
    pub fn expanded_to_original(&self, pos: u32) -> Option<u32> {
        if self.is_in_generated(pos) {
            return None;
        }

        // OPTIMIZATION: Binary search
        let idx = self.segments.partition_point(|seg| seg.expanded_end <= pos);

        if let Some(seg) = self.segments.get(idx)
            && pos >= seg.expanded_start
            && pos < seg.expanded_end
        {
            let offset = pos - seg.expanded_start;
            return Some(seg.original_start + offset);
        }

        if let Some(last) = self.segments.last()
            && pos >= last.expanded_end
        {
            let delta = pos - last.expanded_end;
            return Some(last.original_end + delta);
        }

        None
    }

    #[napi]
    pub fn generated_by(&self, pos: u32) -> Option<String> {
        // generated_regions are usually small, linear scan is fine, but can optimize if needed
        self.generated_regions
            .iter()
            .find(|r| pos >= r.start && pos < r.end)
            .map(|r| r.source_macro.clone())
    }

    #[napi]
    pub fn map_span_to_original(&self, start: u32, length: u32) -> Option<SpanResult> {
        let end = start.saturating_add(length);
        let original_start = self.expanded_to_original(start)?;
        let original_end = self.expanded_to_original(end)?;

        Some(SpanResult {
            start: original_start,
            length: original_end.saturating_sub(original_start),
        })
    }

    #[napi]
    pub fn map_span_to_expanded(&self, start: u32, length: u32) -> SpanResult {
        let end = start.saturating_add(length);
        let expanded_start = self.original_to_expanded(start);
        let expanded_end = self.original_to_expanded(end);

        SpanResult {
            start: expanded_start,
            length: expanded_end.saturating_sub(expanded_start),
        }
    }

    #[napi]
    pub fn is_in_generated(&self, pos: u32) -> bool {
        self.generated_regions
            .iter()
            .any(|r| pos >= r.start && pos < r.end)
    }
}

#[napi]
impl NativeMapper {
    #[napi(constructor)]
    pub fn new(mapping: SourceMappingResult) -> Self {
        Self {
            inner: NativePositionMapper::new(mapping),
        }
    }
    // Delegate all methods to inner
    #[napi(js_name = "isEmpty")]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    #[napi]
    pub fn original_to_expanded(&self, pos: u32) -> u32 {
        self.inner.original_to_expanded(pos)
    }
    #[napi]
    pub fn expanded_to_original(&self, pos: u32) -> Option<u32> {
        self.inner.expanded_to_original(pos)
    }
    #[napi]
    pub fn generated_by(&self, pos: u32) -> Option<String> {
        self.inner.generated_by(pos)
    }
    #[napi]
    pub fn map_span_to_original(&self, start: u32, length: u32) -> Option<SpanResult> {
        self.inner.map_span_to_original(start, length)
    }
    #[napi]
    pub fn map_span_to_expanded(&self, start: u32, length: u32) -> SpanResult {
        self.inner.map_span_to_expanded(start, length)
    }
    #[napi]
    pub fn is_in_generated(&self, pos: u32) -> bool {
        self.inner.is_in_generated(pos)
    }
}

#[napi]
pub fn check_syntax(code: String, filepath: String) -> SyntaxCheckResult {
    match parse_program(&code, &filepath) {
        Ok(_) => SyntaxCheckResult {
            ok: true,
            error: None,
        },
        Err(err) => SyntaxCheckResult {
            ok: false,
            error: Some(err.to_string()),
        },
    }
}

// ============================================================================
// Core Plugin Logic
// ============================================================================

#[napi(object)]
pub struct ProcessFileOptions {
    pub keep_decorators: Option<bool>,
    pub version: Option<String>,
}

#[napi(object)]
pub struct ExpandOptions {
    pub keep_decorators: Option<bool>,
}

#[napi]
pub struct NativePlugin {
    cache: std::sync::Mutex<std::collections::HashMap<String, CachedResult>>,
    log_file: std::sync::Mutex<Option<std::path::PathBuf>>,
}

impl Default for NativePlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
struct CachedResult {
    version: Option<String>,
    result: ExpandResult,
}

fn option_expand_options(opts: Option<ProcessFileOptions>) -> Option<ExpandOptions> {
    opts.map(|o| ExpandOptions {
        keep_decorators: o.keep_decorators,
    })
}

#[napi]
impl NativePlugin {
    #[napi(constructor)]
    pub fn new() -> Self {
        let plugin = Self {
            cache: std::sync::Mutex::new(std::collections::HashMap::new()),
            log_file: std::sync::Mutex::new(None),
        };

        // Initialize log file with default path
        if let Ok(mut log_guard) = plugin.log_file.lock() {
            let log_path = std::path::PathBuf::from("/tmp/macroforge-plugin.log");

            // Clear/create log file
            if let Err(e) = std::fs::write(&log_path, "=== macroforge plugin loaded ===\n") {
                eprintln!("[macroforge] Failed to initialize log file: {}", e);
            } else {
                *log_guard = Some(log_path);
            }
        }

        plugin
    }

    #[napi]
    pub fn log(&self, message: String) {
        if let Ok(log_guard) = self.log_file.lock()
            && let Some(log_path) = log_guard.as_ref()
        {
            use std::io::Write;
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(log_path)
            {
                let _ = writeln!(file, "{}", message);
            }
        }
    }

    #[napi]
    pub fn set_log_file(&self, path: String) {
        if let Ok(mut log_guard) = self.log_file.lock() {
            *log_guard = Some(std::path::PathBuf::from(path));
        }
    }

    #[napi]
    pub fn process_file(
        &self,
        _env: Env,
        filepath: String,
        code: String,
        options: Option<ProcessFileOptions>,
    ) -> Result<ExpandResult> {
        let version = options.as_ref().and_then(|o| o.version.clone());

        // Cache Check
        if let (Some(ver), Ok(guard)) = (version.as_ref(), self.cache.lock())
            && let Some(cached) = guard.get(&filepath)
            && cached.version.as_ref() == Some(ver)
        {
            return Ok(cached.result.clone());
        }

        // FIX: Run expansion in a separate thread with a LARGE stack (32MB).
        // Standard threads (and Node threads) often have 2MB stacks, which causes
        // "Broken pipe" / SEGFAULTS when SWC recurses deeply in macros.
        let opts_clone = option_expand_options(options);
        let filepath_for_thread = filepath.clone();

        let builder = std::thread::Builder::new().stack_size(32 * 1024 * 1024);
        let handle = builder
            .spawn(move || {
                let globals = Globals::default();
                GLOBALS.set(&globals, || {
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        // We need a dummy Env here or refactor expand_inner to not take Env
                        // since Env cannot be sent across threads.
                        // However, expand_inner only uses Env for MacroHostIntegration which likely needs it.
                        // IMPORTANT: NAPI Env is NOT thread safe. We cannot pass it.
                        // We must initialize MacroHostIntegration without Env or create a temporary scope if possible.
                        // Assuming expand_inner logic handles mostly pure Rust AST operations:
                        expand_inner(&code, &filepath_for_thread, opts_clone)
                    }))
                })
            })
            .map_err(|e| {
                Error::new(
                    Status::GenericFailure,
                    format!("Failed to spawn worker thread: {}", e),
                )
            })?;

        let expand_result = handle
            .join()
            .map_err(|_| {
                Error::new(
                    Status::GenericFailure,
                    "Macro expansion worker thread panicked (Stack Overflow?)",
                )
            })?
            .map_err(|_| {
                Error::new(
                    Status::GenericFailure,
                    "Macro expansion panicked inside worker",
                )
            })??;

        // Update Cache
        if let Ok(mut guard) = self.cache.lock() {
            guard.insert(
                filepath.clone(),
                CachedResult {
                    version,
                    result: expand_result.clone(),
                },
            );
        }

        Ok(expand_result)
    }

    #[napi]
    pub fn get_mapper(&self, filepath: String) -> Option<NativeMapper> {
        let mapping = match self.cache.lock() {
            Ok(guard) => guard
                .get(&filepath)
                .cloned()
                .and_then(|c| c.result.source_mapping),
            Err(_) => None,
        };

        mapping.map(|m| NativeMapper {
            inner: NativePositionMapper::new(m),
        })
    }

    #[napi]
    pub fn map_diagnostics(&self, filepath: String, diags: Vec<JsDiagnostic>) -> Vec<JsDiagnostic> {
        let Some(mapper) = self.get_mapper(filepath) else {
            return diags;
        };

        diags
            .into_iter()
            .map(|mut d| {
                if let (Some(start), Some(length)) = (d.start, d.length)
                    && let Some(mapped) = mapper.map_span_to_original(start, length)
                {
                    d.start = Some(mapped.start);
                    d.length = Some(mapped.length);
                }
                d
            })
            .collect()
    }
}

// ============================================================================
// Sync Functions (Refactored for Thread Safety & Performance)
// ============================================================================

#[napi]
pub fn parse_import_sources(code: String, filepath: String) -> Result<Vec<ImportSourceResult>> {
    let (program, _cm) = parse_program(&code, &filepath)?;
    let module = match program {
        Program::Module(module) => module,
        Program::Script(_) => return Ok(vec![]),
    };

    let import_map = crate::host::collect_import_sources(&module, &code);
    let mut imports = Vec::with_capacity(import_map.len());
    for (local, module) in import_map {
        imports.push(ImportSourceResult { local, module });
    }
    Ok(imports)
}

#[napi(
    js_name = "Derive",
    ts_return_type = "ClassDecorator",
    ts_args_type = "...features: any[]"
)]
pub fn derive_decorator() {}

#[napi]
pub fn transform_sync(_env: Env, code: String, filepath: String) -> Result<TransformResult> {
    // FIX: Thread isolation for transforms too
    let builder = std::thread::Builder::new().stack_size(32 * 1024 * 1024);
    let handle = builder
        .spawn(move || {
            let globals = Globals::default();
            GLOBALS.set(&globals, || {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    transform_inner(&code, &filepath)
                }))
            })
        })
        .map_err(|e| {
            Error::new(
                Status::GenericFailure,
                format!("Failed to spawn transform thread: {}", e),
            )
        })?;

    handle
        .join()
        .map_err(|_| Error::new(Status::GenericFailure, "Transform worker crashed"))?
        .map_err(|_| Error::new(Status::GenericFailure, "Transform panicked"))?
}

/// Expand macros in TypeScript code and return the transformed TS (types) and diagnostics
#[napi]
pub fn expand_sync(
    _env: Env,
    code: String,
    filepath: String,
    options: Option<ExpandOptions>,
) -> Result<ExpandResult> {
    // FIX: Thread isolation for expands too
    let builder = std::thread::Builder::new().stack_size(32 * 1024 * 1024);
    let handle = builder
        .spawn(move || {
            let globals = Globals::default();
            GLOBALS.set(&globals, || {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    expand_inner(&code, &filepath, options)
                }))
            })
        })
        .map_err(|e| {
            Error::new(
                Status::GenericFailure,
                format!("Failed to spawn expand thread: {}", e),
            )
        })?;

    handle
        .join()
        .map_err(|_| Error::new(Status::GenericFailure, "Expand worker crashed"))?
        .map_err(|_| Error::new(Status::GenericFailure, "Expand panicked"))?
}

// ============================================================================
// Inner Logic (Optimized)
// ============================================================================

/// Inner logic decoupled from NAPI Env to allow threading
fn expand_inner(
    code: &str,
    filepath: &str,
    options: Option<ExpandOptions>,
) -> Result<ExpandResult> {
    // We create a NEW macro host for this thread.
    // Note: If MacroHostIntegration requires NAPI Env for calling back into JS,
    // that part will fail in a threaded context. Assuming pure-Rust expansion here.
    let mut macro_host = MacroHostIntegration::new().map_err(|err| {
        Error::new(
            Status::GenericFailure,
            format!("Failed to initialize macro host: {err:?}"),
        )
    })?;

    if let Some(opts) = options
        && let Some(keep) = opts.keep_decorators
    {
        macro_host.set_keep_decorators(keep);
    }

    let (program, _) = match parse_program(code, filepath) {
        Ok(p) => p,
        Err(e) => {
            // Instead of failing on parse errors (which can happen frequently
            // when the user is typing or the code is incomplete), return a
            // no-op expansion result with the original code unchanged.
            // This allows the language server to continue functioning smoothly.
            let error_msg = e.to_string();

            // Return a "no-op" expansion result: original code, no changes,
            // and optionally a diagnostic for the user.
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
    // Heuristic fix: Ensure we don't inject toJSON if it exists, and be careful about placement
    if let Some(types) = &mut types_output
        && expansion.code.contains("toJSON(")
        && !types.contains("toJSON(")
    {
        // Find the last closing brace. This is still a heuristic but functional for simple cases.
        if let Some(insert_at) = types.rfind('}') {
            types.insert_str(insert_at, "  toJSON(): Record<string, unknown>;\n");
        }
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
            format!("Failed to init host: {err:?}"),
        )
    })?;

    let (program, cm) = parse_program(code, filepath)?;

    let expansion = macro_host
        .expand(code, &program, filepath)
        .map_err(|err| Error::new(Status::GenericFailure, format!("Expansion failed: {err:?}")))?;

    handle_macro_diagnostics(&expansion.diagnostics, filepath)?;

    // FIX: REMOVED REDUNDANT ROUND-TRIP
    // Previously: Parse -> Expand -> Stringify -> Parse -> Stringify
    // Now: Parse -> Expand -> Stringify (or use cached result)
    let generated = if expansion.changed {
        expansion.code
    } else {
        // Only emit if we didn't change anything (fallback to standard emit)
        emit_program(&program, &cm)?
    };

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
            let msg = format!("Failed to parse TypeScript: {:?}", error);
            error.into_diagnostic(&handler).emit();
            Err(Error::new(Status::GenericFailure, msg))
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
    emitter
        .emit_program(program)
        .map_err(|e| Error::new(Status::GenericFailure, format!("{:?}", e)))?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}

fn handle_macro_diagnostics(diags: &[Diagnostic], file: &str) -> Result<()> {
    for diag in diags {
        if matches!(diag.level, DiagnosticLevel::Error) {
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

// ============================================================================
// Manifest / Debug API
// ============================================================================

#[napi(object)]
pub struct MacroManifestEntry {
    pub name: String,
    pub kind: String,
    pub description: String,
    pub package: String,
}
#[napi(object)]
pub struct DecoratorManifestEntry {
    pub module: String,
    pub export: String,
    pub kind: String,
    pub docs: String,
}
#[napi(object)]
pub struct MacroManifest {
    pub version: u32,
    pub macros: Vec<MacroManifestEntry>,
    pub decorators: Vec<DecoratorManifestEntry>,
}

#[napi(js_name = "__macroforgeGetManifest")]
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

#[napi(js_name = "__macroforgeIsMacroPackage")]
pub fn is_macro_package() -> bool {
    !derived::macro_names().is_empty()
}
#[napi(js_name = "__macroforgeGetMacroNames")]
pub fn get_macro_names() -> Vec<String> {
    derived::macro_names()
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}
#[napi(js_name = "__macroforgeDebugGetModules")]
pub fn debug_get_modules() -> Vec<String> {
    crate::host::derived::modules()
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

#[napi(js_name = "__macroforgeDebugLookup")]
pub fn debug_lookup(module: String, name: String) -> String {
    match MacroHostIntegration::new() {
        Ok(host) => match host.dispatcher.registry().lookup(&module, &name) {
            Ok(_) => format!("Found: ({}, {})", module, name),
            Err(_) => format!("Not found: ({}, {})", module, name),
        },
        Err(e) => format!("Host init failed: {}", e),
    }
}

#[napi(js_name = "__macroforgeDebugDescriptors")]
pub fn debug_descriptors() -> Vec<String> {
    inventory::iter::<crate::host::derived::DerivedMacroRegistration>()
        .map(|entry| {
            format!(
                "name={}, module={}, package={}",
                entry.descriptor.name, entry.descriptor.module, entry.descriptor.package
            )
        })
        .collect()
}
