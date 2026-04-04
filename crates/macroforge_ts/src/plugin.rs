use napi::bindgen_prelude::*;
use napi_derive::napi;
#[cfg(feature = "swc")]
use swc_core::common::{GLOBALS, Globals};

use crate::api_types::{ExpandOptions, ExpandResult, JsDiagnostic, ProcessFileOptions};
use crate::expand_core::expand_inner;
use crate::position_mapper::{NativeMapper, NativePositionMapper};

// ============================================================================
// Core Plugin Logic
// ============================================================================

/// The main plugin class for macro expansion with caching support.
///
/// `NativePlugin` is designed to be instantiated once and reused across multiple
/// file processing operations. It maintains a cache of expansion results keyed
/// by filepath and version, enabling efficient incremental processing.
///
/// # Thread Safety
///
/// The plugin is thread-safe through the use of `Mutex` for internal state.
/// However, macro expansion itself runs in a separate thread with a 32MB stack
/// to prevent stack overflow during deep AST recursion.
///
/// # Example
///
/// ```javascript
/// // Create a single plugin instance (typically at startup)
/// const plugin = new NativePlugin();
///
/// // Process files with caching
/// const result1 = plugin.process_file("src/foo.ts", code1, { version: "1" });
/// const result2 = plugin.process_file("src/foo.ts", code2, { version: "1" }); // Cache hit!
/// const result3 = plugin.process_file("src/foo.ts", code3, { version: "2" }); // Cache miss
///
/// // Get a mapper for position translation
/// const mapper = plugin.get_mapper("src/foo.ts");
/// ```
#[napi]
pub struct NativePlugin {
    /// Cache of expansion results, keyed by filepath.
    /// Protected by a mutex for thread-safe access.
    cache: std::sync::Mutex<std::collections::HashMap<String, CachedResult>>,
    /// Optional path to a log file for debugging.
    /// Protected by a mutex for thread-safe access.
    log_file: std::sync::Mutex<Option<std::path::PathBuf>>,
}

impl Default for NativePlugin {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal structure for cached expansion results.
#[derive(Clone)]
struct CachedResult {
    /// Version string at the time of caching.
    /// Used for cache invalidation when version changes.
    version: Option<String>,
    /// The cached expansion result.
    result: ExpandResult,
}

/// Converts `ProcessFileOptions` to `ExpandOptions`.
///
/// Extracts only the options relevant for expansion, discarding cache-related
/// options like `version`.
pub(crate) fn option_expand_options(opts: Option<ProcessFileOptions>) -> Option<ExpandOptions> {
    opts.map(|o| ExpandOptions {
        keep_decorators: o.keep_decorators,
        external_decorator_modules: o.external_decorator_modules,
        config_path: o.config_path,
        type_registry_json: o.type_registry_json,
    })
}

#[napi]
impl NativePlugin {
    /// Creates a new `NativePlugin` instance.
    ///
    /// Initializes the plugin with an empty cache and sets up a default log file
    /// at `/tmp/macroforge-plugin.log` for debugging purposes.
    ///
    /// # Returns
    ///
    /// A new `NativePlugin` ready for processing files.
    ///
    /// # Side Effects
    ///
    /// Creates or clears the log file at `/tmp/macroforge-plugin.log`.
    #[napi(constructor)]
    pub fn new() -> Self {
        let plugin = Self {
            cache: std::sync::Mutex::new(std::collections::HashMap::new()),
            log_file: std::sync::Mutex::new(None),
        };

        // Initialize log file with default path for debugging.
        // This is useful for diagnosing issues in production environments
        // where console output might not be easily accessible.
        if let Ok(mut log_guard) = plugin.log_file.lock() {
            let log_path = std::path::PathBuf::from("/tmp/macroforge-plugin.log");

            // Clear/create log file to start fresh on each plugin instantiation
            if let Err(e) = std::fs::write(&log_path, "=== macroforge plugin loaded ===\n") {
                eprintln!("[macroforge] Failed to initialize log file: {}", e);
            } else {
                *log_guard = Some(log_path);
            }
        }

        plugin
    }

    /// Writes a message to the plugin's log file.
    ///
    /// Useful for debugging macro expansion issues in production environments.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log
    ///
    /// # Note
    ///
    /// Messages are appended to the log file. If the log file hasn't been
    /// configured or cannot be written to, the message is silently dropped.
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

    /// Sets the path for the plugin's log file.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path to use for logging
    ///
    /// # Note
    ///
    /// This does not create the file; it will be created when the first
    /// message is logged.
    #[napi]
    pub fn set_log_file(&self, path: String) {
        if let Ok(mut log_guard) = self.log_file.lock() {
            *log_guard = Some(std::path::PathBuf::from(path));
        }
    }

