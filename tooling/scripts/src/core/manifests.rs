//! Manifest manipulation core functions
//!
//! Handles reading/writing versions to package.json and Cargo.toml,
//! managing versions.json cache, and swapping dependency paths.

use crate::core::config::Config;
use crate::core::repos::Repo;
use crate::core::versions::VersionsCache;
use crate::utils::format;
use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::fs;
use std::path::Path;

/// The fields of a deno.json the publishing tools read.
#[derive(serde::Deserialize)]
struct DenoManifest {
    name: Option<String>,
}

/// The fields of a package.json the publishing tools read.
#[derive(serde::Deserialize)]
struct NpmManifest {
    name: Option<String>,
    #[serde(default)]
    private: bool,
}

fn read_npm_manifest(dir: &Path) -> Result<Option<NpmManifest>> {
    let path = dir.join("package.json");
    if !path.exists() {
        return Ok(None);
    }
    let content =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_str(&content)
        .map(Some)
        .with_context(|| format!("{} is not valid JSON", path.display()))
}

/// The npm package a directory publishes: its package.json `name`, unless the
/// package is private.
pub fn npm_package_name(dir: &Path) -> Result<Option<String>> {
    Ok(read_npm_manifest(dir)?
        .filter(|manifest| !manifest.private)
        .and_then(|manifest| manifest.name)
        .filter(|name| !name.is_empty()))
}

/// The JSR package a directory publishes: its deno.json `name`, if any.
pub fn jsr_package_name(dir: &Path) -> Result<Option<String>> {
    let path = dir.join("deno.json");
    if !path.exists() {
        return Ok(None);
    }
    let content =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    let manifest: DenoManifest = serde_json::from_str(&content)
        .with_context(|| format!("{} is not valid JSON", path.display()))?;
    Ok(manifest.name.filter(|name| !name.is_empty()))
}

/// Whether package.json marks the package private, so npm refuses to publish
/// it (EPRIVATE). Deno-only packages such as `@macroforge/deno-plugin` set this
/// and ship through JSR alone.
pub fn npm_private(dir: &Path) -> Result<bool> {
    Ok(read_npm_manifest(dir)?.is_some_and(|manifest| manifest.private))
}

/// Write one repo's version into every manifest that carries it.
///
/// The single place that knows which files a version lives in. Bumping and
/// rolling back both go through it, because when they each had their own list
/// the rollback's was short one entry and left every `deno.json` at the version
/// of the build that failed.
fn write_repo_version(repo: &Repo, version: &str, versions: &VersionsCache) -> Result<()> {
    if let Some(pkg_path) = &repo.package_json {
        update_package_json(pkg_path, version, versions)?;
    }
    if let Some(cargo_path) = &repo.cargo_toml {
        update_cargo_toml(cargo_path, version, versions)?;
    }
    update_jsr_json(&repo.abs_path, version)
}

/// Set version in a repo's manifest files
pub fn set_version(
    config: &Config,
    versions: &mut VersionsCache,
    repo: &str,
    version: &str,
) -> Result<()> {
    if let Some(r) = config.repos.get(repo) {
        write_repo_version(r, version, versions)?;
    }
    versions.set_local(repo, version);
    Ok(())
}

/// Update JSR version in deno.json if it has a "name" field (JSR package)
fn update_jsr_json(dir: &Path, version: &str) -> Result<()> {
    let path = dir.join("deno.json");
    if !path.exists() {
        return Ok(());
    }
    let content = fs::read_to_string(&path)?;
    let mut deno: Value = serde_json::from_str(&content)?;
    // Only update version if this deno.json is a JSR package (has "name" field)
    if deno.get("name").is_some() {
        deno["version"] = json!(version);
        fs::write(&path, crate::utils::json::to_string_pretty(&deno)? + "\n")?;
        format::success(&format!("Updated {}", path.display()));
    }
    Ok(())
}

/// Update Zed extension files with version constants
/// Uses registry versions since extensions download from npm
pub fn update_zed_extensions(root: &Path, versions: &VersionsCache) -> Result<()> {
    // Vtsls extension
    let vtsls_lib = root.join("crates/extensions/vtsls_macroforge/src/lib.rs");
    if vtsls_lib.exists() {
        let mut content = fs::read_to_string(&vtsls_lib)?;
        // Use registry version (what's published) since extensions download from npm
        if let Some(v) = versions.get_registry("typescript-plugin") {
            content = replace_const(&content, "TS_PLUGIN_VERSION", v);
        }
        fs::write(&vtsls_lib, content)?;
        format::success("Updated crates/extensions/vtsls_macroforge/src/lib.rs");
    }

    // Svelte extension
    let svelte_lib = root.join("crates/extensions/svelte_macroforge/src/lib.rs");
    if svelte_lib.exists() {
        let mut content = fs::read_to_string(&svelte_lib)?;
        if let Some(v) = versions.get_registry("svelte-language-server") {
            content = replace_const(&content, "SVELTE_LS_VERSION", v);
        }
        fs::write(&svelte_lib, content)?;
        format::success("Updated crates/extensions/svelte_macroforge/src/lib.rs");
    }

    Ok(())
}

