//! # Macroforge CLI Binary
//!
//! This binary provides command-line utilities for working with Macroforge TypeScript macros.
//! It is designed for development workflows, enabling macro expansion and type checking
//! without requiring Node.js integration.
//!
//! ## Commands
//!
//! ### `macroforge expand`
//!
//! Expands macros in TypeScript/TSX files:
//!
//! ```bash
//! # Expand a single file
//! macroforge expand src/User.ts
//!
//! # Expand to specific output file
//! macroforge expand src/User.ts --out dist/User.js
//!
//! # Scan and expand all files in a directory
//! macroforge expand --scan src/
//!
//! # Use only built-in Rust macros (faster, no external dependencies)
//! macroforge expand src/User.ts --builtin-only
//!
//! # Print expanded output to stdout
//! macroforge expand src/User.ts --print
//! ```
//!
//! ### `macroforge tsc`
//!
//! Run TypeScript type checking with macro expansion baked in:
//!
//! ```bash
//! # Type check with default tsconfig.json
//! macroforge tsc
//!
//! # Type check with custom tsconfig
//! macroforge tsc -p tsconfig.build.json
//! ```
//!
//! ### `macroforge svelte-check`
//!
//! Run svelte-check with macro expansion baked into file reads:
//!
//! ```bash
//! # Type check a SvelteKit project
//! macroforge svelte-check
//!
//! # Explicit tsconfig
//! macroforge svelte-check --tsconfig tsconfig.json
//!
//! # Fail on warnings
//! macroforge svelte-check --fail-on-warnings
//!
//! # Machine-readable output
//! macroforge svelte-check --output machine
//! ```
//!
//! ## Configuration
//!
//! The CLI automatically searches for a configuration file starting from the input file's
//! directory, walking up to the nearest `package.json` (project root). Configuration files
//! are searched in this order:
//!
//! 1. `macroforge.config.ts`
//! 2. `macroforge.config.mts`
//! 3. `macroforge.config.js`
//! 4. `macroforge.config.mjs`
//! 5. `macroforge.config.cjs`
//!
//! ### Foreign Types
//!
//! Configuration files can define foreign type handlers for external types like Effect's
//! `DateTime`. When a matching type is found during expansion, the configured handlers
//! are used automatically:
//!
//! ```javascript
//! // macroforge.config.ts
//! import { DateTime } from "effect";
//!
//! export default {
//!   foreignTypes: {
//!     "DateTime.DateTime": {
//!       from: ["effect"],
//!       serialize: (v) => DateTime.formatIso(v),
//!       deserialize: (raw) => DateTime.unsafeFromDate(new Date(raw)),
//!       default: () => DateTime.unsafeNow()
//!     }
//!   }
//! }
//! ```
//!
//! See the [Configuration](crate::host::config) module for full documentation.
//!
//! ## Output File Naming
//!
//! By default, expanded files are written with `.expanded` inserted before the extension:
//!
//! - `foo.ts` → `foo.expanded.ts`
//! - `foo.svelte.ts` → `foo.expanded.svelte.ts`
//!
//! ## Exit Codes
//!
//! - `0` - Success
//! - `1` - Error during expansion
//! - `2` - No macros found in the input file (with `--quiet` suppresses output)
//!
//! ## Node.js Integration
//!
//! By default, the CLI uses Node.js to support external/custom macros from npm packages.
//! Use `--builtin-only` for fast expansion with only the built-in Rust macros (Debug, Clone,
//! PartialEq, Hash, Ord, PartialOrd, Default, Serialize, Deserialize).
//!
//! Both modes load and respect `macroforge.config.ts/js` for foreign type configuration.
//! The config is parsed natively using SWC, so foreign types work without Node.js.

use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use ignore::WalkBuilder;
use macroforge_ts::host::{MacroExpander, MacroExpansion};
use notify_debouncer_full::{new_debouncer, notify::RecursiveMode};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
    time::Duration,
};

/// Cached path to the type registry JSON file (built once per process via scanProjectSync).
static TYPE_REGISTRY_CACHE_PATH: Mutex<Option<String>> = Mutex::new(None);

/// Builds the type registry via a one-shot Node.js scanProjectSync call and
/// caches the result to a temp file. Subsequent calls are no-ops.
fn ensure_type_registry_cache() {
    let mut cached = TYPE_REGISTRY_CACHE_PATH.lock().unwrap();
    if cached.is_some() {
        return;
    }

    let scan_script = r#"
const { createRequire } = require('module');
const fs = require('fs');
const cwdRequire = createRequire(process.cwd() + '/package.json');

try {
  const macroforge = cwdRequire('macroforge');
  if (!macroforge.scanProjectSync) {
    process.exit(0);
  }
  const result = macroforge.scanProjectSync(process.cwd(), { exportedOnly: false });
  const outPath = process.argv[2];
  fs.writeFileSync(outPath, result.registryJson);
  console.log(JSON.stringify({ ok: true, types: result.typesFound, files: result.filesScanned }));
} catch (err) {
  console.error('Type scan failed:', err.message);
  process.exit(0);
}
"#;

    let mut temp_dir = std::env::temp_dir();
    temp_dir.push("macroforge-cli");
    let _ = fs::create_dir_all(&temp_dir);
    let script_path = temp_dir.join("scan-project-wrapper.js");
    let registry_path = temp_dir.join("type-registry.json");
    let _ = fs::write(&script_path, scan_script);

    let cwd = std::env::current_dir().unwrap_or_default();
    if let Ok(output) = std::process::Command::new("node")
        .arg(&script_path)
        .arg(&registry_path)
        .current_dir(&cwd)
        .output()
        && output.status.success()
        && registry_path.exists()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Ok(info) = serde_json::from_str::<serde_json::Value>(&stdout)
            && info.get("ok").and_then(|v| v.as_bool()).unwrap_or(false)
        {
            let types = info.get("types").and_then(|v| v.as_u64()).unwrap_or(0);
            let files = info.get("files").and_then(|v| v.as_u64()).unwrap_or(0);
            eprintln!(
                "[macroforge] Type scan: {} types from {} files",
                types, files
            );
            *cached = Some(registry_path.to_string_lossy().to_string());
            return;
        }
    }

    // Scan failed or not available — continue without type registry
    *cached = None;
}

/// Command-line interface for Macroforge TypeScript macro utilities.
///
/// Provides three main commands:
/// - `expand` - Expand macros in TypeScript files
/// - `tsc` - Run TypeScript type checking with macro expansion
/// - `svelte-check` - Run svelte-check with macro expansion
#[derive(Parser)]
#[command(name = "macroforge", about = "TypeScript macro development utilities")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Available CLI subcommands.
#[derive(Subcommand)]
enum Command {
    /// Expand macros in a TypeScript file or directory.
    ///
    /// By default, uses Node.js for full macro support including external macros.
    /// Use `--builtin-only` for faster expansion with only built-in Rust macros.
    Expand {
        /// Path to the TypeScript/TSX file or directory to expand
        input: Option<PathBuf>,
        /// Optional path to write the transformed JS/TS output
        #[arg(long)]
        out: Option<PathBuf>,
        /// Optional path to write the generated .d.ts surface
        #[arg(long = "types-out")]
        types_out: Option<PathBuf>,
        /// Print expansion result to stdout even if --out is specified
        #[arg(long)]
        print: bool,
        /// Use only built-in Rust macros (faster, but no external macro support)
        #[arg(long)]
        builtin_only: bool,
        /// Suppress output when no macros are found (exit silently with code 2)
        #[arg(long, short = 'q')]
        quiet: bool,
        /// Scan directory for TypeScript files with macros (uses input as root, or cwd if not specified)
        #[arg(long)]
        scan: bool,
        /// Include files ignored by .gitignore when scanning
        #[arg(long)]
        include_ignored: bool,
    },
    /// Run tsc with macro expansion baked into file reads (tsc --noEmit semantics)
    Tsc {
        /// Path to tsconfig.json (defaults to tsconfig.json in cwd)
        #[arg(long, short = 'p')]
        project: Option<PathBuf>,
    },
    /// Run svelte-check with macro expansion baked into file reads
    SvelteCheck {
        /// Path to the workspace directory (defaults to cwd)
        #[arg(long)]
        workspace: Option<PathBuf>,
        /// Path to tsconfig.json (defaults to tsconfig.json in cwd)
        #[arg(long)]
        tsconfig: Option<PathBuf>,
        /// Output format: human, human-verbose, machine, machine-verbose
        #[arg(long)]
        output: Option<String>,
        /// Fail on warnings in addition to errors
        #[arg(long)]
        fail_on_warnings: bool,
    },
    /// Watch source files and maintain a .macroforge/cache for fast Vite dev mode.
    ///
    /// Expands macros in all TypeScript files and writes results to .macroforge/cache/.
    /// When a file changes, only that file is re-expanded. When the config changes,
    /// all files are re-expanded. Use with `vite dev` for instant macro expansion.
    Watch {
        /// Root directory to watch (defaults to cwd)
        root: Option<PathBuf>,
        /// Use only built-in Rust macros (faster, no Node.js)
        #[arg(long)]
        builtin_only: bool,
        /// Debounce interval in milliseconds
        #[arg(long, default_value = "100")]
        debounce_ms: u64,
    },
    /// Build the .macroforge/cache once and exit.
    ///
    /// Same as `watch` but without the file-watching loop — expands all TypeScript
    /// files, writes the cache, then exits. Useful in CI or as a pre-build step.
    Cache {
        /// Root directory to cache (defaults to cwd)
        root: Option<PathBuf>,
        /// Use only built-in Rust macros (faster, no Node.js)
        #[arg(long)]
        builtin_only: bool,
    },
    /// Delete the .macroforge/cache directory and rebuild from scratch.
    ///
    /// Equivalent to manually deleting the cache and running `cache`.
    /// Useful when the cache is corrupted or you want a guaranteed clean state.
    Refresh {
        /// Root directory (defaults to cwd)
        root: Option<PathBuf>,
        /// Use only built-in Rust macros (faster, no Node.js)
        #[arg(long)]
        builtin_only: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Expand {
            input,
            out,
            types_out,
            print,
            builtin_only,
            quiet,
            scan,
            include_ignored,
        } => {
            if scan {
                let root = input.unwrap_or_else(|| PathBuf::from("."));
                scan_and_expand(root, builtin_only, include_ignored)
            } else {
                let input = input.ok_or_else(|| {
                    anyhow!("input file required (use --scan to scan a directory)")
                })?;

                // If input is a directory, treat it as --scan
                if input.is_dir() {
                    scan_and_expand(input, builtin_only, include_ignored)
                } else {
                    expand_file(input, out, types_out, print, builtin_only, quiet)
                }
            }
        }
        Command::Tsc { project } => run_tsc_wrapper(project),
        Command::SvelteCheck {
            workspace,
            tsconfig,
            output,
            fail_on_warnings,
        } => run_svelte_check_wrapper(workspace, tsconfig, output, fail_on_warnings),
        Command::Watch {
            root,
            builtin_only,
            debounce_ms,
        } => run_watch(root, builtin_only, debounce_ms),
        Command::Cache { root, builtin_only } => run_cache(root, builtin_only),
        Command::Refresh { root, builtin_only } => run_refresh(root, builtin_only),
    }
}

