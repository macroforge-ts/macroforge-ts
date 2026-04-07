//! Commit command — stages and commits all changes in the monorepo

use crate::cli::CommitArgs;
use crate::core::config::Config;
use crate::core::shell;
use crate::utils::format;
use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::Input;

pub fn run(args: CommitArgs) -> Result<()> {
    let config = Config::load()?;
    let root = &config.root;
    let versions = &config.versions;

    // Get current version for default commit message
    let version = versions
        .get_local("core")
        .map(|s| s.to_string())
        .unwrap_or_else(|| "0.0.0".to_string());

    // Check for changes
    let status = shell::git::status(root)?;
    if status.trim().is_empty() {
        format::warning("Nothing to commit");
        return Ok(());
    }

    // Show status
    format::header("Commit");
    let changed_files: Vec<&str> = status.lines().collect();
    println!("{} files changed", changed_files.len().to_string().yellow());
    if changed_files.len() <= 10 {
        for file in &changed_files {
            println!("  {}", file.dimmed());
        }
    }
    println!();

    // Get commit message
    let message = if let Some(msg) = args.message {
        msg
    } else if args.dry_run {
        format!("Bump to {}", version)
    } else {
        Input::new()
            .with_prompt("Commit message")
            .default(format!("Bump to {}", version))
            .interact_text()?
    };

    if args.dry_run {
        format::info(&format!(
            "[dry-run] git add -A && git commit -m {:?}",
            message
        ));
        return Ok(());
    }

    // Stage and commit
    shell::git::add_all(root).context("git add -A failed")?;
    shell::git::commit(root, &message).context("git commit failed")?;

    format::success(&format!("Committed: {}", message));
    Ok(())
}
