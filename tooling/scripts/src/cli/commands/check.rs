//! Single-file diagnostics command
//!
//! Gets TypeScript diagnostics for a specific file.

use crate::cli::CheckArgs;
use crate::core::config::Config;
use crate::utils::format;
use anyhow::{Context, Result};
use colored::Colorize;
use std::path::Path;
use std::process::Command;

pub fn run(args: CheckArgs) -> Result<()> {
    let config = Config::load()?;

    // Resolve file path
    let file_path = if args.file.is_absolute() {
        args.file.clone()
    } else {
        config.root.join(&args.file)
    };

    if !file_path.exists() {
        anyhow::bail!("File not found: {}", file_path.display());
    }

    // Find tsconfig.json
    let tsconfig = find_tsconfig(&file_path)?;

    format::header("TypeScript Diagnostics");
    println!("File: {}", file_path.display());
    println!("Using tsconfig: {}", tsconfig.display());
    println!();

    // Run TypeScript check using macroforge tsc
    let output = Command::new("npx")
        .args(["macroforge", "tsc", "-p", &tsconfig.to_string_lossy()])
        .current_dir(&config.root)
        .output()
        .context("Failed to run macroforge tsc")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Parse errors for the specific file
    let file_str = file_path.to_string_lossy();
    let mut semantic_errors: Vec<(u32, u32, String, String)> = Vec::new();
    let syntactic_errors: Vec<(u32, u32, String, String)> = Vec::new();

    // Match errors for this file
    let error_regex = regex::Regex::new(r"^(.+?)\((\d+),(\d+)\): error (TS\d+): (.+)$")?;

    for line in stdout.lines().chain(stderr.lines()) {
        if let Some(caps) = error_regex.captures(line) {
            let err_file = caps.get(1).map(|m| m.as_str()).unwrap_or("");

            // Check if error is for our file
            if err_file.contains(&*file_str) || file_str.contains(err_file) {
                let line_num = caps.get(2).and_then(|m| m.as_str().parse::<u32>().ok()).unwrap_or(1);
                let col = caps.get(3).and_then(|m| m.as_str().parse::<u32>().ok()).unwrap_or(1);
                let code = caps.get(4).map(|m| m.as_str()).unwrap_or("TS0000");
                let message = caps.get(5).map(|m| m.as_str()).unwrap_or("");

                semantic_errors.push((line_num, col, code.to_string(), message.to_string()));
            }
        }
    }

    // Display results
    println!("{}", "=== Semantic Diagnostics ===".bold());
    if semantic_errors.is_empty() {
        println!("{}", "No semantic errors found!".green());
    } else {
        for (line, col, code, message) in &semantic_errors {
            println!(
                "{}({}:{}): {} {}: {}",
                file_path.display(),
                line,
                col,
                "error".red(),
                code.cyan(),
                message
            );
        }
    }

    println!();
    println!("{}", "=== Syntactic Diagnostics ===".bold());
    if syntactic_errors.is_empty() {
        println!("{}", "No syntactic errors found!".green());
    } else {
        for (line, col, code, message) in &syntactic_errors {
            println!(
                "{}({}:{}): {} {}: {}",
                file_path.display(),
                line,
                col,
                "error".red(),
                code.cyan(),
                message
            );
        }
    }

    // Summary
    println!();
    println!("{}", "=== Summary ===".bold());
    println!("Semantic errors: {}", semantic_errors.len());
    println!("Syntactic errors: {}", syntactic_errors.len());

    Ok(())
}

/// Find tsconfig.json starting from a file's directory
fn find_tsconfig(start_file: &Path) -> Result<std::path::PathBuf> {
    let start_dir = start_file.parent().context("File has no parent directory")?;

    let mut dir = start_dir;
    loop {
        let config_path = dir.join("tsconfig.json");
        if config_path.exists() {
            return Ok(config_path);
        }

        match dir.parent() {
            Some(parent) if parent != dir => dir = parent,
            _ => break,
        }
    }

    anyhow::bail!("Could not find tsconfig.json for {}", start_file.display())
}
