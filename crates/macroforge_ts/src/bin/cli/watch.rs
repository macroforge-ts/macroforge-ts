use anyhow::{Context, Result};
use ignore::WalkBuilder;
use notify_debouncer_full::{new_debouncer, notify::RecursiveMode};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use crate::cache::{
    CONFIG_FILE_NAMES, CacheEntry, collect_watch_files, compute_config_hash, content_hash,
    expand_for_cache, expansion_pool, init_cache, is_watchable_ts_file, normalized_content_hash,
    warm_cache, write_cache_file,
};
use crate::lock::ProjectLock;
use crate::wrappers::refresh_type_registry;

// =========================================================================
// Macro source watching: discover, detect build system, rebuild
// =========================================================================

#[derive(Debug)]
struct MacroSourceInfo {
    name: String,
    package_dir: PathBuf,
    source_dirs: Vec<PathBuf>,
    kind: MacroSourceKind,
}

#[derive(Debug)]
enum MacroSourceKind {
    /// Rust macro crate (has Cargo.toml with macroforge_ts dependency)
    Rust,
    /// JavaScript/TypeScript macro package
    JsTs,
}

#[derive(Debug, Clone, Copy)]
enum BuildSystem {
    Npm,
    Pnpm,
    Yarn,
}

fn detect_build_system(root: &Path) -> BuildSystem {
    if root.join("pnpm-lock.yaml").exists() {
        BuildSystem::Pnpm
    } else if root.join("yarn.lock").exists() {
        BuildSystem::Yarn
    } else {
        BuildSystem::Npm
    }
}

/// The part of the root `package.json` that lists workspace packages.
#[derive(serde::Deserialize)]
struct RootManifest {
    workspaces: Option<Workspaces>,
}

/// npm and yarn take a list of patterns; yarn also takes `{ packages: [...] }`.
#[derive(serde::Deserialize)]
#[serde(untagged)]
enum Workspaces {
    Patterns(Vec<String>),
    Config {
        #[serde(default)]
        packages: Vec<String>,
    },
}

/// The part of a workspace package's `package.json` that marks it as a macro
/// package.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct MemberManifest {
    name: Option<String>,
    #[serde(default)]
    dependencies: HashMap<String, serde::de::IgnoredAny>,
    #[serde(default)]
    dev_dependencies: HashMap<String, serde::de::IgnoredAny>,
    #[serde(default)]
    peer_dependencies: HashMap<String, serde::de::IgnoredAny>,
}

impl MemberManifest {
    fn depends_on(&self, package: &str) -> bool {
        [
            &self.dependencies,
            &self.dev_dependencies,
            &self.peer_dependencies,
        ]
        .iter()
        .any(|dependencies| dependencies.contains_key(package))
    }
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let text =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("failed to parse {}", path.display()))
}

/// The directories the root manifest's workspace patterns name.
fn workspace_dirs(root: &Path) -> Result<Vec<PathBuf>> {
    let manifest_path = root.join("package.json");
    if !manifest_path.is_file() {
        return Ok(Vec::new());
    }
    let patterns = match read_json::<RootManifest>(&manifest_path)?.workspaces {
        Some(Workspaces::Patterns(patterns)) => patterns,
        Some(Workspaces::Config { packages }) => packages,
        None => Vec::new(),
    };

    let mut dirs = Vec::new();
    for pattern in &patterns {
        if !pattern.contains('*') {
            let dir = root.join(pattern);
            if dir.is_dir() {
                dirs.push(dir);
            }
            continue;
        }
        // Simple glob: "packages/*" -> list subdirs of "packages/"
        let base = root.join(pattern.trim_end_matches("/*").trim_end_matches("/**"));
        if !base.is_dir() {
            continue;
        }
        for entry in
            fs::read_dir(&base).with_context(|| format!("failed to list {}", base.display()))?
        {
            let entry = entry.with_context(|| format!("failed to list {}", base.display()))?;
            let file_type = entry
                .file_type()
                .with_context(|| format!("failed to inspect {}", entry.path().display()))?;
            if file_type.is_dir() {
                dirs.push(entry.path());
            }
        }
    }
    Ok(dirs)
}

