#!/usr/bin/env rust-script
//! Prepare for commit: bump version, update dependencies, build, test, and sync documentation.
//!
//! Uses petgraph to topologically sort packages by their dependencies, ensuring
//! packages are built in the correct order.
//!
//! ```cargo
//! [dependencies]
//! clap = { version = "4", features = ["derive"] }
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! toml = "0.8"
//! petgraph = "0.6"
//! colored = "2"
//! ctrlc = "3.4"
//! ```

use clap::Parser;
use colored::*;
use petgraph::algo::toposort;
use petgraph::graph::DiGraph;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// ============================================================================
// CLI
// ============================================================================

#[derive(Parser, Debug)]
#[command(name = "prep")]
#[command(about = "Prepare a release: bump version, build, test, and sync documentation")]
struct Args {
    /// Version string (e.g., 0.1.4). If not provided, auto-increments patch version
    #[arg()]
    version: Option<String>,

    /// Repos to update (comma-separated, or 'all', 'rust', 'ts')
    #[arg(short, long, default_value = "all")]
    repos: String,

    /// Run build and tests without bumping version
    #[arg(long)]
    dry_run: bool,

    /// Skip build and test steps
    #[arg(long)]
    skip_build: bool,

    /// Skip documentation steps
    #[arg(long)]
    skip_docs: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Don't cascade version bumps to dependent packages
    #[arg(long)]
    no_cascade: bool,

    /// Only bump versions (skip build, test, and docs)
    #[arg(long)]
    bump_only: bool,

    /// Sync all packages/crates/extensions to their latest versions from versions.json
    #[arg(long)]
    sync_versions: bool,
}

// ============================================================================
// Data Structures (Matching manifests.rs)
// ============================================================================

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
enum RepoType {
    Rust,
    Ts,
    Website,
    Tooling,
    Extension,
}

#[derive(Debug, Clone, Deserialize)]
struct Repo {
    name: String,
    path: String,
    repo_type: RepoType,
    package_json: Option<String>,
    cargo_toml: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct VersionInfo {
    local: String,
    registry: String,
}

#[derive(Debug, Clone, Deserialize)]
struct VersionsCache(HashMap<String, Option<VersionInfo>>);

impl VersionsCache {
    fn get(&self, name: &str) -> Option<&VersionInfo> {
        self.0.get(name).and_then(|v| v.as_ref())
    }
}

// ============================================================================
// Dependency Graph
// ============================================================================

struct DependencyGraph {
    graph: DiGraph<String, ()>,
    deps: HashMap<String, Vec<String>>,
}

impl DependencyGraph {
    fn new(repos: &[Repo], deps: HashMap<String, Vec<String>>) -> Self {
        let mut graph = DiGraph::new();
        let mut node_indices = HashMap::new();

        for repo in repos {
            let idx = graph.add_node(repo.name.clone());
            node_indices.insert(repo.name.clone(), idx);
        }

        for repo in repos {
            if let Some(dependencies) = deps.get(&repo.name) {
                let dependent_idx = node_indices[&repo.name];
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

    fn get_dependencies(&self, repo_name: &str) -> Vec<String> {
        self.deps.get(repo_name).cloned().unwrap_or_default()
    }

    fn cascade_to_dependents(&self, repo_names: &[&str]) -> Vec<String> {
        let mut result: std::collections::HashSet<String> =
            repo_names.iter().map(|s| s.to_string()).collect();
        let mut to_process: Vec<String> = repo_names.iter().map(|s| s.to_string()).collect();

        // Get dependents map (reverse dependencies)
        let mut dependents_map: HashMap<String, Vec<String>> = HashMap::new();
        for (dependent, deps) in &self.deps {
            for dep in deps {
                dependents_map
                    .entry(dep.clone())
                    .or_default()
                    .push(dependent.clone());
            }
        }

        while let Some(repo) = to_process.pop() {
            if let Some(dependents) = dependents_map.get(&repo) {
                for dependent in dependents {
                    if result.insert(dependent.clone()) {
                        to_process.push(dependent.clone());
                    }
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
// Helpers
// ============================================================================

fn increment_patch(version: &str) -> String {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() == 3 {
        let patch: u32 = parts[2].parse().unwrap_or(0);
        format!("{}.{}.{}", parts[0], parts[1], patch + 1)
    } else {
        version.to_string()
    }
}

fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
    let parse = |v: &str| -> (u32, u32, u32) {
        let parts: Vec<&str> = v.split('.').collect();
        (
            parts.first().and_then(|s| s.parse().ok()).unwrap_or(0),
            parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0),
            parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0),
        )
    };
    parse(a).cmp(&parse(b))
}

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
    eprintln!("{} Could not load tooling/deps.toml", "Warning:".yellow());
    HashMap::new()
}

// ============================================================================
// Manifests.rs Interop
// ============================================================================

fn manifests_cmd(args: &[&str]) -> Result<(), String> {
    let status = Command::new("rust-script")
        .arg("tooling/scripts/manifests.rs")
        .args(args)
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("manifests.rs failed with {:?}", status.code()))
    }
}

fn get_repos() -> Vec<Repo> {
    let output = Command::new("rust-script")
        .args(["tooling/scripts/manifests.rs", "list"])
        .output()
        .expect("Failed to list repos");
    serde_json::from_slice(&output.stdout).expect("Failed to parse repos")
}

fn get_versions() -> VersionsCache {
    let output = Command::new("rust-script")
        .args(["tooling/scripts/manifests.rs", "dump-versions"])
        .output()
        .expect("Failed to dump versions");
    let map: HashMap<String, Option<VersionInfo>> =
        serde_json::from_slice(&output.stdout).expect("Failed to parse versions");
    VersionsCache(map)
}

fn published_names() -> HashMap<&'static str, &'static str> {
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
        ("syn", "macroforge_ts_syn"),
        ("template", "macroforge_ts_quote"),
        ("macros", "macroforge_ts_macros"),
        ("website", "macroforge.dev"),
        ("zed-extensions", "zed extensions"),
    ]
    .into_iter()
    .collect()
}

// ============================================================================
// Build Logic
// ============================================================================

fn run_cmd(cmd: &str, cwd: &Path) -> Result<(), String> {
    println!("\n> {}\n", cmd.yellow());
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", cmd])
            .current_dir(cwd)
            .status()
    } else {
        Command::new("sh")
            .args(["-c", cmd])
            .current_dir(cwd)
            .status()
    };
    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(format!("Command failed with exit code: {:?}", s.code())),
        Err(e) => Err(format!("Failed to execute command: {}", e)),
    }
}

