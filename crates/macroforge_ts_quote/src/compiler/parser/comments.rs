use super::*;

// =========================================================================
// Comments
// =========================================================================

impl Parser {
    pub(super) fn parse_line_comment(&mut self) -> Option<IrNode> {
        // Consume {>
        self.consume()?;

        let mut text = String::new();
        while !self.at_eof() && !self.at(SyntaxKind::CommentLineClose) {
            if let Some(token) = self.consume() {
                text.push_str(&token.text);
            }
        }

        self.expect(SyntaxKind::CommentLineClose);

        Some(IrNode::LineComment {
            text: text.trim().to_string(),
        })
    }

    pub(super) fn parse_block_comment(&mut self) -> Option<IrNode> {
        // Consume {>>
        self.consume()?;

        let mut text = String::new();
        while !self.at_eof() && !self.at(SyntaxKind::CommentBlockClose) {
            if let Some(token) = self.consume() {
                text.push_str(&token.text);
            }
        }

        self.expect(SyntaxKind::CommentBlockClose);

        Some(IrNode::BlockComment {
            text: text.trim().to_string(),
        })
    }

    pub(super) fn parse_rust_doc_attr(&mut self) -> Option<IrNode> {
        // Token text is already just the doc content (extracted by lexer)
        let token = self.consume()?;
        Some(IrNode::DocComment { text: token.text })
    }

    pub(super) fn parse_doc_comment(&mut self) -> Option<IrNode> {
        let mut text = String::new();

        match self.current_kind() {
            Some(SyntaxKind::DocCommentPrefix) => {
                // /// style
                self.consume();
                // Collect until end of line
                while !self.at_eof() {
                    if let Some(token) = self.current() {
                        if token.text.contains('\n') {
                            break;
                        }
                    }
                    if let Some(token) = self.consume() {
                        text.push_str(&token.text);
                    }
                }
            }
            Some(SyntaxKind::JsDocOpen) => {
                // /** style
                self.consume();
                while !self.at_eof() && !self.at(SyntaxKind::JsDocClose) {
                    if let Some(token) = self.consume() {
                        text.push_str(&token.text);
                    }
                }
                self.expect(SyntaxKind::JsDocClose);
            }
            _ => return None,
        }

        Some(IrNode::DocComment {
            text: text.trim().to_string(),
        })
    }
}
