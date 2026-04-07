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
/// * `include_ignored` - If true, also process files ignored by `.gitignore`
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if the scan fails.
pub fn scan_and_expand(root: PathBuf, include_ignored: bool) -> Result<()> {
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
    let pool = rayon::ThreadPoolBuilder::new().build()?;

    let results: Vec<_> = pool.install(|| {
        files
            .par_iter()
            .map(|path| {
                let result = try_expand_file(path.clone(), None, None, false, true);
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
    quiet: bool,
) -> Result<()> {
    match try_expand_file(input.clone(), out, types_out, print, false)? {
        true => Ok(()),
        false => {
            if !quiet {
                eprintln!("[macroforge] no macros found in {}", input.display());
            }
            std::process::exit(2);
        }
    }
}

/// Attempts to expand macros in a file using the Rust-native expander.
///
/// Uses the Rust `MacroExpander` which supports both built-in macros and
/// external macros (via FFI for compiled packages or Node.js subprocess fallback).
///
/// # Arguments
///
/// * `input` - Path to the input TypeScript file
/// * `out` - Optional output path for expanded code
/// * `types_out` - Optional output path for type declarations
/// * `print` - Whether to print output to stdout
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
    _is_scanning: bool,
) -> Result<bool> {
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
///
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
