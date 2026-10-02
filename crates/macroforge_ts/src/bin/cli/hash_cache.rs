//! Hashes of the files every run reads, kept while each file's stat holds.
//!
//! Deciding whether a build is current means hashing the whole project's
//! sources and every installed macro package. Almost none of those files
//! change between runs, so each one's hashes are kept with its modification
//! time and length and reused while both are unchanged. The hashes themselves
//! are computed exactly as before, so every aggregate built from them is the
//! same value it always was.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::atomic_fs::write_atomic;
use crate::package_state::{StatStamp, stamp_bytes};

/// Where the hashes persist, inside the project's existing `.macroforge/`.
const FILE_NAME: &str = "file-hashes.bin";

/// What one file hashed to at one stat. Each hash is filled in the first time
/// something asks for it.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct Known {
    stat: StatStamp,
    /// The file's length and SHA-256.
    sized: Option<(u64, String)>,
    /// The SHA-256 of its whitespace-normalized text.
    normalized: Option<String>,
    /// Whether it carries the generated `__macroforgeRun` exports.
    macro_exports: Option<bool>,
}

/// The hashes a persisted cache starts with, so another release's are never
/// read as this one's.
#[derive(serde::Serialize, serde::Deserialize)]
struct Persisted {
    version: String,
    entries: HashMap<PathBuf, Known>,
}

#[derive(Default)]
pub(crate) struct HashCache {
    entries: HashMap<PathBuf, Known>,
    /// Every file asked about this run; the rest are dropped on save.
    visited: HashSet<PathBuf>,
    dirty: bool,
}

impl HashCache {
    /// The hashes persisted under `root`. A missing file is an empty cache;
    /// one from another release, or unreadable, is reported and starts empty,
    /// since every entry can be recomputed.
    pub(crate) fn load(root: &Path) -> Self {
        let path = root.join(".macroforge").join(FILE_NAME);
        let bytes = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Self::default(),
            Err(error) => {
                eprintln!(
                    "[macroforge] warning: could not read {}: {error}",
                    path.display()
                );
                return Self::default();
            }
        };
        match postcard::from_bytes::<Persisted>(&bytes) {
            Ok(persisted) if persisted.version == env!("CARGO_PKG_VERSION") => Self {
                entries: persisted.entries,
                ..Self::default()
            },
            Ok(_) => Self::default(),
            Err(error) => {
                eprintln!(
                    "[macroforge] warning: discarding the unreadable {}: {error}",
                    path.display()
                );
                Self::default()
            }
        }
    }

    /// Persists the hashes of the files this run asked about, when anything
    /// changed. Only into a `.macroforge/` the project already has: a project
    /// without macros never gets one. The file is a cache, so a failure to
    /// write it is reported and the run goes on.
    pub(crate) fn save(self, root: &Path) {
        if let Err(error) = self.write(root) {
            eprintln!("[macroforge] warning: could not save the file hashes: {error:#}");
        }
    }

    fn write(mut self, root: &Path) -> Result<()> {
        let dir = root.join(".macroforge");
        let before = self.entries.len();
        self.entries.retain(|path, _| self.visited.contains(path));
        if !(self.dirty || self.entries.len() != before) || !dir.is_dir() {
            return Ok(());
        }
        let bytes = postcard::to_stdvec(&Persisted {
            version: env!("CARGO_PKG_VERSION").to_string(),
            entries: self.entries,
        })
        .context("failed to encode the file hashes")?;
        write_atomic(&dir.join(FILE_NAME), &bytes)
    }

    /// The entry for `path` at its current stat, or `None` when the file has
    /// no stat worth trusting (unreadable, or written a moment ago).
    fn entry(&mut self, path: &Path) -> Option<&mut Known> {
        self.visited.insert(path.to_path_buf());
        let stat = StatStamp::of(&fs::metadata(path).ok()?)?;
        let known = self
            .entries
            .entry(path.to_path_buf())
            .or_insert_with(|| Known {
                stat,
                sized: None,
                normalized: None,
                macro_exports: None,
            });
        if known.stat != stat {
            *known = Known {
                stat,
                sized: None,
                normalized: None,
                macro_exports: None,
            };
        }
        Some(known)
    }

    /// The SHA-256 of `path`'s whitespace-normalized text, or of its bytes
    /// when it is not text.
    pub(crate) fn normalized_hash(&mut self, path: &Path) -> Result<String> {
        if let Some(hash) = self.entry(path).and_then(|known| known.normalized.clone()) {
            return Ok(hash);
        }
        let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
        let hash = stamp_bytes(&bytes, true, None).normalized_hash;
        if let Some(known) = self.entry(path) {
            known.normalized = Some(hash.clone());
            self.dirty = true;
        }
        Ok(hash)
    }

    /// `path`'s length and SHA-256, or `None` when it cannot be read.
    pub(crate) fn sized_hash(&mut self, path: &Path) -> Option<(u64, String)> {
        if let Some(sized) = self.entry(path).and_then(|known| known.sized.clone()) {
            return Some(sized);
        }
        let bytes = fs::read(path).ok()?;
        let sized = (bytes.len() as u64, crate::cache::content_hash(&bytes));
        if let Some(known) = self.entry(path) {
            known.sized = Some(sized.clone());
            self.dirty = true;
        }
        Some(sized)
    }

    /// Whether the script at `path` carries the generated `__macroforgeRun`
    /// exports. A script that cannot be read is not part of a package anything
    /// can load, so it does not qualify one.
    pub(crate) fn has_macro_exports(&mut self, path: &Path) -> bool {
        if let Some(known) = self.entry(path).and_then(|known| known.macro_exports) {
            return known;
        }
        let found =
            fs::read_to_string(path).is_ok_and(|content| content.contains("__macroforgeRun"));
        if let Some(known) = self.entry(path) {
            known.macro_exports = Some(found);
            self.dirty = true;
        }
        found
    }
}

