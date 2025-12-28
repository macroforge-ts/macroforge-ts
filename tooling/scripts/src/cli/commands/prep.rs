//! Prepare release command
//!
//! Bumps versions, builds packages, runs tests, and generates documentation.

use crate::cli::commands::docs::{build_book, extract_rust, extract_ts};
use crate::cli::PrepArgs;
use crate::core::config::Config;
use crate::core::deps;
use crate::core::manifests;
use crate::core::repos::{Repo, RepoType};
use crate::core::shell;
use crate::core::versions;
use crate::utils::format;
use anyhow::{Context, Result};
use colored::Colorize;
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Step configuration for the prep workflow
struct Step {
    number: f32,
    total: usize,
    label: String,
}

impl Step {
    fn print(&self) {
        if self.number.fract() == 0.0 {
            println!(
                "\n{} {}",
                format!("[{}/{}]", self.number as usize, self.total).bold(),
                self.label.bold()
            );
        } else {
            println!(
                "\n{} {}",
                format!("[{}/{}]", self.number, self.total).bold(),
                self.label.bold()
            );
        }
    }
}

/// Cascade to all dependents recursively
fn cascade_to_dependents(
    initial_repos: &[&Repo],
    _all_repos: &HashMap<String, Repo>,
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
        Ok(sorted) => sorted
            .into_iter()
            .filter(|r| result.contains(r))
            .collect(),
        Err(_) => result.into_iter().collect(),
    }
}


/// Build a single repository
fn build_repo(
    config: &Config,
    repo: &Repo,
    built_repos: &[&str],
    deps_map: &HashMap<String, Vec<String>>,
    versions_cache: &crate::core::versions::VersionsCache,
    verbose: bool,
) -> Result<()> {
    match repo.repo_type {
        RepoType::Rust => {
            if repo.name == "core" {
                shell::run("npm run cleanbuild", &repo.abs_path, verbose)?;
            }
        }
        RepoType::Ts => {
            // Get local deps that need linking
            let deps = deps_map.get(&repo.name).cloned().unwrap_or_default();
            let local_deps: Vec<String> = deps
                .iter()
                .filter(|d| {
                    if !built_repos.contains(&d.as_str()) {
                        return false;
                    }
                    if let Some(local) = versions_cache.get_local(d)
                        && let Some(registry) = versions_cache.get_registry(d)
                    {
                        return local != registry;
                    }
                    false
                })
                .cloned()
                .collect();

            // Link local deps
            if !local_deps.is_empty() {
                if verbose {
                    println!("    Using local deps: {}", local_deps.join(", ").dimmed());
                }
                manifests::link_local_deps(config, &repo.name, &local_deps)?;
            }

            // Check if package has build script
            let has_build_script = repo
                .package_json
                .as_ref()
                .filter(|p| p.exists())
                .and_then(|p| std::fs::read_to_string(p).ok())
                .and_then(|content| serde_json::from_str::<serde_json::Value>(&content).ok())
                .and_then(|pkg| pkg.get("scripts")?.get("build").cloned())
                .is_some();

            let build_cmd = if has_build_script {
                "rm -rf node_modules dist && npm install && npm run build"
            } else {
                "rm -rf node_modules && npm install"
            };

            let result = shell::run(build_cmd, &repo.abs_path, verbose);

            // Restore repo
            manifests::restore_repo(config, versions_cache, &repo.name)?;

            result?;
        }
        RepoType::Website => {
            shell::git::pull(&repo.abs_path, "origin")?;
            shell::run(
                "rm -rf node_modules .svelte-kit && npm install && npm run build",
                &repo.abs_path,
                verbose,
            )?;
        }
        _ => {}
    }
    Ok(())
}