    /// Processes a TypeScript file through the macro expansion system.
    ///
    /// This is the main entry point for file processing. It handles caching,
    /// thread isolation (to prevent stack overflow), and error recovery.
    ///
    /// # Arguments
    ///
    /// * `_env` - NAPI environment (unused but required by NAPI)
    /// * `filepath` - Path to the file (used for TSX detection and caching)
    /// * `code` - The TypeScript source code to process
    /// * `options` - Optional configuration for expansion and caching
    ///
    /// # Returns
    ///
    /// An [`ExpandResult`] containing the expanded code, diagnostics, and source mapping.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Thread spawning fails
    /// - The worker thread panics (often due to stack overflow)
    /// - Macro expansion fails internally
    ///
    /// # Performance
    ///
    /// - Uses a 32MB thread stack to prevent stack overflow during deep AST recursion
    /// - Caches results by filepath and version for efficient incremental processing
    /// - Early bailout for files without `@derive` decorators
    ///
    /// # Thread Safety
    ///
    /// Macro expansion runs in a separate thread because:
    /// 1. SWC AST operations can be deeply recursive, exceeding default stack limits
    /// 2. Node.js thread stack is typically only 2MB
    /// 3. Panics in the worker thread are caught and reported gracefully
    #[napi]
    pub fn process_file(
        &self,
        _env: Env,
        filepath: String,
        code: String,
        options: Option<ProcessFileOptions>,
    ) -> Result<ExpandResult> {
        let version = options.as_ref().and_then(|o| o.version.clone());

        // Cache Check: Return cached result if version matches.
        // This enables efficient incremental processing when files haven't changed.
        if let (Some(ver), Ok(guard)) = (version.as_ref(), self.cache.lock())
            && let Some(cached) = guard.get(&filepath)
            && cached.version.as_ref() == Some(ver)
        {
            return Ok(cached.result.clone());
        }

        // Run expansion in a separate thread with a LARGE stack (32MB).
        // Standard threads (and Node threads) often have 2MB stacks, which causes
        // "Broken pipe" / SEGFAULTS when SWC recurses deeply in macros.
        let opts_clone = option_expand_options(options);
        let filepath_for_thread = filepath.clone();

        let builder = std::thread::Builder::new().stack_size(32 * 1024 * 1024);
        let handle = builder
            .spawn(move || {
                let work = move || {
                    // Catch panics to report them gracefully instead of crashing.
                    // Common cause: stack overflow from deeply nested AST.
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        // Note: NAPI Env is NOT thread safe - we cannot pass it.
                        // The expand_inner function uses pure Rust AST operations.
                        expand_inner(&code, &filepath_for_thread, opts_clone)
                    }))
                };

                #[cfg(feature = "swc")]
                {
                    // Set up SWC globals for this thread.
                    // SWC uses thread-local storage for some operations.
                    let globals = Globals::default();
                    GLOBALS.set(&globals, work)
                }

                #[cfg(all(not(feature = "swc"), feature = "oxc"))]
                {
                    work()
                }
            })
            .map_err(|e| {
                Error::new(
                    Status::GenericFailure,
                    format!("Failed to spawn worker thread: {}", e),
                )
            })?;

        // Wait for the worker thread and unwrap nested Results.
        // Result structure: join() -> panic catch -> expand_inner -> final result
        let expand_result = handle
            .join()
            .map_err(|_| {
                Error::new(
                    Status::GenericFailure,
                    "Macro expansion worker thread panicked (Stack Overflow?)".to_string(),
                )
            })?
            .map_err(|_| {
                Error::new(
                    Status::GenericFailure,
                    "Macro expansion panicked inside worker".to_string(),
                )
            })?
            .map_err(|e| {
                Error::new(
                    Status::GenericFailure,
                    format!("Macro expansion failed: {}", e),
                )
            })?;

        // Update Cache: Store the result for future requests with the same version.
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

    /// Retrieves a position mapper for a previously processed file.
    ///
    /// The mapper enables translation between original and expanded source positions,
    /// which is essential for IDE features like error reporting and navigation.
    ///
    /// # Arguments
    ///
    /// * `filepath` - Path to the file (must have been previously processed)
    ///
    /// # Returns
    ///
    /// `Some(NativeMapper)` if the file has been processed and has source mapping data,
    /// `None` if the file hasn't been processed or has no mapping (no macros expanded).
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

    /// Maps diagnostics from expanded source positions back to original source positions.
    ///
    /// This is used by IDE integrations to show errors at the correct locations
    /// in the user's original code, rather than in the macro-expanded output.
    ///
    /// # Arguments
    ///
    /// * `filepath` - Path to the file the diagnostics are for
    /// * `diags` - Diagnostics with positions in the expanded source
    ///
    /// # Returns
    ///
    /// Diagnostics with positions mapped back to the original source.
    /// If no mapper is available for the file, returns diagnostics unchanged.
    #[napi]
    pub fn map_diagnostics(&self, filepath: String, diags: Vec<JsDiagnostic>) -> Vec<JsDiagnostic> {
        let Some(mapper) = self.get_mapper(filepath) else {
            // No mapper available - return diagnostics unchanged
            return diags;
        };

        diags
            .into_iter()
            .map(|mut d| {
                // Attempt to map the diagnostic span back to original source
                if let (Some(start), Some(length)) = (d.start, d.length)
                    && let Some(mapped) = mapper.map_span_to_original(start, length)
                {
                    d.start = Some(mapped.start);
                    d.length = Some(mapped.length);
                }
                // Note: Diagnostics in generated code cannot be mapped and keep
                // their original (expanded) positions
                d
            })
            .collect()
    }
}