/// Recursively scans a directory for TypeScript files and expands macros in each.
///
/// This function walks the directory tree, respecting `.gitignore` rules (unless
/// `include_ignored` is true), and attempts to expand each `.ts` and `.tsx` file.
///
/// Files are skipped if:
/// - They are `.d.ts` declaration files
/// - They already contain `.expanded.` in their name
/// - They are in directories ignored by `.gitignore` (unless `include_ignored`)
///
/// # Arguments
///
/// * `root` - The root directory to start scanning from
/// * `builtin_only` - If true, use only built-in Rust macros (faster)
/// * `include_ignored` - If true, also process files ignored by `.gitignore`
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if the scan fails.
fn scan_and_expand(root: PathBuf, builtin_only: bool, include_ignored: bool) -> Result<()> {
    use rayon::prelude::*;

    let root = root.canonicalize().unwrap_or(root);
    eprintln!("[macroforge] scanning {}", root.display());

    // Phase 1: Collect all files (sequential walk)
    let mut files: Vec<PathBuf> = Vec::new();
    let walker = WalkBuilder::new(&root)
        .hidden(false)
        .git_ignore(!include_ignored)
        .git_global(false)
        .git_exclude(false)
        .build();

    for entry in walker.flatten() {
        let path = entry.path();

        let is_ts_file = path
            .extension()
            .is_some_and(|ext| ext == "ts" || ext == "tsx")
            && !path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .ends_with(".d.ts");

        if !is_ts_file || !path.is_file() {
            continue;
        }

        let filename = path.file_name().unwrap_or_default().to_string_lossy();
        if filename.contains(".expanded.") {
            continue;
        }

        files.push(path.to_path_buf());
    }

    let files_found = files.len();

    // Phase 2: Expand in parallel
    let pool = if builtin_only {
        rayon::ThreadPoolBuilder::new().build()?
    } else {
        rayon::ThreadPoolBuilder::new().num_threads(4).build()?
    };

    let results: Vec<_> = pool.install(|| {
        files
            .par_iter()
            .map(|path| {
                let result =
                    try_expand_file(path.clone(), None, None, false, builtin_only, true);
                (path.clone(), result)
            })
            .collect()
    });

    // Phase 3: Report results (sequential)
    let mut files_expanded = 0;
    for (path, result) in &results {
        match result {
            Ok(true) => files_expanded += 1,
            Ok(false) => {}
            Err(e) => {
                eprintln!(
                    "[macroforge] error expanding {}: {}",
                    path.strip_prefix(&root).unwrap_or(path).display(),
                    e
                );
            }
        }
    }

    eprintln!(
        "[macroforge] scan complete: {} files found, {} expanded",
        files_found, files_expanded
    );

    Ok(())
}

/// Expands macros in a single TypeScript file.
///
/// This is the main entry point for single-file expansion. It handles output
/// routing (to file and/or stdout) and quiet mode behavior.
///
/// # Arguments
///
/// * `input` - Path to the input TypeScript file
/// * `out` - Optional path for the expanded output (default: `input.expanded.ts`)
/// * `types_out` - Optional path for the `.d.ts` type output
/// * `print` - If true, also print expanded code to stdout
/// * `builtin_only` - If true, use only built-in Rust macros
/// * `quiet` - If true, suppress output when no macros are found
///
/// # Exit Codes
///
/// Calls `std::process::exit(2)` if no macros are found and not in quiet mode.
fn expand_file(
    input: PathBuf,
    out: Option<PathBuf>,
    types_out: Option<PathBuf>,
    print: bool,
    builtin_only: bool,
    quiet: bool,
) -> Result<()> {
    match try_expand_file(input.clone(), out, types_out, print, builtin_only, false)? {
        true => Ok(()),
        false => {
            if !quiet {
                eprintln!("[macroforge] no macros found in {}", input.display());
            }
            std::process::exit(2);
        }
    }
}

/// Attempts to expand macros in a file using the built-in Rust expander.
///
/// This function uses the Rust-native `MacroExpander` for fast expansion
/// when `builtin_only` is true. Otherwise, it delegates to Node.js for
/// full external macro support.
///
/// # Arguments
///
/// * `input` - Path to the input TypeScript file
/// * `out` - Optional output path for expanded code
/// * `types_out` - Optional output path for type declarations
/// * `print` - Whether to print output to stdout
/// * `builtin_only` - Whether to use only built-in macros
/// * `is_scanning` - Whether this is part of a directory scan (affects error output)
///
/// # Returns
///
/// - `Ok(true)` - Macros were found and successfully expanded
/// - `Ok(false)` - No macros were found in the file
/// - `Err(...)` - An error occurred during expansion
fn try_expand_file(
    input: PathBuf,
    out: Option<PathBuf>,
    types_out: Option<PathBuf>,
    print: bool,
    builtin_only: bool,
    is_scanning: bool,
) -> Result<bool> {
    // Default: use Node.js for full macro support (including external macros)
    // With --builtin-only: use fast Rust expander (built-in macros only)
    if !builtin_only {
        return try_expand_file_via_node(input, out, types_out, print, is_scanning);
    }

    try_expand_file_builtin(input, out, types_out, print)
}

/// Expands macros using only the built-in Rust expander.
///
/// This is the fast path that doesn't require Node.js. It only supports
/// the built-in macros (Debug, Clone, PartialEq, Hash, Ord, PartialOrd,
/// Default, Serialize, Deserialize).
///
/// ## Configuration Loading
///
/// This function searches for and loads `macroforge.config.ts/js` to enable
/// foreign type handlers. The config is parsed natively using SWC without
/// requiring Node.js.
fn try_expand_file_builtin(
    input: PathBuf,
    out: Option<PathBuf>,
    types_out: Option<PathBuf>,
    print: bool,
) -> Result<bool> {
    use macroforge_ts::host::MacroforgeConfigLoader;

    // Load config if available (for foreign types support).
    // Foreign types are set on the registry before expansion; source imports
    // are built from the AST during prepare_expansion_context / expand_source.
    if let Ok(Some(config)) = MacroforgeConfigLoader::find_from_path(&input) {
        macroforge_ts::host::set_foreign_types(config.foreign_types.clone());
    }

    let source = fs::read_to_string(&input)
        .with_context(|| format!("failed to read {}", input.display()))?;

    let expander = MacroExpander::new().context("failed to initialize macro expander")?;

    let expansion = expander
        .expand_source(&source, &input.display().to_string())
        .map_err(|err| anyhow!(format!("{err:?}")))?;

    // Single cleanup
    macroforge_ts::host::clear_registry();
    macroforge_ts::host::clear_foreign_types();

    if !expansion.changed {
        return Ok(false);
    }

    emit_diagnostics(&expansion, &source, &input);
    emit_runtime_output(&expansion, &input, out.as_ref(), print)?;
    emit_type_output(&expansion, &input, types_out.as_ref(), print)?;

    Ok(true)
}

// extract_import_sources_from_code deleted — absorbed into ImportRegistry::from_module

