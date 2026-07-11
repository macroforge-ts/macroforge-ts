use anyhow::{Context, Result, anyhow};
use ignore::WalkBuilder;
use macroforge_ts::host::{MacroExpander, MacroExpansion};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Output routing for a directory scan.
///
/// A scan can emit into one or more destinations, each independent:
/// - `out_dir` — a **mirrored tree** of the scanned sources: every macro file
///   is written expanded at `<out_dir>/<relative-path>` (same filename), and
///   every other file (`.svelte`, `.css`, `.d.ts`, macro-free `.ts`, assets)
///   is copied verbatim. This produces a packager-ready staging tree.
/// - `types_out_dir` — mirrored `.d.ts` type surfaces for each macro file that
///   produces one (`foo.ts` → `foo.d.ts`, `foo.svelte.ts` → `foo.svelte.d.ts`).
/// - `emit_expanded` — legacy behavior: write a `<name>.expanded.<ext>` debug
///   sibling next to each expanded source file. Default off.
///
/// With none of these set, the scan is a diagnostics/check pass that writes
/// nothing (and still fails with a non-zero exit if any file cannot expand).
#[derive(Default)]
pub struct ScanOptions {
    /// Also process files ignored by `.gitignore`.
    pub include_ignored: bool,
    /// Directory for the mirrored expanded source tree.
    pub out_dir: Option<PathBuf>,
    /// Directory for the mirrored `.d.ts` type surfaces.
    pub types_out_dir: Option<PathBuf>,
    /// Write `<name>.expanded.<ext>` siblings next to each expanded source.
    pub emit_expanded: bool,
}

/// Recursively scans a directory for TypeScript files and expands macros in each.
///
/// This function walks the directory tree, respecting `.gitignore` rules (unless
/// `opts.include_ignored` is true). Where output is emitted is controlled by
/// [`ScanOptions`].
///
/// Files whose name contains `.expanded.` are always skipped so macroforge never
/// re-scans or mirrors its own debug artifacts. `.d.ts` declaration files are not
/// expanded, but are copied verbatim into `out_dir` when a mirrored tree is
/// requested.
///
/// # Returns
///
/// Returns `Ok(())` on success. Returns an error if any file fails to expand —
/// a partially-populated staging tree must never silently feed a packager.
pub fn scan_and_expand(root: PathBuf, opts: ScanOptions) -> Result<()> {
    use rayon::prelude::*;

    let root = root.canonicalize().unwrap_or(root);
    eprintln!("[macroforge] scanning {}", root.display());

    // Output directories commonly live inside the scanned tree — the canonical
    // form is `expand --scan . --types-out dist/types`. That is fine: we prune
    // the output directories from the walk (below) so their contents are never
    // re-scanned, re-expanded, or recursively copied on a later run. What is not
    // fine is an output directory that *is* the scan root or an ancestor of it —
    // that would overwrite the sources in place — so reject only that case.
    let output_dirs: Vec<PathBuf> = [
        ("--out", opts.out_dir.as_ref()),
        ("--types-out", opts.types_out_dir.as_ref()),
    ]
    .into_iter()
    .filter_map(|(flag, dir)| dir.map(|d| (flag, canonicalized_target(d), d)))
    .map(|(flag, canonical, dir)| {
        if root.starts_with(&canonical) {
            return Err(anyhow!(
                "{flag} directory {} contains the scan root {}; choose a location that does not overwrite the scanned sources",
                dir.display(),
                root.display()
            ));
        }
        Ok(canonical)
    })
    .collect::<Result<_>>()?;

    // Phase 1: Collect files (sequential walk). TypeScript sources are expansion
    // candidates; every other file is a passthrough that is only tracked when a
    // mirrored tree is being produced. Output directories are pruned from the
    // walk so a re-run never ingests previously-emitted files.
    let mut ts_files: Vec<PathBuf> = Vec::new();
    let mut passthrough_files: Vec<PathBuf> = Vec::new();
    let walker = WalkBuilder::new(&root)
        .hidden(false)
        .git_ignore(!opts.include_ignored)
        .git_global(false)
        .git_exclude(false)
        .filter_entry(move |entry| {
            let path = entry.path();
            !output_dirs.iter().any(|dir| path.starts_with(dir))
        })
        .build();

    for entry in walker.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let filename = path.file_name().unwrap_or_default().to_string_lossy();
        if filename.contains(".expanded.") {
            continue;
        }

        let is_ts_file = path
            .extension()
            .is_some_and(|ext| ext == "ts" || ext == "tsx")
            && !filename.ends_with(".d.ts");

        if is_ts_file {
            ts_files.push(path.to_path_buf());
        } else if opts.out_dir.is_some() {
            passthrough_files.push(path.to_path_buf());
        }
    }

    let files_found = ts_files.len();

    // Phase 2: Expand candidates in parallel (no filesystem writes in workers).
    let pool = rayon::ThreadPoolBuilder::new().build()?;

    let results: Vec<(PathBuf, Result<Option<FileExpansion>>)> = pool.install(|| {
        ts_files
            .par_iter()
            .map(|path| (path.clone(), expand_file_in_memory(path)))
            .collect()
    });

    // Phase 3: Emit sequentially, mirroring each source's path under the outputs.
    let mut files_expanded = 0;
    let mut failures = 0;
    for (path, result) in &results {
        let rel = path.strip_prefix(&root).unwrap_or(path);
        match result {
            Ok(Some(file)) => {
                files_expanded += 1;
                emit_diagnostics(&file.expansion, &file.source, path);

                if let Some(out_dir) = &opts.out_dir {
                    write_file(&out_dir.join(rel), &file.expansion.code)?;
                }
                if let Some(types_dir) = &opts.types_out_dir
                    && let Some(types) = file.expansion.type_output.as_ref()
                {
                    write_file(&types_dir.join(type_surface_rel_path(rel)), types)?;
                }
                if opts.emit_expanded {
                    write_file(&get_expanded_path(path), &file.expansion.code)?;
                }
            }
            Ok(None) => {
                // No macros: keep the mirrored tree complete by copying verbatim.
                if let Some(out_dir) = &opts.out_dir {
                    copy_file(path, &out_dir.join(rel))?;
                }
            }
            Err(e) => {
                failures += 1;
                eprintln!("[macroforge] error expanding {}: {}", rel.display(), e);
            }
        }
    }

    // Copy non-TypeScript files verbatim so the staging tree is packager-ready.
    if let Some(out_dir) = &opts.out_dir {
        for path in &passthrough_files {
            let rel = path.strip_prefix(&root).unwrap_or(path);
            copy_file(path, &out_dir.join(rel))?;
        }
    }

    eprintln!(
        "[macroforge] scan complete: {} files found, {} expanded",
        files_found, files_expanded
    );

    if failures > 0 {
        return Err(anyhow!(
            "{} file(s) failed to expand under {}",
            failures,
            root.display()
        ));
    }

    Ok(())
}

