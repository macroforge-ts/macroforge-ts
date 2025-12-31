use super::*;

impl Lexer {
    /// Lexes inside an interpolation `@{...}`.
    pub(super) fn lex_interpolation(&mut self) -> SyntaxKind {
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
                        return SyntaxKind::RBrace;
                    }
                    self.advance(1);
                }
                _ => {
                    self.advance(c.len_utf8());
                }
            }
        }

        // If we get here, we didn't find the closing brace
        SyntaxKind::RustTokens
    }
}
