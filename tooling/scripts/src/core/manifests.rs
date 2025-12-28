//! Manifest manipulation core functions
//!
//! Handles reading/writing versions to package.json and Cargo.toml,
//! managing versions.json cache, and swapping dependency paths.

use crate::core::config::{self, Config, INTERNAL_CRATES, PLATFORMS};
use crate::core::versions::VersionsCache;
use crate::utils::format;
use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

/// Swap internal crate dependencies to local paths
pub fn swap_local(root: &Path) -> Result<()> {
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
                    // Use compact format without space after { for consistency
                    return format!("{} = {{path = \"{}\"}}", crate_name, local_path);
                }
            }
            line.to_string()
        })
        .collect();

    fs::write(&path, lines.join("\n") + "\n")?;
    Ok(())
}

/// Swap internal crate dependencies back to registry versions
pub fn swap_registry(root: &Path, versions: &VersionsCache) -> Result<()> {
    let path = root.join("crates/macroforge_ts/Cargo.toml");
    if !path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&path)?;
    let lines: Vec<String> = content
        .lines()
        .map(|line| {
            for (crate_name, repo_name, local_path) in INTERNAL_CRATES {
                // Handle both formats: `{ path = "..."` and `{path = "..."`
                let with_space = format!("{} = {{ path = \"{}\"", crate_name, local_path);
                let no_space = format!("{} = {{path = \"{}\"", crate_name, local_path);
                if line.trim().starts_with(&with_space) || line.trim().starts_with(&no_space) {
                    let v = versions
                        .get_local(repo_name)
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "0.1.0".to_string());
                    return format!("{} = \"{}\"", crate_name, v);
                }
            }
            line.to_string()
        })
        .collect();

    fs::write(&path, lines.join("\n") + "\n")?;
    Ok(())
}

/// Swap npm internal dependencies to local file: paths for repos being built
/// Only swaps deps that are themselves in the build list
pub fn swap_npm_local(config: &Config, repos_to_build: &[&str]) -> Result<()> {
    let npm_names = config::npm_package_names();
    let build_set: std::collections::HashSet<&str> = repos_to_build.iter().copied().collect();

    for repo in config.repos.values() {
        let Some(pkg_path) = &repo.package_json else {
            continue;
        };

        let full_path = config.root.join(pkg_path);
        if !full_path.exists() {
            continue;
        }

        let content = fs::read_to_string(&full_path)?;
        let mut pkg: Value = serde_json::from_str(&content)?;
        let pkg_dir = full_path.parent().unwrap();
        let mut modified = false;

        // Only swap deps that are in the build list
        for (dep_repo_name, npm_name) in &npm_names {
            // Only link if this dep is being built
            if !build_set.contains(*dep_repo_name) {
                continue;
            }

            if let Some(dep_repo) = config.repos.get(*dep_repo_name) {
                let relative = pathdiff_relative(pkg_dir, &dep_repo.abs_path);
                let file_ref = json!(format!("file:{}", relative));

                // Update dependencies
                if let Some(deps) = pkg.get_mut("dependencies").and_then(|v| v.as_object_mut())
                    && deps.contains_key(*npm_name)
                    && !deps[*npm_name].as_str().is_some_and(|s| s.starts_with("file:"))
                {
                    deps[*npm_name] = file_ref.clone();
                    modified = true;
                }

                // Update peerDependencies
                if let Some(deps) = pkg.get_mut("peerDependencies").and_then(|v| v.as_object_mut())
                    && deps.contains_key(*npm_name)
                    && !deps[*npm_name].as_str().is_some_and(|s| s.starts_with("file:"))
                {
                    deps[*npm_name] = file_ref.clone();
                    modified = true;
                }
            }
        }

        if modified {
            fs::write(&full_path, serde_json::to_string_pretty(&pkg)? + "\n")?;
        }
    }

    Ok(())
}

