use crate::api::CoreEngine;
use crate::api_types::{
    ExpandOptions, ExpandResult, ImportSourceResult, LoadConfigResult, ScanOptions, ScanResult,
    SyntaxCheckResult, TransformResult,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn check_syntax(code: String, filepath: String) -> SyntaxCheckResult {
    CoreEngine::check_syntax(&code, &filepath).unwrap_or_else(|e| SyntaxCheckResult {
        ok: false,
        error: Some(e),
    })
}

#[napi]
pub fn parse_import_sources(code: String, filepath: String) -> Result<Vec<ImportSourceResult>> {
    CoreEngine::parse_import_sources(&code, &filepath)
        .map_err(|e| Error::new(Status::GenericFailure, e))
}

#[napi(
    js_name = "Derive",
    ts_return_type = "ClassDecorator",
    ts_args_type = "...features: any[]"
)]
pub fn derive_decorator() {}

#[napi]
pub fn load_config(content: String, filepath: String) -> Result<LoadConfigResult> {
    CoreEngine::load_config(&content, &filepath).map_err(|e| Error::new(Status::GenericFailure, e))
}

#[napi]
pub fn clear_config_cache() {
    CoreEngine::clear_config_cache();
}

#[napi]
pub fn transform_sync(_env: Env, code: String, filepath: String) -> Result<TransformResult> {
    CoreEngine::transform_sync(code, filepath).map_err(|e| Error::new(Status::GenericFailure, e))
}

#[napi]
pub fn expand_sync(
    _env: Env,
    code: String,
    filepath: String,
    options: Option<ExpandOptions>,
) -> Result<ExpandResult> {
    CoreEngine::expand_sync(code, filepath, options)
        .map_err(|e| Error::new(Status::GenericFailure, e))
}

/// Async macro expansion.
///
/// Returns a Promise that resolves to the same [`ExpandResult`] as
/// [`expand_sync`]. Vite's async `transform` hook can `await` this so
/// expansion doesn't block the plugin worker for the duration of a
/// long `@buildtime` evaluation. Internally the pipeline is still
/// synchronous; hosting it on an `async fn` is the NAPI idiom for
/// dispatching to a worker-pool thread and returning a Promise.
#[napi(js_name = "expand")]
pub async fn expand(
    code: String,
    filepath: String,
    options: Option<ExpandOptions>,
) -> Result<ExpandResult> {
    CoreEngine::expand_sync(code, filepath, options)
        .map_err(|e| Error::new(Status::GenericFailure, e))
}

#[napi]
pub fn scan_project_sync(root_dir: String, options: Option<ScanOptions>) -> Result<ScanResult> {
    CoreEngine::scan_project_sync(root_dir, options)
        .map_err(|e| Error::new(Status::GenericFailure, e))
}

/// Phase 17 — drop a single file from the singleton scan cache.
/// Called by the Vite plugin's HMR hook when a file changes on disk.
/// The next `scanProjectSync` call will re-parse the invalidated file
/// and serve fresh IR to the rest of the pipeline.
#[napi]
pub fn invalidate_scan_cache_entry(path: String) -> bool {
    CoreEngine::invalidate_scan_cache_entry(&path)
}

/// Phase 17 — drop the entire scan cache. Called when config files
/// (`macroforge.config.ts`, `tsconfig.json`) change, since lowered IR
/// can depend on config-driven options.
#[napi]
pub fn clear_scan_cache() {
    CoreEngine::clear_scan_cache();
}
