//! Commit queue for releasing packages in dependency order
//!
//! Uses dependency graph to ensure packages are committed/pushed in the correct
//! order. Handles git operations, tag creation, and registry polling.

use crate::cli::CommitArgs;
use crate::core::config::{self, Config};
use crate::core::deps;
use crate::core::registry;
use crate::core::repos::RepoType;
use crate::core::shell;
use crate::utils::format;
use anyhow::Result;
use colored::Colorize;
use dialoguer::{Confirm, Input};
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Represents a repository to be committed
#[derive(Debug, Clone)]
struct CommitItem {
    name: String,
    version: String,
    path: PathBuf,
    repo_type: RepoType,
    npm_name: Option<String>,
    crate_name: Option<String>,
    dependencies: Vec<String>,
}

pub fn run(args: CommitArgs) -> Result<()> {
    let config = Config::load()?;
    let versions = config.versions.clone();
    let mut repos = config.filter_repos(&args.repos);

    // Set up Ctrl+C handler
    let interrupted = Arc::new(AtomicBool::new(false));
    let interrupted_clone = Arc::clone(&interrupted);
    ctrlc::set_handler(move || {
        eprintln!("\n{} Interrupted by user", "⚠".yellow());
        interrupted_clone.store(true, Ordering::SeqCst);
        std::process::exit(130);
    })?;

    if repos.is_empty() {
        anyhow::bail!("No repos matched filter: {}", args.repos);
    }

    // Filter repos with cascading if not disabled
    if !args.no_cascade {
        // Cascade to dependents
        let initial_names: Vec<String> = repos.iter().map(|r| r.name.clone()).collect();
        let cascaded = cascade_to_dependents(&config.deps, &initial_names);
        repos = cascaded
            .into_iter()
            .filter_map(|name| config.repos.get(&name))
            .collect();
    }

    // Filter to only repos that have been prepped (local != registry)
    let prepped_repos: Vec<_> = repos
        .into_iter()
        .filter(|repo| is_prepped(&versions, &repo.name))
        .collect();

    if prepped_repos.is_empty() && !args.dry_run {
        format::warning("No repos have been prepped (all local versions match registry)");
        return Ok(());
    }

    format::header("Commit Queue");

    // Build commit queue in dependency order
    let order = deps::topo_order(&config.deps).unwrap_or_default();
    let npm_names = config::npm_package_names();
    let crate_names = config::crate_package_names();

    let mut queue: Vec<CommitItem> = Vec::new();

    for name in &order {
        if let Some(repo) = prepped_repos.iter().find(|r| r.name == *name) {
            let version = versions
                .get_local(&repo.name)
                .map(|s| s.to_string())
                .unwrap_or_else(|| "0.1.0".to_string());

            let dependencies = config
                .deps
                .get(name)
                .cloned()
                .unwrap_or_default();

            queue.push(CommitItem {
                name: repo.name.clone(),
                version,
                path: repo.abs_path.clone(),
                repo_type: repo.repo_type,
                npm_name: npm_names.get(repo.name.as_str()).map(|s| s.to_string()),
                crate_name: crate_names.get(repo.name.as_str()).map(|s| s.to_string()),
                dependencies,
            });
        }
    }

    // Add any repos not in the dependency graph
    for repo in &prepped_repos {
        if !queue.iter().any(|p| p.name == repo.name) {
            let version = versions
                .get_local(&repo.name)
                .map(|s| s.to_string())
                .unwrap_or_else(|| "0.1.0".to_string());

            let dependencies = config
                .deps
                .get(&repo.name)
                .cloned()
                .unwrap_or_default();

            queue.push(CommitItem {
                name: repo.name.clone(),
                version,
                path: repo.abs_path.clone(),
                repo_type: repo.repo_type,
                npm_name: npm_names.get(repo.name.as_str()).map(|s| s.to_string()),
                crate_name: crate_names.get(repo.name.as_str()).map(|s| s.to_string()),
                dependencies,
            });
        }
    }

    // Display queue
    println!("\n{}", "Commit queue (in dependency order):".bold());
    for (i, item) in queue.iter().enumerate() {
        let type_label = match item.repo_type {
            RepoType::Rust => "rust".yellow(),
            RepoType::Ts => "ts".cyan(),
            RepoType::Website => "web".magenta(),
            RepoType::Tooling => "tool".blue(),
            RepoType::Extension => "ext".white(),
        };

        let pkg_name = item
            .npm_name
            .as_ref()
            .or(item.crate_name.as_ref())
            .map(|s| s.as_str())
            .unwrap_or(&item.name);

        let deps_display = if item.dependencies.is_empty() {
            String::new()
        } else {
            format!(" {}", format!("(deps: {})", item.dependencies.join(", ")).dimmed())
        };

        println!(
            "  {}. {} {} @ {}{}",
            i + 1,
            type_label,
            pkg_name.bold(),
            item.version.green(),
            deps_display
        );
    }

    // Confirm
    if !args.yes && !args.dry_run {
        println!();
        if !Confirm::new()
            .with_prompt("Proceed with commits?")
            .default(false)
            .interact()?
        {
            format::warning("Aborted");
            return Ok(());
        }
    }

    // Track which packages have been committed (for dependency waiting)
    let mut committed_packages: HashMap<String, String> = HashMap::new();

    // Process queue
    for (i, item) in queue.iter().enumerate() {
        if interrupted.load(Ordering::SeqCst) {
            anyhow::bail!("Interrupted by user");
        }

        println!();
        format::step(i + 1, queue.len(), &format!("Processing {}", item.name));

        if !item.path.exists() {
            format::warning(&format!("Path does not exist: {:?}", item.path));
            continue;
        }

        // Show dependencies
        if !item.dependencies.is_empty() {
            let deps_with_status: Vec<String> = item
                .dependencies
                .iter()
                .map(|d| {
                    if committed_packages.contains_key(d) {
                        format!("{} {}", d.cyan(), "(waiting)".yellow())
                    } else {
                        format!("{} {}", d, "(ready)".dimmed())
                    }
                })
                .collect();
            println!("  {} Deps: {}", "◆".blue(), deps_with_status.join(", "));
        }

        // Wait for dependencies to appear on registries
        for dep_name in &item.dependencies {
            if let Some(dep_version) = committed_packages.get(dep_name) && !args.dry_run {
                wait_for_package(
                    dep_name,
                    dep_version,
                    &npm_names,
                    &crate_names,
                    &interrupted,
                )?;
            }
        }

        // Check for changes
        let status = shell::git::status(&item.path)?;
        let has_changes = !status.trim().is_empty();
        let unpushed_count = shell::git::unpushed_count(&item.path);

        if !has_changes && unpushed_count == 0 {
            println!("  {} No changes to commit", "→".dimmed());
            continue;
        }

        // Collect commit message
        let message = if let Some(ref msg) = args.message {
            msg.clone()
        } else if args.dry_run {
            default_commit_message(&item.name, &item.version, &status, has_package(item))
        } else {
            // Interactive prompt
            let default_msg = default_commit_message(&item.name, &item.version, &status, has_package(item));
            Input::new()
                .with_prompt(format!("Commit message for {}", item.name.cyan()))
                .default(default_msg)
                .interact_text()?
        };

        if args.dry_run {
            println!("  {} Would commit: {}", "→".blue(), message.dimmed());
            if has_changes {
                println!("  {} Would stage changes with git add -A", "→".blue());
            }
            if should_create_tag(item, &interrupted)? {
                println!("  {} Would create tag: v{}", "→".blue(), item.version.dimmed());
            }
            println!("  {} Would push to remote", "→".blue());
            continue;
        }

        // Sync lockfile for npm packages
        if item.path.join("package-lock.json").exists() {
            print!("  {} Syncing lockfile... ", "→".blue());
            io::stdout().flush()?;

            sync_lockfile(item, &item.dependencies, &versions, &committed_packages, &npm_names)?;
            println!("{}", "done".green());
        }

        // Stage changes
        if has_changes {
            print!("  {} Staging changes... ", "→".blue());
            io::stdout().flush()?;

            shell::git::add_all(&item.path)?;
            println!("{}", "done".green());
        }

        // Create commit
        print!("  {} Creating commit... ", "→".blue());
        io::stdout().flush()?;

        shell::git::commit(&item.path, &message)?;
        println!("{}", "done".green());

        // Create tag if needed
        if should_create_tag(item, &interrupted)? {
            let tag = format!("v{}", item.version);
            print!("  {} Creating tag {}... ", "→".blue(), tag.dimmed());
            io::stdout().flush()?;

            shell::git::tag(&item.path, &tag)?;
            println!("{}", "done".green());
        }

        // Push
        print!("  {} Pushing to remote... ", "→".blue());
        io::stdout().flush()?;

        // Push with upstream tracking if needed
        if shell::git::has_upstream(&item.path) {
            shell::git::push(&item.path)?;
        } else if let Some(branch) = shell::git::current_branch(&item.path) {
            shell::git::push_with_upstream(&item.path, &branch)?;
        } else {
            shell::git::push(&item.path)?;
        }
        shell::git::push_tags(&item.path)?;
        println!("{}", "done".green());

        // Mark as committed for dependency tracking
        committed_packages.insert(item.name.clone(), item.version.clone());

        println!("  {} {}", "✓".green(), "Complete".bold());
    }

    println!();
    format::success("All commits complete");

    if args.dry_run {
        format::warning("This was a dry run - no changes were made");
    }

    Ok(())
}