fn build_repo(
    root: &Path,
    repo: &Repo,
    built_repos: &[&str],
    graph: &DependencyGraph,
    versions: &VersionsCache,
) -> Result<(), String> {
    match repo.repo_type {
        RepoType::Rust => {
            if repo.name == "core" {
                run_cmd("npm run cleanbuild", &root.join(&repo.path))?;
            }
        }
        RepoType::Ts => {
            let repo_path = root.join(&repo.path);
            let deps = graph.get_dependencies(&repo.name);
            let local_deps: Vec<String> = deps
                .iter()
                .filter(|d| {
                    if !built_repos.contains(&d.as_str()) {
                        return false;
                    }
                    if let Some(info) = versions.get(d) {
                        return info.local != info.registry;
                    }
                    false
                })
                .cloned()
                .collect();

            if !local_deps.is_empty() {
                println!("  Using local deps: {}", local_deps.join(", ").dimmed());
                let mut cmd = Command::new("rust-script");
                cmd.arg("tooling/scripts/manifests.rs")
                    .arg("link-local")
                    .arg(&repo.name);
                for d in &local_deps {
                    cmd.arg(d);
                }
                cmd.status().map_err(|e| e.to_string())?;
            }

            let has_build_script = repo
                .package_json
                .as_ref()
                .map(|p| root.join(p))
                .filter(|p| p.exists())
                .and_then(|p| fs::read_to_string(&p).ok())
                .and_then(|content| serde_json::from_str::<serde_json::Value>(&content).ok())
                .and_then(|pkg| pkg.get("scripts")?.get("build").cloned())
                .is_some();

            let build_cmd = if has_build_script {
                "rm -rf node_modules dist && npm install && npm run build"
            } else {
                "rm -rf node_modules && npm install"
            };
            let result = run_cmd(build_cmd, &repo_path);

            manifests_cmd(&["restore-repo", &repo.name])?;
            result?;
        }
        RepoType::Website => {
            let website_path = root.join(&repo.path);
            run_cmd("git pull origin", &website_path)?;
            run_cmd(
                "rm -rf node_modules .svelte-kit && npm install && npm run build",
                &website_path,
            )?;
        }
        _ => {}
    }
    Ok(())
}

// ============================================================================
// Main
// ============================================================================

