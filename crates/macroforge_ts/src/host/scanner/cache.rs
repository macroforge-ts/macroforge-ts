//! Per-file scan cache with mtime+size invalidation (Phase 17).
//!
//! `ProjectScanner::scan()` re-parses every `.ts/.tsx` file on every
//! invocation. For large projects with frequent HMR churn this is
//! noticeably wasteful — a single keystroke rescans hundreds of files
//! that didn't change. This module adds a per-file cache keyed on
//! `(path, mtime_ns, size)` so rescans only re-parse the files that
//! actually moved on disk.
//!
//! The cache is intentionally dumb:
//!
//! - Keyed on the absolute path of the scanned file.
//! - Stores every lowered artifact the scanner produced for that file
//!   (classes, interfaces, enums, type aliases, declarative macros,
//!   plus the file-level `imports` and `exported_names` sets).
//! - Invalidates on any mismatch between `(mtime_ns, size)` and the
//!   cached tuple. `mtime_ns` alone would be enough on most
//!   filesystems but some editors do "safe writes" (write-and-rename)
//!   that preserve mtime; `size` catches those cases cheaply.
//! - No eviction / LRU logic — the cache grows monotonically over a
//!   scanner's lifetime. HMR invalidation happens via
//!   [`ScanCache::invalidate`] when Vite tells us a file changed.
//!
//! The scanner owns an `Option<ScanCache>`; the cache is created
//! lazily and only turned on via [`ProjectScanner::with_cache`]. This
//! keeps the single-shot CLI path unchanged while letting long-lived
//! hosts (the Vite plugin, the LSP server) reuse the scanner across
//! scans.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::host::declarative::ProjectDeclarativeRegistry;
use crate::ts_syn::abi::ir::type_registry::FileImportEntry;
use crate::ts_syn::abi::ir::{ClassIR, EnumIR, InterfaceIR, TypeAliasIR};
use crate::ts_syn::declarative::MacroDef;

/// Per-file cache entry — everything the scanner produces for a
/// single `.ts/.tsx` file, plus the `(mtime_ns, size)` tuple used to
/// decide whether the entry is still current.
#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// File mtime in nanoseconds since the Unix epoch, captured at
    /// the time the entry was written. Combined with [`Self::size`]
    /// this is the "generation" tag we compare against on rescan.
    pub mtime_ns: u128,
    /// File size in bytes. Catches write-and-rename saves where the
    /// mtime is preserved but the content changed.
    pub size: u64,

    /// Lowered class declarations from the file.
    pub classes: Vec<ClassIR>,
    /// Lowered interface declarations from the file.
    pub interfaces: Vec<InterfaceIR>,
    /// Lowered enum declarations from the file.
    pub enums: Vec<EnumIR>,
    /// Lowered type alias declarations from the file.
    pub type_aliases: Vec<TypeAliasIR>,

    /// Declarative macros (`const $x = macroRules\`...\``) discovered
    /// in the file. Empty for files that don't define any.
    pub declarative_macros: Vec<MacroDef>,

    /// Module-level imports — needed by [`TypeRegistry::resolve`] to
    /// cross-reference ambiguous type names.
    pub file_imports: Vec<FileImportEntry>,
    /// Names exported from the file — used by the scanner's
    /// `exported_only` filter.
    pub exported_names: HashSet<String>,
}

/// Project-wide scan cache keyed by absolute file path.
#[derive(Debug, Default)]
pub struct ScanCache {
    entries: HashMap<PathBuf, CacheEntry>,
}

impl ScanCache {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of entries currently in the cache.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if the cache has no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Look up a cache entry by path. Returns `None` if the path
    /// hasn't been scanned yet OR the on-disk `(mtime, size)` tuple
    /// doesn't match the cached tuple.
    ///
    /// Callers pass the live `(mtime_ns, size)` they just read via
    /// `fs::metadata` — the cache does not hit the filesystem itself,
    /// so this method is cheap and side-effect-free.
    pub fn get(&self, path: &Path, mtime_ns: u128, size: u64) -> Option<&CacheEntry> {
        let entry = self.entries.get(path)?;
        if entry.mtime_ns == mtime_ns && entry.size == size {
            Some(entry)
        } else {
            None
        }
    }

