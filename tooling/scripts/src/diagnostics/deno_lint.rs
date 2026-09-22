//! Deno lint diagnostic runner

use super::{DiagnosticLevel, DiagnosticTool, UnifiedDiagnostic};
use crate::core::shell;
use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// `deno lint --json` output.
#[derive(Deserialize)]
struct DenoLintOutput {
    diagnostics: Vec<DenoLintDiagnostic>,
    /// Files deno could not lint at all, such as ones that fail to parse.
    errors: Vec<DenoLintError>,
}

#[derive(Deserialize)]
struct DenoLintDiagnostic {
    range: DenoLintRange,
    /// A `file://` URL.
    filename: String,
    message: String,
    code: String,
}

#[derive(Deserialize)]
struct DenoLintRange {
    start: DenoLintPosition,
}

/// One-based line, zero-based column.
#[derive(Deserialize)]
struct DenoLintPosition {
    line: u32,
    col: u32,
}

#[derive(Deserialize)]
struct DenoLintError {
    file_path: String,
    message: String,
}

/// Runs deno lint once over the repository and collects its diagnostics.
///
/// The root `deno.json` governs every file beneath it, including projects that
/// are not workspace members, so one run covers the tree.
pub fn run(root: &Path) -> Result<Vec<UnifiedDiagnostic>> {
    let result = shell::deno::lint_json(root)?;
    let output: DenoLintOutput = serde_json::from_str(&result.stdout)
        .with_context(|| format!("deno lint did not produce JSON:\n{}", result.output()))?;

    let mut diagnostics = Vec::new();
    for diagnostic in output.diagnostics {
        diagnostics.push(UnifiedDiagnostic {
            file: relative_to_root(root, &file_url_path(&diagnostic.filename)?),
            line: diagnostic.range.start.line,
            column: diagnostic.range.start.col + 1,
            raw: format!("{}: {}", diagnostic.code, diagnostic.message),
            code: diagnostic.code,
            message: diagnostic.message,
            tool: DiagnosticTool::DenoLint,
            level: DiagnosticLevel::Warning,
        });
    }
    for error in output.errors {
        diagnostics.push(UnifiedDiagnostic {
            file: relative_to_root(root, Path::new(&error.file_path)),
            line: 1,
            column: 1,
            raw: error.message.clone(),
            code: "deno-lint".to_string(),
            message: error.message,
            tool: DiagnosticTool::DenoLint,
            level: DiagnosticLevel::Error,
        });
    }
    Ok(diagnostics)
}

fn file_url_path(file_url: &str) -> Result<PathBuf> {
    let url = url::Url::parse(file_url)
        .with_context(|| format!("deno lint reported an invalid file URL: {file_url}"))?;
    match url.to_file_path() {
        Ok(path) => Ok(path),
        Err(()) => bail!("deno lint reported a non-file URL: {file_url}"),
    }
}

fn relative_to_root(root: &Path, file: &Path) -> String {
    file.strip_prefix(root)
        .unwrap_or(file)
        .to_string_lossy()
        .into_owned()
}
