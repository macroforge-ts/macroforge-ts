//! Multi-tool diagnostics command
//!
//! Runs comprehensive diagnostics across the codebase using multiple tools.

use crate::cli::DiagnosticsArgs;
use crate::core::config::Config;
use crate::diagnostics::runner::DiagnosticOptions;
use crate::diagnostics::{DiagnosticLevel, DiagnosticTool, DiagnosticsRunner};
use crate::utils::format;
use anyhow::Result;
use colored::Colorize;
use std::fs;

pub fn run(args: DiagnosticsArgs) -> Result<()> {
    let config = Config::load()?;
    let logs_dir = config.root.join(".tmp/diagnostics");

    // Create logs directory if --log is specified
    if args.log {
        let _ = fs::remove_dir_all(&logs_dir);
        fs::create_dir_all(&logs_dir)?;
        println!("Diagnostics logs will be saved in: {}", logs_dir.display());
    }

    // Parse tool options
    let mut options = if let Some(tools) = &args.tools {
        DiagnosticOptions::parse(tools)
    } else {
        DiagnosticOptions::all()
    };
    options.format = args.format;

    // Run diagnostics
    let runner = DiagnosticsRunner::new(&config.root, options);
    let aggregator = runner.run()?;

    // JSON output mode
    if args.json {
        let json = serde_json::to_string_pretty(aggregator.diagnostics())?;
        println!("{}", json);
        return Ok(());
    }

    // Summary
    println!("\n{}", "═".repeat(60));
    println!("{}", "Diagnostics Summary".bold());
    println!("{}", "═".repeat(60));

    let by_code = aggregator.by_code();
    let mut codes: Vec<_> = by_code.keys().collect();
    codes.sort();

    for code in &codes {
        let diags = &by_code[*code];
        let count = diags.len();

        if args.log {
            // Write to log file
            let log_file =
                logs_dir.join(format!("{}.log", code.replace("::", "_").replace("/", "_")));
            let content: Vec<String> = diags
                .iter()
                .map(|d| format!("{}:{}:{}: {}", d.file, d.line, d.column, d.raw))
                .collect();
            fs::write(&log_file, content.join("\n") + "\n")?;
            println!(
                "  {}: {} errors -> {}",
                code.cyan(),
                count,
                log_file.display()
            );
        } else {
            println!("  {}: {} errors", code.cyan(), count);
        }
    }

    // Print all diagnostics if not logging
    if !args.log && !aggregator.diagnostics().is_empty() {
        println!("\n{}", "═".repeat(60));
        println!("{}", "All Diagnostics".bold());
        println!("{}", "═".repeat(60));

        for diag in aggregator.diagnostics() {
            let level_str = match diag.level {
                DiagnosticLevel::Error => "error".red(),
                DiagnosticLevel::Warning => "warning".yellow(),
                DiagnosticLevel::Info => "info".blue(),
            };

            let tool_str = match diag.tool {
                DiagnosticTool::DenoLint => "deno-lint",
                DiagnosticTool::Clippy => "clippy",
                DiagnosticTool::Tsc => "tsc",
                DiagnosticTool::TsPlugin => "ts-plugin",
                DiagnosticTool::SvelteCheck => "svelte",
                DiagnosticTool::Macroforge => "macroforge",
            };

            println!(
                "\n{}[{}]: {} ({})",
                level_str,
                diag.code.cyan(),
                diag.message,
                tool_str.dimmed()
            );
            println!(
                "  {} {}:{}:{}",
                "-->".blue(),
                diag.file,
                diag.line,
                diag.column
            );
        }
    }

    let error_count = aggregator.error_count();
    let warning_count = aggregator.warning_count();
    let total = error_count + warning_count;

    println!();
    if total > 0 {
        format::error(&format!(
            "Diagnostics completed with {} errors, {} warnings",
            error_count, warning_count
        ));
        std::process::exit(1);
    } else {
        format::success("Diagnostics completed with no errors or warnings");
    }

    Ok(())
}