/// Resolves `path` to an absolute location, canonicalizing the nearest existing
/// ancestor so symlinked prefixes (e.g. macOS `/tmp` → `/private/tmp`) match the
/// canonicalized scan root. The target itself need not exist yet.
fn canonicalized_target(path: &Path) -> PathBuf {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map(|cwd| cwd.join(path))
            .unwrap_or_else(|_| path.to_path_buf())
    };

    let mut existing = absolute.as_path();
    let mut tail: Vec<std::ffi::OsString> = Vec::new();
    loop {
        if let Ok(canonical) = existing.canonicalize() {
            let mut resolved = canonical;
            for component in tail.iter().rev() {
                resolved.push(component);
            }
            return resolved;
        }
        match (existing.file_name(), existing.parent()) {
            (Some(name), Some(parent)) => {
                tail.push(name.to_os_string());
                existing = parent;
            }
            _ => return absolute,
        }
    }
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
    match try_expand_file(input.clone(), out, types_out, print)? {
        true => Ok(()),
        false => {
            if !quiet {
                eprintln!("[macroforge] no macros found in {}", input.display());
            }
            std::process::exit(2);
        }
    }
}

/// A source file together with the result of expanding its macros.
pub(crate) struct FileExpansion {
    /// The original source text (needed to resolve diagnostic spans).
    pub source: String,
    /// The macro expansion result.
    pub expansion: MacroExpansion,
}

