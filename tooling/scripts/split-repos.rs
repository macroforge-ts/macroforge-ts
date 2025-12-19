#!/usr/bin/env rust-script
//! Repository Splitting Script
//! This script splits each crate/package into its own repository while preserving git history
//!
//! Prerequisites:
//! - git-filter-repo installed (pip install git-filter-repo)
//! - gh CLI installed and authenticated (brew install gh && gh auth login)
//! - All changes committed and pushed to the main repo
//!
//! ```cargo
//! [dependencies]
//! clap = { version = "4", features = ["derive"] }
//! colored = "2"
//! dialoguer = "0.11"
//! dirs = "5"
//! ```

use clap::{Parser, Subcommand};
use colored::*;
use dialoguer::Confirm;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

const GITHUB_ORG: &str = "macroforge-ts";
const SOURCE_REPO: &str = "https://github.com/macroforge-ts/macroforge-ts.git";

#[derive(Debug, Clone)]
struct Component {
    path: &'static str,
    repo_name: &'static str,
    description: &'static str,
}

const COMPONENTS: &[Component] = &[
    // Rust Crates
    Component {
        path: "crates/macroforge_ts",
        repo_name: "core",
        description: "Core Macroforge TypeScript macro engine",
    },
    Component {
        path: "crates/macroforge_ts_macros",
        repo_name: "macros",
        description: "Procedural macros for Macroforge",
    },
    Component {
        path: "crates/macroforge_ts_quote",
        repo_name: "template",
        description: "Template macro implementation for Macroforge",
    },
    Component {
        path: "crates/macroforge_ts_syn",
        repo_name: "syn",
        description: "Syntax parsing utilities for Macroforge",
    },
    // Zed Extensions
    Component {
        path: "crates/extensions",
        repo_name: "zed-extensions",
        description: "Zed IDE extensions for Macroforge",
    },
    // TypeScript Packages
    Component {
        path: "packages/mcp-server",
        repo_name: "mcp-server",
        description: "MCP server for Macroforge",
    },
    Component {
        path: "packages/shared",
        repo_name: "shared",
        description: "Shared utilities for Macroforge packages",
    },
    Component {
        path: "packages/svelte-language-server",
        repo_name: "svelte-language-server",
        description: "Svelte language server with Macroforge support",
    },
    Component {
        path: "packages/svelte-preprocessor",
        repo_name: "svelte-preprocessor",
        description: "Svelte preprocessor for Macroforge",
    },
    Component {
        path: "packages/typescript-plugin",
        repo_name: "typescript-plugin",
        description: "TypeScript plugin for Macroforge",
    },
    Component {
        path: "packages/vite-plugin",
        repo_name: "vite-plugin",
        description: "Vite plugin for Macroforge",
    },
    // Website
    Component {
        path: "website",
        repo_name: "website",
        description: "Macroforge documentation website",
    },
    // Tooling
    Component {
        path: "tooling",
        repo_name: "tooling",
        description: "Scripts, playground, and tests for Macroforge",
    },
];

#[derive(Parser)]
#[command(name = "split-repos")]
#[command(about = "Split Macroforge monorepo into individual repositories")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Run without creating repos or pushing
    #[arg(long, global = true)]
    dry_run: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// List all components that will be split
    List,
    /// Process a specific component by index
    One {
        /// Component index (0-based)
        index: usize,
    },
    /// Process all components
    All,
}

fn log_info(msg: &str) {
    println!("{} {}", "[INFO]".blue(), msg);
}

fn log_success(msg: &str) {
    println!("{} {}", "[SUCCESS]".green(), msg);
}

fn log_warning(msg: &str) {
    println!("{} {}", "[WARNING]".yellow(), msg);
}

fn log_error(msg: &str) {
    eprintln!("{} {}", "[ERROR]".red(), msg);
}

