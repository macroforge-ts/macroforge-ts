//! Generated documentation as data.
//!
//! Every generator returns the files it produces instead of writing them, so
//! regenerating (`write`) and verifying that the committed docs are current
//! (`stale`) are two uses of the same output and cannot drift apart.

use crate::core::shell;
use anyhow::{Context, Result};
use rayon::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// The files one or more generators produce, keyed by repo-relative path,
/// each holding exactly what is committed: formatted the way `deno fmt`
/// leaves it.
#[derive(Default)]
pub struct GeneratedFiles {
    files: BTreeMap<PathBuf, String>,
    /// Repo-relative directories whose every file a generator produces, so
    /// one it no longer produces is an orphan.
    owned_dirs: Vec<PathBuf>,
}

/// Why a committed file does not match its generator.
pub enum Staleness {
    Missing,
    Outdated,
    Orphaned,
}

pub struct StaleFile {
    pub path: PathBuf,
    pub staleness: Staleness,
}

impl GeneratedFiles {
    /// Formats `raw` files the way `deno fmt` would in their destination, in
    /// parallel since each is a `deno fmt` process.
    pub fn build(
        root: &Path,
        owned_dirs: Vec<PathBuf>,
        raw: Vec<(PathBuf, String)>,
    ) -> Result<Self> {
        let files = raw
            .into_par_iter()
            .map(|(path, content)| {
                let formatted = format_as_committed(root, &path, content)
                    .with_context(|| format!("failed to format {}", path.display()))?;
                Ok((path, formatted))
            })
            .collect::<Result<BTreeMap<_, _>>>()?;
        Ok(Self { files, owned_dirs })
    }

    pub fn extend(&mut self, other: GeneratedFiles) {
        self.files.extend(other.files);
        self.owned_dirs.extend(other.owned_dirs);
    }

    /// Writes every file whose content changed and deletes orphans. Returns
    /// how many files changed.
    pub fn write(&self, root: &Path) -> Result<usize> {
        let mut changed = 0;
        for (path, content) in &self.files {
            let absolute = root.join(path);
            if fs::read_to_string(&absolute).is_ok_and(|current| current == *content) {
                continue;
            }
            if let Some(parent) = absolute.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("failed to create {}", parent.display()))?;
            }
            fs::write(&absolute, content)
                .with_context(|| format!("failed to write {}", absolute.display()))?;
            changed += 1;
        }
        for orphan in self.orphans(root)? {
            let absolute = root.join(&orphan);
            fs::remove_file(&absolute)
                .with_context(|| format!("failed to remove {}", absolute.display()))?;
            changed += 1;
        }
        for dir in &self.owned_dirs {
            remove_empty_dirs(&root.join(dir))?;
        }
        Ok(changed)
    }

    /// The committed files that differ from what the generators produce.
    pub fn stale(&self, root: &Path) -> Result<Vec<StaleFile>> {
        let mut stale = Vec::new();
        for (path, content) in &self.files {
            let absolute = root.join(path);
            let staleness = match fs::read_to_string(&absolute) {
                Ok(current) if current == *content => continue,
                Ok(_) => Staleness::Outdated,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => Staleness::Missing,
                Err(error) => {
                    return Err(error)
                        .with_context(|| format!("failed to read {}", absolute.display()));
                }
            };
            stale.push(StaleFile {
                path: path.clone(),
                staleness,
            });
        }
        stale.extend(self.orphans(root)?.into_iter().map(|path| StaleFile {
            path,
            staleness: Staleness::Orphaned,
        }));
        Ok(stale)
    }

    /// Files in an owned directory that no generator produced.
    fn orphans(&self, root: &Path) -> Result<BTreeSet<PathBuf>> {
        let mut orphans = BTreeSet::new();
        for dir in &self.owned_dirs {
            let absolute = root.join(dir);
            if !absolute.exists() {
                continue;
            }
            for entry in WalkDir::new(&absolute) {
                let entry =
                    entry.with_context(|| format!("failed to walk {}", absolute.display()))?;
                if !entry.file_type().is_file() {
                    continue;
                }
                let relative = entry
                    .path()
                    .strip_prefix(root)
                    .with_context(|| {
                        format!("{} is outside the repository", entry.path().display())
                    })?
                    .to_path_buf();
                if !self.files.contains_key(&relative) {
                    orphans.insert(relative);
                }
            }
        }
        Ok(orphans)
    }
}

/// Formats a generated file as `deno fmt` would in its destination, whose
/// nearest deno.json supplies the options. Formats deno does not handle, such
/// as `.svx`, are committed as generated.
fn format_as_committed(root: &Path, path: &Path, content: String) -> Result<String> {
    let Some(extension) = path.extension().and_then(|extension| extension.to_str()) else {
        return Ok(content);
    };
    if !matches!(extension, "md" | "json") {
        return Ok(content);
    }
    let destination = root.join(path);
    let config_dir = destination
        .ancestors()
        .skip(1)
        .find(|dir| dir.is_dir())
        .with_context(|| format!("{} has no existing parent directory", destination.display()))?;
    shell::deno::format(config_dir, extension, &content)
}

/// Removes the empty directories under `dir`, deepest first, keeping `dir`.
fn remove_empty_dirs(dir: &Path) -> Result<()> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in WalkDir::new(dir).min_depth(1).contents_first(true) {
        let entry = entry.with_context(|| format!("failed to walk {}", dir.display()))?;
        if !entry.file_type().is_dir() {
            continue;
        }
        let is_empty = fs::read_dir(entry.path())
            .with_context(|| format!("failed to read {}", entry.path().display()))?
            .next()
            .is_none();
        if is_empty {
            fs::remove_dir(entry.path())
                .with_context(|| format!("failed to remove {}", entry.path().display()))?;
        }
    }
    Ok(())
}
