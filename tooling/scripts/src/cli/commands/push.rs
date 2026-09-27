//! Push command: tags the release on main and pushes the tag, which CI
//! verifies and publishes.

use crate::cli::args::PushArgs;
use crate::core::config::Config;
use crate::core::shell;
use crate::utils::format;
use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::Confirm;

/// The branch releases are tagged on; the pipeline refuses tags elsewhere.
const RELEASE_BRANCH: &str = "main";

/// Entry point for `mf push`: tags origin's main at the manifest version.
pub fn run(args: &PushArgs) -> Result<()> {
    let config = Config::load()?;
    let root = &config.root;

    let version = config
        .versions
        .get_local("core")
        .context("No local version for 'core'")?;
    let tag = format!("v{version}");

    format::header("Push");
    println!("  {} {}", "version:".dimmed(), version.green());
    println!("  {} {}", "tag:".dimmed(), tag.cyan());
    println!();

    let branch = shell::git::current_branch(root)?;
    if branch != RELEASE_BRANCH {
        anyhow::bail!("Releases are tagged on {RELEASE_BRANCH}, not {branch}");
    }
    if !shell::git::status(root)?.trim().is_empty() {
        anyhow::bail!("There are uncommitted changes; commit them through a merge request first");
    }

    // The tag must name the commit origin's main holds: the one its merge
    // request pipeline verified.
    shell::git::fetch(root, RELEASE_BRANCH)?;
    let head = shell::git::commit_of(root, "HEAD")?.context("HEAD has no commit")?;
    let remote = shell::git::commit_of(root, "FETCH_HEAD")?.context("FETCH_HEAD is missing")?;
    if head != remote {
        anyhow::bail!(
            "Local {RELEASE_BRANCH} is at {head}, origin/{RELEASE_BRANCH} at {remote}; pull or push first"
        );
    }

    if shell::git::tag_exists_remote(root, &tag)? {
        anyhow::bail!("{tag} is already on origin; run `pixi run bump` for a new version");
    }
    let local_tag = shell::git::commit_of(root, &format!("refs/tags/{tag}"))?;
    if let Some(tagged) = &local_tag
        && *tagged != head
    {
        anyhow::bail!("A local {tag} points at {tagged}, not HEAD; delete it or bump the version");
    }

    if !args.yes
        && !args.dry_run
        && !Confirm::new()
            .with_prompt(format!("Tag {head} as {tag} and push the tag?"))
            .default(false)
            .interact()?
    {
        format::warning("Aborted");
        return Ok(());
    }

    if args.dry_run {
        if local_tag.is_none() {
            format::info(&format!("[dry-run] git tag {tag}"));
        }
        format::info(&format!("[dry-run] git push origin {tag}"));
        return Ok(());
    }

    if local_tag.is_none() {
        shell::git::tag(root, &tag).with_context(|| format!("Failed to create tag {tag}"))?;
        format::success(&format!("Tagged {tag}"));
    }
    shell::git::push_tag(root, &tag).with_context(|| format!("Failed to push tag {tag}"))?;
    format::success(&format!(
        "Pushed {tag}; its pipeline verifies and publishes it"
    ));

    Ok(())
}
