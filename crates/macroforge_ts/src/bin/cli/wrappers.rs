use anyhow::{Context, Result};
use std::{fs, path::PathBuf, sync::Mutex};

/// Cached path to the type registry JSON file (built once per process via scanProjectSync).
pub(crate) static TYPE_REGISTRY_CACHE_PATH: Mutex<Option<String>> = Mutex::new(None);

/// Cached path to the declarative macro registry JSON file (built in the
/// same scan as the type registry).
pub(crate) static DECLARATIVE_REGISTRY_CACHE_PATH: Mutex<Option<String>> = Mutex::new(None);

/// Builds the type registry using the native ProjectScanner (no Node.js needed)
/// and caches the result to a temp file. Subsequent calls are no-ops.
///
/// The same scan also builds the declarative macro registry and writes it
/// to `.macroforge/declarative-registry.json` alongside the type registry,
/// so cross-file `/** import macro */` resolution works in the
/// tsc/svelte-check wrappers and the Vite plugin.
pub(crate) fn ensure_type_registry_cache() {
    let mut cached = TYPE_REGISTRY_CACHE_PATH.lock().unwrap();
    if cached.is_some() {
        return;
    }

    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    // Write registry to project-local .macroforge/ directory (not global /tmp)
    let cache_dir = cwd.join(".macroforge");
    let _ = fs::create_dir_all(&cache_dir);
    let registry_path = cache_dir.join("type-registry.json");
    let declarative_path = cache_dir.join("declarative-registry.json");

    use macroforge_ts::host::scanner::{ProjectScanner, ScanConfig};

    let config = ScanConfig {
        root_dir: cwd.clone(),
        ..ScanConfig::default()
    };
    let scanner = ProjectScanner::new(config);

    match scanner.scan() {
        Ok(output) => {
            let types_found = output.registry.len();
            let macros_found = output.declarative_registry.macro_count();
            match serde_json::to_string(&output.registry) {
                Ok(json) => {
                    if let Err(e) = fs::write(&registry_path, &json) {
                        eprintln!("[macroforge] Failed to write type registry: {e}");
                        *cached = None;
                        return;
                    }
                    eprintln!(
                        "[macroforge] Type scan: {} types from {} files",
                        types_found, output.files_scanned
                    );
                    *cached = Some(registry_path.to_string_lossy().to_string());
                }
                Err(e) => {
                    eprintln!("[macroforge] Failed to serialize type registry: {e}");
                    *cached = None;
                }
            }

            // Emit the declarative macro registry alongside the type
            // registry. Failures here are non-fatal: the type registry
            // still ships, and declarative cross-file imports just won't
            // resolve (they'll surface as diagnostics at use sites).
            match output.declarative_registry.to_json() {
                Ok(json) => {
                    if let Err(e) = fs::write(&declarative_path, &json) {
                        eprintln!("[macroforge] Failed to write declarative macro registry: {e}");
                    } else {
                        eprintln!(
                            "[macroforge] Declarative scan: {} macros across {} files",
                            macros_found,
                            output.declarative_registry.file_count()
                        );
                        *DECLARATIVE_REGISTRY_CACHE_PATH.lock().unwrap() =
                            Some(declarative_path.to_string_lossy().to_string());
                    }
                }
                Err(e) => {
                    eprintln!("[macroforge] Failed to serialize declarative macro registry: {e}");
                }
            }
        }
        Err(e) => {
            eprintln!("[macroforge] Type scan failed: {e}");
            *cached = None;
        }
    }
}

