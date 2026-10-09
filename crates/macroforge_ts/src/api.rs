use crate::api_types::{
    ExpandOptions, ExpandResult, ImportSourceResult, LoadConfigResult, MacroDiagnostic,
    ScanOptions, ScanResult, SyntaxCheckResult,
};
use crate::expand_core::expand_inner;
use crate::host::expand::trace_enabled;

// ---------------------------------------------------------------------------
// Singleton scanner
// ---------------------------------------------------------------------------
//
// Long-lived hosts reuse one scanner across calls, so the per-file scan cache
// carries over between scans. It is kept per root and scan options, and its
// cache persists in the project for the next process. Native only: the wasm
// build scans through Node's filesystem one call at a time.

#[cfg(not(target_arch = "wasm32"))]
struct CachedScanner {
    root_dir: std::path::PathBuf,
    extensions: Vec<String>,
    exported_only: bool,
    scanner: crate::host::scanner::ProjectScanner,
    /// The registries' JSON from the last scan, reused while nothing changed.
    last_json: Option<(String, String)>,
}

#[cfg(not(target_arch = "wasm32"))]
static SINGLETON_SCANNER: std::sync::OnceLock<std::sync::Mutex<Option<CachedScanner>>> =
    std::sync::OnceLock::new();

#[cfg(not(target_arch = "wasm32"))]
fn singleton() -> &'static std::sync::Mutex<Option<CachedScanner>> {
    SINGLETON_SCANNER.get_or_init(|| std::sync::Mutex::new(None))
}

#[cfg(not(target_arch = "wasm32"))]
use crate::workers::worker_pool;

/// Output-agnostic facade over the macro engine.
///
/// Every public entry point in `bindings_wasm`, and the CLI, delegates to an
/// associated function here, so this is the single place where parsing,
/// expansion, and scanning behavior is defined.
/// On native targets, [`CoreEngine::expand_sync`] and
/// [`CoreEngine::scan_project_sync`] run their work on a dedicated worker
/// thread with a 32MB stack (deep AST recursion overflows the default stack)
/// and catch panics, reporting them as `Err(String)` instead of aborting the
/// host process.
pub struct CoreEngine;

impl CoreEngine {
    /// Parse `code` and report whether it is syntactically valid TypeScript.
    pub fn check_syntax(code: &str, filepath: &str) -> Result<SyntaxCheckResult, String> {
        use oxc::allocator::Allocator;
        use oxc::parser::Parser;

        let allocator = Allocator::default();
        let source_type = crate::source_type::for_path(filepath);
        let parsed = Parser::new(&allocator, code, source_type).parse();

        if parsed.diagnostics.is_empty() {
            Ok(SyntaxCheckResult {
                ok: true,
                error: None,
            })
        } else {
            Ok(SyntaxCheckResult {
                ok: false,
                error: Some(
                    parsed
                        .diagnostics
                        .into_iter()
                        .map(|diagnostic| diagnostic.to_string())
                        .collect::<Vec<_>>()
                        .join("; "),
                ),
            })
        }
    }

    /// Collect the `(local name, module specifier)` pairs of every import in `code`.
    pub fn parse_import_sources(
        code: &str,
        filepath: &str,
    ) -> Result<Vec<ImportSourceResult>, String> {
        use oxc::allocator::Allocator;
        use oxc::parser::Parser;

        let allocator = Allocator::default();
        let source_type = crate::source_type::for_path(filepath);
        let parsed = Parser::new(&allocator, code, source_type).parse();
        if !parsed.diagnostics.is_empty() {
            return Err(parsed
                .diagnostics
                .into_iter()
                .map(|diagnostic| diagnostic.to_string())
                .collect::<Vec<_>>()
                .join("; "));
        }

        let registry = crate::ts_syn::ImportRegistry::from_program(&parsed.program, code);
        Ok(registry
            .source_modules()
            .into_iter()
            .map(|(local, module)| ImportSourceResult { local, module })
            .collect())
    }