/// Check if a repo has been prepped (local version differs from registry)
fn is_prepped(versions: &crate::core::versions::VersionsCache, name: &str) -> bool {
    versions
        .versions
        .get(name)
        .and_then(|v| v.as_ref())
        .map(|v| v.local != v.registry)
        .unwrap_or(false)
}

/// Check if a repo has a publishable package
fn has_package(item: &CommitItem) -> bool {
    item.npm_name.is_some() || item.crate_name.is_some()
}

/// Generate default commit message
fn default_commit_message(_repo_name: &str, version: &str, status: &str, has_package: bool) -> String {
    if has_package {
        format!("Bump to {}", version)
    } else {
        // For repos without packages (tooling, website, zed-extensions),
        // show "Updated" with the list of changed files
        let files: Vec<&str> = status
            .lines()
            .filter_map(|line| {
                // Git status --short format: "XY filename"
                if line.len() > 3 {
                    Some(line[3..].trim())
                } else {
                    None
                }
            })
            .collect();

        if files.is_empty() {
            "Updated".to_string()
        } else if files.len() <= 3 {
            format!("Updated {}", files.join(", "))
        } else {
            format!("Updated {} files", files.len())
        }
    }
}

/// Cascade to dependent packages
fn cascade_to_dependents(deps: &HashMap<String, Vec<String>>, initial: &[String]) -> Vec<String> {
    let mut result = std::collections::HashSet::new();
    let mut to_process: Vec<String> = initial.to_vec();

    while let Some(repo) = to_process.pop() {
        if result.insert(repo.clone()) {
            // Add all dependents
            for dependent in deps::get_dependents(deps, &repo) {
                to_process.push(dependent);
            }
        }
    }

    // Return in topological order
    if let Ok(order) = deps::topo_order(deps) {
        order.into_iter().filter(|r| result.contains(r)).collect()
    } else {
        result.into_iter().collect()
    }
}


