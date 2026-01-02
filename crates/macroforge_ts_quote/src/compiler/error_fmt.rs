//! Shared error formatting utilities for parser and codegen errors.
//!
//! Provides rustc-style error formatting with source context.

use super::parser::expr::errors::SourceLocation;

/// Configuration for formatting an error with source context.
pub struct ErrorFormat<'a> {
    /// The error message (e.g., "unexpected token")
    pub message: &'a str,
    /// Source code
    pub source: &'a str,
    /// Byte position in source where error occurred
    pub position: usize,
    /// Filename to display
    pub filename: &'a str,
    /// Line offset to add (for embedded templates)
    pub line_offset: usize,
    /// Annotation text (e.g., "found: X" or "expected: Y")
    pub annotation: Option<String>,
    /// Help text
    pub help: Option<&'a str>,
}

impl<'a> ErrorFormat<'a> {
    /// Creates a new error format configuration.
    pub fn new(message: &'a str, source: &'a str, position: usize) -> Self {
        Self {
            message,
            source,
            position,
            filename: "input",
            line_offset: 0,
            annotation: None,
            help: None,
        }
    }

    /// Sets the filename.
    pub fn filename(mut self, filename: &'a str) -> Self {
        self.filename = filename;
        self
    }

    /// Sets the line offset.
    pub fn line_offset(mut self, offset: usize) -> Self {
        self.line_offset = offset;
        self
    }

    /// Sets the annotation (found/expected text).
    pub fn annotation(mut self, ann: String) -> Self {
        self.annotation = Some(ann);
        self
    }

    /// Sets the help text.
    pub fn help(mut self, help: &'a str) -> Self {
        self.help = Some(help);
        self
    }

    /// Formats the error with source context.
    ///
    /// Output format:
    /// ```text
    /// error: message
    ///  --> file:line:column
    /// `N | source line content        `
    /// `  |     ^ annotation            `
    /// help: suggestion
    /// ```
    pub fn format(&self) -> String {
        let loc = SourceLocation::from_offset(self.source, self.position);
        let absolute_line = loc.line + self.line_offset;

        // Header: error: message
        let mut msg = format!("error: {}\n", self.message);

        // Location: --> file:line:column
        msg.push_str(&format!(
            " --> {}:{}:{}\n",
            self.filename, absolute_line, loc.column
        ));

        // Extract and show the problematic line with caret
        let lines: Vec<&str> = self.source.lines().collect();
        if loc.line > 0 && loc.line <= lines.len() {
            let line_content = lines[loc.line - 1];
            // Expand tabs to spaces for consistent alignment
            let expanded_content = line_content.replace('\t', "    ");

            // Calculate byte offset within the line for visual caret positioning
            let line_start_offset = self.source[..self.position]
                .rfind('\n')
                .map(|pos| pos + 1)
                .unwrap_or(0);
            let byte_offset_in_line = self.position.saturating_sub(line_start_offset);

            // Calculate visual column (accounting for tabs) from byte offset
            let visual_column: usize = line_content
                .char_indices()
                .take_while(|(i, _)| *i < byte_offset_in_line)
                .map(|(_, c)| if c == '\t' { 4 } else { 1 })
                .sum();

            // Strip leading/trailing whitespace for display and truncation calculation
            let leading_spaces = expanded_content.len() - expanded_content.trim_start().len();
            let trimmed_content = expanded_content.trim().to_string();

            // Adjust visual column to account for stripped leading whitespace
            let adjusted_column = visual_column.saturating_sub(leading_spaces);

            // Annotation text
            let annotation = self.annotation.as_deref().unwrap_or("");

            // Truncate long lines to show context around the error
            const MAX_LINE_LEN: usize = 80;
            const CONTEXT_CHARS: usize = 30;

            let (display_content, display_caret_col) = if trimmed_content.len() > MAX_LINE_LEN {
                // Find a window around the error position
                let content_chars: Vec<char> = trimmed_content.chars().collect();
                let char_len = content_chars.len();

                // Calculate desired window (may extend beyond content)
                let desired_start = adjusted_column.saturating_sub(CONTEXT_CHARS);
                let desired_end = adjusted_column + CONTEXT_CHARS;

                // Clamp to actual content bounds
                let actual_start = desired_start.min(char_len);
                let actual_end = desired_end.min(char_len);

                // Add ellipsis if we're truncating at either end
                let prefix = if actual_start > 0 { "..." } else { "" };
                let suffix = if actual_end < char_len { "..." } else { "" };

                let snippet: String = content_chars[actual_start..actual_end].iter().collect();
                let new_caret_col = adjusted_column.saturating_sub(actual_start) + prefix.len();

                (format!("{}{}{}", prefix, snippet, suffix), new_caret_col)
            } else {
                (trimmed_content, adjusted_column)
            };

            // Line number width for alignment
            let line_num_width = absolute_line.to_string().len();

            // Source line with line number and pipe
            let source_line = format!(
                "{:>width$} | {}",
                absolute_line,
                display_content,
                width = line_num_width
            );
            // Caret line with pipe (no line number, just spaces)
            let caret_content = format!("{:>col$}^ {}", "", annotation, col = display_caret_col);
            let caret_line = format!("{:>width$} | {}", "", caret_content, width = line_num_width);

            // Pad both lines to equal length for alignment
            let max_len = source_line.len().max(caret_line.len());
            let padded_source = format!("{:<width$}", source_line, width = max_len);
            let padded_caret = format!("{:<width$}", caret_line, width = max_len);

            msg.push_str(&format!("`{}`\n", padded_source));
            msg.push_str(&format!("`{}`\n", padded_caret));
        }

        // Help note if available
        if let Some(help) = self.help {
            msg.push_str(&format!("help: {}\n", help));
        }

        msg
    }
}

