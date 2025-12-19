#!/usr/bin/env rust-script
//! Interactive commit script for releasing packages.
//!
//! Handles git add, commit, tag, and push for each repo with proper error handling
//! and user prompts.
//!
//! ```cargo
//! [dependencies]
//! clap = { version = "4", features = ["derive"] }
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! colored = "2"
//! dialoguer = "0.11"
//! ```

use clap::Parser;
use colored::*;
use dialoguer::{Confirm, Input, Select};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

// ============================================================================
// CLI
// ============================================================================

#[derive(Parser, Debug)]
#[command(name = "commit")]
#[command(about = "Interactive commit and push for released packages")]
struct Args {
    /// Version to tag (reads from versions.json if not provided)
    #[arg(short, long)]
    version: Option<String>,

    /// Repos to commit (comma-separated, or 'all', 'rust', 'ts')
    #[arg(short, long, default_value = "all")]
    repos: String,

    /// Skip confirmation prompts
    #[arg(short = 'y', long)]
    yes: bool,

    /// Dry run - show what would be done without doing it
    #[arg(long)]
    dry_run: bool,

    /// Custom commit message (will prompt if not provided)
    #[arg(short, long)]
    message: Option<String>,
}

// ============================================================================
// Repository Configuration
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
enum RepoType {
    Rust,
    Ts,
    Website,
    Tooling,
    Extension,
}

#[derive(Debug, Clone)]
struct Repo {
    name: &'static str,
    path: &'static str,
    repo_type: RepoType,
}

fn all_repos() -> Vec<Repo> {
    vec![
        Repo { name: "core", path: "crates/macroforge_ts", repo_type: RepoType::Rust },
        Repo { name: "macros", path: "crates/macroforge_ts_macros", repo_type: RepoType::Rust },
        Repo { name: "syn", path: "crates/macroforge_ts_syn", repo_type: RepoType::Rust },
        Repo { name: "template", path: "crates/macroforge_ts_quote", repo_type: RepoType::Rust },
        Repo { name: "shared", path: "packages/shared", repo_type: RepoType::Ts },
        Repo { name: "vite-plugin", path: "packages/vite-plugin", repo_type: RepoType::Ts },
        Repo { name: "typescript-plugin", path: "packages/typescript-plugin", repo_type: RepoType::Ts },
        Repo { name: "svelte-language-server", path: "packages/svelte-language-server", repo_type: RepoType::Ts },
        Repo { name: "svelte-preprocessor", path: "packages/svelte-preprocessor", repo_type: RepoType::Ts },
        Repo { name: "mcp-server", path: "packages/mcp-server", repo_type: RepoType::Ts },
        Repo { name: "website", path: "website", repo_type: RepoType::Website },
        Repo { name: "tooling", path: "tooling", repo_type: RepoType::Tooling },
        Repo { name: "zed-extensions", path: "crates/extensions", repo_type: RepoType::Extension },
    ]
}

fn parse_repos(arg: &str) -> Vec<Repo> {
    let all = all_repos();
    match arg {
        "all" => all,
        "rust" => all.into_iter().filter(|r| r.repo_type == RepoType::Rust).collect(),
        "ts" => all.into_iter().filter(|r| r.repo_type == RepoType::Ts).collect(),
        names => {
            let names: Vec<&str> = names.split(',').map(|s| s.trim()).collect();
            all.into_iter().filter(|r| names.contains(&r.name)).collect()
        }
    }
}

// ============================================================================
// Version Cache
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Default)]
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

    fn get(&self, name: &str) -> Option<&String> {
        self.0.get(name)
    }
}

// ============================================================================
// Git Operations
// ============================================================================

fn run_git(args: &[&str], cwd: &Path) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .map_err(|e| format!("Failed to run git: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(stdout)
    } else {
        Err(format!("{}\n{}", stdout, stderr).trim().to_string())
    }
}

fn run_git_interactive(args: &[&str], cwd: &Path) -> Result<(), String> {
    let status = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| format!("Failed to run git: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("git {} failed", args.join(" ")))
    }
}

fn get_status(cwd: &Path) -> Result<String, String> {
    run_git(&["status", "--short"], cwd)
}

fn has_changes(cwd: &Path) -> bool {
    get_status(cwd).map(|s| !s.trim().is_empty()).unwrap_or(false)
}

fn get_current_branch(cwd: &Path) -> Result<String, String> {
    run_git(&["branch", "--show-current"], cwd).map(|s| s.trim().to_string())
}

fn tag_exists_locally(tag: &str, cwd: &Path) -> bool {
    run_git(&["tag", "-l", tag], cwd)
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false)
}

fn tag_exists_remotely(tag: &str, cwd: &Path) -> bool {
    run_git(&["ls-remote", "--tags", "origin", tag], cwd)
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false)
}

fn get_unpushed_count(cwd: &Path) -> usize {
    run_git(&["rev-list", "@{u}..HEAD", "--count"], cwd)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}

