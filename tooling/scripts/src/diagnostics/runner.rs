//! Diagnostic orchestration

use super::aggregator::DiagnosticAggregator;
use super::{clippy, deno_lint, svelte, tsc};
use anyhow::Result;
use std::path::Path;

/// Tools to run
#[derive(Debug, Clone, Default)]
pub struct DiagnosticOptions {
    pub deno_lint: bool,
    pub clippy: bool,
    pub tsc: bool,
    pub svelte: bool,
}

impl DiagnosticOptions {
    /// Enable all tools
    pub fn all() -> Self {
        Self {
            deno_lint: true,
            clippy: true,
            tsc: true,
            svelte: true,
        }
    }

    /// Parse from comma-separated string
    pub fn parse(s: &str) -> Self {
        let mut opts = Self::default();
        for tool in s.split(',') {
            match tool.trim().to_lowercase().as_str() {
                "deno_lint" | "deno-lint" | "lint" => opts.deno_lint = true,
                "clippy" => opts.clippy = true,
                "tsc" | "typescript" => opts.tsc = true,
                "svelte" => opts.svelte = true,
                "all" => return Self::all(),
                _ => {}
            }
        }
        opts
    }
}

/// Main diagnostics runner
pub struct DiagnosticsRunner {
    root: std::path::PathBuf,
    options: DiagnosticOptions,
}

