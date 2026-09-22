//! Clippy diagnostic runner

use super::{DiagnosticLevel, DiagnosticTool, UnifiedDiagnostic};
use crate::core::shell;
use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::path::{Path, PathBuf};

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

/// The root `Cargo.toml` fields that say which crates the workspace covers.
#[derive(Deserialize)]
struct RootManifest {
    workspace: WorkspaceTable,
}

#[derive(Deserialize)]
struct WorkspaceTable {
    #[serde(default)]
    exclude: Vec<String>,
}

/// Crates the root workspace excludes, which build against their own lockfile
/// and so need a pass of their own.
pub fn excluded_crates(root: &Path) -> Result<Vec<PathBuf>> {
    let manifest_path = root.join("Cargo.toml");
    let text = std::fs::read_to_string(&manifest_path)
        .with_context(|| format!("failed to read {}", manifest_path.display()))?;
    let manifest: RootManifest = toml::from_str(&text)
        .with_context(|| format!("failed to parse {}", manifest_path.display()))?;
    Ok(manifest
        .workspace
        .exclude
        .iter()
        .map(|path| root.join(path))
        .collect())
}

/// Runs clippy over the whole workspace with every feature and target, then
/// over each excluded crate, and collects the diagnostics.
pub fn run(root: &Path) -> Result<Vec<UnifiedDiagnostic>> {
    let mut all_diagnostics = Vec::new();

    eprintln!("  Checking the workspace...");
    let workspace = shell::cargo::clippy_workspace_json(root)?;
    collect(root, root, &workspace, &mut all_diagnostics)?;

    for crate_dir in excluded_crates(root)? {
        eprintln!(
            "  Checking {}...",
            crate_dir.strip_prefix(root).unwrap_or(&crate_dir).display()
        );
        let result = shell::cargo::clippy_json(&crate_dir)?;
        collect(root, &crate_dir, &result, &mut all_diagnostics)?;
    }

    Ok(all_diagnostics)
}

/// Parses one clippy run's JSON messages into `out`. A failed run that
/// reported nothing parseable is an error, not a clean result.
fn collect(
    root: &Path,
    project_dir: &Path,
    result: &shell::CommandResult,
    out: &mut Vec<UnifiedDiagnostic>,
) -> Result<()> {
    let found_before = out.len();

    for line in result.stdout.lines() {
        // Cargo interleaves non-JSON progress lines with its messages.
        let Ok(msg) = serde_json::from_str::<CargoMessage>(line) else {
            continue;
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

        let Some(span) = diag.spans.iter().find(|span| span.is_primary) else {
            continue;
        };
        // Skip external crates (paths outside root)
        if span.file_name.starts_with('/')
            && !span.file_name.contains(root.to_string_lossy().as_ref())
        {
            continue;
        }

        let code = diag
            .code
            .map(|code| code.code)
            .unwrap_or_else(|| "clippy".to_string());

        out.push(UnifiedDiagnostic {
            file: span.file_name.clone(),
            line: span.line_start,
            column: span.column_start,
            code,
            message: diag.message.clone(),
            raw: diag.message,
            tool: DiagnosticTool::Clippy,
            level,
        });
    }

    if !result.success && out.len() == found_before {
        bail!(
            "clippy failed in {} without reporting diagnostics:\n{}",
            project_dir.display(),
            result.output()
        );
    }
    Ok(())
}
