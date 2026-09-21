//! Manifest manipulation command
//!
//! Handles reading/writing versions to package.json and Cargo.toml,
//! managing versions.json cache, and swapping dependency paths.

use crate::cli::ManifestArgs;
use crate::core::config::Config;
use crate::core::manifests;
use crate::core::shell;
use crate::diagnostics::deno_lint;
use anyhow::{Context, Result};

/// Entry point for `mf manifest`: dispatches manifest subcommands (versions, swaps, linking).
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
            let config_path = deno_lint::governing_config(&config.root);
            shell::deno::deno_fmt(
                &config.root,
                &["tooling/versions.json"],
                config_path.as_deref(),
            )
            .context("Failed to format tooling/versions.json")?;
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

        crate::cli::ManifestCommands::DumpVersions => {
            let json = serde_json::to_string_pretty(&versions)?;
            println!("{}", json);
        }

        crate::cli::ManifestCommands::UpdateZed => {
            manifests::update_zed_extensions(&config.root, &versions)?;
        }
    }

    Ok(())
}