    /// Parse a `macroforge.config.*` source, cache it process-wide, and
    /// return a summary of which config blocks were provided.
    pub fn load_config(content: &str, filepath: &str) -> Result<LoadConfigResult, String> {
        use crate::host::MacroforgeConfigLoader;
        if trace_enabled() {
            eprintln!("[macroforge:api] load_config called for {}", filepath);
        }

        // The caller receives the failure; the trace only records it.
        let config = MacroforgeConfigLoader::load_and_cache(content, filepath).map_err(|e| {
            if trace_enabled() {
                eprintln!("[macroforge:api] load_config failed: {}", e);
            }
            format!("Failed to parse config: {}", e)
        })?;

        let has_foreign_types = !config.foreign_types.is_empty();
        let foreign_type_count = config.foreign_types.len() as u32;

        // A block is considered "provided" when any sub-field deviates from
        // the library default. Plugins use these booleans purely for logging.
        let default_config = macroforge_ts_syn::config::MacroforgeConfig::default();
        let has_cfg_flags = !config.cfg.features.is_empty()
            || config.cfg.target.is_some()
            || config.cfg.debug_assertions
            || !config.cfg.custom.is_empty();
        let has_deprecated_config = config.deprecated.runtime_warn
            != default_config.deprecated.runtime_warn
            || config.deprecated.fail_on_use != default_config.deprecated.fail_on_use;
        let has_must_use_config = !matches!(
            config.must_use.mode,
            macroforge_ts_syn::config::MustUseMode::Lint
        );
        let has_non_exhaustive_config =
            config.non_exhaustive.brand != default_config.non_exhaustive.brand;

        if trace_enabled() {
            eprintln!(
                "[macroforge:api] load_config success: keep_decorators={}, foreign_types={}, cfg={}",
                config.keep_decorators, foreign_type_count, has_cfg_flags
            );
        }

        Ok(LoadConfigResult {
            keep_decorators: config.keep_decorators,
            generate_convenience_const: config.generate_convenience_const,
            has_foreign_types,
            foreign_type_count,
            has_cfg_flags,
            has_deprecated_config,
            has_must_use_config,
            has_non_exhaustive_config,
        })
    }

    /// Drop the process-wide config cache populated by [`Self::load_config`].
    pub fn clear_config_cache() {
        if trace_enabled() {
            eprintln!("[macroforge:api] clear_config_cache called");
        }
        crate::host::clear_config_cache();
    }

    /// Parses a type registry and keeps it for the process, returning the id
    /// [`ExpandOptions::type_registry_id`] names it by.
    pub fn set_type_registry(json: &str) -> Result<u32, String> {
        crate::expand_core::set_type_registry(json).map_err(|err| format!("{err:#}"))
    }

    /// Parses a declarative registry and keeps it for the process, returning
    /// the id [`ExpandOptions::declarative_registry_id`] names it by.
    pub fn set_declarative_registry(json: &str) -> Result<u32, String> {
        crate::expand_core::set_declarative_registry(json).map_err(|err| format!("{err:#}"))
    }

    /// Forgets a registry kept by [`Self::set_type_registry`] or
    /// [`Self::set_declarative_registry`].
    pub fn release_registry(id: u32) -> Result<(), String> {
        crate::expand_core::release_registry(id).map_err(|err| format!("{err:#}"))
    }

