//! Svelte diagnostic runner

use super::{DiagnosticLevel, DiagnosticTool, UnifiedDiagnostic};
use crate::core::shell;
use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::path::Path;

/// One diagnostic line of `svelte-check --output machine-verbose`.
#[derive(Deserialize)]
struct MachineDiagnostic {
    #[serde(rename = "type")]
    severity: MachineSeverity,
    filename: String,
    start: MachinePosition,
    message: String,
    code: Option<MachineCode>,
}

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum MachineSeverity {
    Error,
    Warning,
}

/// Zero-based, as the language server reports it.
#[derive(Deserialize)]
struct MachinePosition {
    line: u32,
    character: u32,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum MachineCode {
    Number(i64),
    Text(String),
}

/// Runs `macroforge svelte-check` in each project and collects its diagnostics.
pub fn run(root: &Path, project_dirs: &[&Path]) -> Result<Vec<UnifiedDiagnostic>> {
    let mut diagnostics = Vec::new();

    for project_dir in project_dirs {
        let result = shell::macroforge::svelte_check(root, project_dir).with_context(|| {
            format!(
                "macroforge svelte-check failed to start in {}",
                project_dir.display()
            )
        })?;
        let found_before = diagnostics.len();
        let mut reported_total = None;

        // Every line is `<timestamp> <payload>`.
        for line in result.stdout.lines() {
            let Some((_, payload)) = line.split_once(' ') else {
                continue;
            };
            if let Some(summary) = payload.strip_prefix("COMPLETED ") {
                reported_total = Some(completed_total(summary)?);
                continue;
            }
            if !payload.starts_with('{') {
                continue;
            }
            let entry: MachineDiagnostic = serde_json::from_str(payload)
                .with_context(|| format!("unexpected svelte-check diagnostic: {payload}"))?;
            let file = project_dir.join(&entry.filename);
            diagnostics.push(UnifiedDiagnostic {
                file: file
                    .strip_prefix(root)
                    .unwrap_or(&file)
                    .to_string_lossy()
                    .to_string(),
                line: entry.start.line + 1,
                column: entry.start.character + 1,
                code: match entry.code {
                    Some(MachineCode::Number(number)) => number.to_string(),
                    Some(MachineCode::Text(text)) => text,
                    None => "svelte".to_string(),
                },
                message: entry.message,
                raw: line.to_string(),
                tool: DiagnosticTool::SvelteCheck,
                level: match entry.severity {
                    MachineSeverity::Error => DiagnosticLevel::Error,
                    MachineSeverity::Warning => DiagnosticLevel::Warning,
                },
            });
        }

        let parsed = diagnostics.len() - found_before;
        match reported_total {
            Some(total) if total == parsed => {}
            Some(total) => bail!(
                "svelte-check in {} reported {total} diagnostics but {parsed} were parsed",
                project_dir.display()
            ),
            None => bail!(
                "svelte-check in {} did not complete:\n{}",
                project_dir.display(),
                result.output()
            ),
        }
    }

    Ok(diagnostics)
}

/// Sums errors and warnings from `N FILES E ERRORS W WARNINGS F FILES_WITH_PROBLEMS`.
fn completed_total(summary: &str) -> Result<usize> {
    let words: Vec<&str> = summary.split_whitespace().collect();
    let count_before = |label: &str| -> Result<usize> {
        let position = words
            .iter()
            .position(|word| *word == label)
            .with_context(|| format!("svelte-check summary has no {label}: {summary}"))?;
        let count = position
            .checked_sub(1)
            .and_then(|index| words.get(index))
            .with_context(|| format!("svelte-check summary has no count for {label}"))?;
        count
            .parse()
            .with_context(|| format!("svelte-check {label} count is not a number: {count}"))
    };
    Ok(count_before("ERRORS")? + count_before("WARNINGS")?)
}

pub fn find_svelte_projects(root: &Path) -> Result<Vec<std::path::PathBuf>> {
    super::discovery::find_projects(root, super::discovery::SVELTE_MARKERS)
}
