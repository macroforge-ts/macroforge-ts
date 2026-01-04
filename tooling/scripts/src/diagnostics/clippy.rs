//! Clippy diagnostic runner

use super::{DiagnosticLevel, DiagnosticTool, UnifiedDiagnostic};
use crate::core::shell;
use anyhow::Result;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct CargoMessage {
    reason: String,
    #[serde(default)]
    message: Option<ClippyDiagnostic>,
}

#[derive(Debug, Deserialize)]
struct ClippyDiagnostic {
    #[serde(default)]
    level: String,
    #[serde(default)]
    message: String,
    #[serde(default)]
    code: Option<ClippyCode>,
    #[serde(default)]
    spans: Vec<ClippySpan>,
}

#[derive(Debug, Deserialize)]
struct ClippyCode {
    code: String,
}

#[derive(Debug, Deserialize)]
struct ClippySpan {
    file_name: String,
    line_start: u32,
    column_start: u32,
    is_primary: bool,
}

/// Run clippy on multiple Rust project directories and collect diagnostics
pub fn run(root: &Path, project_dirs: &[&Path]) -> Result<Vec<UnifiedDiagnostic>> {
    let mut all_diagnostics = Vec::new();

    for project_dir in project_dirs {
        eprintln!(
            "  Checking {:?}...",
            project_dir.strip_prefix(root).unwrap_or(project_dir)
        );

        let result = shell::cargo::clippy_json(project_dir)?;
        let stdout = &result.stdout;

        for line in stdout.lines() {
            let msg: CargoMessage = match serde_json::from_str(line) {
                Ok(m) => m,
                Err(_) => continue,
            };

            if msg.reason != "compiler-message" {
                continue;
            }

            let Some(diag) = msg.message else {
                continue;
            };

            let level = match diag.level.as_str() {
                "error" => DiagnosticLevel::Error,
                "warning" => DiagnosticLevel::Warning,
                _ => continue,
            };

            // Find primary span
            let primary_span = diag.spans.iter().find(|s| s.is_primary);
            let (file, line_num, column) = if let Some(span) = primary_span {
                // Skip external crates (paths outside root)
                if span.file_name.starts_with('/')
                    && !span.file_name.contains(root.to_string_lossy().as_ref())
                {
                    continue;
                }
                (span.file_name.clone(), span.line_start, span.column_start)
            } else {
                continue;
            };

            let code = diag
                .code
                .map(|c| c.code)
                .unwrap_or_else(|| "clippy".to_string());

            all_diagnostics.push(UnifiedDiagnostic {
                file,
                line: line_num,
                column,
                code,
                message: diag.message.clone(),
                raw: diag.message,
                tool: DiagnosticTool::Clippy,
                level,
            });
        }
    }

    Ok(all_diagnostics)
}

/// Find all Rust projects (directories with Cargo.toml, respects .gitignore)
pub fn find_rust_projects(root: &Path) -> Result<Vec<std::path::PathBuf>> {
    use ignore::WalkBuilder;

    let mut projects = Vec::new();

    for entry in WalkBuilder::new(root)
        .hidden(true) // Skip hidden files/dirs
        .git_ignore(true) // Respect .gitignore
        .git_exclude(true) // Respect .git/info/exclude
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            // Also skip common non-source directories
            name != "target" && name != "node_modules" && name != "vendor"
        })
        .build()
    {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy();

        if name == "Cargo.toml"
            && let Some(parent) = entry.path().parent()
        {
            projects.push(parent.to_path_buf());
        }
    }

    Ok(projects)
}