/// Swap all npm internal dependencies back to registry versions
pub fn swap_npm_registry(config: &Config, versions: &VersionsCache) -> Result<()> {
    let npm_names = config::npm_package_names();

    for repo in config.repos.values() {
        let Some(pkg_path) = &repo.package_json else {
            continue;
        };

        let full_path = config.root.join(pkg_path);
        if !full_path.exists() {
            continue;
        }

        let content = fs::read_to_string(&full_path)?;
        let mut pkg: Value = serde_json::from_str(&content)?;
        let mut modified = false;

        // Check each internal npm package as a potential dependency
        for (dep_repo_name, npm_name) in &npm_names {
            let version = versions
                .get_local(dep_repo_name)
                .map(|s| format!("^{}", s))
                .unwrap_or_else(|| "^0.1.0".to_string());
            let version_ref = json!(version);

            // Update dependencies
            if let Some(deps) = pkg.get_mut("dependencies").and_then(|v| v.as_object_mut())
                && deps.contains_key(*npm_name)
                && deps[*npm_name].as_str().is_some_and(|s| s.starts_with("file:"))
            {
                deps[*npm_name] = version_ref.clone();
                modified = true;
            }

            // Update peerDependencies
            if let Some(deps) = pkg.get_mut("peerDependencies").and_then(|v| v.as_object_mut())
                && deps.contains_key(*npm_name)
                && deps[*npm_name].as_str().is_some_and(|s| s.starts_with("file:"))
            {
                deps[*npm_name] = version_ref.clone();
                modified = true;
            }
        }

        if modified {
            fs::write(&full_path, serde_json::to_string_pretty(&pkg)? + "\n")?;
        }
    }

    Ok(())
}

/// Link npm dependencies to local file: paths
pub fn link_local_deps(config: &Config, repo: &str, deps: &[String]) -> Result<()> {
    let r = config
        .repos
        .get(repo)
        .context(format!("Unknown repo: {}", repo))?;
    let pkg_path = r
        .package_json
        .as_ref()
        .context("Repo has no package.json")?;

    let full_path = config.root.join(pkg_path);
    if !full_path.exists() {
        anyhow::bail!("package.json not found: {}", full_path.display());
    }

    let content = fs::read_to_string(&full_path)?;
    let mut pkg: Value = serde_json::from_str(&content)?;
    let pkg_dir = full_path.parent().unwrap();
    let npm_names = config::npm_package_names();

    for dep_name in deps {
        if let Some(npm_name) = npm_names.get(dep_name.as_str())
            && let Some(dep_repo) = config.repos.get(dep_name)
        {
            let dep_path = &dep_repo.abs_path;
            let relative = pathdiff_relative(pkg_dir, dep_path);
            let file_ref = json!(format!("file:{}", relative));

            if let Some(d) = pkg.get_mut("dependencies").and_then(|v| v.as_object_mut())
                && d.contains_key(*npm_name)
            {
                d[*npm_name] = file_ref.clone();
            }
            if let Some(d) = pkg.get_mut("peerDependencies").and_then(|v| v.as_object_mut())
                && d.contains_key(*npm_name)
            {
                d[*npm_name] = file_ref.clone();
            }
        }
    }

    fs::write(&full_path, serde_json::to_string_pretty(&pkg)? + "\n")?;
    Ok(())
}

/// Restore a repo's package.json to use registry versions
pub fn restore_repo(config: &Config, versions: &VersionsCache, repo: &str) -> Result<()> {
    if let Some(r) = config.repos.get(repo)
        && let Some(pkg_path) = &r.package_json
    {
        let ver = versions
            .get_local(repo)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "0.1.0".to_string());
        update_package_json(&config.root, pkg_path, &ver, versions)?;
    }
    Ok(())
}

/// Set version in a repo's manifest files
pub fn set_version(
    config: &Config,
    versions: &mut VersionsCache,
    repo: &str,
    version: &str,
) -> Result<()> {
    if let Some(r) = config.repos.get(repo) {
        if let Some(pkg_path) = &r.package_json {
            update_package_json(&config.root, pkg_path, version, versions)?;
        }
        if let Some(cargo_path) = &r.cargo_toml {
            update_cargo_toml(&config.root, cargo_path, version, versions)?;
        }
        if repo == "core" {
            update_core_platform_packages(&config.root, version)?;
        }
    }
    versions.set_local(repo, version);
    Ok(())
}

/// Update Zed extension files with version constants
/// Uses registry versions since extensions download from npm
pub fn update_zed_extensions(root: &Path, versions: &VersionsCache) -> Result<()> {
    // Vtsls extension
    let vtsls_lib = root.join("crates/extensions/vtsls-macroforge/src/lib.rs");
    if vtsls_lib.exists() {
        let mut content = fs::read_to_string(&vtsls_lib)?;
        // Use registry version (what's published) since extensions download from npm
        if let Some(v) = versions.get_registry("typescript-plugin") {
            content = replace_const(&content, "TS_PLUGIN_VERSION", v);
        }
        if let Some(v) = versions.get_registry("core") {
            content = replace_const(&content, "MACROFORGE_VERSION", v);
        }
        fs::write(&vtsls_lib, content)?;
        format::success("Updated crates/extensions/vtsls-macroforge/src/lib.rs");
    }

    // Svelte extension
    let svelte_lib = root.join("crates/extensions/svelte-macroforge/src/lib.rs");
    if svelte_lib.exists() {
        let mut content = fs::read_to_string(&svelte_lib)?;
        if let Some(v) = versions.get_registry("svelte-language-server") {
            content = replace_const(&content, "SVELTE_LS_VERSION", v);
        }
        fs::write(&svelte_lib, content)?;
        format::success("Updated crates/extensions/svelte-macroforge/src/lib.rs");
    }

    Ok(())
}

