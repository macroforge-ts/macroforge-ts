//! Test runner command
//!
//! Runs tests for Rust crates, TypeScript packages, and playground.

use crate::cli::TestArgs;
use crate::core::config::Config;
use crate::core::repos::RepoType;
use crate::core::shell;
use anyhow::{Context, Result};
use colored::Colorize;
use std::io::{self, Write};

/// Run tests based on the specified suite
pub fn run(args: TestArgs) -> Result<()> {
    let config = Config::load()?;

    match args.suite.as_str() {
        "rust" => run_rust_tests(&config)?,
        "packages" => run_package_tests(&config)?,
        "playground" => run_playground_tests(&config)?,
        "all" => {
            run_rust_tests(&config)?;
            run_package_tests(&config)?;
            run_playground_tests(&config)?;
        }
        other => anyhow::bail!(
            "Unknown test suite: {}. Use 'rust', 'packages', 'playground', or 'all'",
            other
        ),
    }

    println!("\n{} All tests passed!", "✓".green());
    Ok(())
}

/// Runs every Rust test in the workspace once, then each excluded crate's.
pub fn run_rust_tests(config: &Config) -> Result<()> {
    println!("\n{}", "Running Rust tests".bold());
    println!("{}", "─".repeat(40));

    let mut rust_roots = vec![config.root.clone()];
    rust_roots.extend(crate::diagnostics::clippy::excluded_crates(&config.root)?);
    for rust_root in &rust_roots {
        let label = rust_root
            .strip_prefix(&config.root)
            .ok()
            .filter(|relative| !relative.as_os_str().is_empty())
            .map_or_else(
                || "workspace".to_string(),
                |relative| relative.display().to_string(),
            );
        print!("  {} {}... ", "Testing:".bold(), label.cyan());
        io::stdout().flush()?;
        match shell::cargo::test(rust_root) {
            Ok(_) => println!("{}", "passed".green()),
            Err(e) => {
                println!("{}", "failed".red());
                return Err(e).context(format!("Tests failed for {label}"));
            }
        }
    }
    Ok(())
}

/// Runs the `test` task of every TypeScript package that defines one.
pub fn run_package_tests(config: &Config) -> Result<()> {
    println!("\n{}", "Running package tests".bold());
    println!("{}", "─".repeat(40));

    let mut tested = 0;
    for repo in config
        .repos
        .values()
        .filter(|repo| repo.repo_type == RepoType::Ts)
    {
        if !repo.has_script("test")? {
            continue;
        }
        print!("  {} {}... ", "Testing:".bold(), repo.name.cyan());
        io::stdout().flush()?;
        match shell::deno::task(&repo.abs_path, "test") {
            Ok(_) => println!("{}", "passed".green()),
            Err(e) => {
                println!("{}", "failed".red());
                return Err(e).context(format!("Tests failed for {}", repo.name));
            }
        }
        tested += 1;
    }
    if tested == 0 {
        println!("  {} No TypeScript packages with tests", "⚠".yellow());
    }
    Ok(())
}

fn run_playground_tests(config: &Config) -> Result<()> {
    println!("\n{}", "Running playground tests".bold());
    println!("{}", "─".repeat(40));
    run_playground_suites(config)
}

/// The playground projects that consume the published packages.
const PLAYGROUND_APPS: [&str; 4] = ["vanilla", "svelte", "library", "tests"];

/// Reinstalls every playground app. Each is its own project linking the built
/// `npm/` packages, and Deno copies a linked package at install time, so only
/// a fresh install makes the playground see the current build.
pub fn install_playground_apps(config: &Config) -> Result<()> {
    for app in PLAYGROUND_APPS {
        let app_dir = config.root.join("tooling/playground").join(app);
        println!("  {} installing {app}...", "→".blue());
        shell::deno::install(&app_dir).with_context(|| format!("deno install failed in {app}"))?;
    }
    Ok(())
}

/// The apps the Playwright suites drive. They are served as production builds,
/// so the run builds them here: a build inside Playwright's `webServer` would
/// have to finish inside its start-up budget while the suite waits.
const E2E_APPS: [&str; 2] = ["vanilla", "svelte"];

fn build_e2e_apps(config: &Config) -> Result<()> {
    for app in E2E_APPS {
        let app_dir = config.root.join("tooling/playground").join(app);
        println!("  {} building {app}...", "→".blue());
        shell::deno::task(&app_dir, "build")
            .with_context(|| format!("deno task build failed in {app}"))?;
    }
    Ok(())
}

/// Runs the playground's deno, validator and e2e suites with their output
/// shown./// Runs the playground's deno, validator and e2e suites with their output
/// shown. The suites drive this checkout's debug CLI (`MACROFORGE_CLI`).
pub fn run_playground_suites(config: &Config) -> Result<()> {
    install_playground_apps(config)?;
    build_e2e_apps(config)?;

    let playground_tests = config.root.join("tooling/playground/tests");
    let suites = [
        ("Deno tests", "test"),
        ("Validator tests", "test:validators"),
        ("E2E tests", "test:e2e"),
    ];
    for (label, task) in suites {
        println!("  {} {label}...", "→".blue());
        shell::deno::task_inherit(&playground_tests, task)
            .with_context(|| format!("{label} failed"))?;
        println!("  {} {label} passed", "✓".green());
    }
    Ok(())
}