/// Attempts to expand macros by invoking Node.js with the macroforge npm package.
///
/// This function writes a temporary Node.js script that calls `macroforge.expandSync()`,
/// then parses the JSON result. This approach supports external macros from npm packages
/// but requires Node.js and the macroforge package to be installed.
///
/// ## Configuration Loading
///
/// The function automatically searches for a `macroforge.config.ts/js` file starting from
/// the input file's directory, walking up to the nearest `package.json`. If found, the
/// configuration is loaded and passed to `expandSync`, enabling foreign type handlers.
///
/// ## Module Resolution
///
/// The function tries to resolve macroforge from:
/// 1. The current working directory
/// 2. The input file's parent directory
///
/// # Arguments
///
/// * `input` - Path to the input TypeScript file
/// * `out` - Optional output path for expanded code
/// * `types_out` - Optional output path for type declarations
/// * `print` - Whether to print output to stdout
/// * `is_scanning` - Whether this is part of a directory scan (affects warning output)
///
/// # Returns
///
/// - `Ok(true)` - Macros were found and successfully expanded
/// - `Ok(false)` - No macros were found (empty `generatedRegions`)
/// - `Err(...)` - Node.js execution failed or macroforge not found
fn try_expand_file_via_node(
    input: PathBuf,
    out: Option<PathBuf>,
    types_out: Option<PathBuf>,
    print: bool,
    is_scanning: bool,
) -> Result<bool> {
    let script = r#"
const { createRequire } = require('module');
const fs = require('fs');
const path = require('path');

// Create require from the cwd to resolve modules properly
const cwdRequire = createRequire(process.cwd() + '/package.json');

let expandSync, loadConfig, clearConfigCache;
try {
  const macroforge = cwdRequire('macroforge');
  expandSync = macroforge.expandSync;
  loadConfig = macroforge.loadConfig;
  clearConfigCache = macroforge.clearConfigCache;
} catch {
  // macroforge not installed - output fallback marker for Rust to detect
  console.log(JSON.stringify({ fallback: true }));
  process.exit(0);
}

const inputPath = process.argv[2];
const code = fs.readFileSync(inputPath, 'utf8');

// Search for macroforge.config.ts/js starting from input file's directory
const CONFIG_FILES = [
  'macroforge.config.ts',
  'macroforge.config.mts',
  'macroforge.config.js',
  'macroforge.config.mjs',
  'macroforge.config.cjs',
];

function findConfigFile(startDir) {
  let dir = startDir;
  while (dir) {
    for (const configName of CONFIG_FILES) {
      const configPath = path.join(dir, configName);
      if (fs.existsSync(configPath)) {
        return configPath;
      }
    }
    // Stop at package.json (project root)
    if (fs.existsSync(path.join(dir, 'package.json'))) {
      break;
    }
    const parent = path.dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  return null;
}

try {
  const inputDir = path.dirname(path.resolve(inputPath));
  const configPath = findConfigFile(inputDir);

  let options = {};
  if (configPath) {
    // Load config and pass the path to expandSync
    const configContent = fs.readFileSync(configPath, 'utf8');
    loadConfig(configContent, configPath);
    options.configPath = configPath;
  }

  const result = expandSync(code, inputPath, options);

  // Output as JSON for the Rust CLI to parse
  console.log(JSON.stringify({
    code: result.code,
    types: result.types,
    diagnostics: result.diagnostics || [],
    sourceMapping: result.sourceMapping || null
  }));
} catch (err) {
  console.error('Error:', err.message);
  process.exit(1);
}
"#;

    let mut temp_dir = std::env::temp_dir();
    temp_dir.push("macroforge-cli");
    fs::create_dir_all(&temp_dir)?;
    let script_path = temp_dir.join("expand-wrapper.js");
    fs::write(&script_path, script)?;

    // Try from cwd first, then from input file's directory if macroforge not found
    let cwd = std::env::current_dir()?;
    let input_dir = input.parent().unwrap_or(Path::new(".")).to_path_buf();
    let dirs_to_try = [cwd, input_dir];

    let mut last_error = String::new();
    let mut output_result = None;

    for dir in &dirs_to_try {
        let output = std::process::Command::new("node")
            .arg(&script_path)
            .arg(&input)
            .current_dir(dir)
            .output()
            .context("failed to run node expand wrapper")?;

        if output.status.success() {
            output_result = Some(output);
            break;
        }

        let stderr = String::from_utf8_lossy(&output.stderr);
        // If it's a MODULE_NOT_FOUND error for macroforge, try the next directory
        if stderr.contains("Cannot find module 'macroforge'") {
            last_error = stderr.to_string();
            continue;
        }

        // For other errors, fail immediately
        anyhow::bail!("node expansion failed: {}", stderr);
    }

    let output = output_result.ok_or_else(|| anyhow!("node expansion failed: {}", last_error))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let result: serde_json::Value =
        serde_json::from_str(&stdout).context("failed to parse expansion result from node")?;

    // Check for fallback marker (macroforge npm module not installed)
    if result
        .get("fallback")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        // Fall back to built-in expansion
        if !is_scanning {
            eprintln!(
                "[macroforge] warning: macroforge npm module not found, using built-in expander for {}",
                input.display()
            );
        }
        return try_expand_file_builtin(input, out, types_out, print);
    }

    let code = result["code"]
        .as_str()
        .ok_or_else(|| anyhow!("missing 'code' in expansion result"))?;
    let types = result["types"].as_str();

    // Check if any macros were actually expanded by looking at generatedRegions
    let has_expansions = result["sourceMapping"]
        .get("generatedRegions")
        .and_then(|r| r.as_array())
        .map(|arr| !arr.is_empty())
        .unwrap_or(false);

    if !has_expansions {
        return Ok(false);
    }

    // Write outputs if macros were expanded
    let out_path = out.unwrap_or_else(|| get_expanded_path(&input));
    write_file(&out_path, code)?;
    println!(
        "[macroforge] wrote expanded output for {} to {}",
        input.display(),
        out_path.display()
    );

    if print {
        println!("// --- {} (expanded) ---", input.display());
        println!("{}", code);
    }

    if let Some(types_str) = types {
        if let Some(types_path) = types_out {
            write_file(&types_path, types_str)?;
            println!(
                "[macroforge] wrote type output for {} to {}",
                input.display(),
                types_path.display()
            );
        } else if print {
            println!("// --- {} (.d.ts) ---", input.display());
            println!("{}", types_str);
        }
    }

    // Print diagnostics
    if let Some(diags) = result["diagnostics"].as_array() {
        for diag in diags {
            if let (Some(level), Some(message)) = (diag["level"].as_str(), diag["message"].as_str())
            {
                eprintln!("[macroforge] {} at {}: {}", level, input.display(), message);
            }
        }
    }

    Ok(true)
}

/// Writes the expanded runtime code to a file and optionally prints to stdout.
///
/// # Arguments
///
/// * `result` - The macro expansion result containing the generated code
/// * `input` - The original input file path (for display purposes)
/// * `explicit_out` - Optional explicit output path (defaults to `.expanded.ts`)
/// * `should_print` - Whether to also print the code to stdout
fn emit_runtime_output(
    result: &MacroExpansion,
    input: &Path,
    explicit_out: Option<&PathBuf>,
    should_print: bool,
) -> Result<()> {
    let code = &result.code;
    let out_path = explicit_out
        .cloned()
        .unwrap_or_else(|| get_expanded_path(input));
    write_file(&out_path, code)?;
    println!(
        "[macroforge] wrote expanded output for {} to {}",
        input.display(),
        out_path.display()
    );
    if should_print {
        println!("// --- {} (expanded) ---", input.display());
        println!("{code}");
    }
    Ok(())
}

/// Writes the generated type declarations (`.d.ts`) to a file and optionally prints to stdout.
///
/// If no explicit output path is provided and `print` is false, the type output
/// is silently discarded.
///
/// # Arguments
///
/// * `result` - The macro expansion result containing the type declarations
/// * `input` - The original input file path (for display purposes)
/// * `explicit_out` - Optional explicit output path for `.d.ts`
/// * `print` - Whether to print type declarations to stdout
fn emit_type_output(
    result: &MacroExpansion,
    input: &Path,
    explicit_out: Option<&PathBuf>,
    print: bool,
) -> Result<()> {
    let Some(types) = result.type_output.as_ref() else {
        return Ok(());
    };

    if let Some(path) = explicit_out {
        write_file(path, types)?;
        println!(
            "[macroforge] wrote type output for {} to {}",
            input.display(),
            path.display()
        );
    } else if print {
        println!("// --- {} (.d.ts) ---", input.display());
        println!("{types}");
    }
    Ok(())
}

