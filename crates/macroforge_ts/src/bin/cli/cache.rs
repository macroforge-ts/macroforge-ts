use anyhow::{Context, Result, anyhow};
use ignore::WalkBuilder;
use macroforge_ts::host::MacroExpander;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::wrappers::{
    DECLARATIVE_REGISTRY_CACHE_PATH, TYPE_REGISTRY_CACHE_PATH, ensure_type_registry_cache,
};

/// Cache manifest stored at `.macroforge/cache/manifest.json`.
///
/// Tracks expanded file hashes so the Vite plugin (and subsequent watch runs)
/// can skip re-expansion for unchanged files.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CacheManifest {
    /// Macroforge crate version — full invalidation on upgrade.
    pub(crate) version: String,
    /// SHA-256 of the macroforge config file content (or `"none"`).
    pub(crate) config_hash: String,
    /// Hash of external macro NAPI binaries (mtime+size). Invalidates the cache
    /// when a local macro package (e.g. `@dealdraft/macros`) is rebuilt.
    #[serde(default)]
    pub(crate) external_macro_hash: String,
    /// Per-file entries keyed by path relative to project root.
    pub(crate) entries: HashMap<String, CacheEntry>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CacheEntry {
    /// SHA-256 of the source file content.
    pub(crate) source_hash: String,
    /// Whether the file contained macros.
    pub(crate) has_macros: bool,
    /// SHA-256 of the whitespace-normalized source content.
    /// Used to detect whitespace-only changes in watch mode.
    #[serde(default)]
    pub(crate) normalized_hash: String,
}

/// Computes SHA-256 of a byte slice, returned as lowercase hex.
pub(crate) fn content_hash(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let result = hasher.finalize();
    result.iter().fold(String::with_capacity(64), |mut s, b| {
        use std::fmt::Write;
        let _ = write!(s, "{b:02x}");
        s
    })
}

/// Computes SHA-256 of whitespace-normalized content.
///
/// Normalization: trim trailing whitespace per line, collapse runs of blank
/// lines into a single newline, trim leading/trailing blank lines. Leading
/// indentation is preserved (meaningful in TS template literals).
pub(crate) fn normalized_content_hash(content: &str) -> String {
    let mut normalized = String::with_capacity(content.len());
    let mut prev_blank = false;

    for line in content.lines() {
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            if !prev_blank && !normalized.is_empty() {
                normalized.push('\n');
            }
            prev_blank = true;
        } else {
            if !normalized.is_empty() {
                normalized.push('\n');
            }
            normalized.push_str(trimmed);
            prev_blank = false;
        }
    }

    // Trim trailing blank lines so "foo\n" and "foo\n\n\n" hash the same
    let normalized = normalized.trim_end_matches('\n');
    content_hash(normalized.as_bytes())
}

/// Config file names searched in order of precedence.
pub(crate) const CONFIG_FILE_NAMES: &[&str] = &[
    "macroforge.config.ts",
    "macroforge.config.mts",
    "macroforge.config.js",
    "macroforge.config.mjs",
    "macroforge.config.cjs",
];

/// Computes a hash of the macroforge config file for cache invalidation.
pub(crate) fn compute_config_hash(root: &Path) -> String {
    for name in CONFIG_FILE_NAMES {
        let path = root.join(name);
        if let Ok(content) = fs::read(&path) {
            return content_hash(&content);
        }
    }
    "none".to_string()
}