fn dir_name(dir: &Path) -> String {
    dir.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

/// The directories to watch in a macro package: `src/` when it has one.
fn source_dirs(dir: &Path) -> Vec<PathBuf> {
    let src_dir = dir.join("src");
    if src_dir.is_dir() {
        vec![src_dir]
    } else {
        vec![dir.to_path_buf()]
    }
}

/// Discover macro source packages from workspace configuration.
fn discover_macro_sources(root: &Path) -> Result<Vec<MacroSourceInfo>> {
    let mut sources = Vec::new();
    for dir in workspace_dirs(root)? {
        // A Rust macro crate
        let cargo_toml = dir.join("Cargo.toml");
        if cargo_toml.is_file() {
            let content = fs::read_to_string(&cargo_toml)
                .with_context(|| format!("failed to read {}", cargo_toml.display()))?;
            if content.contains("macroforge_ts") {
                sources.push(MacroSourceInfo {
                    name: dir_name(&dir),
                    source_dirs: source_dirs(&dir),
                    package_dir: dir,
                    kind: MacroSourceKind::Rust,
                });
                continue;
            }
        }

        // A JS/TS macro package
        let manifest_path = dir.join("package.json");
        if !manifest_path.is_file() {
            continue;
        }
        let manifest: MemberManifest = read_json(&manifest_path)?;
        if manifest.depends_on(macroforge_ts::package::PACKAGE) {
            sources.push(MacroSourceInfo {
                name: manifest
                    .name
                    .filter(|name| !name.is_empty())
                    .unwrap_or_else(|| dir_name(&dir)),
                source_dirs: source_dirs(&dir),
                package_dir: dir,
                kind: MacroSourceKind::JsTs,
            });
        }
    }
    Ok(sources)
}

/// Rebuild a macro package. Returns Ok(()) on success.
fn rebuild_macro(info: &MacroSourceInfo, build_system: BuildSystem) -> Result<()> {
    let (cmd, args) = match build_system {
        BuildSystem::Pnpm => ("pnpm", vec!["run", "build"]),
        BuildSystem::Yarn => ("yarn", vec!["build"]),
        BuildSystem::Npm => ("npm", vec!["run", "build"]),
    };

    eprintln!(
        "[macroforge watch] Running `{} {}` in {}",
        cmd,
        args.join(" "),
        info.package_dir.display()
    );

    let output = std::process::Command::new(cmd)
        .args(&args)
        .current_dir(&info.package_dir)
        .output()
        .with_context(|| format!("failed to run `{cmd}` for macro package '{}'", info.name))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        anyhow::bail!(
            "macro rebuild failed for '{}' (exit {}):\n{}\n{}",
            info.name,
            output.status,
            stdout,
            stderr
        );
    }

    Ok(())
}

/// Directories to watch: every directory under `root` that survives
/// .gitignore filtering (ancestor gitignores apply), minus `.git` and our
/// own cache output. Mirrors cargo-watch's registration model.
fn gitignore_watch_dirs(root: &Path, cache_dir: &Path) -> Vec<PathBuf> {
    let cache_dir = cache_dir.to_path_buf();
    WalkBuilder::new(root)
        .hidden(false)
        .git_ignore(true)
        .git_global(false)
        .git_exclude(false)
        .filter_entry(move |entry| {
            entry.file_name() != ".git" && !entry.path().starts_with(&cache_dir)
        })
        .build()
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.into_path())
        .collect()
}

