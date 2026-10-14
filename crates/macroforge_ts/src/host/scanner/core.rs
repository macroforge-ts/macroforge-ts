use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
use ignore::WalkBuilder;

use crate::host::file_access;
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
use crate::host::file_access::PathKind;

use crate::host::declarative::ProjectDeclarativeRegistry;
use crate::ts_syn::abi::ir::type_registry::{
    ReExport, TypeDefinitionIR, TypeRegistry, TypeRegistryEntry,
};

use crate::ts_syn::{
    collect_export_aliases, collect_exported_names, collect_file_imports, collect_re_exports,
    lower_classes, lower_enums, lower_interfaces, lower_type_aliases,
};

use super::cache::{CacheEntry, FileStamp, ScanCache, file_stamp, splice_declarative};
use super::config::ScanConfig;
use super::re_exports::link_re_exports;

/// Result of scanning a project.
pub struct ScanOutput {
    /// The populated type registry.
    pub registry: TypeRegistry,
    /// Project-wide declarative macro registry populated during the same
    /// walk as [`ScanOutput::registry`]. Empty if no files contained
    /// declarative macros.
    pub declarative_registry: ProjectDeclarativeRegistry,
    /// Number of files scanned.
    pub files_scanned: u32,
    /// Number of scanned files containing anything the engine expands. A
    /// project where this is zero needs no registries.
    pub macro_files: u32,
    /// Warnings from files that failed to parse.
    pub warnings: Vec<String>,
    /// Whether anything may differ from the previous scan by this scanner:
    /// a file was lowered afresh or left the project. Always true without a
    /// cache.
    pub changed: bool,
}

/// Scans a TypeScript project and builds a [`TypeRegistry`].
///
/// The optional [`ScanCache`] turns repeated scans of unchanged files
/// into O(1) lookups. HMR and LSP hosts keep a long-lived
/// `ProjectScanner` and avoid re-parsing on every edit. The single-
/// shot CLI path passes no cache and runs the original walker
/// unchanged.
pub struct ProjectScanner {
    config: ScanConfig,
    /// Optional per-file scan cache. When present, [`Self::scan`]
    /// consults it before parsing and writes fresh entries back on
    /// cache misses. `RefCell` because `scan` takes `&self` and the
    /// callers (the wasm bindings) want interior mutability to
    /// keep the existing call signature.
    cache: Option<RefCell<ScanCache>>,
}

impl ProjectScanner {
    /// Create a new scanner with the given configuration.
    pub fn new(config: ScanConfig) -> Self {
        Self {
            config,
            cache: None,
        }
    }

    /// Create a scanner with defaults, rooted at the given directory.
    pub fn with_root(root_dir: PathBuf) -> Self {
        Self {
            config: ScanConfig {
                root_dir,
                ..Default::default()
            },
            cache: None,
        }
    }

    /// Install a scan cache on this scanner. Subsequent `scan()`
    /// calls will consult the cache for each file's `(mtime, size)`
    /// tuple and skip parsing on hits.
    pub fn with_cache(mut self, cache: ScanCache) -> Self {
        self.cache = Some(RefCell::new(cache));
        self
    }

    /// Remove a single cache entry. Used by the HMR bridge to tell
    /// the scanner that a file just changed on disk; the next
    /// [`Self::scan`] will re-parse it. No-op when the cache isn't
    /// installed.
    pub fn invalidate_cache_entry(&self, path: &Path) -> bool {
        match self.cache.as_ref() {
            Some(c) => c.borrow_mut().invalidate(path),
            None => false,
        }
    }

    /// Drop the entire cache. Used when the scanner's config (or
    /// anything that feeds into lowering) changes and cached IR
    /// might be stale.
    pub fn clear_cache(&self) {
        if let Some(c) = self.cache.as_ref() {
            c.borrow_mut().clear();
        }
    }

    /// Persists the cache at `path`, when one is installed.
    pub fn save_cache(&self, path: &Path) -> anyhow::Result<()> {
        match self.cache.as_ref() {
            Some(cache) => cache.borrow().save(path),
            None => Ok(()),
        }
    }

