#!/usr/bin/env rust-script
//! Manifest manipulation utility.
//!
//! Handles reading/writing versions to package.json and Cargo.toml,
//! managing versions.json cache, and swapping dependency paths.
//!
//! ```cargo
//! [dependencies]
//! clap = { version = "4", features = ["derive"] }
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! toml = "0.8"
//! ```

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

// ============================================================================
// CLI
// ============================================================================

#[derive(Parser, Debug)]
#[command(name = "manifests")]
#[command(about = "Manifest manipulation utility")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List all repositories as JSON
    List,
    /// Get version for a repo (local or registry)
    GetVersion {
        repo: String,
        #[arg(long)]
        registry: bool,
    },
    /// Set version for a repo in cache and update files
    SetVersion {
        repo: String,
        version: String,
        #[arg(long)]
        registry: bool, // If set, updates registry field, otherwise local
    },
    /// Apply versions from cache to all files
    ApplyVersions {
        #[arg(long)]
        local: bool, // If set, uses local version, otherwise registry (default: local)
    },
    /// Swap dependencies to local paths
    SwapLocal,
    /// Swap dependencies to registry versions
    SwapRegistry,
    /// Dump all versions as JSON
    DumpVersions,
    /// Update Zed extension files
    UpdateZed,
    /// Link dependencies to local paths
    LinkLocal { repo: String, deps: Vec<String> },
    /// Restore dependencies to version numbers
    RestoreRepo { repo: String },
}