/// Writes content to a file, creating parent directories as needed.
fn write_file(path: &PathBuf, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(path, contents).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
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
fn run_tsc_wrapper(project: Option<PathBuf>) -> Result<()> {
    // Build the type registry before launching tsc so that expandSync
    // can resolve cross-module type references.
    ensure_type_registry_cache();
    let registry_path = TYPE_REGISTRY_CACHE_PATH.lock().unwrap().clone();

    // Write a temporary Node.js script that wraps tsc and expands macros on file load
    let script = r#"
const { createRequire } = require('module');
const fs = require('fs');
const cwdRequire = createRequire(process.cwd() + '/package.json');
const ts = cwdRequire('typescript');
const macros = cwdRequire('macroforge');
const path = require('path');

const projectArg = process.argv[2] || 'tsconfig.json';
const configPath = ts.findConfigFile(process.cwd(), ts.sys.fileExists, projectArg);
if (!configPath) {
  console.error(`[macroforge] tsconfig not found: ${projectArg}`);
  process.exit(1);
}

// Find and load macroforge config for foreign types
const CONFIG_FILES = [
  'macroforge.config.ts',
  'macroforge.config.mts',
  'macroforge.config.js',
  'macroforge.config.mjs',
  'macroforge.config.cjs',
];
let macroConfigPath = null;
let currentDir = process.cwd();
while (true) {
  for (const filename of CONFIG_FILES) {
    const candidate = path.join(currentDir, filename);
    if (fs.existsSync(candidate)) {
      macroConfigPath = candidate;
      break;
    }
  }
  if (macroConfigPath) break;
  // Stop at package.json boundary
  if (fs.existsSync(path.join(currentDir, 'package.json'))) break;
  const parent = path.dirname(currentDir);
  if (parent === currentDir) break;
  currentDir = parent;
}

// Load the config if found (caches foreign types in native plugin)
if (macroConfigPath) {
  try {
    const configContent = fs.readFileSync(macroConfigPath, 'utf8');
    macros.loadConfig(configContent, macroConfigPath);
  } catch (e) {
    // Config load failed, continue without foreign types
  }
}

// Load pre-built type registry if available
const typeRegistryPath = process.env.MACROFORGE_TYPE_REGISTRY_PATH;
let typeRegistryJson = undefined;
if (typeRegistryPath) {
  try {
    typeRegistryJson = fs.readFileSync(typeRegistryPath, 'utf8');
  } catch {}
}

const configFile = ts.readConfigFile(configPath, ts.sys.readFile);
if (configFile.error) {
  console.error(ts.formatDiagnostic(configFile.error, {
    getCanonicalFileName: (f) => f,
    getCurrentDirectory: ts.sys.getCurrentDirectory,
    getNewLine: () => ts.sys.newLine
  }));
  process.exit(1);
}

const parsed = ts.parseJsonConfigFileContent(
  configFile.config,
  ts.sys,
  path.dirname(configPath)
);

const options = { ...parsed.options, noEmit: true };
const formatHost = {
  getCanonicalFileName: (f) => f,
  getCurrentDirectory: ts.sys.getCurrentDirectory,
  getNewLine: () => ts.sys.newLine,
};

const host = ts.createCompilerHost(options);
const origGetSourceFile = host.getSourceFile.bind(host);
host.getSourceFile = (fileName, languageVersion, ...rest) => {
  try {
    if (
      (fileName.endsWith('.ts') || fileName.endsWith('.tsx')) &&
      !fileName.endsWith('.d.ts')
    ) {
      const sourceText = ts.sys.readFile(fileName);
      if (sourceText && sourceText.includes('@derive')) {
        const expandOpts = {};
        if (macroConfigPath) expandOpts.configPath = macroConfigPath;
        if (typeRegistryJson) expandOpts.typeRegistryJson = typeRegistryJson;
        const expanded = macros.expandSync(sourceText, fileName, expandOpts);
        const text = expanded.code || sourceText;
        return ts.createSourceFile(fileName, text, languageVersion, true);
      }
    }
  } catch (e) {
    // fall through to original host
  }
  return origGetSourceFile(fileName, languageVersion, ...rest);
};

const program = ts.createProgram(parsed.fileNames, options, host);
const diagnostics = ts.getPreEmitDiagnostics(program);
if (diagnostics.length) {
  diagnostics.forEach((d) => {
    const msg = ts.formatDiagnostic(d, formatHost);
    console.error(msg.trimEnd());
  });
}
const hasError = diagnostics.some((d) => d.category === ts.DiagnosticCategory.Error);
process.exit(hasError ? 1 : 0);
"#;

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
fn run_svelte_check_wrapper(
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

    let script = r#"
const { createRequire } = require('module');
const fs = require('fs');
const path = require('path');
const cwdRequire = createRequire(process.cwd() + '/package.json');

// --- 1. Load TypeScript and macroforge from the project ---
let ts, macros;
try {
  ts = cwdRequire('typescript');
} catch {
  console.error('[macroforge] error: typescript is not installed in this project');
  process.exit(1);
}
try {
  macros = cwdRequire('macroforge');
} catch {
  console.error('[macroforge] error: macroforge is not installed in this project');
  process.exit(1);
}

// --- 2. Find and load macroforge config ---
const CONFIG_FILES = [
  'macroforge.config.ts',
  'macroforge.config.mts',
  'macroforge.config.js',
  'macroforge.config.mjs',
  'macroforge.config.cjs',
];
let macroConfigPath = null;
let currentDir = process.cwd();
while (true) {
  for (const filename of CONFIG_FILES) {
    const candidate = path.join(currentDir, filename);
    if (fs.existsSync(candidate)) {
      macroConfigPath = candidate;
      break;
    }
  }
  if (macroConfigPath) break;
  if (fs.existsSync(path.join(currentDir, 'package.json'))) break;
  const parent = path.dirname(currentDir);
  if (parent === currentDir) break;
  currentDir = parent;
}

if (macroConfigPath) {
  try {
    const configContent = fs.readFileSync(macroConfigPath, 'utf8');
    macros.loadConfig(configContent, macroConfigPath);
  } catch (e) {
    // Config load failed, continue without foreign types
  }
}

// --- 2b. Load pre-built type registry if available ---
const typeRegistryPath = process.env.MACROFORGE_TYPE_REGISTRY_PATH;
let typeRegistryJson = undefined;
if (typeRegistryPath) {
  try {
    typeRegistryJson = fs.readFileSync(typeRegistryPath, 'utf8');
  } catch {}
}

// --- 3. Patch ts.sys.readFile to expand macros ---
const origReadFile = ts.sys.readFile.bind(ts.sys);
ts.sys.readFile = (filePath, encoding) => {
  const content = origReadFile(filePath, encoding);
  if (content == null) return content;
  try {
    if (
      (filePath.endsWith('.ts') || filePath.endsWith('.tsx')) &&
      !filePath.endsWith('.d.ts') &&
      content.includes('@derive')
    ) {
      const expandOpts = {};
      if (macroConfigPath) expandOpts.configPath = macroConfigPath;
      if (typeRegistryJson) expandOpts.typeRegistryJson = typeRegistryJson;
      const expanded = macros.expandSync(content, filePath, expandOpts);
      return expanded.code || content;
    }
  } catch (e) {
    // Expansion failed, fall through to original content
  }
  return content;
};

// --- 4. Forward to svelte-check ---
// Rewrite process.argv so sade (svelte-check's CLI parser) sees the right command.
// argv[0] = node, argv[1] = 'svelte-check', rest = flags
const args = ['svelte-check'];
// The Rust side passes extra CLI args starting at process.argv[2]
for (let i = 2; i < process.argv.length; i++) {
  args.push(process.argv[i]);
}
process.argv = [process.argv[0], ...args];

try {
  cwdRequire('svelte-check');
} catch (e) {
  if (e.code === 'MODULE_NOT_FOUND') {
    console.error('[macroforge] error: svelte-check is not installed in this project');
    console.error('[macroforge] install it with: npm install --save-dev svelte-check');
    process.exit(1);
  }
  throw e;
}
"#;

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

/// Prints macro expansion diagnostics (warnings, errors) to stderr.
///
/// Each diagnostic is formatted with its level, file location, and message.
fn emit_diagnostics(expansion: &MacroExpansion, source: &str, input: &Path) {
    if expansion.diagnostics.is_empty() {
        return;
    }

    for diag in &expansion.diagnostics {
        let (line, col) = diag
            .span
            .map(|s| offset_to_line_col(source, s.start as usize))
            .unwrap_or((1, 1));
        eprintln!(
            "[macroforge] {} at {}:{}:{}: {}",
            format!("{:?}", diag.level).to_lowercase(),
            input.display(),
            line,
            col,
            diag.message
        );
    }
}

/// Converts a byte offset in source code to a (line, column) position.
///
/// Lines and columns are 1-indexed for user-friendly display.
fn offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    for (idx, ch) in source.char_indices() {
        if idx >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}

/// Generate an expanded output path, inserting `.expanded` as the first extension.
/// Examples: `foo.svelte.ts` → `foo.expanded.svelte.ts`, `foo.ts` → `foo.expanded.ts`
fn get_expanded_path(input: &Path) -> PathBuf {
    let dir = input.parent().unwrap_or_else(|| Path::new("."));
    let basename = input.file_name().unwrap_or_default().to_string_lossy();

    if let Some(first_dot) = basename.find('.') {
        let name_without_ext = &basename[..first_dot];
        let extensions = &basename[first_dot..];
        dir.join(format!("{}.expanded{}", name_without_ext, extensions))
    } else {
        dir.join(format!("{}.expanded", basename))
    }
}

// =============================================================================
// Watch command — cache structs and implementation
// =============================================================================

/// Cache manifest stored at `.macroforge/cache/manifest.json`.
///
/// Tracks expanded file hashes so the Vite plugin (and subsequent watch runs)
/// can skip re-expansion for unchanged files.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct CacheManifest {
    /// Macroforge crate version — full invalidation on upgrade.
    version: String,
    /// SHA-256 of the macroforge config file content (or `"none"`).
    config_hash: String,
    /// Whether this cache was built with `--builtin-only` (no external macros).
    #[serde(default)]
    builtin_only: bool,
    /// Per-file entries keyed by path relative to project root.
    entries: HashMap<String, CacheEntry>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct CacheEntry {
    /// SHA-256 of the source file content.
    source_hash: String,
    /// Whether the file contained macros.
    has_macros: bool,
    /// SHA-256 of the whitespace-normalized source content.
    /// Used to detect whitespace-only changes in watch mode.
    #[serde(default)]
    normalized_hash: String,
}

/// Computes SHA-256 of a byte slice, returned as lowercase hex.
fn content_hash(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let result = hasher.finalize();
    result.iter().fold(String::with_capacity(64), |mut s, b| {
        use std::fmt::Write;
        let _ = write!(s, "{b:02x}");
        s
    })
}

/// Computes SHA-256 of whitespace-normalized content.
///
/// Normalization: trim trailing whitespace per line, collapse runs of blank
/// lines into a single newline, trim leading/trailing blank lines. Leading
/// indentation is preserved (meaningful in TS template literals).
fn normalized_content_hash(content: &str) -> String {
    let mut normalized = String::with_capacity(content.len());
    let mut prev_blank = false;

    for line in content.lines() {
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            if !prev_blank && !normalized.is_empty() {
                normalized.push('\n');
            }
            prev_blank = true;
        } else {
            if !normalized.is_empty() {
                normalized.push('\n');
            }
            normalized.push_str(trimmed);
            prev_blank = false;
        }
    }

    // Trim trailing blank lines so "foo\n" and "foo\n\n\n" hash the same
    let normalized = normalized.trim_end_matches('\n');
    content_hash(normalized.as_bytes())
}

/// Config file names searched in order of precedence.
const CONFIG_FILE_NAMES: &[&str] = &[
    "macroforge.config.ts",
    "macroforge.config.mts",
    "macroforge.config.js",
    "macroforge.config.mjs",
    "macroforge.config.cjs",
];

/// Computes a hash of the macroforge config file for cache invalidation.
fn compute_config_hash(root: &Path) -> String {
    for name in CONFIG_FILE_NAMES {
        let path = root.join(name);
        if let Ok(content) = fs::read(&path) {
            return content_hash(&content);
        }
    }
    "none".to_string()
}

impl CacheManifest {
    fn new(version: String, config_hash: String, builtin_only: bool) -> Self {
        Self {
            version,
            config_hash,
            builtin_only,
            entries: HashMap::new(),
        }
    }

    fn load(cache_dir: &Path) -> Option<Self> {
        let manifest_path = cache_dir.join("manifest.json");
        let content = fs::read_to_string(manifest_path).ok()?;
        serde_json::from_str(&content).ok()
    }

    /// Atomically saves the manifest via write-to-tmp + rename.
    fn save(&self, cache_dir: &Path) -> Result<()> {
        fs::create_dir_all(cache_dir)?;
        let manifest_path = cache_dir.join("manifest.json");
        let json = serde_json::to_string_pretty(self)?;

        // Atomic write: temp file in same directory, then rename
        let tmp_path = cache_dir.join(".manifest.json.tmp");
        fs::write(&tmp_path, &json)?;
        fs::rename(&tmp_path, &manifest_path)?;
        Ok(())
    }
}

/// Writes expanded code to `<cache_dir>/<rel_path>.cache`.
fn write_cache_file(cache_dir: &Path, rel_path: &str, expanded_code: &str) -> Result<()> {
    let cache_path = cache_dir.join(format!("{rel_path}.cache"));
    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&cache_path, expanded_code)?;
    Ok(())
}

/// Returns true if `path` is a `.ts` or `.tsx` file that should be processed.
fn is_watchable_ts_file(path: &Path, root: &Path) -> bool {
    let ext = path.extension().and_then(|e| e.to_str());
    if !matches!(ext, Some("ts" | "tsx")) {
        return false;
    }
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    if name.ends_with(".d.ts") || name.contains(".expanded.") {
        return false;
    }
    let rel = path.strip_prefix(root).unwrap_or(path).to_string_lossy();
    if rel.contains("node_modules") || rel.contains(".macroforge") {
        return false;
    }
    true
}

/// Collects all watchable TypeScript files under `root`.
fn collect_watch_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let walker = WalkBuilder::new(root)
        .hidden(false)
        .git_ignore(true)
        .git_global(false)
        .git_exclude(false)
        .build();

    for entry in walker.flatten() {
        let path = entry.path();
        if path.is_file() && is_watchable_ts_file(path, root) {
            files.push(path.to_path_buf());
        }
    }
    files
}

