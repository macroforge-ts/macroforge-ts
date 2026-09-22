//! Bump release versions
//!
//! Raises the selected packages' versions, cascading to their dependents, and
//! rewrites everything stamped with a version: the manifests, versions.json,
//! the Zed extension constants, Cargo.lock and the extracted API docs. Rolls
//! the whole bump back on failure or Ctrl+C.

use crate::cli::BumpArgs;
use crate::cli::commands::docs::extract_api_docs;
use crate::core::config::{self, Config};
use crate::core::deps;
use crate::core::manifests;
use crate::core::registry;
use crate::core::repos::Repo;
use crate::core::shell;
use crate::core::versions;
use anyhow::{Context, Result};
use colored::Colorize;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Cascade to all dependents recursively
fn cascade_to_dependents(
    initial_repos: &[&Repo],
    deps_map: &HashMap<String, Vec<String>>,
) -> Vec<String> {
    let mut result: HashSet<String> = initial_repos.iter().map(|r| r.name.clone()).collect();
    let mut to_process: Vec<String> = initial_repos.iter().map(|r| r.name.clone()).collect();

    // Build reverse dependency map (pkg -> dependents)
    let mut dependents_map: HashMap<String, Vec<String>> = HashMap::new();
    for (dependent, dependencies) in deps_map {
        for dep in dependencies {
            dependents_map
                .entry(dep.clone())
                .or_default()
                .push(dependent.clone());
        }
    }

    // Traverse dependents
    while let Some(repo) = to_process.pop() {
        if let Some(dependents) = dependents_map.get(&repo) {
            for dependent in dependents {
                if result.insert(dependent.clone()) {
                    to_process.push(dependent.clone());
                }
            }
        }
    }

    // Return in topological order
    match deps::topo_order(deps_map) {
        Ok(sorted) => sorted.into_iter().filter(|r| result.contains(r)).collect(),
        Err(_) => result.into_iter().collect(),
    }
}

/// The trees `extract_api_docs` rewrites.
const GENERATED_DOC_TREES: &[&str] = &[
    "website/static/api-data",
    "website/src/routes/docs/builtin-macros",
    "packages/mcp-server/docs/builtin-macros",
];

/// The generated documentation as it stood before a run touched it.
///
/// The docs carry the version they were extracted at, so a bump rewrites them
/// and undoing the bump has to put them back. Restoring the bytes rather than
/// re-extracting is what makes that exact: every extraction stamps a fresh
/// `generated` time, so a regenerated file matches the original in content and
/// still shows up as a change on a run that produced nothing.
struct DocsSnapshot {
    files: Vec<(PathBuf, Vec<u8>)>,
}

impl DocsSnapshot {
    fn capture(config: &Config) -> Self {
        let mut files = Vec::new();
        for tree in GENERATED_DOC_TREES {
            collect_files(&config.root.join(tree), &mut files);
        }
        Self { files }
    }

    fn restore(&self) {
        for (path, contents) in &self.files {
            if std::fs::read(path).is_ok_and(|current| current == *contents) {
                continue;
            }
            if let Err(e) = std::fs::write(path, contents) {
                eprintln!(
                    "  {} Failed to restore {}: {}",
                    "✗".red(),
                    path.display(),
                    e
                );
            }
        }
    }
}

/// Reads every file under `dir` into `out`, ignoring a tree that is not there.
fn collect_files(dir: &Path, out: &mut Vec<(PathBuf, Vec<u8>)>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, out);
        } else if let Ok(contents) = std::fs::read(&path) {
            out.push((path, contents));
        }
    }
}

/// Put versions.json, every manifest, Cargo.lock and the docs back as they were.
fn rollback(config: &Config, original: &versions::VersionsCache, docs: &DocsSnapshot) {
    eprintln!("\n{} Rolling back version changes...", "⚠".yellow());
    if let Err(e) = original.save(&config.root) {
        eprintln!("  {} Failed to restore versions.json: {}", "✗".red(), e);
    }
    if let Err(e) = manifests::apply_versions(config, original) {
        eprintln!("  {} Failed to restore manifest files: {}", "✗".red(), e);
    }
    if let Err(e) = shell::cargo::sync_lock(&config.root) {
        eprintln!("  {} Failed to restore Cargo.lock: {}", "✗".red(), e);
    }
    docs.restore();
    eprintln!("{} Rollback complete", "✓".green());
}

