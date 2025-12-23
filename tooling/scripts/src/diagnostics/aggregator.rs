//! Diagnostic aggregation and deduplication

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Source tool for a diagnostic
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticTool {
    Biome,
    Clippy,
    Tsc,
    TsPlugin,
    SvelteCheck,
    Macroforge,
}

/// Severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
}

/// Unified diagnostic format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedDiagnostic {
    /// Normalized file path relative to project root
    pub file: String,
    /// 1-based line number
    pub line: u32,
    /// 1-based column number
    pub column: u32,
    /// Tool-specific error code (e.g., "TS2304", "clippy::needless_return")
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Raw output line for logging
    pub raw: String,
    /// Source tool identifier
    pub tool: DiagnosticTool,
    /// Severity level
    pub level: DiagnosticLevel,
}

impl UnifiedDiagnostic {
    /// Create a deduplication key for this diagnostic
    pub fn dedup_key(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.file, self.line, self.column, self.code, self.message
        )
    }
}

/// Diagnostic aggregator with deduplication
#[derive(Debug, Default)]
pub struct DiagnosticAggregator {
    diagnostics: Vec<UnifiedDiagnostic>,
    seen: HashSet<String>,
}

impl DiagnosticAggregator {
    /// Create a new aggregator
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a diagnostic, deduplicating by key
    pub fn add(&mut self, diag: UnifiedDiagnostic) {
        let key = diag.dedup_key();
        if self.seen.insert(key) {
            self.diagnostics.push(diag);
        }
    }

    /// Add multiple diagnostics
    pub fn add_all(&mut self, diags: Vec<UnifiedDiagnostic>) {
        for diag in diags {
            self.add(diag);
        }
    }