    /// Expand macros in `code` and return the full result (code, diagnostics,
    /// source mapping, metadata). On native targets the work runs on a pooled
    /// worker thread with a 32MB stack; panics are caught and returned as
    /// `Err`.
    pub fn expand_sync(
        code: String,
        filepath: String,
        options: Option<ExpandOptions>,
    ) -> Result<ExpandResult, String> {
        if trace_enabled() {
            eprintln!("[macroforge:api] expand_sync called for {}", filepath);
        }
        if trace_enabled()
            && let Some(ref opts) = options
        {
            eprintln!(
                "[macroforge:api] options: config_path={:?}, external_decorator_modules={:?}, has_type_registry={}",
                opts.config_path,
                opts.external_decorator_modules,
                opts.type_registry_json.is_some() || opts.type_registry_id.is_some()
            );
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            worker_pool()?
                .install(|| {
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let thread_state = crate::host::project::ThreadStateReset;
                        let result = expand_inner(&code, &filepath, options);
                        drop(thread_state);
                        result
                    }))
                })
                .map_err(|_| "Expand panicked".to_string())?
                .map_err(|e| e.to_string())
        }

        #[cfg(target_arch = "wasm32")]
        {
            expand_inner(&code, &filepath, options).map_err(|e| e.to_string())
        }
    }

    /// Invalidate a single file in the singleton scan cache. Called
    /// by the Vite plugin's `handleHotUpdate` hook so the next
    /// [`Self::scan_project_sync`] re-parses the changed file instead
    /// of serving stale IR. Returns `true` when an entry was actually
    /// dropped, `false` when the cache was empty or the path wasn't
    /// cached. No-op on WASM.
    pub fn invalidate_scan_cache_entry(path: &str) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let guard = singleton().lock().ok();
            if let Some(guard) = guard
                && let Some(cs) = guard.as_ref()
            {
                return cs
                    .scanner
                    .invalidate_cache_entry(std::path::Path::new(path));
            }
            false
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = path;
            false
        }
    }

    /// Clear the entire singleton scan cache. Called when
    /// `macroforge.config.ts` or `tsconfig.json` changes: anything
    /// that could invalidate previously-lowered IR.
    pub fn clear_scan_cache() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Ok(guard) = singleton().lock()
                && let Some(cs) = guard.as_ref()
            {
                cs.scanner.clear_cache();
            }
        }
    }

    /// Scan `root_dir` for exported types and declarative macros, returning
    /// the serialized registries. Reuses the process-global cached scanner
    /// when the root matches; runs on a pooled 32MB-stack worker on native.
    pub fn scan_project_sync(
        root_dir: String,
        options: Option<ScanOptions>,
    ) -> Result<ScanResult, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            worker_pool()?
                .install(|| {
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        Self::scan_project_inner(&root_dir, options)
                    }))
                })
                .map_err(|_| "Scan worker panicked".to_string())?
        }
        #[cfg(target_arch = "wasm32")]
        {
            Self::scan_project_inner(&root_dir, options)
        }
    }

    fn scan_project_inner(
        root_dir: &str,
        options: Option<ScanOptions>,
    ) -> Result<ScanResult, String> {
        use crate::host::scanner::{ProjectScanner, ScanConfig};

        let mut config = ScanConfig {
            root_dir: std::path::PathBuf::from(root_dir),
            ..ScanConfig::default()
        };

        if let Some(opts) = options {
            if let Some(exts) = opts.extensions {
                config.extensions = exts;
            }
            if let Some(exported) = opts.exported_only {
                config.exported_only = exported;
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        let (output, (registry_json, declarative_registry_json)) = {
            use crate::host::scanner::{ScanCache, cache::persisted_path};

            let mut guard = singleton()
                .lock()
                .map_err(|_| "singleton scanner mutex poisoned".to_string())?;
            let reusable = guard.as_ref().is_some_and(|cached| {
                cached.root_dir == config.root_dir
                    && cached.extensions == config.extensions
                    && cached.exported_only == config.exported_only
            });
            let cached = match guard.take().filter(|_| reusable) {
                Some(cached) => cached,
                None => {
                    let root_dir = config.root_dir.clone();
                    let extensions = config.extensions.clone();
                    let exported_only = config.exported_only;
                    let cache = ScanCache::load(&persisted_path(&root_dir));
                    CachedScanner {
                        root_dir,
                        extensions,
                        exported_only,
                        scanner: ProjectScanner::new(config).with_cache(cache),
                        last_json: None,
                    }
                }
            };
            let cached = guard.insert(cached);
            let output = cached
                .scanner
                .scan()
                .map_err(|e| format!("Project scan failed: {}", e))?;
            // A project without macros never gets a `.macroforge/` directory.
            if output.changed
                && output.macro_files > 0
                && let Err(error) = cached.scanner.save_cache(&persisted_path(&cached.root_dir))
            {
                eprintln!("[macroforge] warning: could not persist the scan cache: {error:#}");
            }
            if output.macro_files > 0 {
                crate::host::config::hoist::sync_expanded_config(&cached.root_dir).map_err(
                    |error| {
                        format!(
                            "could not expand the config of {}: {error}",
                            cached.root_dir.display()
                        )
                    },
                )?;
            }
            let json = match cached.last_json.take().filter(|_| !output.changed) {
                Some(json) => json,
                None => registries_json(&output)?,
            };
            cached.last_json = Some(json.clone());
            (output, json)
        };
        #[cfg(target_arch = "wasm32")]
        let (output, (registry_json, declarative_registry_json)) = {
            let output = ProjectScanner::new(config)
                .scan()
                .map_err(|e| format!("Project scan failed: {}", e))?;
            let json = registries_json(&output)?;
            (output, json)
        };

        let types_found = output.registry.len() as u32;
        let declarative_macros_found = output.declarative_registry.macro_count() as u32;

        let diagnostics = output
            .warnings
            .into_iter()
            .map(|msg| MacroDiagnostic {
                level: "warning".to_string(),
                message: msg,
                start: None,
                end: None,
            })
            .collect();

        Ok(ScanResult {
            registry_json,
            declarative_registry_json,
            files_scanned: output.files_scanned,
            types_found,
            declarative_macros_found,
            diagnostics,
        })
    }
}

/// The JSON of a scan's type registry and declarative registry.
fn registries_json(output: &crate::host::scanner::ScanOutput) -> Result<(String, String), String> {
    let registry_json = serde_json::to_string(&output.registry)
        .map_err(|e| format!("Failed to serialize registry: {}", e))?;
    let declarative_registry_json = output
        .declarative_registry
        .to_json()
        .map_err(|e| format!("Failed to serialize declarative registry: {}", e))?;
    Ok((registry_json, declarative_registry_json))
}