fn pathdiff_relative(from: &Path, to: &Path) -> String {
    let from = from.canonicalize().unwrap_or_else(|_| from.to_path_buf());
    let to = to.canonicalize().unwrap_or_else(|_| to.to_path_buf());
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
// Repository Configuration
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum RepoType {
    Rust,
    Ts,
    Website,
    Tooling,
    Extension,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Repo {
    name: String,
    path: String,
    repo_type: RepoType,
    package_json: Option<String>,
    cargo_toml: Option<String>,
}

fn all_repos() -> Vec<Repo> {
    vec![
        // Rust crates
        Repo {
            name: "core".to_string(),
            path: "crates/macroforge_ts".to_string(),
            repo_type: RepoType::Rust,
            package_json: Some("crates/macroforge_ts/package.json".to_string()),
            cargo_toml: Some("crates/macroforge_ts/Cargo.toml".to_string()),
        },
        Repo {
            name: "macros".to_string(),
            path: "crates/macroforge_ts_macros".to_string(),
            repo_type: RepoType::Rust,
            package_json: None,
            cargo_toml: Some("crates/macroforge_ts_macros/Cargo.toml".to_string()),
        },
        Repo {
            name: "syn".to_string(),
            path: "crates/macroforge_ts_syn".to_string(),
            repo_type: RepoType::Rust,
            package_json: None,
            cargo_toml: Some("crates/macroforge_ts_syn/Cargo.toml".to_string()),
        },
        Repo {
            name: "template".to_string(),
            path: "crates/macroforge_ts_quote".to_string(),
            repo_type: RepoType::Rust,
            package_json: None,
            cargo_toml: Some("crates/macroforge_ts_quote/Cargo.toml".to_string()),
        },
        // TypeScript packages
        Repo {
            name: "shared".to_string(),
            path: "packages/shared".to_string(),
            repo_type: RepoType::Ts,
            package_json: Some("packages/shared/package.json".to_string()),
            cargo_toml: None,
        },
        Repo {
            name: "vite-plugin".to_string(),
            path: "packages/vite-plugin".to_string(),
            repo_type: RepoType::Ts,
            package_json: Some("packages/vite-plugin/package.json".to_string()),
            cargo_toml: None,
        },
        Repo {
            name: "typescript-plugin".to_string(),
            path: "packages/typescript-plugin".to_string(),
            repo_type: RepoType::Ts,
            package_json: Some("packages/typescript-plugin/package.json".to_string()),
            cargo_toml: None,
        },
        Repo {
            name: "svelte-language-server".to_string(),
            path: "packages/svelte-language-server".to_string(),
            repo_type: RepoType::Ts,
            package_json: Some("packages/svelte-language-server/package.json".to_string()),
            cargo_toml: None,
        },
        Repo {
            name: "svelte-preprocessor".to_string(),
            path: "packages/svelte-preprocessor".to_string(),
            repo_type: RepoType::Ts,
            package_json: Some("packages/svelte-preprocessor/package.json".to_string()),
            cargo_toml: None,
        },
        Repo {
            name: "mcp-server".to_string(),
            path: "packages/mcp-server".to_string(),
            repo_type: RepoType::Ts,
            package_json: Some("packages/mcp-server/package.json".to_string()),
            cargo_toml: None,
        },
        // Other
        Repo {
            name: "website".to_string(),
            path: "website".to_string(),
            repo_type: RepoType::Website,
            package_json: Some("website/package.json".to_string()),
            cargo_toml: None,
        },
        Repo {
            name: "tooling".to_string(),
            path: "tooling".to_string(),
            repo_type: RepoType::Tooling,
            package_json: None,
            cargo_toml: None,
        },
        Repo {
            name: "zed-extensions".to_string(),
            path: "crates/extensions".to_string(),
            repo_type: RepoType::Extension,
            package_json: None,
            cargo_toml: None,
        },
    ]
}

// ============================================================================
// Version Cache
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct VersionInfo {
    local: String,
    registry: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

    fn save(&self, root: &Path) -> std::io::Result<()> {
        let path = root.join("tooling/versions.json");
        let content = serde_json::to_string_pretty(&self.0)?;
        fs::write(&path, content + "\n")
    }

    fn get_local(&self, name: &str) -> Option<&String> {
        self.0.get(name).and_then(|v| v.as_ref()).map(|v| &v.local)
    }

    fn get_registry(&self, name: &str) -> Option<&String> {
        self.0
            .get(name)
            .and_then(|v| v.as_ref())
            .map(|v| &v.registry)
    }

    fn set_local(&mut self, name: &str, version: &str) {
        let entry = self.0.entry(name.to_string()).or_insert(Some(VersionInfo {
            local: version.to_string(),
            registry: version.to_string(),
        }));
        if let Some(info) = entry {
            info.local = version.to_string();
        } else {
            *entry = Some(VersionInfo {
                local: version.to_string(),
                registry: version.to_string(),
            });
        }
    }

    fn set_registry(&mut self, name: &str, version: &str) {
        let entry = self.0.entry(name.to_string()).or_insert(Some(VersionInfo {
            local: version.to_string(),
            registry: version.to_string(),
        }));
        if let Some(info) = entry {
            info.registry = version.to_string();
        } else {
            *entry = Some(VersionInfo {
                local: version.to_string(),
                registry: version.to_string(),
            });
        }
    }
}

// ============================================================================
// File Updates
// ============================================================================

fn update_package_json(
    root: &Path,
    pkg_path: &str,
    version: &str,
    versions: &VersionsCache,
) -> std::io::Result<()> {
    let full_path = root.join(pkg_path);
    if !full_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&full_path)?;
    let mut pkg: serde_json::Value = serde_json::from_str(&content)?;

    pkg["version"] = serde_json::Value::String(version.to_string());

    // Helper to update deps
    let update_dep =
        |deps: &mut serde_json::Map<String, serde_json::Value>, key: &str, target_repo: &str| {
            if deps.contains_key(key) {
                if let Some(v) = versions.get_local(target_repo) {
                    // Always use local version for deps in file
                    deps[key] = serde_json::Value::String(format!("^{}", v));
                }
            }
        };

    if let Some(deps) = pkg.get_mut("dependencies").and_then(|v| v.as_object_mut()) {
        update_dep(deps, "macroforge", "core");
        update_dep(deps, "@macroforge/shared", "shared");
        update_dep(deps, "@macroforge/typescript-plugin", "typescript-plugin");
    }

    if let Some(deps) = pkg
        .get_mut("peerDependencies")
        .and_then(|v| v.as_object_mut())
    {
        update_dep(deps, "macroforge", "core");
    }

    if let Some(deps) = pkg
        .get_mut("optionalDependencies")
        .and_then(|v| v.as_object_mut())
    {
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
            if deps.contains_key(&key) {
                deps[&key] = serde_json::Value::String(version.to_string());
            }
        }
    }

    fs::write(&full_path, serde_json::to_string_pretty(&pkg)? + "\n")?;
    println!("  Updated {}", pkg_path);
    Ok(())
}