/// Update a package.json with new version and dependency versions
fn update_package_json(
    root: &Path,
    pkg_path: &Path,
    version: &str,
    versions: &VersionsCache,
) -> Result<()> {
    let full_path = root.join(pkg_path);
    if !full_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&full_path).context("Failed to read package.json")?;
    let mut pkg: Value = serde_json::from_str(&content).context("Failed to parse package.json")?;

    pkg["version"] = json!(version);

    // Update internal dependencies
    let update_dep = |deps: &mut serde_json::Map<String, Value>, key: &str, target_repo: &str| {
        if deps.contains_key(key)
            && let Some(v) = versions.get_local(target_repo)
        {
            deps[key] = json!(format!("^{}", v));
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

    // Update optionalDependencies for platform binaries
    if let Some(deps) = pkg
        .get_mut("optionalDependencies")
        .and_then(|v| v.as_object_mut())
    {
        for platform in PLATFORMS {
            let key = format!("@macroforge/bin-{}", platform);
            if deps.contains_key(&key) {
                deps[&key] = json!(version);
            }
        }
    }

    fs::write(&full_path, serde_json::to_string_pretty(&pkg)? + "\n")?;
    format::success(&format!("Updated {}", pkg_path.display()));

    Ok(())
}

/// Update a Cargo.toml with new version and dependency versions
fn update_cargo_toml(
    root: &Path,
    cargo_path: &Path,
    version: &str,
    versions: &VersionsCache,
) -> Result<()> {
    let full_path = root.join(cargo_path);
    if !full_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&full_path).context("Failed to read Cargo.toml")?;

    // Update version line
    let mut lines: Vec<String> = content
        .lines()
        .map(|line| {
            if line.starts_with("version = \"") {
                format!("version = \"{}\"", version)
            } else {
                line.to_string()
            }
        })
        .collect();

    // Update internal crate dependencies
    let deps = [
        ("macroforge_ts_macros", "macros"),
        ("macroforge_ts_syn", "syn"),
        ("macroforge_ts_quote", "template"),
    ];

    for (crate_name, repo_name) in deps {
        if let Some(target_v) = versions.get_local(repo_name) {
            lines = lines
                .into_iter()
                .map(|line| {
                    if line.trim().starts_with(&format!("{} = \"", crate_name)) {
                        format!("{} = \"{}\"", crate_name, target_v)
                    } else {
                        line
                    }
                })
                .collect();
        }
    }

    fs::write(&full_path, lines.join("\n"))?;
    format::success(&format!("Updated {}", cargo_path.display()));

    Ok(())
}

/// Update platform-specific npm packages in core crate
fn update_core_platform_packages(root: &Path, version: &str) -> Result<()> {
    for platform in PLATFORMS {
        let path = root.join(format!("crates/macroforge_ts/npm/{}/package.json", platform));
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let mut pkg: Value = serde_json::from_str(&content)?;
            pkg["version"] = json!(version);
            fs::write(&path, serde_json::to_string_pretty(&pkg)? + "\n")?;
            format::success(&format!("Updated {}", path.display()));
        }
    }
    Ok(())
}

/// Helper to replace a const value in Rust source
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

/// Apply all versions from a VersionsCache to all manifest files
/// Used for rollback when prep fails
pub fn apply_versions(config: &Config, versions: &VersionsCache) -> Result<()> {
    for repo in config.repos.values() {
        if let Some(ver) = versions.get_local(&repo.name) {
            if let Some(pkg_path) = &repo.package_json {
                let _ = update_package_json(&config.root, pkg_path, ver, versions);
            }
            if let Some(cargo_path) = &repo.cargo_toml {
                let _ = update_cargo_toml(&config.root, cargo_path, ver, versions);
            }
            if repo.name == "core" {
                let _ = update_core_platform_packages(&config.root, ver);
            }
        }
    }
    let _ = update_zed_extensions(&config.root, versions);
    Ok(())
}

/// Calculate relative path from one path to another
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