    /// Perform the full project scan and return a populated [`TypeRegistry`].
    ///
    /// Files are registered in path order, whatever order they were lowered
    /// in, because `TypeRegistry::insert` keeps the first declaration of a
    /// name.
    pub fn scan(&self) -> anyhow::Result<ScanOutput> {
        let (paths, mut warnings) = self.source_files();

        // Cached entries first, then every miss lowered in parallel.
        let mut misses = Vec::new();
        let mut entries: Vec<Option<anyhow::Result<Arc<CacheEntry>>>> = paths
            .iter()
            .enumerate()
            .map(|(index, path)| {
                let stamp = file_stamp(path);
                let cached = self
                    .cache
                    .as_ref()
                    .and_then(|cache| cache.borrow().get(path, stamp));
                if cached.is_none() {
                    misses.push((index, stamp));
                }
                cached.map(Ok)
            })
            .collect();
        let changed = self.cache.is_none() || !misses.is_empty();

        for (index, stamp, lowered) in lower_files(&paths, misses)? {
            let lowered = lowered.map(Arc::new);
            if let (Some(cache), Some(_), Ok(entry)) = (self.cache.as_ref(), stamp, &lowered) {
                cache
                    .borrow_mut()
                    .insert(paths[index].clone(), Arc::clone(entry));
            }
            entries[index] = Some(lowered);
        }

        let mut removed = false;
        if let Some(cache) = self.cache.as_ref() {
            let mut cache = cache.borrow_mut();
            let before = cache.len();
            cache.retain_paths(&paths.iter().map(PathBuf::as_path).collect());
            removed = cache.len() != before;
        }

        // A barrel declares nothing: what it exports again is recorded on
        // the declarations it leads to.
        let scanned: Vec<(String, &CacheEntry)> = paths
            .iter()
            .zip(&entries)
            .filter_map(|(path, entry)| match entry {
                Some(Ok(entry)) => Some((path.to_string_lossy().to_string(), entry.as_ref())),
                Some(Err(_)) | None => None,
            })
            .collect();
        let re_exported = link_re_exports(&scanned);

        let mut registry = TypeRegistry::new();
        let mut declarative_registry = ProjectDeclarativeRegistry::new();
        let root_str = self.config.root_dir.to_string_lossy().to_string();
        let mut macro_files: u32 = 0;
        for (path, entry) in paths.iter().zip(&entries) {
            match entry {
                Some(Ok(entry)) => {
                    let file_name = path.to_string_lossy().to_string();
                    splice_declarative(&mut declarative_registry, &file_name, entry);
                    macro_files += u32::from(entry.uses_macros);
                    self.register_items(&mut registry, &root_str, &file_name, entry, &re_exported);
                }
                Some(Err(e)) => warnings.push(format!("Failed to scan {:?}: {}", path, e)),
                None => warnings.push(format!("{} was not scanned", path.display())),
            }
        }

        Ok(ScanOutput {
            registry,
            declarative_registry,
            files_scanned: paths.len() as u32,
            macro_files,
            warnings,
            changed: changed || removed,
        })
    }

    /// The source files under the root, in path order, and warnings for
    /// entries the walk could not read or files past the limit.
    #[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
    fn source_files(&self) -> (Vec<PathBuf>, Vec<String>) {
        let root = self.config.root_dir.clone();
        let skip_dirs = self.config.skip_dirs.clone();
        // Sorted, so registration order does not depend on the filesystem.
        // Skipped directories are pruned below the root; the root's own path
        // may contain any name.
        let walker = WalkBuilder::new(&self.config.root_dir)
            .hidden(true) // Skip hidden files by default
            .git_ignore(true) // Respect .gitignore
            .git_global(false)
            .git_exclude(false)
            .sort_by_file_path(|left, right| left.cmp(right))
            .filter_entry(move |entry| {
                entry.path() == root
                    || !entry.file_type().is_some_and(|kind| kind.is_dir())
                    || !entry
                        .file_name()
                        .to_str()
                        .is_some_and(|name| skip_dirs.contains(name))
            })
            .build();

        let mut paths = Vec::new();
        let mut warnings = Vec::new();
        for entry in walker {
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => {
                    warnings.push(format!("Skipped an unreadable entry: {error}"));
                    continue;
                }
            };
            if entry.file_type().is_some_and(|kind| kind.is_dir()) {
                continue;
            }
            let path = entry.path();
            let has_matching_ext = path
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|ext| self.config.extensions.contains(&format!(".{ext}")));
            if !has_matching_ext {
                continue;
            }
            if paths.len() == self.config.max_files {
                warnings.push(format!(
                    "Reached max file limit ({}). Some types may be missing.",
                    self.config.max_files
                ));
                break;
            }
            paths.push(path.to_path_buf());
        }
        (paths, warnings)
    }

    /// The source files under the root, walked through Node's `fs` as the
    /// wasm build has no filesystem of its own: in path order, hidden entries
    /// and the configured directories left out, and, inside a git repository,
    /// what each directory's `.gitignore` names, as the native walk does.
    #[cfg(all(target_arch = "wasm32", feature = "wasm"))]
    fn source_files(&self) -> (Vec<PathBuf>, Vec<String>) {
        let mut walk = BridgedWalk {
            scanner: self,
            in_repository: self.config.root_dir.ancestors().any(|dir| {
                file_access::kind(&dir.join(".git")).is_ok_and(|kind| kind != PathKind::Absent)
            }),
            ignores: Vec::new(),
            paths: Vec::new(),
            warnings: Vec::new(),
        };
        walk.directory(&self.config.root_dir);
        (walk.paths, walk.warnings)
    }

    fn register_items(
        &self,
        registry: &mut TypeRegistry,
        project_root: &str,
        file_name: &str,
        entry: &CacheEntry,
        re_exported: &HashMap<(String, String), Vec<ReExport>>,
    ) {
        let definitions = entry
            .classes
            .iter()
            .map(|class| (&class.name, TypeDefinitionIR::Class(class.clone())))
            .chain(
                entry
                    .interfaces
                    .iter()
                    .map(|iface| (&iface.name, TypeDefinitionIR::Interface(iface.clone()))),
            )
            .chain(
                entry
                    .enums
                    .iter()
                    .map(|enum_ir| (&enum_ir.name, TypeDefinitionIR::Enum(enum_ir.clone()))),
            )
            .chain(
                entry
                    .type_aliases
                    .iter()
                    .map(|alias| (&alias.name, TypeDefinitionIR::TypeAlias(alias.clone()))),
            );
        for (name, definition) in definitions {
            let is_exported = entry.exported_names.contains(name);
            if self.config.exported_only && !is_exported {
                continue;
            }
            registry.insert(
                TypeRegistryEntry {
                    name: name.clone(),
                    file_path: file_name.to_string(),
                    is_exported,
                    exported_as: entry
                        .export_aliases
                        .iter()
                        .filter(|(declared, _)| declared == name)
                        .map(|(_, exported)| exported.clone())
                        .collect(),
                    re_exported: re_exported
                        .get(&(file_name.to_string(), name.clone()))
                        .cloned()
                        .unwrap_or_default(),
                    definition,
                    file_imports: entry.file_imports.clone(),
                },
                project_root,
            );
        }
    }
}

