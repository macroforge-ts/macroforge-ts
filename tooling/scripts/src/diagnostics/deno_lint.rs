//! Deno lint diagnostic runner

use super::{DiagnosticLevel, DiagnosticTool, UnifiedDiagnostic};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;
use std::process::Command;

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

/// Run deno lint and collect diagnostics
pub fn run(root: &Path) -> Result<Vec<UnifiedDiagnostic>> {
    let output = Command::new("deno")
        .args(["lint", "--json", "."])
        .current_dir(root)
        .output()
        .context("Failed to run deno lint")?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Try to parse as JSON
    if stdout.trim().is_empty() {
        return Ok(vec![]);
    }

    let lint_output: DenoLintOutput = match serde_json::from_str(&stdout) {
        Ok(o) => o,
        Err(_) => {
            // deno lint might output non-JSON on success with no issues
            return Ok(vec![]);
        }
    };

    let mut diagnostics = Vec::new();

    for diag in lint_output.diagnostics {
        // deno lint uses 1-based line numbers, 0-based columns
        let line = diag.range.start.line;
        let column = diag.range.start.col + 1; // Convert to 1-based

        diagnostics.push(UnifiedDiagnostic {
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

    Ok(diagnostics)
}
