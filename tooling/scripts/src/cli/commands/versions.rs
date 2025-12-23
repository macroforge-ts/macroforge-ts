//! Fetch latest versions from npm and crates.io and update versions.json

use crate::cli::VersionsArgs;
use crate::core::config::{self, Config};
use crate::core::registry;
use crate::utils::format;
use anyhow::Result;
use colored::Colorize;
use std::io::{self, Write};

pub fn run(args: VersionsArgs) -> Result<()> {
    let config = Config::load()?;
    let mut versions = config.versions.clone();

    println!("{}", "═".repeat(60));
    println!("{}", "Fetching latest versions from registries".bold());
    println!("{}", "═".repeat(60));

    let mut updated = Vec::new();
    let mut unchanged = Vec::new();
    let mut failed = Vec::new();

    // npm packages
    let npm_packages = config::npm_package_names();
    println!("\n{}", "npm packages:".bold());

    for (repo, npm_pkg) in npm_packages.iter() {
        print!("  {} {}... ", "→".blue(), npm_pkg);
        io::stdout().flush()?;

        match registry::npm_version(npm_pkg) {
            Ok(Some(version)) => {
                let old_registry = versions
                    .get_registry(repo)
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                let old_local = versions
                    .get_local(repo)
                    .map(|s| s.to_string())
                    .unwrap_or_default();

                let registry_changed = old_registry != version;
                let local_out_of_sync = old_local != version;

                if registry_changed || local_out_of_sync {
                    if registry_changed {
                        println!("{} (was {})", version.green(), old_registry.dimmed());
                    } else {
                        println!("{} (local was {})", version.green(), old_local.dimmed());
                    }

                    if !args.check_only {
                        versions.set_registry(repo, &version);
                        versions.set_local(repo, &version);
                    }
                    updated.push((repo.to_string(), version));
                } else {
                    println!("{}", version.dimmed());
                    unchanged.push(repo.to_string());
                }
            }
            Ok(None) => {
                println!("{}", "not found".yellow());
                failed.push(repo.to_string());
            }
            Err(_) => {
                println!("{}", "failed".red());
                failed.push(repo.to_string());
            }
        }
    }

    // crates.io packages
    let crate_packages = config::crate_package_names();
    println!("\n{}", "crates.io packages:".bold());

    for (repo, crate_name) in crate_packages.iter() {
        print!("  {} {}... ", "→".blue(), crate_name);
        io::stdout().flush()?;

        match registry::crates_version(crate_name) {
            Ok(Some(version)) => {
                let old_registry = versions
                    .get_registry(repo)
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                let old_local = versions
                    .get_local(repo)
                    .map(|s| s.to_string())
                    .unwrap_or_default();

                let registry_changed = old_registry != version;
                let local_out_of_sync = old_local != version;

                if registry_changed || local_out_of_sync {
                    if registry_changed {
                        println!("{} (was {})", version.green(), old_registry.dimmed());
                    } else {
                        println!("{} (local was {})", version.green(), old_local.dimmed());
                    }

                    if !args.check_only {
                        versions.set_registry(repo, &version);
                        versions.set_local(repo, &version);
                    }
                    updated.push((repo.to_string(), version));
                } else {
                    println!("{}", version.dimmed());
                    unchanged.push(repo.to_string());
                }
            }
            Ok(None) => {
                println!("{}", "not found".yellow());
                failed.push(repo.to_string());
            }
            Err(_) => {
                println!("{}", "failed".red());
                failed.push(repo.to_string());
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

        if !args.check_only {
            // Save versions
            versions.save(&config.root)?;
            format::success("versions.json updated");
        }
    }

    if !unchanged.is_empty() {
        println!(
            "\n{}: {}",
            "Unchanged".dimmed(),
            unchanged.join(", ")
        );
    }

    if !failed.is_empty() {
        println!("\n{}: {}", "Failed".red(), failed.join(", "));
    }

    Ok(())
}