/// Helper to build annotation text from found/expected values.
pub fn build_annotation(found: Option<&str>, expected: &[&str]) -> Option<String> {
    if let Some(f) = found {
        Some(format!("found: {}", f))
    } else if !expected.is_empty() {
        Some(format!("expected: {}", expected.join(" or ")))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_format_basic() {
        let formatted = ErrorFormat::new("unexpected token", "let x = ;", 8)
            .filename("test.ts")
            .annotation("expected: expression".to_string())
            .format();

        assert!(formatted.contains("error: unexpected token"));
        assert!(formatted.contains("--> test.ts:1:9"));
        assert!(formatted.contains("let x = ;"));
        assert!(formatted.contains("^"));
        assert!(formatted.contains("expected: expression"));
    }

    #[test]
    fn test_error_format_with_help() {
        let formatted = ErrorFormat::new("missing semicolon", "let x = 1", 9)
            .help("add `;` at end of statement")
            .format();

        assert!(formatted.contains("help: add `;` at end of statement"));
    }

    #[test]
    fn test_error_format_multiline() {
        let source = "function foo() {\n  return;\n}";
        let formatted = ErrorFormat::new("unexpected return", source, 19)
            .filename("test.ts")
            .format();

        assert!(formatted.contains("--> test.ts:2:"));
        assert!(formatted.contains("return;"));
    }

    #[test]
    fn test_build_annotation_found() {
        let ann = build_annotation(Some("identifier"), &[]);
        assert_eq!(ann, Some("found: identifier".to_string()));
    }

    #[test]
    fn test_build_annotation_expected() {
        let ann = build_annotation(None, &["expression", "statement"]);
        assert_eq!(ann, Some("expected: expression or statement".to_string()));
    }

    #[test]
    fn test_build_annotation_none() {
        let ann = build_annotation(None, &[]);
        assert_eq!(ann, None);
    }

    #[test]
    fn test_long_line_truncation_both_ends() {
        // Create a line longer than 80 chars with error in the middle
        let long_line = "let veryLongVariableName = someFunction(anotherLongArgument, yetAnotherArgument, andEvenMoreArguments, finalArgument);";
        // Error at position 50 (somewhere in the middle, so both ends should be truncated)
        let formatted = ErrorFormat::new("test error", long_line, 50)
            .filename("test.ts")
            .format();

        // Should have ellipsis at start (prefix)
        assert!(formatted.contains("`1 | ..."), "should have prefix ellipsis, got:\n{}", formatted);
        // Should have ellipsis at end (suffix)
        let ellipsis_count = formatted.matches("...").count();
        assert!(ellipsis_count >= 2, "should have both prefix and suffix ellipsis (found {}), got:\n{}", ellipsis_count, formatted);
    }

    #[test]
    fn test_long_line_truncation_suffix_only() {
        // Error near the start of a long line - should only have suffix ellipsis
        let long_line = "x = someVeryLongFunctionCallWithManyArguments(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8);";
        let formatted = ErrorFormat::new("test error", long_line, 2)
            .filename("test.ts")
            .format();

        // Should NOT have prefix ellipsis (error is near start)
        assert!(!formatted.contains("`1 | ..."), "should NOT have prefix ellipsis for error near start, got:\n{}", formatted);
        // Should have suffix ellipsis
        assert!(formatted.contains("..."), "should have suffix ellipsis, got:\n{}", formatted);
    }

    #[test]
    fn test_long_line_truncation_prefix_only() {
        // Error near the end of a long line - should only have prefix ellipsis
        let long_line = "let result = someVeryLongFunctionCallWithManyArguments(arg1, arg2, arg3, arg4, arg5, arg6, arg7);";
        let len = long_line.len();
        let formatted = ErrorFormat::new("test error", long_line, len - 5)
            .filename("test.ts")
            .format();

        // Should have prefix ellipsis
        assert!(formatted.contains("..."), "should have prefix ellipsis, got:\n{}", formatted);
        // The line should end with `);` not `...` (no suffix needed)
        assert!(formatted.contains(");"), "line should show the actual end, got:\n{}", formatted);
    }

    #[test]
    fn test_indented_line_strips_whitespace() {
        // Line with lots of leading whitespace - should strip it for display
        let indented_line = "                    let x = error;";
        // Error at "error" which is at byte 28
        let formatted = ErrorFormat::new("test error", indented_line, 28)
            .filename("test.ts")
            .format();

        // Should NOT have leading whitespace in display
        assert!(formatted.contains("| let x"), "should strip leading whitespace, got:\n{}", formatted);
        // Should NOT have ellipsis (line is short after trimming)
        assert!(!formatted.contains("..."), "short line should not be truncated, got:\n{}", formatted);
    }

    #[test]
    fn test_indented_long_line_truncates_correctly() {
        // Long line with leading whitespace - should strip whitespace AND truncate
        let indented_long = "                    let veryLongVariableName = someFunction(anotherLongArgument, yetAnotherArgument, moreArgs);";
        // Error somewhere in the middle of the code
        let formatted = ErrorFormat::new("test error", indented_long, 60)
            .filename("test.ts")
            .format();

        // Should have ellipsis (content after trimming is > 80 chars)
        assert!(formatted.contains("..."), "long content should be truncated, got:\n{}", formatted);
        // Should start with code, not whitespace (after the line number and pipe)
        // The trimmed content starts with "let", and after truncation should show "...ngVariableName" or similar
        assert!(formatted.contains("| ..."), "should show truncated content without leading spaces, got:\n{}", formatted);
    }
}
