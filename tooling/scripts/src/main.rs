//! Macroforge tooling: build, verify, document and release the monorepo.

use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;

mod cli;
mod core;
mod diagnostics;
mod parsers;
mod utils;

use cli::args::{Cli, Commands, DocsCommands};

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run_cli(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            utils::format::error(&format!("{error:#}"));
            ExitCode::FAILURE
        }
    }
}

fn run_cli(cli: Cli) -> Result<()> {
    match cli.command {
        Some(Commands::Verify(args)) => cli::commands::verify::run(args),
        Some(Commands::Bump(args)) => cli::commands::bump::run(args),
        Some(Commands::Diagnostics(args)) => cli::commands::diagnostics::run(args),
        Some(Commands::Docs(args)) => {
            let config = crate::core::config::Config::load()?;
            let root = &config.root;
            match args.command {
                DocsCommands::ExtractRust => {
                    cli::commands::docs::extract_rust::generate(root)?.write(root)?;
                    Ok(())
                }
                DocsCommands::ExtractTs => {
                    cli::commands::docs::extract_ts::generate(root)?.write(root)?;
                    Ok(())
                }
                DocsCommands::GenerateReadmes => {
                    cli::commands::docs::generate_readmes::generate(root)?.write(root)?;
                    Ok(())
                }
                DocsCommands::ExtractMcp => {
                    cli::commands::docs::extract_mcp::generate(root)?.write(root)?;
                    Ok(())
                }
                DocsCommands::CheckFreshness => cli::commands::docs::check_freshness::run(),
                DocsCommands::All => {
                    utils::format::header("Generating all documentation");
                    cli::commands::docs::extract_api_docs(root)?;
                    cli::commands::docs::extract_derived_docs(root)?;
                    utils::format::success("All documentation generated");
                    Ok(())
                }
            }
        }
        Some(Commands::Test(args)) => cli::commands::test::run(args),
        Some(Commands::PublishLocal(args)) => cli::commands::publish_local::run(&args),
        Some(Commands::Tag(args)) => cli::commands::tag::run(&args),
        None => {
            // No command: show help
            use clap::CommandFactory;
            Cli::command().print_help()?;
            Ok(())
        }
    }
}