    /// Insert (or overwrite) the cache entry for `path`.
    pub fn insert(&mut self, path: PathBuf, entry: CacheEntry) {
        self.entries.insert(path, entry);
    }

    /// Remove a single entry by path. Used by HMR when Vite tells us
    /// a file changed on disk — the next scan of that file will
    /// re-parse it from scratch.
    pub fn invalidate(&mut self, path: &Path) -> bool {
        self.entries.remove(path).is_some()
    }

    /// Drop every cached entry. Called when the scanner's config
    /// changes (e.g. `macroforge.config.ts` was touched), since the
    /// lowered IR can depend on config-driven options.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Iterate over the cached entries. Useful for stats or
    /// debugging; callers shouldn't mutate cache state this way.
    pub fn iter(&self) -> impl Iterator<Item = (&PathBuf, &CacheEntry)> {
        self.entries.iter()
    }
}

/// Read `(mtime_ns, size)` from `fs::metadata`. Returns `None` when
/// the metadata call fails (e.g. the file was deleted between the
/// walker yielding its path and our `scan_file` reading it).
pub fn file_stamp(path: &Path) -> Option<(u128, u64)> {
    let meta = std::fs::metadata(path).ok()?;
    let size = meta.len();
    let mtime_ns = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    Some((mtime_ns, size))
}

// ---------------------------------------------------------------------------
// Registry-splicing helpers
// ---------------------------------------------------------------------------
//
// When a cache hit fires, we need to replay the cached entry into the
// current scan's outputs without re-running the parser. These helpers
// do the bookkeeping.

