//! Deno lint diagnostic runner

use super::{DiagnosticLevel, DiagnosticTool, UnifiedDiagnostic};
use crate::core::shell;
use anyhow::Result;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct DenoLintOutput {
    diagnostics: Vec<DenoLintDiagnostic>,
    #[allow(dead_code)]
    errors: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct DenoLintDiagnostic {
    range: DenoLintRange,
    filename: String,
    message: String,
    code: String,
    #[allow(dead_code)]
    hint: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DenoLintRange {
    start: DenoLintPosition,
    #[allow(dead_code)]
    end: DenoLintPosition,
}

#[derive(Debug, Deserialize)]
struct DenoLintPosition {
    line: u32,
    col: u32,
}

/// Run deno lint on multiple project directories and collect diagnostics
pub fn run(_root: &Path, project_dirs: &[&Path]) -> Result<Vec<UnifiedDiagnostic>> {
    let mut all_diagnostics = Vec::new();

    for project_dir in project_dirs {
        // Check if this project has a deno.json config
        let has_config =
            project_dir.join("deno.json").exists() || project_dir.join("deno.jsonc").exists();

        // If no local config, use the tooling config
        let config = if !has_config {
            find_tooling_config(project_dir)
        } else {
            None
        };

        let result = shell::deno::lint_json(project_dir, config.as_deref())?;
        let stdout = &result.stdout;

        // Try to parse as JSON
        if stdout.trim().is_empty() {
            continue;
        }

        let lint_output: DenoLintOutput = match serde_json::from_str(stdout) {
            Ok(o) => o,
            Err(_) => {
                // deno lint might output non-JSON on success with no issues
                continue;
            }
        };

        for diag in lint_output.diagnostics {
            // deno lint uses 1-based line numbers, 0-based columns
            let line = diag.range.start.line;
            let column = diag.range.start.col + 1; // Convert to 1-based

            all_diagnostics.push(UnifiedDiagnostic {
                file: diag.filename,
                line,
                column,
                code: diag.code.clone(),
                message: diag.message.clone(),
                raw: format!("{}: {}", diag.code, diag.message),
                tool: DiagnosticTool::DenoLint,
                level: DiagnosticLevel::Warning, // deno lint issues are typically warnings
            });
        }
    }

    Ok(all_diagnostics)
}

/// Find tooling/deno.json by walking up from project dir
fn find_tooling_config(start: &Path) -> Option<std::path::PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        let tooling_config = current.join("tooling/deno.json");
        if tooling_config.exists() {
            return Some(tooling_config);
        }
        if !current.pop() {
            return None;
        }
    }
}

/// Find all directories with JS/TS projects (deno.json or package.json, respects .gitignore)
pub fn find_js_projects(root: &Path) -> Result<Vec<std::path::PathBuf>> {
    use ignore::WalkBuilder;

    let mut projects = Vec::new();

    for entry in WalkBuilder::new(root)
        .hidden(true) // Skip hidden files/dirs
        .git_ignore(true) // Respect .gitignore
        .git_exclude(true) // Respect .git/info/exclude
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            // Also skip common non-source directories
            name != "node_modules"
                && name != "dist"
                && name != "target"
                && name != "build"
                && name != "vendor"
        })
        .build()
    {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy();

        // Look for deno.json or package.json
        if (name == "deno.json" || name == "package.json")
            && let Some(parent) = entry.path().parent()
        {
            // Avoid duplicates (a dir might have both deno.json and package.json)
            if !projects.contains(&parent.to_path_buf()) {
                projects.push(parent.to_path_buf());
            }
        }
    }

    Ok(projects)
}