/// A lowered file, by its index among the scanned paths, with the stamp it
/// was read at.
type Lowered = (usize, Option<FileStamp>, anyhow::Result<CacheEntry>);

/// Lowers each of `misses`, the indices of `paths` the cache did not answer,
/// in parallel on the engine's workers where there are threads.
fn lower_files(
    paths: &[PathBuf],
    misses: Vec<(usize, Option<FileStamp>)>,
) -> anyhow::Result<Vec<Lowered>> {
    let lower = |(index, stamp): (usize, Option<FileStamp>)| {
        (index, stamp, lower_file(&paths[index], stamp))
    };
    #[cfg(not(target_arch = "wasm32"))]
    {
        use rayon::prelude::*;
        Ok(crate::workers::worker_pool()
            .map_err(anyhow::Error::msg)?
            .install(|| misses.into_par_iter().map(lower).collect()))
    }
    #[cfg(target_arch = "wasm32")]
    {
        Ok(misses.into_iter().map(lower).collect())
    }
}

/// Parses and lowers one file. `stamp` is its metadata as read before the
/// file was, so an edit in between only makes the entry look stale.
fn lower_file(path: &Path, stamp: Option<FileStamp>) -> anyhow::Result<CacheEntry> {
    use oxc::allocator::Allocator;
    use oxc::parser::Parser;
    use oxc::span::SourceType;

    let file_name = path.to_string_lossy();
    let source = file_access::read_to_string(path)?;
    let uses_macros = crate::has_macro_annotations(&source, &file_name);

    let allocator = Allocator::default();
    let source_type = SourceType::ts().with_jsx(file_name.ends_with(".tsx"));
    let ret = Parser::new(&allocator, &source, source_type).parse();

    if !ret.diagnostics.is_empty() {
        let messages: Vec<String> = ret.diagnostics.iter().map(ToString::to_string).collect();
        return Err(anyhow::anyhow!("parse errors: {}", messages.join("; ")));
    }

    // Declarative macro discovery shares the same parse. The discovery
    // helper bails out on files that don't import `macroRules` from
    // `"@macroforge/core/rules"`, so on other files it costs one
    // import-statement scan.
    let declarative_macros = Arc::new(crate::host::declarative::project_registry::by_name(
        crate::host::declarative::discover(&ret.program, &source)
            .map_err(|e| anyhow::anyhow!("Declarative macro discovery failed: {}", e))?
            .into_iter()
            .map(|dm| dm.def)
            .collect(),
    ));

    let lower_error = |kind: &str, error: crate::ts_syn::TsSynError| {
        anyhow::anyhow!("failed to lower {kind}: {error}")
    };
    Ok(CacheEntry {
        mtime_ns: stamp.map_or(0, |stamp| stamp.mtime_ns),
        size: stamp.map_or(0, |stamp| stamp.size),
        classes: lower_classes(&ret.program, &source, None)
            .map_err(|error| lower_error("classes", error))?,
        interfaces: lower_interfaces(&ret.program, &source, None)
            .map_err(|error| lower_error("interfaces", error))?,
        enums: lower_enums(&ret.program, &source, None)
            .map_err(|error| lower_error("enums", error))?,
        type_aliases: lower_type_aliases(&ret.program, &source)
            .map_err(|error| lower_error("type aliases", error))?,
        declarative_macros,
        file_imports: collect_file_imports(&ret.program),
        exported_names: collect_exported_names(&ret.program),
        export_aliases: collect_export_aliases(&ret.program),
        re_exports: collect_re_exports(&ret.program),
        uses_macros,
    })
}

