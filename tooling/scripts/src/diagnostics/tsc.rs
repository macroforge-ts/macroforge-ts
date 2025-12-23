//! TypeScript diagnostic runner

use super::{DiagnosticLevel, DiagnosticTool, UnifiedDiagnostic};
use anyhow::{Context, Result};
use regex::Regex;
use std::path::Path;
use std::process::Command;

/// Run TypeScript type checking and collect diagnostics
pub fn run(root: &Path, tsconfig_paths: &[&Path]) -> Result<Vec<UnifiedDiagnostic>> {
    let mut diagnostics = Vec::new();

    // Regex to match TypeScript error format: file(line,col): error TSxxxx: message
    let error_regex = Regex::new(r"^(.+?)\((\d+),(\d+)\): error (TS\d+): (.+)$")
        .context("Failed to compile error regex")?;

    for tsconfig in tsconfig_paths {
        let output = Command::new("npx")
            .args(["macroforge", "tsc", "-p", &tsconfig.to_string_lossy()])
            .current_dir(root)
            .output()
            .context("Failed to run macroforge tsc")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Parse both stdout and stderr
        for line in stdout.lines().chain(stderr.lines()) {
            if let Some(caps) = error_regex.captures(line) {
                let file = caps.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
                let line_num: u32 = caps
                    .get(2)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or(1);
                let column: u32 = caps
                    .get(3)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or(1);
                let code = caps.get(4).map(|m| m.as_str()).unwrap_or("TS0000").to_string();
                let message = caps.get(5).map(|m| m.as_str()).unwrap_or("").to_string();

                diagnostics.push(UnifiedDiagnostic {
                    file,
                    line: line_num,
                    column,
                    code,
                    message: message.clone(),
                    raw: line.to_string(),
                    tool: DiagnosticTool::Tsc,
                    level: DiagnosticLevel::Error,
                });
            }
        }
    }

    Ok(diagnostics)
}

/// Find all tsconfig.json files in the project
pub fn find_tsconfigs(root: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut tsconfigs = Vec::new();

    for entry in walkdir::WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            // Skip node_modules and hidden directories
            !name.starts_with('.') && name != "node_modules" && name != "dist"
        })
    {
        let entry = entry?;
        if entry.file_name() == "tsconfig.json" {
            tsconfigs.push(entry.path().to_path_buf());
        }
    }

    Ok(tsconfigs)
}