    /// Get error count
    pub fn error_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| matches!(d.level, DiagnosticLevel::Error))
            .count()
    }

    /// Get warning count
    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| matches!(d.level, DiagnosticLevel::Warning))
            .count()
    }

    /// Get all diagnostics
    pub fn diagnostics(&self) -> &[UnifiedDiagnostic] {
        &self.diagnostics
    }

    /// Group diagnostics by error code
    pub fn by_code(&self) -> HashMap<String, Vec<&UnifiedDiagnostic>> {
        let mut grouped: HashMap<String, Vec<&UnifiedDiagnostic>> = HashMap::new();
        for diag in &self.diagnostics {
            grouped.entry(diag.code.clone()).or_default().push(diag);
        }
        grouped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a test diagnostic with all fields specified
    fn create_diagnostic(
        file: &str,
        line: u32,
        column: u32,
        code: &str,
        message: &str,
        tool: DiagnosticTool,
        level: DiagnosticLevel,
    ) -> UnifiedDiagnostic {
        UnifiedDiagnostic {
            file: file.to_string(),
            line,
            column,
            code: code.to_string(),
            message: message.to_string(),
            raw: format!("{}:{}:{}: {}", file, line, column, message),
            tool,
            level,
        }
    }

    // Helper function to create a test error diagnostic
    fn create_error(file: &str, line: u32, column: u32, code: &str, message: &str) -> UnifiedDiagnostic {
        create_diagnostic(file, line, column, code, message, DiagnosticTool::Clippy, DiagnosticLevel::Error)
    }

    // Helper function to create a test warning diagnostic
    fn create_warning(file: &str, line: u32, column: u32, code: &str, message: &str) -> UnifiedDiagnostic {
        create_diagnostic(file, line, column, code, message, DiagnosticTool::Biome, DiagnosticLevel::Warning)
    }

    // Helper function to create a test info diagnostic
    fn create_info(file: &str, line: u32, column: u32, code: &str, message: &str) -> UnifiedDiagnostic {
        create_diagnostic(file, line, column, code, message, DiagnosticTool::Tsc, DiagnosticLevel::Info)
    }

    #[test]
    fn test_dedup_key_generation() {
        let diag = create_error("src/main.rs", 42, 10, "E0308", "mismatched types");
        let key = diag.dedup_key();
        assert_eq!(key, "src/main.rs:42:10:E0308:mismatched types");
    }

    #[test]
    fn test_dedup_key_uniqueness() {
        let diag1 = create_error("src/main.rs", 42, 10, "E0308", "mismatched types");
        let diag2 = create_error("src/main.rs", 42, 10, "E0308", "mismatched types");
        let diag3 = create_error("src/main.rs", 43, 10, "E0308", "mismatched types");
        let diag4 = create_error("src/main.rs", 42, 11, "E0308", "mismatched types");
        let diag5 = create_error("src/main.rs", 42, 10, "E0309", "mismatched types");
        let diag6 = create_error("src/main.rs", 42, 10, "E0308", "different message");
        let diag7 = create_error("src/other.rs", 42, 10, "E0308", "mismatched types");

        // Same diagnostics should have same key
        assert_eq!(diag1.dedup_key(), diag2.dedup_key());

        // Different line should have different key
        assert_ne!(diag1.dedup_key(), diag3.dedup_key());

        // Different column should have different key
        assert_ne!(diag1.dedup_key(), diag4.dedup_key());

        // Different code should have different key
        assert_ne!(diag1.dedup_key(), diag5.dedup_key());

        // Different message should have different key
        assert_ne!(diag1.dedup_key(), diag6.dedup_key());

        // Different file should have different key
        assert_ne!(diag1.dedup_key(), diag7.dedup_key());
    }

    #[test]
    fn test_add_single_diagnostic() {
        let mut aggregator = DiagnosticAggregator::new();
        let diag = create_error("src/main.rs", 42, 10, "E0308", "mismatched types");

        aggregator.add(diag);

        assert_eq!(aggregator.diagnostics().len(), 1);
        assert_eq!(aggregator.diagnostics()[0].file, "src/main.rs");
        assert_eq!(aggregator.diagnostics()[0].line, 42);
        assert_eq!(aggregator.diagnostics()[0].column, 10);
        assert_eq!(aggregator.diagnostics()[0].code, "E0308");
        assert_eq!(aggregator.diagnostics()[0].message, "mismatched types");
    }

    #[test]
    fn test_add_multiple_different_diagnostics() {
        let mut aggregator = DiagnosticAggregator::new();

        aggregator.add(create_error("src/main.rs", 42, 10, "E0308", "mismatched types"));
        aggregator.add(create_warning("src/lib.rs", 100, 5, "W1234", "unused variable"));
        aggregator.add(create_info("src/util.rs", 15, 1, "I5678", "consider using as_ref"));

        assert_eq!(aggregator.diagnostics().len(), 3);
    }

    #[test]
    fn test_add_all_multiple_diagnostics() {
        let mut aggregator = DiagnosticAggregator::new();

        let diags = vec![
            create_error("src/main.rs", 42, 10, "E0308", "mismatched types"),
            create_warning("src/lib.rs", 100, 5, "W1234", "unused variable"),
            create_info("src/util.rs", 15, 1, "I5678", "consider using as_ref"),
        ];

        aggregator.add_all(diags);

        assert_eq!(aggregator.diagnostics().len(), 3);
    }

    #[test]
    fn test_add_all_empty_vector() {
        let mut aggregator = DiagnosticAggregator::new();
        aggregator.add_all(vec![]);
        assert_eq!(aggregator.diagnostics().len(), 0);
    }

    #[test]
    fn test_deduplication_exact_duplicates() {
        let mut aggregator = DiagnosticAggregator::new();

        let diag1 = create_error("src/main.rs", 42, 10, "E0308", "mismatched types");
        let diag2 = create_error("src/main.rs", 42, 10, "E0308", "mismatched types");

        aggregator.add(diag1);
        aggregator.add(diag2);

        // Only one diagnostic should be stored
        assert_eq!(aggregator.diagnostics().len(), 1);
    }

    #[test]
    fn test_deduplication_with_add_all() {
        let mut aggregator = DiagnosticAggregator::new();

        let diags = vec![
            create_error("src/main.rs", 42, 10, "E0308", "mismatched types"),
            create_error("src/main.rs", 42, 10, "E0308", "mismatched types"),
            create_error("src/main.rs", 42, 10, "E0308", "mismatched types"),
            create_warning("src/lib.rs", 100, 5, "W1234", "unused variable"),
            create_warning("src/lib.rs", 100, 5, "W1234", "unused variable"),
        ];

        aggregator.add_all(diags);

        // Only 2 unique diagnostics should be stored
        assert_eq!(aggregator.diagnostics().len(), 2);
    }

    #[test]
    fn test_deduplication_preserves_first_occurrence() {
        let mut aggregator = DiagnosticAggregator::new();

        let diag1 = create_diagnostic(
            "src/main.rs",
            42,
            10,
            "E0308",
            "mismatched types",
            DiagnosticTool::Clippy,
            DiagnosticLevel::Error,
        );
        let diag2 = create_diagnostic(
            "src/main.rs",
            42,
            10,
            "E0308",
            "mismatched types",
            DiagnosticTool::Tsc, // Different tool, same dedup key
            DiagnosticLevel::Warning, // Different level, same dedup key
        );

        aggregator.add(diag1);
        aggregator.add(diag2);

        assert_eq!(aggregator.diagnostics().len(), 1);
        // First occurrence should be preserved
        assert_eq!(aggregator.diagnostics()[0].tool, DiagnosticTool::Clippy);
        assert_eq!(aggregator.diagnostics()[0].level, DiagnosticLevel::Error);
    }

    #[test]
    fn test_deduplication_different_locations() {
        let mut aggregator = DiagnosticAggregator::new();

        aggregator.add(create_error("src/main.rs", 42, 10, "E0308", "mismatched types"));
        aggregator.add(create_error("src/main.rs", 43, 10, "E0308", "mismatched types")); // Different line
        aggregator.add(create_error("src/main.rs", 42, 11, "E0308", "mismatched types")); // Different column
        aggregator.add(create_error("src/lib.rs", 42, 10, "E0308", "mismatched types"));  // Different file

        // All should be stored as they have different locations
        assert_eq!(aggregator.diagnostics().len(), 4);
    }

    #[test]
    fn test_error_count_empty() {
        let aggregator = DiagnosticAggregator::new();
        assert_eq!(aggregator.error_count(), 0);
    }

    #[test]
    fn test_error_count_only_errors() {
        let mut aggregator = DiagnosticAggregator::new();

        aggregator.add(create_error("src/main.rs", 42, 10, "E0308", "error 1"));
        aggregator.add(create_error("src/main.rs", 43, 10, "E0309", "error 2"));
        aggregator.add(create_error("src/lib.rs", 100, 5, "E0310", "error 3"));

        assert_eq!(aggregator.error_count(), 3);
        assert_eq!(aggregator.warning_count(), 0);
    }

    #[test]
    fn test_error_count_mixed_levels() {
        let mut aggregator = DiagnosticAggregator::new();

        aggregator.add(create_error("src/main.rs", 42, 10, "E0308", "error 1"));
        aggregator.add(create_warning("src/main.rs", 43, 10, "W1234", "warning 1"));
        aggregator.add(create_error("src/lib.rs", 100, 5, "E0309", "error 2"));
        aggregator.add(create_info("src/util.rs", 15, 1, "I5678", "info 1"));
        aggregator.add(create_warning("src/lib.rs", 101, 5, "W1235", "warning 2"));
        aggregator.add(create_error("src/util.rs", 20, 3, "E0310", "error 3"));

        assert_eq!(aggregator.error_count(), 3);
        assert_eq!(aggregator.warning_count(), 2);
    }

    #[test]
    fn test_warning_count_empty() {
        let aggregator = DiagnosticAggregator::new();
        assert_eq!(aggregator.warning_count(), 0);
    }

    #[test]
    fn test_warning_count_only_warnings() {
        let mut aggregator = DiagnosticAggregator::new();

        aggregator.add(create_warning("src/main.rs", 42, 10, "W1234", "warning 1"));
        aggregator.add(create_warning("src/main.rs", 43, 10, "W1235", "warning 2"));
        aggregator.add(create_warning("src/lib.rs", 100, 5, "W1236", "warning 3"));

        assert_eq!(aggregator.warning_count(), 3);
        assert_eq!(aggregator.error_count(), 0);
    }

    #[test]
    fn test_count_with_duplicates() {
        let mut aggregator = DiagnosticAggregator::new();

        // Add 5 errors, but 2 are duplicates
        aggregator.add(create_error("src/main.rs", 42, 10, "E0308", "error 1"));
        aggregator.add(create_error("src/main.rs", 42, 10, "E0308", "error 1")); // Duplicate
        aggregator.add(create_error("src/main.rs", 43, 10, "E0309", "error 2"));
        aggregator.add(create_error("src/main.rs", 43, 10, "E0309", "error 2")); // Duplicate
        aggregator.add(create_error("src/lib.rs", 100, 5, "E0310", "error 3"));

        // Should count only unique errors
        assert_eq!(aggregator.error_count(), 3);
    }

    #[test]
    fn test_by_code_empty() {
        let aggregator = DiagnosticAggregator::new();
        let grouped = aggregator.by_code();
        assert_eq!(grouped.len(), 0);
    }

    #[test]
    fn test_by_code_single_code() {
        let mut aggregator = DiagnosticAggregator::new();

        aggregator.add(create_error("src/main.rs", 42, 10, "E0308", "error 1"));
        aggregator.add(create_error("src/main.rs", 43, 10, "E0308", "error 2"));
        aggregator.add(create_error("src/lib.rs", 100, 5, "E0308", "error 3"));

        let grouped = aggregator.by_code();

        assert_eq!(grouped.len(), 1);
        assert_eq!(grouped.get("E0308").unwrap().len(), 3);
    }

    #[test]
    fn test_by_code_multiple_codes() {
        let mut aggregator = DiagnosticAggregator::new();

        aggregator.add(create_error("src/main.rs", 42, 10, "E0308", "error 1"));
        aggregator.add(create_error("src/main.rs", 43, 10, "E0308", "error 2"));
        aggregator.add(create_warning("src/lib.rs", 100, 5, "W1234", "warning 1"));
        aggregator.add(create_warning("src/lib.rs", 101, 5, "W1234", "warning 2"));
        aggregator.add(create_info("src/util.rs", 15, 1, "I5678", "info 1"));

        let grouped = aggregator.by_code();

        assert_eq!(grouped.len(), 3);
        assert_eq!(grouped.get("E0308").unwrap().len(), 2);
        assert_eq!(grouped.get("W1234").unwrap().len(), 2);
        assert_eq!(grouped.get("I5678").unwrap().len(), 1);
    }

    #[test]
    fn test_by_code_preserves_diagnostic_data() {
        let mut aggregator = DiagnosticAggregator::new();

        aggregator.add(create_error("src/main.rs", 42, 10, "E0308", "error 1"));
        aggregator.add(create_error("src/lib.rs", 100, 5, "E0308", "error 2"));

        let grouped = aggregator.by_code();
        let e0308_diags = grouped.get("E0308").unwrap();

        assert_eq!(e0308_diags[0].file, "src/main.rs");
        assert_eq!(e0308_diags[0].line, 42);
        assert_eq!(e0308_diags[0].message, "error 1");

        assert_eq!(e0308_diags[1].file, "src/lib.rs");
        assert_eq!(e0308_diags[1].line, 100);
        assert_eq!(e0308_diags[1].message, "error 2");
    }

    #[test]
    fn test_by_code_different_tools_same_code() {
        let mut aggregator = DiagnosticAggregator::new();

        aggregator.add(create_diagnostic(
            "src/main.rs",
            42,
            10,
            "TS2304",
            "Cannot find name",
            DiagnosticTool::Tsc,
            DiagnosticLevel::Error,
        ));
        aggregator.add(create_diagnostic(
            "src/lib.rs",
            100,
            5,
            "TS2304",
            "Cannot find name",
            DiagnosticTool::TsPlugin,
            DiagnosticLevel::Error,
        ));

        let grouped = aggregator.by_code();

        assert_eq!(grouped.len(), 1);
        assert_eq!(grouped.get("TS2304").unwrap().len(), 2);
        assert_eq!(grouped.get("TS2304").unwrap()[0].tool, DiagnosticTool::Tsc);
        assert_eq!(grouped.get("TS2304").unwrap()[1].tool, DiagnosticTool::TsPlugin);
    }

    #[test]
    fn test_diagnostics_getter() {
        let mut aggregator = DiagnosticAggregator::new();

        aggregator.add(create_error("src/main.rs", 42, 10, "E0308", "error 1"));
        aggregator.add(create_warning("src/lib.rs", 100, 5, "W1234", "warning 1"));

        let diags = aggregator.diagnostics();
        assert_eq!(diags.len(), 2);
        assert_eq!(diags[0].code, "E0308");
        assert_eq!(diags[1].code, "W1234");
    }

    #[test]
    fn test_new_aggregator_is_empty() {
        let aggregator = DiagnosticAggregator::new();
        assert_eq!(aggregator.diagnostics().len(), 0);
        assert_eq!(aggregator.error_count(), 0);
        assert_eq!(aggregator.warning_count(), 0);
        assert_eq!(aggregator.by_code().len(), 0);
    }

    #[test]
    fn test_default_aggregator_is_empty() {
        let aggregator = DiagnosticAggregator::default();
        assert_eq!(aggregator.diagnostics().len(), 0);
        assert_eq!(aggregator.error_count(), 0);
        assert_eq!(aggregator.warning_count(), 0);
    }

    #[test]
    fn test_all_diagnostic_tools() {
        let mut aggregator = DiagnosticAggregator::new();

        aggregator.add(create_diagnostic("src/a.rs", 1, 1, "C1", "msg", DiagnosticTool::Biome, DiagnosticLevel::Error));
        aggregator.add(create_diagnostic("src/b.rs", 2, 2, "C2", "msg", DiagnosticTool::Clippy, DiagnosticLevel::Error));
        aggregator.add(create_diagnostic("src/c.rs", 3, 3, "C3", "msg", DiagnosticTool::Tsc, DiagnosticLevel::Error));
        aggregator.add(create_diagnostic("src/d.rs", 4, 4, "C4", "msg", DiagnosticTool::TsPlugin, DiagnosticLevel::Error));
        aggregator.add(create_diagnostic("src/e.rs", 5, 5, "C5", "msg", DiagnosticTool::SvelteCheck, DiagnosticLevel::Error));
        aggregator.add(create_diagnostic("src/f.rs", 6, 6, "C6", "msg", DiagnosticTool::Macroforge, DiagnosticLevel::Error));

        assert_eq!(aggregator.diagnostics().len(), 6);
        assert_eq!(aggregator.diagnostics()[0].tool, DiagnosticTool::Biome);
        assert_eq!(aggregator.diagnostics()[1].tool, DiagnosticTool::Clippy);
        assert_eq!(aggregator.diagnostics()[2].tool, DiagnosticTool::Tsc);
        assert_eq!(aggregator.diagnostics()[3].tool, DiagnosticTool::TsPlugin);
        assert_eq!(aggregator.diagnostics()[4].tool, DiagnosticTool::SvelteCheck);
        assert_eq!(aggregator.diagnostics()[5].tool, DiagnosticTool::Macroforge);
    }

    #[test]
    fn test_all_diagnostic_levels() {
        let mut aggregator = DiagnosticAggregator::new();

        aggregator.add(create_diagnostic("src/a.rs", 1, 1, "C1", "msg", DiagnosticTool::Clippy, DiagnosticLevel::Error));
        aggregator.add(create_diagnostic("src/b.rs", 2, 2, "C2", "msg", DiagnosticTool::Clippy, DiagnosticLevel::Warning));
        aggregator.add(create_diagnostic("src/c.rs", 3, 3, "C3", "msg", DiagnosticTool::Clippy, DiagnosticLevel::Info));

        assert_eq!(aggregator.diagnostics().len(), 3);
        assert_eq!(aggregator.error_count(), 1);
        assert_eq!(aggregator.warning_count(), 1);
        // Note: there's no info_count method, but we verify it's stored
        assert!(matches!(aggregator.diagnostics()[2].level, DiagnosticLevel::Info));
    }

    #[test]
    fn test_complex_deduplication_scenario() {
        let mut aggregator = DiagnosticAggregator::new();

        // Add same diagnostic from different sources at different times
        aggregator.add(create_error("src/main.rs", 42, 10, "E0308", "mismatched types"));
        aggregator.add(create_warning("src/lib.rs", 100, 5, "W1234", "unused variable"));
        aggregator.add(create_error("src/main.rs", 42, 10, "E0308", "mismatched types")); // Duplicate
        aggregator.add(create_info("src/util.rs", 15, 1, "I5678", "consider using as_ref"));
        aggregator.add(create_warning("src/lib.rs", 100, 5, "W1234", "unused variable")); // Duplicate
        aggregator.add(create_error("src/main.rs", 43, 10, "E0308", "mismatched types")); // Different line, not duplicate

        assert_eq!(aggregator.diagnostics().len(), 4);
        assert_eq!(aggregator.error_count(), 2);
        assert_eq!(aggregator.warning_count(), 1);
    }

    #[test]
    fn test_by_code_ordering_is_deterministic() {
        let mut aggregator = DiagnosticAggregator::new();

        aggregator.add(create_error("src/main.rs", 1, 1, "E0308", "error 1"));
        aggregator.add(create_error("src/main.rs", 2, 1, "E0308", "error 2"));
        aggregator.add(create_error("src/main.rs", 3, 1, "E0308", "error 3"));

        let grouped = aggregator.by_code();
        let e0308_diags = grouped.get("E0308").unwrap();

        // Diagnostics should be in the order they were added
        assert_eq!(e0308_diags[0].line, 1);
        assert_eq!(e0308_diags[1].line, 2);
        assert_eq!(e0308_diags[2].line, 3);
    }
}