fn main() -> ExitCode {
    let args = Args::parse();
    let root = std::env::var("MACROFORGE_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let cwd = std::env::current_dir().unwrap();
            cwd.ancestors()
                .find(|p| p.join("pixi.toml").exists())
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| cwd.clone())
        });

    let interrupted = Arc::new(AtomicBool::new(false));
    let interrupted_clone = interrupted.clone();
    let root_clone = root.clone();
    if let Err(e) = ctrlc::set_handler(move || {
        if interrupted_clone.swap(true, Ordering::SeqCst) {
            eprintln!("\n{} Force exiting...", "⚠".red());
            std::process::exit(130);
        }
        eprintln!("\n{} Received Ctrl+C, cleaning up...", "⚠".yellow());
        let _ = Command::new("rust-script")
            .args(["tooling/scripts/manifests.rs", "swap-registry"])
            .current_dir(&root_clone)
            .status();
        eprintln!("{} Cleanup complete. Exiting.", "✓".green());
        std::process::exit(130);
    }) {
        eprintln!(
            "{} Failed to set Ctrl+C handler: {}",
            "Warning:".yellow(),
            e
        );
    }

    let all = get_repos();
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
        names => names
            .split(',')
            .filter_map(|n| all.iter().find(|r| r.name == n.trim()).cloned())
            .collect(),
    };

    if initial_repos.is_empty() {
        eprintln!("{}", "No repos selected".red());
        return ExitCode::FAILURE;
    }

    let deps = load_dependency_map(&root);
    let graph = DependencyGraph::new(&all, deps);
    let initial_names: Vec<&str> = initial_repos.iter().map(|r| r.name.as_str()).collect();

    let cascaded_names: Vec<String> = if args.no_cascade || args.repos == "all" {
        initial_names.iter().map(|s| s.to_string()).collect()
    } else {
        let cascaded = graph.cascade_to_dependents(&initial_names);
        if cascaded.len() > initial_names.len() {
            let added: Vec<&str> = cascaded
                .iter()
                .filter(|n| !initial_names.contains(&n.as_str()))
                .map(|s| s.as_str())
                .collect();
            println!(
                "{}: Adding dependents: {}",
                "Cascading".yellow(),
                added.join(", ").cyan()
            );
        }
        cascaded
    };

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
        .filter_map(|name| all.iter().find(|r| r.name == *name).cloned())
        .collect();

    let repo_names: Vec<&str> = repos.iter().map(|r| r.name.as_str()).collect();
    let versions = get_versions();
    let original_versions: HashMap<String, String> = repos
        .iter()
        .filter_map(|r| {
            versions
                .get(&r.name)
                .map(|v| (r.name.clone(), v.local.clone()))
        })
        .collect();

    // Calculate max version and increment
    let base_version = repos
        .iter()
        .filter_map(|r| versions.get(&r.name).map(|v| v.local.clone()))
        .max_by(|a, b| compare_versions(a, b))
        .unwrap_or_else(|| "0.1.0".to_string());

    let global_next_version = increment_patch(&base_version);

    if args.dry_run {
        println!("{}", "=".repeat(60));
        println!("{}: Testing with current versions", "DRY RUN".yellow());
        if args.sync_versions {
            println!("Mode: Sync (Increment Global)");
        } else {
            println!("Mode: Increment");
        }
        println!("Repos: {}", repo_names.join(", ").cyan());
        println!("{}", "=".repeat(60));
    } else {
        println!("\n[1/9] Bumping versions...");
        for repo in &repos {
            let version = if args.sync_versions {
                args.version
                    .clone()
                    .unwrap_or_else(|| global_next_version.clone())
            } else {
                if let Some(v) = args.version.clone() {
                    v
                } else {
                    let current = versions
                        .get(&repo.name)
                        .map(|v| v.local.clone())
                        .unwrap_or("0.1.0".to_string());
                    increment_patch(&current)
                }
            };

            if repo.package_json.is_some() || repo.cargo_toml.is_some() {
                if let Err(e) = manifests_cmd(&["set-version", &repo.name, &version]) {
                    eprintln!("{}: {}", "Error setting version".red(), e);
                    return ExitCode::FAILURE;
                }
            }
        }
        if args.repos == "all" {
            let _ = manifests_cmd(&["update-zed"]);
        }
    }

    if !args.skip_docs && !args.bump_only {
        println!("\n{}", "[2/9] Extracting API documentation...".bold());
        let _ = run_cmd("rust-script tooling/scripts/extract-rust-docs.rs", &root);
        let _ = run_cmd("rust-script tooling/scripts/extract-ts-docs.rs", &root);
    }

    if args.skip_build || args.bump_only {
        println!("\n{}", "[3/9] Skipping build/fmt/clippy/test...".dimmed());
    } else {
        println!("\n{}", "[3/9] Clean building packages...".bold());
        println!("  {} Swapping to local path dependencies...", "→".blue());
        if let Err(e) = manifests_cmd(&["swap-local"]) {
            eprintln!("{}: {}", "Failed to swap local".red(), e);
            return ExitCode::FAILURE;
        }

        let mut built = Vec::new();
        // Reload versions after updates
        let current_versions = get_versions();
        for repo in &repos {
            if interrupted.load(Ordering::SeqCst) {
                break;
            }
            println!("\n  {} {}", "Building:".bold(), repo.name.cyan());
            if let Err(e) = build_repo(&root, repo, &built, &graph, &current_versions) {
                eprintln!("{}: {}", "Build failed".red(), e);
                let _ = manifests_cmd(&["swap-registry"]);
                return ExitCode::FAILURE;
            }
            built.push(repo.name.as_str());
        }

        // Rust checks
        let rust_repos: Vec<_> = repos
            .iter()
            .filter(|r| r.repo_type == RepoType::Rust)
            .collect();
        println!("\n{}", "[4/9] Checking formatting...".bold());
        for repo in &rust_repos {
            if run_cmd("cargo fmt -- --check", &root.join(&repo.path)).is_err() {
                return ExitCode::FAILURE;
            }
        }
        println!("\n{}", "[5/9] Running clippy...".bold());
        for repo in &rust_repos {
            if run_cmd("cargo clippy -- -D warnings", &root.join(&repo.path)).is_err() {
                return ExitCode::FAILURE;
            }
        }
        println!("\n{}", "[6/9] Running tests...".bold());
        for repo in &rust_repos {
            if run_cmd("cargo test", &root.join(&repo.path)).is_err() {
                return ExitCode::FAILURE;
            }
        }

        println!("  {} Restoring registry dependencies...", "→".blue());
        let _ = manifests_cmd(&["swap-registry"]);
    }

    if !args.skip_docs && !args.bump_only {
        println!("\n{}", "[7/9] Rebuilding docs book...".bold());
        let _ = run_cmd("rust-script tooling/scripts/build-docs-book.rs", &root);
        println!("\n{}", "[8/9] Syncing MCP server docs...".bold());
        let _ = run_cmd("npm run build:docs", &root.join("packages/mcp-server"));
    }

    println!("\n{}", "[9/9] Done!".bold().green());

    // Summary
    println!("\n{}", "═".repeat(80));
    println!("{}", "Version Summary".bold());
    println!("\n{}", "═".repeat(80));
    let pub_names = published_names();
    // Reload final versions
    let final_versions = get_versions();

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
        let pub_name = pub_names.get(repo.name.as_str()).copied().unwrap_or("—");
        if is_package {
            let before = original_versions
                .get(&repo.name)
                .map(|s| s.as_str())
                .unwrap_or("(new)");
            let after = final_versions
                .get(&repo.name)
                .map(|v| v.local.as_str())
                .unwrap_or("?");
            let changed = before != after;
            let color_fn = if changed {
                |s: &str| s.green()
            } else {
                |s: &str| s.dimmed()
            };

            println!(
                "  {:<nw$}  {:<pw$}  {:>vw$}  →  {:>vw$}",
                repo.name.cyan(),
                pub_name,
                before.dimmed(),
                color_fn(after),
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

    ExitCode::SUCCESS
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_patch() {
        assert_eq!(increment_patch("0.1.0"), "0.1.1");
        assert_eq!(increment_patch("1.2.3"), "1.2.4");
        assert_eq!(increment_patch("0.0.0"), "0.0.1");
        assert_eq!(increment_patch("10.20.99"), "10.20.100");
    }

    #[test]
    fn test_increment_patch_invalid() {
        // Invalid versions should return as-is
        assert_eq!(increment_patch("1.2"), "1.2");
        assert_eq!(increment_patch("invalid"), "invalid");
    }

    #[test]
    fn test_compare_versions_equal() {
        assert_eq!(
            compare_versions("1.0.0", "1.0.0"),
            std::cmp::Ordering::Equal
        );
        assert_eq!(
            compare_versions("0.1.5", "0.1.5"),
            std::cmp::Ordering::Equal
        );
    }

    #[test]
    fn test_compare_versions_less() {
        assert_eq!(compare_versions("0.1.0", "0.1.1"), std::cmp::Ordering::Less);
        assert_eq!(compare_versions("0.1.0", "0.2.0"), std::cmp::Ordering::Less);
        assert_eq!(compare_versions("0.1.0", "1.0.0"), std::cmp::Ordering::Less);
        assert_eq!(compare_versions("1.9.9", "2.0.0"), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_compare_versions_greater() {
        assert_eq!(
            compare_versions("0.1.1", "0.1.0"),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            compare_versions("0.2.0", "0.1.0"),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            compare_versions("1.0.0", "0.9.9"),
            std::cmp::Ordering::Greater
        );
    }

    #[test]
    fn test_compare_versions_partial() {
        // Partial versions should still work (missing parts treated as 0)
        assert_eq!(compare_versions("1", "0.9.9"), std::cmp::Ordering::Greater);
        assert_eq!(
            compare_versions("1.2", "1.1.9"),
            std::cmp::Ordering::Greater
        );
    }

    fn test_repos() -> Vec<Repo> {
        vec![
            Repo {
                name: "syn".to_string(),
                path: "crates/syn".to_string(),
                repo_type: RepoType::Rust,
                package_json: None,
                cargo_toml: Some("crates/syn/Cargo.toml".to_string()),
            },
            Repo {
                name: "template".to_string(),
                path: "crates/template".to_string(),
                repo_type: RepoType::Rust,
                package_json: None,
                cargo_toml: Some("crates/template/Cargo.toml".to_string()),
            },
            Repo {
                name: "macros".to_string(),
                path: "crates/macros".to_string(),
                repo_type: RepoType::Rust,
                package_json: None,
                cargo_toml: Some("crates/macros/Cargo.toml".to_string()),
            },
            Repo {
                name: "core".to_string(),
                path: "crates/core".to_string(),
                repo_type: RepoType::Rust,
                package_json: Some("crates/core/package.json".to_string()),
                cargo_toml: Some("crates/core/Cargo.toml".to_string()),
            },
            Repo {
                name: "shared".to_string(),
                path: "packages/shared".to_string(),
                repo_type: RepoType::Ts,
                package_json: Some("packages/shared/package.json".to_string()),
                cargo_toml: None,
            },
        ]
    }

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
        deps
    }

    #[test]
    fn test_dependency_graph_sorted_order() {
        let repos = test_repos();
        let deps = test_deps();
        let graph = DependencyGraph::new(&repos, deps);

        let sorted = graph.sorted_order().unwrap();

        // syn should come first (no deps)
        assert_eq!(sorted[0], "syn");

        // Verify topological order
        let pos = |name: &str| sorted.iter().position(|x| x == name).unwrap();
        assert!(pos("syn") < pos("template"));
        assert!(pos("template") < pos("macros"));
        assert!(pos("macros") < pos("core"));
        assert!(pos("core") < pos("shared"));
    }

    #[test]
    fn test_dependency_graph_get_dependencies() {
        let repos = test_repos();
        let deps = test_deps();
        let graph = DependencyGraph::new(&repos, deps);

        assert!(graph.get_dependencies("syn").is_empty());
        assert_eq!(graph.get_dependencies("template"), vec!["syn"]);
        assert_eq!(graph.get_dependencies("core").len(), 3);
        assert!(graph.get_dependencies("nonexistent").is_empty());
    }

    #[test]
    fn test_cascade_to_dependents() {
        let repos = test_repos();
        let deps = test_deps();
        let graph = DependencyGraph::new(&repos, deps);

        // Cascading from syn should include everything
        let cascaded = graph.cascade_to_dependents(&["syn"]);
        assert!(cascaded.contains(&"syn".to_string()));
        assert!(cascaded.contains(&"template".to_string()));
        assert!(cascaded.contains(&"macros".to_string()));
        assert!(cascaded.contains(&"core".to_string()));
        assert!(cascaded.contains(&"shared".to_string()));

        // Cascading from shared should only include shared
        let cascaded = graph.cascade_to_dependents(&["shared"]);
        assert_eq!(cascaded.len(), 1);
        assert!(cascaded.contains(&"shared".to_string()));
    }

    #[test]
    fn test_cascade_preserves_topological_order() {
        let repos = test_repos();
        let deps = test_deps();
        let graph = DependencyGraph::new(&repos, deps);

        let cascaded = graph.cascade_to_dependents(&["syn"]);

        // Should be in topological order
        let pos = |name: &str| cascaded.iter().position(|x| x == name).unwrap();
        assert!(pos("syn") < pos("core"));
        assert!(pos("core") < pos("shared"));
    }

    #[test]
    fn test_published_names() {
        let names = published_names();
        assert_eq!(names.get("core"), Some(&"macroforge"));
        assert_eq!(names.get("shared"), Some(&"@macroforge/shared"));
        assert_eq!(names.get("syn"), Some(&"macroforge_ts_syn"));
    }
}