/// Computes a hash over external macro package binaries so the cache
/// invalidates when a local macro package is rebuilt.
///
/// Scans `node_modules` for packages whose `index.js` exports
/// `__macroforgeRun` (the marker for macro packages), then hashes
/// the metadata (mtime + size) of all binary artifacts (`.node`, `.wasm`)
/// and the JS entry itself.
pub(crate) fn compute_external_macro_hash(root: &Path) -> String {
    let node_modules = root.join("node_modules");
    if !node_modules.exists() {
        return "none".to_string();
    }

    // Collect path:size:mtime parts, sort for deterministic ordering
    // (readdir order varies across platforms and runtimes), then hash.
    let mut parts: Vec<String> = Vec::new();

    let mut check_package = |pkg_dir: &Path| {
        let index_js = pkg_dir.join("index.js");
        let is_macro_pkg = fs::read_to_string(&index_js)
            .map(|content| content.contains("__macroforgeRun"))
            .unwrap_or(false);
        if !is_macro_pkg {
            return;
        }

        let extensions = ["node", "wasm"];
        if let Ok(entries) = fs::read_dir(pkg_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_file() {
                    continue;
                }
                let is_binary = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .is_some_and(|ext| extensions.contains(&ext));
                let is_entry = path.file_name().is_some_and(|n| n == "index.js");
                if !is_binary && !is_entry {
                    continue;
                }
                if let Ok(meta) = fs::metadata(&path) {
                    use std::fmt::Write;
                    let mut buf = String::new();
                    let _ = write!(
                        buf,
                        "{}:{}:{}",
                        path.display(),
                        meta.len(),
                        meta.modified()
                            .map(|t| t
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs())
                            .unwrap_or(0)
                    );
                    parts.push(buf);
                }
            }
        }
    };

    if let Ok(entries) = fs::read_dir(&node_modules) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.starts_with('@') {
                    if let Ok(scoped) = fs::read_dir(&path) {
                        for sub in scoped.flatten() {
                            if sub.path().is_dir() {
                                check_package(&sub.path());
                            }
                        }
                    }
                } else if !name_str.starts_with('.') {
                    check_package(&path);
                }
            }
        }
    }

    if parts.is_empty() {
        return "none".to_string();
    }

    parts.sort();
    let mut hasher = Sha256::new();
    for part in &parts {
        hasher.update(part.as_bytes());
    }

    let result = hasher.finalize();
    result.iter().fold(String::with_capacity(64), |mut s, b| {
        use std::fmt::Write;
        let _ = write!(s, "{b:02x}");
        s
    })
}

impl CacheManifest {
    pub(crate) fn new(version: String, config_hash: String, external_macro_hash: String) -> Self {
        Self {
            version,
            config_hash,
            external_macro_hash,
            entries: HashMap::new(),
        }
    }

    pub(crate) fn load(cache_dir: &Path) -> Option<Self> {
        let manifest_path = cache_dir.join("manifest.json");
        let content = fs::read_to_string(manifest_path).ok()?;
        serde_json::from_str(&content).ok()
    }

    /// Atomically saves the manifest via write-to-tmp + rename.
    pub(crate) fn save(&self, cache_dir: &Path) -> Result<()> {
        fs::create_dir_all(cache_dir)?;
        let manifest_path = cache_dir.join("manifest.json");
        let json = serde_json::to_string_pretty(self)?;

        // Atomic write: temp file in same directory, then rename
        let tmp_path = cache_dir.join(".manifest.json.tmp");
        fs::write(&tmp_path, &json)?;
        fs::rename(&tmp_path, &manifest_path)?;
        Ok(())
    }
}

/// Writes expanded code to `<cache_dir>/<rel_path>.cache`.
pub(crate) fn write_cache_file(
    cache_dir: &Path,
    rel_path: &str,
    expanded_code: &str,
) -> Result<()> {
    let cache_path = cache_dir.join(format!("{rel_path}.cache"));
    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&cache_path, expanded_code)?;
    Ok(())
}

/// Returns true if `path` is a `.ts` or `.tsx` file that should be processed.
pub(crate) fn is_watchable_ts_file(path: &Path, root: &Path) -> bool {
    let ext = path.extension().and_then(|e| e.to_str());
    if !matches!(ext, Some("ts" | "tsx")) {
        return false;
    }
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    if name.ends_with(".d.ts") || name.contains(".expanded.") {
        return false;
    }
    let rel = path.strip_prefix(root).unwrap_or(path).to_string_lossy();
    if rel.contains("node_modules") || rel.contains(".macroforge") {
        return false;
    }
    true
}

/// Collects all watchable TypeScript files under `root`.
pub(crate) fn collect_watch_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let walker = WalkBuilder::new(root)
        .hidden(false)
        .git_ignore(true)
        .git_global(false)
        .git_exclude(false)
        .build();

    for entry in walker.flatten() {
        let path = entry.path();
        if path.is_file() && is_watchable_ts_file(path, root) {
            files.push(path.to_path_buf());
        }
    }
    files
}

