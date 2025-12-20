#!/usr/bin/env rust-script
//! Commit queue for releasing packages in dependency order.
//!
//! Uses dependency graph to ensure packages are committed/pushed in the correct
//! order. If any commit fails, remaining repos are unstaged and the process aborts.
//!
//! ```cargo
//! [dependencies]
//! clap = { version = "4", features = ["derive"] }
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! toml = "0.8"
//! colored = "2"
//! dialoguer = "0.11"
//! petgraph = "0.6"
//! ctrlc = "3.4"
//! ```

use clap::Parser;
use colored::*;
use dialoguer::{Confirm, Input};
use petgraph::algo::toposort;
use petgraph::graph::DiGraph;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

// ============================================================================
// CLI
// ============================================================================

#[derive(Parser, Debug)]
#[command(name = "commit")]
#[command(about = "Commit queue for releasing packages in dependency order")]
struct Args {
    /// Repos to commit (comma-separated, or 'all', 'rust', 'ts')
    #[arg(short, long, default_value = "all")]
    repos: String,

    /// Skip confirmation prompts
    #[arg(short = 'y', long)]
    yes: bool,

    /// Dry run - show what would be done without doing it
    #[arg(long)]
    dry_run: bool,

    /// Custom commit message (will prompt per-repo if not provided)
    #[arg(short, long)]
    message: Option<String>,

    /// Don't cascade to dependent packages
    #[arg(long)]
    no_cascade: bool,

    /// Internal flag: indicates script is running in spawned terminal
    #[arg(long, hide = true)]
    spawned: bool,

    /// Enable debug logging
    #[arg(long)]
    debug: bool,
}

// ============================================================================
// Repository Configuration
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
enum RepoType {
    Rust,
    Ts,
    Website,
    Tooling,
    Extension,
}

#[derive(Debug, Clone)]
struct Repo {
    name: &'static str,
    path: &'static str,
    repo_type: RepoType,
}

fn all_repos() -> Vec<Repo> {
    vec![
        Repo {
            name: "core",
            path: "crates/macroforge_ts",
            repo_type: RepoType::Rust,
        },
        Repo {
            name: "macros",
            path: "crates/macroforge_ts_macros",
            repo_type: RepoType::Rust,
        },
        Repo {
            name: "syn",
            path: "crates/macroforge_ts_syn",
            repo_type: RepoType::Rust,
        },
        Repo {
            name: "template",
            path: "crates/macroforge_ts_quote",
            repo_type: RepoType::Rust,
        },
        Repo {
            name: "shared",
            path: "packages/shared",
            repo_type: RepoType::Ts,
        },
        Repo {
            name: "vite-plugin",
            path: "packages/vite-plugin",
            repo_type: RepoType::Ts,
        },
        Repo {
            name: "typescript-plugin",
            path: "packages/typescript-plugin",
            repo_type: RepoType::Ts,
        },
        Repo {
            name: "svelte-language-server",
            path: "packages/svelte-language-server",
            repo_type: RepoType::Ts,
        },
        Repo {
            name: "svelte-preprocessor",
            path: "packages/svelte-preprocessor",
            repo_type: RepoType::Ts,
        },
        Repo {
            name: "mcp-server",
            path: "packages/mcp-server",
            repo_type: RepoType::Ts,
        },
        Repo {
            name: "website",
            path: "website",
            repo_type: RepoType::Website,
        },
        Repo {
            name: "tooling",
            path: "tooling",
            repo_type: RepoType::Tooling,
        },
        Repo {
            name: "zed-extensions",
            path: "crates/extensions",
            repo_type: RepoType::Extension,
        },
    ]
}

// ============================================================================
// Dependency Graph
// ============================================================================

/// Load dependency map from tooling/deps.toml
fn load_dependency_map(root: &Path) -> HashMap<String, Vec<String>> {
    let deps_path = root.join("tooling/deps.toml");

    #[derive(Deserialize)]
    struct DepsFile {
        #[serde(default)]
        crates: HashMap<String, Vec<String>>,
        #[serde(default)]
        packages: HashMap<String, Vec<String>>,
        #[serde(default)]
        other: HashMap<String, Vec<String>>,
    }

    if let Ok(content) = fs::read_to_string(&deps_path) {
        if let Ok(deps_file) = toml::from_str::<DepsFile>(&content) {
            let mut result = HashMap::new();
            result.extend(deps_file.crates);
            result.extend(deps_file.packages);
            result.extend(deps_file.other);
            return result;
        }
    }

    // Fallback to empty map if file doesn't exist or parse fails
    eprintln!(
        "{} Could not load tooling/deps.toml, using empty dependency map",
        "Warning:".yellow()
    );
    HashMap::new()
}

/// Map repo names to their npm package names
fn npm_package_names() -> HashMap<&'static str, &'static str> {
    [
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
    ]
    .into_iter()
    .collect()
}

/// Map repo names to their crates.io crate names
fn crate_names() -> HashMap<&'static str, &'static str> {
    [
        ("syn", "macroforge_ts_syn"),
        ("template", "macroforge_ts_quote"),
        ("macros", "macroforge_ts_macros"),
        ("core", "macroforge_ts"), // dual-published: also on npm as "macroforge"
    ]
    .into_iter()
    .collect()
}

/// Check if a repo has a publishable package (npm or crate)
fn has_package(repo_name: &str) -> bool {
    npm_package_names().contains_key(repo_name) || crate_names().contains_key(repo_name)
}