/// Entry point for `mf bump`.
pub fn run(args: BumpArgs) -> Result<()> {
    let config = Config::load()?;

    let original_versions_cache = config.versions.clone();
    let docs_snapshot = std::sync::Arc::new(DocsSnapshot::capture(&config));
    {
        let config = config.clone();
        let original = original_versions_cache.clone();
        let docs = docs_snapshot.clone();
        let interrupted = std::sync::atomic::AtomicBool::new(false);
        ctrlc::set_handler(move || {
            if interrupted.swap(true, std::sync::atomic::Ordering::SeqCst) {
                eprintln!("\n{} Force exiting...", "⚠".red());
                std::process::exit(130);
            }
            eprintln!("\n{} Received Ctrl+C", "⚠".yellow());
            rollback(&config, &original, &docs);
            std::process::exit(130);
        })
        .context("Failed to set Ctrl+C handler")?;
    }

    let initial_repos = config.filter_repos(&args.repos);
    if initial_repos.is_empty() {
        anyhow::bail!("No repos matched filter: {}", args.repos);
    }
    let initial_names: HashSet<String> = initial_repos.iter().map(|r| r.name.clone()).collect();

    let repo_names: Vec<String> = if args.no_cascade || args.repos == "all" {
        initial_repos.iter().map(|r| r.name.clone()).collect()
    } else {
        let cascaded = cascade_to_dependents(&initial_repos, &config.deps);
        let added: Vec<&str> = cascaded
            .iter()
            .filter(|n| !initial_names.contains(*n))
            .map(|s| s.as_str())
            .collect();
        if !added.is_empty() {
            println!(
                "{}: Adding dependents: {}",
                "Cascading".yellow(),
                added.join(", ").cyan()
            );
        }
        cascaded
    };

    let sorted_names = deps::topo_order(&config.deps)?;
    let repos: Vec<&Repo> = sorted_names
        .iter()
        .filter(|name| repo_names.contains(name))
        .filter_map(|name| config.repos.get(name))
        .collect();
    if repos.is_empty() {
        anyhow::bail!("No repos to bump");
    }

    let original_versions: HashMap<String, String> = repos
        .iter()
        .filter_map(|r| {
            config
                .versions
                .get_local(&r.name)
                .map(|v| (r.name.clone(), v.to_string()))
        })
        .collect();

    let base_version = repos
        .iter()
        .filter_map(|r| config.versions.get_local(&r.name))
        .max_by(|a, b| {
            if versions::version_gt(a, b) {
                std::cmp::Ordering::Greater
            } else if versions::version_gt(b, a) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .unwrap_or("0.1.0")
        .to_string();
    let target_version = args
        .version
        .clone()
        .unwrap_or_else(|| versions::increment_patch(&base_version).unwrap_or("0.1.0".to_string()));

    println!("\n{}", "=".repeat(60));
    println!(
        "{} ({})",
        "Bump versions".bold(),
        if args.sync_versions {
            "sync"
        } else {
            "increment"
        }
    );
    println!(
        "Repos: {}",
        repos
            .iter()
            .map(|r| r.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
            .cyan()
    );
    println!("{}", "=".repeat(60));

    let bump = || -> Result<()> {
        let mut versions_cache = config.versions.clone();
        for repo in &repos {
            // A cascaded dependent is bumped only if it has not been already.
            let should_bump = initial_names.contains(&repo.name)
                || versions_cache.get_local(&repo.name) == versions_cache.get_registry(&repo.name);

            let version = if !should_bump {
                versions_cache
                    .get_local(&repo.name)
                    .unwrap_or("0.1.0")
                    .to_string()
            } else if args.sync_versions {
                target_version.clone()
            } else {
                args.version.clone().unwrap_or_else(|| {
                    let current = versions_cache.get_local(&repo.name).unwrap_or("0.1.0");
                    versions::increment_patch(current).unwrap_or("0.1.0".to_string())
                })
            };

            if repo.package_json.is_some() || repo.cargo_toml.is_some() {
                manifests::set_version(&config, &mut versions_cache, &repo.name, &version)?;
            }
        }

        if args.repos == "all" || repo_names.iter().any(|n| n == "zed-extensions") {
            // A dependency being bumped publishes at its new version; any
            // other stays at what npm serves.
            let npm_names = config::npm_package_names();
            for dep in ["typescript-plugin", "core", "svelte-language-server"] {
                if repo_names.iter().any(|name| name == dep) {
                    if let Some(local) = versions_cache.get_local(dep).map(|s| s.to_string()) {
                        versions_cache.set_registry(dep, &local);
                    }
                } else if let Some(npm_name) = npm_names.get(dep)
                    && let Some(published) = registry::npm_version(npm_name)
                        .with_context(|| format!("Failed to look up {npm_name} on npm"))?
                {
                    versions_cache.set_registry(dep, &published);
                }
            }
            manifests::update_zed_extensions(&config.root, &versions_cache)
                .context("Failed to update the zed extension manifests")?;
        }

        versions_cache.save(&config.root)?;
        shell::cargo::sync_lock(&config.root).context("Failed to update Cargo.lock")?;

        println!("\n{}", "Re-extracting the API docs".bold());
        extract_api_docs(&config.root)?;
        shell::deno::deno_fmt(&config.root, &["website/static/api-data"])
            .context("Failed to format the API docs")?;
        Ok(())
    };

    if let Err(error) = bump() {
        rollback(&config, &original_versions_cache, &docs_snapshot);
        return Err(error);
    }

    // Version summary
    println!("\n{}", "═".repeat(80));
    println!("{}", "Version Summary".bold());
    println!("{}", "═".repeat(80));

    // Reload final versions
    let final_versions = versions::VersionsCache::load(&config.root)?;

    // Published names mapping
    let pub_names: HashMap<&str, &str> = [
        ("core", "@macroforge/core"),
        ("shared", "@macroforge/shared"),
        ("vite-plugin", "@macroforge/vite-plugin"),
        ("typescript-plugin", "@macroforge/typescript-plugin"),
        (
            "svelte-language-server",
            "@macroforge/svelte-language-server",
        ),
        ("svelte-preprocessor", "@macroforge/svelte-preprocessor"),
        ("mcp-server", "@macroforge/mcp-server"),
        ("deno-plugin", "@macroforge/deno-plugin"),
        ("syn", "macroforge_ts_syn"),
        ("template", "macroforge_ts_quote"),
        ("macros", "macroforge_ts_macros"),
        ("website", "macroforge.dev"),
        ("zed-extensions", "zed extensions"),
    ]
    .into_iter()
    .collect();

    let name_width = repos.iter().map(|r| r.name.len()).max().unwrap_or(8).max(8);
    let pub_width = repos
        .iter()
        .filter_map(|r| pub_names.get(r.name.as_str()).map(|s| s.len()))
        .max()
        .unwrap_or(20)
        .max(20);
    let ver_width = 8;

    for repo in &repos {
        let is_package = repo.package_json.is_some() || repo.cargo_toml.is_some();
        let pub_name = pub_names.get(repo.name.as_str()).unwrap_or(&"—");

        if is_package {
            let before = original_versions
                .get(&repo.name)
                .map(|s| s.as_str())
                .unwrap_or("(new)");
            let after = final_versions.get_local(&repo.name).unwrap_or("?");
            let changed = before != after;

            println!(
                "  {:<nw$}  {:<pw$}  {:>vw$}  →  {:>vw$}",
                repo.name.cyan(),
                pub_name,
                before.dimmed(),
                if changed {
                    after.green()
                } else {
                    after.dimmed()
                },
                nw = name_width,
                pw = pub_width,
                vw = ver_width
            );
        } else {
            println!(
                "  {:<nw$}  {:<pw$}  {:>vw$}  →  {:>vw$}",
                repo.name.dimmed(),
                pub_name.dimmed(),
                "—".dimmed(),
                "—".dimmed(),
                nw = name_width,
                pw = pub_width,
                vw = ver_width
            );
        }
    }

    println!("\n{}", "═".repeat(80));
    println!("\n{} {}", "Next step:".bold(), "pixi run publish".cyan());
    Ok(())
}
