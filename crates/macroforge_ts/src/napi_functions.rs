use napi::bindgen_prelude::*;
use napi_derive::napi;
use swc_core::{
    common::{GLOBALS, Globals},
    ecma::ast::Program,
};

use crate::expand_core::{expand_inner, parse_program, transform_inner};
use crate::napi_types::{
    ExpandOptions, ExpandResult, ImportSourceResult, LoadConfigResult, MacroDiagnostic, ScanOptions,
    ScanResult, SyntaxCheckResult, TransformResult,
};

// ============================================================================
// Sync Functions (Refactored for Thread Safety & Performance)
// ============================================================================

/// Checks if the given TypeScript code has valid syntax.
///
/// This function attempts to parse the code using SWC's TypeScript parser
/// without performing any macro expansion.
///
/// # Arguments
///
/// * `code` - The TypeScript source code to check
/// * `filepath` - The file path (used to determine if it's TSX based on extension)
///
/// # Returns
///
/// A [`SyntaxCheckResult`] indicating success or containing the parse error.
///
/// # Example
///
/// ```javascript
/// const result = check_syntax("const x: number = 42;", "test.ts");
/// if (!result.ok) {
///     console.error("Syntax error:", result.error);
/// }
/// ```
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

/// Parses import statements from TypeScript code and returns their sources.
///
/// This function extracts information about all import statements in the code,
/// mapping each imported identifier to its source module. Useful for analyzing
/// dependencies and understanding where decorators come from.
///
/// # Arguments
///
/// * `code` - The TypeScript source code to parse
/// * `filepath` - The file path (used for TSX detection)
///
/// # Returns
///
/// A vector of [`ImportSourceResult`] entries, one for each imported identifier.
///
/// # Errors
///
/// Returns an error if the code cannot be parsed.
///
/// # Example
///
/// ```javascript
/// // For code: import { Derive, Clone } from "macroforge-ts";
/// const imports = parse_import_sources(code, "test.ts");
/// // Returns: [
/// //   { local: "Derive", module: "macroforge-ts" },
/// //   { local: "Clone", module: "macroforge-ts" }
/// // ]
/// ```
#[napi]
pub fn parse_import_sources(code: String, filepath: String) -> Result<Vec<ImportSourceResult>> {
    let (program, _cm) = parse_program(&code, &filepath)?;
    let module = match program {
        Program::Module(module) => module,
        // Scripts don't have import statements
        Program::Script(_) => return Ok(vec![]),
    };

    let import_result = crate::host::collect_import_sources(&module, &code);
    let mut imports = Vec::with_capacity(import_result.sources.len());
    for (local, module) in import_result.sources {
        imports.push(ImportSourceResult { local, module });
    }
    Ok(imports)
}

/// The `@Derive` decorator function exported to JavaScript/TypeScript.
///
/// This is a no-op function that exists purely for TypeScript type checking.
/// The actual decorator processing happens during macro expansion, where
/// `@derive(...)` decorators are recognized and transformed.
///
/// # TypeScript Usage
///
/// ```typescript
/// import { Derive } from "macroforge-ts";
///
/// @Derive(Debug, Clone, Serialize)
/// class User {
///     name: string;
///     email: string;
/// }
/// ```
#[napi(
    js_name = "Derive",
    ts_return_type = "ClassDecorator",
    ts_args_type = "...features: any[]"
)]
pub fn derive_decorator() {}

/// Load and parse a macroforge configuration file.
///
/// Parses a `macroforge.config.js/ts` file and caches the result for use
/// during macro expansion. The configuration includes both simple settings
/// (like `keepDecorators`) and foreign type handlers.
///
/// # Arguments
///
/// * `content` - The raw content of the configuration file
/// * `filepath` - Path to the configuration file (used to determine syntax and as cache key)
///
/// # Returns
///
/// A [`LoadConfigResult`] containing the parsed configuration summary.
///
/// # Example
///
/// ```javascript
/// import { loadConfig, expandSync } from 'macroforge';
/// import fs from 'fs';
///
/// const configPath = 'macroforge.config.js';
/// const configContent = fs.readFileSync(configPath, 'utf-8');
///
/// // Load and cache the configuration
/// const result = loadConfig(configContent, configPath);
/// console.log(`Loaded config with ${result.foreignTypeCount} foreign types`);
///
/// // The config is now cached and will be used by expandSync
/// const expanded = expandSync(code, filepath, { configPath });
/// ```
#[napi]
pub fn load_config(content: String, filepath: String) -> Result<LoadConfigResult> {
    use crate::host::MacroforgeConfigLoader;

    let config = MacroforgeConfigLoader::load_and_cache(&content, &filepath).map_err(|e| {
        Error::new(
            Status::GenericFailure,
            format!("Failed to parse config: {}", e),
        )
    })?;

    Ok(LoadConfigResult {
        keep_decorators: config.keep_decorators,
        generate_convenience_const: config.generate_convenience_const,
        has_foreign_types: !config.foreign_types.is_empty(),
        foreign_type_count: config.foreign_types.len() as u32,
    })
}

