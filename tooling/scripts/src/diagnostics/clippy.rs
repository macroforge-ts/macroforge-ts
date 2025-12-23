//! Clippy diagnostic runner

use super::{DiagnosticLevel, DiagnosticTool, UnifiedDiagnostic};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;
use std::process::Command;

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

/// Run clippy and collect diagnostics
pub fn run(root: &Path) -> Result<Vec<UnifiedDiagnostic>> {
    let output = Command::new("cargo")
        .args([
            "clippy",
            "--workspace",
            "--all-targets",
            "--message-format=json",
        ])
        .current_dir(root)
        .output()
        .context("Failed to run cargo clippy")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut diagnostics = Vec::new();

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
            // Skip external crates (absolute paths)
            if span.file_name.starts_with('/') && !span.file_name.contains(root.to_string_lossy().as_ref()) {
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

        diagnostics.push(UnifiedDiagnostic {
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

    Ok(diagnostics)
}