impl DiagnosticsRunner {
    /// Create a new runner
    pub fn new(root: impl AsRef<Path>, options: DiagnosticOptions) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
            options,
        }
    }

    /// Run all enabled diagnostics
    pub fn run(&self) -> Result<DiagnosticAggregator> {
        let mut aggregator = DiagnosticAggregator::new();

        if self.options.deno_lint {
            eprintln!("Running deno lint...");
            match deno_lint::run(&self.root) {
                Ok(diags) => aggregator.add_all(diags),
                Err(e) => eprintln!("deno lint failed: {}", e),
            }
        }

        if self.options.clippy {
            eprintln!("Running clippy...");
            match clippy::run(&self.root) {
                Ok(diags) => aggregator.add_all(diags),
                Err(e) => eprintln!("Clippy failed: {}", e),
            }
        }

        if self.options.tsc {
            eprintln!("Running TypeScript checks...");
            match tsc::find_tsconfigs(&self.root) {
                Ok(tsconfigs) => {
                    let tsconfig_refs: Vec<&Path> = tsconfigs.iter().map(|p| p.as_path()).collect();
                    match tsc::run(&self.root, &tsconfig_refs) {
                        Ok(diags) => aggregator.add_all(diags),
                        Err(e) => eprintln!("TypeScript check failed: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to find tsconfig.json files: {}", e),
            }
        }

        if self.options.svelte {
            eprintln!("Running svelte-check...");
            match svelte::find_svelte_projects(&self.root) {
                Ok(projects) => {
                    let project_refs: Vec<&Path> = projects.iter().map(|p| p.as_path()).collect();
                    match svelte::run(&self.root, &project_refs) {
                        Ok(diags) => aggregator.add_all(diags),
                        Err(e) => eprintln!("Svelte check failed: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to find svelte projects: {}", e),
            }
        }

        Ok(aggregator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_options_all() {
        let opts = DiagnosticOptions::all();
        assert!(opts.deno_lint, "deno_lint should be enabled");
        assert!(opts.clippy, "clippy should be enabled");
        assert!(opts.tsc, "tsc should be enabled");
        assert!(opts.svelte, "svelte should be enabled");
    }

    #[test]
    fn test_diagnostic_options_default() {
        let opts = DiagnosticOptions::default();
        assert!(!opts.deno_lint, "deno_lint should be disabled by default");
        assert!(!opts.clippy, "clippy should be disabled by default");
        assert!(!opts.tsc, "tsc should be disabled by default");
        assert!(!opts.svelte, "svelte should be disabled by default");
    }

    #[test]
    fn test_parse_all() {
        let opts = DiagnosticOptions::parse("all");
        assert!(opts.deno_lint, "deno_lint should be enabled with 'all'");
        assert!(opts.clippy, "clippy should be enabled with 'all'");
        assert!(opts.tsc, "tsc should be enabled with 'all'");
        assert!(opts.svelte, "svelte should be enabled with 'all'");
    }

    #[test]
    fn test_parse_all_uppercase() {
        let opts = DiagnosticOptions::parse("ALL");
        assert!(opts.deno_lint, "deno_lint should be enabled with 'ALL'");
        assert!(opts.clippy, "clippy should be enabled with 'ALL'");
        assert!(opts.tsc, "tsc should be enabled with 'ALL'");
        assert!(opts.svelte, "svelte should be enabled with 'ALL'");
    }

    #[test]
    fn test_parse_all_mixed_case() {
        let opts = DiagnosticOptions::parse("AlL");
        assert!(opts.deno_lint, "deno_lint should be enabled with 'AlL'");
        assert!(opts.clippy, "clippy should be enabled with 'AlL'");
        assert!(opts.tsc, "tsc should be enabled with 'AlL'");
        assert!(opts.svelte, "svelte should be enabled with 'AlL'");
    }

    #[test]
    fn test_parse_lint() {
        let opts = DiagnosticOptions::parse("lint");
        assert!(opts.deno_lint, "deno_lint should be enabled");
        assert!(!opts.clippy, "clippy should be disabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_clippy() {
        let opts = DiagnosticOptions::parse("clippy");
        assert!(!opts.deno_lint, "deno_lint should be disabled");
        assert!(opts.clippy, "clippy should be enabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_tsc() {
        let opts = DiagnosticOptions::parse("tsc");
        assert!(!opts.deno_lint, "deno_lint should be disabled");
        assert!(!opts.clippy, "clippy should be disabled");
        assert!(opts.tsc, "tsc should be enabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_typescript() {
        let opts = DiagnosticOptions::parse("typescript");
        assert!(!opts.deno_lint, "deno_lint should be disabled");
        assert!(!opts.clippy, "clippy should be disabled");
        assert!(opts.tsc, "tsc should be enabled with 'typescript' alias");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_svelte() {
        let opts = DiagnosticOptions::parse("svelte");
        assert!(!opts.deno_lint, "deno_lint should be disabled");
        assert!(!opts.clippy, "clippy should be disabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(opts.svelte, "svelte should be enabled");
    }

    #[test]
    fn test_parse_multiple_tools() {
        let opts = DiagnosticOptions::parse("lint,clippy");
        assert!(opts.deno_lint, "deno_lint should be enabled");
        assert!(opts.clippy, "clippy should be enabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_multiple_tools_with_spaces() {
        let opts = DiagnosticOptions::parse("lint, clippy, tsc");
        assert!(opts.deno_lint, "deno_lint should be enabled");
        assert!(opts.clippy, "clippy should be enabled");
        assert!(opts.tsc, "tsc should be enabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_all_tools_individually() {
        let opts = DiagnosticOptions::parse("lint,clippy,tsc,svelte");
        assert!(opts.deno_lint, "deno_lint should be enabled");
        assert!(opts.clippy, "clippy should be enabled");
        assert!(opts.tsc, "tsc should be enabled");
        assert!(opts.svelte, "svelte should be enabled");
    }

    #[test]
    fn test_parse_with_extra_whitespace() {
        let opts = DiagnosticOptions::parse("  lint  ,  clippy  ");
        assert!(opts.deno_lint, "deno_lint should be enabled");
        assert!(opts.clippy, "clippy should be enabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_unknown_tool_ignored() {
        let opts = DiagnosticOptions::parse("unknown");
        assert!(!opts.deno_lint, "deno_lint should be disabled");
        assert!(!opts.clippy, "clippy should be disabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_unknown_tools_with_valid_tools() {
        let opts = DiagnosticOptions::parse("unknown,lint,invalid,clippy,garbage");
        assert!(opts.deno_lint, "deno_lint should be enabled");
        assert!(opts.clippy, "clippy should be enabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_empty_string() {
        let opts = DiagnosticOptions::parse("");
        assert!(!opts.deno_lint, "deno_lint should be disabled");
        assert!(!opts.clippy, "clippy should be disabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_only_commas() {
        let opts = DiagnosticOptions::parse(",,,");
        assert!(!opts.deno_lint, "deno_lint should be disabled");
        assert!(!opts.clippy, "clippy should be disabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_only_whitespace() {
        let opts = DiagnosticOptions::parse("   ");
        assert!(!opts.deno_lint, "deno_lint should be disabled");
        assert!(!opts.clippy, "clippy should be disabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_all_overrides_previous_selections() {
        // When "all" is encountered, it should return immediately with all enabled
        let opts = DiagnosticOptions::parse("lint,all");
        assert!(opts.deno_lint, "deno_lint should be enabled");
        assert!(opts.clippy, "clippy should be enabled");
        assert!(opts.tsc, "tsc should be enabled");
        assert!(opts.svelte, "svelte should be enabled");
    }

    #[test]
    fn test_parse_all_at_beginning() {
        let opts = DiagnosticOptions::parse("all,lint");
        assert!(opts.deno_lint, "deno_lint should be enabled");
        assert!(opts.clippy, "clippy should be enabled");
        assert!(opts.tsc, "tsc should be enabled");
        assert!(opts.svelte, "svelte should be enabled");
    }

    #[test]
    fn test_parse_duplicate_tools() {
        let opts = DiagnosticOptions::parse("lint,lint,lint");
        assert!(opts.deno_lint, "deno_lint should be enabled");
        assert!(!opts.clippy, "clippy should be disabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_typescript_and_tsc_both_work() {
        let opts1 = DiagnosticOptions::parse("typescript");
        let opts2 = DiagnosticOptions::parse("tsc");

        assert_eq!(opts1.tsc, opts2.tsc, "typescript and tsc should both enable tsc");
        assert!(opts1.tsc, "typescript should enable tsc");
        assert!(opts2.tsc, "tsc should enable tsc");
    }

    #[test]
    fn test_diagnostics_runner_new() {
        let opts = DiagnosticOptions::all();
        let runner = DiagnosticsRunner::new("/tmp/test", opts.clone());

        assert_eq!(runner.root.to_str().unwrap(), "/tmp/test");
        assert!(runner.options.deno_lint);
        assert!(runner.options.clippy);
        assert!(runner.options.tsc);
        assert!(runner.options.svelte);
    }
}
