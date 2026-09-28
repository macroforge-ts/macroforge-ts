//! Package manifests: what the tooling reads from them, and the release
//! version every one of them carries.

use crate::core::repos::Repo;
use crate::utils::format;
use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// The fields of a deno.json the publishing tools read.
#[derive(serde::Deserialize)]
struct DenoManifest {
    name: Option<String>,
}

/// The fields of a package.json the tooling reads.
#[derive(serde::Deserialize)]
struct NpmManifest {
    name: Option<String>,
    #[serde(default)]
    private: bool,
    #[serde(default)]
    dependencies: HashMap<String, String>,
    #[serde(default, rename = "devDependencies")]
    dev_dependencies: HashMap<String, String>,
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

/// Whether a directory's package.json lists `package` as a dependency or
/// dev dependency.
pub fn npm_depends_on(dir: &Path, package: &str) -> Result<bool> {
    Ok(read_npm_manifest(dir)?.is_some_and(|manifest| {
        manifest.dependencies.contains_key(package)
            || manifest.dev_dependencies.contains_key(package)
    }))
}

/// The package whose manifest holds the release version. Every package
/// carries the same one; this is where it is read.
const VERSION_MANIFEST: &str = "crates/macroforge_ts/package.json";

#[derive(serde::Deserialize)]
struct VersionedManifest {
    version: String,
}

/// The release version every package carries.
pub fn current_version(root: &Path) -> Result<String> {
    let path = root.join(VERSION_MANIFEST);
    let content =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    let manifest: VersionedManifest = serde_json::from_str(&content)
        .with_context(|| format!("{} has no readable version", path.display()))?;
    Ok(manifest.version)
}

/// Writes `version` into every npm and JSR manifest of `repo`, and into its
/// requirements on the other workspace packages.
pub fn set_version(repo: &Repo, version: &str) -> Result<()> {
    if let Some(path) = &repo.package_json {
        update_package_json(path, version)?;
    }
    update_deno_json(&repo.abs_path, version)
}

/// Writes `version` into the workspace Cargo.toml, which every released crate
/// inherits its version and internal requirements from.
pub fn set_crate_version(root: &Path, version: &str) -> Result<()> {
    update_cargo_toml(&root.join("Cargo.toml"), version)
}

/// Sets the version of a deno.json that publishes to JSR (has a `name`).
fn update_deno_json(dir: &Path, version: &str) -> Result<()> {
    let path = dir.join("deno.json");
    if !path.exists() {
        return Ok(());
    }
    let content =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    let mut deno: Value = serde_json::from_str(&content)
        .with_context(|| format!("failed to parse {}", path.display()))?;
    if deno.get("name").is_none() {
        return Ok(());
    }
    deno["version"] = json!(version);
    fs::write(&path, crate::utils::json::to_string_pretty(&deno)? + "\n")
        .with_context(|| format!("failed to write {}", path.display()))?;
    format::success(&format!("Updated {}", path.display()));
    Ok(())
}

/// Pins the npm packages the Zed extensions download to `version`.
pub fn update_zed_extensions(root: &Path, version: &str) -> Result<()> {
    for (relative, constant) in [
        (
            "crates/extensions/vtsls_macroforge/src/lib.rs",
            "TS_PLUGIN_VERSION",
        ),
        (
            "crates/extensions/svelte_macroforge/src/lib.rs",
            "SVELTE_LS_VERSION",
        ),
    ] {
        let path = root.join(relative);
        let content = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        fs::write(&path, replace_const(&content, constant, version))
            .with_context(|| format!("failed to write {}", path.display()))?;
        format::success(&format!("Updated {relative}"));
    }
    Ok(())
}

/// The workspace packages other package.json files depend on.
const NPM_PACKAGES: &[&str] = &[
    "@macroforge/core",
    "@macroforge/shared",
    "@macroforge/typescript-plugin",
];

/// Sets a package.json's version and its requirements on workspace packages.
///
/// A `file:` reference stays: it points at the local checkout on purpose.
fn update_package_json(path: &Path, version: &str) -> Result<()> {
    let content =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let mut pkg: Value = serde_json::from_str(&content)
        .with_context(|| format!("failed to parse {}", path.display()))?;

    pkg["version"] = json!(version);
    for section in ["dependencies", "devDependencies", "peerDependencies"] {
        let Some(deps) = pkg.get_mut(section).and_then(Value::as_object_mut) else {
            continue;
        };
        for package in NPM_PACKAGES {
            let is_registry_dep = deps
                .get(*package)
                .and_then(Value::as_str)
                .is_some_and(|current| !current.starts_with("file:"));
            if is_registry_dep {
                deps[*package] = json!(version);
            }
        }
    }

    fs::write(path, crate::utils::json::to_string_pretty(&pkg)? + "\n")
        .with_context(|| format!("failed to write {}", path.display()))?;
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

/// The workspace crates other Cargo.toml files depend on.
const CRATES: &[&str] = &[
    "macroforge_ts_macros",
    "macroforge_ts_syn",
    "macroforge_ts_quote",
];

/// Sets a Cargo.toml's version and its requirements on workspace crates.
fn update_cargo_toml(path: &Path, version: &str) -> Result<()> {
    let content =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;

    let mut lines: Vec<String> = content
        .lines()
        .map(|line| {
            if line.starts_with("version = \"") {
                format!("version = \"{version}\"")
            } else {
                line.to_string()
            }
        })
        .collect();
    for crate_name in CRATES {
        lines = lines
            .into_iter()
            .map(|line| rewrite_dep_version(line, crate_name, version))
            .collect();
    }

    let mut out = lines.join("\n");
    if content.ends_with('\n') {
        out.push('\n');
    }
    fs::write(path, out).with_context(|| format!("failed to write {}", path.display()))?;
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