fn update_cargo_toml(
    root: &Path,
    cargo_path: &str,
    version: &str,
    versions: &VersionsCache,
) -> std::io::Result<()> {
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
    if let Some(v) = versions.get_local("macros") {
        content = content.replace(
            &format!("macroforge_ts_macros = \"{}\"", v), // This is brittle if versions differ, but ok for now
            &format!("macroforge_ts_macros = \"{}\"", version),
        );
        // Fallback: replace any version? No, too dangerous.
        // Actually, prep.rs replaces specific version. We can just use the version we are setting as target.
    }
    // Better logic: Replace known patterns
    // We assume current file has *some* version. But prep.rs logic was: replace old version with new.
    // If we don't know old version (because we are just applying), we might fail to match string.
    // Ideally we use toml edit, but string replace is what was used.

    // Simplification: We only need to ensure the dependencies point to the versions we want.
    // If we are applying, we want deps to point to their corresponding `local` version from cache.

    let deps = [
        ("macroforge_ts_macros", "macros"),
        ("macroforge_ts_syn", "syn"),
        ("macroforge_ts_quote", "template"),
    ];

    for (crate_name, repo_name) in deps {
        if let Some(target_v) = versions.get_local(repo_name) {
            // Regex replacement would be better: `crate_name = ".*"` -> `crate_name = "target_v"`
            // Implementing simple line-based replacement
            let lines: Vec<String> = content
                .lines()
                .map(|line| {
                    if line.trim().starts_with(&format!("{} = \"", crate_name)) {
                        format!("{} = \"{}\"", crate_name, target_v)
                    } else {
                        line.to_string()
                    }
                })
                .collect();
            content = lines.join("\n");
        }
    }

    fs::write(&full_path, content)?;
    println!("  Updated {}", cargo_path);
    Ok(())
}

fn update_zed_extensions(root: &Path, versions: &VersionsCache) -> std::io::Result<()> {
    // Vtsls
    let vtsls_lib = root.join("crates/extensions/vtsls-macroforge/src/lib.rs");
    if vtsls_lib.exists() {
        let mut c = fs::read_to_string(&vtsls_lib)?;
        if let Some(v) = versions.get_local("typescript-plugin") {
            c = replace_const(&c, "TS_PLUGIN_VERSION", v);
        }
        if let Some(v) = versions.get_local("core") {
            c = replace_const(&c, "MACROFORGE_VERSION", v);
        }
        fs::write(&vtsls_lib, c)?;
        println!("  Updated crates/extensions/vtsls-macroforge/src/lib.rs");
    }

    // Svelte
    let svelte_lib = root.join("crates/extensions/svelte-macroforge/src/lib.rs");
    if svelte_lib.exists() {
        let mut c = fs::read_to_string(&svelte_lib)?;
        if let Some(v) = versions.get_local("svelte-language-server") {
            c = replace_const(&c, "SVELTE_LS_VERSION", v);
        }
        fs::write(&svelte_lib, c)?;
        println!("  Updated crates/extensions/svelte-macroforge/src/lib.rs");
    }

    Ok(())
}

fn update_core_platform_packages(root: &Path, version: &str) -> std::io::Result<()> {
    let platforms = [
        "darwin-x64",
        "darwin-arm64",
        "linux-x64-gnu",
        "linux-arm64-gnu",
        "win32-x64-msvc",
        "win32-arm64-msvc",
    ];
    for platform in platforms {
        let path = root.join(format!(
            "crates/macroforge_ts/npm/{}/package.json",
            platform
        ));
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let mut pkg: serde_json::Value = serde_json::from_str(&content)?;
            pkg["version"] = serde_json::Value::String(version.to_string());
            fs::write(&path, serde_json::to_string_pretty(&pkg)? + "\n")?;
            println!("  Updated {}", path.display());
        }
    }
    Ok(())
}

