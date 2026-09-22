use anyhow::{Context, Result, anyhow};
use ignore::WalkBuilder;
use macroforge_ts::host::MacroExpansion;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::atomic_fs::write_atomic;

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
/// `project_root` is the resolved project root that owns `.macroforge/`. It is
/// distinct from `root`: a scan is frequently rooted at a subdirectory
/// (`--scan src/`) while the registries it reads belong to the project above it.
///
/// # Returns
///
/// Returns `Ok(())` on success. Returns an error if any file fails to expand —
/// a partially-populated staging tree must never silently feed a packager.
pub fn scan_and_expand(project_root: &Path, root: PathBuf, opts: ScanOptions) -> Result<()> {
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
    let pool = super::cache::expansion_pool()?;

    let results: Vec<(PathBuf, Result<Option<FileExpansion>>)> = pool.install(|| {
        ts_files
            .par_iter()
            .map(|path| (path.clone(), expand_file_in_memory(project_root, path)))
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
/// * `project_root` - The resolved project root that owns `.macroforge/`
/// * `input` - Path to the input TypeScript file
/// * `out` - Optional path for the expanded output (default: `input.expanded.ts`)
/// * `types_out` - Optional path for the `.d.ts` type output
/// * `print` - If true, also print expanded code to stdout
/// * `quiet` - If true, suppress output when no macros are found
///
/// # Exit Codes
///
/// Calls `std::process::exit(2)` whenever no macros are found; `quiet`
/// only suppresses the stderr message, not the exit code.
pub fn expand_file(
    project_root: &Path,
    input: PathBuf,
    out: Option<PathBuf>,
    types_out: Option<PathBuf>,
    print: bool,
    quiet: bool,
) -> Result<()> {
    match try_expand_file(project_root, input.clone(), out, types_out, print)? {
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
/// The expander comes from [`super::cache::configured_expander`], so the
/// file's macroforge config and the project registries apply exactly as they
/// do in a build.
///
/// # Returns
///
/// - `Ok(Some(_))` - Macros were found and successfully expanded
/// - `Ok(None)` - No macros were found (the source is unchanged)
/// - `Err(...)` - An error occurred while reading or expanding the file
pub(crate) fn expand_file_in_memory(
    project_root: &Path,
    input: &Path,
) -> Result<Option<FileExpansion>> {
    let source =
        fs::read_to_string(input).with_context(|| format!("failed to read {}", input.display()))?;

    // The registry scan logs to stderr, so skip it for files without macros:
    // `--quiet` callers rely on a clean stderr.
    if !macroforge_ts::has_macro_annotations(&source) {
        return Ok(None);
    }
    super::wrappers::ensure_type_registry_cache(project_root)?;
    let expander = super::cache::configured_expander(project_root, input)?;

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
    project_root: &Path,
    input: PathBuf,
    out: Option<PathBuf>,
    types_out: Option<PathBuf>,
    print: bool,
) -> Result<bool> {
    let Some(FileExpansion { source, expansion }) = expand_file_in_memory(project_root, &input)?
    else {
        return Ok(false);
    };

    emit_diagnostics(&expansion, &source, &input);
    emit_runtime_output(&expansion, &input, out.as_ref(), print)?;
    emit_type_output(&expansion, &input, types_out.as_ref(), print)?;

    Ok(true)
}

/// Routes one file's expanded runtime code: to `explicit_out` when given,
/// otherwise to stdout. `should_print` also prints it when it went to a file.
fn emit_runtime_output(
    result: &MacroExpansion,
    input: &Path,
    explicit_out: Option<&PathBuf>,
    should_print: bool,
) -> Result<()> {
    let code = &result.code;
    if let Some(out_path) = explicit_out {
        write_file(out_path, code)?;
        eprintln!(
            "[macroforge] wrote expanded output for {} to {}",
            input.display(),
            out_path.display()
        );
        if !should_print {
            return Ok(());
        }
    }
    println!("// --- {} (expanded) ---", input.display());
    println!("{code}");
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

/// Writes content to a file atomically, creating parent directories as needed.
///
/// A scan's `--out` tree is consumed by a packager and its `--types-out` tree by
/// a type checker, often while the scan is still running; each file has to
/// appear complete or not at all.
fn write_file(path: &Path, contents: &str) -> Result<()> {
    write_atomic(path, contents.as_bytes())
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