/// Expands a single file for caching purposes.
///
/// Returns `Ok(Some(expanded_code))` if macros were found and expanded,
/// `Ok(None)` if no macros present, or `Err` on failure.
/// Check if source code contains `@derive(` as a standalone JSDoc directive,
/// or imports the declarative macro module (`"macroforge/rules"`).
///
/// Only matches `@derive(` when it appears at the start of a JSDoc line (after
/// stripping `/**`, `*/`, `*`, and whitespace). Skips `@derive` embedded in prose
/// (e.g., `"result from @derive(Deserialize)"`) and inside fenced code blocks.
pub(crate) fn has_macro_annotations(source: &str) -> bool {
    // Declarative macros:
    //   - defining files import `macroRules` from `"macroforge/rules"`
    //   - consuming files use a `/** import macro { $name } from "..." */`
    //     JSDoc comment
    // Either signal means the pre-pass must run.
    if source.contains("macroforge/rules") {
        return true;
    }
    if source.contains("import macro") {
        return true;
    }
    if !source.contains("@derive") {
        return false;
    }
    let mut in_code_block = false;
    for line in source.lines() {
        // Strip JSDoc comment syntax: /**, */, leading *, and whitespace
        let trimmed = line
            .trim()
            .trim_start_matches('/')
            .trim_start_matches('*')
            .trim_end_matches('/')
            .trim_end_matches('*')
            .trim();
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block {
            continue;
        }
        // A line must START with @derive( to be a real directive.
        // This rejects prose like "result from @derive(Deserialize)".
        if trimmed.starts_with("@derive(") {
            return true;
        }
    }
    false
}

pub(crate) fn expand_for_cache(path: &Path, source: &str) -> Result<Option<String>> {
    // Quick check: skip files without @derive (ignoring fenced code blocks in docs)
    if !has_macro_annotations(source) {
        return Ok(None);
    }

    use macroforge_ts::host::MacroforgeConfigLoader;

    if let Ok(Some(config)) = MacroforgeConfigLoader::find_from_path(path) {
        macroforge_ts::host::set_foreign_types(config.foreign_types.clone());
    }

    let mut expander = MacroExpander::new().context("failed to initialize macro expander")?;

    // Load the pre-built type registry so generic types (e.g. RecordLink<T>)
    // can be resolved inline at each call site.
    let registry_path = TYPE_REGISTRY_CACHE_PATH.lock().unwrap().clone();
    if let Some(ref rp) = registry_path
        && let Ok(json) = fs::read_to_string(rp)
        && let Ok(registry) = serde_json::from_str::<
            macroforge_ts::ts_syn::abi::ir::type_registry::TypeRegistry,
        >(&json)
    {
        expander.set_type_registry(registry);
    }

    // Also load the declarative macro registry so cross-file
    // `/** import macro { $foo } from "./bar" */` comments resolve.
    let declarative_registry_path = DECLARATIVE_REGISTRY_CACHE_PATH.lock().unwrap().clone();
    if let Some(ref dp) = declarative_registry_path
        && let Ok(json) = fs::read_to_string(dp)
        && let Ok(registry) =
            macroforge_ts::host::declarative::ProjectDeclarativeRegistry::from_json(&json)
    {
        expander.set_declarative_registry(Some(registry));
    }

    let expansion = expander
        .expand_source(source, &path.display().to_string())
        .map_err(|err| anyhow!(format!("{err:?}")))?;

    macroforge_ts::host::clear_registry();
    macroforge_ts::host::clear_foreign_types();

    if !expansion.changed {
        return Ok(None);
    }

    Ok(Some(expansion.code))
}

