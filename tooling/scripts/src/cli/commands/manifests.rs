//! Manifest manipulation command
//!
//! Handles reading/writing versions to package.json and Cargo.toml,
//! managing versions.json cache, and swapping dependency paths.

use crate::cli::ManifestArgs;
use crate::core::config::Config;
use crate::core::manifests;
use crate::utils::format;
use anyhow::Result;
use std::process::Command;

pub fn run(args: ManifestArgs) -> Result<()> {
    let config = Config::load()?;
    let mut versions = config.versions.clone();

    match args.command {
        crate::cli::ManifestCommands::List => {
            // Output repos as JSON
            let repos: Vec<_> = config.repos.values().collect();
            let json = serde_json::to_string_pretty(&repos)?;
            println!("{}", json);
        }

        crate::cli::ManifestCommands::GetVersion { repo, registry } => {
            let version = if registry {
                versions.get_registry(&repo)
            } else {
                versions.get_local(&repo)
            };
            println!("{}", version.unwrap_or(""));
        }

        crate::cli::ManifestCommands::SetVersion {
            repo,
            version,
            registry,
        } => {
            if registry {
                versions.set_registry(&repo, &version);
            } else {
                manifests::set_version(&config, &mut versions, &repo, &version)?;
            }
            versions.save(&config.root)?;
            // Format versions.json with deno fmt
            let _ = Command::new("deno")
                .args(["fmt", "tooling/versions.json"])
                .current_dir(&config.root)
                .output();
        }

        crate::cli::ManifestCommands::ApplyVersions { local } => {
            for repo in config.repos.values() {
                let version = if local {
                    versions.get_local(&repo.name)
                } else {
                    versions.get_registry(&repo.name)
                };

                if let Some(ver) = version {
                    let ver = ver.to_string();
                    manifests::set_version(&config, &mut versions.clone(), &repo.name, &ver)?;
                }
            }
            manifests::update_zed_extensions(&config.root, &versions)?;
        }

        crate::cli::ManifestCommands::SwapLocal => {
            manifests::swap_local(&config.root)?;
            format::success("Swapped to local dependencies");
        }

        crate::cli::ManifestCommands::SwapRegistry => {
            manifests::swap_registry(&config.root, &versions)?;
            format::success("Swapped to registry dependencies");
        }

        crate::cli::ManifestCommands::DumpVersions => {
            let json = serde_json::to_string_pretty(&versions)?;
            println!("{}", json);
        }

        crate::cli::ManifestCommands::UpdateZed => {
            manifests::update_zed_extensions(&config.root, &versions)?;
        }

        crate::cli::ManifestCommands::LinkLocal { repo, deps } => {
            manifests::link_local_deps(&config, &repo, &deps)?;
        }

        crate::cli::ManifestCommands::RestoreRepo { repo } => {
            manifests::restore_repo(&config, &versions, &repo)?;
        }
    }

    Ok(())
}
