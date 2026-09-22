//! Verify a release
//!
//! Builds every package, gates on repo-wide diagnostics, runs the tests and
//! regenerates the documentation. It changes no versions; `mf bump` does.

use crate::cli::VerifyArgs;
use crate::cli::commands::docs::extract_api_docs;
use crate::core::config::Config;
use crate::core::deps;
use crate::core::repos::{Repo, RepoType};
use crate::core::shell;
use crate::diagnostics::runner::{DiagnosticOptions, DiagnosticsRunner, Formatting};
use anyhow::{Context, Result};
use colored::Colorize;
use std::io::{self, Write};

/// The number of numbered steps a run prints.
const STEP_COUNT: usize = 8;

fn step(number: usize, label: &str) {
    println!(
        "\n{} {}",
        format!("[{number}/{STEP_COUNT}]").bold(),
        label.bold()
    );
}

fn skipped(number: usize, what: &str) {
    println!(
        "\n{} Skipping {what}",
        format!("[{number}/{STEP_COUNT}]").dimmed()
    );
}

/// Build a single repository. Dependencies are installed once for the whole
/// workspace beforehand, and repos build in dependency order, which the npm
/// packages rely on: each links the built outputs of the ones it depends on.
fn build_repo(repo: &Repo, verbose: bool) -> Result<()> {
    match repo.repo_type {
        RepoType::Rust if repo.name == "core" => {
            // NAPI_BUILD_SKIP_WATCHER stops build.rs from spawning another napi build.
            shell::run(
                "NAPI_BUILD_SKIP_WATCHER=1 deno task build",
                &repo.abs_path,
                verbose,
            )?;
        }
        RepoType::Ts => {
            if repo.has_script("build")? {
                shell::deno::task(&repo.abs_path, "build")?;
            }
        }
        RepoType::Website => {
            shell::deno::task(&repo.abs_path, "build")?;
        }
        RepoType::Rust | RepoType::Tooling | RepoType::Extension => {}
    }
    Ok(())
}

/// Every diagnostic tool, with formatting checked rather than applied: a gate
/// that rewrites files can pass on a tree nobody committed.
fn run_diagnostics(config: &Config) -> Result<()> {
    let options = DiagnosticOptions {
        formatting: Formatting::Check,
        ..DiagnosticOptions::all()
    };
    let aggregator = DiagnosticsRunner::new(&config.root, options).run()?;
    let diagnostics = aggregator.diagnostics();
    if diagnostics.is_empty() {
        eprintln!("  {} All diagnostics passed", "✓".green());
        return Ok(());
    }
    eprintln!(
        "\n{} Found {} diagnostic issues:",
        "✗".red(),
        diagnostics.len()
    );
    for diag in diagnostics {
        eprintln!(
            "  {}:{}:{}: {}",
            diag.file, diag.line, diag.column, diag.message
        );
    }
    anyhow::bail!("Diagnostics failed")
}

/// The editor extensions build for wasm32-wasip1, which the workspace clippy
/// pass (host target) does not compile, so they get a target pass of their own.
fn check_extensions(config: &Config) -> Result<()> {
    let extensions_path = config.root.join("crates/extensions");
    for extension in ["svelte_macroforge", "vtsls_macroforge"] {
        let extension_path = extensions_path.join(extension);
        print!("  {} {extension} build... ", "→".blue());
        io::stdout().flush()?;
        shell::cargo::build_target(&extension_path, "wasm32-wasip1")
            .with_context(|| format!("{extension} failed to build for wasm32-wasip1"))?;
        println!("{}", "ok".green());

        print!("  {} {extension} clippy... ", "→".blue());
        io::stdout().flush()?;
        shell::cargo::clippy_target(&extension_path, "wasm32-wasip1")
            .with_context(|| format!("clippy failed for {extension} on wasm32-wasip1"))?;
        println!("{}", "ok".green());
    }
    Ok(())
}

/// Entry point for `mf verify`: builds, checks and tests the whole workspace.
pub fn run(args: VerifyArgs) -> Result<()> {
    let config = Config::load()?;
    let verbose = std::env::var("VERBOSE").is_ok() || std::env::var("DEBUG").is_ok();

    let repos: Vec<&Repo> = deps::topo_order(&config.deps)?
        .iter()
        .filter_map(|name| config.repos.get(name))
        .collect();

    println!("\n{}", "=".repeat(60));
    println!("{}", "Verify".bold());
    println!(
        "Repos: {}",
        repos
            .iter()
            .map(|r| r.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
            .cyan()
    );
    println!("{}", "=".repeat(60));

    if args.skip_docs {
        skipped(1, "API extraction");
    } else {
        step(1, "Extracting API documentation");
        extract_api_docs(&config.root)?;
    }

    if args.skip_build {
        skipped(2, "dependency install");
        skipped(3, "build");
    } else {
        step(2, "Installing dependencies");
        shell::deno::install(&config.root).context("deno install failed")?;

        step(3, "Building packages");
        for repo in &repos {
            print!("  {} {}... ", "Building:".bold(), repo.name.cyan());
            io::stdout().flush()?;
            build_repo(repo, verbose).with_context(|| format!("Build failed for {}", repo.name))?;
            println!("{}", "done".green());
        }
        // The playground is type-checked next, against the packages it links.
        super::test::install_playground_apps(&config)?;
    }

    // After the build: the type-checks expand through the engine it just
    // produced, not whatever an earlier build left behind.
    step(4, "Running diagnostics");
    run_diagnostics(&config)?;

    if args.skip_build {
        skipped(5, "JSR publish check");
        skipped(6, "tests");
        skipped(7, "extension checks");
    } else {
        step(5, "Checking the JSR publish");
        shell::deno::publish_check(&config.root).context("deno publish --dry-run failed")?;

        step(6, "Running tests");
        super::test::run_rust_tests(&config)?;
        super::test::run_package_tests(&config)?;
        super::test::run_playground_suites(&config)?;

        step(7, "Checking the extensions (wasm32-wasip1)");
        check_extensions(&config)?;
    }

    if args.skip_docs {
        skipped(8, "MCP docs");
    } else {
        step(8, "Syncing MCP server docs");
        let mcp_path = config.root.join("packages/mcp-server");
        shell::deno::task(&mcp_path, "build:docs").context("Failed to sync the MCP server docs")?;
        // The sync writes raw markdown; the formatted tree is canonical.
        shell::deno::deno_fmt(&config.root, &["packages/mcp-server/docs"])
            .context("Failed to format the MCP server docs")?;
    }

    println!("\n{} {}", "✓".green(), "Verified".bold());
    println!("\n{} {}", "Next step:".bold(), "pixi run bump".cyan());
    Ok(())
}
