//! Diagnostic orchestration

use super::aggregator::DiagnosticAggregator;
use super::{biome, clippy, svelte, tsc};
use anyhow::Result;
use std::path::Path;

/// Tools to run
#[derive(Debug, Clone, Default)]
pub struct DiagnosticOptions {
    pub biome: bool,
    pub clippy: bool,
    pub tsc: bool,
    pub svelte: bool,
}

impl DiagnosticOptions {
    /// Enable all tools
    pub fn all() -> Self {
        Self {
            biome: true,
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
                "biome" => opts.biome = true,
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

        if self.options.biome {
            eprintln!("Running biome check...");
            match biome::run(&self.root) {
                Ok(diags) => aggregator.add_all(diags),
                Err(e) => eprintln!("Biome check failed: {}", e),
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
        assert!(opts.biome, "biome should be enabled");
        assert!(opts.clippy, "clippy should be enabled");
        assert!(opts.tsc, "tsc should be enabled");
        assert!(opts.svelte, "svelte should be enabled");
    }

    #[test]
    fn test_diagnostic_options_default() {
        let opts = DiagnosticOptions::default();
        assert!(!opts.biome, "biome should be disabled by default");
        assert!(!opts.clippy, "clippy should be disabled by default");
        assert!(!opts.tsc, "tsc should be disabled by default");
        assert!(!opts.svelte, "svelte should be disabled by default");
    }

    #[test]
    fn test_parse_all() {
        let opts = DiagnosticOptions::parse("all");
        assert!(opts.biome, "biome should be enabled with 'all'");
        assert!(opts.clippy, "clippy should be enabled with 'all'");
        assert!(opts.tsc, "tsc should be enabled with 'all'");
        assert!(opts.svelte, "svelte should be enabled with 'all'");
    }

    #[test]
    fn test_parse_all_uppercase() {
        let opts = DiagnosticOptions::parse("ALL");
        assert!(opts.biome, "biome should be enabled with 'ALL'");
        assert!(opts.clippy, "clippy should be enabled with 'ALL'");
        assert!(opts.tsc, "tsc should be enabled with 'ALL'");
        assert!(opts.svelte, "svelte should be enabled with 'ALL'");
    }

    #[test]
    fn test_parse_all_mixed_case() {
        let opts = DiagnosticOptions::parse("AlL");
        assert!(opts.biome, "biome should be enabled with 'AlL'");
        assert!(opts.clippy, "clippy should be enabled with 'AlL'");
        assert!(opts.tsc, "tsc should be enabled with 'AlL'");
        assert!(opts.svelte, "svelte should be enabled with 'AlL'");
    }

    #[test]
    fn test_parse_biome() {
        let opts = DiagnosticOptions::parse("biome");
        assert!(opts.biome, "biome should be enabled");
        assert!(!opts.clippy, "clippy should be disabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_clippy() {
        let opts = DiagnosticOptions::parse("clippy");
        assert!(!opts.biome, "biome should be disabled");
        assert!(opts.clippy, "clippy should be enabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_tsc() {
        let opts = DiagnosticOptions::parse("tsc");
        assert!(!opts.biome, "biome should be disabled");
        assert!(!opts.clippy, "clippy should be disabled");
        assert!(opts.tsc, "tsc should be enabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_typescript() {
        let opts = DiagnosticOptions::parse("typescript");
        assert!(!opts.biome, "biome should be disabled");
        assert!(!opts.clippy, "clippy should be disabled");
        assert!(opts.tsc, "tsc should be enabled with 'typescript' alias");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_svelte() {
        let opts = DiagnosticOptions::parse("svelte");
        assert!(!opts.biome, "biome should be disabled");
        assert!(!opts.clippy, "clippy should be disabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(opts.svelte, "svelte should be enabled");
    }

    #[test]
    fn test_parse_multiple_tools() {
        let opts = DiagnosticOptions::parse("biome,clippy");
        assert!(opts.biome, "biome should be enabled");
        assert!(opts.clippy, "clippy should be enabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_multiple_tools_with_spaces() {
        let opts = DiagnosticOptions::parse("biome, clippy, tsc");
        assert!(opts.biome, "biome should be enabled");
        assert!(opts.clippy, "clippy should be enabled");
        assert!(opts.tsc, "tsc should be enabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_all_tools_individually() {
        let opts = DiagnosticOptions::parse("biome,clippy,tsc,svelte");
        assert!(opts.biome, "biome should be enabled");
        assert!(opts.clippy, "clippy should be enabled");
        assert!(opts.tsc, "tsc should be enabled");
        assert!(opts.svelte, "svelte should be enabled");
    }

    #[test]
    fn test_parse_with_extra_whitespace() {
        let opts = DiagnosticOptions::parse("  biome  ,  clippy  ");
        assert!(opts.biome, "biome should be enabled");
        assert!(opts.clippy, "clippy should be enabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_mixed_case_tools() {
        let opts = DiagnosticOptions::parse("BiOmE,ClIpPy,TsC,SvElTe");
        assert!(opts.biome, "biome should be enabled with mixed case");
        assert!(opts.clippy, "clippy should be enabled with mixed case");
        assert!(opts.tsc, "tsc should be enabled with mixed case");
        assert!(opts.svelte, "svelte should be enabled with mixed case");
    }

    #[test]
    fn test_parse_uppercase_tools() {
        let opts = DiagnosticOptions::parse("BIOME,CLIPPY,TSC,SVELTE");
        assert!(opts.biome, "biome should be enabled with uppercase");
        assert!(opts.clippy, "clippy should be enabled with uppercase");
        assert!(opts.tsc, "tsc should be enabled with uppercase");
        assert!(opts.svelte, "svelte should be enabled with uppercase");
    }

    #[test]
    fn test_parse_unknown_tool_ignored() {
        let opts = DiagnosticOptions::parse("unknown");
        assert!(!opts.biome, "biome should be disabled");
        assert!(!opts.clippy, "clippy should be disabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_unknown_tools_with_valid_tools() {
        let opts = DiagnosticOptions::parse("unknown,biome,invalid,clippy,garbage");
        assert!(opts.biome, "biome should be enabled");
        assert!(opts.clippy, "clippy should be enabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_empty_string() {
        let opts = DiagnosticOptions::parse("");
        assert!(!opts.biome, "biome should be disabled");
        assert!(!opts.clippy, "clippy should be disabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_only_commas() {
        let opts = DiagnosticOptions::parse(",,,");
        assert!(!opts.biome, "biome should be disabled");
        assert!(!opts.clippy, "clippy should be disabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_only_whitespace() {
        let opts = DiagnosticOptions::parse("   ");
        assert!(!opts.biome, "biome should be disabled");
        assert!(!opts.clippy, "clippy should be disabled");
        assert!(!opts.tsc, "tsc should be disabled");
        assert!(!opts.svelte, "svelte should be disabled");
    }

    #[test]
    fn test_parse_all_overrides_previous_selections() {
        // When "all" is encountered, it should return immediately with all enabled
        let opts = DiagnosticOptions::parse("biome,all");
        assert!(opts.biome, "biome should be enabled");
        assert!(opts.clippy, "clippy should be enabled");
        assert!(opts.tsc, "tsc should be enabled");
        assert!(opts.svelte, "svelte should be enabled");
    }

    #[test]
    fn test_parse_all_at_beginning() {
        let opts = DiagnosticOptions::parse("all,biome");
        assert!(opts.biome, "biome should be enabled");
        assert!(opts.clippy, "clippy should be enabled");
        assert!(opts.tsc, "tsc should be enabled");
        assert!(opts.svelte, "svelte should be enabled");
    }

    #[test]
    fn test_parse_duplicate_tools() {
        let opts = DiagnosticOptions::parse("biome,biome,biome");
        assert!(opts.biome, "biome should be enabled");
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
        assert!(runner.options.biome);
        assert!(runner.options.clippy);
        assert!(runner.options.tsc);
        assert!(runner.options.svelte);
    }
}
