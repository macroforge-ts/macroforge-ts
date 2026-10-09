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

/// Whitespace bridging two positions: newlines and indentation measured from
/// `base_col`, the template's own left edge, or the spaces between two tokens
/// on one line. Blank lines collapse to one.
pub fn spacing_between(prev_end: Pos, curr_start: Pos, base_col: usize) -> String {
    if curr_start.line > prev_end.line {
        let newlines = (curr_start.line - prev_end.line).min(2);
        let indent = curr_start.col.saturating_sub(base_col);
        format!("{}{}", "\n".repeat(newlines), " ".repeat(indent))
    } else {
        same_line_gap(prev_end, curr_start)
    }
}

/// The spaces between two positions on one line, or nothing when the second
/// starts on a later line.
pub fn same_line_gap(prev_end: Pos, curr_start: Pos) -> String {
    if curr_start.line == prev_end.line && curr_start.col > prev_end.col {
        " ".repeat(curr_start.col - prev_end.col)
    } else {
        String::new()
    }
}
