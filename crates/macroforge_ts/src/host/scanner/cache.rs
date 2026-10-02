//! Per-file scan cache with mtime+size invalidation.
//!
//! A scan lowers every `.ts/.tsx` file in the project. The cache keeps what
//! each file lowered to, keyed on its absolute path and `(mtime_ns, size)`,
//! so a rescan only parses the files that moved on disk.
//!
//! - Stores every lowered artifact the scanner produced for a file (classes,
//!   interfaces, enums, type aliases, declarative macros, the file's imports
//!   and exported names), shared by reference with every scan that reuses it.
//! - Invalidates on any mismatch between `(mtime_ns, size)` and the cached
//!   tuple. `size` catches "safe writes" (write-and-rename) that keep mtime.
//! - Does not trust the stamp of a file written in the last moments: another
//!   write within the filesystem's timestamp resolution could leave it equal.
//! - Persists across processes as `.macroforge/scan-cache.bin`, discarded
//!   whole when the macroforge version that wrote it differs.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::host::declarative::ProjectDeclarativeRegistry;
use crate::host::declarative::project_registry::FileMacros;
use crate::ts_syn::abi::ir::type_registry::FileImportEntry;
use crate::ts_syn::abi::ir::{ClassIR, EnumIR, InterfaceIR, TypeAliasIR};

/// Everything the scanner produces for a single `.ts/.tsx` file, plus the
/// `(mtime_ns, size)` tuple used to decide whether the entry is still current.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CacheEntry {
    /// File mtime in nanoseconds since the Unix epoch, captured at the time
    /// the entry was written. Combined with [`Self::size`] this is the
    /// "generation" tag compared against on rescan.
    pub mtime_ns: u128,
    /// File size in bytes. Catches write-and-rename saves where the mtime is
    /// preserved but the content changed.
    pub size: u64,

    /// Lowered class declarations from the file.
    pub classes: Vec<ClassIR>,
    /// Lowered interface declarations from the file.
    pub interfaces: Vec<InterfaceIR>,
    /// Lowered enum declarations from the file.
    pub enums: Vec<EnumIR>,
    /// Lowered type alias declarations from the file.
    pub type_aliases: Vec<TypeAliasIR>,

    /// Declarative macros (`const $x = macroRules\`...\``) discovered in the
    /// file. Empty for files that don't define any.
    pub declarative_macros: Arc<FileMacros>,

    /// Module-level imports, needed by `TypeRegistry::resolve` to
    /// cross-reference ambiguous type names.
    pub file_imports: Vec<FileImportEntry>,
    /// Names exported from the file, used by the scanner's `exported_only`
    /// filter.
    pub exported_names: HashSet<String>,
    /// Whether the file contains anything the engine expands.
    pub uses_macros: bool,
}

/// Project-wide scan cache keyed by absolute file path.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ScanCache {
    entries: HashMap<PathBuf, Arc<CacheEntry>>,
}

/// What a persisted cache starts with, so one written by another release is
/// never read as this one's.
#[derive(serde::Serialize, serde::Deserialize)]
struct PersistedHeader {
    version: String,
}

/// Where a project's scan cache persists, under its root.
pub fn persisted_path(root: &Path) -> PathBuf {
    root.join(".macroforge").join("scan-cache.bin")
}

impl ScanCache {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// The cache persisted at `path` by this macroforge version. A missing
    /// file is an empty cache; one that is unreadable, from another version,
    /// or corrupt is reported and also starts empty, since every entry in it
    /// can be rebuilt.
    pub fn load(path: &Path) -> Self {
        let bytes = match std::fs::read(path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Self::new(),
            Err(error) => {
                eprintln!(
                    "[macroforge] warning: could not read the scan cache {}: {error}",
                    path.display()
                );
                return Self::new();
            }
        };
        let decoded =
            postcard::take_from_bytes::<PersistedHeader>(&bytes).and_then(|(header, rest)| {
                if header.version == env!("CARGO_PKG_VERSION") {
                    postcard::from_bytes::<ScanCache>(rest).map(Some)
                } else {
                    Ok(None)
                }
            });
        match decoded {
            Ok(Some(cache)) => cache,
            Ok(None) => Self::new(),
            Err(error) => {
                eprintln!(
                    "[macroforge] warning: discarding the unreadable scan cache {}: {error}",
                    path.display()
                );
                Self::new()
            }
        }
    }

