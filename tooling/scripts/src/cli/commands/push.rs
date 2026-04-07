//! Push command
//!
//! Commits, tags, and pushes all repos in dependency order without waiting for
//! registry propagation. This is the "commit" half of the release workflow —
//! use `publish-local` afterwards to publish to registries.

use crate::cli::PushArgs;
use crate::core::config::Config;
use crate::core::deps;
use crate::core::repos::RepoType;
use crate::core::shell;
use crate::utils::format;
use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::{Confirm, Input};

pub fn run(args: &PushArgs) -> Result<()> {
    let config = Config::load()?;
    let versions = &config.versions;

    // Build dependency-ordered list
    let dep_order = deps::topo_order(&config.deps)?;

    // Filter repos that have changes or unpushed commits
    let prepped_repos: Vec<_> = config
        .filter_repos(&args.repos)
        .into_iter()
        .cloned()
        .collect();

    // Build queue in dependency order
    let mut queue: Vec<(&str, &crate::core::repos::Repo, String)> = Vec::new();

    for name in &dep_order {
        if let Some(repo) = prepped_repos.iter().find(|r| r.name == *name) {
            let version = versions
                .get_local(name)
                .map(|s| s.to_string())
                .unwrap_or_else(|| "0.0.0".to_string());
            queue.push((name, repo, version));
        }
    }

    // Add any repos not in the dep graph
    for repo in &prepped_repos {
        if !queue.iter().any(|(_, r, _)| r.name == repo.name) {
            let version = versions
                .get_local(&repo.name)
                .map(|s| s.to_string())
                .unwrap_or_else(|| "0.0.0".to_string());
            queue.push((&repo.name, repo, version));
        }
    }

    if queue.is_empty() {
        format::warning("No repos to push");
        return Ok(());
    }

    // Display queue
    format::header("Push");
    println!("{}", "Queue (dependency order):".bold());
    for (i, (name, repo, version)) in queue.iter().enumerate() {
        let type_label = match repo.repo_type {
            RepoType::Rust => "rust".yellow(),
            RepoType::Ts => "ts".cyan(),
            RepoType::Website => "web".magenta(),
            RepoType::Tooling => "tool".blue(),
            RepoType::Extension => "ext".white(),
        };
        let pkg_name = repo
            .npm_name
            .as_deref()
            .or(repo.crate_name.as_deref())
            .unwrap_or(name);
        println!(
            "  {}. {} {} @ {}",
            i + 1,
            type_label,
            pkg_name.bold(),
            version.green(),
        );
    }
    println!();

    // Confirm
    if !args.yes && !args.dry_run {
        if !Confirm::new()
            .with_prompt("Proceed?")
            .default(false)
            .interact()?
        {
            format::warning("Aborted");
            return Ok(());
        }
        println!();
    }

    let total = queue.len();
    let mut committed = 0usize;
    let mut pushed = 0usize;
    let mut skipped = 0usize;

    for (i, (name, repo, version)) in queue.iter().enumerate() {
        format::step(i + 1, total, &format!("{}", name.bold()));

        if !repo.abs_path.exists() {
            format::warning(&format!("  Path missing: {}", repo.abs_path.display()));
            skipped += 1;
            continue;
        }

        let status = shell::git::status(&repo.abs_path)?;
        let has_changes = !status.trim().is_empty();
        let unpushed = shell::git::unpushed_count(&repo.abs_path);

        if !has_changes && unpushed == 0 {
            println!("  {} nothing to commit or push", "─".dimmed());
            skipped += 1;
            continue;
        }

        // Commit if there are changes
        if has_changes {
            // Get commit message
            let message = if let Some(ref msg) = args.message {
                msg.clone()
            } else {
                let default = format!("Bump to {}", version);
                Input::<String>::new()
                    .with_prompt(format!("  Commit message for {}", name.cyan()))
                    .default(default)
                    .interact_text()?
            };

            if args.dry_run {
                format::info(&format!(
                    "  [dry-run] git add -A && git commit -m {:?}",
                    message
                ));
            } else {
                shell::git::add_all(&repo.abs_path).context("git add failed")?;
                shell::git::commit(&repo.abs_path, &message).context("git commit failed")?;
                println!("  {} committed", "✓".green());
                committed += 1;
            }
        }

        // Tag
        let tag = format!("v{}", version);
        if args.dry_run {
            format::info(&format!("  [dry-run] git tag -f {}", tag));
        } else {
            shell::git::tag_force(&repo.abs_path, &tag)
                .context(format!("Failed to create tag {}", tag))?;
            println!("  {} tagged {}", "✓".green(), tag.cyan());
        }

        // Push
        if args.dry_run {
            format::info("  [dry-run] git push && git push tag");
        } else {
            if shell::git::has_upstream(&repo.abs_path) {
                shell::git::push(&repo.abs_path).context("git push failed")?;
            } else if let Some(branch) = shell::git::current_branch(&repo.abs_path) {
                shell::git::push_with_upstream(&repo.abs_path, &branch)
                    .context("git push -u failed")?;
            } else {
                shell::git::push(&repo.abs_path).context("git push failed")?;
            }

            // Delete remote tag if it exists, then push new one
            if shell::git::tag_exists_remote(&repo.abs_path, &tag) {
                shell::git::delete_remote_tag(&repo.abs_path, &tag).ok();
            }
            shell::git::push_tag(&repo.abs_path, &tag)
                .context(format!("Failed to push tag {}", tag))?;

            println!("  {} pushed", "✓".green());
            pushed += 1;
        }
    }

    // Summary
    println!();
    if args.dry_run {
        format::success("Dry run complete");
    } else {
        format::success(&format!(
            "Done: {} committed, {} pushed, {} skipped",
            committed, pushed, skipped
        ));
    }

    Ok(())
}