fn has_unpushed_commits(cwd: &Path) -> bool {
    get_unpushed_count(cwd) > 0
}

// ============================================================================
// Commit Flow
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
enum Action {
    Continue,
    Skip,
    Abort,
}

fn prompt_action(message: &str) -> Action {
    let items = vec!["Continue", "Skip this repo", "Abort"];
    let selection = Select::new()
        .with_prompt(message)
        .items(&items)
        .default(0)
        .interact()
        .unwrap_or(2);

    match selection {
        0 => Action::Continue,
        1 => Action::Skip,
        _ => Action::Abort,
    }
}

fn commit_repo(
    root: &Path,
    repo: &Repo,
    version: &str,
    commit_msg: &str,
    auto_yes: bool,
    dry_run: bool,
) -> Result<Action, String> {
    let repo_path = root.join(repo.path);
    let tag = format!("v{}", version);

    println!("\n{}", "─".repeat(60).dimmed());
    println!("{} {}", "Repository:".bold(), repo.name.cyan());
    println!("{} {}", "Path:".bold(), repo.path.dimmed());
    println!("{} {}", "Version:".bold(), version.green());
    println!("{}", "─".repeat(60).dimmed());

    // Check if repo directory exists
    if !repo_path.exists() {
        println!("{} Repository path does not exist", "⚠".yellow());
        return Ok(Action::Skip);
    }

    // Check for changes
    if !has_changes(&repo_path) {
        let unpushed = get_unpushed_count(&repo_path);
        if unpushed > 0 {
            println!("{} No changes to commit, but {} unpushed commit{}",
                "ℹ".blue(),
                unpushed.to_string().yellow(),
                if unpushed == 1 { "" } else { "s" }
            );

            if !auto_yes {
                if !Confirm::new()
                    .with_prompt("Push unpushed commits?")
                    .default(true)
                    .interact()
                    .unwrap_or(false)
                {
                    return Ok(Action::Skip);
                }
            }

            if dry_run {
                println!("\n{} Would run: git push", "[dry-run]".yellow());
                return Ok(Action::Continue);
            }

            print!("  {} Pushing commits... ", "→".blue());
            io::stdout().flush().ok();
            match run_git(&["push"], &repo_path) {
                Ok(_) => println!("{}", "✓".green()),
                Err(e) => {
                    println!("{}", "✗".red());
                    println!("    {}", e.dimmed());
                    return Ok(Action::Skip);
                }
            }
            println!("\n  {} {} pushed!", "✓".green().bold(), repo.name.cyan());
        } else {
            println!("{} No changes to commit", "ℹ".blue());
        }
        return Ok(Action::Continue);
    }

    // Show status
    println!("\n{}", "Changes:".bold());
    let status = get_status(&repo_path).unwrap_or_default();
    for line in status.lines().take(20) {
        println!("  {}", line);
    }
    if status.lines().count() > 20 {
        println!("  {} more files...", format!("... and {}", status.lines().count() - 20).dimmed());
    }

    // Confirm
    if !auto_yes {
        println!();
        if !Confirm::new()
            .with_prompt("Commit these changes?")
            .default(true)
            .interact()
            .unwrap_or(false)
        {
            return Ok(Action::Skip);
        }
    }

    if dry_run {
        let would_tag = !tag_exists_remotely(&tag, &repo_path);
        println!("\n{} Would run:", "[dry-run]".yellow());
        println!("  git add -A");
        println!("  git commit -m \"{}\"", commit_msg);
        if would_tag {
            println!("  git tag {}", tag);
            println!("  git push && git push --tags");
        } else {
            println!("  {} (tag {} already exists)", "skip tagging".dimmed(), tag);
            println!("  git push");
        }
        return Ok(Action::Continue);
    }

    // Stage all changes
    print!("  {} Staging changes... ", "→".blue());
    io::stdout().flush().ok();
    run_git(&["add", "-A"], &repo_path)?;
    println!("{}", "✓".green());

    // Commit
    print!("  {} Committing... ", "→".blue());
    io::stdout().flush().ok();
    match run_git(&["commit", "-m", commit_msg], &repo_path) {
        Ok(_) => println!("{}", "✓".green()),
        Err(e) => {
            println!("{}", "✗".red());
            println!("    {}", e.dimmed());
            match prompt_action("Commit failed. What do you want to do?") {
                Action::Continue => {}
                Action::Skip => return Ok(Action::Skip),
                Action::Abort => return Ok(Action::Abort),
            }
        }
    }

    // Check if this is a new version (tag doesn't exist remotely)
    let should_tag = !tag_exists_remotely(&tag, &repo_path);

    if should_tag {
        // Clean up local tag if it exists
        if tag_exists_locally(&tag, &repo_path) {
            print!("  {} Deleting stale local tag {}... ", "→".blue(), tag.yellow());
            io::stdout().flush().ok();
            match run_git(&["tag", "-d", &tag], &repo_path) {
                Ok(_) => println!("{}", "✓".green()),
                Err(e) => {
                    println!("{}", "✗".red());
                    println!("    {}", e.dimmed());
                }
            }
        }

        // Create tag
        print!("  {} Creating tag {}... ", "→".blue(), tag.cyan());
        io::stdout().flush().ok();
        match run_git(&["tag", &tag], &repo_path) {
            Ok(_) => println!("{}", "✓".green()),
            Err(e) => {
                println!("{}", "✗".red());
                println!("    {}", e.dimmed());
                match prompt_action("Failed to create tag. What do you want to do?") {
                    Action::Continue => {}
                    Action::Skip => return Ok(Action::Skip),
                    Action::Abort => return Ok(Action::Abort),
                }
            }
        }
    } else {
        println!("  {} Tag {} already exists, skipping", "ℹ".blue(), tag.dimmed());
    }

    // Push commits
    print!("  {} Pushing commits... ", "→".blue());
    io::stdout().flush().ok();
    match run_git(&["push"], &repo_path) {
        Ok(_) => println!("{}", "✓".green()),
        Err(e) => {
            println!("{}", "✗".red());
            println!("    {}", e.dimmed());
            match prompt_action("Push failed. What do you want to do?") {
                Action::Continue => {}
                Action::Skip => return Ok(Action::Skip),
                Action::Abort => return Ok(Action::Abort),
            }
        }
    }

    // Push tags only if we created one
    if should_tag {
        print!("  {} Pushing tags... ", "→".blue());
        io::stdout().flush().ok();
        match run_git(&["push", "--tags"], &repo_path) {
            Ok(_) => println!("{}", "✓".green()),
            Err(e) => {
                println!("{}", "✗".red());
                println!("    {}", e.dimmed());
                match prompt_action("Push tags failed. What do you want to do?") {
                    Action::Continue => {}
                    Action::Skip => return Ok(Action::Skip),
                    Action::Abort => return Ok(Action::Abort),
                }
            }
        }
    }

    println!("\n  {} {} committed!", "✓".green().bold(), repo.name.cyan());

    Ok(Action::Continue)
}