    /// Writes the cache to `path`, replacing any earlier one in one rename.
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        use anyhow::Context;

        let mut bytes = postcard::to_stdvec(&PersistedHeader {
            version: env!("CARGO_PKG_VERSION").to_string(),
        })
        .context("failed to encode the scan cache header")?;
        bytes.extend(postcard::to_stdvec(self).context("failed to encode the scan cache")?);

        // Staged under a name of its own, so saves running at once, in this
        // process or another, never rename each other's file away.
        let parent = path.parent().unwrap_or_else(|| Path::new("."));
        std::fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
        let mut staged = tempfile::NamedTempFile::new_in(parent)
            .with_context(|| format!("failed to stage the scan cache in {}", parent.display()))?;
        std::io::Write::write_all(&mut staged, &bytes)
            .with_context(|| format!("failed to write the scan cache for {}", path.display()))?;
        staged
            .persist(path)
            .map(drop)
            .map_err(|error| error.error)
            .with_context(|| format!("failed to move the scan cache into {}", path.display()))
    }

    /// Returns the number of entries currently in the cache.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if the cache has no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The entry for `path` when it was cached at `stamp`. A file without a
    /// trustworthy stamp (see [`file_stamp`]) never matches.
    pub fn get(&self, path: &Path, stamp: Option<FileStamp>) -> Option<Arc<CacheEntry>> {
        let stamp = stamp?;
        let entry = self.entries.get(path)?;
        (entry.mtime_ns == stamp.mtime_ns && entry.size == stamp.size).then(|| Arc::clone(entry))
    }

    /// Insert (or overwrite) the cache entry for `path`.
    pub fn insert(&mut self, path: PathBuf, entry: Arc<CacheEntry>) {
        self.entries.insert(path, entry);
    }

    /// Keeps only the entries for `paths`, so files deleted from the project
    /// do not stay in the cache forever.
    pub fn retain_paths(&mut self, paths: &HashSet<&Path>) {
        self.entries
            .retain(|path, _| paths.contains(path.as_path()));
    }

    /// Remove a single entry by path. Used by HMR when Vite tells us a file
    /// changed on disk: the next scan of that file re-parses it.
    pub fn invalidate(&mut self, path: &Path) -> bool {
        self.entries.remove(path).is_some()
    }

    /// Drop every cached entry. Called when the scanner's config changes
    /// (e.g. `macroforge.config.ts` was touched), since the lowered IR can
    /// depend on config-driven options.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Iterate over the cached entries. Useful for stats or debugging.
    pub fn iter(&self) -> impl Iterator<Item = (&PathBuf, &Arc<CacheEntry>)> {
        self.entries.iter()
    }
}

/// A file's `(mtime_ns, size)` when its metadata was read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileStamp {
    pub mtime_ns: u128,
    pub size: u64,
}

/// How recently written a file may be before its stamp is not trusted: an
/// edit within the filesystem's timestamp resolution could keep both fields.
const RACY_WINDOW: Duration = Duration::from_secs(2);

/// The file's stamp, or `None` when it has none worth trusting: the
/// metadata or mtime is unavailable (the file was deleted between the walk
/// and the read, or the platform has no mtime), or the file was written
/// within [`RACY_WINDOW`].
pub fn file_stamp(path: &Path) -> Option<FileStamp> {
    let meta = std::fs::metadata(path).ok()?;
    let modified = meta.modified().ok()?;
    if SystemTime::now()
        .duration_since(modified)
        .is_ok_and(|age| age < RACY_WINDOW)
    {
        return None;
    }
    Some(FileStamp {
        mtime_ns: modified.duration_since(UNIX_EPOCH).ok()?.as_nanos(),
        size: meta.len(),
    })
}