/// A walk of the project through Node's `fs`, for the wasm build.
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
struct BridgedWalk<'a> {
    scanner: &'a ProjectScanner,
    /// Whether the root is in a git repository, outside which the native
    /// walk reads no `.gitignore`.
    in_repository: bool,
    /// The `.gitignore` of each directory from the root down to the current.
    ignores: Vec<ignore::gitignore::Gitignore>,
    paths: Vec<PathBuf>,
    warnings: Vec<String>,
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
impl BridgedWalk<'_> {
    /// Walks `dir`, returning `false` once the file limit is reached.
    fn directory(&mut self, dir: &Path) -> bool {
        let pushed = self.in_repository && self.push_gitignore(dir);
        let keep_going = self.entries(dir);
        if pushed {
            self.ignores.pop();
        }
        keep_going
    }

    fn entries(&mut self, dir: &Path) -> bool {
        let mut entries = match file_access::list_directory(dir) {
            Ok(entries) => entries,
            Err(error) => {
                self.warnings.push(format!(
                    "Skipped an unreadable directory {}: {error}",
                    dir.display()
                ));
                return true;
            }
        };
        entries.sort_by(|left, right| left.0.cmp(&right.0));
        let config = &self.scanner.config;
        for (name, kind) in entries {
            if name.starts_with('.') {
                continue;
            }
            let path = dir.join(&name);
            let is_dir = kind == PathKind::Directory;
            if self.ignored(&path, is_dir) {
                continue;
            }
            match kind {
                PathKind::Directory => {
                    if !config.skip_dirs.contains(&name) && !self.directory(&path) {
                        return false;
                    }
                }
                PathKind::File => {
                    let matches = path
                        .extension()
                        .and_then(|extension| extension.to_str())
                        .is_some_and(|extension| {
                            config.extensions.contains(&format!(".{extension}"))
                        });
                    if !matches {
                        continue;
                    }
                    if self.paths.len() == config.max_files {
                        self.warnings.push(format!(
                            "Reached max file limit ({}). Some types may be missing.",
                            config.max_files
                        ));
                        return false;
                    }
                    self.paths.push(path);
                }
                PathKind::Absent => {}
            }
        }
        true
    }

    /// Reads `dir`'s `.gitignore` onto the stack, returning whether it did.
    fn push_gitignore(&mut self, dir: &Path) -> bool {
        let file = dir.join(".gitignore");
        if !file_access::kind(&file).is_ok_and(|kind| kind == PathKind::File) {
            return false;
        }
        let text = match file_access::read_to_string(&file) {
            Ok(text) => text,
            Err(error) => {
                self.warnings
                    .push(format!("Could not read {}: {error}", file.display()));
                return false;
            }
        };
        let mut builder = ignore::gitignore::GitignoreBuilder::new(dir);
        for line in text.lines() {
            if let Err(error) = builder.add_line(None, line) {
                self.warnings.push(format!("{}: {error}", file.display()));
            }
        }
        match builder.build() {
            Ok(gitignore) => {
                self.ignores.push(gitignore);
                true
            }
            Err(error) => {
                self.warnings.push(format!("{}: {error}", file.display()));
                false
            }
        }
    }

    /// Whether the nearest `.gitignore` that speaks of `path` ignores it.
    fn ignored(&self, path: &Path, is_dir: bool) -> bool {
        for gitignore in self.ignores.iter().rev() {
            match gitignore.matched(path, is_dir) {
                ignore::Match::Ignore(_) => return true,
                ignore::Match::Whitelist(_) => return false,
                ignore::Match::None => {}
            }
        }
        false
    }
}
