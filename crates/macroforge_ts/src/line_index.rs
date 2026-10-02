//! Line and column positions of byte offsets in a source.

/// Byte offsets of every line break in a source, so each position is a
/// binary search rather than a scan from the file start.
pub struct LineIndex {
    newlines: Vec<usize>,
}

impl LineIndex {
    pub fn new(source: &str) -> Self {
        Self {
            newlines: source.match_indices('\n').map(|(at, _)| at).collect(),
        }
    }

    /// The 1-based `(line, column)` of a byte offset into `source`, the
    /// column counted in characters.
    pub fn line_col(&self, source: &str, offset: u32) -> (u32, u32) {
        let offset = (offset as usize).min(source.len());
        let breaks_before = self.newlines.partition_point(|&at| at < offset);
        let line_start = match breaks_before {
            0 => 0,
            count => self.newlines[count - 1] + 1,
        };
        let column = source[line_start..]
            .char_indices()
            .take_while(|(at, _)| line_start + at < offset)
            .count();
        (breaks_before as u32 + 1, column as u32 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::LineIndex;

    #[test]
    fn positions_count_lines_and_characters() {
        let source = "const a = 1;\nconst é = 'ü';\n\n/** @buildtime */";
        let lines = LineIndex::new(source);
        assert_eq!(lines.line_col(source, 0), (1, 1));
        assert_eq!(lines.line_col(source, 12), (1, 13));
        assert_eq!(lines.line_col(source, 13), (2, 1));
        // `é` is two bytes and one column.
        let after_e = source.find(" = 'ü'").expect("fixture holds the binding") as u32;
        assert_eq!(lines.line_col(source, after_e), (2, 8));
        let doc = source.find("/**").expect("fixture holds the doc") as u32;
        assert_eq!(lines.line_col(source, doc), (4, 1));
        assert_eq!(lines.line_col(source, u32::MAX), (4, 18));
    }

    #[test]
    fn blank_lines_count() {
        let source = "a\n\nb";
        let lines = LineIndex::new(source);
        assert_eq!(lines.line_col(source, 2), (2, 1));
        assert_eq!(lines.line_col(source, 3), (3, 1));
    }
}
