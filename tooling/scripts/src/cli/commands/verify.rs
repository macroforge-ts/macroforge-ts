//! Verify a release
//!
//! Builds every package, gates on repo-wide diagnostics, runs the tests and
//! regenerates the documentation. It changes no versions; `mf bump` does.

use crate::cli::args::VerifyArgs;
use crate::cli::commands::docs::{
    check_freshness, extract_api_docs, extract_mcp, generate_readmes,
};
use crate::core::config::Config;
use crate::core::deps;
use crate::core::repos::{Repo, RepoType};
use crate::core::shell;
use crate::diagnostics::runner::{DiagnosticOptions, DiagnosticsRunner, Fixes};
use anyhow::{Context, Result};
use colored::Colorize;
use std::io::{self, Write};

/// A step of the run. The flags decide which steps run, and only those are
/// numbered and printed.
enum Step {
    ExtractApiDocs,
    InstallDependencies,
    BuildPackages,
    Diagnostics,
    PublishCheck,
    Tests,
    BuildExtensions,
    CheckDocs,
    ExtractMcpDocs,
}

impl Step {
    fn label(&self) -> &'static str {
        match self {
            Step::ExtractApiDocs => "Extracting API documentation",
            Step::InstallDependencies => "Installing dependencies",
            Step::BuildPackages => "Building packages",
            Step::Diagnostics => "Running diagnostics",
            Step::PublishCheck => "Checking the JSR publish",
            Step::Tests => "Running tests",
            Step::BuildExtensions => "Building the extensions (wasm32-wasip1)",
            Step::CheckDocs => "Checking the generated documentation",
            Step::ExtractMcpDocs => "Extracting the MCP server docs",
        }
    }
}

/// The steps `args` enables, in order.
fn plan(args: &VerifyArgs) -> Vec<Step> {
    let mut steps = Vec::new();
    // With `check`, the final step compares the checked-in docs instead.
    if !args.skip_docs && !args.check {
        steps.push(Step::ExtractApiDocs);
    }
    if !args.skip_build {
        steps.push(Step::InstallDependencies);
        steps.push(Step::BuildPackages);
    }
    // After the build: the type-checks expand through the engine it just
    // produced, not whatever an earlier build left behind.
    steps.push(Step::Diagnostics);
    if !args.skip_build {
        steps.push(Step::PublishCheck);
        steps.push(Step::Tests);
        steps.push(Step::BuildExtensions);
    }
    // Both need the website the build step produced.
    if !args.skip_docs {
        steps.push(if args.check {
            Step::CheckDocs
        } else {
            Step::ExtractMcpDocs
        });
    }
    steps
}

/// Whether `repo` builds anything of its own. The other Rust crates compile as
/// `core`'s dependencies, and the extensions build in their own target pass.
fn has_build_step(repo: &Repo) -> Result<bool> {
    Ok(match repo.repo_type {
        RepoType::Rust => repo.name == "core",
        RepoType::Ts => repo.has_script("build")?,
        RepoType::Website => true,
    })
}

/// Build a single repository. Dependencies are installed once for the whole
/// workspace beforehand, and repos build in dependency order, which the npm
/// packages rely on: each links the built outputs of the ones it depends on.
fn build_repo(repo: &Repo) -> Result<()> {
    shell::deno::task(&repo.abs_path, "build")?;
    Ok(())
}

/// Every diagnostic tool. Locally the formatting and lint fixes are applied
/// first; with `check` nothing is changed, since a gate that rewrites files can
/// pass on a tree nobody committed.
fn run_diagnostics(config: &Config, check: bool) -> Result<()> {
    let options = DiagnosticOptions {
        fixes: if check { Fixes::Check } else { Fixes::Apply },
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

/// Builds the editor extensions for wasm32-wasip1, the target Zed loads them
/// as. The workspace clippy pass lints them for the host; their own source has
/// no target-specific code, so only the build needs the real target.
fn build_extensions(config: &Config) -> Result<()> {
    let extensions_path = config.root.join("crates/extensions");
    for extension in ["svelte_macroforge", "vtsls_macroforge"] {
        let extension_path = extensions_path.join(extension);
        print!("  {} {extension} build... ", "→".blue());
        io::stdout().flush()?;
        shell::cargo::build_target(&extension_path, "wasm32-wasip1")
            .with_context(|| format!("{extension} failed to build for wasm32-wasip1"))?;
        println!("{}", "ok".green());
    }
    Ok(())
}

/// Entry point for `mf verify`: builds, checks and tests the whole workspace.
pub fn run(args: VerifyArgs) -> Result<()> {
    let config = Config::load()?;

    let repos: Vec<&Repo> = deps::topo_order(&config.deps)?
        .iter()
        .filter_map(|name| config.repos.get(name))
        .collect();

    let steps = plan(&args);
    let total = steps.len();
    for (index, step) in steps.iter().enumerate() {
        println!(
            "\n{} {}",
            format!("[{}/{total}]", index + 1).bold(),
            step.label().bold()
        );
        match step {
            Step::ExtractApiDocs => {
                extract_api_docs(&config.root)?;
                generate_readmes::generate(&config.root)?.write(&config.root)?;
            }
            Step::InstallDependencies => {
                shell::deno::install(&config.root).context("deno install failed")?;
            }
            Step::BuildPackages => {
                for repo in &repos {
                    if !has_build_step(repo)? {
                        continue;
                    }
                    print!("  {} {}... ", "Building:".bold(), repo.name.cyan());
                    io::stdout().flush()?;
                    build_repo(repo).with_context(|| format!("Build failed for {}", repo.name))?;
                    println!("{}", "done".green());
                }
                // The testground macro package and the test suites run this
                // checkout's CLI, so a stale one would test older engine code.
                print!("  {} {}... ", "Building:".bold(), "cli".cyan());
                io::stdout().flush()?;
                shell::cargo::build_bin(&config.root, "macroforge_ts", "macroforge")
                    .context("Build failed for the macroforge CLI")?;
                println!("{}", "done".green());
                // The testground is type-checked next, against the packages it links.
                super::test::prepare_testground_apps(&config)?;
            }
            Step::Diagnostics => run_diagnostics(&config, args.check)?,
            Step::PublishCheck => {
                shell::deno::publish_check(&config.root)
                    .context("deno publish --dry-run failed")?;
            }
            Step::Tests => {
                super::test::run_rust_tests(&config)?;
                super::test::run_package_tests(&config)?;
                super::test::run_testground_suites(&config)?;
            }
            Step::BuildExtensions => build_extensions(&config)?,
            Step::CheckDocs => check_freshness::check(&config.root)?,
            Step::ExtractMcpDocs => {
                extract_mcp::generate(&config.root)?.write(&config.root)?;
            }
        }
    }

    println!("\n{} {}", "✓".green(), "Verified".bold());
    Ok(())
}
