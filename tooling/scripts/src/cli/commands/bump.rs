//! Bump the release version
//!
//! Every package carries one version. `mf bump` writes it into every
//! manifest, the Zed extensions' pinned npm versions and the Cargo locks, then
//! re-extracts the API docs, which record it. Rerunning with `--version`
//! finishes an interrupted bump.

use crate::cli::args::BumpArgs;
use crate::cli::commands::docs::extract_api_docs;
use crate::core::config::Config;
use crate::core::manifests;
use crate::core::shell;
use crate::diagnostics::clippy;
use crate::utils::format;
use anyhow::{Context, Result};
use colored::Colorize;

/// Entry point for `mf bump`.
pub fn run(args: BumpArgs) -> Result<()> {
    let config = Config::load()?;
    let current = manifests::current_version(&config.root)?;
    let version = match args.version {
        Some(version) => version,
        None => increment_patch(&current)?,
    };

    format::header("Bump");
    println!("{} → {}", current.dimmed(), version.green());

    for repo in config.repos.values() {
        manifests::set_version(repo, &version)
            .with_context(|| format!("failed to set the version of {}", repo.name))?;
    }
    manifests::set_crate_version(&config.root, &version)?;
    manifests::update_zed_extensions(&config.root, &version)?;
    shell::cargo::sync_lock(&config.root).context("failed to update Cargo.lock")?;
    // Crates excluded from the workspace keep their own lock, which records the
    // engine crates they depend on by path.
    for crate_dir in clippy::excluded_crates(&config.root)? {
        shell::cargo::sync_lock(&crate_dir)
            .with_context(|| format!("failed to update {}/Cargo.lock", crate_dir.display()))?;
    }

    println!("\n{}", "Re-extracting the API docs".bold());
    extract_api_docs(&config.root)?;

    format::success(&format!("Bumped to {version}"));
    Ok(())
}

/// `version` with its patch number incremented.
fn increment_patch(version: &str) -> Result<String> {
    let parts: Vec<&str> = version.split('.').collect();
    let [major, minor, patch] = parts.as_slice() else {
        anyhow::bail!("{version} is not a major.minor.patch version");
    };
    let patch: u32 = patch
        .parse()
        .with_context(|| format!("{version} has a non-numeric patch number"))?;
    Ok(format!("{major}.{minor}.{}", patch + 1))
}

#[cfg(test)]
mod tests {
    use super::increment_patch;

    #[test]
    fn increments_the_patch_number() {
        assert_eq!(increment_patch("1.2.3").unwrap(), "1.2.4");
        assert_eq!(increment_patch("0.3.9").unwrap(), "0.3.10");
    }

    #[test]
    fn rejects_anything_but_three_parts() {
        assert!(increment_patch("1.2").is_err());
        assert!(increment_patch("1.2.3.4").is_err());
        assert!(increment_patch("1.2.x").is_err());
    }
}
