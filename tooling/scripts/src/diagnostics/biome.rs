//! Biome diagnostic runner

use super::{DiagnosticLevel, DiagnosticTool, UnifiedDiagnostic};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct BiomeOutput {
    diagnostics: Vec<BiomeDiagnostic>,
}

#[derive(Debug, Deserialize)]
struct BiomeDiagnostic {
    #[serde(default)]
    location: Option<BiomeLocation>,
    #[serde(default)]
    message: Option<String>,
    severity: String,
    #[serde(default)]
    category: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BiomeLocation {
    path: Option<BiomePath>,
    #[allow(dead_code)]
    span: Option<BiomeSpan>,
}

#[derive(Debug, Deserialize)]
struct BiomePath {
    file: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct BiomeSpan {
    start: u32,
    end: u32,
}

/// Run biome check and collect diagnostics
pub fn run(root: &Path) -> Result<Vec<UnifiedDiagnostic>> {
    let output = Command::new("deno")
        .args(["run", "-A", "npm:@biomejs/biome", "check", ".", "--reporter=json"])
        .current_dir(root)
        .output()
        .context("Failed to run biome check")?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Try to parse as JSON
    if stdout.trim().is_empty() {
        return Ok(vec![]);
    }

    let biome_output: BiomeOutput = match serde_json::from_str(&stdout) {
        Ok(o) => o,
        Err(_) => {
            // Biome might output non-JSON on success
            return Ok(vec![]);
        }
    };

    let mut diagnostics = Vec::new();

    for diag in biome_output.diagnostics {
        let level = match diag.severity.as_str() {
            "error" => DiagnosticLevel::Error,
            "warning" => DiagnosticLevel::Warning,
            _ => DiagnosticLevel::Info,
        };

        let (file, line, column) = if let Some(loc) = &diag.location {
            let file = loc
                .path
                .as_ref()
                .map(|p| p.file.clone())
                .unwrap_or_default();
            // Biome uses byte offsets, we'd need source to convert to line/col
            // For now, use 1,1 as placeholder
            (file, 1, 1)
        } else {
            (String::new(), 1, 1)
        };

        let code = diag.category.unwrap_or_else(|| "biome".to_string());
        let message = diag.message.unwrap_or_default();

        diagnostics.push(UnifiedDiagnostic {
            file,
            line,
            column,
            code,
            message: message.clone(),
            raw: message,
            tool: DiagnosticTool::Biome,
            level,
        });
    }

    Ok(diagnostics)
}
