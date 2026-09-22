//! Project discovery shared by the diagnostic runners

use anyhow::{Context, Result};
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

/// Directory names that never hold a project of ours.
const SKIPPED_DIRS: &[&str] = &["node_modules", "dist", "target", "build", "vendor"];

/// Trees of test inputs that are broken on purpose: the tests that own them
/// assert the exact diagnostics they produce. Relative to the repo root.
const FIXTURE_ROOTS: &[&str] = &[
    "packages/svelte-language-server/test",
    "tooling/tests/e2e/fixtures",
];

/// Directories under `root` holding any of `markers`, honouring `.gitignore`
/// and leaving out dependency, build and fixture trees.
pub fn find_projects(root: &Path, markers: &[&str]) -> Result<Vec<PathBuf>> {
    let fixture_roots: Vec<PathBuf> = FIXTURE_ROOTS.iter().map(|path| root.join(path)).collect();
    let mut projects = Vec::new();

    for entry in WalkBuilder::new(root)
        .hidden(true)
        .git_ignore(true)
        .git_exclude(true)
        .filter_entry(move |entry| {
            let name = entry.file_name().to_string_lossy();
            !SKIPPED_DIRS.contains(&name.as_ref())
                && !fixture_roots.iter().any(|fixture| entry.path() == fixture)
        })
        .build()
    {
        let entry = entry.with_context(|| format!("failed to walk {}", root.display()))?;
        let name = entry.file_name().to_string_lossy();
        if markers.contains(&name.as_ref())
            && let Some(parent) = entry.path().parent()
            && !projects.iter().any(|project: &PathBuf| project == parent)
        {
            projects.push(parent.to_path_buf());
        }
    }

    Ok(projects)
}

/// Whether a directory is a Svelte project, which svelte-check covers.
pub fn is_svelte_project(dir: &Path) -> bool {
    SVELTE_MARKERS
        .iter()
        .any(|marker| dir.join(marker).exists())
}

pub const SVELTE_MARKERS: &[&str] = &["svelte.config.js", "svelte.config.ts"];
