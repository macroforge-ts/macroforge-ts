//! Test runner command
//!
//! Runs tests for Rust crates, TypeScript packages, and playground.

use crate::cli::{ExpandArgs, TestArgs};
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
        "rust" => run_rust_tests(&config, args.repos.as_deref())?,
        "packages" => run_package_tests(&config)?,
        "playground" => run_playground_tests(&config)?,
        "all" => {
            run_rust_tests(&config, args.repos.as_deref())?;
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

/// Run Rust tests using cargo test with workspace support
fn run_rust_tests(config: &Config, repos_filter: Option<&str>) -> Result<()> {
    println!("\n{}", "Running Rust tests".bold());
    println!("{}", "─".repeat(40));

    // Get Rust repos
    let rust_repos: Vec<_> = config
        .repos
        .values()
        .filter(|r| r.repo_type == RepoType::Rust)
        .collect();

    // Filter repos if specified
    let repos_to_test: Vec<_> = if let Some(filter) = repos_filter {
        let filter_names: Vec<&str> = filter.split(',').map(|s| s.trim()).collect();
        rust_repos
            .into_iter()
            .filter(|r| filter_names.contains(&r.name.as_str()))
            .collect()
    } else {
        rust_repos
    };

    if repos_to_test.is_empty() {
        println!("  {} No Rust repos to test", "⚠".yellow());
        return Ok(());
    }

    for repo in &repos_to_test {
        print!("  {} {}... ", "Testing:".bold(), repo.name.cyan());
        io::stdout().flush()?;

        match shell::cargo::test(&repo.abs_path) {
            Ok(_) => println!("{}", "passed".green()),
            Err(e) => {
                println!("{}", "failed".red());
                return Err(e).context(format!("Tests failed for {}", repo.name));
            }
        }
    }

    Ok(())
}

/// Run TypeScript package tests using deno
fn run_package_tests(config: &Config) -> Result<()> {
    println!("\n{}", "Running package tests".bold());
    println!("{}", "─".repeat(40));

    // Get TS packages that have test scripts
    let ts_repos_with_tests: Vec<_> = config
        .repos
        .values()
        .filter(|r| r.repo_type == RepoType::Ts)
        .filter(|r| {
            r.package_json
                .as_ref()
                .filter(|p| p.exists())
                .and_then(|p| std::fs::read_to_string(p).ok())
                .and_then(|content| serde_json::from_str::<serde_json::Value>(&content).ok())
                .and_then(|pkg| pkg.get("scripts")?.get("test").cloned())
                .is_some()
        })
        .collect();

    if ts_repos_with_tests.is_empty() {
        println!("  {} No TypeScript packages with tests", "⚠".yellow());
        return Ok(());
    }

    for repo in &ts_repos_with_tests {
        print!("  {} {}... ", "Testing:".bold(), repo.name.cyan());
        io::stdout().flush()?;

        match shell::deno::task(&repo.abs_path, "test") {
            Ok(_) => println!("{}", "passed".green()),
            Err(e) => {
                println!("{}", "failed".red());
                return Err(e).context(format!("Tests failed for {}", repo.name));
            }
        }
    }

    Ok(())
}

/// Run playground tests (expand + deno tests + rust tests + e2e)
fn run_playground_tests(config: &Config) -> Result<()> {
    println!("\n{}", "Running playground tests".bold());
    println!("{}", "─".repeat(40));

    let playground_tests = config.root.join("tooling/playground/tests");
    let rust_tests = playground_tests.join("rust-tests");

    // First expand macros (call directly via our expand module)
    print!("  {} Expanding macros... ", "→".blue());
    io::stdout().flush()?;
    match super::expand::run(ExpandArgs {
        use_cli: false,
        path: None,
    }) {
        Ok(_) => println!("{}", "done".green()),
        Err(e) => {
            println!("{}", "failed".red());
            return Err(e).context("Failed to expand macros");
        }
    }

    // Run deno tests (replaces node --test)
    print!("  {} Deno tests... ", "→".blue());
    io::stdout().flush()?;
    match shell::deno::task(&playground_tests, "test") {
        Ok(_) => println!("{}", "passed".green()),
        Err(e) => {
            println!("{}", "failed".red());
            return Err(e).context("Deno tests failed");
        }
    }

    // Run validator tests
    print!("  {} Validator tests... ", "→".blue());
    io::stdout().flush()?;
    match shell::deno::task(&playground_tests, "test:validators") {
        Ok(_) => println!("{}", "passed".green()),
        Err(e) => {
            println!("{}", "failed".red());
            return Err(e).context("Validator tests failed");
        }
    }

    // Run Rust playground tests
    print!("  {} Rust tests... ", "→".blue());
    io::stdout().flush()?;
    match shell::cargo::test(&rust_tests) {
        Ok(_) => println!("{}", "passed".green()),
        Err(e) => {
            println!("{}", "failed".red());
            return Err(e).context("Rust playground tests failed");
        }
    }

    // Run e2e tests
    print!("  {} E2E tests... ", "→".blue());
    io::stdout().flush()?;
    match shell::deno::task(&playground_tests, "test:e2e") {
        Ok(_) => println!("{}", "passed".green()),
        Err(e) => {
            println!("{}", "failed".red());
            return Err(e).context("E2E tests failed");
        }
    }

    Ok(())
}