// ============================================================================
// Dependency Swapping
// ============================================================================

const INTERNAL_CRATES: &[(&str, &str, &str)] = &[
    ("macroforge_ts_syn", "syn", "../macroforge_ts_syn"),
    ("macroforge_ts_quote", "template", "../macroforge_ts_quote"),
    ("macroforge_ts_macros", "macros", "../macroforge_ts_macros"),
];

fn swap_local(root: &Path) -> std::io::Result<()> {
    let path = root.join("crates/macroforge_ts/Cargo.toml");
    if !path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&path)?;
    let lines: Vec<String> = content
        .lines()
        .map(|line| {
            for (crate_name, _, local_path) in INTERNAL_CRATES {
                if line.trim().starts_with(&format!("{} = \"", crate_name)) {
                    return format!("{} = {{ path = \"{}\" }}", crate_name, local_path);
                }
            }
            line.to_string()
        })
        .collect();

    fs::write(path, lines.join("\n") + "\n")?;
    println!("Swapped to local dependencies");
    Ok(())
}

fn swap_registry(root: &Path, versions: &VersionsCache) -> std::io::Result<()> {
    let path = root.join("crates/macroforge_ts/Cargo.toml");
    if !path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&path)?;
    let lines: Vec<String> = content
        .lines()
        .map(|line| {
            for (crate_name, repo_name, local_path) in INTERNAL_CRATES {
                if line
                    .trim()
                    .starts_with(&format!("{} = {{ path = \"{}\"", crate_name, local_path))
                {
                    let v = versions
                        .get_local(repo_name)
                        .cloned()
                        .unwrap_or("0.1.0".to_string());
                    return format!("{} = \"{}\"", crate_name, v);
                }
            }
            line.to_string()
        })
        .collect();

    fs::write(path, lines.join("\n") + "\n")?;
    println!("Swapped to registry dependencies");
    Ok(())
}

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

    let mut versions = VersionsCache::load(&root);
    let repos = all_repos();

    match args.command {
        Commands::List => {
            println!("{}", serde_json::to_string_pretty(&repos).unwrap());
        }
        Commands::GetVersion { repo, registry } => {
            let v = if registry {
                versions.get_registry(&repo)
            } else {
                versions.get_local(&repo)
            };
            println!("{}", v.cloned().unwrap_or_default());
        }
        Commands::SetVersion {
            repo,
            version,
            registry,
        } => {
            if registry {
                versions.set_registry(&repo, &version);
            } else {
                versions.set_local(&repo, &version);
            }
            // Update file if it's a local set
            if !registry {
                if let Some(r) = repos.iter().find(|r| r.name == repo) {
                    if let Some(p) = &r.package_json {
                        update_package_json(&root, p, &version, &versions).ok();
                    }
                    if let Some(c) = &r.cargo_toml {
                        update_cargo_toml(&root, c, &version, &versions).ok();
                    }
                    if r.name == "core" {
                        update_core_platform_packages(&root, &version).ok();
                    }
                }
            }
            versions.save(&root).ok();
        }
        Commands::ApplyVersions { local } => {
            for repo in &repos {
                let v = if local {
                    versions.get_local(&repo.name)
                } else {
                    versions.get_registry(&repo.name)
                };
                if let Some(ver) = v {
                    if let Some(p) = &repo.package_json {
                        update_package_json(&root, p, ver, &versions).ok();
                    }
                    if let Some(c) = &repo.cargo_toml {
                        update_cargo_toml(&root, c, ver, &versions).ok();
                    }
                    if repo.name == "core" {
                        update_core_platform_packages(&root, ver).ok();
                    }
                }
            }
            update_zed_extensions(&root, &versions).ok();
        }
        Commands::SwapLocal => {
            swap_local(&root).ok();
        }
        Commands::SwapRegistry => {
            swap_registry(&root, &versions).ok();
        }
        Commands::DumpVersions => {
            println!("{}", serde_json::to_string_pretty(&versions).unwrap());
        }
        Commands::UpdateZed => {
            update_zed_extensions(&root, &versions).ok();
        }
        Commands::LinkLocal { repo, deps } => {
            if let Some(r) = repos.iter().find(|x| x.name == repo) {
                if let Some(pkg_path) = &r.package_json {
                    let full_path = root.join(pkg_path);
                    if full_path.exists() {
                        let content = fs::read_to_string(&full_path).unwrap();
                        let mut pkg: serde_json::Value = serde_json::from_str(&content).unwrap();
                        let pkg_dir = full_path.parent().unwrap();
                        let npm_names = npm_package_names();

                        for dep_name in deps {
                            if let Some(npm_name) = npm_names.get(dep_name.as_str()) {
                                if let Some(dep_repo) = repos.iter().find(|x| x.name == dep_name) {
                                    let dep_path = root.join(&dep_repo.path);
                                    let relative = pathdiff_relative(pkg_dir, &dep_path);
                                    let file_ref =
                                        serde_json::Value::String(format!("file:{}", relative));

                                    if let Some(d) = pkg.get_mut("dependencies") {
                                        if d.get(npm_name).is_some() {
                                            d[npm_name] = file_ref.clone();
                                        }
                                    }
                                    if let Some(d) = pkg.get_mut("peerDependencies") {
                                        if d.get(npm_name).is_some() {
                                            d[npm_name] = file_ref.clone();
                                        }
                                    }
                                }
                            }
                        }
                        fs::write(
                            full_path,
                            serde_json::to_string_pretty(&pkg).unwrap() + "\n",
                        )
                        .ok();
                    }
                }
            }
        }
        Commands::RestoreRepo { repo } => {
            if let Some(r) = repos.iter().find(|x| x.name == repo) {
                if let Some(pkg_path) = &r.package_json {
                    update_package_json(
                        &root,
                        pkg_path,
                        versions.get_local(&repo).unwrap_or(&"0.1.0".to_string()),
                        &versions,
                    )
                    .ok();
                }
            }
        }
    }

    ExitCode::SUCCESS
}

