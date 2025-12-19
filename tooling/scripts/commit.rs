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
//! colored = "2"
//! dialoguer = "0.11"
//! petgraph = "0.6"
//! ctrlc = "3.4"
//! ```

use clap::Parser;
use colored::*;
use dialoguer::{Confirm, Input};
use petgraph::algo::toposort;
use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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
        Repo { name: "core", path: "crates/macroforge_ts", repo_type: RepoType::Rust },
        Repo { name: "macros", path: "crates/macroforge_ts_macros", repo_type: RepoType::Rust },
        Repo { name: "syn", path: "crates/macroforge_ts_syn", repo_type: RepoType::Rust },
        Repo { name: "template", path: "crates/macroforge_ts_quote", repo_type: RepoType::Rust },
        Repo { name: "shared", path: "packages/shared", repo_type: RepoType::Ts },
        Repo { name: "vite-plugin", path: "packages/vite-plugin", repo_type: RepoType::Ts },
        Repo { name: "typescript-plugin", path: "packages/typescript-plugin", repo_type: RepoType::Ts },
        Repo { name: "svelte-language-server", path: "packages/svelte-language-server", repo_type: RepoType::Ts },
        Repo { name: "svelte-preprocessor", path: "packages/svelte-preprocessor", repo_type: RepoType::Ts },
        Repo { name: "mcp-server", path: "packages/mcp-server", repo_type: RepoType::Ts },
        Repo { name: "website", path: "website", repo_type: RepoType::Website },
        Repo { name: "tooling", path: "tooling", repo_type: RepoType::Tooling },
        Repo { name: "zed-extensions", path: "crates/extensions", repo_type: RepoType::Extension },
    ]
}

// ============================================================================
// Dependency Graph
// ============================================================================

/// Dependency map: dependent -> dependencies
fn dependency_map() -> HashMap<&'static str, Vec<&'static str>> {
    [
        ("shared", vec!["core"]),
        ("vite-plugin", vec!["core", "shared"]),
        ("typescript-plugin", vec!["core", "shared"]),
        ("svelte-language-server", vec!["core", "typescript-plugin"]),
        ("svelte-preprocessor", vec!["core"]),
        ("mcp-server", vec!["core"]),
        ("website", vec!["core"]),
        ("zed-extensions", vec!["typescript-plugin", "svelte-language-server"]),
    ]
    .into_iter()
    .collect()
}

/// Map repo names to their npm package names
fn npm_package_names() -> HashMap<&'static str, &'static str> {
    [
        ("core", "macroforge"),
        ("shared", "@macroforge/shared"),
        ("vite-plugin", "@macroforge/vite-plugin"),
        ("typescript-plugin", "@macroforge/typescript-plugin"),
        ("svelte-language-server", "@macroforge/svelte-language-server"),
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
    ]
    .into_iter()
    .collect()
}

#[allow(dead_code)]
struct DependencyGraph {
    graph: DiGraph<String, ()>,
    node_indices: HashMap<String, NodeIndex>,
}

