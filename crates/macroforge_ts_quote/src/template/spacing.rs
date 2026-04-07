//! Spacing utilities for TypeScript output.
//!
//! Provides span-based whitespace reconstruction to preserve user formatting.

/// Position in source (line, column), both 1-indexed.
#[derive(Clone, Copy, Debug, Default)]
pub struct Pos {
    pub line: usize,
    pub col: usize,
}

impl Pos {
    pub fn from_span_start(span: proc_macro2::Span) -> Self {
        let start = span.start();
        Self {
            line: start.line,
            col: start.column,
        }
    }

    pub fn from_span_end(span: proc_macro2::Span) -> Self {
        let end = span.end();
        Self {
            line: end.line,
            col: end.column,
        }
    }
}

/// Calculate whitespace needed between two positions.
/// Returns a string with newlines and/or spaces to bridge the gap.
pub fn spacing_between(prev_end: Pos, curr_start: Pos) -> String {
    if curr_start.line > prev_end.line {
        // Different lines: emit newlines + indentation
        let newlines = curr_start.line - prev_end.line;
        let indent = curr_start.col;
        format!("{}{}", "\n".repeat(newlines), " ".repeat(indent))
    } else if curr_start.col > prev_end.col {
        // Same line, gap between tokens
        " ".repeat(curr_start.col - prev_end.col)
    } else {
        // No gap (or overlapping spans - shouldn't happen)
        String::new()
    }
}