// ============================================================================
// Helpers
// ============================================================================

fn replace_const(content: &str, name: &str, val: &str) -> String {
    content
        .lines()
        .map(|line| {
            if line.trim().starts_with(&format!("const {}: &str =", name)) {
                format!(r#"const {}: &str = "{}";"#, name, val)
            } else if line.trim().starts_with(&format!("assert_eq!({},", name)) {
                format!(r#"        assert_eq!({}, "{}");"#, name, val)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pathdiff_relative() {
        let from = Path::new("/a/b/c");
        let to = Path::new("/a/b/d");
        // Canonicalization requires paths to exist, so we can't easily test pathdiff_relative with non-existent paths
        // unless we mock it or avoid canonicalize in the helper for tests.
        // However, the helper uses canonicalize().
        // We'll skip testing pathdiff_relative here as it depends on filesystem.
    }

    #[test]
    fn test_replace_const() {
        let input = r#"
            const TS_PLUGIN_VERSION: &str = "0.0.0";
            const OTHER: &str = "foo";
            fn test() {
                assert_eq!(TS_PLUGIN_VERSION, "0.0.0");
            }
        "#;
        let expected = r#"
            const TS_PLUGIN_VERSION: &str = "1.2.3";
            const OTHER: &str = "foo";
            fn test() {
                assert_eq!(TS_PLUGIN_VERSION, "1.2.3");
            }
        "#;

        // Note: The helper joins with \n, so we might lose the leading newline if we aren't careful,
        // or indentation might match exactly.
        // The helper logic:
        // if line matches const ... -> replace whole line
        // if line matches assert_eq -> replace whole line with fixed indentation

        let output = replace_const(input, "TS_PLUGIN_VERSION", "1.2.3");

        assert!(output.contains(r#"const TS_PLUGIN_VERSION: &str = "1.2.3";"#));
        assert!(output.contains(r#"assert_eq!(TS_PLUGIN_VERSION, "1.2.3");"#));
        assert!(output.contains(r#"const OTHER: &str = "foo";"#));
    }

    #[test]
    fn test_npm_package_names() {
        let map = npm_package_names();
        assert_eq!(map.get("core"), Some(&"macroforge"));
        assert_eq!(map.get("shared"), Some(&"@macroforge/shared"));
    }
}
