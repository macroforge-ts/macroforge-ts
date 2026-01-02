use super::*;

// =========================================================================
// Ident blocks
// =========================================================================

impl Parser {
    pub(super) fn parse_ident_block(&mut self) -> ParseResult<IrNode> {
        // Consume {|
        self.consume();

        let mut parts = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::PipeClose) {
            if self.at(SyntaxKind::At) {
                let node = self.parse_interpolation()?;
                parts.push(node);
            } else if let Some(token) = self.consume() {
                parts.push(IrNode::Raw(token.text));
            }
        }

        self.expect(SyntaxKind::PipeClose);

        Ok(IrNode::IdentBlock {
            parts: Self::merge_adjacent_text(parts),
        })
    }

    // parse_string_literal and parse_template_literal are now in expr/primary.rs
    // with proper error handling (returning ParseResult)
}