/// Update a package.json with new version and dependency versions
fn update_package_json(path: &Path, version: &str, versions: &VersionsCache) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(path).context("Failed to read package.json")?;
    let mut pkg: Value = serde_json::from_str(&content).context("Failed to parse package.json")?;

    pkg["version"] = json!(version);

    // Update internal dependencies.
    //
    // A `file:` reference is left alone. Whether a dependency points at the
    // local checkout or at the registry is owned by the swap functions, and
    // overwriting it here silently converts a tree that is mid-build back to
    // registry deps: exactly what the rollback did to every package it touched.
    let update_dep = |deps: &mut serde_json::Map<String, Value>, key: &str, target_repo: &str| {
        if deps
            .get(key)
            .and_then(Value::as_str)
            .is_some_and(|current| current.starts_with("file:"))
        {
            return;
        }
        if deps.contains_key(key)
            && let Some(v) = versions.get_local(target_repo)
        {
            deps[key] = json!(v);
        }
    };

    if let Some(deps) = pkg.get_mut("dependencies").and_then(|v| v.as_object_mut()) {
        update_dep(deps, "@macroforge/core", "core");
        update_dep(deps, "@macroforge/shared", "shared");
        update_dep(deps, "@macroforge/typescript-plugin", "typescript-plugin");
    }

    // The website carries `@macroforge/core` here, and it resolves to the
    // workspace package only while the two versions agree. Leaving it behind
    // pins it at a release the workspace has moved past and the registry does
    // not have yet, which fails the next install.
    if let Some(deps) = pkg
        .get_mut("devDependencies")
        .and_then(|v| v.as_object_mut())
    {
        update_dep(deps, "@macroforge/core", "core");
        update_dep(deps, "@macroforge/shared", "shared");
        update_dep(deps, "@macroforge/typescript-plugin", "typescript-plugin");
    }

    if let Some(deps) = pkg
        .get_mut("peerDependencies")
        .and_then(|v| v.as_object_mut())
    {
        update_dep(deps, "@macroforge/core", "core");
    }

    fs::write(path, crate::utils::json::to_string_pretty(&pkg)? + "\n")?;
    format::success(&format!("Updated {}", path.display()));

    Ok(())
}

/// Rewrite one internal dependency's version requirement, in either form.
///
/// A workspace dependency is carried as `dep = { version = "..", path = ".." }`
/// so it builds locally and still publishes: cargo drops the path when it
/// packs. Matching only the bare `dep = ".."` form left those requirements at
/// whatever the previous release set, which a bump then contradicted.
fn rewrite_dep_version(line: String, crate_name: &str, target: &str) -> String {
    let trimmed = line.trim();
    if trimmed.starts_with(&format!("{crate_name} = \"")) {
        return format!("{crate_name} = \"{target}\"");
    }
    if !trimmed.starts_with(&format!("{crate_name} = {{")) {
        return line;
    }
    // Replace the version field, leaving `path`, `features` and the rest alone.
    let Some(start) = line.find("version = \"") else {
        return line;
    };
    let value = start + "version = \"".len();
    let Some(len) = line[value..].find('"') else {
        return line;
    };
    format!("{}{}{}", &line[..value], target, &line[value + len..])
}

/// Update a Cargo.toml with new version and dependency versions
fn update_cargo_toml(path: &Path, version: &str, versions: &VersionsCache) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(path).context("Failed to read Cargo.toml")?;
    let had_trailing_newline = content.ends_with('\n');

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
                .map(|line| rewrite_dep_version(line, crate_name, target_v))
                .collect();
        }
    }

    let mut out = lines.join("\n");
    if had_trailing_newline {
        out.push('\n');
    }
    fs::write(path, out)?;
    format::success(&format!("Updated {}", path.display()));

    Ok(())
}

/// Helper to replace a const value in Rust source
fn replace_const(content: &str, name: &str, val: &str) -> String {
    let had_trailing_newline = content.ends_with('\n');
    let mut result = content
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
        .join("\n");
    if had_trailing_newline {
        result.push('\n');
    }
    result
}

/// Apply all versions from a VersionsCache to all manifest files
/// Used for rollback when prep fails
pub fn apply_versions(config: &Config, versions: &VersionsCache) -> Result<()> {
    for repo in config.repos.values() {
        if let Some(ver) = versions.get_local(&repo.name) {
            write_repo_version(repo, ver, versions).unwrap_or_else(|e| {
                eprintln!("  Warning: Failed to restore {}: {}", repo.name, e);
            });
        }
    }
    update_zed_extensions(&config.root, versions)?;
    Ok(())
}