/// Expands a single file for caching purposes.
///
/// Returns `Ok(Some(expanded_code))` if macros were found and expanded,
/// `Ok(None)` if no macros present, or `Err` on failure.
/// Check if source code contains `@derive(` as a standalone JSDoc directive.
///
/// Only matches `@derive(` when it appears at the start of a JSDoc line (after
/// stripping `/**`, `*/`, `*`, and whitespace). Skips `@derive` embedded in prose
/// (e.g., `"result from @derive(Deserialize)"`) and inside fenced code blocks.
fn has_macro_annotations(source: &str) -> bool {
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

fn expand_for_cache(path: &Path, source: &str, builtin_only: bool) -> Result<Option<String>> {
    // Quick check: skip files without @derive (ignoring fenced code blocks in docs)
    if !has_macro_annotations(source) {
        return Ok(None);
    }

    if builtin_only {
        use macroforge_ts::host::MacroforgeConfigLoader;

        if let Ok(Some(config)) = MacroforgeConfigLoader::find_from_path(path) {
            macroforge_ts::host::set_foreign_types(config.foreign_types.clone());
        }

        let expander = MacroExpander::new().context("failed to initialize macro expander")?;
        let expansion = expander
            .expand_source(source, &path.display().to_string())
            .map_err(|err| anyhow!(format!("{err:?}")))?;

        macroforge_ts::host::clear_registry();
        macroforge_ts::host::clear_foreign_types();

        if !expansion.changed {
            return Ok(None);
        }

        Ok(Some(expansion.code))
    } else {
        expand_for_cache_via_node(path, source)
    }
}

/// Expands a file via Node.js for caching, returning the expanded code.
fn expand_for_cache_via_node(path: &Path, source: &str) -> Result<Option<String>> {
    // Ensure type registry has been built (once per process)
    ensure_type_registry_cache();

    let script = r#"
const { createRequire } = require('module');
const fs = require('fs');
const path = require('path');
const cwdRequire = createRequire(process.cwd() + '/package.json');

let expandSync, loadConfig;
try {
  const macroforge = cwdRequire('macroforge');
  expandSync = macroforge.expandSync;
  loadConfig = macroforge.loadConfig;
} catch {
  console.log(JSON.stringify({ fallback: true }));
  process.exit(0);
}

const inputPath = process.argv[2];
const typeRegistryPath = process.argv[3];
const code = fs.readFileSync(inputPath, 'utf8');

const CONFIG_FILES = [
  'macroforge.config.ts', 'macroforge.config.mts',
  'macroforge.config.js', 'macroforge.config.mjs', 'macroforge.config.cjs',
];

function findConfigFile(startDir) {
  let dir = startDir;
  while (dir) {
    for (const configName of CONFIG_FILES) {
      const configPath = path.join(dir, configName);
      if (fs.existsSync(configPath)) return configPath;
    }
    if (fs.existsSync(path.join(dir, 'package.json'))) break;
    const parent = path.dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  return null;
}

try {
  const inputDir = path.dirname(path.resolve(inputPath));
  const configPath = findConfigFile(inputDir);
  let options = {};
  if (configPath) {
    const configContent = fs.readFileSync(configPath, 'utf8');
    loadConfig(configContent, configPath);
    options.configPath = configPath;
  }

  // Load pre-built type registry if available
  if (typeRegistryPath) {
    try {
      options.typeRegistryJson = fs.readFileSync(typeRegistryPath, 'utf8');
    } catch {}
  }

  const result = expandSync(code, inputPath, options);
  const hasExpansions = result.sourceMapping?.generatedRegions?.length > 0;
  console.log(JSON.stringify({
    code: result.code,
    hasExpansions,
  }));
} catch (err) {
  console.error('Error:', err.message);
  process.exit(1);
}
"#;

    let mut temp_dir = std::env::temp_dir();
    temp_dir.push("macroforge-cli");
    fs::create_dir_all(&temp_dir)?;
    let script_path = temp_dir.join("cache-expand-wrapper.js");
    fs::write(&script_path, script)?;

    let cwd = std::env::current_dir()?;
    let registry_path = TYPE_REGISTRY_CACHE_PATH.lock().unwrap().clone();
    let mut cmd = std::process::Command::new("node");
    cmd.arg(&script_path).arg(path);
    if let Some(ref rp) = registry_path {
        cmd.arg(rp);
    }
    let output = cmd
        .current_dir(&cwd)
        .output()
        .context("failed to run node expand wrapper")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("node expansion failed: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let result: serde_json::Value =
        serde_json::from_str(&stdout).context("failed to parse expansion result")?;

    // Check for fallback marker
    if result
        .get("fallback")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        // Fall back to builtin
        return expand_for_cache(path, source, true);
    }

    let has_expansions = result["hasExpansions"].as_bool().unwrap_or(false);
    if !has_expansions {
        return Ok(None);
    }

    let code = result["code"]
        .as_str()
        .ok_or_else(|| anyhow!("missing 'code' in expansion result"))?;

    Ok(Some(code.to_string()))
}

/// Warm the cache: expand all files and save the manifest. Returns the manifest for reuse.
fn warm_cache(
    label: &str,
    root: &Path,
    cache_dir: &Path,
    manifest: &mut CacheManifest,
    builtin_only: bool,
) -> Result<()> {
    use rayon::prelude::*;

    eprintln!("[macroforge {label}] Warming cache for {}", root.display());
    let start = std::time::Instant::now();
    let files = collect_watch_files(root);
    let mut expanded_count = 0u32;

    // Phase 1: Read files, hash, filter out already-cached (sequential for manifest reads)
    let mut files_to_expand: Vec<(PathBuf, String, String, String, String)> = Vec::new();
    for file_path in &files {
        let rel_path = file_path
            .strip_prefix(root)
            .unwrap_or(file_path)
            .to_string_lossy()
            .to_string();

        let source = match fs::read_to_string(file_path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let source_hash = content_hash(source.as_bytes());

        // Skip if already cached with matching hash
        if let Some(entry) = manifest.entries.get(&rel_path)
            && entry.source_hash == source_hash
        {
            // Backfill normalized_hash for entries from older manifests
            if entry.normalized_hash.is_empty() {
                let norm_hash = normalized_content_hash(&source);
                manifest.entries.insert(
                    rel_path,
                    CacheEntry {
                        source_hash,
                        has_macros: entry.has_macros,
                        normalized_hash: norm_hash,
                    },
                );
            }
            continue;
        }

        let norm_hash = normalized_content_hash(&source);
        files_to_expand.push((
            file_path.clone(),
            rel_path,
            source,
            source_hash,
            norm_hash,
        ));
    }

    // Phase 2: Expand in parallel
    let pool = if builtin_only {
        rayon::ThreadPoolBuilder::new().build()?
    } else {
        rayon::ThreadPoolBuilder::new().num_threads(4).build()?
    };

    let results: Vec<_> = pool.install(|| {
        files_to_expand
            .par_iter()
            .map(|(file_path, rel_path, source, source_hash, norm_hash)| {
                let result = expand_for_cache(file_path, source, builtin_only);
                (
                    rel_path.clone(),
                    source_hash.clone(),
                    norm_hash.clone(),
                    result,
                )
            })
            .collect()
    });

    // Phase 3: Apply results to manifest (sequential)
    for (rel_path, source_hash, norm_hash, result) in results {
        match result {
            Ok(Some(expanded)) => {
                if let Err(e) = write_cache_file(cache_dir, &rel_path, &expanded) {
                    eprintln!("  [!] {} — write failed: {}", rel_path, e);
                    continue;
                }
                manifest.entries.insert(
                    rel_path.clone(),
                    CacheEntry {
                        source_hash,
                        has_macros: true,
                        normalized_hash: norm_hash,
                    },
                );
                expanded_count += 1;
                eprintln!("  [+] {}", rel_path);
            }
            Ok(None) => {
                manifest.entries.insert(
                    rel_path,
                    CacheEntry {
                        source_hash,
                        has_macros: false,
                        normalized_hash: norm_hash,
                    },
                );
            }
            Err(e) => {
                eprintln!("  [!] {} — {}", rel_path, e);
            }
        }
    }

    manifest.save(cache_dir)?;
    let elapsed = start.elapsed();
    eprintln!(
        "[macroforge {label}] Cache warm: {} files expanded in {:.1}s ({} total files)",
        expanded_count,
        elapsed.as_secs_f64(),
        files.len()
    );
    Ok(())
}

/// Resolves root path, creates cache dir reference, and loads/creates the manifest.
fn init_cache(
    root: Option<PathBuf>,
    label: &str,
    builtin_only: bool,
) -> Result<(PathBuf, PathBuf, CacheManifest)> {
    let root = root
        .unwrap_or_else(|| PathBuf::from("."))
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from("."));
    let cache_dir = root.join(".macroforge").join("cache");
    let version = env!("CARGO_PKG_VERSION").to_string();
    let config_hash = compute_config_hash(&root);

    let manifest = CacheManifest::load(&cache_dir)
        .filter(|m| {
            m.version == version && m.config_hash == config_hash && m.builtin_only == builtin_only
        })
        .unwrap_or_else(|| {
            eprintln!("[macroforge {label}] Creating fresh cache");
            CacheManifest::new(version.clone(), config_hash.clone(), builtin_only)
        });

    Ok((root, cache_dir, manifest))
}

/// Build the .macroforge/cache once and exit.
fn run_cache(root: Option<PathBuf>, builtin_only: bool) -> Result<()> {
    let (root, cache_dir, mut manifest) = init_cache(root, "cache", builtin_only)?;
    warm_cache("cache", &root, &cache_dir, &mut manifest, builtin_only)?;
    Ok(())
}

/// Delete the .macroforge/cache directory and rebuild from scratch.
fn run_refresh(root: Option<PathBuf>, builtin_only: bool) -> Result<()> {
    let root_resolved = root
        .clone()
        .unwrap_or_else(|| PathBuf::from("."))
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from("."));
    let cache_dir = root_resolved.join(".macroforge").join("cache");

    if cache_dir.exists() {
        eprintln!("[macroforge refresh] Deleting {}", cache_dir.display());
        fs::remove_dir_all(&cache_dir).context("failed to delete .macroforge/cache")?;
    } else {
        eprintln!("[macroforge refresh] No existing cache found, building fresh");
    }

    let (root, cache_dir, mut manifest) = init_cache(root, "refresh", builtin_only)?;
    warm_cache("refresh", &root, &cache_dir, &mut manifest, builtin_only)?;
    Ok(())
}

// =========================================================================
// Macro source watching: discover, detect build system, rebuild
// =========================================================================

#[derive(Debug)]
struct MacroSourceInfo {
    name: String,
    package_dir: PathBuf,
    source_dirs: Vec<PathBuf>,
    kind: MacroSourceKind,
}

#[derive(Debug)]
enum MacroSourceKind {
    /// Rust NAPI macro (has Cargo.toml with macroforge_ts dependency)
    RustNapi,
    /// JavaScript/TypeScript macro package
    JsTs,
}

#[derive(Debug, Clone, Copy)]
enum BuildSystem {
    Npm,
    Pnpm,
    Yarn,
}

fn detect_build_system(root: &Path) -> BuildSystem {
    if root.join("pnpm-lock.yaml").exists() {
        BuildSystem::Pnpm
    } else if root.join("yarn.lock").exists() {
        BuildSystem::Yarn
    } else {
        BuildSystem::Npm
    }
}

/// Discover macro source packages from workspace configuration.
fn discover_macro_sources(root: &Path) -> Vec<MacroSourceInfo> {
    let pkg_json_path = root.join("package.json");
    let pkg_json = match fs::read_to_string(&pkg_json_path) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };
    let pkg: serde_json::Value = match serde_json::from_str(&pkg_json) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    // Collect workspace patterns
    let workspace_patterns: Vec<String> = match &pkg["workspaces"] {
        serde_json::Value::Array(arr) => arr
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect(),
        serde_json::Value::Object(obj) => obj
            .get("packages")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default(),
        _ => Vec::new(),
    };

    // Expand workspace patterns into actual directories
    let mut workspace_dirs: Vec<PathBuf> = Vec::new();
    for pattern in &workspace_patterns {
        if pattern.contains('*') {
            // Simple glob: "packages/*" -> list subdirs of "packages/"
            let prefix = pattern.trim_end_matches("/*").trim_end_matches("/**");
            let base = root.join(prefix);
            if let Ok(entries) = fs::read_dir(&base) {
                for entry in entries.flatten() {
                    if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                        workspace_dirs.push(entry.path());
                    }
                }
            }
        } else {
            let dir = root.join(pattern);
            if dir.is_dir() {
                workspace_dirs.push(dir);
            }
        }
    }

    let mut sources = Vec::new();
    for dir in &workspace_dirs {
        let cargo_toml = dir.join("Cargo.toml");
        let child_pkg_json = dir.join("package.json");

        // Check if it's a Rust NAPI macro crate
        if cargo_toml.exists() {
            if let Ok(content) = fs::read_to_string(&cargo_toml) {
                if content.contains("macroforge_ts") {
                    let src_dir = dir.join("src");
                    let name = dir
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    sources.push(MacroSourceInfo {
                        name,
                        package_dir: dir.clone(),
                        source_dirs: if src_dir.is_dir() {
                            vec![src_dir]
                        } else {
                            vec![dir.clone()]
                        },
                        kind: MacroSourceKind::RustNapi,
                    });
                    continue;
                }
            }
        }

        // Check if it's a JS/TS macro package
        if child_pkg_json.exists() {
            if let Ok(content) = fs::read_to_string(&child_pkg_json) {
                if let Ok(child_pkg) = serde_json::from_str::<serde_json::Value>(&content) {
                    let has_macroforge_dep = ["dependencies", "devDependencies", "peerDependencies"]
                        .iter()
                        .any(|key| {
                            child_pkg[key]
                                .as_object()
                                .is_some_and(|deps| deps.contains_key("macroforge"))
                        });

                    if has_macroforge_dep {
                        let src_dir = dir.join("src");
                        let name = child_pkg["name"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();
                        sources.push(MacroSourceInfo {
                            name: if name.is_empty() {
                                dir.file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string()
                            } else {
                                name
                            },
                            package_dir: dir.clone(),
                            source_dirs: if src_dir.is_dir() {
                                vec![src_dir]
                            } else {
                                vec![dir.clone()]
                            },
                            kind: MacroSourceKind::JsTs,
                        });
                    }
                }
            }
        }
    }

    sources
}

