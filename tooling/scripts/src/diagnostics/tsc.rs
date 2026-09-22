//! TypeScript diagnostic runner

use super::{DiagnosticLevel, DiagnosticTool, UnifiedDiagnostic};
use crate::core::shell;
use anyhow::{Context, Result, bail};
use regex::Regex;
use std::path::Path;

/// Runs `macroforge tsc` for each tsconfig and collects its diagnostics.
pub fn run(root: &Path, tsconfig_paths: &[&Path]) -> Result<Vec<UnifiedDiagnostic>> {
    let mut diagnostics = Vec::new();

    // file(line,col): error TSxxxx: message
    let error_regex = Regex::new(r"^(.+?)\((\d+),(\d+)\): error (TS\d+): (.+)$")
        .context("Failed to compile error regex")?;

    for tsconfig in tsconfig_paths {
        let result = shell::macroforge::tsc(root, tsconfig).with_context(|| {
            format!("macroforge tsc failed to start for {}", tsconfig.display())
        })?;
        let project_dir = tsconfig.parent().unwrap_or(root);
        let found_before = diagnostics.len();

        for line in result.stdout.lines().chain(result.stderr.lines()) {
            let Some(captures) = error_regex.captures(line) else {
                continue;
            };
            let file = project_dir.join(&captures[1]);
            diagnostics.push(UnifiedDiagnostic {
                file: file
                    .strip_prefix(root)
                    .unwrap_or(&file)
                    .to_string_lossy()
                    .to_string(),
                line: captures[2]
                    .parse()
                    .context("tsc reported a non-numeric line")?,
                column: captures[3]
                    .parse()
                    .context("tsc reported a non-numeric column")?,
                code: captures[4].to_string(),
                message: captures[5].to_string(),
                raw: line.to_string(),
                tool: DiagnosticTool::Tsc,
                level: DiagnosticLevel::Error,
            });
        }

        if !result.success && diagnostics.len() == found_before {
            bail!(
                "macroforge tsc failed for {} without reporting diagnostics:\n{}",
                tsconfig.display(),
                result.output()
            );
        }
    }

    Ok(diagnostics)
}

/// tsconfig.json files outside Svelte projects: plain tsc cannot resolve
/// `.svelte` imports, and svelte-check type-checks those projects instead.
pub fn find_tsconfigs(root: &Path) -> Result<Vec<std::path::PathBuf>> {
    Ok(super::discovery::find_projects(root, &["tsconfig.json"])?
        .into_iter()
        .filter(|dir| !super::discovery::is_svelte_project(dir))
        .map(|dir| dir.join("tsconfig.json"))
        .collect())
}
