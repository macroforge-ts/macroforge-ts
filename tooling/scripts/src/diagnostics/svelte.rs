//! Svelte diagnostic runner

use super::{DiagnosticLevel, DiagnosticTool, UnifiedDiagnostic};
use anyhow::{Context, Result};
use regex::Regex;
use std::path::Path;
use std::process::Command;

/// Run svelte-check and collect diagnostics
pub fn run(_root: &Path, project_dirs: &[&Path]) -> Result<Vec<UnifiedDiagnostic>> {
    let mut diagnostics = Vec::new();

    // Regex to match svelte-check error format: /path/file.svelte:line:col - error/warning message
    let error_regex = Regex::new(r"^(.+?):(\d+):(\d+)\s+-?\s*(error|warning|Error|Warning)\s+(.+)$")
        .context("Failed to compile error regex")?;

    for project_dir in project_dirs {
        let output = Command::new("npx")
            .args(["svelte-check", "--tsconfig", "./tsconfig.json"])
            .current_dir(project_dir)
            .output()
            .context("Failed to run svelte-check")?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines() {
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
                let level_str = caps.get(4).map(|m| m.as_str()).unwrap_or("error");
                let message = caps.get(5).map(|m| m.as_str()).unwrap_or("").to_string();

                let level = if level_str.to_lowercase() == "warning" {
                    DiagnosticLevel::Warning
                } else {
                    DiagnosticLevel::Error
                };

                diagnostics.push(UnifiedDiagnostic {
                    file,
                    line: line_num,
                    column,
                    code: "svelte".to_string(),
                    message: message.clone(),
                    raw: line.to_string(),
                    tool: DiagnosticTool::SvelteCheck,
                    level,
                });
            }
        }
    }

    Ok(diagnostics)
}

/// Find all directories with svelte.config.js
pub fn find_svelte_projects(root: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut projects = Vec::new();

    for entry in walkdir::WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            // Skip node_modules, hidden dirs, test directories
            !name.starts_with('.')
                && name != "node_modules"
                && name != "dist"
                && name != "test"
                && name != "tests"
                && name != "fixtures"
        })
    {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy();
        if (name == "svelte.config.js" || name == "svelte.config.ts")
            && let Some(parent) = entry.path().parent()
        {
            projects.push(parent.to_path_buf());
        }
    }

    Ok(projects)
}
