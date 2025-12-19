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
//! petgraph = "0.6"
//! colored = "2"
//! ctrlc = "3.4"
//! ```

use clap::Parser;
use colored::*;
use petgraph::algo::toposort;
use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
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
    sync_latest: bool,
}

// ============================================================================
// Repository Configuration
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    package_json: Option<&'static str>,
    cargo_toml: Option<&'static str>,
}

/// All repository definitions
fn all_repos() -> Vec<Repo> {
    vec![
        // Rust crates
        Repo {
            name: "core",
            path: "crates/macroforge_ts",
            repo_type: RepoType::Rust,
            package_json: Some("crates/macroforge_ts/package.json"),
            cargo_toml: Some("crates/macroforge_ts/Cargo.toml"),
        },
        Repo {
            name: "macros",
            path: "crates/macroforge_ts_macros",
            repo_type: RepoType::Rust,
            package_json: None,
            cargo_toml: Some("crates/macroforge_ts_macros/Cargo.toml"),
        },
        Repo {
            name: "syn",
            path: "crates/macroforge_ts_syn",
            repo_type: RepoType::Rust,
            package_json: None,
            cargo_toml: Some("crates/macroforge_ts_syn/Cargo.toml"),
        },
        Repo {
            name: "template",
            path: "crates/macroforge_ts_quote",
            repo_type: RepoType::Rust,
            package_json: None,
            cargo_toml: Some("crates/macroforge_ts_quote/Cargo.toml"),
        },
        // TypeScript packages
        Repo {
            name: "shared",
            path: "packages/shared",
            repo_type: RepoType::Ts,
            package_json: Some("packages/shared/package.json"),
            cargo_toml: None,
        },
        Repo {
            name: "vite-plugin",
            path: "packages/vite-plugin",
            repo_type: RepoType::Ts,
            package_json: Some("packages/vite-plugin/package.json"),
            cargo_toml: None,
        },
        Repo {
            name: "typescript-plugin",
            path: "packages/typescript-plugin",
            repo_type: RepoType::Ts,
            package_json: Some("packages/typescript-plugin/package.json"),
            cargo_toml: None,
        },
        Repo {
            name: "svelte-language-server",
            path: "packages/svelte-language-server",
            repo_type: RepoType::Ts,
            package_json: Some("packages/svelte-language-server/package.json"),
            cargo_toml: None,
        },
        Repo {
            name: "svelte-preprocessor",
            path: "packages/svelte-preprocessor",
            repo_type: RepoType::Ts,
            package_json: Some("packages/svelte-preprocessor/package.json"),
            cargo_toml: None,
        },
        Repo {
            name: "mcp-server",
            path: "packages/mcp-server",
            repo_type: RepoType::Ts,
            package_json: Some("packages/mcp-server/package.json"),
            cargo_toml: None,
        },
        // Other
        Repo {
            name: "website",
            path: "website",
            repo_type: RepoType::Website,
            package_json: Some("website/package.json"),
            cargo_toml: None,
        },
        Repo {
            name: "tooling",
            path: "tooling",
            repo_type: RepoType::Tooling,
            package_json: None,
            cargo_toml: None,
        },
        Repo {
            name: "zed-extensions",
            path: "crates/extensions",
            repo_type: RepoType::Extension,
            package_json: None,
            cargo_toml: None,
        },
    ]
}

/// Dependency map: which repos depend on which (dependency -> dependents)
fn dependency_map() -> HashMap<&'static str, Vec<&'static str>> {
    let mut deps: HashMap<&str, Vec<&str>> = HashMap::new();

    // Format: dependent -> dependencies
    // We'll invert this for the graph
    let dependents = [
        ("shared", vec!["core"]),
        ("vite-plugin", vec!["core", "shared"]),
        ("typescript-plugin", vec!["core", "shared"]),
        ("svelte-language-server", vec!["core", "typescript-plugin"]),
        ("svelte-preprocessor", vec!["core"]),
        ("mcp-server", vec!["core"]),
        ("website", vec!["core"]),
        ("zed-extensions", vec!["typescript-plugin", "svelte-language-server"]),
    ];

    for (dependent, dependencies) in dependents {
        deps.insert(dependent, dependencies);
    }

    deps
}

/// Map package names to their npm package names for dependency resolution
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

/// Map repo names to their published package/crate names
fn published_names() -> HashMap<&'static str, &'static str> {
    [
        // npm packages
        ("core", "macroforge"),
        ("shared", "@macroforge/shared"),
        ("vite-plugin", "@macroforge/vite-plugin"),
        ("typescript-plugin", "@macroforge/typescript-plugin"),
        ("svelte-language-server", "@macroforge/svelte-language-server"),
        ("svelte-preprocessor", "@macroforge/svelte-preprocessor"),
        ("mcp-server", "@macroforge/mcp-server"),
        // crates
        ("syn", "macroforge_ts_syn"),
        ("template", "macroforge_ts_quote"),
        ("macros", "macroforge_ts_macros"),
        // other
        ("website", "macroforge.dev"),
        ("zed-extensions", "zed extensions"),
    ]
    .into_iter()
    .collect()
}

