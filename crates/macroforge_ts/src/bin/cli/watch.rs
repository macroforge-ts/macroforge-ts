use anyhow::{Context, Result};
use notify_debouncer_full::{new_debouncer, notify::RecursiveMode};
use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use crate::cache::{
    CacheEntry,
    collect_watch_files, compute_config_hash, content_hash, expand_for_cache,
    init_cache, is_watchable_ts_file, normalized_content_hash, warm_cache, write_cache_file,
};

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
    /// Rust NAPI macro (has Cargo.toml with macroforge_ts dependency)
    RustNapi,
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

/// Discover macro source packages from workspace configuration.
fn discover_macro_sources(root: &Path) -> Vec<MacroSourceInfo> {
    let pkg_json_path = root.join("package.json");
    let pkg_json = match fs::read_to_string(&pkg_json_path) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };
    let pkg: serde_json::Value = match serde_json::from_str(&pkg_json) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    // Collect workspace patterns
    let workspace_patterns: Vec<String> = match &pkg["workspaces"] {
        serde_json::Value::Array(arr) => arr
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect(),
        serde_json::Value::Object(obj) => obj
            .get("packages")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default(),
        _ => Vec::new(),
    };

    // Expand workspace patterns into actual directories
    let mut workspace_dirs: Vec<PathBuf> = Vec::new();
    for pattern in &workspace_patterns {
        if pattern.contains('*') {
            // Simple glob: "packages/*" -> list subdirs of "packages/"
            let prefix = pattern.trim_end_matches("/*").trim_end_matches("/**");
            let base = root.join(prefix);
            if let Ok(entries) = fs::read_dir(&base) {
                for entry in entries.flatten() {
                    if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                        workspace_dirs.push(entry.path());
                    }
                }
            }
        } else {
            let dir = root.join(pattern);
            if dir.is_dir() {
                workspace_dirs.push(dir);
            }
        }
    }

    let mut sources = Vec::new();
    for dir in &workspace_dirs {
        let cargo_toml = dir.join("Cargo.toml");
        let child_pkg_json = dir.join("package.json");

        // Check if it's a Rust NAPI macro crate
        if cargo_toml.exists()
            && let Ok(content) = fs::read_to_string(&cargo_toml)
            && content.contains("macroforge_ts")
        {
            let src_dir = dir.join("src");
            let name = dir
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            sources.push(MacroSourceInfo {
                name,
                package_dir: dir.clone(),
                source_dirs: if src_dir.is_dir() {
                    vec![src_dir]
                } else {
                    vec![dir.clone()]
                },
                kind: MacroSourceKind::RustNapi,
            });
            continue;
        }

        // Check if it's a JS/TS macro package
        if child_pkg_json.exists()
            && let Ok(content) = fs::read_to_string(&child_pkg_json)
            && let Ok(child_pkg) = serde_json::from_str::<serde_json::Value>(&content)
        {
            let has_macroforge_dep = ["dependencies", "devDependencies", "peerDependencies"]
                .iter()
                .any(|key| {
                    child_pkg[key]
                        .as_object()
                        .is_some_and(|deps| deps.contains_key("macroforge"))
                });

            if has_macroforge_dep {
                let src_dir = dir.join("src");
                let name = child_pkg["name"].as_str().unwrap_or_default().to_string();
                sources.push(MacroSourceInfo {
                    name: if name.is_empty() {
                        dir.file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string()
                    } else {
                        name
                    },
                    package_dir: dir.clone(),
                    source_dirs: if src_dir.is_dir() {
                        vec![src_dir]
                    } else {
                        vec![dir.clone()]
                    },
                    kind: MacroSourceKind::JsTs,
                });
            }
        }
    }

    sources
}

