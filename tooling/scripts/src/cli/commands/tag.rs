//! Tag command: tags the release on main and pushes the tag, which CI
//! verifies and publishes.

use crate::cli::args::TagArgs;
use crate::core::config::Config;
use crate::core::manifests;
use crate::core::shell;
use crate::utils::format;
use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::Confirm;

/// The branch releases are tagged on; the pipeline refuses tags elsewhere.
const RELEASE_BRANCH: &str = "main";

/// Entry point for `mf tag`: tags origin's main at the release version.
pub fn run(args: &TagArgs) -> Result<()> {
    let config = Config::load()?;
    let root = &config.root;

    let version = manifests::current_version(root)?;
    let tag = format!("v{version}");

    format::header("Tag");
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

    let remote_tag = shell::git::remote_tag_commit(root, &tag)?;
    let local_tag = shell::git::commit_of(root, &format!("refs/tags/{tag}"))?;
    if !args.retag {
        if remote_tag.is_some() {
            anyhow::bail!(
                "{tag} is already on origin; pass --retag to move it to HEAD, or bump the version"
            );
        }
        if let Some(tagged) = &local_tag
            && *tagged != head
        {
            anyhow::bail!(
                "A local {tag} points at {tagged}, not HEAD; pass --retag to move it to HEAD"
            );
        }
    }
    if let Some(tagged) = &remote_tag
        && *tagged != head
    {
        // Publishing skips what a registry already has, so a retag after a
        // partial release publishes the rest from different code.
        format::warning(&format!(
            "{tag} is at {tagged}. Anything its pipeline already published stays as \
             that commit built it; only the rest publishes from {head}."
        ));
    }

    let action = if remote_tag.is_some() {
        format!("Move {tag} to {head} and push it?")
    } else {
        format!("Tag {head} as {tag} and push the tag?")
    };
    if !args.yes
        && !args.dry_run
        && !Confirm::new()
            .with_prompt(action)
            .default(false)
            .interact()?
    {
        format::warning("Aborted");
        return Ok(());
    }

    if args.dry_run {
        if remote_tag.is_some() {
            format::info(&format!("[dry-run] delete {tag} on origin"));
        }
        if local_tag.is_some() {
            format::info(&format!("[dry-run] git tag --delete {tag}"));
        }
        format::info(&format!("[dry-run] git tag {tag}"));
        format::info(&format!("[dry-run] git push origin {tag}"));
        return Ok(());
    }

    if remote_tag.is_some() {
        shell::glab::delete_tag(root, &tag)
            .with_context(|| format!("Failed to delete {tag} on origin"))?;
        format::success(&format!("Deleted {tag} on origin"));
    }
    if local_tag.is_some() {
        shell::git::delete_tag(root, &tag)
            .with_context(|| format!("Failed to delete the local {tag}"))?;
    }
    shell::git::tag(root, &tag).with_context(|| format!("Failed to create tag {tag}"))?;
    format::success(&format!("Tagged {tag}"));
    shell::git::push_tag(root, &tag).with_context(|| format!("Failed to push tag {tag}"))?;
    format::success(&format!(
        "Pushed {tag}; its pipeline verifies and publishes it"
    ));

    Ok(())
}