/// Rebuild a macro package. Returns Ok(()) on success.
fn rebuild_macro(info: &MacroSourceInfo, build_system: BuildSystem, _root: &Path) -> Result<()> {
    let (cmd, args) = match build_system {
        BuildSystem::Pnpm => ("pnpm", vec!["run", "build"]),
        BuildSystem::Yarn => ("yarn", vec!["build"]),
        BuildSystem::Npm => ("npm", vec!["run", "build"]),
    };

    eprintln!(
        "[macroforge watch] Running `{} {}` in {}",
        cmd,
        args.join(" "),
        info.package_dir.display()
    );

    let output = std::process::Command::new(cmd)
        .args(&args)
        .current_dir(&info.package_dir)
        .env("NAPI_BUILD_SKIP_WATCHER", "1")
        .output()
        .with_context(|| {
            format!(
                "failed to run `{cmd}` for macro package '{}'",
                info.name
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        anyhow::bail!(
            "macro rebuild failed for '{}' (exit {}):\n{}\n{}",
            info.name,
            output.status,
            stdout,
            stderr
        );
    }

    Ok(())
}

/// Main watch loop: warm cache then watch for changes.
fn run_watch(root: Option<PathBuf>, builtin_only: bool, debounce_ms: u64) -> Result<()> {
    let (root, cache_dir, mut manifest) = init_cache(root, "watch", builtin_only)?;
    warm_cache("watch", &root, &cache_dir, &mut manifest, builtin_only)?;

    // Discover macro source packages and watch them
    let macro_sources = discover_macro_sources(&root);
    let build_system = detect_build_system(&root);

    if !macro_sources.is_empty() {
        eprintln!(
            "[macroforge watch] Discovered {} macro source package(s):",
            macro_sources.len()
        );
        for src in &macro_sources {
            eprintln!("  - {} ({:?})", src.name, src.kind);
        }
    }

    eprintln!("[macroforge watch] Watching for changes... (Ctrl+C to stop)");

    // --- Watch loop ---
    let (tx, rx) = std::sync::mpsc::channel();
    let mut debouncer = new_debouncer(Duration::from_millis(debounce_ms), None, tx)
        .context("failed to create file watcher")?;

    debouncer
        .watch(&root, RecursiveMode::Recursive)
        .context("failed to start watching")?;

    // Watch macro source directories that are outside root
    for src in &macro_sources {
        for dir in &src.source_dirs {
            if dir.exists() && !dir.starts_with(&root) {
                debouncer
                    .watch(dir, RecursiveMode::Recursive)
                    .with_context(|| {
                        format!("failed to watch macro source: {}", dir.display())
                    })?;
            }
        }
    }

    for result in rx {
        match result {
            Ok(events) => {
                let mut config_changed = false;
                let mut macro_source_changed: Option<&MacroSourceInfo> = None;
                let mut changed_files: Vec<PathBuf> = Vec::new();

                for event in &events {
                    for event_path in &event.paths {
                        // Check for config file changes
                        if let Some(name) = event_path.file_name() {
                            let name_str = name.to_string_lossy();
                            if name_str.starts_with("macroforge.config.") {
                                config_changed = true;
                                continue;
                            }
                        }

                        // Check if path belongs to a macro source package
                        let is_macro_src = macro_sources.iter().find(|ms| {
                            ms.source_dirs.iter().any(|d| event_path.starts_with(d))
                        });
                        if let Some(ms) = is_macro_src {
                            macro_source_changed = Some(ms);
                            continue;
                        }

                        if is_watchable_ts_file(event_path, &root) {
                            changed_files.push(event_path.clone());
                        }
                    }
                }

                changed_files.sort();
                changed_files.dedup();

                // Macro source change: rebuild then full re-expand
                if let Some(macro_info) = macro_source_changed {
                    eprintln!(
                        "[macroforge watch] Macro source changed in '{}', rebuilding...",
                        macro_info.name
                    );

                    match rebuild_macro(macro_info, build_system, &root) {
                        Ok(()) => {
                            eprintln!(
                                "[macroforge watch] Macro '{}' rebuilt, re-expanding all files...",
                                macro_info.name
                            );
                            manifest.entries.clear();
                            macroforge_ts::host::clear_config_cache();
                            warm_cache("watch", &root, &cache_dir, &mut manifest, builtin_only)?;
                        }
                        Err(e) => {
                            eprintln!(
                                "[macroforge watch] Macro rebuild failed: {}",
                                e
                            );
                        }
                    }
                } else if config_changed {
                    use rayon::prelude::*;

                    let new_config_hash = compute_config_hash(&root);
                    manifest.config_hash = new_config_hash;
                    manifest.entries.clear();
                    eprintln!("[macroforge watch] Config changed, re-expanding all files...");

                    let all_files = collect_watch_files(&root);

                    // Read and hash all files (sequential)
                    let files_with_source: Vec<_> = all_files
                        .iter()
                        .filter_map(|file_path| {
                            let rel_path = file_path
                                .strip_prefix(&root)
                                .unwrap_or(file_path)
                                .to_string_lossy()
                                .to_string();
                            let source = fs::read_to_string(file_path).ok()?;
                            let source_hash = content_hash(source.as_bytes());
                            let norm_hash = normalized_content_hash(&source);
                            Some((file_path.clone(), rel_path, source, source_hash, norm_hash))
                        })
                        .collect();

                    // Expand in parallel
                    let results: Vec<_> = files_with_source
                        .par_iter()
                        .map(|(file_path, rel_path, source, source_hash, norm_hash)| {
                            let result = expand_for_cache(file_path, source, builtin_only);
                            (rel_path.clone(), source_hash.clone(), norm_hash.clone(), result)
                        })
                        .collect();

                    // Apply results (sequential)
                    let mut count = 0u32;
                    for (rel_path, source_hash, norm_hash, result) in results {
                        match result {
                            Ok(Some(expanded)) => {
                                let _ = write_cache_file(&cache_dir, &rel_path, &expanded);
                                manifest.entries.insert(
                                    rel_path.clone(),
                                    CacheEntry {
                                        source_hash,
                                        has_macros: true,
                                        normalized_hash: norm_hash,
                                    },
                                );
                                count += 1;
                                eprintln!("  [~] {}", rel_path);
                            }
                            Ok(None) => {
                                manifest.entries.insert(
                                    rel_path,
                                    CacheEntry {
                                        source_hash,
                                        has_macros: false,
                                        normalized_hash: norm_hash,
                                    },
                                );
                            }
                            Err(e) => eprintln!("  [!] {} — {}", rel_path, e),
                        }
                    }
                    manifest.save(&cache_dir)?;
                    eprintln!("[macroforge watch] Re-expanded {} files", count);
                } else if !changed_files.is_empty() {
                    for file_path in &changed_files {
                        let rel_path = file_path
                            .strip_prefix(&root)
                            .unwrap_or(file_path)
                            .to_string_lossy()
                            .to_string();

                        let source = match fs::read_to_string(file_path) {
                            Ok(s) => s,
                            Err(_) => {
                                // File was deleted
                                manifest.entries.remove(&rel_path);
                                // Remove cached file too
                                let cache_path = cache_dir.join(format!("{rel_path}.cache"));
                                let _ = fs::remove_file(&cache_path);
                                eprintln!("  [-] {} (removed)", rel_path);
                                continue;
                            }
                        };

                        let source_hash = content_hash(source.as_bytes());

                        // Fast path: file is byte-identical to cached version
                        if let Some(entry) = manifest.entries.get(&rel_path)
                            && entry.source_hash == source_hash
                        {
                            continue;
                        }

                        // Check if only whitespace changed
                        let norm_hash = normalized_content_hash(&source);
                        if let Some(entry) = manifest.entries.get(&rel_path)
                            && !entry.normalized_hash.is_empty()
                            && entry.normalized_hash == norm_hash
                        {
                            // Update raw hash but skip re-expansion
                            manifest.entries.insert(
                                rel_path.clone(),
                                CacheEntry {
                                    source_hash,
                                    has_macros: entry.has_macros,
                                    normalized_hash: norm_hash,
                                },
                            );
                            eprintln!("  [·] {} (whitespace only)", rel_path);
                            continue;
                        }

                        let file_start = std::time::Instant::now();

                        match expand_for_cache(file_path, &source, builtin_only) {
                            Ok(Some(expanded)) => {
                                let _ = write_cache_file(&cache_dir, &rel_path, &expanded);
                                manifest.entries.insert(
                                    rel_path.clone(),
                                    CacheEntry {
                                        source_hash,
                                        has_macros: true,
                                        normalized_hash: norm_hash,
                                    },
                                );
                                let elapsed = file_start.elapsed();
                                eprintln!("  [~] {} ({}ms)", rel_path, elapsed.as_millis());
                            }
                            Ok(None) => {
                                manifest.entries.insert(
                                    rel_path.clone(),
                                    CacheEntry {
                                        source_hash,
                                        has_macros: false,
                                        normalized_hash: norm_hash,
                                    },
                                );
                                eprintln!("  [.] {} (no macros)", rel_path);
                            }
                            Err(e) => {
                                eprintln!("  [!] {} — {}", rel_path, e);
                            }
                        }
                    }
                    manifest.save(&cache_dir)?;
                }
            }
            Err(errors) => {
                for e in errors {
                    eprintln!("[macroforge watch] Watch error: {}", e);
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // get_expanded_path tests
    // =========================================================================

    #[test]
    fn test_get_expanded_path_simple_ts() {
        let input = Path::new("src/User.ts");
        let result = get_expanded_path(input);
        assert_eq!(result, PathBuf::from("src/User.expanded.ts"));
    }

    #[test]
    fn test_get_expanded_path_svelte_ts() {
        let input = Path::new("src/lib/demo/types/appointment.svelte.ts");
        let result = get_expanded_path(input);
        assert_eq!(
            result,
            PathBuf::from("src/lib/demo/types/appointment.expanded.svelte.ts")
        );
    }

    #[test]
    fn test_get_expanded_path_tsx() {
        let input = Path::new("components/Button.tsx");
        let result = get_expanded_path(input);
        assert_eq!(result, PathBuf::from("components/Button.expanded.tsx"));
    }

    #[test]
    fn test_get_expanded_path_no_extension() {
        let input = Path::new("src/Makefile");
        let result = get_expanded_path(input);
        assert_eq!(result, PathBuf::from("src/Makefile.expanded"));
    }

    #[test]
    fn test_get_expanded_path_multiple_dots() {
        let input = Path::new("lib/config.test.spec.ts");
        let result = get_expanded_path(input);
        assert_eq!(result, PathBuf::from("lib/config.expanded.test.spec.ts"));
    }

    #[test]
    fn test_get_expanded_path_root_file() {
        let input = Path::new("index.ts");
        let result = get_expanded_path(input);
        assert_eq!(result, PathBuf::from("index.expanded.ts"));
    }

    // =========================================================================
    // offset_to_line_col tests
    // =========================================================================

    #[test]
    fn test_offset_to_line_col_first_char() {
        let source = "hello\nworld";
        assert_eq!(offset_to_line_col(source, 0), (1, 1));
    }

    #[test]
    fn test_offset_to_line_col_same_line() {
        let source = "hello\nworld";
        assert_eq!(offset_to_line_col(source, 3), (1, 4)); // 'l' in hello
    }

    #[test]
    fn test_offset_to_line_col_second_line() {
        let source = "hello\nworld";
        assert_eq!(offset_to_line_col(source, 6), (2, 1)); // 'w' in world
    }

    #[test]
    fn test_offset_to_line_col_second_line_middle() {
        let source = "hello\nworld";
        assert_eq!(offset_to_line_col(source, 9), (2, 4)); // 'l' in world
    }

    #[test]
    fn test_offset_to_line_col_multiple_lines() {
        let source = "line1\nline2\nline3";
        assert_eq!(offset_to_line_col(source, 12), (3, 1)); // 'l' in line3
    }

    #[test]
    fn test_offset_to_line_col_empty_lines() {
        let source = "a\n\nb";
        assert_eq!(offset_to_line_col(source, 2), (2, 1)); // empty line
        assert_eq!(offset_to_line_col(source, 3), (3, 1)); // 'b'
    }

    // =========================================================================
    // ImportRegistry::from_module tests (replaces extract_import_sources_from_code)
    // =========================================================================

    /// Parse code into a Module and build an ImportRegistry.
    fn registry_from_code(code: &str) -> macroforge_ts_syn::ImportRegistry {
        use macroforge_ts_syn::parse_ts_module;
        let module = parse_ts_module(code).expect("failed to parse");
        macroforge_ts_syn::ImportRegistry::from_module(&module, code)
    }

    #[test]
    fn test_extract_imports_named() {
        let code = r#"import { DateTime } from 'effect';"#;
        let r = registry_from_code(code);
        let imports = r.source_modules();
        assert_eq!(imports.get("DateTime"), Some(&"effect".to_string()));
    }

    #[test]
    fn test_extract_imports_multiple_named() {
        let code = r#"import { DateTime, Duration } from 'effect';"#;
        let r = registry_from_code(code);
        let imports = r.source_modules();
        assert_eq!(imports.get("DateTime"), Some(&"effect".to_string()));
        assert_eq!(imports.get("Duration"), Some(&"effect".to_string()));
    }

    #[test]
    fn test_extract_imports_type_import() {
        let code = r#"import type { DateTime } from 'effect';"#;
        let r = registry_from_code(code);
        let imports = r.source_modules();
        assert_eq!(imports.get("DateTime"), Some(&"effect".to_string()));
    }

    #[test]
    fn test_extract_imports_default() {
        let code = r#"import React from 'react';"#;
        let r = registry_from_code(code);
        let imports = r.source_modules();
        assert_eq!(imports.get("React"), Some(&"react".to_string()));
    }

    #[test]
    fn test_extract_imports_namespace() {
        let code = r#"import * as Effect from 'effect';"#;
        let r = registry_from_code(code);
        let imports = r.source_modules();
        assert_eq!(imports.get("Effect"), Some(&"effect".to_string()));
    }

    #[test]
    fn test_extract_imports_scoped_package() {
        let code = r#"import { Schema } from '@effect/schema';"#;
        let r = registry_from_code(code);
        let imports = r.source_modules();
        assert_eq!(imports.get("Schema"), Some(&"@effect/schema".to_string()));
    }

    #[test]
    fn test_extract_imports_subpath() {
        let code = r#"import { DateTime } from 'effect/DateTime';"#;
        let r = registry_from_code(code);
        let imports = r.source_modules();
        assert_eq!(
            imports.get("DateTime"),
            Some(&"effect/DateTime".to_string())
        );
    }

    #[test]
    fn test_extract_imports_multiple_statements() {
        let code = r#"
            import { DateTime } from 'effect';
            import { ZonedDateTime } from '@internationalized/date';
            import type { Site } from './site.svelte';
        "#;
        let r = registry_from_code(code);
        let imports = r.source_modules();
        assert_eq!(imports.get("DateTime"), Some(&"effect".to_string()));
        assert_eq!(
            imports.get("ZonedDateTime"),
            Some(&"@internationalized/date".to_string())
        );
        assert_eq!(imports.get("Site"), Some(&"./site.svelte".to_string()));
    }

    #[test]
    fn test_extract_imports_tsx_file() {
        // Note: parse_ts_module uses tsx mode by default in macroforge_ts_syn
        let code = r#"
            import React from 'react';
            import { useState } from 'react';
        "#;
        let r = registry_from_code(code);
        let imports = r.source_modules();
        assert_eq!(imports.get("React"), Some(&"react".to_string()));
        assert_eq!(imports.get("useState"), Some(&"react".to_string()));
    }

    #[test]
    fn test_extract_imports_empty_code() {
        let code = "export {};";
        let r = registry_from_code(code);
        let imports = r.source_modules();
        assert!(imports.is_empty());
    }

    #[test]
    fn test_extract_imports_no_imports() {
        let code = r#"
            const x = 1;
            export function foo() { return x; }
        "#;
        let r = registry_from_code(code);
        let imports = r.source_modules();
        assert!(imports.is_empty());
    }

    #[test]
    fn test_extract_imports_with_jsdoc_decorators() {
        let code = r#"
            import type { DateTime } from 'effect';

            /** @derive(Serialize) */
            interface Event {
                /** @serde({ validate: ["nonEmpty"] }) */
                name: string;
                begins: DateTime.DateTime;
            }
        "#;
        let r = registry_from_code(code);
        let imports = r.source_modules();
        assert_eq!(imports.get("DateTime"), Some(&"effect".to_string()));
    }

    #[test]
    fn test_extract_imports_with_real_decorators() {
        let code = r#"
            import type { DateTime } from 'effect';
            import { Component } from '@angular/core';

            @Component({ selector: 'app-root' })
            class AppComponent {
                begins: DateTime.DateTime;
            }
        "#;
        let r = registry_from_code(code);
        let imports = r.source_modules();
        assert_eq!(imports.get("DateTime"), Some(&"effect".to_string()));
        assert_eq!(imports.get("Component"), Some(&"@angular/core".to_string()));
    }

    #[test]
    fn test_extract_imports_with_alias() {
        let code = r#"
            import type { Option as EffectOption } from 'effect/Option';
            import { DateTime as EffectDateTime } from 'effect';
        "#;
        let r = registry_from_code(code);
        let imports = r.source_modules();
        let aliases = r.aliases();

        assert_eq!(
            imports.get("EffectOption"),
            Some(&"effect/Option".to_string())
        );
        assert_eq!(imports.get("EffectDateTime"), Some(&"effect".to_string()));

        assert_eq!(aliases.get("EffectOption"), Some(&"Option".to_string()));
        assert_eq!(aliases.get("EffectDateTime"), Some(&"DateTime".to_string()));
    }

    // =========================================================================
    // normalized_content_hash tests
    // =========================================================================

    #[test]
    fn test_normalized_hash_trailing_whitespace() {
        let a = "fn foo() {\n    bar();\n}\n";
        let b = "fn foo() {   \n    bar();   \n}   \n";
        assert_eq!(normalized_content_hash(a), normalized_content_hash(b));
    }

    #[test]
    fn test_normalized_hash_blank_lines() {
        let a = "import { x } from 'y';\n\nexport class Foo {}\n";
        let b = "import { x } from 'y';\n\n\n\nexport class Foo {}\n";
        assert_eq!(normalized_content_hash(a), normalized_content_hash(b));
    }

    #[test]
    fn test_normalized_hash_trailing_newlines() {
        let a = "const x = 1;\n";
        let b = "const x = 1;\n\n\n";
        assert_eq!(normalized_content_hash(a), normalized_content_hash(b));
    }

    #[test]
    fn test_normalized_hash_real_change_differs() {
        let a = "const x = 1;\n";
        let b = "const x = 2;\n";
        assert_ne!(normalized_content_hash(a), normalized_content_hash(b));
    }

    #[test]
    fn test_normalized_hash_indentation_change_differs() {
        let a = "  const x = 1;\n";
        let b = "    const x = 1;\n";
        // Leading indentation IS significant
        assert_ne!(normalized_content_hash(a), normalized_content_hash(b));
    }

    // =========================================================================
    // warm_cache normalized_hash backfill test
    // =========================================================================

    #[test]
    fn test_warm_cache_backfills_normalized_hash() {
        // Simulate a manifest entry from before the normalized_hash feature:
        // source_hash matches the file on disk, but normalized_hash is empty.
        // warm_cache should backfill the normalized_hash without re-expanding.

        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let cache_dir = root.join(".macroforge").join("cache");
        fs::create_dir_all(&cache_dir).unwrap();

        // Write a source file
        let src_dir = root.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        let src_file = src_dir.join("test.ts");
        let source = "const x = 1;\n";
        fs::write(&src_file, source).unwrap();

        // Create a manifest with an entry that has a matching source_hash
        // but an empty normalized_hash (simulating an old manifest)
        let source_hash = content_hash(source.as_bytes());
        let mut manifest = CacheManifest::new(
            env!("CARGO_PKG_VERSION").to_string(),
            "none".to_string(),
            true,
        );
        manifest.entries.insert(
            "src/test.ts".to_string(),
            CacheEntry {
                source_hash: source_hash.clone(),
                has_macros: false,
                normalized_hash: String::new(), // empty = old manifest
            },
        );

        // warm_cache should skip re-expansion (source_hash matches)
        // but backfill the normalized_hash
        warm_cache("test", root, &cache_dir, &mut manifest, true).unwrap();

        let entry = manifest.entries.get("src/test.ts").unwrap();
        assert_eq!(entry.source_hash, source_hash);
        assert!(
            !entry.normalized_hash.is_empty(),
            "normalized_hash should be backfilled"
        );
        assert_eq!(entry.normalized_hash, normalized_content_hash(source));
    }
}
