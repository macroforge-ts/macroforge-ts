#!/usr/bin/env rust-script
//! Fetch latest versions from npm and crates.io and update versions.json.
//!
//! ```cargo
//! [dependencies]
//! colored = "2"
//! ```

use colored::*;
use std::collections::HashMap;
use std::process::{Command, ExitCode};

// ============================================================================ //
// Package Mappings //
// ============================================================================ //

fn npm_packages() -> HashMap<&'static str, &'static str> {
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

fn crate_packages() -> HashMap<&'static str, &'static str> {
    [
        ("syn", "macroforge_ts_syn"),
        ("template", "macroforge_ts_quote"),
        ("macros", "macroforge_ts_macros"),
    ]
    .into_iter()
    .collect()
}

// ============================================================================ //
// Manifests Interop //
// ============================================================================ //

fn get_version(repo: &str, registry: bool) -> String {
    let mut cmd = Command::new("rust-script");
    cmd.args(["tooling/scripts/manifests.rs", "get-version", repo]);
    if registry {
        cmd.arg("--registry");
    }

    let output = cmd.output().expect("Failed to get version");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn set_version(repo: &str, version: &str, registry: bool) {
    let mut cmd = Command::new("rust-script");
    cmd.args(["tooling/scripts/manifests.rs", "set-version", repo, version]);
    if registry {
        cmd.arg("--registry");
    }

    let status = cmd.status().expect("Failed to set version");
    if !status.success() {
        eprintln!("Failed to set version for {}", repo);
    }
}

fn update_zed() {
    Command::new("rust-script")
        .args(["tooling/scripts/manifests.rs", "update-zed"])
        .status()
        .expect("Failed to update zed");
}

// ============================================================================ //
// Fetching //
// ============================================================================ //

fn fetch_npm_version(package: &str) -> Option<String> {
    let output = Command::new("npm")
        .args(["view", package, "version"])
        .output()
        .ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

fn fetch_crate_version(crate_name: &str) -> Option<String> {
    let output = Command::new("cargo")
        .args(["search", crate_name, "--limit", "1"])
        .output()
        .ok()?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.starts_with(crate_name) {
                if let Some(start) = line.find('"') {
                    if let Some(end) = line[start + 1..].find('"') {
                        return Some(line[start + 1..start + 1 + end].to_string());
                    }
                }
            }
        }
    }
    None
}

// ============================================================================ //
// Main //
// ============================================================================ //

fn main() -> ExitCode {
    println!("{}", "═".repeat(60));
    println!("{}", "Fetching latest versions from registries".bold());
    println!("{}", "═".repeat(60));

    let mut updated = Vec::new();
    let mut unchanged = Vec::new();
    let mut failed = Vec::new();

    // Fetch npm versions
    println!("\n{}", "npm packages:".bold());
    for (repo, npm_pkg) in npm_packages() {
        print!("  {} {}... ", "→".blue(), npm_pkg);
        std::io::Write::flush(&mut std::io::stdout()).ok();

        match fetch_npm_version(npm_pkg) {
            Some(version) => {
                let old_registry = get_version(repo, true);
                let old_local = get_version(repo, false);
                let registry_changed = old_registry != version;
                let local_out_of_sync = old_local != version;

                if registry_changed || local_out_of_sync {
                    if registry_changed {
                        println!("{} (was {})", version.green(), old_registry.dimmed());
                    } else {
                        println!("{} (local was {})", version.green(), old_local.dimmed());
                    }
                    // Update registry record
                    set_version(repo, &version, true);
                    // Update local record and files (Sync/Reset behavior)
                    set_version(repo, &version, false);
                    updated.push((repo, version));
                } else {
                    println!("{}", version.dimmed());
                    unchanged.push(repo);
                }
            }
            None => {
                println!("{}", "failed".red());
                failed.push(repo);
            }
        }
    }

    // Fetch crates.io versions
    println!("\n{}", "crates.io packages:".bold());
    for (repo, crate_name) in crate_packages() {
        print!("  {} {}... ", "→".blue(), crate_name);
        std::io::Write::flush(&mut std::io::stdout()).ok();

        match fetch_crate_version(crate_name) {
            Some(version) => {
                let old_registry = get_version(repo, true);
                let old_local = get_version(repo, false);
                let registry_changed = old_registry != version;
                let local_out_of_sync = old_local != version;

                if registry_changed || local_out_of_sync {
                    if registry_changed {
                        println!("{} (was {})", version.green(), old_registry.dimmed());
                    } else {
                        println!("{} (local was {})", version.green(), old_local.dimmed());
                    }
                    set_version(repo, &version, true);
                    set_version(repo, &version, false);
                    updated.push((repo, version));
                } else {
                    println!("{}", version.dimmed());
                    unchanged.push(repo);
                }
            }
            None => {
                println!("{}", "failed".red());
                failed.push(repo);
            }
        }
    }

    // Summary
    println!("\n{}", "═".repeat(60));
    println!("{}", "Summary".bold());
    println!("{}", "═".repeat(60));

    if !updated.is_empty() {
        println!("\n{}:", "Updated".green().bold());
        for (repo, version) in &updated {
            println!("  {} → {}", repo.cyan(), version.green());
        }

        // Final sync for zed extensions since they depend on other versions
        update_zed();
        println!("\n{} versions.json and files updated", "✓".green());
    }

    if !unchanged.is_empty() {
        println!("\n{}: {}", "Unchanged".dimmed(), unchanged.join(", "));
    }

    if !failed.is_empty() {
        println!("\n{}: {}", "Failed".red(), failed.join(", "));
    }

    ExitCode::SUCCESS
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npm_packages_mapping() {
        let packages = npm_packages();

        assert_eq!(packages.get("core"), Some(&"macroforge"));
        assert_eq!(packages.get("shared"), Some(&"@macroforge/shared"));
        assert_eq!(
            packages.get("vite-plugin"),
            Some(&"@macroforge/vite-plugin")
        );
        assert_eq!(
            packages.get("typescript-plugin"),
            Some(&"@macroforge/typescript-plugin")
        );
        assert_eq!(
            packages.get("svelte-language-server"),
            Some(&"@macroforge/svelte-language-server")
        );
        assert_eq!(
            packages.get("svelte-preprocessor"),
            Some(&"@macroforge/svelte-preprocessor")
        );
        assert_eq!(packages.get("mcp-server"), Some(&"@macroforge/mcp-server"));
    }

    #[test]
    fn test_npm_packages_count() {
        let packages = npm_packages();
        assert_eq!(packages.len(), 7);
    }

    #[test]
    fn test_crate_packages_mapping() {
        let packages = crate_packages();

        assert_eq!(packages.get("syn"), Some(&"macroforge_ts_syn"));
        assert_eq!(packages.get("template"), Some(&"macroforge_ts_quote"));
        assert_eq!(packages.get("macros"), Some(&"macroforge_ts_macros"));
    }

    #[test]
    fn test_crate_packages_count() {
        let packages = crate_packages();
        assert_eq!(packages.len(), 3);
    }

    #[test]
    fn test_npm_packages_no_overlap_with_crates() {
        let npm = npm_packages();
        let crates = crate_packages();

        // core is special - it's in npm but not in crates (in this mapping)
        // Other packages should not overlap
        for key in crates.keys() {
            if *key != "core" {
                assert!(
                    !npm.contains_key(key),
                    "Unexpected overlap for key: {}",
                    key
                );
            }
        }
    }

    #[test]
    fn test_all_npm_packages_have_scope() {
        let packages = npm_packages();

        for (repo, npm_name) in packages {
            if repo != "core" {
                assert!(
                    npm_name.starts_with("@macroforge/"),
                    "Expected {} to have @macroforge/ scope, got {}",
                    repo,
                    npm_name
                );
            }
        }
    }

    #[test]
    fn test_all_crates_have_prefix() {
        let packages = crate_packages();

        for (repo, crate_name) in packages {
            assert!(
                crate_name.starts_with("macroforge_ts_"),
                "Expected {} to have macroforge_ts_ prefix, got {}",
                repo,
                crate_name
            );
        }
    }
}