#[cfg(test)]
mod tests {
    use super::HashCache;
    use std::path::Path;
    use std::time::{Duration, SystemTime};

    /// Writes `contents` with an mtime old enough for its stat to be trusted.
    fn write_settled(path: &Path, contents: &str) {
        std::fs::write(path, contents).expect("write fixture");
        std::fs::File::options()
            .write(true)
            .open(path)
            .and_then(|file| file.set_modified(SystemTime::now() - Duration::from_secs(60)))
            .expect("backdate fixture");
    }

    #[test]
    fn hashes_persist_and_follow_a_changed_file() {
        let tmp = tempfile::tempdir().expect("temp dir");
        let root = tmp.path();
        std::fs::create_dir(root.join(".macroforge")).expect("state dir");
        let file = root.join("a.ts");
        write_settled(&file, "export const a = 1;\n");

        let mut hashes = HashCache::load(root);
        let first = hashes.normalized_hash(&file).expect("hash");
        hashes.save(root);

        let mut reloaded = HashCache::load(root);
        assert!(reloaded.entries.contains_key(&file), "the hash persisted");
        assert_eq!(reloaded.normalized_hash(&file).expect("hash"), first);

        write_settled(&file, "export const a = 22;\n");
        assert_ne!(
            reloaded.normalized_hash(&file).expect("hash"),
            first,
            "a changed file is hashed again"
        );
    }

    #[test]
    fn a_project_without_macroforge_state_gets_none() {
        let tmp = tempfile::tempdir().expect("temp dir");
        let file = tmp.path().join("a.ts");
        write_settled(&file, "export const a = 1;\n");

        let mut hashes = HashCache::load(tmp.path());
        hashes.normalized_hash(&file).expect("hash");
        hashes.save(tmp.path());
        assert!(!tmp.path().join(".macroforge").exists());
    }
}