/// Clears the configuration cache.
///
/// This is useful for testing to ensure each test starts with a clean state.
/// In production, clearing the cache will force configs to be re-parsed on next access.
///
/// # Example
///
/// ```javascript
/// const { clearConfigCache, loadConfig } = require('macroforge-ts');
///
/// // Clear cache before each test
/// clearConfigCache();
///
/// // Now load a fresh config
/// const result = loadConfig(configContent, configPath);
/// ```
#[napi]
pub fn clear_config_cache() {
    crate::host::clear_config_cache();
}

/// Synchronously transforms TypeScript code through the macro expansion system.
///
/// This is similar to [`expand_sync`] but returns a [`TransformResult`] which
/// includes source map information (when available).
///
/// # Arguments
///
/// * `_env` - NAPI environment (unused but required by NAPI)
/// * `code` - The TypeScript source code to transform
/// * `filepath` - The file path (used for TSX detection)
///
/// # Returns
///
/// A [`TransformResult`] containing the transformed code and metadata.
///
/// # Errors
///
/// Returns an error if:
/// - Thread spawning fails
/// - The worker thread panics
/// - Macro expansion fails
///
/// # Thread Safety
///
/// Uses a 32MB thread stack to prevent stack overflow during deep AST recursion.
#[napi]
pub fn transform_sync(_env: Env, code: String, filepath: String) -> Result<TransformResult> {
    // Run in a separate thread with large stack for deep AST recursion
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

/// Synchronously expands macros in TypeScript code.
///
/// This is the standalone macro expansion function that doesn't use caching.
/// For cached expansion, use [`NativePlugin::process_file`] instead.
///
/// # Arguments
///
/// * `_env` - NAPI environment (unused but required by NAPI)
/// * `code` - The TypeScript source code to expand
/// * `filepath` - The file path (used for TSX detection)
/// * `options` - Optional configuration (e.g., `keep_decorators`)
///
/// # Returns
///
/// An [`ExpandResult`] containing the expanded code, diagnostics, and source mapping.
///
/// # Errors
///
/// Returns an error if:
/// - Thread spawning fails
/// - The worker thread panics
/// - Macro host initialization fails
///
/// # Performance
///
/// - Uses a 32MB thread stack to prevent stack overflow
/// - Performs early bailout for files without `@derive` decorators
///
/// # Example
///
/// ```javascript
/// const result = expand_sync(env, code, "user.ts", { keep_decorators: false });
/// console.log(result.code); // Expanded TypeScript code
/// console.log(result.diagnostics); // Any warnings or errors
/// ```
#[napi]
pub fn expand_sync(
    _env: Env,
    code: String,
    filepath: String,
    options: Option<ExpandOptions>,
) -> Result<ExpandResult> {
    // Run in a separate thread with large stack for deep AST recursion
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
// Project Scanning for Type Awareness
// ============================================================================

/// Scan a TypeScript project and build a type registry.
///
/// This should be called once at build start (e.g., in Vite's `buildStart` hook)
/// and the resulting `registry_json` should be passed to [`expand_sync`] via
/// `ExpandOptions.type_registry_json`.
///
/// # Arguments
///
/// * `root_dir` - The project root directory to scan
/// * `options` - Optional scan configuration
///
/// # Returns
///
/// A [`ScanResult`] with the serialized registry and scan statistics.
///
/// # Example
///
/// ```javascript
/// const scan = scanProjectSync(projectRoot);
/// console.log(`Found ${scan.typesFound} types in ${scan.filesScanned} files`);
///
/// // Pass to expand_sync for each file
/// const result = expandSync(code, filepath, {
///   typeRegistryJson: scan.registryJson,
/// });
/// ```
#[napi]
pub fn scan_project_sync(root_dir: String, options: Option<ScanOptions>) -> Result<ScanResult> {
    let builder = std::thread::Builder::new().stack_size(32 * 1024 * 1024);
    let handle = builder
        .spawn(move || scan_project_inner(&root_dir, options))
        .map_err(|e| {
            Error::new(
                Status::GenericFailure,
                format!("Failed to spawn scan thread: {}", e),
            )
        })?;

    handle
        .join()
        .map_err(|_| Error::new(Status::GenericFailure, "Scan worker panicked"))?
}

fn scan_project_inner(root_dir: &str, options: Option<ScanOptions>) -> Result<ScanResult> {
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

    let scanner = ProjectScanner::new(config);
    let output = scanner.scan().map_err(|e| {
        Error::new(
            Status::GenericFailure,
            format!("Project scan failed: {}", e),
        )
    })?;

    let types_found = output.registry.len() as u32;
    let registry_json = serde_json::to_string(&output.registry).map_err(|e| {
        Error::new(
            Status::GenericFailure,
            format!("Failed to serialize registry: {}", e),
        )
    })?;

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
        files_scanned: output.files_scanned,
        types_found,
        diagnostics,
    })
}
