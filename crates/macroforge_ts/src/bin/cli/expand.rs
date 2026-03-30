use anyhow::{Context, Result, anyhow};
use ignore::WalkBuilder;
use macroforge_ts::host::{MacroExpander, MacroExpansion};
use std::{
    fs,
    path::{Path, PathBuf},
};

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
pub fn scan_and_expand(root: PathBuf, builtin_only: bool, include_ignored: bool) -> Result<()> {
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
                let result = try_expand_file(path.clone(), None, None, false, builtin_only, true);
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
pub fn expand_file(
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
pub(crate) fn try_expand_file(
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
pub(crate) fn try_expand_file_builtin(
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

let expandSync, loadConfig, clearConfigCache, collectExternalDecoratorModules;
try {
  const macroforge = cwdRequire('macroforge');
  expandSync = macroforge.expandSync;
  loadConfig = macroforge.loadConfig;
  clearConfigCache = macroforge.clearConfigCache;
  try {
    const shared = cwdRequire('@macroforge/shared');
    collectExternalDecoratorModules = shared.collectExternalDecoratorModules;
  } catch (e) {
    console.error('[macroforge] warning: @macroforge/shared not available, external decorator filtering disabled:', e.message);
  }
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

  // Collect external decorator modules so the annotation filter includes them
  if (collectExternalDecoratorModules) {
    try {
      options.externalDecoratorModules = collectExternalDecoratorModules(code, cwdRequire);
    } catch (e) {
      console.error('[macroforge] warning: failed to collect external decorator modules:', e.message);
    }
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

/// Prints macro expansion diagnostics (warnings, errors) to stderr.
///
/// Each diagnostic is formatted with its level, file location, and message.
pub(crate) fn emit_diagnostics(expansion: &MacroExpansion, source: &str, input: &Path) {
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
pub(crate) fn offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
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
/// Examples: `foo.svelte.ts` -> `foo.expanded.svelte.ts`, `foo.ts` -> `foo.expanded.ts`
pub(crate) fn get_expanded_path(input: &Path) -> PathBuf {
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
