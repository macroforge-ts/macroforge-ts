use anyhow::{Context, Result, anyhow};
use ignore::WalkBuilder;
use macroforge_ts::host::MacroExpander;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::wrappers::{TYPE_REGISTRY_CACHE_PATH, ensure_type_registry_cache};

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
    /// Whether this cache was built with `--builtin-only` (no external macros).
    #[serde(default)]
    pub(crate) builtin_only: bool,
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

impl CacheManifest {
    pub(crate) fn new(version: String, config_hash: String, builtin_only: bool) -> Self {
        Self {
            version,
            config_hash,
            builtin_only,
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
/// Check if source code contains `@derive(` as a standalone JSDoc directive.
///
/// Only matches `@derive(` when it appears at the start of a JSDoc line (after
/// stripping `/**`, `*/`, `*`, and whitespace). Skips `@derive` embedded in prose
/// (e.g., `"result from @derive(Deserialize)"`) and inside fenced code blocks.
pub(crate) fn has_macro_annotations(source: &str) -> bool {
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

pub(crate) fn expand_for_cache(
    path: &Path,
    source: &str,
    builtin_only: bool,
) -> Result<Option<String>> {
    // Quick check: skip files without @derive (ignoring fenced code blocks in docs)
    if !has_macro_annotations(source) {
        return Ok(None);
    }

    if builtin_only {
        use macroforge_ts::host::MacroforgeConfigLoader;

        if let Ok(Some(config)) = MacroforgeConfigLoader::find_from_path(path) {
            macroforge_ts::host::set_foreign_types(config.foreign_types.clone());
        }

        let expander = MacroExpander::new().context("failed to initialize macro expander")?;
        let expansion = expander
            .expand_source(source, &path.display().to_string())
            .map_err(|err| anyhow!(format!("{err:?}")))?;

        macroforge_ts::host::clear_registry();
        macroforge_ts::host::clear_foreign_types();

        if !expansion.changed {
            return Ok(None);
        }

        Ok(Some(expansion.code))
    } else {
        expand_for_cache_via_node(path, source)
    }
}

/// Expands a file via Node.js for caching, returning the expanded code.
fn expand_for_cache_via_node(path: &Path, source: &str) -> Result<Option<String>> {
    // Ensure type registry has been built (once per process)
    ensure_type_registry_cache();

    let script = r#"
const { createRequire } = require('module');
const fs = require('fs');
const path = require('path');
const cwdRequire = createRequire(process.cwd() + '/package.json');

let expandSync, loadConfig, collectExternalDecoratorModules;
try {
  const macroforge = cwdRequire('macroforge');
  expandSync = macroforge.expandSync;
  loadConfig = macroforge.loadConfig;
  try {
    const shared = cwdRequire('@macroforge/shared');
    collectExternalDecoratorModules = shared.collectExternalDecoratorModules;
  } catch (e) {
    console.error('[macroforge] warning: @macroforge/shared not available, external decorator filtering disabled:', e.message);
  }
} catch {
  console.log(JSON.stringify({ fallback: true }));
  process.exit(0);
}

const inputPath = process.argv[2];
const typeRegistryPath = process.argv[3];
const code = fs.readFileSync(inputPath, 'utf8');

const CONFIG_FILES = [
  'macroforge.config.ts', 'macroforge.config.mts',
  'macroforge.config.js', 'macroforge.config.mjs', 'macroforge.config.cjs',
];

function findConfigFile(startDir) {
  let dir = startDir;
  while (dir) {
    for (const configName of CONFIG_FILES) {
      const configPath = path.join(dir, configName);
      if (fs.existsSync(configPath)) return configPath;
    }
    if (fs.existsSync(path.join(dir, 'package.json'))) break;
    const parent = path.dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  return null;
}

try {
  const inputDir = path.dirname(path.resolve(inputPath));
  const configPath = findConfigFile(inputDir);
  let options = {};
  if (configPath) {
    const configContent = fs.readFileSync(configPath, 'utf8');
    loadConfig(configContent, configPath);
    options.configPath = configPath;
  }

  // Load pre-built type registry if available
  if (typeRegistryPath) {
    try {
      options.typeRegistryJson = fs.readFileSync(typeRegistryPath, 'utf8');
    } catch {}
  }

  // Collect external decorator modules so the annotation filter includes them
  if (collectExternalDecoratorModules) {
    try {
      options.externalDecoratorModules = collectExternalDecoratorModules(code, cwdRequire);
    } catch (e) {
      console.error('[macroforge] warning: failed to collect external decorator modules:', e.message);
    }
  }

  const result = expandSync(code, inputPath, options);
  const hasExpansions = result.sourceMapping?.generatedRegions?.length > 0;
  console.log(JSON.stringify({
    code: result.code,
    hasExpansions,
  }));
} catch (err) {
  console.error('Error:', err.message);
  process.exit(1);
}
"#;

    let mut temp_dir = std::env::temp_dir();
    temp_dir.push("macroforge-cli");
    fs::create_dir_all(&temp_dir)?;
    let script_path = temp_dir.join("cache-expand-wrapper.js");
    fs::write(&script_path, script)?;

    let cwd = std::env::current_dir()?;
    let registry_path = TYPE_REGISTRY_CACHE_PATH.lock().unwrap().clone();
    let mut cmd = std::process::Command::new("node");
    cmd.arg(&script_path).arg(path);
    if let Some(ref rp) = registry_path {
        cmd.arg(rp);
    }
    let output = cmd
        .current_dir(&cwd)
        .output()
        .context("failed to run node expand wrapper")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("node expansion failed: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let result: serde_json::Value =
        serde_json::from_str(&stdout).context("failed to parse expansion result")?;

    // Check for fallback marker
    if result
        .get("fallback")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        // Fall back to builtin
        return expand_for_cache(path, source, true);
    }

    let has_expansions = result["hasExpansions"].as_bool().unwrap_or(false);
    if !has_expansions {
        return Ok(None);
    }

    let code = result["code"]
        .as_str()
        .ok_or_else(|| anyhow!("missing 'code' in expansion result"))?;

    Ok(Some(code.to_string()))
}

/// Warm the cache: expand all files and save the manifest. Returns the manifest for reuse.
pub(crate) fn warm_cache(
    label: &str,
    root: &Path,
    cache_dir: &Path,
    manifest: &mut CacheManifest,
    builtin_only: bool,
) -> Result<()> {
    use rayon::prelude::*;

    eprintln!("[macroforge {label}] Warming cache for {}", root.display());
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
    let pool = if builtin_only {
        rayon::ThreadPoolBuilder::new().build()?
    } else {
        rayon::ThreadPoolBuilder::new().num_threads(4).build()?
    };

    let results: Vec<_> = pool.install(|| {
        files_to_expand
            .par_iter()
            .map(|(file_path, rel_path, source, source_hash, norm_hash)| {
                let result = expand_for_cache(file_path, source, builtin_only);
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
    builtin_only: bool,
) -> Result<(PathBuf, PathBuf, CacheManifest)> {
    let root = root
        .unwrap_or_else(|| PathBuf::from("."))
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from("."));
    let cache_dir = root.join(".macroforge").join("cache");
    let version = env!("CARGO_PKG_VERSION").to_string();
    let config_hash = compute_config_hash(&root);

    let manifest = CacheManifest::load(&cache_dir)
        .filter(|m| {
            m.version == version && m.config_hash == config_hash && m.builtin_only == builtin_only
        })
        .unwrap_or_else(|| {
            eprintln!("[macroforge {label}] Creating fresh cache");
            CacheManifest::new(version.clone(), config_hash.clone(), builtin_only)
        });

    Ok((root, cache_dir, manifest))
}

/// Build the .macroforge/cache once and exit.
pub fn run_cache(root: Option<PathBuf>, builtin_only: bool) -> Result<()> {
    let (root, cache_dir, mut manifest) = init_cache(root, "cache", builtin_only)?;
    warm_cache("cache", &root, &cache_dir, &mut manifest, builtin_only)?;
    Ok(())
}

/// Delete the .macroforge/cache directory and rebuild from scratch.
pub fn run_refresh(root: Option<PathBuf>, builtin_only: bool) -> Result<()> {
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

    let (root, cache_dir, mut manifest) = init_cache(root, "refresh", builtin_only)?;
    warm_cache("refresh", &root, &cache_dir, &mut manifest, builtin_only)?;
    Ok(())
}
