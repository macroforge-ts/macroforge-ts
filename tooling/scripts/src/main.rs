//! Macroforge Tooling - unified CLI/TUI for build, release, and diagnostics

use std::io;
use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;

mod cli;
mod core;
mod diagnostics;
mod parsers;
mod tui;
mod utils;

use cli::{Cli, Commands, DocsCommands};

fn main() -> ExitCode {
    let cli = Cli::parse();

    // TUI mode
    if cli.tui || matches!(cli.command, Some(Commands::Tui)) {
        return run_tui();
    }

    // CLI mode
    match run_cli(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            utils::format::error(&format!("{:#}", e));
            ExitCode::FAILURE
        }
    }
}

fn run_cli(cli: Cli) -> Result<()> {
    match cli.command {
        Some(Commands::Tui) => {
            // Already handled above
            unreachable!()
        }
        Some(Commands::Prep(args)) => cli::commands::prep::run(args),
        Some(Commands::Commit(args)) => cli::commands::commit::run(args),
        Some(Commands::Manifest(args)) => cli::commands::manifests::run(args),
        Some(Commands::Versions(args)) => cli::commands::versions::run(args),
        Some(Commands::Diagnostics(args)) => cli::commands::diagnostics::run(args),
        Some(Commands::Build(args)) => cli::commands::build::run(args),
        Some(Commands::Docs(args)) => {
            match args.command {
                DocsCommands::ExtractRust { output_dir } => {
                    cli::commands::docs::extract_rust::run(&output_dir)
                }
                DocsCommands::ExtractTs { output_dir } => {
                    cli::commands::docs::extract_ts::run(&output_dir)
                }
                DocsCommands::BuildBook { output_path } => {
                    cli::commands::docs::build_book::run(&output_path)
                }
                DocsCommands::GenerateReadmes => cli::commands::docs::generate_readmes::run(),
                DocsCommands::CheckFreshness => cli::commands::docs::check_freshness::run(),
                DocsCommands::All => {
                    utils::format::header("Generating all documentation");
                    // Run all doc generation steps
                    let root = std::env::current_dir()?;
                    cli::commands::docs::extract_rust::run(
                        &root.join("website/static/api-data/rust"),
                    )?;
                    cli::commands::docs::extract_ts::run(
                        &root.join("website/static/api-data/typescript"),
                    )?;
                    cli::commands::docs::generate_readmes::run()?;
                    utils::format::success("All documentation generated");
                    Ok(())
                }
            }
        }
        Some(Commands::Expand(args)) => cli::commands::expand::run(args),
        Some(Commands::Check(args)) => cli::commands::check::run(args),
        Some(Commands::Test(args)) => cli::commands::test::run(args),
        None => {
            // No command: show help
            use clap::CommandFactory;
            Cli::command().print_help()?;
            Ok(())
        }
    }
}

fn run_tui() -> ExitCode {
    match run_tui_inner() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("TUI error: {:#}", e);
            ExitCode::FAILURE
        }
    }
}

fn run_tui_inner() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = tui::App::new();
    app.init_default_tasks();
    app.log(
        tui::app::LogLevel::Info,
        "Welcome to Macroforge Tooling Dashboard",
    );
    app.log(
        tui::app::LogLevel::Info,
        "Press 'q' to quit, j/k to navigate, Enter to run task",
    );

    // Event handler
    let events = tui::event::EventHandler::new(250);

    // Main loop
    loop {
        // Process any pending task messages
        app.process_messages();

        // Render
        terminal.draw(|frame| tui::ui::render(frame, &app))?;

        // Handle events
        match events.next()? {
            tui::event::AppEvent::Key(key) => {
                tui::event::handle_key(&mut app, key);
            }
            tui::event::AppEvent::Tick => {}
            tui::event::AppEvent::Resize => {}
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