/// Determine if we should create a tag for this repo/version
/// For packages with registries: tag if version is NOT on the registry
/// For packages without registries: tag if tag doesn't exist remotely
fn should_create_tag(item: &CommitItem, _interrupted: &Arc<AtomicBool>) -> Result<bool> {
    // Check if version exists on registries
    if let Some(ref npm_name) = item.npm_name
        && let Ok(Some(v)) = registry::npm_version(npm_name)
        && v == item.version
    {
        // Already on npm, don't tag
        return Ok(false);
    }

    if let Some(ref crate_name) = item.crate_name
        && let Ok(Some(v)) = registry::crates_version(crate_name)
        && v == item.version
    {
        // Already on crates.io, don't tag
        return Ok(false);
    }

    // For packages with registries, tag if not on registry
    if item.npm_name.is_some() || item.crate_name.is_some() {
        return Ok(true);
    }

    // For packages without registries, check git tag
    let tag = format!("v{}", item.version);
    Ok(!shell::git::tag_exists_remote(&item.path, &tag))
}

/// Wait for a package to be available on the registry
fn wait_for_package(
    repo_name: &str,
    version: &str,
    npm_names: &HashMap<&str, &str>,
    crate_names: &HashMap<&str, &str>,
    interrupted: &Arc<AtomicBool>,
) -> Result<()> {
    let npm_name = npm_names.get(repo_name).copied();
    let crate_name = crate_names.get(repo_name).copied();

    // No registry to check
    if npm_name.is_none() && crate_name.is_none() {
        return Ok(());
    }

    // Wait for crates.io first if applicable (usually faster)
    if let Some(crate_name) = crate_name {
        wait_for_registry(
            crate_name,
            version,
            "crates.io",
            || registry::crates_version(crate_name).ok().flatten().as_deref() == Some(version),
            Duration::from_secs(600),
            Duration::from_secs(10),
            interrupted,
        )?;
    }

    // Wait for npm if applicable
    if let Some(npm_name) = npm_name {
        wait_for_registry(
            npm_name,
            version,
            "npm",
            || registry::npm_version(npm_name).ok().flatten().as_deref() == Some(version),
            Duration::from_secs(600),
            Duration::from_secs(10),
            interrupted,
        )?;
    }

    Ok(())
}