// ============================================================================
// Dependency Graph
// ============================================================================

#[allow(dead_code)]
struct DependencyGraph {
    graph: DiGraph<String, ()>,
    node_indices: HashMap<String, NodeIndex>,
}

impl DependencyGraph {
    fn new(repos: &[Repo]) -> Self {
        let mut graph = DiGraph::new();
        let mut node_indices = HashMap::new();

        // Add all repos as nodes
        for repo in repos {
            let idx = graph.add_node(repo.name.to_string());
            node_indices.insert(repo.name.to_string(), idx);
        }

        // Add edges based on dependency map
        let deps = dependency_map();
        for repo in repos {
            if let Some(dependencies) = deps.get(repo.name) {
                let dependent_idx = node_indices[repo.name];
                for dep in dependencies {
                    if let Some(&dep_idx) = node_indices.get(*dep) {
                        // Edge from dependency -> dependent (so toposort gives us build order)
                        graph.add_edge(dep_idx, dependent_idx, ());
                    }
                }
            }
        }

        Self { graph, node_indices }
    }

    /// Returns repos in topological order (dependencies first)
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

    /// Get dependencies for a repo (what it depends on)
    fn get_dependencies(&self, repo_name: &str) -> Vec<String> {
        let deps = dependency_map();
        deps.get(repo_name)
            .map(|d| d.iter().map(|s| s.to_string()).collect())
            .unwrap_or_default()
    }

    /// Get all repos that depend on the given repo (direct dependents)
    fn get_dependents(&self, repo_name: &str) -> Vec<String> {
        let deps = dependency_map();
        deps.iter()
            .filter(|(_, dependencies)| dependencies.contains(&repo_name))
            .map(|(dependent, _)| dependent.to_string())
            .collect()
    }

    /// Compute all repos that should be updated when the given repos are bumped.
    /// This includes the original repos plus all transitive dependents (cascade).
    fn cascade_to_dependents(&self, repo_names: &[&str]) -> Vec<String> {
        let mut result: std::collections::HashSet<String> = repo_names.iter().map(|s| s.to_string()).collect();
        let mut to_process: Vec<String> = repo_names.iter().map(|s| s.to_string()).collect();

        while let Some(repo) = to_process.pop() {
            for dependent in self.get_dependents(&repo) {
                if result.insert(dependent.clone()) {
                    to_process.push(dependent);
                }
            }
        }

        // Return in topological order
        match self.sorted_order() {
            Ok(sorted) => sorted.into_iter().filter(|r| result.contains(r)).collect(),
            Err(_) => result.into_iter().collect(),
        }
    }
}

// ============================================================================
// Version Management
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

    fn save(&self, root: &Path) -> std::io::Result<()> {
        let path = root.join("tooling/versions.json");
        let content = serde_json::to_string_pretty(&self.0)?;
        fs::write(&path, content + "\n")?;
        println!("  Updated tooling/versions.json");
        Ok(())
    }

    fn get(&self, name: &str) -> Option<&String> {
        self.0.get(name)
    }

    fn set(&mut self, name: &str, version: &str) {
        self.0.insert(name.to_string(), version.to_string());
    }
}

fn increment_patch(version: &str) -> String {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() == 3 {
        let patch: u32 = parts[2].parse().unwrap_or(0);
        format!("{}.{}.{}", parts[0], parts[1], patch + 1)
    } else {
        version.to_string()
    }
}

fn get_repo_version(root: &Path, repo: &Repo) -> Option<String> {
    if let Some(pkg_path) = repo.package_json {
        let full_path = root.join(pkg_path);
        if full_path.exists() {
            if let Ok(content) = fs::read_to_string(&full_path) {
                if let Ok(pkg) = serde_json::from_str::<serde_json::Value>(&content) {
                    return pkg.get("version").and_then(|v| v.as_str()).map(String::from);
                }
            }
        }
    }

    if let Some(cargo_path) = repo.cargo_toml {
        let full_path = root.join(cargo_path);
        if full_path.exists() {
            if let Ok(content) = fs::read_to_string(&full_path) {
                for line in content.lines() {
                    if line.starts_with("version = \"") {
                        return line
                            .trim_start_matches("version = \"")
                            .trim_end_matches('"')
                            .to_string()
                            .into();
                    }
                }
            }
        }
    }

    None
}

// ============================================================================
// Package.json Manipulation
// ============================================================================