fn run_cmd(cmd: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute {}: {}", cmd, e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

fn run_cmd_in_dir(dir: &PathBuf, cmd: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .output()
        .map_err(|e| format!("Failed to execute {}: {}", cmd, e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

fn check_prerequisites() -> Result<(), String> {
    // Check git-filter-repo
    run_cmd("which", &["git-filter-repo"])
        .map_err(|_| "git-filter-repo is not installed. Install with: pip install git-filter-repo")?;

    // Check gh CLI
    run_cmd("which", &["gh"])
        .map_err(|_| "GitHub CLI (gh) is not installed. Install with: brew install gh")?;

    // Check gh auth
    run_cmd("gh", &["auth", "status"])
        .map_err(|_| "GitHub CLI is not authenticated. Run: gh auth login")?;

    Ok(())
}

fn list_components() {
    println!();
    log_info("Components to be split:");
    println!();
    println!("{:<5} {:<45} {:<35}", "IDX", "PATH", "NEW REPO NAME");
    println!("{:<5} {:<45} {:<35}", "---", "----", "-------------");

    for (i, component) in COMPONENTS.iter().enumerate() {
        println!(
            "{:<5} {:<45} {:<35}",
            i, component.path, component.repo_name
        );
    }
    println!();
}

fn clone_fresh(work_dir: &PathBuf, repo_name: &str) -> Result<PathBuf, String> {
    log_info(&format!("Cloning fresh copy for {}...", repo_name));

    let repo_dir = work_dir.join(repo_name);

    // Remove if exists
    if repo_dir.exists() {
        log_warning(&format!(
            "Directory {} already exists, removing...",
            repo_dir.display()
        ));
        std::fs::remove_dir_all(&repo_dir)
            .map_err(|e| format!("Failed to remove directory: {}", e))?;
    }

    run_cmd_in_dir(work_dir, "git", &["clone", SOURCE_REPO, repo_name])?;

    Ok(repo_dir)
}

fn filter_to_path(repo_dir: &PathBuf, path: &str) -> Result<(), String> {
    log_info(&format!("Filtering repository to path: {}", path));

    run_cmd_in_dir(
        repo_dir,
        "git",
        &["filter-repo", "--subdirectory-filter", path, "--force"],
    )?;

    Ok(())
}

fn get_commit_count(repo_dir: &PathBuf) -> Result<usize, String> {
    let output = run_cmd_in_dir(repo_dir, "git", &["rev-list", "--count", "HEAD"])?;
    output
        .trim()
        .parse()
        .map_err(|e| format!("Failed to parse commit count: {}", e))
}

fn check_repo_exists(repo_name: &str) -> bool {
    let full_name = format!("{}/{}", GITHUB_ORG, repo_name);
    run_cmd("gh", &["repo", "view", &full_name]).is_ok()
}

fn create_github_repo(repo_dir: &PathBuf, repo_name: &str, description: &str, dry_run: bool) -> Result<(), String> {
    if dry_run {
        log_warning(&format!(
            "[DRY RUN] Would create repo: {}/{}",
            GITHUB_ORG, repo_name
        ));
        return Ok(());
    }

    log_info(&format!(
        "Creating GitHub repository: {}/{}",
        GITHUB_ORG, repo_name
    ));

    // Check if repo already exists
    if check_repo_exists(repo_name) {
        log_warning(&format!(
            "Repository {}/{} already exists",
            GITHUB_ORG, repo_name
        ));
        return Ok(());
    }

    run_cmd_in_dir(
        repo_dir,
        "gh",
        &[
            "repo",
            "create",
            &format!("{}/{}", GITHUB_ORG, repo_name),
            "--public",
            "--description",
            description,
            "--source",
            ".",
            "--remote",
            "origin",
            "--push",
        ],
    )?;

    Ok(())
}

fn push_to_repo(repo_dir: &PathBuf, repo_name: &str, dry_run: bool) -> Result<(), String> {
    if dry_run {
        log_warning(&format!(
            "[DRY RUN] Would push to: {}/{}",
            GITHUB_ORG, repo_name
        ));
        return Ok(());
    }

    log_info(&format!("Pushing to {}/{}...", GITHUB_ORG, repo_name));

    // Remove old origin if exists
    let _ = run_cmd_in_dir(repo_dir, "git", &["remote", "remove", "origin"]);

    // Add new origin
    let remote_url = format!("git@github.com:{}/{}.git", GITHUB_ORG, repo_name);
    run_cmd_in_dir(repo_dir, "git", &["remote", "add", "origin", &remote_url])?;

    // Push all branches
    run_cmd_in_dir(repo_dir, "git", &["push", "-u", "origin", "--all"])?;

    // Push tags
    run_cmd_in_dir(repo_dir, "git", &["push", "origin", "--tags"])?;

    Ok(())
}

fn process_component(work_dir: &PathBuf, component: &Component, dry_run: bool) -> Result<(), String> {
    println!();
    log_info("==========================================");
    log_info(&format!("Processing: {}", component.repo_name));
    log_info(&format!("Source path: {}", component.path));
    log_info("==========================================");

    // Clone fresh
    let repo_dir = clone_fresh(work_dir, component.repo_name)?;

    // Filter to path
    filter_to_path(&repo_dir, component.path)?;

    // Show stats
    let commit_count = get_commit_count(&repo_dir)?;
    log_info(&format!("Filtered repository has {} commits", commit_count));

    if commit_count == 0 {
        return Err("No commits found after filtering. Path may not exist in history.".to_string());
    }

    // Create GitHub repo (which also pushes via --push flag)
    create_github_repo(&repo_dir, component.repo_name, component.description, dry_run)?;

    // If repo already existed, we need to push manually
    if check_repo_exists(component.repo_name) && !dry_run {
        push_to_repo(&repo_dir, component.repo_name, dry_run)?;
    }

    log_success(&format!("Completed: {}", component.repo_name));
    Ok(())
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    println!();
    println!("==========================================");
    println!("  Macroforge Repository Splitter");
    println!("==========================================");
    println!();

    // Check prerequisites
    if let Err(e) = check_prerequisites() {
        log_error(&e);
        return ExitCode::FAILURE;
    }

    let dry_run = cli.dry_run;
    if dry_run {
        log_warning("Running in DRY RUN mode - no repos will be created");
    }

    match cli.command {
        Some(Commands::List) | None => {
            list_components();

            if cli.command.is_none() {
                println!("Run with 'all' to process all components, or 'one <index>' for a specific one.");
                println!("Use --dry-run to test without making changes.");
            }
            return ExitCode::SUCCESS;
        }
        Some(Commands::One { index }) => {
            if index >= COMPONENTS.len() {
                log_error(&format!("Invalid component index: {}. Max is {}", index, COMPONENTS.len() - 1));
                return ExitCode::FAILURE;
            }

            let component = &COMPONENTS[index];
            println!("Will process: {} -> {}", component.path, component.repo_name);

            if !dry_run && !Confirm::new()
                .with_prompt("Continue?")
                .default(false)
                .interact()
                .unwrap_or(false)
            {
                log_info("Aborted.");
                return ExitCode::SUCCESS;
            }

            let work_dir = dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("macroforge-split-work");
            std::fs::create_dir_all(&work_dir).expect("Failed to create work directory");

            if let Err(e) = process_component(&work_dir, component, dry_run) {
                log_error(&e);
                return ExitCode::FAILURE;
            }
        }
        Some(Commands::All) => {
            list_components();

            if !dry_run && !Confirm::new()
                .with_prompt("Continue with splitting all components?")
                .default(false)
                .interact()
                .unwrap_or(false)
            {
                log_info("Aborted.");
                return ExitCode::SUCCESS;
            }

            let work_dir = dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("macroforge-split-work");
            std::fs::create_dir_all(&work_dir).expect("Failed to create work directory");

            let mut failed = Vec::new();

            for component in COMPONENTS {
                if let Err(e) = process_component(&work_dir, component, dry_run) {
                    log_error(&format!("Failed to process {}: {}", component.repo_name, e));
                    failed.push(component.repo_name);

                    if !Confirm::new()
                        .with_prompt("Continue with next component?")
                        .default(true)
                        .interact()
                        .unwrap_or(false)
                    {
                        break;
                    }
                }
            }

            println!();
            log_success("==========================================");
            if failed.is_empty() {
                log_success("All components processed successfully!");
            } else {
                log_warning(&format!("Completed with {} failures: {:?}", failed.len(), failed));
            }
            log_success("==========================================");
            log_info(&format!("Split repositories are in: {}", work_dir.display()));
        }
    }

    ExitCode::SUCCESS
}