impl DependencyGraph {
    fn new(repos: &[Repo]) -> Self {
        let mut graph = DiGraph::new();
        let mut node_indices = HashMap::new();

        for repo in repos {
            let idx = graph.add_node(repo.name.to_string());
            node_indices.insert(repo.name.to_string(), idx);
        }

        let deps = dependency_map();
        for repo in repos {
            if let Some(dependencies) = deps.get(repo.name) {
                let dependent_idx = node_indices[repo.name];
                for dep in dependencies {
                    if let Some(&dep_idx) = node_indices.get(*dep) {
                        graph.add_edge(dep_idx, dependent_idx, ());
                    }
                }
            }
        }

        Self { graph, node_indices }
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
        let deps = dependency_map();
        deps.iter()
            .filter(|(_, dependencies)| dependencies.contains(&repo_name))
            .map(|(dependent, _)| dependent.to_string())
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

#[derive(Debug, Serialize, Deserialize, Default)]
struct VersionsCache(HashMap<String, String>);

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

    fn get(&self, name: &str) -> Option<&String> {
        self.0.get(name)
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
    get_status(cwd).map(|s| !s.trim().is_empty()).unwrap_or(false)
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

fn unstage_all(cwd: &Path) {
    let _ = run_git(&["reset", "HEAD"], cwd);
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

/// Wait for a package to be available on the registry
fn wait_for_package(
    repo_name: &str,
    version: &str,
    timeout_secs: u64,
    interrupted: &Arc<AtomicBool>,
) -> Result<(), String> {
    let npm_packages = npm_package_names();
    let crates = crate_names();

    let (registry, check_fn): (&str, Box<dyn Fn() -> bool>) = if let Some(&npm_name) = npm_packages.get(repo_name) {
        ("npm", Box::new(move || check_npm_version(npm_name, version)))
    } else if let Some(&crate_name) = crates.get(repo_name) {
        ("crates.io", Box::new(move || check_crate_version(crate_name, version)))
    } else {
        // No registry to check (e.g., website, tooling, zed-extensions)
        return Ok(());
    };

    let start = std::time::Instant::now();
    let poll_interval = std::time::Duration::from_secs(10);
    let timeout = std::time::Duration::from_secs(timeout_secs);

    print!("  {} Waiting for {} on {}... ", "⏳".yellow(), version.cyan(), registry);
    io::stdout().flush().ok();

    loop {
        // Check for Ctrl+C
        if interrupted.load(Ordering::SeqCst) {
            println!("{}", "interrupted".yellow());
            return Err("Interrupted by user".to_string());
        }

        if check_fn() {
            println!("{}", "available!".green());
            return Ok(());
        }

        if start.elapsed() > timeout {
            println!("{}", "timeout".red());
            return Err(format!(
                "Timeout waiting for {} v{} on {}",
                repo_name, version, registry
            ));
        }

        print!(".");
        io::stdout().flush().ok();
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
    /// Packages we've pushed that need to be published (repo_name -> version)
    pushed_packages: HashMap<String, String>,
    dry_run: bool,
    /// Timeout for waiting on registry (seconds)
    registry_timeout: u64,
    /// Flag to track if Ctrl+C was pressed
    interrupted: Arc<AtomicBool>,
}

#[derive(Clone)]
struct QueueItem {
    repo: Repo,
    version: String,
    message: Option<String>,
}

impl CommitQueue {
    fn new(root: PathBuf, dry_run: bool, interrupted: Arc<AtomicBool>) -> Self {
        Self {
            root,
            queue: Vec::new(),
            completed: Vec::new(),
            pushed_packages: HashMap::new(),
            dry_run,
            registry_timeout: 600, // 10 minutes default
            interrupted,
        }
    }

    fn is_interrupted(&self) -> bool {
        self.interrupted.load(Ordering::SeqCst)
    }

    fn add(&mut self, repo: Repo, version: String, message: Option<String>) {
        self.queue.push(QueueItem { repo, version, message });
    }

    /// Get dependencies that we pushed in this session and need to wait for
    fn get_pending_dependencies(&self, repo_name: &str) -> Vec<(String, String)> {
        let deps = dependency_map();
        deps.get(repo_name)
            .map(|d| {
                d.iter()
                    .filter_map(|dep| {
                        self.pushed_packages
                            .get(*dep)
                            .map(|v| (dep.to_string(), v.clone()))
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    fn process(&mut self, auto_yes: bool, global_message: Option<&str>) -> Result<(), String> {
        let total = self.queue.len();

        // Clone queue items for iteration since we need to mutate pushed_packages
        let items: Vec<_> = self.queue.iter().cloned().collect();

        for (idx, item) in items.iter().enumerate() {
            // Check for Ctrl+C
            if self.is_interrupted() {
                println!("\n{} Interrupted by user", "⚠".yellow());
                self.abort_remaining(idx)?;
                return Err("Interrupted by user (Ctrl+C)".to_string());
            }

            let repo_path = self.root.join(item.repo.path);
            let position = format!("[{}/{}]", idx + 1, total);

            println!("\n{} {} {}", position.dimmed(), "Processing".bold(), item.repo.name.cyan());
            println!("{}", "─".repeat(50).dimmed());

            if !repo_path.exists() {
                println!("  {} Path does not exist, skipping", "⚠".yellow());
                continue;
            }

            // Wait for dependencies that we pushed in this session
            let pending_deps = self.get_pending_dependencies(item.repo.name);
            if !pending_deps.is_empty() && !self.dry_run {
                println!("  {} Waiting for dependencies...", "⏳".yellow());
                for (dep_name, dep_version) in &pending_deps {
                    if let Err(e) = wait_for_package(dep_name, dep_version, self.registry_timeout, &self.interrupted) {
                        self.abort_remaining(idx)?;
                        return Err(e);
                    }
                    // Remove from pending after confirmed available
                    self.pushed_packages.remove(dep_name);
                }
            }

            // Check for changes or unpushed commits
            let has_local_changes = has_changes(&repo_path);
            let unpushed = get_unpushed_count(&repo_path);

            if !has_local_changes && unpushed == 0 {
                println!("  {} No changes, skipping", "ℹ".blue());
                continue;
            }

            if !has_local_changes && unpushed > 0 {
                println!("  {} {} unpushed commit{}",
                    "ℹ".blue(),
                    unpushed.to_string().yellow(),
                    if unpushed == 1 { "" } else { "s" }
                );

                if self.dry_run {
                    println!("  {} Would push", "[dry-run]".yellow());
                    self.completed.push(item.repo.name.to_string());
                    continue;
                }

                print!("  {} Pushing... ", "→".blue());
                io::stdout().flush().ok();
                match run_git(&["push"], &repo_path) {
                    Ok(_) => {
                        println!("{}", "✓".green());
                        self.completed.push(item.repo.name.to_string());
                    }
                    Err(e) => {
                        println!("{}", "✗".red());
                        self.abort_remaining(idx)?;
                        return Err(format!("Push failed for {}: {}", item.repo.name, e));
                    }
                }
                continue;
            }

            // Show changes
            let status = get_status(&repo_path).unwrap_or_default();
            let change_count = status.lines().count();
            println!("  {} {} file{} changed", "ℹ".blue(), change_count, if change_count == 1 { "" } else { "s" });

            // Get commit message
            let default_msg = format!("Bump to {}", item.version);
            let commit_msg = match global_message.or(item.message.as_deref()) {
                Some(msg) => msg.to_string(),
                None if auto_yes => default_msg,
                None => {
                    Input::new()
                        .with_prompt(format!("  Commit message for {}", item.repo.name.cyan()))
                        .default(default_msg.clone())
                        .interact_text()
                        .unwrap_or(default_msg)
                }
            };

            if self.dry_run {
                let tag = format!("v{}", item.version);
                let would_tag = !tag_exists_remotely(&tag, &repo_path);
                println!("  {} Would: add, commit, {}",
                    "[dry-run]".yellow(),
                    if would_tag { format!("tag {}, push", tag) } else { "push".to_string() }
                );
                self.completed.push(item.repo.name.to_string());
                continue;
            }

            // Stage
            print!("  {} Staging... ", "→".blue());
            io::stdout().flush().ok();
            if let Err(e) = run_git(&["add", "-A"], &repo_path) {
                println!("{}", "✗".red());
                self.abort_remaining(idx)?;
                return Err(format!("Stage failed for {}: {}", item.repo.name, e));
            }
            println!("{}", "✓".green());

            // Commit
            print!("  {} Committing... ", "→".blue());
            io::stdout().flush().ok();
            if let Err(e) = run_git(&["commit", "-m", &commit_msg], &repo_path) {
                println!("{}", "✗".red());
                self.abort_remaining(idx)?;
                return Err(format!("Commit failed for {}: {}", item.repo.name, e));
            }
            println!("{}", "✓".green());

            // Tag if needed
            let tag = format!("v{}", item.version);
            let should_tag = !tag_exists_remotely(&tag, &repo_path);

            if should_tag {
                if tag_exists_locally(&tag, &repo_path) {
                    let _ = run_git(&["tag", "-d", &tag], &repo_path);
                }

                print!("  {} Tagging {}... ", "→".blue(), tag.cyan());
                io::stdout().flush().ok();
                if let Err(e) = run_git(&["tag", &tag], &repo_path) {
                    println!("{}", "✗".red());
                    self.abort_remaining(idx)?;
                    return Err(format!("Tag failed for {}: {}", item.repo.name, e));
                }
                println!("{}", "✓".green());
            }

            // Push
            print!("  {} Pushing... ", "→".blue());
            io::stdout().flush().ok();
            if let Err(e) = run_git(&["push"], &repo_path) {
                println!("{}", "✗".red());
                self.abort_remaining(idx)?;
                return Err(format!("Push failed for {}: {}", item.repo.name, e));
            }
            println!("{}", "✓".green());

            // Track this package as pushed (for dependency waiting)
            let npm_packages = npm_package_names();
            let crates = crate_names();
            if npm_packages.contains_key(item.repo.name) || crates.contains_key(item.repo.name) {
                self.pushed_packages.insert(item.repo.name.to_string(), item.version.clone());
            }

            // Push tags
            if should_tag {
                print!("  {} Pushing tags... ", "→".blue());
                io::stdout().flush().ok();
                if let Err(e) = run_git(&["push", "--tags"], &repo_path) {
                    println!("{}", "✗".red());
                    // Don't abort for tag push failure, just warn
                    println!("    {} {}", "Warning:".yellow(), e);
                } else {
                    println!("{}", "✓".green());
                }
            }

            self.completed.push(item.repo.name.to_string());
        }

        Ok(())
    }

    fn abort_remaining(&self, failed_idx: usize) -> Result<(), String> {
        println!("\n{} Unstaging remaining repos...", "⚠".yellow());

        for item in self.queue.iter().skip(failed_idx) {
            let repo_path = self.root.join(item.repo.path);
            if repo_path.exists() && has_changes(&repo_path) {
                print!("  {} Unstaging {}... ", "→".yellow(), item.repo.name);
                io::stdout().flush().ok();
                unstage_all(&repo_path);
                println!("{}", "done".dimmed());
            }
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

        let full_cmd = format!("cd '{}' && {}; read -p 'Press Enter to close...'", root.display(), cmd_string);

        for (term, term_args) in terminals {
            if Command::new("which").arg(term).output().map(|o| o.status.success()).unwrap_or(false) {
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
        eprintln!("{} Failed to set Ctrl+C handler: {}", "Warning:".yellow(), e);
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
        "rust" => all.iter().filter(|r| r.repo_type == RepoType::Rust).cloned().collect(),
        "ts" => all.iter().filter(|r| r.repo_type == RepoType::Ts).cloned().collect(),
        names => {
            let names: Vec<&str> = names.split(',').map(|s| s.trim()).collect();
            all.iter().filter(|r| names.contains(&r.name)).cloned().collect()
        }
    };

    if initial_repos.is_empty() {
        eprintln!("{}", "No repos selected".red());
        return ExitCode::FAILURE;
    }

    // Build dependency graph with all repos
    let graph = DependencyGraph::new(&all);

    // Cascade to dependents
    let initial_names: Vec<&str> = initial_repos.iter().map(|r| r.name).collect();
    let cascaded_names: Vec<String> = if args.no_cascade || args.repos == "all" {
        initial_names.iter().map(|s| s.to_string()).collect()
    } else {
        let cascaded = graph.cascade_to_dependents(&initial_names);
        if cascaded.len() > initial_names.len() {
            let added: Vec<_> = cascaded.iter()
                .filter(|n| !initial_names.contains(&n.as_str()))
                .collect();
            println!(
                "{}: Adding dependents: {}",
                "Cascading".yellow(),
                added.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ").cyan()
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
    let versions = VersionsCache::load(&root);

    // Build commit queue
    let mut queue = CommitQueue::new(root.clone(), args.dry_run, interrupted);

    for repo in &repos {
        let version = versions.get(repo.name).cloned().unwrap_or_else(|| "0.0.0".to_string());
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
        println!("{} {} repos have changes: {}",
            "ℹ".blue(),
            repos_with_changes.len(),
            repos_with_changes.iter().map(|r| r.name).collect::<Vec<_>>().join(", ").cyan()
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
                    println!("{} Commit queue started in new terminal window", "✓".green());
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
    println!("{} {}", "Repos:".bold(), repos.iter().map(|r| r.name).collect::<Vec<_>>().join(" → ").cyan());
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
                println!("{} {}", "✓ Completed:".green().bold(), queue.completed.join(", "));
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
                println!("{} {}", "Completed before failure:".dimmed(), queue.completed.join(", "));
            }
            ExitCode::FAILURE
        }
    }
}
