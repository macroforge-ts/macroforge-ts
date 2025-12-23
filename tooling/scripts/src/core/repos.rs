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

/// Environment configuration loaded from .env file
#[derive(Debug, Clone, Default)]
pub struct EnvConfig {
    vars: HashMap<String, String>,
}

impl EnvConfig {
    /// Load from tooling/.env file
    pub fn load(root: &Path) -> Self {
        let mut vars = HashMap::new();
        let env_path = root.join("tooling/.env");

        if let Ok(content) = std::fs::read_to_string(&env_path) {
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some((key, value)) = line.split_once('=') {
                    vars.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
        }

        Self { vars }
    }

    /// Get a path from config, falling back to default
    fn path(&self, key: &str, default: PathBuf) -> PathBuf {
        self.vars
            .get(key)
            .map(PathBuf::from)
            .unwrap_or(default)
    }
}

/// Build the repository map
pub fn build_repos_map(root: &Path, env: &EnvConfig) -> HashMap<String, Repo> {
    let mut repos = HashMap::new();

    // Rust crates - use env vars for paths
    let crate_configs = [
        ("core", "MACROFORGE_TS_CRATE", "macroforge_ts", "crates/macroforge_ts"),
        ("macros", "MACROFORGE_TS_MACROS_CRATE", "macroforge_ts_macros", "crates/macroforge_ts_macros"),
        ("syn", "MACROFORGE_TS_SYN_CRATE", "macroforge_ts_syn", "crates/macroforge_ts_syn"),
        ("template", "MACROFORGE_TS_QUOTE_CRATE", "macroforge_ts_quote", "crates/macroforge_ts_quote"),
    ];

    for (name, env_var, crate_name, default_path) in crate_configs {
        let abs_path = env.path(env_var, root.join(default_path));
        let rel_path = abs_path.strip_prefix(root).unwrap_or(&abs_path);
        repos.insert(
            name.to_string(),
            Repo {
                name: name.to_string(),
                path: rel_path.to_string_lossy().to_string(),
                abs_path: abs_path.clone(),
                repo_type: RepoType::Rust,
                package_json: if name == "core" {
                    Some(abs_path.join("package.json"))
                } else {
                    None
                },
                cargo_toml: Some(abs_path.join("Cargo.toml")),
                npm_name: if name == "core" {
                    Some("macroforge".to_string())
                } else {
                    None
                },
                crate_name: Some(crate_name.to_string()),
            },
        );
    }

    // TypeScript packages - use env vars for paths
    let pkg_configs = [
        ("shared", "SHARED_PKG", "@macroforge/shared", "packages/shared"),
        ("typescript-plugin", "TYPESCRIPT_PLUGIN_PKG", "@macroforge/typescript-plugin", "packages/typescript-plugin"),
        ("vite-plugin", "VITE_PLUGIN_PKG", "@macroforge/vite-plugin", "packages/vite-plugin"),
        ("svelte-preprocessor", "SVELTE_PREPROCESSOR_PKG", "@macroforge/svelte-preprocessor", "packages/svelte-preprocessor"),
        ("svelte-language-server", "SVELTE_LANGUAGE_SERVER_PKG", "@macroforge/svelte-language-server", "packages/svelte-language-server"),
        ("mcp-server", "MCP_SERVER_PKG", "@macroforge/mcp-server", "packages/mcp-server"),
    ];

    for (name, env_var, npm_name, default_path) in pkg_configs {
        let abs_path = env.path(env_var, root.join(default_path));
        let rel_path = abs_path.strip_prefix(root).unwrap_or(&abs_path);
        repos.insert(
            name.to_string(),
            Repo {
                name: name.to_string(),
                path: rel_path.to_string_lossy().to_string(),
                abs_path: abs_path.clone(),
                repo_type: RepoType::Ts,
                package_json: Some(abs_path.join("package.json")),
                cargo_toml: None,
                npm_name: Some(npm_name.to_string()),
                crate_name: None,
            },
        );
    }

    // Website
    let website_path = env.path("WEBSITE_ROOT", root.join("website"));
    let website_rel = website_path.strip_prefix(root).unwrap_or(&website_path);
    repos.insert(
        "website".to_string(),
        Repo {
            name: "website".to_string(),
            path: website_rel.to_string_lossy().to_string(),
            abs_path: website_path.clone(),
            repo_type: RepoType::Website,
            package_json: Some(website_path.join("package.json")),
            cargo_toml: None,
            npm_name: None,
            crate_name: None,
        },
    );

    // Tooling
    let tooling_path = env.path("TOOLING_ROOT", root.join("tooling"));
    let tooling_rel = tooling_path.strip_prefix(root).unwrap_or(&tooling_path);
    repos.insert(
        "tooling".to_string(),
        Repo {
            name: "tooling".to_string(),
            path: tooling_rel.to_string_lossy().to_string(),
            abs_path: tooling_path.clone(),
            repo_type: RepoType::Tooling,
            package_json: Some(tooling_path.join("package.json")),
            cargo_toml: None,
            npm_name: None,
            crate_name: None,
        },
    );

    // Zed extensions
    let ext_path = env.path("EXTENSIONS_ROOT", root.join("crates/extensions"));
    let ext_rel = ext_path.strip_prefix(root).unwrap_or(&ext_path);
    repos.insert(
        "zed-extensions".to_string(),
        Repo {
            name: "zed-extensions".to_string(),
            path: ext_rel.to_string_lossy().to_string(),
            abs_path: ext_path.clone(),
            repo_type: RepoType::Extension,
            package_json: None,
            cargo_toml: None,
            npm_name: None,
            crate_name: None,
        },
    );

    repos
}