/// Main watch loop: warm cache then watch for changes.
///
/// Unlike the one-shot subcommands, which hold the project lock for their whole
/// run, the watcher takes it only around the bursts below and is unlocked while
/// idle. Holding it for the lifetime of a daemon would block `svelte-package`,
/// `tsc` and every other command until the watcher was killed; scoping it per
/// burst gives the same protection with a delay of one burst instead.
pub fn run_watch(root: &Path, debounce_ms: u64) -> Result<()> {
    let (cache_dir, mut manifest) = init_cache(root, "watch")?;
    let lock = ProjectLock::acquire(root, "watch", false)?;
    warm_cache("watch", root, &cache_dir, &mut manifest)?;
    drop(lock);

    // Discover macro source packages and watch them
    let macro_sources = discover_macro_sources(root)?;
    let build_system = detect_build_system(root);

    if !macro_sources.is_empty() {
        eprintln!(
            "[macroforge watch] Discovered {} macro source package(s):",
            macro_sources.len()
        );
        for src in &macro_sources {
            eprintln!("  - {} ({:?})", src.name, src.kind);
        }
    }

    eprintln!("[macroforge watch] Watching for changes... (Ctrl+C to stop)");

    // --- Watch loop ---
    let (tx, rx) = std::sync::mpsc::channel();
    let mut debouncer = new_debouncer(Duration::from_millis(debounce_ms), None, tx)
        .context("failed to create file watcher")?;

    // Register watches the cargo-watch way: walk the tree honoring
    // .gitignore (ancestor gitignores included), and always exclude our own
    // cache output. Ignored trees (node_modules, .svelte-kit, …) never
    // reach the kernel, so their write bursts can't overflow the inotify
    // event queue and trigger notify's rescan storms. Watches are
    // per-directory (non-recursive): a recursive watch on an included
    // directory would silently re-include its ignored children.
    let mut watched: HashSet<PathBuf> = HashSet::new();
    for dir in gitignore_watch_dirs(root, &cache_dir) {
        debouncer
            .watch(&dir, RecursiveMode::NonRecursive)
            .with_context(|| format!("failed to watch {}", dir.display()))?;
        watched.insert(dir);
    }

    // Watch macro source directories that are outside root
    for src in &macro_sources {
        for dir in &src.source_dirs {
            if dir.exists() && !dir.starts_with(root) {
                debouncer
                    .watch(dir, RecursiveMode::Recursive)
                    .with_context(|| format!("failed to watch macro source: {}", dir.display()))?;
            }
        }
    }

    for result in rx {
        match result {
            Ok(events) => {
                // Kernel event-queue overflow makes the backend synthesize
                // events for every tracked file (rescan flag): none of
                // them describe real changes. Resync against content
                // hashes once (sub-second when nothing changed) instead of
                // processing thousands of phantom "changes".
                if events.iter().any(|event| event.need_rescan()) {
                    eprintln!(
                        "[macroforge watch] Watch backend requested a rescan \
                         (event queue overflow): resyncing against content hashes"
                    );
                    let lock = ProjectLock::acquire(root, "watch", false)?;
                    warm_cache("watch", root, &cache_dir, &mut manifest)?;
                    drop(lock);
                    continue;
                }

                // Per-directory registration means new directories need
                // their own watches; deletions free theirs (and must be
                // forgotten so a recreated dir gets a fresh watch).
                let mut dir_created = false;
                for event in &events {
                    if event.kind.is_remove() {
                        for event_path in &event.paths {
                            watched.remove(event_path);
                        }
                    } else if event.kind.is_create() && event.paths.iter().any(|p| p.is_dir()) {
                        dir_created = true;
                    }
                }
                if dir_created {
                    for dir in gitignore_watch_dirs(root, &cache_dir) {
                        if watched.insert(dir.clone())
                            && let Err(e) = debouncer.watch(&dir, RecursiveMode::NonRecursive)
                        {
                            eprintln!(
                                "[macroforge watch] failed to watch new dir {}: {e}",
                                dir.display()
                            );
                        }
                    }
                }

                let mut config_changed = false;
                let mut config_event_path: Option<PathBuf> = None;
                let mut macro_source_changed: Option<&MacroSourceInfo> = None;
                let mut changed_files: Vec<PathBuf> = Vec::new();

                for event in &events {
                    // Reads are not changes. Reacting to access events feeds
                    // back: the config hash guard below reads the config to
                    // compare it, which emits the next access event: an
                    // endless read→event→read loop on an untouched file.
                    if event.kind.is_access() || event.kind.is_other() {
                        continue;
                    }
                    for event_path in &event.paths {
                        // Check for config file changes. Exact names only:
                        // editor/bundler siblings (macroforge.config.ts~,
                        // .swp, .timestamp-*.mjs) are not the config.
                        if let Some(name) = event_path.file_name() {
                            let name_str = name.to_string_lossy();
                            if CONFIG_FILE_NAMES.iter().any(|c| name_str == *c) {
                                config_changed = true;
                                config_event_path = Some(event_path.clone());
                                continue;
                            }
                        }

                        // Check if path belongs to a macro source package
                        let is_macro_src = macro_sources
                            .iter()
                            .find(|ms| ms.source_dirs.iter().any(|d| event_path.starts_with(d)));
                        if let Some(ms) = is_macro_src {
                            macro_source_changed = Some(ms);
                            continue;
                        }

                        if is_watchable_ts_file(event_path, root) {
                            changed_files.push(event_path.clone());
                        }
                    }
                }

                changed_files.sort();
                changed_files.dedup();

                // An event on the config file does not mean its content
                // changed (attrib updates, relinks, and watcher rescans all
                // land here). Re-expanding every file takes minutes and
                // gigabytes, so only do it for a real content change.
                if config_changed {
                    let new_config_hash = compute_config_hash(root);
                    if new_config_hash == manifest.config_hash {
                        eprintln!(
                            "[macroforge watch] Config file event with unchanged content: \
                             ignoring ({})",
                            config_event_path
                                .as_deref()
                                .map(|p| p.display().to_string())
                                .unwrap_or_else(|| "unknown path".to_string())
                        );
                        config_changed = false;
                    }
                }

                // Macro source change: rebuild then full re-expand
                if let Some(macro_info) = macro_source_changed {
                    eprintln!(
                        "[macroforge watch] Macro source changed in '{}', rebuilding...",
                        macro_info.name
                    );

                    // The rebuild shells out to a package manager and can take
                    // minutes; it touches the macro package, not this
                    // project's `.macroforge/`, so it runs unlocked. Only the
                    // re-expansion that consumes its output needs the lock.
                    match rebuild_macro(macro_info, build_system) {
                        Ok(()) => {
                            eprintln!(
                                "[macroforge watch] Macro '{}' rebuilt, re-expanding all files...",
                                macro_info.name
                            );
                            let lock = ProjectLock::acquire(root, "watch", false)?;
                            manifest.entries.clear();
                            macroforge_ts::host::clear_config_cache();
                            warm_cache("watch", root, &cache_dir, &mut manifest)?;
                            drop(lock);
                        }
                        Err(e) => {
                            eprintln!("[macroforge watch] Macro rebuild failed: {}", e);
                        }
                    }
                } else if config_changed {
                    use rayon::prelude::*;

                    let lock = ProjectLock::acquire(root, "watch", false)?;
                    if let Err(err) = refresh_type_registry(root) {
                        eprintln!("[macroforge watch] {err:#}");
                    }

                    let new_config_hash = compute_config_hash(root);
                    manifest.config_hash = new_config_hash;
                    manifest.entries.clear();
                    eprintln!("[macroforge watch] Config changed, re-expanding all files...");

                    let all_files = collect_watch_files(root);

                    // Read and hash all files (sequential)
                    let files_with_source: Vec<_> = all_files
                        .iter()
                        .filter_map(|file_path| {
                            let rel_path = file_path
                                .strip_prefix(root)
                                .unwrap_or(file_path)
                                .to_string_lossy()
                                .to_string();
                            let source = match fs::read_to_string(file_path) {
                                Ok(source) => source,
                                Err(error) => {
                                    eprintln!("  [!] {rel_path}: could not read it: {error}");
                                    return None;
                                }
                            };
                            let source_hash = content_hash(source.as_bytes());
                            let norm_hash = normalized_content_hash(&source);
                            Some((file_path.clone(), rel_path, source, source_hash, norm_hash))
                        })
                        .collect();

                    // Expand in parallel
                    let results: Vec<_> = expansion_pool()?.install(|| {
                        files_with_source
                            .par_iter()
                            .map(|(file_path, rel_path, source, source_hash, norm_hash)| {
                                let result = expand_for_cache(root, file_path, source);
                                (
                                    rel_path.clone(),
                                    source_hash.clone(),
                                    norm_hash.clone(),
                                    result,
                                )
                            })
                            .collect()
                    });

                    // Apply results (sequential)
                    let mut count = 0u32;
                    for (rel_path, source_hash, norm_hash, result) in results {
                        match result {
                            Ok(Some(expansion)) => {
                                for error in &expansion.errors {
                                    eprintln!("  [!] {rel_path}: {error}");
                                }
                                // An entry is recorded only for an expansion
                                // that reached the cache.
                                if let Err(error) =
                                    write_cache_file(&cache_dir, &rel_path, &expansion.code)
                                {
                                    eprintln!("  [!] {rel_path}: {error:#}");
                                    continue;
                                }
                                manifest.entries.insert(
                                    rel_path.clone(),
                                    CacheEntry {
                                        source_hash,
                                        has_macros: true,
                                        normalized_hash: norm_hash,
                                    },
                                );
                                count += 1;
                                eprintln!("  [~] {}", rel_path);
                            }
                            Ok(None) => {
                                manifest.entries.insert(
                                    rel_path,
                                    CacheEntry {
                                        source_hash,
                                        has_macros: false,
                                        normalized_hash: norm_hash,
                                    },
                                );
                            }
                            Err(e) => eprintln!("  [!] {}: {}", rel_path, e),
                        }
                    }
                    manifest.save(&cache_dir)?;
                    drop(lock);
                    eprintln!("[macroforge watch] Re-expanded {} files", count);
                } else if !changed_files.is_empty() {
                    let lock = ProjectLock::acquire(root, "watch", false)?;
                    // The edits can change types other files resolve against.
                    if let Err(err) = refresh_type_registry(root) {
                        eprintln!("[macroforge watch] {err:#}");
                    }

                    for file_path in &changed_files {
                        let rel_path = file_path
                            .strip_prefix(root)
                            .unwrap_or(file_path)
                            .to_string_lossy()
                            .to_string();

                        let source = match fs::read_to_string(file_path) {
                            Ok(s) => s,
                            Err(_) => {
                                // File was deleted
                                manifest.entries.remove(&rel_path);
                                // Remove cached file too
                                let cache_path = cache_dir.join(format!("{rel_path}.cache"));
                                match fs::remove_file(&cache_path) {
                                    Ok(()) => {}
                                    Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
                                    Err(err) => eprintln!(
                                        "[macroforge watch] failed to remove {}: {err}",
                                        cache_path.display()
                                    ),
                                }
                                eprintln!("  [-] {} (removed)", rel_path);
                                continue;
                            }
                        };

                        let source_hash = content_hash(source.as_bytes());

                        // Fast path: file is byte-identical to cached version
                        if let Some(entry) = manifest.entries.get(&rel_path)
                            && entry.source_hash == source_hash
                        {
                            continue;
                        }

                        // Check if only whitespace changed
                        let norm_hash = normalized_content_hash(&source);
                        if let Some(entry) = manifest.entries.get(&rel_path)
                            && !entry.normalized_hash.is_empty()
                            && entry.normalized_hash == norm_hash
                        {
                            // Update raw hash but skip re-expansion
                            manifest.entries.insert(
                                rel_path.clone(),
                                CacheEntry {
                                    source_hash,
                                    has_macros: entry.has_macros,
                                    normalized_hash: norm_hash,
                                },
                            );
                            eprintln!("  [·] {} (whitespace only)", rel_path);
                            continue;
                        }

                        let file_start = std::time::Instant::now();

                        match expand_for_cache(root, file_path, &source) {
                            Ok(Some(expansion)) => {
                                // Reported, not fatal: a watch loop that exits
                                // on the first bad edit is worse than one that
                                // says what is wrong and waits for the fix.
                                for error in &expansion.errors {
                                    eprintln!("  [!] {rel_path}: {error}");
                                }
                                // An entry is recorded only for an expansion
                                // that reached the cache.
                                if let Err(error) =
                                    write_cache_file(&cache_dir, &rel_path, &expansion.code)
                                {
                                    eprintln!("  [!] {rel_path}: {error:#}");
                                    continue;
                                }
                                manifest.entries.insert(
                                    rel_path.clone(),
                                    CacheEntry {
                                        source_hash,
                                        has_macros: true,
                                        normalized_hash: norm_hash,
                                    },
                                );
                                let elapsed = file_start.elapsed();
                                eprintln!("  [~] {} ({}ms)", rel_path, elapsed.as_millis());
                            }
                            Ok(None) => {
                                manifest.entries.insert(
                                    rel_path.clone(),
                                    CacheEntry {
                                        source_hash,
                                        has_macros: false,
                                        normalized_hash: norm_hash,
                                    },
                                );
                                eprintln!("  [.] {} (no macros)", rel_path);
                            }
                            Err(e) => {
                                eprintln!("  [!] {}: {}", rel_path, e);
                            }
                        }
                    }
                    manifest.save(&cache_dir)?;
                    drop(lock);
                }
            }
            Err(errors) => {
                for e in errors {
                    eprintln!("[macroforge watch] Watch error: {}", e);
                }
            }
        }
    }

    Ok(())
}
