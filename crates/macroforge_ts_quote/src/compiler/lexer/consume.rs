use super::*;

impl Lexer {
    /// Consumes text until a special template character is found.
    pub(super) fn consume_text_until_special(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                '@' | '{' | '}' | '"' | '\'' | '`' | ':' | ';' | '(' | ')' | '[' | ']' | '<'
                | '>' | ',' | '=' | '?' | '.' => break,
                _ if c.is_whitespace() => break,
                _ if c.is_alphabetic() || c == '_' => break,
                _ => self.advance(c.len_utf8()),
            }
        }
    }

    /// Consumes Rust tokens until a closing brace at depth 0.
    pub(super) fn consume_rust_tokens(&mut self) {
        let mut depth = 0;
        while let Some(c) = self.peek() {
            match c {
                '{' => {
                    depth += 1;
                    self.advance(1);
                }
                '}' => {
                    if depth == 0 {
                        // Don't consume the closing brace
                        break;
                    }
                    depth -= 1;
                    self.advance(1);
                }
                '"' => {
                    // Skip string literals
                    self.advance(1);
                    while let Some(c) = self.peek() {
                        if c == '\\' {
                            self.advance(2);
                        } else if c == '"' {
                            self.advance(1);
                            break;
                        } else {
                            self.advance(c.len_utf8());
                        }
                    }
                }
                '\'' => {
                    // Skip char literals
                    self.advance(1);
                    while let Some(c) = self.peek() {
                        if c == '\\' {
                            self.advance(2);
                        } else if c == '\'' {
                            self.advance(1);
                            break;
                        } else {
                            self.advance(c.len_utf8());
                        }
                    }
                }
                _ => {
                    self.advance(c.len_utf8());
                }
            }
        }
    }

    /// Consumes characters while the predicate is true.
    pub(super) fn consume_while<F: Fn(char) -> bool>(&mut self, pred: F) {
        while let Some(c) = self.peek() {
            if pred(c) {
                self.advance(c.len_utf8());
            } else {
                break;
            }
        }
    }
}