// ============================================================================
// Main
// ============================================================================

fn main() -> ExitCode {
    let args = Args::parse();

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

    let repos = parse_repos(&args.repos);
    if repos.is_empty() {
        eprintln!("{}", "No repos selected".red());
        return ExitCode::FAILURE;
    }

    // Load versions
    let versions = VersionsCache::load(&root);

    // Get version (from arg or from versions.json using first repo)
    let version = args.version.clone().or_else(|| {
        versions.get(repos[0].name).cloned()
    });

    let version = match version {
        Some(v) => v,
        None => {
            eprintln!("{} No version specified and none found in versions.json", "Error:".red());
            eprintln!("Run with: {} {}", "--version".cyan(), "<version>".dimmed());
            return ExitCode::FAILURE;
        }
    };

    println!("{}", "═".repeat(60));
    println!("{}", "  Macroforge Release Commit".bold());
    println!("{}", "═".repeat(60));
    println!();
    println!("{} {}", "Version:".bold(), version.green());
    println!("{} {}", "Repos:".bold(), repos.iter().map(|r| r.name).collect::<Vec<_>>().join(", ").cyan());

    if args.dry_run {
        println!("{} {}", "Mode:".bold(), "DRY RUN".yellow());
    }

    // Get commit message
    let default_msg = format!("Bump to {}", version);
    let commit_msg = match args.message {
        Some(msg) => msg,
        None => {
            println!();
            Input::new()
                .with_prompt("Commit message")
                .default(default_msg.clone())
                .interact_text()
                .unwrap_or(default_msg)
        }
    };

    println!("{} {}", "Message:".bold(), commit_msg.cyan());

    let mut completed = Vec::new();
    let mut skipped = Vec::new();

    for repo in &repos {
        match commit_repo(&root, repo, &version, &commit_msg, args.yes, args.dry_run) {
            Ok(Action::Continue) => completed.push(repo.name),
            Ok(Action::Skip) => {
                println!("  {} Skipping {}", "→".yellow(), repo.name);
                skipped.push(repo.name);
            }
            Ok(Action::Abort) => {
                println!("\n{} Aborted by user", "✗".red());
                break;
            }
            Err(e) => {
                eprintln!("\n{} {}: {}", "Error".red(), repo.name, e);
                break;
            }
        }
    }

    // Summary
    println!("\n{}", "═".repeat(60));
    println!("{}", "  Summary".bold());
    println!("{}", "═".repeat(60));

    if !completed.is_empty() {
        println!("\n{} {}", "Completed:".green().bold(), completed.join(", "));
    }
    if !skipped.is_empty() {
        println!("{} {}", "Skipped:".yellow().bold(), skipped.join(", "));
    }

    if args.dry_run {
        println!("\n{}", "This was a dry run. No changes were made.".yellow());
    }

    println!();

    ExitCode::SUCCESS
}
