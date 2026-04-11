//! Project-wide registry of declarative macros.
//!
//! Populated by [`crate::host::scanner::ProjectScanner`] during the
//! single pre-expansion project walk. Keyed by absolute file path, and
//! for each file stores the map of `$name` → parsed [`MacroDef`].
//!
//! The scanner produces this alongside the existing `TypeRegistry` so
//! cross-file macro imports can resolve without a second file-tree
//! traversal.
//!
//! The registry is `Serialize + Deserialize` so it can cross the NAPI /
//! WASM boundary as JSON, mirroring how `TypeRegistry` is threaded
//! through `ExpandOptions.type_registry_json`.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::ts_syn::declarative::MacroDef;

/// Project-wide declarative macro registry keyed by absolute file path.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectDeclarativeRegistry {
    /// Map of `absolute_file_path` → (`$name` sans `$` → parsed `MacroDef`).
    ///
    /// File paths are stored as `String` for JSON friendliness; convert to
    /// `PathBuf` on lookup if needed.
    by_file: HashMap<String, HashMap<String, MacroDef>>,
}

impl ProjectDeclarativeRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert all macros discovered in a single file. The `file_path` must be
    /// an absolute path — the Vite plugin and CLI both hand absolute paths
    /// to `expand_sync`, so lookups from the host will match this key.
    pub fn insert_file(&mut self, file_path: impl Into<String>, macros: Vec<MacroDef>) {
        if macros.is_empty() {
            return;
        }
        let mut entry: HashMap<String, MacroDef> = HashMap::with_capacity(macros.len());
        for def in macros {
            entry.insert(def.name.clone(), def);
        }
        self.by_file.insert(file_path.into(), entry);
    }

    /// Look up the macros declared in a given file.
    pub fn file_macros(&self, file_path: &Path) -> Option<&HashMap<String, MacroDef>> {
        self.by_file.get(file_path.to_string_lossy().as_ref())
    }

    /// Look up a specific macro by (file, name).
    pub fn lookup(&self, file_path: &Path, name: &str) -> Option<&MacroDef> {
        self.file_macros(file_path).and_then(|m| m.get(name))
    }

    /// True if no files contained any declarative macros.
    pub fn is_empty(&self) -> bool {
        self.by_file.is_empty()
    }

    /// Number of files with at least one declarative macro.
    pub fn file_count(&self) -> usize {
        self.by_file.len()
    }

    /// Total macro count across all files.
    pub fn macro_count(&self) -> usize {
        self.by_file.values().map(|m| m.len()).sum()
    }

    /// Iterate over (file_path, name → MacroDef) entries.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &HashMap<String, MacroDef>)> {
        self.by_file.iter()
    }

    /// Resolve an import specifier (`"./foo"`, `"../bar/baz"`) relative to an
    /// importer file path, trying `.ts`, `.tsx`, and `index.ts` forms in
    /// order. Returns the first candidate that exists in the registry, or
    /// `None` if no candidate matches.
    pub fn resolve_specifier(&self, importer: &Path, specifier: &str) -> Option<PathBuf> {
        let parent = importer.parent()?;
        let base = normalize_path(&parent.join(specifier));

        // Candidate list: the bare path (in case the specifier already had an
        // extension), then `.ts` / `.tsx`, then the `index` forms.
        let mut candidates: Vec<PathBuf> = Vec::with_capacity(5);
        candidates.push(base.clone());
        if base.extension().is_none() {
            let mut with_ts = base.clone();
            with_ts.set_extension("ts");
            candidates.push(with_ts);
            let mut with_tsx = base.clone();
            with_tsx.set_extension("tsx");
            candidates.push(with_tsx);
        }
        candidates.push(base.join("index.ts"));
        candidates.push(base.join("index.tsx"));

        for candidate in candidates {
            let key = candidate.to_string_lossy();
            if self.by_file.contains_key(key.as_ref()) {
                return Some(candidate);
            }
        }
        None
    }

    /// Serialize to JSON for crossing the NAPI/WASM boundary.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserialize from JSON produced by [`to_json`].
    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}

/// Normalize a path by collapsing `.` components and resolving `..` where
/// possible without touching the filesystem. Used so that registry key
/// comparisons match regardless of whether the importer used `./` or not.
///
/// This is deliberately filesystem-free — `std::fs::canonicalize` would
/// require the file to exist, which it doesn't during unit tests (and may
/// not during scanning if the user deleted a file between buildStart and
/// transform).
fn normalize_path(path: &Path) -> PathBuf {
    use std::path::Component;

    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                // Pop the last component if it's a normal segment; otherwise
                // preserve the `..` (we can't collapse past a root).
                if !out.as_os_str().is_empty()
                    && out
                        .components()
                        .next_back()
                        .is_some_and(|c| matches!(c, Component::Normal(_)))
                {
                    out.pop();
                } else {
                    out.push(component.as_os_str());
                }
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}
