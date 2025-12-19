#!/usr/bin/env rust-script
//! Fetch latest versions from npm and crates.io and update versions.json.
//!
//! ```cargo
//! [dependencies]
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! colored = "2"
//! ```

use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

// ============================================================================
// Package Mappings
// ============================================================================

/// Map repo names to their npm package names
fn npm_packages() -> HashMap<&'static str, &'static str> {
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

/// Map repo names to their crates.io crate names
fn crate_packages() -> HashMap<&'static str, &'static str> {
    [
        ("syn", "macroforge_ts_syn"),
        ("template", "macroforge_ts_quote"),
        ("macros", "macroforge_ts_macros"),
    ]
    .into_iter()
    .collect()
}

// ============================================================================
// Version Fetching
// ============================================================================

/// Fetch latest version from npm
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

/// Fetch latest version from crates.io
fn fetch_crate_version(crate_name: &str) -> Option<String> {
    let output = Command::new("cargo")
        .args(["search", crate_name, "--limit", "1"])
        .output()
        .ok()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Output format: "crate_name = \"version\""
        for line in stdout.lines() {
            if line.starts_with(crate_name) {
                // Extract version from: crate_name = "version"
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

// ============================================================================
// Versions Cache
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct VersionsCache(HashMap<String, String>);

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
        fs::write(path, content + "\n")
    }

    fn get(&self, name: &str) -> Option<&String> {
        self.0.get(name)
    }

    fn set(&mut self, name: &str, version: &str) {
        self.0.insert(name.to_string(), version.to_string());
    }
}

// ============================================================================
// Main
// ============================================================================

fn main() -> ExitCode {
    // Get root directory
    let root = std::env::var("MACROFORGE_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let cwd = std::env::current_dir().unwrap();
            cwd.ancestors()
                .find(|p| p.join("pixi.toml").exists())
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| cwd.clone())
        });

    println!("{}", "═".repeat(60));
    println!("{}", "Fetching latest versions from registries".bold());
    println!("{}", "═".repeat(60));

    let mut versions = VersionsCache::load(&root);

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
                let old = versions.get(repo).cloned();
                if old.as_ref() != Some(&version) {
                    println!("{} (was {})", version.green(), old.unwrap_or_else(|| "none".to_string()).dimmed());
                    versions.set(repo, &version);
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
                let old = versions.get(repo).cloned();
                if old.as_ref() != Some(&version) {
                    println!("{} (was {})", version.green(), old.unwrap_or_else(|| "none".to_string()).dimmed());
                    versions.set(repo, &version);
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
    }

    if !unchanged.is_empty() {
        println!("\n{}: {}", "Unchanged".dimmed(), unchanged.join(", "));
    }

    if !failed.is_empty() {
        println!("\n{}: {}", "Failed".red(), failed.join(", "));
    }

    // Save if there were updates
    if !updated.is_empty() {
        if let Err(e) = versions.save(&root) {
            eprintln!("\n{}: {}", "Error saving versions.json".red(), e);
            return ExitCode::FAILURE;
        }
        println!("\n{} versions.json updated", "✓".green());
        println!("\n{} Run {} to apply these versions to all packages.", "Tip:".dimmed(), "pixi run sync".cyan());
    } else {
        println!("\n{} All versions are up to date", "✓".green());
    }

    ExitCode::SUCCESS
}