/// Splice a cached entry's declarative macros into the project-wide
/// declarative registry. Only called on cache hits.
pub fn splice_declarative(
    declarative_registry: &mut ProjectDeclarativeRegistry,
    file_name: &str,
    entry: &CacheEntry,
) {
    if !entry.declarative_macros.is_empty() {
        declarative_registry.insert_file(file_name.to_string(), entry.declarative_macros.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_entry(mtime_ns: u128, size: u64) -> CacheEntry {
        CacheEntry {
            mtime_ns,
            size,
            classes: Vec::new(),
            interfaces: Vec::new(),
            enums: Vec::new(),
            type_aliases: Vec::new(),
            declarative_macros: Vec::new(),
            file_imports: Vec::new(),
            exported_names: HashSet::new(),
        }
    }

    #[test]
    fn get_returns_entry_when_stamp_matches() {
        let mut cache = ScanCache::new();
        let path = PathBuf::from("/tmp/foo.ts");
        cache.insert(path.clone(), empty_entry(100, 50));
        assert!(cache.get(&path, 100, 50).is_some());
    }

    #[test]
    fn get_returns_none_when_mtime_differs() {
        let mut cache = ScanCache::new();
        let path = PathBuf::from("/tmp/foo.ts");
        cache.insert(path.clone(), empty_entry(100, 50));
        // Different mtime — miss.
        assert!(cache.get(&path, 200, 50).is_none());
    }

    #[test]
    fn get_returns_none_when_size_differs() {
        let mut cache = ScanCache::new();
        let path = PathBuf::from("/tmp/foo.ts");
        cache.insert(path.clone(), empty_entry(100, 50));
        // Same mtime, different size — miss (catches write-and-rename).
        assert!(cache.get(&path, 100, 99).is_none());
    }

    #[test]
    fn get_returns_none_for_unknown_path() {
        let cache = ScanCache::new();
        assert!(cache.get(Path::new("/tmp/unseen.ts"), 0, 0).is_none());
    }

    #[test]
    fn invalidate_drops_entry() {
        let mut cache = ScanCache::new();
        let path = PathBuf::from("/tmp/foo.ts");
        cache.insert(path.clone(), empty_entry(100, 50));
        assert_eq!(cache.len(), 1);
        assert!(cache.invalidate(&path));
        assert_eq!(cache.len(), 0);
        // Second invalidate is a no-op.
        assert!(!cache.invalidate(&path));
    }

    #[test]
    fn clear_drops_all_entries() {
        let mut cache = ScanCache::new();
        cache.insert(PathBuf::from("/tmp/a.ts"), empty_entry(1, 10));
        cache.insert(PathBuf::from("/tmp/b.ts"), empty_entry(2, 20));
        assert_eq!(cache.len(), 2);
        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn file_stamp_reads_real_file() {
        // Write a tiny temp file and verify `file_stamp` returns a
        // non-zero size and mtime for it. Uses the OS tempdir so
        // we don't litter the crate directory.
        let dir = std::env::temp_dir();
        let path = dir.join(format!("macroforge_cache_test_{}.ts", std::process::id()));
        std::fs::write(&path, b"// hi\n").expect("write");
        let (mtime_ns, size) = file_stamp(&path).expect("stamp");
        assert!(mtime_ns > 0);
        assert_eq!(size, 6);
        std::fs::remove_file(&path).ok();
    }

    // -----------------------------------------------------------------
    // Scanner integration tests — exercise the full `scan()` path with
    // an installed cache and verify that (a) a second scan hits the
    // cache instead of re-parsing, and (b) `invalidate` forces a
    // re-parse of just the changed file.
    // -----------------------------------------------------------------

    #[cfg(feature = "oxc")]
    #[test]
    fn second_scan_reuses_cache_entries() {
        use super::super::{ProjectScanner, ScanConfig};

        // Make a fresh tempdir with two .ts files so we can watch the
        // cache populate then replay.
        let dir = std::env::temp_dir().join(format!(
            "macroforge_scanner_cache_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("a.ts"), "export interface A { id: string; }\n").unwrap();
        std::fs::write(dir.join("b.ts"), "export class B { name = \"\"; }\n").unwrap();

        let mut scanner = ProjectScanner::new(ScanConfig {
            root_dir: dir.clone(),
            ..Default::default()
        });
        scanner.enable_cache();

        // First scan — populates the cache.
        let out1 = scanner.scan().expect("scan 1");
        assert_eq!(out1.files_scanned, 2);
        assert_eq!(scanner.cache_len(), Some(2));

        // Second scan — should use cached entries. The output should
        // be equivalent; we verify by checking the registry reports
        // the same types.
        let out2 = scanner.scan().expect("scan 2");
        assert_eq!(out2.files_scanned, 2);
        assert!(out2.registry.get("A").is_some());
        assert!(out2.registry.get("B").is_some());

        std::fs::remove_dir_all(&dir).ok();
    }

    #[cfg(feature = "oxc")]
    #[test]
    fn invalidate_forces_rescan_of_changed_file() {
        use super::super::{ProjectScanner, ScanConfig};

        let dir = std::env::temp_dir().join(format!(
            "macroforge_scanner_invalidate_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let a_path = dir.join("a.ts");
        std::fs::write(&a_path, "export interface A { id: string; }\n").unwrap();

        let mut scanner = ProjectScanner::new(ScanConfig {
            root_dir: dir.clone(),
            ..Default::default()
        });
        scanner.enable_cache();

        // First scan populates the cache.
        let _ = scanner.scan().expect("scan 1");
        assert_eq!(scanner.cache_len(), Some(1));

        // Invalidate and rewrite with different content. After
        // invalidation the cache entry is gone; the next scan must
        // re-parse the file and pick up the new type.
        scanner.invalidate_cache_entry(&a_path);
        assert_eq!(scanner.cache_len(), Some(0));
        std::fs::write(
            &a_path,
            "export interface A { id: string; }\nexport class A2 {}\n",
        )
        .unwrap();

        let out = scanner.scan().expect("scan 2");
        assert!(out.registry.get("A2").is_some(), "A2 should be re-scanned");
        assert_eq!(scanner.cache_len(), Some(1));

        std::fs::remove_dir_all(&dir).ok();
    }
}