/// Generate a default commit message for a repo
fn default_commit_message(repo_name: &str, version: &str, status: &str) -> String {
    if has_package(repo_name) {
        format!("Bump to {}", version)
    } else {
        // For repos without packages (tooling, website, zed-extensions),
        // show "Updated" with the list of changed files
        let files: Vec<&str> = status
            .lines()
            .filter_map(|line| {
                // Git status --short format: "XY filename" where XY is status code + space
                // Example: " M scripts/commit.rs" or "?? newfile.txt"
                // The filename starts at position 3 (after XY and space)
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

struct DependencyGraph {
    graph: DiGraph<String, ()>,
    deps: HashMap<String, Vec<String>>,
}

impl DependencyGraph {
    fn new(repos: &[Repo], deps: HashMap<String, Vec<String>>) -> Self {
        let mut graph = DiGraph::new();
        let mut node_indices = HashMap::new();

        for repo in repos {
            let idx = graph.add_node(repo.name.to_string());
            node_indices.insert(repo.name.to_string(), idx);
        }

        for repo in repos {
            if let Some(dependencies) = deps.get(repo.name) {
                let dependent_idx = node_indices[repo.name];
                for dep in dependencies {
                    if let Some(&dep_idx) = node_indices.get(dep) {
                        graph.add_edge(dep_idx, dependent_idx, ());
                    }
                }
            }
        }

        Self { graph, deps }
    }

    fn sorted_order(&self) -> Result<Vec<String>, String> {
        match toposort(&self.graph, None) {
            Ok(indices) => Ok(indices
                .into_iter()
                .map(|idx| self.graph[idx].clone())
                .collect()),
            Err(cycle) => Err(format!(
                "Dependency cycle detected involving: {}",
                self.graph[cycle.node_id()]
            )),
        }
    }

    fn get_dependents(&self, repo_name: &str) -> Vec<String> {
        self.deps
            .iter()
            .filter(|(_, dependencies)| dependencies.iter().any(|d| d == repo_name))
            .map(|(dependent, _)| dependent.clone())
            .collect()
    }

    fn cascade_to_dependents(&self, repo_names: &[&str]) -> Vec<String> {
        let mut result: HashSet<String> = repo_names.iter().map(|s| s.to_string()).collect();
        let mut to_process: Vec<String> = repo_names.iter().map(|s| s.to_string()).collect();

        while let Some(repo) = to_process.pop() {
            for dependent in self.get_dependents(&repo) {
                if result.insert(dependent.clone()) {
                    to_process.push(dependent);
                }
            }
        }

        match self.sorted_order() {
            Ok(sorted) => sorted.into_iter().filter(|r| result.contains(r)).collect(),
            Err(_) => result.into_iter().collect(),
        }
    }
}

// ============================================================================
// Version Cache
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct VersionInfo {
    local: String,
    registry: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct VersionsCache(HashMap<String, Option<VersionInfo>>);

impl VersionsCache {
    fn load(root: &Path) -> Self {
        let path = root.join("tooling/versions.json");
        if path.exists() {
            let content = fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    fn get(&self, name: &str) -> Option<&VersionInfo> {
        self.0.get(name).and_then(|v| v.as_ref())
    }

    fn get_local(&self, name: &str) -> Option<&String> {
        self.get(name).map(|v| &v.local)
    }

    /// Check if a repo has been prepped (local version differs from registry)
    fn is_prepped(&self, name: &str) -> bool {
        self.get(name)
            .map(|v| v.local != v.registry)
            .unwrap_or(false)
    }
}

// ============================================================================
// Git Operations
// ============================================================================

fn run_git(args: &[&str], cwd: &Path) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .map_err(|e| format!("Failed to run git: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(stdout)
    } else {
        Err(format!("{}\n{}", stdout, stderr).trim().to_string())
    }
}

fn get_status(cwd: &Path) -> Result<String, String> {
    run_git(&["status", "--short"], cwd)
}

fn has_changes(cwd: &Path) -> bool {
    get_status(cwd)
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false)
}

fn tag_exists_locally(tag: &str, cwd: &Path) -> bool {
    run_git(&["tag", "-l", tag], cwd)
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false)
}

fn tag_exists_remotely(tag: &str, cwd: &Path) -> bool {
    run_git(&["ls-remote", "--tags", "origin", tag], cwd)
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false)
}

fn get_unpushed_count(cwd: &Path) -> usize {
    run_git(&["rev-list", "@{u}..HEAD", "--count"], cwd)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}

fn get_current_branch(cwd: &Path) -> Option<String> {
    run_git(&["rev-parse", "--abbrev-ref", "HEAD"], cwd)
        .ok()
        .map(|s| s.trim().to_string())
}

fn has_upstream(cwd: &Path) -> bool {
    run_git(&["rev-parse", "--abbrev-ref", "@{u}"], cwd).is_ok()
}

/// Push to remote, setting upstream if needed
fn push_with_upstream(cwd: &Path) -> Result<String, String> {
    if has_upstream(cwd) {
        run_git(&["push"], cwd)
    } else {
        // No upstream set - push with -u to set it
        let branch = get_current_branch(cwd).unwrap_or_else(|| "main".to_string());
        run_git(&["push", "-u", "origin", &branch], cwd)
    }
}

// ============================================================================
// Registry Polling
// ============================================================================

/// Check if a version is available on npm
fn check_npm_version(package: &str, version: &str) -> bool {
    let output = Command::new("npm")
        .args(["view", &format!("{}@{}", package, version), "version"])
        .output();

    match output {
        Ok(o) => o.status.success() && String::from_utf8_lossy(&o.stdout).trim() == version,
        Err(_) => false,
    }
}

/// Check if a version is available on crates.io
fn check_crate_version(crate_name: &str, version: &str) -> bool {
    let output = Command::new("cargo")
        .args(["search", crate_name, "--limit", "1"])
        .output();

    match output {
        Ok(o) if o.status.success() => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            // Output format: "crate_name = \"version\""
            stdout.contains(&format!("\"{}\"", version))
        }
        _ => false,
    }
}

/// Determine if we should create a tag for this repo/version.
/// For packages with registries: tag if version is NOT on the registry (registry is source of truth)
/// For packages without registries: tag if tag doesn't exist remotely
fn should_create_tag(repo_name: &str, version: &str, repo_path: &Path) -> bool {
    let npm_packages = npm_package_names();
    let crates = crate_names();

    let npm_name = npm_packages.get(repo_name).copied();
    let crate_name = crates.get(repo_name).copied();

    // If package has a registry, check if version exists there
    // Registry is the source of truth - stale git tags should be overwritten
    if let Some(npm_pkg) = npm_name {
        // If version is NOT on npm, we should tag
        if !check_npm_version(npm_pkg, version) {
            return true;
        }
    }

    if let Some(crate_pkg) = crate_name {
        // If version is NOT on crates.io, we should tag
        if !check_crate_version(crate_pkg, version) {
            return true;
        }
    }

    // For packages without registries (tooling, website, zed-extensions),
    // or if version already exists on all registries, check git tag
    if npm_name.is_none() && crate_name.is_none() {
        // No registry - use git tag as source of truth
        return !tag_exists_remotely(&format!("v{}", version), repo_path);
    }

    // Version exists on all applicable registries - don't retag
    false
}

/// Wait for a package to be available on the registry (or both for dual-published)
/// If output_buffer is Some, logs to buffer instead of stdout
fn wait_for_package(
    repo_name: &str,
    version: &str,
    timeout_secs: u64,
    interrupted: &Arc<AtomicBool>,
    output_buffer: Option<&Arc<Mutex<Vec<String>>>>,
) -> Result<(), String> {
    let npm_packages = npm_package_names();
    let crates = crate_names();

    let npm_name = npm_packages.get(repo_name).copied();
    let crate_name = crates.get(repo_name).copied();

    // No registry to check (e.g., website, tooling, zed-extensions)
    if npm_name.is_none() && crate_name.is_none() {
        return Ok(());
    }

    // Wait for crates.io first if applicable (usually faster)
    if let Some(crate_name) = crate_name {
        wait_for_registry(
            crate_name,
            version,
            "crates.io",
            || check_crate_version(crate_name, version),
            timeout_secs,
            interrupted,
            output_buffer,
        )?;
    }

    // Wait for npm if applicable
    if let Some(npm_name) = npm_name {
        wait_for_registry(
            npm_name,
            version,
            "npm",
            || check_npm_version(npm_name, version),
            timeout_secs,
            interrupted,
            output_buffer,
        )?;
    }

    Ok(())
}

/// Wait for a specific registry
/// If output_buffer is Some, logs to buffer instead of stdout (for background processing)
fn wait_for_registry<F: Fn() -> bool>(
    package_name: &str,
    version: &str,
    registry: &str,
    check_fn: F,
    timeout_secs: u64,
    interrupted: &Arc<AtomicBool>,
    output_buffer: Option<&Arc<Mutex<Vec<String>>>>,
) -> Result<(), String> {
    let start = std::time::Instant::now();
    let poll_interval = std::time::Duration::from_secs(10);
    let timeout = std::time::Duration::from_secs(timeout_secs);

    let log_start = format!(
        "  {} Waiting for {}@{} on {}... ",
        "⏳".yellow(),
        package_name.cyan(),
        version.cyan(),
        registry
    );

    // If buffering, just log start; otherwise print with flush for live updates
    if let Some(buf) = &output_buffer {
        buf.lock().unwrap().push(log_start);
    } else {
        print!("{}", log_start);
        io::stdout().flush().ok();
    }

    let mut dots = 0;
    loop {
        // Check for Ctrl+C
        if interrupted.load(Ordering::SeqCst) {
            let msg = format!("{}", "interrupted".yellow());
            if let Some(buf) = &output_buffer {
                buf.lock().unwrap().push(msg);
            } else {
                println!("{}", msg);
            }
            return Err("Interrupted by user".to_string());
        }

        if check_fn() {
            let msg = format!("{}{}", ".".repeat(dots), "available!".green());
            if let Some(buf) = &output_buffer {
                buf.lock().unwrap().push(msg);
            } else {
                println!("{}", msg);
            }
            return Ok(());
        }

        if start.elapsed() > timeout {
            let msg = format!("{}{}", ".".repeat(dots), "timeout".red());
            if let Some(buf) = &output_buffer {
                buf.lock().unwrap().push(msg);
            } else {
                println!("{}", msg);
            }
            return Err(format!(
                "Timeout waiting for {}@{} on {}",
                package_name, version, registry
            ));
        }

        dots += 1;
        if output_buffer.is_none() {
            print!(".");
            io::stdout().flush().ok();
        }
        std::thread::sleep(poll_interval);
    }
}

// ============================================================================
// Commit Queue
// ============================================================================

struct CommitQueue {
    root: PathBuf,
    queue: Vec<QueueItem>,
    completed: Vec<String>,
    /// Dependency map loaded from deps.toml
    deps: HashMap<String, Vec<String>>,
    /// Version map for all repos
    versions: HashMap<String, Option<VersionInfo>>,
    dry_run: bool,
    /// Timeout for waiting on registry (seconds)
    registry_timeout: u64,
    /// Flag to track if Ctrl+C was pressed
    interrupted: Arc<AtomicBool>,
    /// Debug logging enabled
    debug: bool,
}

#[derive(Clone)]
struct QueueItem {
    repo: Repo,
    version: String,
    message: Option<String>,
}

impl CommitQueue {
    fn new(
        root: PathBuf,
        deps: HashMap<String, Vec<String>>,
        versions: HashMap<String, Option<VersionInfo>>,
        dry_run: bool,
        interrupted: Arc<AtomicBool>,
        debug: bool,
    ) -> Self {
        Self {
            root,
            queue: Vec::new(),
            completed: Vec::new(),
            deps,
            versions,
            dry_run,
            registry_timeout: 600, // 10 minutes default
            interrupted,
            debug,
        }
    }

    fn add(&mut self, repo: Repo, version: String, message: Option<String>) {
        self.queue.push(QueueItem {
            repo,
            version,
            message,
        });
    }

    /// Process with background worker - collect messages while processing happens
    fn process(&mut self, auto_yes: bool, global_message: Option<&str>) -> Result<(), String> {
        // Find repos that have changes
        let repos_with_changes: Vec<usize> = self
            .queue
            .iter()
            .enumerate()
            .filter(|(_, item)| {
                let repo_path = self.root.join(item.repo.path);
                repo_path.exists()
                    && (has_changes(&repo_path) || get_unpushed_count(&repo_path) > 0)
            })
            .map(|(idx, _)| idx)
            .collect();

        if repos_with_changes.is_empty() && !self.dry_run {
            println!("{}", "No repos with changes".yellow());
            return Ok(());
        }

        // Shared state for worker thread
        let messages: Arc<Mutex<HashMap<usize, String>>> = Arc::new(Mutex::new(HashMap::new()));
        let output_buffer: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let completed_shared: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let worker_done = Arc::new(AtomicBool::new(false));
        let all_messages_collected = Arc::new(AtomicBool::new(false));
        let worker_error: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

        // Clone data for worker thread
        let worker_queue: Vec<_> = self.queue.iter().cloned().collect();
        let worker_root = self.root.clone();
        let worker_deps = self.deps.clone();
        let worker_versions = self.versions.clone();
        let worker_messages = Arc::clone(&messages);
        let worker_output = Arc::clone(&output_buffer);
        let worker_completed = Arc::clone(&completed_shared);
        let worker_done_flag = Arc::clone(&worker_done);
        let worker_all_messages = Arc::clone(&all_messages_collected);
        let worker_interrupted = Arc::clone(&self.interrupted);
        let worker_error_slot = Arc::clone(&worker_error);
        let registry_timeout = self.registry_timeout;
        let dry_run = self.dry_run;
        let debug = self.debug;

        // Spawn worker thread
        let worker_handle = thread::spawn(move || {
            let mut pushed_packages: HashMap<String, String> = HashMap::new();
            let total = worker_queue.len();

            let log = |output: &Arc<Mutex<Vec<String>>>, msg: String| {
                output.lock().unwrap().push(msg);
            };

            for (idx, item) in worker_queue.iter().enumerate() {
                if worker_interrupted.load(Ordering::SeqCst) {
                    log(
                        &worker_output,
                        format!("\n{} Interrupted by user", "⚠".yellow()),
                    );
                    break;
                }

                let repo_path = worker_root.join(item.repo.path);
                let position = format!("[{}/{}]", idx + 1, total);

                log(
                    &worker_output,
                    format!(
                        "\n{} {} {}",
                        position.dimmed(),
                        "Processing".bold(),
                        item.repo.name.cyan()
                    ),
                );
                log(&worker_output, format!("{}", "─".repeat(50).dimmed()));

                if !repo_path.exists() {
                    log(
                        &worker_output,
                        format!("  {} Path does not exist, skipping", "⚠".yellow()),
                    );
                    continue;
                }

                // Show and wait for dependencies
                let all_deps = worker_deps.get(item.repo.name).cloned().unwrap_or_default();
                let pending_deps: Vec<(String, String)> = all_deps
                    .iter()
                    .filter_map(|dep| pushed_packages.get(dep).map(|v| (dep.clone(), v.clone())))
                    .collect();

                // Always show dependencies
                if !all_deps.is_empty() {
                    let deps_display: Vec<String> = all_deps
                        .iter()
                        .map(|d| {
                            if pushed_packages.contains_key(d) {
                                format!("{} {}", d.cyan(), "(waiting)".yellow())
                            } else {
                                format!("{} {}", d, "(ready)".dimmed())
                            }
                        })
                        .collect();
                    log(
                        &worker_output,
                        format!("  {} Deps: {}", "◆".blue(), deps_display.join(", ")),
                    );
                }

                if debug {
                    log(
                        &worker_output,
                        format!(
                            "  {} pushed_packages: {:?}",
                            "[DEBUG]".magenta(),
                            pushed_packages
                        ),
                    );
                    log(
                        &worker_output,
                        format!("  {} pending_deps: {:?}", "[DEBUG]".magenta(), pending_deps),
                    );
                }

                if !pending_deps.is_empty() && !dry_run {
                    for (dep_name, dep_version) in &pending_deps {
                        // Pass output buffer so wait messages don't interfere with message collection
                        if let Err(e) = wait_for_package(
                            dep_name,
                            dep_version,
                            registry_timeout,
                            &worker_interrupted,
                            Some(&worker_output),
                        ) {
                            *worker_error_slot.lock().unwrap() = Some(e);
                            worker_done_flag.store(true, Ordering::SeqCst);
                            return;
                        }
                        // Don't remove - other packages may also depend on this
                    }
                }

                // Sync lockfile now that dependencies are available
                if !dry_run && repo_path.join("package-lock.json").exists() {
                    log(
                        &worker_output,
                        format!("  {} Verifying lockfile... ", "→".blue()),
                    );

                    let npm_names = npm_package_names();
                    let mut install_args = vec![
                        "install".to_string(),
                        "--ignore-scripts".to_string(),
                        "--no-audit".to_string(),
                    ];

                    // Force install registry versions of ALL dependencies to break local links
                    for dep_repo in &all_deps {
                        if let Some(pkg_name) = npm_names.get(dep_repo.as_str()) {
                            // CRITICAL: Determine which version to install
                            // 1. If it's a pending dep (we just pushed it), use 'local' (new) version
                            // 2. Otherwise, use 'registry' (stable) version to ensure we don't depend on unreleased code
                            if let Some(info) =
                                worker_versions.get(dep_repo).and_then(|v| v.as_ref())
                            {
                                let target_ver = if pushed_packages.contains_key(dep_repo) {
                                    &info.local
                                } else {
                                    &info.registry
                                };
                                install_args.push(format!("{}@{}", pkg_name, target_ver));
                            }
                        }
                    }

                    // Run npm install (either general or specific packages)
                    let status = Command::new("npm")
                        .args(&install_args)
                        .current_dir(&repo_path)
                        .output();

                    match status {
                        Ok(output) => {
                            if output.status.success() {
                                log(&worker_output, format!("{}", "✓".green()));
                            } else {
                                log(&worker_output, format!("{}", "✗".red()));
                                let stderr = String::from_utf8_lossy(&output.stderr);
                                *worker_error_slot.lock().unwrap() = Some(format!(
                                    "npm install failed for {}: {}",
                                    item.repo.name, stderr
                                ));
                                worker_done_flag.store(true, Ordering::SeqCst);
                                return;
                            }
                        }
                        Err(e) => {
                            log(&worker_output, format!("{}", "✗".red()));
                            *worker_error_slot.lock().unwrap() = Some(format!(
                                "Failed to execute npm install for {}: {}",
                                item.repo.name, e
                            ));
                            worker_done_flag.store(true, Ordering::SeqCst);
                            return;
                        }
                    }
                }

                // Wait for message to be entered
                let commit_msg = loop {
                    if worker_interrupted.load(Ordering::SeqCst) {
                        worker_done_flag.store(true, Ordering::SeqCst);
                        return;
                    }
                    if let Some(msg) = worker_messages.lock().unwrap().get(&idx).cloned() {
                        break msg;
                    }
                    // If all messages collected and we don't have one, use default
                    if worker_all_messages.load(Ordering::SeqCst) {
                        let status = get_status(&repo_path).unwrap_or_default();
                        break default_commit_message(item.repo.name, &item.version, &status);
                    }
                    thread::sleep(std::time::Duration::from_millis(100));
                };

                // Check for changes
                let has_local_changes = has_changes(&repo_path);
                let unpushed = get_unpushed_count(&repo_path);

                if !has_local_changes && unpushed == 0 {
                    log(
                        &worker_output,
                        format!("  {} No changes, skipping", "ℹ".blue()),
                    );
                    continue;
                }

                if !has_local_changes && unpushed > 0 {
                    log(
                        &worker_output,
                        format!(
                            "  {} {} unpushed commit{}",
                            "ℹ".blue(),
                            unpushed.to_string().yellow(),
                            if unpushed == 1 { "" } else { "s" }
                        ),
                    );

                    // Tag if needed (even for unpushed-only case)
                    // Use registry as source of truth - if version not on registry, we should tag
                    let tag = format!("v{}", item.version);
                    let should_tag = should_create_tag(item.repo.name, &item.version, &repo_path);
                    let tag_exists_remote = tag_exists_remotely(&tag, &repo_path);

                    if dry_run {
                        log(
                            &worker_output,
                            format!(
                                "  {} Would: push{}",
                                "[dry-run]".yellow(),
                                if should_tag {
                                    format!(
                                        ", tag {}{}",
                                        tag,
                                        if tag_exists_remote { " (force)" } else { "" }
                                    )
                                } else {
                                    String::new()
                                }
                            ),
                        );
                        worker_completed
                            .lock()
                            .unwrap()
                            .push(item.repo.name.to_string());
                        continue;
                    }

                    // Create tag if needed
                    if should_tag {
                        if tag_exists_locally(&tag, &repo_path) {
                            let _ = run_git(&["tag", "-d", &tag], &repo_path);
                        }
                        log(
                            &worker_output,
                            format!("  {} Tagging {}... ", "→".blue(), tag.cyan()),
                        );
                        if let Err(e) = run_git(&["tag", &tag], &repo_path) {
                            *worker_error_slot.lock().unwrap() =
                                Some(format!("Tag failed for {}: {}", item.repo.name, e));
                            worker_done_flag.store(true, Ordering::SeqCst);
                            return;
                        }
                        log(&worker_output, format!("{}", "✓".green()));
                    }

                    // Push commits
                    log(&worker_output, format!("  {} Pushing... ", "→".blue()));
                    match push_with_upstream(&repo_path) {
                        Ok(_) => {
                            log(&worker_output, format!("{}", "✓".green()));
                        }
                        Err(e) => {
                            log(&worker_output, format!("{}", "✗".red()));
                            *worker_error_slot.lock().unwrap() =
                                Some(format!("Push failed for {}: {}", item.repo.name, e));
                            worker_done_flag.store(true, Ordering::SeqCst);
                            return;
                        }
                    }

                    // Push tags (force if tag already exists remotely but we need to update it)
                    if should_tag {
                        log(&worker_output, format!("  {} Pushing tags... ", "→".blue()));
                        let push_args = if tag_exists_remote {
                            vec!["push", "origin", &tag, "--force"]
                        } else {
                            vec!["push", "origin", &tag]
                        };
                        if let Err(e) = run_git(&push_args, &repo_path) {
                            log(&worker_output, format!("{}", "✗".red()));
                            log(&worker_output, format!("    {} {}", "Warning:".yellow(), e));
                        } else {
                            log(&worker_output, format!("{}", "✓".green()));
                        }
                    }

                    // Track pushed package
                    let npm_packages = npm_package_names();
                    let crates = crate_names();
                    if npm_packages.contains_key(item.repo.name)
                        || crates.contains_key(item.repo.name)
                    {
                        pushed_packages.insert(item.repo.name.to_string(), item.version.clone());
                    }

                    worker_completed
                        .lock()
                        .unwrap()
                        .push(item.repo.name.to_string());
                    continue;
                }

                log(
                    &worker_output,
                    format!("  {} Message: {}", "ℹ".blue(), commit_msg.dimmed()),
                );

                if dry_run {
                    let tag = format!("v{}", item.version);
                    let would_tag = should_create_tag(item.repo.name, &item.version, &repo_path);
                    let tag_exists_remote = tag_exists_remotely(&tag, &repo_path);
                    log(
                        &worker_output,
                        format!(
                            "  {} Would: add, commit, {}",
                            "[dry-run]".yellow(),
                            if would_tag {
                                format!(
                                    "tag {}{}, push",
                                    tag,
                                    if tag_exists_remote { " (force)" } else { "" }
                                )
                            } else {
                                "push".to_string()
                            }
                        ),
                    );
                    worker_completed
                        .lock()
                        .unwrap()
                        .push(item.repo.name.to_string());
                    continue;
                }

                // Stage
                log(&worker_output, format!("  {} Staging... ", "→".blue()));
                if let Err(e) = run_git(&["add", "-A"], &repo_path) {
                    *worker_error_slot.lock().unwrap() =
                        Some(format!("Stage failed for {}: {}", item.repo.name, e));
                    worker_done_flag.store(true, Ordering::SeqCst);
                    return;
                }
                log(&worker_output, format!("{}", "✓".green()));

                // Commit
                log(&worker_output, format!("  {} Committing... ", "→".blue()));
                if let Err(e) = run_git(&["commit", "-m", &commit_msg], &repo_path) {
                    *worker_error_slot.lock().unwrap() =
                        Some(format!("Commit failed for {}: {}", item.repo.name, e));
                    worker_done_flag.store(true, Ordering::SeqCst);
                    return;
                }
                log(&worker_output, format!("{}", "✓".green()));

                // Tag (use registry as source of truth)
                let tag = format!("v{}", item.version);
                let should_tag = should_create_tag(item.repo.name, &item.version, &repo_path);
                let tag_exists_remote = tag_exists_remotely(&tag, &repo_path);
                if should_tag {
                    if tag_exists_locally(&tag, &repo_path) {
                        let _ = run_git(&["tag", "-d", &tag], &repo_path);
                    }
                    log(
                        &worker_output,
                        format!("  {} Tagging {}... ", "→".blue(), tag.cyan()),
                    );
                    if let Err(e) = run_git(&["tag", &tag], &repo_path) {
                        *worker_error_slot.lock().unwrap() =
                            Some(format!("Tag failed for {}: {}", item.repo.name, e));
                        worker_done_flag.store(true, Ordering::SeqCst);
                        return;
                    }
                    log(&worker_output, format!("{}", "✓".green()));
                }

                // Push
                log(&worker_output, format!("  {} Pushing... ", "→".blue()));
                if let Err(e) = push_with_upstream(&repo_path) {
                    *worker_error_slot.lock().unwrap() =
                        Some(format!("Push failed for {}: {}", item.repo.name, e));
                    worker_done_flag.store(true, Ordering::SeqCst);
                    return;
                }
                log(&worker_output, format!("{}", "✓".green()));

                // Track pushed package
                let npm_packages = npm_package_names();
                let crates = crate_names();
                if npm_packages.contains_key(item.repo.name) || crates.contains_key(item.repo.name)
                {
                    pushed_packages.insert(item.repo.name.to_string(), item.version.clone());
                }

                // Push tags (force if tag already exists remotely but we need to update it)
                if should_tag {
                    log(&worker_output, format!("  {} Pushing tags... ", "→".blue()));
                    let push_args = if tag_exists_remote {
                        vec!["push", "origin", &tag, "--force"]
                    } else {
                        vec!["push", "origin", &tag]
                    };
                    if let Err(e) = run_git(&push_args, &repo_path) {
                        log(&worker_output, format!("{}", "✗".red()));
                        log(&worker_output, format!("    {} {}", "Warning:".yellow(), e));
                    } else {
                        log(&worker_output, format!("{}", "✓".green()));
                    }
                }

                worker_completed
                    .lock()
                    .unwrap()
                    .push(item.repo.name.to_string());
            }

            worker_done_flag.store(true, Ordering::SeqCst);
        });

        // Main thread: collect messages (skip for dry_run since worker uses defaults)
        if !self.dry_run && !repos_with_changes.is_empty() {
            println!("\n{}", "═".repeat(60));
            println!("{}", "Enter Commit Messages".bold());
            println!("{}", "═".repeat(60));
            println!(
                "{}",
                "(Processing starts as you enter each message)".dimmed()
            );
            println!();

            let mut first_message = true;
            for idx in &repos_with_changes {
                let item = &self.queue[*idx];
                let repo_path = self.root.join(item.repo.path);
                let status = get_status(&repo_path).unwrap_or_default();
                let change_count = status.lines().count();

                let default_msg = default_commit_message(item.repo.name, &item.version, &status);
                let commit_msg = match global_message.or(item.message.as_deref()) {
                    Some(msg) => {
                        println!(
                            "  {} {} ({} file{}) → {}",
                            "✓".green(),
                            item.repo.name.cyan(),
                            change_count,
                            if change_count == 1 { "" } else { "s" },
                            msg.dimmed()
                        );
                        msg.to_string()
                    }
                    None if auto_yes => {
                        println!(
                            "  {} {} ({} file{}) → {}",
                            "✓".green(),
                            item.repo.name.cyan(),
                            change_count,
                            if change_count == 1 { "" } else { "s" },
                            default_msg.dimmed()
                        );
                        default_msg
                    }
                    None => {
                        print!(
                            "  {} {} ({} file{}) → ",
                            "●".cyan(),
                            item.repo.name.bold(),
                            change_count,
                            if change_count == 1 { "" } else { "s" }
                        );
                        io::stdout().flush().ok();

                        let msg = Input::new()
                            .with_prompt("")
                            .default(default_msg.clone())
                            .interact_text()
                            .unwrap_or(default_msg);
                        msg
                    }
                };

                // Send message to worker
                messages.lock().unwrap().insert(*idx, commit_msg);

                // Show processing started after first message
                if first_message {
                    println!(
                        "  {} {}",
                        "▶".green(),
                        "Processing started in background...".dimmed()
                    );
                    first_message = false;
                }
            }

            all_messages_collected.store(true, Ordering::SeqCst);

            println!();
            println!("{}", "═".repeat(60));
            println!(
                "{} All messages entered. Waiting for processing...",
                "✓".green()
            );
            println!("{}", "═".repeat(60));
        } else {
            // For dry_run or no changes, signal worker to use defaults
            all_messages_collected.store(true, Ordering::SeqCst);
        }

        // Stream output as it becomes available
        println!();
        let mut last_printed = 0;
        loop {
            // Print any new output lines
            {
                let buffer = output_buffer.lock().unwrap();
                for line in buffer.iter().skip(last_printed) {
                    println!("{}", line);
                }
                last_printed = buffer.len();
            }

            // Check if worker is done
            if worker_done.load(Ordering::SeqCst) {
                // Print any final output
                let buffer = output_buffer.lock().unwrap();
                for line in buffer.iter().skip(last_printed) {
                    println!("{}", line);
                }
                break;
            }

            thread::sleep(std::time::Duration::from_millis(50));
        }

        // Wait for worker thread to fully complete
        worker_handle.join().unwrap();

        // Copy completed list from worker
        self.completed = completed_shared.lock().unwrap().clone();

        // Check for errors
        if let Some(error) = worker_error.lock().unwrap().take() {
            return Err(error);
        }

        Ok(())
    }
}

// ============================================================================
// Terminal Spawning
// ============================================================================

/// Spawn a new terminal window and run the commit script there
fn spawn_in_new_terminal(args: &Args, root: &Path) -> Result<(), String> {
    let script_path = root.join("tooling/scripts/commit.rs");

    // Build the command with all the same args plus --spawned
    let mut cmd_parts = vec![
        "rust-script".to_string(),
        script_path.to_string_lossy().to_string(),
        "--spawned".to_string(),
    ];

    cmd_parts.push("--repos".to_string());
    cmd_parts.push(args.repos.clone());

    if args.yes {
        cmd_parts.push("--yes".to_string());
    }
    if args.dry_run {
        cmd_parts.push("--dry-run".to_string());
    }
    if let Some(ref msg) = args.message {
        cmd_parts.push("--message".to_string());
        cmd_parts.push(format!("\"{}\"", msg));
    }
    if args.no_cascade {
        cmd_parts.push("--no-cascade".to_string());
    }

    let cmd_string = cmd_parts.join(" ");

    // macOS: use osascript to open Terminal.app
    #[cfg(target_os = "macos")]
    {
        let apple_script = format!(
            r#"tell application "Terminal"
                activate
                do script "cd '{}' && {}"
            end tell"#,
            root.display(),
            cmd_string
        );

        let status = Command::new("osascript")
            .args(["-e", &apple_script])
            .status()
            .map_err(|e| format!("Failed to spawn terminal: {}", e))?;

        if !status.success() {
            return Err("Failed to spawn terminal window".to_string());
        }
    }

    // Linux: try common terminal emulators
    #[cfg(target_os = "linux")]
    {
        let terminals = [
            ("gnome-terminal", vec!["--", "bash", "-c"]),
            ("konsole", vec!["-e", "bash", "-c"]),
            ("xterm", vec!["-e", "bash", "-c"]),
        ];

        let full_cmd = format!(
            "cd '{}' && {}; read -p 'Press Enter to close...'",
            root.display(),
            cmd_string
        );

        for (term, term_args) in terminals {
            if Command::new("which")
                .arg(term)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
            {
                let mut cmd = Command::new(term);
                for arg in &term_args {
                    cmd.arg(arg);
                }
                cmd.arg(&full_cmd);

                if cmd.spawn().is_ok() {
                    return Ok(());
                }
            }
        }
        return Err("No supported terminal emulator found".to_string());
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        return Err("Terminal spawning not supported on this platform".to_string());
    }

    #[allow(unreachable_code)]
    Ok(())
}

/// Run prep script for the given repos and wait for completion
fn run_prep(root: &Path, repo_names: &[&str]) -> Result<(), String> {
    let repos_arg = repo_names.join(",");

    println!("\n{} Running prep for: {}", "→".blue(), repos_arg.cyan());
    println!("{}", "─".repeat(50).dimmed());

    let status = Command::new("rust-script")
        .args([
            "tooling/scripts/prep.rs",
            "--repos",
            &repos_arg,
            "--sync-versions",
        ])
        .current_dir(root)
        .status()
        .map_err(|e| format!("Failed to run prep: {}", e))?;

    println!("{}", "─".repeat(50).dimmed());

    if status.success() {
        println!("{} Prep completed successfully\n", "✓".green());
        Ok(())
    } else {
        Err("Prep failed".to_string())
    }
}

// ============================================================================
// Main
// ============================================================================

fn main() -> ExitCode {
    let args = Args::parse();

    // Set up Ctrl+C handler
    let interrupted = Arc::new(AtomicBool::new(false));
    let interrupted_clone = interrupted.clone();
    if let Err(e) = ctrlc::set_handler(move || {
        interrupted_clone.store(true, Ordering::SeqCst);
        // Print immediately so user knows it was received
        eprintln!("\n{} Received Ctrl+C, cleaning up...", "⚠".yellow());
    }) {
        eprintln!(
            "{} Failed to set Ctrl+C handler: {}",
            "Warning:".yellow(),
            e
        );
    }

    // Get root directory
    let root = std::env::var("MACROFORGE_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let cwd = std::env::current_dir().unwrap();
            cwd.ancestors()
                .find(|p| p.join("pixi.toml").exists())
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| cwd.clone())
        });

    // Parse initial repos
    let all = all_repos();
    let initial_repos: Vec<Repo> = match args.repos.as_str() {
        "all" => all.clone(),
        "rust" => all
            .iter()
            .filter(|r| r.repo_type == RepoType::Rust)
            .cloned()
            .collect(),
        "ts" => all
            .iter()
            .filter(|r| r.repo_type == RepoType::Ts)
            .cloned()
            .collect(),
        names => {
            let names: Vec<&str> = names.split(',').map(|s| s.trim()).collect();
            all.iter()
                .filter(|r| names.contains(&r.name))
                .cloned()
                .collect()
        }
    };

    if initial_repos.is_empty() {
        eprintln!("{}", "No repos selected".red());
        return ExitCode::FAILURE;
    }

    // Load dependency graph from deps.toml
    let deps = load_dependency_map(&root);
    let graph = DependencyGraph::new(&all, deps.clone());

    // Cascade to dependents
    let initial_names: Vec<&str> = initial_repos.iter().map(|r| r.name).collect();
    let cascaded_names: Vec<String> = if args.no_cascade || args.repos == "all" {
        initial_names.iter().map(|s| s.to_string()).collect()
    } else {
        let cascaded = graph.cascade_to_dependents(&initial_names);
        if cascaded.len() > initial_names.len() {
            let added: Vec<_> = cascaded
                .iter()
                .filter(|n| !initial_names.contains(&n.as_str()))
                .collect();
            println!(
                "{}: Adding dependents: {}",
                "Cascading".yellow(),
                added
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
                    .cyan()
            );
        }
        cascaded
    };

    // Get repos in topological order
    let sorted_names = match graph.sorted_order() {
        Ok(names) => names,
        Err(e) => {
            eprintln!("{}: {}", "Error".red(), e);
            return ExitCode::FAILURE;
        }
    };

    let repos: Vec<Repo> = sorted_names
        .iter()
        .filter(|name| cascaded_names.contains(name))
        .filter_map(|name| all.iter().find(|r| r.name == name).cloned())
        .collect();

    // Load versions
    let mut versions = VersionsCache::load(&root);

    // Check for unprepped repos (local == registry, meaning no version bump)
    // Include all repos - even versionless ones need prep for CI checks
    let unprepped_repos: Vec<&str> = cascaded_names
        .iter()
        .filter(|name| !versions.is_prepped(name))
        .map(|s| s.as_str())
        .collect();

    if !unprepped_repos.is_empty() && !args.dry_run {
        println!(
            "\n{} The following repos have not been prepped:",
            "⚠ Warning:".yellow().bold()
        );
        for repo in &unprepped_repos {
            if let Some(info) = versions.get(repo) {
                println!(
                    "  {} {} ({} = {})",
                    "•".yellow(),
                    repo.cyan(),
                    "local".dimmed(),
                    info.local.dimmed()
                );
            } else {
                println!(
                    "  {} {} ({})",
                    "•".yellow(),
                    repo.cyan(),
                    "no version - needs CI check".dimmed()
                );
            }
        }
        println!();

        if Confirm::new()
            .with_prompt("Would you like to run prep for these repos first?")
            .default(true)
            .interact()
            .unwrap_or(false)
        {
            match run_prep(&root, &unprepped_repos) {
                Ok(()) => {
                    // Reload versions after prep
                    versions = VersionsCache::load(&root);
                }
                Err(e) => {
                    eprintln!("{}: {}", "Error".red(), e);
                    return ExitCode::FAILURE;
                }
            }
        } else {
            println!(
                "{} Continuing without prep. Tags may not be created for unprepped repos.",
                "ℹ".blue()
            );
        }
    }

    // Build commit queue
    let mut queue = CommitQueue::new(
        root.clone(),
        deps,
        versions.0.clone(),
        args.dry_run,
        interrupted,
        args.debug,
    );

    for repo in &repos {
        let version = versions
            .get_local(repo.name)
            .cloned()
            .unwrap_or_else(|| "0.0.0".to_string());
        queue.add(repo.clone(), version, None);
    }

    // Count repos with actual changes
    let repos_with_changes: Vec<_> = repos
        .iter()
        .filter(|r| {
            let repo_path = root.join(r.path);
            repo_path.exists() && (has_changes(&repo_path) || get_unpushed_count(&repo_path) > 0)
        })
        .collect();

    // If there's a queue and we're not already spawned, offer to open in new terminal
    if repos_with_changes.len() > 1 && !args.spawned && !args.dry_run {
        println!("{}", "═".repeat(50));
        println!(
            "{} {} repos have changes: {}",
            "ℹ".blue(),
            repos_with_changes.len(),
            repos_with_changes
                .iter()
                .map(|r| r.name)
                .collect::<Vec<_>>()
                .join(", ")
                .cyan()
        );
        println!("{}", "═".repeat(50));
        println!();

        if Confirm::new()
            .with_prompt("Open commit queue in a new terminal window?")
            .default(true)
            .interact()
            .unwrap_or(false)
        {
            match spawn_in_new_terminal(&args, &root) {
                Ok(()) => {
                    println!(
                        "{} Commit queue started in new terminal window",
                        "✓".green()
                    );
                    return ExitCode::SUCCESS;
                }
                Err(e) => {
                    eprintln!("{} {}", "Warning:".yellow(), e);
                    println!("Continuing in current terminal...\n");
                }
            }
        }
    }

    // Header
    println!("{}", "═".repeat(50));
    println!("{}", "  Commit Queue".bold());
    println!("{}", "═".repeat(50));
    println!();
    println!(
        "{} {}",
        "Repos:".bold(),
        repos
            .iter()
            .map(|r| r.name)
            .collect::<Vec<_>>()
            .join(" → ")
            .cyan()
    );
    if args.dry_run {
        println!("{} {}", "Mode:".bold(), "DRY RUN".yellow());
    }

    // Confirm
    if !args.yes && !args.dry_run {
        println!();
        if !Confirm::new()
            .with_prompt("Start commit queue?")
            .default(true)
            .interact()
            .unwrap_or(false)
        {
            println!("{}", "Cancelled".yellow());
            return ExitCode::SUCCESS;
        }
    }

    // Process queue
    match queue.process(args.yes, args.message.as_deref()) {
        Ok(()) => {
            println!("\n{}", "═".repeat(50));
            if !queue.completed.is_empty() {
                println!(
                    "{} {}",
                    "✓ Completed:".green().bold(),
                    queue.completed.join(", ")
                );
            }
            if args.dry_run {
                println!("\n{}", "This was a dry run. No changes were made.".yellow());
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!("\n{}", "═".repeat(50));
            eprintln!("{} {}", "✗ Failed:".red().bold(), e);
            if !queue.completed.is_empty() {
                println!(
                    "{} {}",
                    "Completed before failure:".dimmed(),
                    queue.completed.join(", ")
                );
            }
            ExitCode::FAILURE
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_deps() -> HashMap<String, Vec<String>> {
        let mut deps = HashMap::new();
        deps.insert("syn".to_string(), vec![]);
        deps.insert("template".to_string(), vec!["syn".to_string()]);
        deps.insert(
            "macros".to_string(),
            vec!["syn".to_string(), "template".to_string()],
        );
        deps.insert(
            "core".to_string(),
            vec![
                "syn".to_string(),
                "template".to_string(),
                "macros".to_string(),
            ],
        );
        deps.insert("shared".to_string(), vec!["core".to_string()]);
        deps.insert(
            "vite-plugin".to_string(),
            vec!["core".to_string(), "shared".to_string()],
        );
        deps.insert(
            "typescript-plugin".to_string(),
            vec!["core".to_string(), "shared".to_string()],
        );
        deps
    }

    fn test_repos() -> Vec<Repo> {
        vec![
            Repo {
                name: "syn",
                path: "crates/macroforge_ts_syn",
                repo_type: RepoType::Rust,
            },
            Repo {
                name: "template",
                path: "crates/macroforge_ts_quote",
                repo_type: RepoType::Rust,
            },
            Repo {
                name: "macros",
                path: "crates/macroforge_ts_macros",
                repo_type: RepoType::Rust,
            },
            Repo {
                name: "core",
                path: "crates/macroforge_ts",
                repo_type: RepoType::Rust,
            },
            Repo {
                name: "shared",
                path: "packages/shared",
                repo_type: RepoType::Ts,
            },
            Repo {
                name: "vite-plugin",
                path: "packages/vite-plugin",
                repo_type: RepoType::Ts,
            },
            Repo {
                name: "typescript-plugin",
                path: "packages/typescript-plugin",
                repo_type: RepoType::Ts,
            },
        ]
    }

    #[test]
    fn test_dependency_graph_sorted_order() {
        let repos = test_repos();
        let deps = test_deps();
        let graph = DependencyGraph::new(&repos, deps);

        let sorted = graph.sorted_order().unwrap();

        // syn should come first (no deps)
        assert_eq!(sorted[0], "syn");

        // template depends on syn, so it comes after
        let syn_pos = sorted.iter().position(|x| x == "syn").unwrap();
        let template_pos = sorted.iter().position(|x| x == "template").unwrap();
        assert!(syn_pos < template_pos, "syn should come before template");

        // macros depends on syn and template
        let macros_pos = sorted.iter().position(|x| x == "macros").unwrap();
        assert!(syn_pos < macros_pos, "syn should come before macros");
        assert!(
            template_pos < macros_pos,
            "template should come before macros"
        );

        // core depends on syn, template, macros
        let core_pos = sorted.iter().position(|x| x == "core").unwrap();
        assert!(macros_pos < core_pos, "macros should come before core");

        // shared depends on core
        let shared_pos = sorted.iter().position(|x| x == "shared").unwrap();
        assert!(core_pos < shared_pos, "core should come before shared");

        // vite-plugin depends on core and shared
        let vite_pos = sorted.iter().position(|x| x == "vite-plugin").unwrap();
        assert!(
            shared_pos < vite_pos,
            "shared should come before vite-plugin"
        );
    }

    #[test]
    fn test_get_dependents() {
        let repos = test_repos();
        let deps = test_deps();
        let graph = DependencyGraph::new(&repos, deps);

        // What depends on syn?
        let syn_dependents = graph.get_dependents("syn");
        assert!(syn_dependents.contains(&"template".to_string()));
        assert!(syn_dependents.contains(&"macros".to_string()));
        assert!(syn_dependents.contains(&"core".to_string()));

        // What depends on core?
        let core_dependents = graph.get_dependents("core");
        assert!(core_dependents.contains(&"shared".to_string()));
        assert!(core_dependents.contains(&"vite-plugin".to_string()));
        assert!(core_dependents.contains(&"typescript-plugin".to_string()));

        // Nothing depends on vite-plugin
        let vite_dependents = graph.get_dependents("vite-plugin");
        assert!(vite_dependents.is_empty());
    }

    #[test]
    fn test_cascade_to_dependents() {
        let repos = test_repos();
        let deps = test_deps();
        let graph = DependencyGraph::new(&repos, deps);

        // If we bump syn, what needs to be updated?
        let cascaded = graph.cascade_to_dependents(&["syn"]);
        assert!(cascaded.contains(&"syn".to_string()));
        assert!(cascaded.contains(&"template".to_string()));
        assert!(cascaded.contains(&"macros".to_string()));
        assert!(cascaded.contains(&"core".to_string()));
        assert!(cascaded.contains(&"shared".to_string()));
        assert!(cascaded.contains(&"vite-plugin".to_string()));
        assert!(cascaded.contains(&"typescript-plugin".to_string()));

        // If we bump shared, what needs to be updated?
        let cascaded = graph.cascade_to_dependents(&["shared"]);
        assert!(cascaded.contains(&"shared".to_string()));
        assert!(cascaded.contains(&"vite-plugin".to_string()));
        assert!(cascaded.contains(&"typescript-plugin".to_string()));
        // Should NOT include upstream deps
        assert!(!cascaded.contains(&"core".to_string()));
        assert!(!cascaded.contains(&"syn".to_string()));
    }

    #[test]
    fn test_cascade_preserves_order() {
        let repos = test_repos();
        let deps = test_deps();
        let graph = DependencyGraph::new(&repos, deps);

        let cascaded = graph.cascade_to_dependents(&["syn"]);

        // Should be in topological order
        let syn_pos = cascaded.iter().position(|x| x == "syn").unwrap();
        let core_pos = cascaded.iter().position(|x| x == "core").unwrap();
        let shared_pos = cascaded.iter().position(|x| x == "shared").unwrap();

        assert!(syn_pos < core_pos);
        assert!(core_pos < shared_pos);
    }

    #[test]
    fn test_deps_toml_parsing() {
        let toml_content = r#"
[crates]
syn = []
template = ["syn"]

[packages]
shared = ["core"]

[other]
website = ["core"]
"#;

        #[derive(Deserialize)]
        struct DepsFile {
            #[serde(default)]
            crates: HashMap<String, Vec<String>>,
            #[serde(default)]
            packages: HashMap<String, Vec<String>>,
            #[serde(default)]
            other: HashMap<String, Vec<String>>,
        }

        let deps_file: DepsFile = toml::from_str(toml_content).unwrap();

        assert_eq!(deps_file.crates.get("syn").unwrap().len(), 0);
        assert_eq!(
            deps_file.crates.get("template").unwrap(),
            &vec!["syn".to_string()]
        );
        assert_eq!(
            deps_file.packages.get("shared").unwrap(),
            &vec!["core".to_string()]
        );
        assert_eq!(
            deps_file.other.get("website").unwrap(),
            &vec!["core".to_string()]
        );
    }

    #[test]
    fn test_queue_order_respects_dependencies() {
        let repos = test_repos();
        let deps = test_deps();
        let graph = DependencyGraph::new(&repos, deps.clone());

        // Simulate what happens when we select "all" repos
        let sorted = graph.sorted_order().unwrap();

        // Verify the queue order ensures dependencies come first
        let positions: HashMap<&str, usize> = sorted
            .iter()
            .enumerate()
            .map(|(i, name)| (name.as_str(), i))
            .collect();

        // For each package, verify all its dependencies come before it
        for (pkg, pkg_deps) in &deps {
            if let Some(&pkg_pos) = positions.get(pkg.as_str()) {
                for dep in pkg_deps {
                    if let Some(&dep_pos) = positions.get(dep.as_str()) {
                        assert!(
                            dep_pos < pkg_pos,
                            "{} (pos {}) should come before {} (pos {})",
                            dep,
                            dep_pos,
                            pkg,
                            pkg_pos
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_npm_package_names() {
        let names = npm_package_names();

        assert_eq!(names.get("core"), Some(&"macroforge"));
        assert_eq!(names.get("shared"), Some(&"@macroforge/shared"));
        assert_eq!(names.get("vite-plugin"), Some(&"@macroforge/vite-plugin"));
        assert_eq!(
            names.get("typescript-plugin"),
            Some(&"@macroforge/typescript-plugin")
        );
        assert_eq!(
            names.get("svelte-language-server"),
            Some(&"@macroforge/svelte-language-server")
        );
        assert_eq!(
            names.get("svelte-preprocessor"),
            Some(&"@macroforge/svelte-preprocessor")
        );
        assert_eq!(names.get("mcp-server"), Some(&"@macroforge/mcp-server"));
        assert_eq!(names.len(), 7);
    }

    #[test]
    fn test_crate_names() {
        let names = crate_names();

        assert_eq!(names.get("syn"), Some(&"macroforge_ts_syn"));
        assert_eq!(names.get("template"), Some(&"macroforge_ts_quote"));
        assert_eq!(names.get("macros"), Some(&"macroforge_ts_macros"));
        assert_eq!(names.get("core"), Some(&"macroforge_ts"));
        assert_eq!(names.len(), 4);
    }

    #[test]
    fn test_versions_cache_get_local() {
        let mut cache = VersionsCache::default();
        cache.0.insert(
            "core".to_string(),
            Some(VersionInfo {
                local: "1.2.3".to_string(),
                registry: "1.2.0".to_string(),
            }),
        );

        assert_eq!(cache.get_local("core"), Some(&"1.2.3".to_string()));
        assert_eq!(cache.get_local("nonexistent"), None);
    }

    #[test]
    fn test_versions_cache_handles_null() {
        let mut cache = VersionsCache::default();
        cache.0.insert("tooling".to_string(), None);

        // null entries should return None for get_local
        assert_eq!(cache.get_local("tooling"), None);
    }

    #[test]
    fn test_all_repos_contains_expected() {
        let repos = all_repos();

        let names: Vec<&str> = repos.iter().map(|r| r.name).collect();
        assert!(names.contains(&"core"));
        assert!(names.contains(&"syn"));
        assert!(names.contains(&"template"));
        assert!(names.contains(&"macros"));
        assert!(names.contains(&"shared"));
        assert!(names.contains(&"vite-plugin"));
        assert!(names.contains(&"typescript-plugin"));
        assert!(names.contains(&"svelte-language-server"));
        assert!(names.contains(&"svelte-preprocessor"));
        assert!(names.contains(&"mcp-server"));
        assert!(names.contains(&"website"));
        assert!(names.contains(&"tooling"));
        assert!(names.contains(&"zed-extensions"));
    }

    #[test]
    fn test_repo_types() {
        let repos = all_repos();

        let rust_repos: Vec<_> = repos
            .iter()
            .filter(|r| r.repo_type == RepoType::Rust)
            .collect();
        let ts_repos: Vec<_> = repos
            .iter()
            .filter(|r| r.repo_type == RepoType::Ts)
            .collect();

        assert_eq!(rust_repos.len(), 4); // core, syn, template, macros
        assert_eq!(ts_repos.len(), 6); // shared, vite-plugin, typescript-plugin, svelte-language-server, svelte-preprocessor, mcp-server
    }

    #[test]
    fn test_dependency_graph_empty_deps() {
        let repos = test_repos();
        let deps = HashMap::new();
        let graph = DependencyGraph::new(&repos, deps);

        // With no dependencies, all repos should be independent
        let sorted = graph.sorted_order().unwrap();
        assert_eq!(sorted.len(), repos.len());
    }

    #[test]
    fn test_get_dependents_returns_direct_dependents() {
        let repos = test_repos();
        let deps = test_deps();
        let graph = DependencyGraph::new(&repos, deps);

        // What directly depends on core?
        let core_dependents = graph.get_dependents("core");
        assert!(core_dependents.contains(&"shared".to_string()));
        assert!(core_dependents.contains(&"vite-plugin".to_string()));
        assert!(core_dependents.contains(&"typescript-plugin".to_string()));

        // syn has many direct dependents
        let syn_dependents = graph.get_dependents("syn");
        assert!(syn_dependents.contains(&"template".to_string()));
        assert!(syn_dependents.contains(&"macros".to_string()));
        assert!(syn_dependents.contains(&"core".to_string()));
    }

    #[test]
    fn test_has_package() {
        // Repos with packages
        assert!(has_package("core")); // npm + crate
        assert!(has_package("syn")); // crate only
        assert!(has_package("shared")); // npm only
        assert!(has_package("vite-plugin")); // npm only

        // Repos without packages
        assert!(!has_package("tooling"));
        assert!(!has_package("website"));
        assert!(!has_package("zed-extensions"));
    }

    #[test]
    fn test_default_commit_message_with_package() {
        // Repos with packages get "Bump to X.Y.Z"
        let msg = default_commit_message("core", "1.2.3", "");
        assert_eq!(msg, "Bump to 1.2.3");

        let msg = default_commit_message("shared", "0.1.5", " M some/file.ts");
        assert_eq!(msg, "Bump to 0.1.5");
    }

    #[test]
    fn test_default_commit_message_without_package() {
        // Repos without packages get "Updated" with file list
        let msg = default_commit_message("tooling", "0.1.0", "");
        assert_eq!(msg, "Updated");

        let msg = default_commit_message("tooling", "0.1.0", " M scripts/commit.rs");
        assert_eq!(msg, "Updated scripts/commit.rs");

        let msg = default_commit_message("tooling", "0.1.0", " M a.rs\n M b.rs\n M c.rs");
        assert_eq!(msg, "Updated a.rs, b.rs, c.rs");

        let msg = default_commit_message("tooling", "0.1.0", " M a.rs\n M b.rs\n M c.rs\n M d.rs");
        assert_eq!(msg, "Updated 4 files");
    }
}
