use super::*;

// =========================================================================
// Ident blocks and strings
// =========================================================================

impl Parser {
    pub(super) fn parse_ident_block(&mut self) -> Option<IrNode> {
        // Consume {|
        self.consume()?;

        let mut parts = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::PipeClose) {
            if self.at(SyntaxKind::At) {
                if let Some(node) = self.parse_interpolation() {
                    parts.push(node);
                }
            } else if let Some(token) = self.consume() {
                parts.push(IrNode::Raw(token.text));
            }
        }

        self.expect(SyntaxKind::PipeClose);

        Some(IrNode::IdentBlock {
            parts: Self::merge_adjacent_text(parts),
        })
    }

    pub(super) fn parse_string_literal(&mut self) -> Option<IrNode> {
        // Consume opening "
        self.consume()?;

        let mut parts = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::DoubleQuote) {
            if self.at(SyntaxKind::At) {
                if let Some(node) = self.parse_interpolation() {
                    parts.push(node);
                }
            } else if let Some(token) = self.consume() {
                parts.push(IrNode::Raw(token.text));
            }
        }

        self.expect(SyntaxKind::DoubleQuote);

        Some(IrNode::StringInterp {
            quote: '"',
            parts: Self::merge_adjacent_text(parts),
        })
    }

    pub(super) fn parse_template_literal(&mut self) -> Option<IrNode> {
        // Consume opening `
        self.consume()?;

        let mut parts = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::Backtick) {
            if self.at(SyntaxKind::At) {
                if let Some(node) = self.parse_interpolation() {
                    parts.push(node);
                }
            } else if let Some(token) = self.consume() {
                parts.push(IrNode::Raw(token.text));
            }
        }

        self.expect(SyntaxKind::Backtick);

        Some(IrNode::StringInterp {
            quote: '`',
            parts: Self::merge_adjacent_text(parts),
        })
    }
}