pub fn run(args: PrepArgs) -> Result<()> {
    let config = Config::load()?;
    let verbose = std::env::var("VERBOSE").is_ok() || std::env::var("DEBUG").is_ok();

    // Set up Ctrl+C handler with full rollback capability
    let interrupted = Arc::new(AtomicBool::new(false));
    let interrupted_clone = interrupted.clone();
    let config_clone = config.clone();
    let original_versions_for_handler = config.versions.clone();
    ctrlc::set_handler(move || {
        if interrupted_clone.swap(true, Ordering::SeqCst) {
            eprintln!("\n{} Force exiting...", "⚠".red());
            std::process::exit(130);
        }
        eprintln!("\n{} Received Ctrl+C, rolling back...", "⚠".yellow());
        // Restore versions.json
        if let Err(e) = original_versions_for_handler.save(&config_clone.root) {
            eprintln!("  {} Failed to restore versions.json: {}", "✗".red(), e);
        }
        // Re-apply original versions to all files
        if let Err(e) = manifests::apply_versions(&config_clone, &original_versions_for_handler) {
            eprintln!("  {} Failed to restore manifest files: {}", "✗".red(), e);
        }
        // Swap back to registry deps
        let _ = manifests::swap_registry(&config_clone.root, &original_versions_for_handler);
        eprintln!("{} Rollback complete. Exiting.", "✓".green());
        std::process::exit(130);
    })
    .context("Failed to set Ctrl+C handler")?;

    // Get initial repos
    let initial_repos = config.filter_repos(&args.repos);
    if initial_repos.is_empty() {
        anyhow::bail!("No repos matched filter: {}", args.repos);
    }

    // Track initial repos for conditional version bumping
    let initial_names: HashSet<String> = initial_repos.iter().map(|r| r.name.clone()).collect();

    // Cascade to dependents if needed
    let repo_names: Vec<String> = if args.no_cascade || args.repos == "all" {
        initial_repos.iter().map(|r| r.name.clone()).collect()
    } else {
        let cascaded = cascade_to_dependents(&initial_repos, &config.repos, &config.deps);
        if cascaded.len() > initial_repos.len() {
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
        }
        cascaded
    };

    // Get repos in topological order
    let sorted_names = deps::topo_order(&config.deps)?;
    let repos: Vec<&Repo> = sorted_names
        .iter()
        .filter(|name| repo_names.contains(name))
        .filter_map(|name| config.repos.get(name))
        .collect();

    if repos.is_empty() {
        anyhow::bail!("No repos to process");
    }

    // Save original versions for summary
    let original_versions: HashMap<String, String> = repos
        .iter()
        .filter_map(|r| {
            config
                .versions
                .get_local(&r.name)
                .map(|v| (r.name.clone(), v.to_string()))
        })
        .collect();

    // Calculate target version
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

    // Print header
    println!("\n{}", "=".repeat(60));
    if args.dry_run {
        println!("{}: Testing with current versions", "DRY RUN".yellow());
    } else {
        println!("{}", "Prepare Release".bold());
    }
    if args.sync_versions {
        println!("Mode: Sync (Increment Global)");
    } else {
        println!("Mode: Increment");
    }
    println!(
        "Repos: {}",
        repos.iter().map(|r| r.name.as_str()).collect::<Vec<_>>().join(", ").cyan()
    );
    println!("{}", "=".repeat(60));

    // Save original versions for rollback
    let original_versions_cache = config.versions.clone();

    // Rollback helper - restores original versions on failure
    let rollback = |config: &Config, original: &versions::VersionsCache| {
        eprintln!("\n{} Rolling back version changes...", "⚠".yellow());
        // Restore versions.json
        if let Err(e) = original.save(&config.root) {
            eprintln!("  {} Failed to restore versions.json: {}", "✗".red(), e);
        }
        // Re-apply original versions to all files
        if let Err(e) = manifests::apply_versions(config, original) {
            eprintln!("  {} Failed to restore manifest files: {}", "✗".red(), e);
        }
        // Swap back to registry deps
        let _ = manifests::swap_registry(&config.root, original);
        eprintln!("{} Rollback complete", "✓".green());
    };

    // [1/9] Bump versions
    let step = Step {
        number: 1.0,
        total: 9,
        label: "Bumping versions".to_string(),
    };
    step.print();

    let mut versions_cache = config.versions.clone();

    if !args.dry_run {
        for repo in &repos {
            // For cascaded dependents (not in initial_names), only bump if local == registry
            let is_cascaded = !initial_names.contains(&repo.name);
            let should_bump = if is_cascaded {
                let local = versions_cache.get_local(&repo.name);
                let registry = versions_cache.get_registry(&repo.name);
                // Only bump cascaded repos if they haven't been bumped yet (local == registry)
                local == registry
            } else {
                true // Always bump initial repos
            };

            let version = if should_bump {
                if args.sync_versions {
                    target_version.clone()
                } else {
                    args.version
                        .clone()
                        .unwrap_or_else(|| {
                            let current = versions_cache
                                .get_local(&repo.name)
                                .unwrap_or("0.1.0");
                            versions::increment_patch(current).unwrap_or("0.1.0".to_string())
                        })
                }
            } else {
                // Keep current version, but still update dependencies
                versions_cache
                    .get_local(&repo.name)
                    .unwrap_or("0.1.0")
                    .to_string()
            };

            if repo.package_json.is_some() || repo.cargo_toml.is_some() {
                manifests::set_version(&config, &mut versions_cache, &repo.name, &version)?;
            }
        }

        // Update zed extensions if processing all repos
        if args.repos == "all" {
            let _ = manifests::update_zed_extensions(&config.root, &versions_cache);
        }

        // Save the updated versions cache to disk
        versions_cache.save(&config.root)?;
    }

    // [2/9] Extract API documentation
    if !args.skip_docs && !args.bump_only {
        let step = Step {
            number: 2.0,
            total: 9,
            label: "Extracting API documentation".to_string(),
        };
        step.print();

        let docs_output = config.root.join("docs/api");
        let _ = extract_rust::run(&docs_output);
        let _ = extract_ts::run(&docs_output);
    } else {
        println!("\n{} Skipping API extraction", "[2/9]".dimmed());
    }

    // [3/9] Clean building packages
    if args.skip_build || args.bump_only || args.dry_run {
        println!("\n{} Skipping build/fmt/clippy/test...", "[3/9]".dimmed());
    } else {
        let step = Step {
            number: 3.0,
            total: 9,
            label: "Clean building packages".to_string(),
        };
        step.print();

        // Swap to local path dependencies
        println!("  {} Swapping to local path dependencies...", "→".blue());
        manifests::swap_local(&config.root)?;

        let mut built: Vec<&str> = Vec::new();
        for repo in &repos {
            if interrupted.load(Ordering::SeqCst) {
                break;
            }

            print!("  {} {}... ", "Building:".bold(), repo.name.cyan());
            io::stdout().flush()?;

            match build_repo(
                &config,
                repo,
                &built,
                &config.deps,
                &versions_cache,
                verbose,
            ) {
                Ok(_) => {
                    println!("{}", "done".green());
                    built.push(&repo.name);
                }
                Err(e) => {
                    println!("{}", "failed".red());
                    format::error(&format!("Build failed: {}", e));
                    rollback(&config, &original_versions_cache);
                    return Err(e);
                }
            }
        }

        // [4/9] Check formatting
        let rust_repos: Vec<_> = repos
            .iter()
            .filter(|r| r.repo_type == RepoType::Rust)
            .collect();

        if !rust_repos.is_empty() {
            let step = Step {
                number: 4.0,
                total: 9,
                label: "Checking formatting".to_string(),
            };
            step.print();

            for repo in &rust_repos {
                print!("  {} {}... ", "→".blue(), repo.name);
                io::stdout().flush()?;
                match shell::cargo::fmt_check(&repo.abs_path) {
                    Ok(_) => println!("{}", "ok".green()),
                    Err(e) => {
                        println!("{}", "failed".red());
                        rollback(&config, &original_versions_cache);
                        return Err(e);
                    }
                }
            }
        }

        // [5/9] Run clippy
        if !rust_repos.is_empty() {
            let step = Step {
                number: 5.0,
                total: 9,
                label: "Running clippy".to_string(),
            };
            step.print();

            for repo in &rust_repos {
                print!("  {} {}... ", "→".blue(), repo.name);
                io::stdout().flush()?;
                match shell::cargo::clippy(&repo.abs_path) {
                    Ok(_) => println!("{}", "ok".green()),
                    Err(e) => {
                        println!("{}", "failed".red());
                        rollback(&config, &original_versions_cache);
                        return Err(e);
                    }
                }
            }
        }

        // [6/9] Run tests
        if !rust_repos.is_empty() {
            let step = Step {
                number: 6.0,
                total: 9,
                label: "Running tests".to_string(),
            };
            step.print();

            for repo in &rust_repos {
                print!("  {} {}... ", "→".blue(), repo.name);
                io::stdout().flush()?;
                match shell::cargo::test(&repo.abs_path) {
                    Ok(_) => println!("{}", "passed".green()),
                    Err(e) => {
                        println!("{}", "failed".red());
                        rollback(&config, &original_versions_cache);
                        return Err(e);
                    }
                }
            }
        }

        // [6.5/9] Run biome checks
        let step = Step {
            number: 6.5,
            total: 9,
            label: "Running biome checks".to_string(),
        };
        step.print();

        let tooling_path = config.root.join("tooling");
        match shell::npx::biome_check(&tooling_path) {
            Ok(_) => println!("  {}", "ok".green()),
            Err(e) => {
                format::error(&format!("Biome check failed: {}", e));
                rollback(&config, &original_versions_cache);
                return Err(e);
            }
        }

        // Restore registry dependencies
        println!("  {} Restoring registry dependencies...", "→".blue());
        manifests::swap_registry(&config.root, &versions_cache)?;
    }

    // [7/9] Rebuild docs book
    if !args.skip_docs && !args.bump_only {
        let step = Step {
            number: 7.0,
            total: 9,
            label: "Rebuilding docs book".to_string(),
        };
        step.print();

        let book_output = config.root.join("docs/book.md");
        let _ = build_book::run(&book_output);
    } else {
        println!("\n{} Skipping docs book", "[7/9]".dimmed());
    }

    // [8/9] Sync MCP server docs
    if !args.skip_docs && !args.bump_only {
        let step = Step {
            number: 8.0,
            total: 9,
            label: "Syncing MCP server docs".to_string(),
        };
        step.print();

        let mcp_path = config.root.join("packages/mcp-server");
        let _ = shell::npm::run_script(&mcp_path, "build:docs");
    } else {
        println!("\n{} Skipping MCP docs", "[8/9]".dimmed());
    }

    // [9/9] Done
    let step = Step {
        number: 9.0,
        total: 9,
        label: "Done!".to_string(),
    };
    step.print();

    // Version summary
    println!("\n{}", "═".repeat(80));
    println!("{}", "Version Summary".bold());
    println!("{}", "═".repeat(80));

    // Reload final versions
    let final_versions = versions::VersionsCache::load(&config.root)?;

    // Published names mapping
    let pub_names: HashMap<&str, &str> = [
        ("core", "macroforge"),
        ("shared", "@macroforge/shared"),
        ("vite-plugin", "@macroforge/vite-plugin"),
        ("typescript-plugin", "@macroforge/typescript-plugin"),
        (
            "svelte-language-server",
            "@macroforge/svelte-language-server",
        ),
        ("svelte-preprocessor", "@macroforge/svelte-preprocessor"),
        ("mcp-server", "@macroforge/mcp-server"),
        ("syn", "macroforge_ts_syn"),
        ("template", "macroforge_ts_quote"),
        ("macros", "macroforge_ts_macros"),
        ("website", "macroforge.dev"),
        ("zed-extensions", "zed extensions"),
    ]
    .into_iter()
    .collect();

    let name_width = repos
        .iter()
        .map(|r| r.name.len())
        .max()
        .unwrap_or(8)
        .max(8);
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
            let after = final_versions
                .get_local(&repo.name)
                .unwrap_or("?");
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
    println!("{} Ready to commit!", "✓".green());
    println!("\n{}", "═".repeat(80));
    println!("\n{}", "Next step:".bold());
    println!(
        "  {}",
        format!("pixi run commit --repos {}", args.repos).cyan()
    );
    println!();

    Ok(())
}