/// Expands macros in a file entirely in memory, performing no filesystem writes.
///
/// Uses the Rust-native `MacroExpander` (the fast, Node-free path). Callers are
/// responsible for emitting diagnostics and routing output. This is the shared
/// core used by both single-file expansion and directory scans; keeping it
/// write-free lets scans expand in parallel and emit sequentially.
///
/// ## Configuration Loading
///
/// Searches for and loads `macroforge.config.ts/js` to enable foreign type
/// handlers, parsed natively using SWC without requiring Node.js.
///
/// # Returns
///
/// - `Ok(Some(_))` - Macros were found and successfully expanded
/// - `Ok(None)` - No macros were found (the source is unchanged)
/// - `Err(...)` - An error occurred while reading or expanding the file
pub(crate) fn expand_file_in_memory(input: &Path) -> Result<Option<FileExpansion>> {
    use macroforge_ts::host::MacroforgeConfigLoader;

    // Load config if available (for foreign types support).
    // Foreign types are set on the registry before expansion; source imports
    // are built from the AST during prepare_expansion_context / expand_source.
    if let Ok(Some(config)) = MacroforgeConfigLoader::find_from_path(input) {
        macroforge_ts::host::set_foreign_types(config.foreign_types.clone());
    }

    let source = fs::read_to_string(input)
        .with_context(|| format!("failed to read {}", input.display()))?;

    let mut expander = MacroExpander::new().context("failed to initialize macro expander")?;

    // Load the project-wide type and declarative registries from the
    // `.macroforge/` cache so generic aliases (`RecordLink<T>`) and
    // cross-file `/** import macro */` comments resolve at expansion time.
    // Macroforge is built around these registries — running without them
    // silently degrades codegen (e.g. union @default variants emit
    // `undefined` instead of the proper `xDefaultValue<T>()` call).
    //
    // Skip the scan when the source has no macro annotations: `expand_source`
    // short-circuits in that case anyway, and `ensure_type_registry_cache`
    // would otherwise emit "[macroforge] Type scan: …" stderr noise that
    // breaks tools relying on a clean stderr in `--quiet` mode.
    if super::cache::has_macro_annotations(&source) {
        super::wrappers::ensure_type_registry_cache();
        let registry_path = super::wrappers::TYPE_REGISTRY_CACHE_PATH
            .lock()
            .unwrap()
            .clone();
        if let Some(ref rp) = registry_path
            && let Ok(json) = fs::read_to_string(rp)
            && let Ok(registry) = serde_json::from_str::<
                macroforge_ts::ts_syn::abi::ir::type_registry::TypeRegistry,
            >(&json)
        {
            expander.set_type_registry(registry);
        }
        let declarative_registry_path = super::wrappers::DECLARATIVE_REGISTRY_CACHE_PATH
            .lock()
            .unwrap()
            .clone();
        if let Some(ref dp) = declarative_registry_path
            && let Ok(json) = fs::read_to_string(dp)
            && let Ok(registry) =
                macroforge_ts::host::declarative::ProjectDeclarativeRegistry::from_json(&json)
        {
            expander.set_declarative_registry(Some(registry));
        }
    }

    let expansion = expander
        .expand_source(&source, &input.display().to_string())
        .map_err(|err| anyhow!(format!("{err:?}")))?;

    // Single cleanup
    macroforge_ts::host::clear_registry();
    macroforge_ts::host::clear_foreign_types();

    if !expansion.changed {
        return Ok(None);
    }

    Ok(Some(FileExpansion { source, expansion }))
}

/// Expands a single file and routes its output (used by the single-file path).
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
) -> Result<bool> {
    let Some(FileExpansion { source, expansion }) = expand_file_in_memory(&input)? else {
        return Ok(false);
    };

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
fn write_file(path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(path, contents).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

/// Copies a file verbatim, creating parent directories as needed.
///
/// Used when mirroring a scanned tree into an output directory: files with no
/// macros (and non-TypeScript assets) are copied unchanged so the destination is
/// a complete, packager-ready copy of the sources.
fn copy_file(src: &Path, dest: &Path) -> Result<()> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::copy(src, dest)
        .with_context(|| format!("failed to copy {} to {}", src.display(), dest.display()))?;
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

/// Maps a source-relative path to the path of its generated `.d.ts` type surface,
/// preserving directory structure and any middle extensions.
///
/// Examples: `User.ts` → `User.d.ts`, `types/person-name.svelte.ts` →
/// `types/person-name.svelte.d.ts`, `Button.tsx` → `Button.d.ts`.
pub(crate) fn type_surface_rel_path(rel: &Path) -> PathBuf {
    let filename = rel.file_name().unwrap_or_default().to_string_lossy();
    let stem = filename
        .strip_suffix(".ts")
        .or_else(|| filename.strip_suffix(".tsx"))
        .unwrap_or(&filename);
    let new_name = format!("{stem}.d.ts");

    match rel.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent.join(new_name),
        _ => PathBuf::from(new_name),
    }
}