/// Rebuild a macro package. Returns Ok(()) on success.
fn rebuild_macro(info: &MacroSourceInfo, build_system: BuildSystem, _root: &Path) -> Result<()> {
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
        .env("NAPI_BUILD_SKIP_WATCHER", "1")
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

/// Main watch loop: warm cache then watch for changes.
pub fn run_watch(root: Option<PathBuf>, builtin_only: bool, debounce_ms: u64) -> Result<()> {
    let (root, cache_dir, mut manifest) = init_cache(root, "watch", builtin_only)?;
    warm_cache("watch", &root, &cache_dir, &mut manifest, builtin_only)?;

    // Discover macro source packages and watch them
    let macro_sources = discover_macro_sources(&root);
    let build_system = detect_build_system(&root);

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

    debouncer
        .watch(&root, RecursiveMode::Recursive)
        .context("failed to start watching")?;

    // Watch macro source directories that are outside root
    for src in &macro_sources {
        for dir in &src.source_dirs {
            if dir.exists() && !dir.starts_with(&root) {
                debouncer
                    .watch(dir, RecursiveMode::Recursive)
                    .with_context(|| format!("failed to watch macro source: {}", dir.display()))?;
            }
        }
    }

    for result in rx {
        match result {
            Ok(events) => {
                let mut config_changed = false;
                let mut macro_source_changed: Option<&MacroSourceInfo> = None;
                let mut changed_files: Vec<PathBuf> = Vec::new();

                for event in &events {
                    for event_path in &event.paths {
                        // Check for config file changes
                        if let Some(name) = event_path.file_name() {
                            let name_str = name.to_string_lossy();
                            if name_str.starts_with("macroforge.config.") {
                                config_changed = true;
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

                        if is_watchable_ts_file(event_path, &root) {
                            changed_files.push(event_path.clone());
                        }
                    }
                }

                changed_files.sort();
                changed_files.dedup();

                // Macro source change: rebuild then full re-expand
                if let Some(macro_info) = macro_source_changed {
                    eprintln!(
                        "[macroforge watch] Macro source changed in '{}', rebuilding...",
                        macro_info.name
                    );

                    match rebuild_macro(macro_info, build_system, &root) {
                        Ok(()) => {
                            eprintln!(
                                "[macroforge watch] Macro '{}' rebuilt, re-expanding all files...",
                                macro_info.name
                            );
                            manifest.entries.clear();
                            macroforge_ts::host::clear_config_cache();
                            warm_cache("watch", &root, &cache_dir, &mut manifest, builtin_only)?;
                        }
                        Err(e) => {
                            eprintln!("[macroforge watch] Macro rebuild failed: {}", e);
                        }
                    }
                } else if config_changed {
                    use rayon::prelude::*;

                    let new_config_hash = compute_config_hash(&root);
                    manifest.config_hash = new_config_hash;
                    manifest.entries.clear();
                    eprintln!("[macroforge watch] Config changed, re-expanding all files...");

                    let all_files = collect_watch_files(&root);

                    // Read and hash all files (sequential)
                    let files_with_source: Vec<_> = all_files
                        .iter()
                        .filter_map(|file_path| {
                            let rel_path = file_path
                                .strip_prefix(&root)
                                .unwrap_or(file_path)
                                .to_string_lossy()
                                .to_string();
                            let source = fs::read_to_string(file_path).ok()?;
                            let source_hash = content_hash(source.as_bytes());
                            let norm_hash = normalized_content_hash(&source);
                            Some((file_path.clone(), rel_path, source, source_hash, norm_hash))
                        })
                        .collect();

                    // Expand in parallel
                    let results: Vec<_> = files_with_source
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
                        .collect();

                    // Apply results (sequential)
                    let mut count = 0u32;
                    for (rel_path, source_hash, norm_hash, result) in results {
                        match result {
                            Ok(Some(expanded)) => {
                                let _ = write_cache_file(&cache_dir, &rel_path, &expanded);
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
                            Err(e) => eprintln!("  [!] {} — {}", rel_path, e),
                        }
                    }
                    manifest.save(&cache_dir)?;
                    eprintln!("[macroforge watch] Re-expanded {} files", count);
                } else if !changed_files.is_empty() {
                    for file_path in &changed_files {
                        let rel_path = file_path
                            .strip_prefix(&root)
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
                                let _ = fs::remove_file(&cache_path);
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

                        match expand_for_cache(file_path, &source, builtin_only) {
                            Ok(Some(expanded)) => {
                                let _ = write_cache_file(&cache_dir, &rel_path, &expanded);
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
                                eprintln!("  [!] {} — {}", rel_path, e);
                            }
                        }
                    }
                    manifest.save(&cache_dir)?;
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