/// Splice a cached entry's declarative macros into the project-wide
/// declarative registry.
pub fn splice_declarative(
    declarative_registry: &mut ProjectDeclarativeRegistry,
    file_name: &str,
    entry: &CacheEntry,
) {
    declarative_registry.insert_file(file_name, Arc::clone(&entry.declarative_macros));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_entry(mtime_ns: u128, size: u64) -> Arc<CacheEntry> {
        Arc::new(CacheEntry {
            mtime_ns,
            size,
            classes: Vec::new(),
            interfaces: Vec::new(),
            enums: Vec::new(),
            type_aliases: Vec::new(),
            declarative_macros: Arc::default(),
            file_imports: Vec::new(),
            exported_names: HashSet::new(),
            uses_macros: false,
        })
    }

    fn stamp(mtime_ns: u128, size: u64) -> Option<FileStamp> {
        Some(FileStamp { mtime_ns, size })
    }

    /// How many files `scanner` holds in its cache, read back from a save.
    fn cached_entries(scanner: &super::super::ProjectScanner, dir: &Path) -> usize {
        let path = dir.join("scan-cache.bin");
        scanner.save_cache(&path).expect("save the scan cache");
        ScanCache::load(&path).len()
    }

    /// A fresh directory under the system temp dir.
    fn temp_dir(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|elapsed| elapsed.as_nanos())
            .unwrap_or_default();
        let dir = std::env::temp_dir().join(format!("macroforge_{label}_{nanos}"));
        std::fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    /// Writes `contents` to `path` with an mtime old enough to be trusted.
    fn write_settled(path: &Path, contents: &str) {
        std::fs::write(path, contents).expect("write fixture");
        let settled = SystemTime::now() - Duration::from_secs(60);
        std::fs::File::options()
            .write(true)
            .open(path)
            .and_then(|file| file.set_modified(settled))
            .expect("backdate fixture");
    }

    #[test]
    fn get_returns_entry_when_stamp_matches() {
        let mut cache = ScanCache::new();
        let path = PathBuf::from("/tmp/foo.ts");
        cache.insert(path.clone(), empty_entry(100, 50));
        assert!(cache.get(&path, stamp(100, 50)).is_some());
    }

    #[test]
    fn get_returns_none_when_mtime_differs() {
        let mut cache = ScanCache::new();
        let path = PathBuf::from("/tmp/foo.ts");
        cache.insert(path.clone(), empty_entry(100, 50));
        assert!(cache.get(&path, stamp(200, 50)).is_none());
    }

    #[test]
    fn get_returns_none_when_size_differs() {
        let mut cache = ScanCache::new();
        let path = PathBuf::from("/tmp/foo.ts");
        cache.insert(path.clone(), empty_entry(100, 50));
        // Same mtime, different size: catches write-and-rename.
        assert!(cache.get(&path, stamp(100, 99)).is_none());
    }

    #[test]
    fn get_returns_none_without_a_trusted_stamp() {
        let mut cache = ScanCache::new();
        let path = PathBuf::from("/tmp/foo.ts");
        cache.insert(path.clone(), empty_entry(100, 50));
        assert!(cache.get(&path, None).is_none());
    }

    #[test]
    fn get_returns_none_for_unknown_path() {
        let cache = ScanCache::new();
        assert!(
            cache
                .get(Path::new("/tmp/unseen.ts"), stamp(0, 0))
                .is_none()
        );
    }

    #[test]
    fn invalidate_drops_entry() {
        let mut cache = ScanCache::new();
        let path = PathBuf::from("/tmp/foo.ts");
        cache.insert(path.clone(), empty_entry(100, 50));
        assert_eq!(cache.len(), 1);
        assert!(cache.invalidate(&path));
        assert_eq!(cache.len(), 0);
        assert!(!cache.invalidate(&path));
    }

    #[test]
    fn clear_drops_all_entries() {
        let mut cache = ScanCache::new();
        cache.insert(PathBuf::from("/tmp/a.ts"), empty_entry(1, 10));
        cache.insert(PathBuf::from("/tmp/b.ts"), empty_entry(2, 20));
        assert_eq!(cache.len(), 2);
        cache.clear();
        assert!(cache.is_empty());
    }

    #[test]
    fn a_just_written_file_has_no_trusted_stamp() {
        let dir = temp_dir("cache_stamp");
        let path = dir.join("fresh.ts");
        std::fs::write(&path, b"// hi\n").expect("write");
        assert_eq!(file_stamp(&path), None);

        write_settled(&path, "// hi\n");
        let settled = file_stamp(&path).expect("settled stamp");
        assert!(settled.mtime_ns > 0);
        assert_eq!(settled.size, 6);
        std::fs::remove_dir_all(&dir).expect("remove temp dir");
    }

    #[test]
    fn a_saved_cache_loads_back() {
        let dir = temp_dir("cache_persist");
        let path = persisted_path(&dir);
        let mut cache = ScanCache::new();
        cache.insert(PathBuf::from("/tmp/a.ts"), empty_entry(1, 10));
        cache.save(&path).expect("save");

        let loaded = ScanCache::load(&path);
        assert!(loaded.get(Path::new("/tmp/a.ts"), stamp(1, 10)).is_some());
        std::fs::remove_dir_all(&dir).expect("remove temp dir");
    }

    #[test]
    fn a_corrupt_cache_loads_empty() {
        let dir = temp_dir("cache_corrupt");
        let path = persisted_path(&dir);
        std::fs::create_dir_all(path.parent().expect("cache dir")).expect("create");
        std::fs::write(&path, b"not a cache").expect("write");
        assert!(ScanCache::load(&path).is_empty());
        std::fs::remove_dir_all(&dir).expect("remove temp dir");
    }

    #[test]
    fn second_scan_reuses_cache_entries() {
        use super::super::{ProjectScanner, ScanConfig};

        let dir = temp_dir("scanner_cache");
        write_settled(&dir.join("a.ts"), "export interface A { id: string; }\n");
        write_settled(&dir.join("b.ts"), "export class B { name = \"\"; }\n");

        let scanner = ProjectScanner::new(ScanConfig {
            root_dir: dir.clone(),
            ..Default::default()
        })
        .with_cache(ScanCache::new());

        let out1 = scanner.scan().expect("scan 1");
        assert_eq!(out1.files_scanned, 2);
        assert!(out1.changed);
        assert_eq!(cached_entries(&scanner, &dir), 2);

        let out2 = scanner.scan().expect("scan 2");
        assert_eq!(out2.files_scanned, 2);
        assert!(
            !out2.changed,
            "nothing moved, so the scan reports no change"
        );
        assert!(out2.registry.get("A").is_some());
        assert!(out2.registry.get("B").is_some());

        std::fs::remove_dir_all(&dir).expect("remove temp dir");
    }

    #[test]
    fn invalidate_forces_rescan_of_changed_file() {
        use super::super::{ProjectScanner, ScanConfig};

        let dir = temp_dir("scanner_invalidate");
        let a_path = dir.join("a.ts");
        write_settled(&a_path, "export interface A { id: string; }\n");

        let scanner = ProjectScanner::new(ScanConfig {
            root_dir: dir.clone(),
            ..Default::default()
        })
        .with_cache(ScanCache::new());

        scanner.scan().expect("scan 1");
        assert_eq!(cached_entries(&scanner, &dir), 1);

        scanner.invalidate_cache_entry(&a_path);
        assert_eq!(cached_entries(&scanner, &dir), 0);
        write_settled(
            &a_path,
            "export interface A { id: string; }\nexport class A2 {}\n",
        );

        let out = scanner.scan().expect("scan 2");
        assert!(out.registry.get("A2").is_some(), "A2 should be re-scanned");
        assert_eq!(cached_entries(&scanner, &dir), 1);

        std::fs::remove_dir_all(&dir).expect("remove temp dir");
    }

    #[test]
    fn a_deleted_file_leaves_the_cache() {
        use super::super::{ProjectScanner, ScanConfig};

        let dir = temp_dir("scanner_deleted");
        write_settled(&dir.join("a.ts"), "export interface A { id: string; }\n");
        write_settled(&dir.join("b.ts"), "export interface B { id: string; }\n");

        let scanner = ProjectScanner::new(ScanConfig {
            root_dir: dir.clone(),
            ..Default::default()
        })
        .with_cache(ScanCache::new());
        scanner.scan().expect("scan 1");
        assert_eq!(cached_entries(&scanner, &dir), 2);

        std::fs::remove_file(dir.join("b.ts")).expect("delete b.ts");
        let out = scanner.scan().expect("scan 2");
        assert!(out.changed, "a file left the project");
        assert!(out.registry.get("B").is_none());
        assert_eq!(cached_entries(&scanner, &dir), 1);

        std::fs::remove_dir_all(&dir).expect("remove temp dir");
    }
}
