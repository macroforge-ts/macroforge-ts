//! Shared configuration management

use crate::core::deps;
use crate::core::repos::{self, Repo};
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
}

impl Config {
    /// Load configuration from project root
    pub fn load() -> Result<Self> {
        let root = find_root()?;

        let repos = repos::build_repos_map(&root);
        let deps_map = deps::load_deps(&root)?;

        Ok(Self {
            root,
            repos,
            deps: deps_map,
        })
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