/// Wait for a specific registry
fn wait_for_registry<F>(
    package_name: &str,
    version: &str,
    registry: &str,
    check_fn: F,
    timeout: Duration,
    poll_interval: Duration,
    interrupted: &Arc<AtomicBool>,
) -> Result<()>
where
    F: Fn() -> bool,
{
    print!(
        "  {} Waiting for {}@{} on {}... ",
        "⏳".yellow(),
        package_name.cyan(),
        version.cyan(),
        registry
    );
    io::stdout().flush()?;

    let start = std::time::Instant::now();
    let mut dots = 0;

    loop {
        if interrupted.load(Ordering::SeqCst) {
            println!("{}", "interrupted".yellow());
            anyhow::bail!("Interrupted by user");
        }

        if check_fn() {
            println!("{}{}", ".".repeat(dots), "available!".green());
            return Ok(());
        }

        if start.elapsed() > timeout {
            println!("{}{}", ".".repeat(dots), "timeout".red());
            anyhow::bail!(
                "Timeout waiting for {}@{} on {}",
                package_name,
                version,
                registry
            );
        }

        dots += 1;
        print!(".");
        io::stdout().flush()?;
        std::thread::sleep(poll_interval);
    }
}

/// Sync npm lockfile after dependencies are available
fn sync_lockfile(
    item: &CommitItem,
    dependencies: &[String],
    versions: &crate::core::versions::VersionsCache,
    committed_packages: &HashMap<String, String>,
    npm_names: &HashMap<&str, &str>,
) -> Result<()> {
    let mut packages: Vec<String> = Vec::new();

    // Force install registry versions of ALL dependencies to break local links
    for dep_repo in dependencies {
        if let Some(pkg_name) = npm_names.get(dep_repo.as_str()) {
            // If we just committed it, use local (new) version
            // Otherwise, use registry (stable) version
            let target_ver = if committed_packages.contains_key(dep_repo) {
                versions.get_local(dep_repo).unwrap_or("latest")
            } else {
                versions.get_registry(dep_repo).unwrap_or("latest")
            };
            packages.push(format!("{}@{}", pkg_name, target_ver));
        }
    }

    let cmd = if packages.is_empty() {
        "npm install --ignore-scripts --no-audit".to_string()
    } else {
        format!("npm install --ignore-scripts --no-audit {}", packages.join(" "))
    };

    shell::run(&cmd, &item.path, false)?;
    Ok(())
}
