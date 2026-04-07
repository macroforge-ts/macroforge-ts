use super::errors::{LexError, LexResult};
use super::*;

impl Lexer {
    /// Lexes inside an interpolation `@{...}`.
    /// Collects raw Rust tokens without parsing them as TypeScript.
    pub(super) fn lex_interpolation(&mut self) -> LexResult<SyntaxKind> {
        let start_pos = self.pos;

        // Track brace depth to find the matching close brace
        while let Some(c) = self.peek() {
            match c {
                '{' => {
                    self.brace_depth += 1;
                    self.advance(1);
                }
                '}' => {
                    self.brace_depth -= 1;
                    if self.brace_depth == 0 {
                        self.advance(1);
                        self.pop_mode();
                        return Ok(SyntaxKind::RBrace);
                    }
                    self.advance(1);
                }
                _ => {
                    self.advance(c.len_utf8());
                }
            }
        }

        // If we get here, we didn't find the closing brace
        Err(LexError::unterminated_interpolation(
            self.pos,
            start_pos,
            self.brace_depth,
        ))
    }

    /// Lexes inside a JS template expression `${...}`.
    /// Tokenizes the content as normal TypeScript, tracking brace depth.
    pub(super) fn lex_template_expr(&mut self) -> LexResult<SyntaxKind> {
        let _start_pos = self.pos;
        let remaining = self.remaining();

        // Check for closing brace at current depth
        if remaining.starts_with("}") {
            self.brace_depth -= 1;
            if self.brace_depth == 0 {
                self.advance(1);
                self.pop_mode();
                return Ok(SyntaxKind::RBrace);
            }
            // Not the final close, return as regular RBrace
            self.advance(1);
            return Ok(SyntaxKind::RBrace);
        }

        // Check for opening brace - increases depth
        if remaining.starts_with("{") {
            self.brace_depth += 1;
            self.advance(1);
            return Ok(SyntaxKind::LBrace);
        }

        // Check for @{...} placeholder inside the template expression
        if remaining.starts_with("@{") {
            self.advance(2);
            // push_mode saves our current brace_depth before pushing Interpolation
            self.push_mode(LexerMode::Interpolation);
            self.brace_depth = 1;
            return Ok(SyntaxKind::At);
        }

        // Nested template literal inside ${...}
        if remaining.starts_with("`") {
            self.advance(1);
            self.push_mode(LexerMode::TemplateLiteral);
            return Ok(SyntaxKind::Backtick);
        }

        // Check for unexpected EOF
        if remaining.is_empty() {
            return Err(LexError::unexpected_eof(self.pos, "interpolation"));
        }

        // Otherwise tokenize as normal TypeScript
        Ok(self.lex_normal())
    }
}
