use super::errors::{LexError, LexErrorKind, LexResult};
use super::*;

impl Lexer {
    /// Lexes inside an ident block `{|...|}`
    pub(super) fn lex_ident_block(&mut self) -> LexResult<SyntaxKind> {
        let remaining = self.remaining();

        // Check for closing |}
        if remaining.starts_with("|}") {
            self.advance(2);
            self.pop_mode();
            return Ok(SyntaxKind::PipeClose);
        }

        // Check for interpolation
        if remaining.starts_with("@{") {
            self.advance(2);
            self.push_mode(LexerMode::Interpolation);
            self.brace_depth = 1;
            return Ok(SyntaxKind::At);
        }

        // Consume text until @{ or |}
        let start = self.pos;
        while self.pos < self.input.len() {
            let r = self.remaining();
            if r.starts_with("|}") || r.starts_with("@{") {
                break;
            }
            if let Some(c) = self.peek() {
                self.advance(c.len_utf8());
            }
        }

        if self.pos > start {
            Ok(SyntaxKind::Text)
        } else if self.pos >= self.input.len() {
            // Reached EOF without finding |}
            Err(LexError::new(LexErrorKind::UnterminatedIdentBlock, self.pos)
                .with_context("ident block {|...|}"))
        } else {
            Ok(SyntaxKind::Error)
        }
    }
}