/// Runs TypeScript type checking with macro expansion baked into file reads.
///
/// This function creates a Node.js script that wraps `tsc --noEmit` behavior
/// while transparently expanding macros when reading `.ts` and `.tsx` files.
/// Files containing `@derive` are expanded before being passed to the TypeScript
/// compiler.
///
/// The wrapper intercepts `CompilerHost.getSourceFile` to:
/// 1. Read the file contents
/// 2. Check for `@derive` decorators
/// 3. Expand macros if found
/// 4. Return the expanded source to TypeScript
///
/// # Arguments
///
/// * `project` - Optional path to `tsconfig.json` (defaults to `tsconfig.json` in cwd)
///
/// # Returns
///
/// Returns `Ok(())` if type checking passes, or an error with diagnostic details.
pub fn run_tsc_wrapper(project: Option<PathBuf>) -> Result<()> {
    // Build the type registry before launching tsc so that expandSync
    // can resolve cross-module type references.
    ensure_type_registry_cache();
    let registry_path = TYPE_REGISTRY_CACHE_PATH.lock().unwrap().clone();
    let declarative_registry_path = DECLARATIVE_REGISTRY_CACHE_PATH.lock().unwrap().clone();

    // The tsc wrapper must be a JS script because it monkey-patches the TypeScript
    // CompilerHost.getSourceFile to expand macros on the fly — that hooking can only
    // happen in JS since it's patching the TS compiler API internals. We write it to
    // a temp file because `node` needs a file path to execute.
    let script = include_str!("../../../js/cli/tsc-wrapper.js");

    let mut temp_dir = std::env::temp_dir();
    temp_dir.push("macroforge-cli");
    fs::create_dir_all(&temp_dir)?;
    let script_path = temp_dir.join("tsc-wrapper.js");
    fs::write(&script_path, script)?;

    let project_arg = project
        .unwrap_or_else(|| PathBuf::from("tsconfig.json"))
        .to_string_lossy()
        .to_string();

    let mut cmd = std::process::Command::new("node");
    cmd.arg(script_path).arg(project_arg);
    if let Some(ref rp) = registry_path {
        cmd.env("MACROFORGE_TYPE_REGISTRY_PATH", rp);
    }
    if let Some(ref drp) = declarative_registry_path {
        cmd.env("MACROFORGE_DECLARATIVE_REGISTRY_PATH", drp);
    }
    let status = cmd.status().context("failed to run node tsc wrapper")?;

    if !status.success() {
        anyhow::bail!("tsc wrapper exited with status {}", status);
    }

    Ok(())
}

/// Runs svelte-check with macro expansion baked into file reads.
///
/// This function creates a Node.js script that patches `ts.sys.readFile` before
/// loading svelte-check. Since svelte-check uses TypeScript as a peer dependency
/// and Node.js caches modules, the patched `ts.sys.readFile` is shared with
/// svelte-check's internal TypeScript language service.
///
/// Files containing `@derive` are expanded before being passed to svelte-check.
///
/// # Arguments
///
/// * `workspace` - Optional workspace directory (defaults to cwd)
/// * `tsconfig` - Optional path to `tsconfig.json`
/// * `output` - Optional output format (human, human-verbose, machine, machine-verbose)
/// * `fail_on_warnings` - If true, exit with error on warnings
///
/// # Returns
///
/// Returns `Ok(())` if svelte-check passes, or an error with diagnostic details.
pub fn run_svelte_check_wrapper(
    workspace: Option<PathBuf>,
    tsconfig: Option<PathBuf>,
    output: Option<String>,
    fail_on_warnings: bool,
) -> Result<()> {
    // Build the type registry before launching svelte-check so that
    // expandSync can resolve cross-module type references (e.g., enum
    // fieldset variants defined in separate files).
    ensure_type_registry_cache();
    let registry_path = TYPE_REGISTRY_CACHE_PATH.lock().unwrap().clone();
    let declarative_registry_path = DECLARATIVE_REGISTRY_CACHE_PATH.lock().unwrap().clone();

    // Same pattern as tsc wrapper: must be JS because it patches svelte-check's
    // file reading to expand macros. Written to temp because `node` needs a file path.
    let script = include_str!("../../../js/cli/svelte-check-wrapper.js");

    let mut temp_dir = std::env::temp_dir();
    temp_dir.push("macroforge-cli");
    fs::create_dir_all(&temp_dir)?;
    let script_path = temp_dir.join("svelte-check-wrapper.js");
    fs::write(&script_path, script)?;

    let mut cmd = std::process::Command::new("node");
    cmd.arg(&script_path);

    // Pass the type registry path via environment variable so the JS
    // script can load it and feed it to expandSync.
    if let Some(ref rp) = registry_path {
        cmd.env("MACROFORGE_TYPE_REGISTRY_PATH", rp);
    }
    if let Some(ref drp) = declarative_registry_path {
        cmd.env("MACROFORGE_DECLARATIVE_REGISTRY_PATH", drp);
    }

    // Pass through CLI args for svelte-check to pick up
    if let Some(ref ws) = workspace {
        cmd.arg("--workspace").arg(ws);
    }
    if let Some(ref ts) = tsconfig {
        cmd.arg("--tsconfig").arg(ts);
    }
    if let Some(ref out) = output {
        cmd.arg("--output").arg(out);
    }
    if fail_on_warnings {
        cmd.arg("--fail-on-warnings");
    }

    let status = cmd
        .status()
        .context("failed to run node svelte-check wrapper")?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}
