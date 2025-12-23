//! Shared configuration management

use crate::core::deps;
use crate::core::repos::{self, EnvConfig, Repo, RepoType};
use crate::core::versions::VersionsCache;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::PathBuf;

/// Main configuration struct
#[derive(Debug, Clone)]
pub struct Config {
    /// Project root directory
    pub root: PathBuf,
    /// All repository definitions
    pub repos: HashMap<String, Repo>,
    /// Dependency graph (repo -> dependencies)
    pub deps: HashMap<String, Vec<String>>,
    /// Version cache
    pub versions: VersionsCache,
}

impl Config {
    /// Load configuration from project root
    pub fn load() -> Result<Self> {
        let root = find_root()?;

        // Load .env file from tooling directory
        let env_config = EnvConfig::load(&root);

        let repos = repos::build_repos_map(&root, &env_config);
        let deps_map = deps::load_deps(&root)?;
        let versions = VersionsCache::load(&root)?;

        Ok(Self {
            root,
            repos,
            deps: deps_map,
            versions,
        })
    }

    /// Get repos filtered by type or name list
    pub fn filter_repos(&self, filter: &str) -> Vec<&Repo> {
        match filter {
            "all" => self.repos.values().collect(),
            "rust" => self
                .repos
                .values()
                .filter(|r| r.repo_type == RepoType::Rust)
                .collect(),
            "ts" => self
                .repos
                .values()
                .filter(|r| r.repo_type == RepoType::Ts)
                .collect(),
            names => names
                .split(',')
                .filter_map(|n| self.repos.get(n.trim()))
                .collect(),
        }
    }
}

/// Find project root by looking for pixi.toml
pub fn find_root() -> Result<PathBuf> {
    // Check MACROFORGE_ROOT env var first
    if let Ok(root) = std::env::var("MACROFORGE_ROOT") {
        let path = PathBuf::from(root);
        if path.exists() {
            return Ok(path);
        }
    }

    let cwd = std::env::current_dir().context("Failed to get current directory")?;

    for ancestor in cwd.ancestors() {
        if ancestor.join("pixi.toml").exists() {
            return Ok(ancestor.to_path_buf());
        }
    }

    anyhow::bail!("Project root not found (no pixi.toml in ancestors)")
}

/// npm package name mappings (repo name -> npm package name)
pub fn npm_package_names() -> HashMap<&'static str, &'static str> {
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

/// crates.io package name mappings (repo name -> crate name)
pub fn crate_package_names() -> HashMap<&'static str, &'static str> {
    [
        ("syn", "macroforge_ts_syn"),
        ("template", "macroforge_ts_quote"),
        ("macros", "macroforge_ts_macros"),
    ]
    .into_iter()
    .collect()
}

/// Internal crate dependencies for swap-local/swap-registry
pub const INTERNAL_CRATES: &[(&str, &str, &str)] = &[
    ("macroforge_ts_syn", "syn", "../macroforge_ts_syn"),
    ("macroforge_ts_quote", "template", "../macroforge_ts_quote"),
    ("macroforge_ts_macros", "macros", "../macroforge_ts_macros"),
];

/// Platform-specific binary package names
pub const PLATFORMS: &[&str] = &[
    "darwin-x64",
    "darwin-arm64",
    "linux-x64-gnu",
    "linux-arm64-gnu",
    "win32-x64-msvc",
    "win32-arm64-msvc",
];