/// Replace version dependencies with file: references for local packages
fn use_local_deps(root: &Path, repo: &Repo, local_repos: &[&str]) -> std::io::Result<serde_json::Value> {
    let pkg_path = match repo.package_json {
        Some(p) => root.join(p),
        None => return Ok(serde_json::Value::Null),
    };

    if !pkg_path.exists() {
        return Ok(serde_json::Value::Null);
    }

    let content = fs::read_to_string(&pkg_path)?;
    let mut pkg: serde_json::Value = serde_json::from_str(&content)?;
    let npm_names = npm_package_names();
    let all = all_repos();

    // Get the current package's directory for relative path calculation
    let pkg_dir = pkg_path.parent().unwrap();

    for dep_name in local_repos {
        let npm_name = match npm_names.get(dep_name) {
            Some(n) => *n,
            None => continue,
        };

        // Find the repo to get its path
        let dep_repo = match all.iter().find(|r| r.name == *dep_name) {
            Some(r) => r,
            None => continue,
        };

        // Calculate relative path from this package to the dependency
        let dep_path = root.join(dep_repo.path);
        let relative_path = pathdiff_relative(pkg_dir, &dep_path);
        let file_ref = format!("file:{}", relative_path);

        // Update dependencies
        if let Some(deps) = pkg.get_mut("dependencies") {
            if deps.get(npm_name).is_some() {
                deps[npm_name] = serde_json::Value::String(file_ref.clone());
            }
        }

        // Update peerDependencies
        if let Some(deps) = pkg.get_mut("peerDependencies") {
            if deps.get(npm_name).is_some() {
                deps[npm_name] = serde_json::Value::String(file_ref.clone());
            }
        }
    }

    // Write modified package.json
    fs::write(&pkg_path, serde_json::to_string_pretty(&pkg)? + "\n")?;

    Ok(pkg)
}

/// Restore version dependencies from versions cache
fn restore_version_deps(root: &Path, repo: &Repo, versions: &VersionsCache) -> std::io::Result<()> {
    let pkg_path = match repo.package_json {
        Some(p) => root.join(p),
        None => return Ok(()),
    };

    if !pkg_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&pkg_path)?;
    let mut pkg: serde_json::Value = serde_json::from_str(&content)?;

    let dep_mappings = [
        ("core", "macroforge"),
        ("shared", "@macroforge/shared"),
        ("typescript-plugin", "@macroforge/typescript-plugin"),
    ];

    for (repo_name, npm_name) in dep_mappings {
        if let Some(version) = versions.get(repo_name) {
            let version_ref = format!("^{}", version);

            if let Some(deps) = pkg.get_mut("dependencies") {
                if deps.get(npm_name).is_some() {
                    deps[npm_name] = serde_json::Value::String(version_ref.clone());
                }
            }

            if let Some(deps) = pkg.get_mut("peerDependencies") {
                if deps.get(npm_name).is_some() {
                    deps[npm_name] = serde_json::Value::String(version_ref);
                }
            }
        }
    }

    fs::write(&pkg_path, serde_json::to_string_pretty(&pkg)? + "\n")?;
    Ok(())
}

/// Simple relative path calculation
fn pathdiff_relative(from: &Path, to: &Path) -> String {
    // Normalize both paths
    let from = from.canonicalize().unwrap_or_else(|_| from.to_path_buf());
    let to = to.canonicalize().unwrap_or_else(|_| to.to_path_buf());

    // Find common prefix
    let from_components: Vec<_> = from.components().collect();
    let to_components: Vec<_> = to.components().collect();

    let mut common_len = 0;
    for (a, b) in from_components.iter().zip(to_components.iter()) {
        if a == b {
            common_len += 1;
        } else {
            break;
        }
    }

    // Build relative path
    let ups = from_components.len() - common_len;
    let mut result = String::new();
    for _ in 0..ups {
        result.push_str("../");
    }
    for component in &to_components[common_len..] {
        result.push_str(&component.as_os_str().to_string_lossy());
        result.push('/');
    }

    result.trim_end_matches('/').to_string()
}

// ============================================================================
// Version Updates
// ============================================================================

fn update_package_json(root: &Path, pkg_path: &str, version: &str, versions: &VersionsCache) -> std::io::Result<()> {
    let full_path = root.join(pkg_path);
    if !full_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&full_path)?;
    let mut pkg: serde_json::Value = serde_json::from_str(&content)?;

    pkg["version"] = serde_json::Value::String(version.to_string());

    // Update dependencies
    if let Some(deps) = pkg.get_mut("dependencies") {
        if deps.get("macroforge").is_some() {
            if let Some(v) = versions.get("core") {
                deps["macroforge"] = serde_json::Value::String(format!("^{}", v));
            }
        }
        if deps.get("@macroforge/shared").is_some() {
            if let Some(v) = versions.get("shared") {
                deps["@macroforge/shared"] = serde_json::Value::String(format!("^{}", v));
            }
        }
        if deps.get("@macroforge/typescript-plugin").is_some() {
            if let Some(v) = versions.get("typescript-plugin") {
                deps["@macroforge/typescript-plugin"] = serde_json::Value::String(format!("^{}", v));
            }
        }
    }

    // Update peerDependencies
    if let Some(deps) = pkg.get_mut("peerDependencies") {
        if deps.get("macroforge").is_some() {
            if let Some(v) = versions.get("core") {
                deps["macroforge"] = serde_json::Value::String(format!("^{}", v));
            }
        }
    }

    // Update optionalDependencies (platform packages for core)
    if let Some(deps) = pkg.get_mut("optionalDependencies") {
        let platforms = [
            "darwin-x64",
            "darwin-arm64",
            "linux-x64-gnu",
            "linux-arm64-gnu",
            "win32-x64-msvc",
            "win32-arm64-msvc",
        ];
        for platform in platforms {
            let key = format!("@macroforge/bin-{}", platform);
            if deps.get(&key).is_some() {
                deps[&key] = serde_json::Value::String(version.to_string());
            }
        }
    }

    fs::write(&full_path, serde_json::to_string_pretty(&pkg)? + "\n")?;
    println!("  Updated {}", pkg_path);
    Ok(())
}

