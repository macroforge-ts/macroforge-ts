//! The expansion pass behind `macroforge svelte-package`.
//!
//! Expansion used to happen lazily, inside the packager, on every read of a
//! source file — which meant twice per macro module, because `@sveltejs/package`
//! reads each one once for the `.d.ts` emit and once for the JS emit, and both
//! reads went through the macro engine over Node's single thread.
//!
//! Doing it once, up front, in Rust replaces that with one expansion per changed
//! file across every core, and turns the result into something that can outlive
//! the run: the expanded tree persists under `.macroforge/`, so a file whose
//! source did not move keeps the artifact from last time. The packager reads
//! through it by path redirect, never knowing expansion happened.
//!
//! Failures are fatal here. The lazy path swallowed them and handed the packager
//! the unexpanded source, which publishes a library whose macro-generated
//! runtime is silently missing — the exact defect this command exists to
//! prevent. A build that cannot expand a file has no correct output to produce.

use anyhow::{Context, Result, bail};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use crate::cache::{CacheExpansion, expand_for_cache};
use crate::package_state::FileStamp;

/// What the pass did, and what the packager should redirect.
pub(crate) struct ExpansionOutcome {
    /// Files expanded on this run.
    pub(crate) expanded: usize,
    /// Every path currently backed by an expanded artifact, `/`-separated and
    /// relative to the input directory. This is the whole tree, not just this
    /// run's work: the packager has to redirect reads of files that were
    /// expanded on an earlier run too.
    pub(crate) entries: Vec<String>,
}

/// Whether a file is one the macro engine should see.
///
/// Matches what the packager's read hooks used to expand: TypeScript sources,
/// excluding declaration files, which carry no macro annotations and whose
/// expansion would be meaningless.
pub(crate) fn is_expandable(rel: &str) -> bool {
    (rel.ends_with(".ts") || rel.ends_with(".tsx")) && !rel.ends_with(".d.ts")
}

/// Expands `targets` into `expanded_dir` and reconciles the tree with `files`.
///
/// `targets` is the set of input-relative paths whose expansion may be out of
/// date — normally the changed files, or every file when the project's type
/// surface moved. Everything else keeps the artifact it already has.
pub(crate) fn run_expansion_pass(
    root: &Path,
    input: &Path,
    expanded_dir: &Path,
    targets: &[String],
    files: &BTreeMap<String, FileStamp>,
) -> Result<ExpansionOutcome> {
    use rayon::prelude::*;

    prune_orphans(expanded_dir, files)?;

    // Read sequentially: expansion is the expensive half, and reading first
    // keeps the parallel section free of I/O error handling.
    let mut work: Vec<(String, PathBuf, String)> = Vec::new();
    for rel in targets {
        if !is_expandable(rel) {
            continue;
        }
        let path = input.join(rel);
        let source = match fs::read_to_string(&path) {
            Ok(source) => source,
            // Removed between the scan and now. `prune_orphans` already dropped
            // any artifact for it, and the next run rescans.
            Err(_) => continue,
        };
        work.push((rel.clone(), path, source));
    }

    let pool = crate::cache::expansion_pool()?;

    let results: Vec<(String, Result<Option<CacheExpansion>>)> = pool.install(|| {
        work.par_iter()
            .map(|(rel, path, source)| (rel.clone(), expand_for_cache(root, path, source)))
            .collect()
    });

    let mut expanded = 0usize;
    let mut failures: Vec<String> = Vec::new();

    for (rel, result) in results {
        match result {
            // A macro that reported an error produced no output for whatever it
            // was supposed to generate. Writing the artifact anyway would
            // publish a module missing exactly that code, and nothing
            // downstream can tell the difference — the file still parses, still
            // type-checks against its own declarations, and simply does less.
            Ok(Some(expansion)) if !expansion.errors.is_empty() => {
                for error in &expansion.errors {
                    failures.push(format!("  {rel}: {error}"));
                }
            }
            Ok(Some(expansion)) => {
                write_entry(expanded_dir, &rel, &expansion.code)?;
                expanded += 1;
            }
            // No macros, or expansion left the source alone. Any artifact from
            // an earlier run is now wrong — the annotations were removed — so
            // the read must fall through to the real file.
            Ok(None) => remove_entry(expanded_dir, &rel)?,
            Err(e) => failures.push(format!("  {rel}: {e:#}")),
        }
    }

    if !failures.is_empty() {
        bail!(
            "macro expansion failed for {} file(s); the existing package was left untouched:\n{}",
            failures.len(),
            failures.join("\n")
        );
    }

    Ok(ExpansionOutcome {
        expanded,
        entries: collect_entries(expanded_dir)?,
    })
}

/// Writes one expanded artifact.
///
/// Written in place rather than atomically: nothing outside this process reads
/// the tree until the pass returns, and a run that dies midway does not save its
/// state, so the next one recomputes the file it was writing.
fn write_entry(expanded_dir: &Path, rel: &str, code: &str) -> Result<()> {
    let path = expanded_dir.join(rel);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(&path, code).with_context(|| format!("failed to write {}", path.display()))
}

fn remove_entry(expanded_dir: &Path, rel: &str) -> Result<()> {
    let path = expanded_dir.join(rel);
    match fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e).with_context(|| format!("failed to remove {}", path.display())),
    }
}

/// Drops artifacts whose source file no longer exists.
///
/// Without this a deleted module keeps being redirected to, so the packager
/// reads a file the project no longer has.
fn prune_orphans(expanded_dir: &Path, files: &BTreeMap<String, FileStamp>) -> Result<()> {
    for rel in collect_entries(expanded_dir)? {
        if !files.contains_key(&rel) {
            remove_entry(expanded_dir, &rel)?;
        }
    }
    prune_empty_dirs(expanded_dir)?;
    Ok(())
}

/// Removes directories left empty by pruning, so a deleted source subtree does
/// not leave its skeleton behind indefinitely.
fn prune_empty_dirs(dir: &Path) -> Result<bool> {
    if !dir.is_dir() {
        return Ok(false);
    }

    let mut empty = true;
    let entries = fs::read_dir(dir).with_context(|| format!("failed to read {}", dir.display()))?;
    for entry in entries {
        let entry =
            entry.with_context(|| format!("failed to read an entry in {}", dir.display()))?;
        let path = entry.path();
        if path.is_dir() {
            if !prune_empty_dirs(&path)? {
                empty = false;
            }
        } else {
            empty = false;
        }
    }

    if empty {
        // Best-effort: losing the race with something else that removed it is
        // the outcome we wanted anyway.
        let _ = fs::remove_dir(dir);
    }
    Ok(empty)
}

/// Lists every artifact in the tree, `/`-separated and relative to its root.
fn collect_entries(expanded_dir: &Path) -> Result<Vec<String>> {
    let mut out = Vec::new();
    if !expanded_dir.is_dir() {
        return Ok(out);
    }
    collect_entries_into(expanded_dir, "", &mut out)?;
    out.sort();
    Ok(out)
}

fn collect_entries_into(dir: &Path, rel: &str, out: &mut Vec<String>) -> Result<()> {
    let entries = fs::read_dir(dir).with_context(|| format!("failed to read {}", dir.display()))?;
    for entry in entries {
        let entry =
            entry.with_context(|| format!("failed to read an entry in {}", dir.display()))?;
        let name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();
        let child = if rel.is_empty() {
            name
        } else {
            format!("{rel}/{name}")
        };
        if path.is_dir() {
            collect_entries_into(&path, &child, out)?;
        } else {
            out.push(child);
        }
    }
    Ok(())
}