/// Warm the cache: expand all files and save the manifest. Returns the manifest for reuse.
pub(crate) fn warm_cache(
    label: &str,
    root: &Path,
    cache_dir: &Path,
    manifest: &mut CacheManifest,
) -> Result<()> {
    use rayon::prelude::*;

    eprintln!("[macroforge {label}] Warming cache for {}", root.display());

    // Build the type registry before expanding so macros have cross-module type awareness
    ensure_type_registry_cache();

    let start = std::time::Instant::now();
    let files = collect_watch_files(root);
    let mut expanded_count = 0u32;

    // Phase 1: Read files, hash, filter out already-cached (sequential for manifest reads)
    let mut files_to_expand: Vec<(PathBuf, String, String, String, String)> = Vec::new();
    for file_path in &files {
        let rel_path = file_path
            .strip_prefix(root)
            .unwrap_or(file_path)
            .to_string_lossy()
            .to_string();

        let source = match fs::read_to_string(file_path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let source_hash = content_hash(source.as_bytes());

        // Skip if already cached with matching hash
        if let Some(entry) = manifest.entries.get(&rel_path)
            && entry.source_hash == source_hash
        {
            // Backfill normalized_hash for entries from older manifests
            if entry.normalized_hash.is_empty() {
                let norm_hash = normalized_content_hash(&source);
                manifest.entries.insert(
                    rel_path,
                    CacheEntry {
                        source_hash,
                        has_macros: entry.has_macros,
                        normalized_hash: norm_hash,
                    },
                );
            }
            continue;
        }

        let norm_hash = normalized_content_hash(&source);
        files_to_expand.push((file_path.clone(), rel_path, source, source_hash, norm_hash));
    }

    // Phase 2: Expand in parallel
    let pool = rayon::ThreadPoolBuilder::new().build()?;

    let results: Vec<_> = pool.install(|| {
        files_to_expand
            .par_iter()
            .map(|(file_path, rel_path, source, source_hash, norm_hash)| {
                let result = expand_for_cache(file_path, source);
                (
                    rel_path.clone(),
                    source_hash.clone(),
                    norm_hash.clone(),
                    result,
                )
            })
            .collect()
    });

    // Phase 3: Apply results to manifest (sequential)
    for (rel_path, source_hash, norm_hash, result) in results {
        match result {
            Ok(Some(expanded)) => {
                if let Err(e) = write_cache_file(cache_dir, &rel_path, &expanded) {
                    eprintln!("  [!] {} — write failed: {}", rel_path, e);
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
                expanded_count += 1;
                eprintln!("  [+] {}", rel_path);
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
            Err(e) => {
                eprintln!("  [!] {} — {}", rel_path, e);
            }
        }
    }

    manifest.save(cache_dir)?;
    let elapsed = start.elapsed();
    eprintln!(
        "[macroforge {label}] Cache warm: {} files expanded in {:.1}s ({} total files)",
        expanded_count,
        elapsed.as_secs_f64(),
        files.len()
    );
    Ok(())
}

/// Resolves root path, creates cache dir reference, and loads/creates the manifest.
pub(crate) fn init_cache(
    root: Option<PathBuf>,
    label: &str,
) -> Result<(PathBuf, PathBuf, CacheManifest)> {
    let root = root
        .unwrap_or_else(|| PathBuf::from("."))
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from("."));
    let cache_dir = root.join(".macroforge").join("cache");
    let version = env!("CARGO_PKG_VERSION").to_string();
    let config_hash = compute_config_hash(&root);
    let external_macro_hash = compute_external_macro_hash(&root);

    let manifest = CacheManifest::load(&cache_dir)
        .filter(|m| {
            if m.version != version {
                eprintln!("[macroforge {label}] Cache invalidated: macroforge version changed");
                return false;
            }
            if m.config_hash != config_hash {
                eprintln!("[macroforge {label}] Cache invalidated: config changed");
                return false;
            }
            if m.external_macro_hash != external_macro_hash {
                eprintln!("[macroforge {label}] Cache invalidated: external macro binary changed");
                return false;
            }
            true
        })
        .unwrap_or_else(|| {
            eprintln!("[macroforge {label}] Creating fresh cache");
            CacheManifest::new(
                version.clone(),
                config_hash.clone(),
                external_macro_hash.clone(),
            )
        });

    Ok((root, cache_dir, manifest))
}

/// Build the .macroforge/cache once and exit.
pub fn run_cache(root: Option<PathBuf>) -> Result<()> {
    let (root, cache_dir, mut manifest) = init_cache(root, "cache")?;
    warm_cache("cache", &root, &cache_dir, &mut manifest)?;
    Ok(())
}

/// Delete the .macroforge/cache directory and rebuild from scratch.
pub fn run_refresh(root: Option<PathBuf>) -> Result<()> {
    let root_resolved = root
        .clone()
        .unwrap_or_else(|| PathBuf::from("."))
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from("."));
    let cache_dir = root_resolved.join(".macroforge").join("cache");

    if cache_dir.exists() {
        eprintln!("[macroforge refresh] Deleting {}", cache_dir.display());
        fs::remove_dir_all(&cache_dir).context("failed to delete .macroforge/cache")?;
    } else {
        eprintln!("[macroforge refresh] No existing cache found, building fresh");
    }

    let (root, cache_dir, mut manifest) = init_cache(root, "refresh")?;
    warm_cache("refresh", &root, &cache_dir, &mut manifest)?;
    Ok(())
}
