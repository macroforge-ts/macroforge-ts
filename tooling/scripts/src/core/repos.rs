//! Repository definitions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Repository type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RepoType {
    Rust,
    Ts,
    Website,
    Tooling,
    Extension,
}

/// Repository definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repo {
    pub name: String,
    pub path: String,
    pub abs_path: PathBuf,
    pub repo_type: RepoType,
    pub package_json: Option<PathBuf>,
    pub cargo_toml: Option<PathBuf>,
    /// npm package name (if different from repo name)
    pub npm_name: Option<String>,
    /// crates.io package name (if different from repo name)
    pub crate_name: Option<String>,
}

/// The part of a `package.json` that says which scripts it defines.
#[derive(Deserialize)]
struct PackageScripts {
    #[serde(default)]
    scripts: HashMap<String, String>,
}

impl Repo {
    /// Whether the repo's `package.json` defines the script `name`. A repo
    /// without a `package.json` defines none.
    pub fn has_script(&self, name: &str) -> anyhow::Result<bool> {
        use anyhow::Context;

        let Some(path) = self.package_json.as_ref().filter(|path| path.exists()) else {
            return Ok(false);
        };
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        let manifest: PackageScripts = serde_json::from_str(&text)
            .with_context(|| format!("failed to parse {}", path.display()))?;
        Ok(manifest.scripts.contains_key(name))
    }
}

/// Build the repository map
pub fn build_repos_map(root: &Path) -> HashMap<String, Repo> {
    let mut repos = HashMap::new();

    let crate_configs = [
        ("core", "macroforge_ts", "crates/macroforge_ts"),
        (
            "macros",
            "macroforge_ts_macros",
            "crates/macroforge_ts_macros",
        ),
        ("syn", "macroforge_ts_syn", "crates/macroforge_ts_syn"),
        (
            "template",
            "macroforge_ts_quote",
            "crates/macroforge_ts_quote",
        ),
    ];

    for (name, crate_name, path) in crate_configs {
        let abs_path = root.join(path);
        repos.insert(
            name.to_string(),
            Repo {
                name: name.to_string(),
                path: path.to_string(),
                abs_path: abs_path.clone(),
                repo_type: RepoType::Rust,
                package_json: if name == "core" {
                    Some(abs_path.join("package.json"))
                } else {
                    None
                },
                cargo_toml: Some(abs_path.join("Cargo.toml")),
                npm_name: if name == "core" {
                    Some("@macroforge/core".to_string())
                } else {
                    None
                },
                crate_name: Some(crate_name.to_string()),
            },
        );
    }

    let pkg_configs = [
        ("shared", "@macroforge/shared", "packages/shared"),
        (
            "typescript-plugin",
            "@macroforge/typescript-plugin",
            "packages/typescript-plugin",
        ),
        (
            "vite-plugin",
            "@macroforge/vite-plugin",
            "packages/vite-plugin",
        ),
        (
            "svelte-preprocessor",
            "@macroforge/svelte-preprocessor",
            "packages/svelte-preprocessor",
        ),
        (
            "svelte-language-server",
            "@macroforge/svelte-language-server",
            "packages/svelte-language-server",
        ),
        (
            "mcp-server",
            "@macroforge/mcp-server",
            "packages/mcp-server",
        ),
        (
            "deno-plugin",
            "@macroforge/deno-plugin",
            "packages/deno-plugin",
        ),
    ];

    for (name, npm_name, path) in pkg_configs {
        let abs_path = root.join(path);
        repos.insert(
            name.to_string(),
            Repo {
                name: name.to_string(),
                path: path.to_string(),
                abs_path: abs_path.clone(),
                repo_type: RepoType::Ts,
                package_json: Some(abs_path.join("package.json")),
                cargo_toml: None,
                npm_name: Some(npm_name.to_string()),
                crate_name: None,
            },
        );
    }

    let other_configs = [
        ("website", RepoType::Website, "website", true),
        ("tooling", RepoType::Tooling, "tooling", true),
        (
            "zed-extensions",
            RepoType::Extension,
            "crates/extensions",
            false,
        ),
    ];

    for (name, repo_type, path, has_package_json) in other_configs {
        let abs_path = root.join(path);
        repos.insert(
            name.to_string(),
            Repo {
                name: name.to_string(),
                path: path.to_string(),
                abs_path: abs_path.clone(),
                repo_type,
                package_json: has_package_json.then(|| abs_path.join("package.json")),
                cargo_toml: None,
                npm_name: None,
                crate_name: None,
            },
        );
    }

    repos
}
