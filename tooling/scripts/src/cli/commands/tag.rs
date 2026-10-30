//! Tag command: tags the release on main and pushes the tag to a forge, whose
//! CI verifies and publishes it.

use crate::cli::args::{Forge, TagArgs};
use crate::core::config::Config;
use crate::core::manifests;
use crate::core::shell;
use crate::utils::format;
use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::Confirm;
use std::path::Path;

/// The branch releases are tagged on; the pipeline refuses tags elsewhere.
const RELEASE_BRANCH: &str = "main";

/// Entry point for `mf tag`: tags the forge's main at the release version.
pub fn run(args: &TagArgs) -> Result<()> {
    let config = Config::load()?;
    let root = &config.root;

    let version = manifests::current_version(root)?;
    let tag = format!("v{version}");
    let remote = forge_remote(root, args.forge)?;

    format::header("Tag");
    println!("  {} {}", "version:".dimmed(), version.green());
    println!("  {} {}", "tag:".dimmed(), tag.cyan());
    println!("  {} {}", "remote:".dimmed(), remote.cyan());
    println!();

    let branch = shell::git::current_branch(root)?;
    if branch != RELEASE_BRANCH {
        anyhow::bail!("Releases are tagged on {RELEASE_BRANCH}, not {branch}");
    }
    if !shell::git::status(root)?.trim().is_empty() {
        anyhow::bail!("There are uncommitted changes; commit them through a merge request first");
    }

    // The tag must name the commit the forge's main holds: the one its merge
    // request or pull request CI verified.
    shell::git::fetch(root, &remote, RELEASE_BRANCH)?;
    let head = shell::git::commit_of(root, "HEAD")?.context("HEAD has no commit")?;
    let fetched = shell::git::commit_of(root, "FETCH_HEAD")?.context("FETCH_HEAD is missing")?;
    if head != fetched {
        anyhow::bail!(
            "Local {RELEASE_BRANCH} is at {head}, {remote}/{RELEASE_BRANCH} at {fetched}; pull or push first"
        );
    }

    let remote_tag = shell::git::remote_tag_commit(root, &remote, &tag)?;
    let local_tag = shell::git::commit_of(root, &format!("refs/tags/{tag}"))?;
    if !args.retag {
        if remote_tag.is_some() {
            anyhow::bail!(
                "{tag} is already on {remote}; pass --retag to move it to HEAD, or bump the version"
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
            "{tag} is at {tagged}. Anything its release already published stays as \
             that commit built it; only the rest publishes from {head}."
        ));
    }

    let action = if remote_tag.is_some() {
        format!("Move {tag} to {head} and push it to {remote}?")
    } else {
        format!("Tag {head} as {tag} and push the tag to {remote}?")
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
            format::info(&format!("[dry-run] delete {tag} on {remote}"));
        }
        if local_tag.is_some() {
            format::info(&format!("[dry-run] git tag --delete {tag}"));
        }
        format::info(&format!("[dry-run] git tag {tag}"));
        format::info(&format!("[dry-run] git push {remote} {tag}"));
        return Ok(());
    }

    if remote_tag.is_some() {
        match args.forge {
            // Release tags are protected there, and GitLab refuses to delete a
            // protected tag through `git push`.
            Forge::GitLab => shell::glab::delete_tag(root, &tag),
            Forge::GitHub => shell::git::delete_remote_tag(root, &remote, &tag),
        }
        .with_context(|| format!("Failed to delete {tag} on {remote}"))?;
        format::success(&format!("Deleted {tag} on {remote}"));
    }
    if local_tag.is_some() {
        shell::git::delete_tag(root, &tag)
            .with_context(|| format!("Failed to delete the local {tag}"))?;
    }
    shell::git::tag(root, &tag).with_context(|| format!("Failed to create tag {tag}"))?;
    format::success(&format!("Tagged {tag}"));
    shell::git::push_tag(root, &remote, &tag)
        .with_context(|| format!("Failed to push tag {tag} to {remote}"))?;
    format::success(&format!(
        "Pushed {tag} to {remote}; its CI verifies and publishes it"
    ));

    Ok(())
}

/// The one remote whose URL is on the forge's host.
fn forge_remote(root: &Path, forge: Forge) -> Result<String> {
    let host = forge.host();
    let mut matching = shell::git::remotes(root)?
        .into_iter()
        .filter(|(_, url)| url_host(url) == Some(host))
        .map(|(name, _)| name);
    let remote = matching
        .next()
        .with_context(|| format!("No git remote points at {host}"))?;
    if let Some(other) = matching.next() {
        anyhow::bail!("Both {remote} and {other} point at {host}; keep one remote per forge");
    }
    Ok(remote)
}

/// The host a git URL names: `scheme://[user@]host[:port]/path` or the scp-like
/// `[user@]host:path`.
fn url_host(url: &str) -> Option<&str> {
    let authority = match url.split_once("://") {
        Some((_, rest)) => rest.split('/').next(),
        None => url.split(':').next(),
    }?;
    let host = authority.rsplit('@').next()?.split(':').next()?;
    (!host.is_empty()).then_some(host)
}

#[cfg(test)]
mod tests {
    use super::url_host;

    #[test]
    fn https_url() {
        assert_eq!(
            url_host("https://github.com/macroforge-ts/macroforge-ts.git"),
            Some("github.com")
        );
    }

    #[test]
    fn ssh_url_with_user_and_port() {
        assert_eq!(
            url_host("ssh://git@gitlab.com:22/macroforge-ts/macroforge-ts.git"),
            Some("gitlab.com")
        );
    }

    #[test]
    fn scp_like_url() {
        assert_eq!(
            url_host("git@gitlab.com:macroforge-ts/macroforge-ts.git"),
            Some("gitlab.com")
        );
    }

    #[test]
    fn https_url_with_credentials() {
        assert_eq!(
            url_host("https://user:secret@github.com/owner/repo.git"),
            Some("github.com")
        );
    }

    #[test]
    fn local_path_names_no_forge() {
        assert_ne!(url_host("/srv/git/repo.git"), Some("github.com"));
        assert_eq!(url_host(""), None);
    }
}