fn update_cargo_toml(root: &Path, cargo_path: &str, version: &str, versions: &VersionsCache) -> std::io::Result<()> {
    let full_path = root.join(cargo_path);
    if !full_path.exists() {
        return Ok(());
    }

    let mut content = fs::read_to_string(&full_path)?;

    // Update version line
    content = content
        .lines()
        .map(|line| {
            if line.starts_with("version = \"") {
                format!("version = \"{}\"", version)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Update crate dependencies
    if let Some(v) = versions.get("macros") {
        content = content.replace(
            &format!("macroforge_ts_macros = \"{}\"", v),
            &format!("macroforge_ts_macros = \"{}\"", version),
        );
    }
    if let Some(v) = versions.get("syn") {
        content = content.replace(
            &format!("macroforge_ts_syn = \"{}\"", v),
            &format!("macroforge_ts_syn = \"{}\"", version),
        );
    }
    if let Some(v) = versions.get("template") {
        content = content.replace(
            &format!("macroforge_ts_quote = \"{}\"", v),
            &format!("macroforge_ts_quote = \"{}\"", version),
        );
    }

    fs::write(&full_path, content)?;
    println!("  Updated {}", cargo_path);
    Ok(())
}

// ============================================================================
// Dependency Swapping (registry <-> local path)
// ============================================================================

/// Internal crate dependencies that need swapping
const INTERNAL_CRATES: &[(&str, &str, &str)] = &[
    ("macroforge_ts_syn", "syn", "../macroforge_ts_syn"),
    ("macroforge_ts_quote", "template", "../macroforge_ts_quote"),
    ("macroforge_ts_macros", "macros", "../macroforge_ts_macros"),
];

/// Swap registry dependencies to local path dependencies for building
fn swap_to_local_deps(root: &Path) -> std::io::Result<()> {
    let cargo_path = root.join("crates/macroforge_ts/Cargo.toml");
    if !cargo_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&cargo_path)?;
    let mut new_lines = Vec::new();

    for line in content.lines() {
        let mut replaced = false;
        for (crate_name, _, local_path) in INTERNAL_CRATES {
            // Match: crate_name = "version"
            if line.trim().starts_with(&format!("{} = \"", crate_name)) {
                new_lines.push(format!("{} = {{ path = \"{}\" }}", crate_name, local_path));
                replaced = true;
                break;
            }
        }
        if !replaced {
            new_lines.push(line.to_string());
        }
    }

    fs::write(&cargo_path, new_lines.join("\n") + "\n")?;
    Ok(())
}

/// Swap local path dependencies back to registry dependencies
fn swap_to_registry_deps(root: &Path, versions: &VersionsCache) -> std::io::Result<()> {
    let cargo_path = root.join("crates/macroforge_ts/Cargo.toml");
    if !cargo_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&cargo_path)?;
    let mut new_lines = Vec::new();

    for line in content.lines() {
        let mut replaced = false;
        for (crate_name, repo_name, local_path) in INTERNAL_CRATES {
            // Match: crate_name = { path = "..." }
            if line.trim().starts_with(&format!("{} = {{ path = \"{}\"", crate_name, local_path)) {
                let version = versions.get(repo_name).cloned().unwrap_or_else(|| "0.1.0".to_string());
                new_lines.push(format!("{} = \"{}\"", crate_name, version));
                replaced = true;
                break;
            }
        }
        if !replaced {
            new_lines.push(line.to_string());
        }
    }

    fs::write(&cargo_path, new_lines.join("\n") + "\n")?;
    Ok(())
}

/// Check if currently using local path dependencies
fn is_using_local_deps(root: &Path) -> bool {
    let cargo_path = root.join("crates/macroforge_ts/Cargo.toml");
    if let Ok(content) = fs::read_to_string(&cargo_path) {
        content.contains("path = \"../macroforge_ts_")
    } else {
        false
    }
}

fn update_repo_version(root: &Path, repo: &Repo, version: &str, versions: &VersionsCache) -> std::io::Result<()> {
    println!("\n[{}] -> {}", repo.name.cyan(), version.green());

    if let Some(pkg_path) = repo.package_json {
        update_package_json(root, pkg_path, version, versions)?;
    }

    if let Some(cargo_path) = repo.cargo_toml {
        update_cargo_toml(root, cargo_path, version, versions)?;
    }

    // Special handling for core - update platform packages
    if repo.name == "core" {
        let platforms = [
            "darwin-x64",
            "darwin-arm64",
            "linux-x64-gnu",
            "linux-arm64-gnu",
            "win32-x64-msvc",
            "win32-arm64-msvc",
        ];
        for platform in platforms {
            let platform_pkg_path = format!("crates/macroforge_ts/npm/{}/package.json", platform);
            let full_path = root.join(&platform_pkg_path);
            if full_path.exists() {
                let content = fs::read_to_string(&full_path)?;
                let mut pkg: serde_json::Value = serde_json::from_str(&content)?;
                pkg["version"] = serde_json::Value::String(version.to_string());
                fs::write(&full_path, serde_json::to_string_pretty(&pkg)? + "\n")?;
                println!("  Updated {}", platform_pkg_path);
            }
        }
    }

    // Special handling for zed-extensions - update lib.rs version constants
    if repo.name == "zed-extensions" {
        update_zed_extensions(root, versions)?;
    }

    Ok(())
}

/// Update version constants in zed extension lib.rs files
fn update_zed_extensions(root: &Path, versions: &VersionsCache) -> std::io::Result<()> {
    // Update vtsls-macroforge extension
    let vtsls_lib_path = root.join("crates/extensions/vtsls-macroforge/src/lib.rs");
    if vtsls_lib_path.exists() {
        let mut content = fs::read_to_string(&vtsls_lib_path)?;

        // Update TS_PLUGIN_VERSION with typescript-plugin version
        if let Some(ts_plugin_ver) = versions.get("typescript-plugin") {
            let re = regex_replace(&content, r#"const TS_PLUGIN_VERSION: &str = "[^"]+";"#,
                &format!(r#"const TS_PLUGIN_VERSION: &str = "{}";"#, ts_plugin_ver));
            content = re;
        }

        // Update MACROFORGE_VERSION with core version
        if let Some(core_ver) = versions.get("core") {
            let re = regex_replace(&content, r#"const MACROFORGE_VERSION: &str = "[^"]+";"#,
                &format!(r#"const MACROFORGE_VERSION: &str = "{}";"#, core_ver));
            content = re;
        }

        fs::write(&vtsls_lib_path, content)?;
        println!("  Updated crates/extensions/vtsls-macroforge/src/lib.rs");
    }

    // Update svelte-macroforge extension
    let svelte_lib_path = root.join("crates/extensions/svelte-macroforge/src/lib.rs");
    if svelte_lib_path.exists() {
        let mut content = fs::read_to_string(&svelte_lib_path)?;

        // Update SVELTE_LS_VERSION with svelte-language-server version
        if let Some(svelte_ls_ver) = versions.get("svelte-language-server") {
            let re = regex_replace(&content, r#"const SVELTE_LS_VERSION: &str = "[^"]+";"#,
                &format!(r#"const SVELTE_LS_VERSION: &str = "{}";"#, svelte_ls_ver));
            content = re;

            // Also update the test assertion
            let re2 = regex_replace(&content, r#"assert_eq!\(SVELTE_LS_VERSION, "[^"]+"\);"#,
                &format!(r#"assert_eq!(SVELTE_LS_VERSION, "{}");"#, svelte_ls_ver));
            content = re2;
        }

        fs::write(&svelte_lib_path, content)?;
        println!("  Updated crates/extensions/svelte-macroforge/src/lib.rs");
    }

    Ok(())
}

/// Simple regex replacement without the regex crate
fn regex_replace(text: &str, pattern: &str, replacement: &str) -> String {
    // For our specific patterns, we can use simple string matching
    // Pattern format: const NAME: &str = "...";
    let lines: Vec<&str> = text.lines().collect();
    let mut result = Vec::new();

    for line in lines {
        // Extract the constant name from the pattern
        if pattern.starts_with(r#"const "#) && pattern.contains(": &str = ") {
            // Extract constant name
            let const_name = pattern
                .strip_prefix(r#"const "#)
                .and_then(|s| s.split(':').next())
                .unwrap_or("");

            if line.trim().starts_with(&format!("const {}", const_name)) && line.contains(": &str = ") {
                result.push(replacement.to_string());
                continue;
            }
        } else if pattern.starts_with("assert_eq!(") {
            // Handle assert_eq! pattern
            let fn_name = pattern
                .strip_prefix("assert_eq!(")
                .and_then(|s| s.split(',').next())
                .unwrap_or("");

            if line.trim().starts_with(&format!("assert_eq!({}", fn_name)) {
                result.push(format!("        {}", replacement));
                continue;
            }
        }
        result.push(line.to_string());
    }

    // Preserve trailing newline if original had one
    let mut output = result.join("\n");
    if text.ends_with('\n') && !output.ends_with('\n') {
        output.push('\n');
    }
    output
}

// ============================================================================
// Build Steps
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

fn build_repo(root: &Path, repo: &Repo, built_repos: &[&str], graph: &DependencyGraph) -> Result<(), String> {
    match repo.repo_type {
        RepoType::Rust => {
            if repo.name == "core" {
                run_cmd("npm run cleanbuild", &root.join(repo.path))?;
            }
            // Other rust crates don't need explicit build here
        }
        RepoType::Ts => {
            let repo_path = root.join(repo.path);

            // Get local dependencies that need file: references
            let deps = graph.get_dependencies(repo.name);
            let local_deps: Vec<&str> = deps
                .iter()
                .filter(|d| built_repos.contains(&d.as_str()))
                .map(|s| s.as_str())
                .collect();

            // Temporarily use file: references for local deps
            if !local_deps.is_empty() {
                println!("  Using local deps: {}", local_deps.join(", ").dimmed());
                use_local_deps(root, repo, &local_deps).map_err(|e| e.to_string())?;
            }

            // Check if package has a build script
            let has_build_script = repo.package_json
                .map(|p| root.join(p))
                .filter(|p| p.exists())
                .and_then(|p| fs::read_to_string(&p).ok())
                .and_then(|content| serde_json::from_str::<serde_json::Value>(&content).ok())
                .and_then(|pkg| pkg.get("scripts")?.get("build").cloned())
                .is_some();

            // Clean and build (only run build if script exists)
            let build_cmd = if has_build_script {
                "rm -rf node_modules dist && npm install && npm run build"
            } else {
                "rm -rf node_modules && npm install"
            };
            let result = run_cmd(build_cmd, &repo_path);

            // Always restore version deps after build (even on failure)
            let versions = VersionsCache::load(root);
            let _ = restore_version_deps(root, repo, &versions);

            result?;
        }
        RepoType::Website => {
            let website_path = root.join(repo.path);
            run_cmd("git pull origin", &website_path)?;
            run_cmd("rm -rf node_modules .svelte-kit && npm install && npm run build", &website_path)?;
        }
        _ => {}
    }

    Ok(())
}

// ============================================================================
// Sync Latest
// ============================================================================

/// Sync all repos to their versions from versions.json
fn sync_all_to_latest(root: &Path) -> ExitCode {
    println!("{}", "═".repeat(60));
    println!("{}", "Syncing all packages to latest versions".bold());
    println!("{}", "═".repeat(60));

    let versions = VersionsCache::load(root);
    let all = all_repos();

    let mut updated = Vec::new();
    let mut skipped = Vec::new();

    for repo in &all {
        let version = match versions.get(repo.name) {
            Some(v) => v.clone(),
            None => {
                skipped.push(repo.name);
                continue;
            }
        };

        if let Err(e) = update_repo_version(root, repo, &version, &versions) {
            eprintln!("{}: {} - {}", "Error".red(), repo.name, e);
            continue;
        }
        updated.push((repo.name, version));
    }

    // Also update zed-extensions lib.rs files
    if let Err(e) = update_zed_extensions(root, &versions) {
        eprintln!("{}: zed-extensions - {}", "Error".red(), e);
    }

    println!("\n{}", "═".repeat(60));
    println!("{}", "Summary".bold());
    println!("{}", "═".repeat(60));

    if !updated.is_empty() {
        println!("\n{}:", "Updated".green().bold());
        for (name, version) in &updated {
            println!("  {} → {}", name.cyan(), version.green());
        }
    }

    if !skipped.is_empty() {
        println!("\n{}: {}", "Skipped (no version in versions.json)".yellow(), skipped.join(", "));
    }

    println!("\n{} Run {} to apply formatting.", "Tip:".dimmed(), "cargo fmt".cyan());

    ExitCode::SUCCESS
}

// ============================================================================
// Main
// ============================================================================

fn main() -> ExitCode {
    let args = Args::parse();

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

    // Set up Ctrl+C handler
    let interrupted = Arc::new(AtomicBool::new(false));
    let interrupted_clone = interrupted.clone();
    let root_clone = root.clone();
    if let Err(e) = ctrlc::set_handler(move || {
        // Check if already interrupted (prevent double cleanup)
        if interrupted_clone.swap(true, Ordering::SeqCst) {
            eprintln!("\n{} Force exiting...", "⚠".red());
            std::process::exit(130);
        }

        eprintln!("\n{} Received Ctrl+C, cleaning up...", "⚠".yellow());

        // Restore registry deps if we were using local
        if is_using_local_deps(&root_clone) {
            eprintln!("  {} Restoring registry dependencies...", "→".yellow());
            let versions = VersionsCache::load(&root_clone);
            let _ = swap_to_registry_deps(&root_clone, &versions);
        }

        eprintln!("{} Cleanup complete. Exiting.", "✓".green());
        std::process::exit(130); // 130 = 128 + SIGINT(2)
    }) {
        eprintln!("{} Failed to set Ctrl+C handler: {}", "Warning:".yellow(), e);
    }

    // Handle --sync-latest: update all repos to their versions.json versions
    if args.sync_latest {
        return sync_all_to_latest(&root);
    }

    // Parse repos
    let all = all_repos();
    let initial_repos: Vec<Repo> = match args.repos.as_str() {
        "all" => all.clone(),
        "rust" => all.iter().filter(|r| r.repo_type == RepoType::Rust).cloned().collect(),
        "ts" => all.iter().filter(|r| r.repo_type == RepoType::Ts).cloned().collect(),
        names => names
            .split(',')
            .filter_map(|n| all.iter().find(|r| r.name == n.trim()).cloned())
            .collect(),
    };

    if initial_repos.is_empty() {
        eprintln!("{}", "No repos selected".red());
        return ExitCode::FAILURE;
    }

    // Build dependency graph with ALL repos to get the full picture
    let graph = DependencyGraph::new(&all);

    // Cascade to dependents unless --no-cascade is specified
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

    // Filter to only selected repos, in topological order
    let repos: Vec<Repo> = sorted_names
        .iter()
        .filter(|name| cascaded_names.contains(name))
        .filter_map(|name| all.iter().find(|r| r.name == name).cloned())
        .collect();

    let repo_names: Vec<&str> = repos.iter().map(|r| r.name).collect();

    // Load versions cache and track original versions for summary
    let mut versions = VersionsCache::load(&root);
    let original_versions: HashMap<String, String> = repos
        .iter()
        .filter_map(|r| {
            versions.get(r.name).map(|v| (r.name.to_string(), v.clone()))
        })
        .collect();

    // Determine version
    let base_version = if repo_names.len() == 1 {
        versions
            .get(repo_names[0])
            .cloned()
            .or_else(|| get_repo_version(&root, &repos[0]))
            .unwrap_or_else(|| "0.1.0".to_string())
    } else {
        versions
            .get("core")
            .cloned()
            .or_else(|| {
                all.iter()
                    .find(|r| r.name == "core")
                    .and_then(|r| get_repo_version(&root, r))
            })
            .unwrap_or_else(|| "0.1.0".to_string())
    };

    let version = if args.dry_run {
        println!("{}", "=".repeat(60));
        println!("{}: Testing with current versions", "DRY RUN".yellow());
        println!("Repos (in build order): {}", repo_names.join(", ").cyan());
        println!("{}", "=".repeat(60));
        base_version.clone()
    } else {
        let v = args.version.unwrap_or_else(|| {
            let new_v = increment_patch(&base_version);
            println!(
                "No version specified, incrementing {} -> {}",
                base_version, new_v.green()
            );
            new_v
        });
        println!("{}", "=".repeat(60));
        println!(
            "Preparing release {} for: {}",
            v.green(),
            repo_names.join(", ").cyan()
        );
        println!("Build order: {}", sorted_names.join(" → ").dimmed());
        println!("{}", "=".repeat(60));
        v
    };

    // Step 1: Bump versions
    if args.dry_run {
        println!("\n{}", "[1/9] Skipping version bump (dry-run)...".dimmed());
    } else {
        println!("\n{}", "[1/9] Bumping versions...".bold());

        for repo in &repos {
            versions.set(repo.name, &version);
        }

        for repo in &repos {
            if let Err(e) = update_repo_version(&root, repo, &version, &versions) {
                eprintln!("{}: {}", "Error updating version".red(), e);
                return ExitCode::FAILURE;
            }
        }

        if let Err(e) = versions.save(&root) {
            eprintln!("{}: {}", "Error saving versions".red(), e);
            return ExitCode::FAILURE;
        }
    }

    // Step 2: Extract docs
    if args.skip_docs || args.bump_only {
        println!("\n{}", "[2/9] Skipping documentation extraction...".dimmed());
    } else {
        println!("\n{}", "[2/9] Extracting API documentation...".bold());
        if let Err(e) = run_cmd("rust-script tooling/scripts/extract-rust-docs.rs", &root) {
            eprintln!("{}: {}", "Error".red(), e);
            // Don't fail on doc extraction
        }
        if let Err(e) = run_cmd("rust-script tooling/scripts/extract-ts-docs.rs", &root) {
            eprintln!("{}: {}", "Error".red(), e);
        }
    }

    // Steps 3-6: Build, fmt, clippy, test
    if args.skip_build || args.bump_only {
        println!("\n{}", "[3/9] Skipping build...".dimmed());
        println!("{}", "[4/9] Skipping fmt...".dimmed());
        println!("{}", "[5/9] Skipping clippy...".dimmed());
        println!("{}", "[6/9] Skipping tests...".dimmed());
    } else {
        println!("\n{}", "[3/9] Clean building packages (in dependency order)...".bold());

        // Swap to local path dependencies for building
        println!("  {} Swapping to local path dependencies...", "→".blue());
        if let Err(e) = swap_to_local_deps(&root) {
            eprintln!("{}: {}", "Failed to swap to local deps".red(), e);
            return ExitCode::FAILURE;
        }

        // Helper closure to restore registry deps and handle cleanup
        let cleanup = |root: &Path, versions: &VersionsCache| {
            println!("  {} Restoring registry dependencies...", "→".blue());
            let _ = swap_to_registry_deps(root, versions);
        };

        let mut built: Vec<&str> = Vec::new();
        for repo in &repos {
            // Check for interrupt before each repo
            if interrupted.load(Ordering::SeqCst) {
                eprintln!("\n{}", "Build interrupted by user".yellow());
                cleanup(&root, &versions);
                return ExitCode::FAILURE;
            }

            println!("\n  {} {}", "Building:".bold(), repo.name.cyan());
            if let Err(e) = build_repo(&root, repo, &built, &graph) {
                eprintln!("{}: {}", "Build failed".red(), e);
                cleanup(&root, &versions);
                return ExitCode::FAILURE;
            }
            built.push(repo.name);
        }

        // Rust-specific checks
        let rust_repos: Vec<_> = repos.iter().filter(|r| r.repo_type == RepoType::Rust).collect();

        println!("\n{}", "[4/9] Checking formatting...".bold());
        for repo in &rust_repos {
            if interrupted.load(Ordering::SeqCst) {
                eprintln!("\n{}", "Format check interrupted by user".yellow());
                cleanup(&root, &versions);
                return ExitCode::FAILURE;
            }
            if let Err(e) = run_cmd("cargo fmt -- --check", &root.join(repo.path)) {
                eprintln!("{}: {}", "Format check failed".red(), e);
                cleanup(&root, &versions);
                return ExitCode::FAILURE;
            }
        }

        println!("\n{}", "[5/9] Running clippy...".bold());
        for repo in &rust_repos {
            if interrupted.load(Ordering::SeqCst) {
                eprintln!("\n{}", "Clippy interrupted by user".yellow());
                cleanup(&root, &versions);
                return ExitCode::FAILURE;
            }
            if let Err(e) = run_cmd("cargo clippy -- -D warnings", &root.join(repo.path)) {
                eprintln!("{}: {}", "Clippy failed".red(), e);
                cleanup(&root, &versions);
                return ExitCode::FAILURE;
            }
        }

        println!("\n{}", "[6/9] Running tests...".bold());
        for repo in &rust_repos {
            if interrupted.load(Ordering::SeqCst) {
                eprintln!("\n{}", "Tests interrupted by user".yellow());
                cleanup(&root, &versions);
                return ExitCode::FAILURE;
            }
            if let Err(e) = run_cmd("cargo test", &root.join(repo.path)) {
                eprintln!("{}: {}", "Tests failed".red(), e);
                cleanup(&root, &versions);
                return ExitCode::FAILURE;
            }
        }

        // Restore registry deps after successful build
        cleanup(&root, &versions);
    }

    // Steps 7-8: Docs
    if args.skip_docs || args.bump_only {
        println!("\n{}", "[7/9] Skipping docs book...".dimmed());
        println!("{}", "[8/9] Skipping MCP docs sync...".dimmed());
    } else {
        println!("\n{}", "[7/9] Rebuilding docs book...".bold());
        let _ = run_cmd("rust-script tooling/scripts/build-docs-book.rs", &root);

        println!("\n{}", "[8/9] Syncing MCP server docs...".bold());
        let _ = run_cmd("npm run build:docs", &root.join("packages/mcp-server"));
    }

    // Step 9: Done
    println!("\n{}", "[9/9] Done!".bold().green());

    if args.dry_run {
        println!("\n{}", "=".repeat(60));
        println!("{}", "DRY RUN COMPLETE - no version changes were made".yellow());
        println!("{}", "=".repeat(60));
        println!(
            "\nAll builds and tests passed. To do a real release, run:\n  {}",
            format!("pixi run prep --repos {}", args.repos).cyan()
        );
    } else {
        println!("\n{}", "═".repeat(80));
        println!("{}", "Version Summary".bold());
        println!("{}", "═".repeat(80));

        let pub_names = published_names();

        // Calculate column widths
        let name_width = repos.iter().map(|r| r.name.len()).max().unwrap_or(8).max(8);
        let pub_width = repos
            .iter()
            .filter_map(|r| pub_names.get(r.name).map(|s| s.len()))
            .max()
            .unwrap_or(20)
            .max(20);
        let ver_width = 8;

        // Header
        println!(
            "  {:<nw$}  {:<pw$}  {:>vw$}  →  {:>vw$}",
            "Repo".bold(),
            "Package/Crate".bold(),
            "Before".dimmed(),
            "After".bold(),
            nw = name_width,
            pw = pub_width,
            vw = ver_width
        );
        println!("  {}", "─".repeat(name_width + pub_width + ver_width * 2 + 11));

        // Rows
        for repo in &repos {
            let pub_name = pub_names.get(repo.name).copied().unwrap_or("—");
            let before = original_versions
                .get(repo.name)
                .map(|s| s.as_str())
                .unwrap_or("(new)");
            let changed = before != version;

            if changed {
                println!(
                    "  {:<nw$}  {:<pw$}  {:>vw$}  →  {:>vw$}",
                    repo.name.cyan(),
                    pub_name,
                    before.dimmed(),
                    version.green(),
                    nw = name_width,
                    pw = pub_width,
                    vw = ver_width
                );
            } else {
                println!(
                    "  {:<nw$}  {:<pw$}  {:>vw$}  →  {:>vw$}",
                    repo.name,
                    pub_name.dimmed(),
                    before.dimmed(),
                    version.dimmed(),
                    nw = name_width,
                    pw = pub_width,
                    vw = ver_width
                );
            }
        }

        println!("\n{}", "═".repeat(80));
        println!("{} Ready to commit!", "✓".green());
        println!("{}", "═".repeat(80));

        println!("\n{}", "Next step:".bold());
        println!(
            "  {}",
            format!("pixi run commit --repos {}", args.repos).cyan()
        );
        println!();
    }

    ExitCode::SUCCESS
}
