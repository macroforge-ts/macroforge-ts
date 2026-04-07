//! Push command — tags and pushes the monorepo

use crate::cli::PushArgs;
use crate::core::config::Config;
use crate::core::shell;
use crate::utils::format;
use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::Confirm;

pub fn run(args: &PushArgs) -> Result<()> {
    let config = Config::load()?;
    let root = &config.root;
    let versions = &config.versions;

    let version = versions
        .get_local("core")
        .map(|s| s.to_string())
        .unwrap_or_else(|| "0.0.0".to_string());

    let tag = format!("v{}", version);
    let unpushed = shell::git::unpushed_count(root);
    let status = shell::git::status(root)?;
    let has_uncommitted = !status.trim().is_empty();

    format::header("Push");
    println!("  {} {}", "version:".dimmed(), version.green());
    println!("  {} {}", "tag:".dimmed(), tag.cyan());
    println!(
        "  {} {}",
        "unpushed:".dimmed(),
        unpushed.to_string().yellow()
    );
    if has_uncommitted {
        format::warning("There are uncommitted changes — run `mf commit` first");
    }
    println!();

    if unpushed == 0 && !has_uncommitted {
        format::warning("Nothing to push");
    }

    if !args.yes && !args.dry_run {
        if !Confirm::new()
            .with_prompt("Proceed?")
            .default(false)
            .interact()?
        {
            format::warning("Aborted");
            return Ok(());
        }
    }

    if args.dry_run {
        format::info(&format!("[dry-run] git tag -f {}", tag));
        format::info("[dry-run] git push");
        format::info(&format!("[dry-run] git push origin {}", tag));
        return Ok(());
    }

    // Create tag
    shell::git::tag_force(root, &tag).context(format!("Failed to create tag {}", tag))?;
    format::success(&format!("Tagged {}", tag));

    // Push commits
    if shell::git::has_upstream(root) {
        shell::git::push(root).context("git push failed")?;
    } else if let Some(branch) = shell::git::current_branch(root) {
        shell::git::push_with_upstream(root, &branch).context("git push -u failed")?;
    } else {
        shell::git::push(root).context("git push failed")?;
    }
    format::success("Pushed commits");

    // Push tag (delete remote first if it exists, to allow re-tagging)
    if shell::git::tag_exists_remote(root, &tag) {
        shell::git::delete_remote_tag(root, &tag).ok();
    }
    shell::git::push_tag(root, &tag).context(format!("Failed to push tag {}", tag))?;
    format::success(&format!("Pushed tag {}", tag));

    Ok(())
}
