use super::errors::{LexError, LexResult};
use super::*;

impl Lexer {
    /// Lexes inside a control block after `{#if`, `{#for`, `{:else if`, `{:case`, etc.
    /// The opening keyword is already consumed as part of the token.
    /// This mode handles the condition/expression until the closing `}`.
    pub(super) fn lex_control_block(&mut self) -> LexResult<SyntaxKind> {
        let remaining = self.remaining();

        // Skip whitespace
        if let Some(c) = self.peek()
            && c.is_whitespace()
        {
            self.consume_while(|c| c.is_whitespace());
            return Ok(SyntaxKind::Whitespace);
        }

        // Check for closing brace - ends the control block
        if remaining.starts_with("}") {
            self.advance(1);
            self.pop_mode();
            return Ok(SyntaxKind::RBrace);
        }

        // Check for `let` keyword (used in `{#if let ...}` patterns)
        if remaining.starts_with("let") {
            let next_char = remaining.chars().nth(3);
            if next_char
                .map(|c| !c.is_alphanumeric() && c != '_')
                .unwrap_or(true)
            {
                self.advance(3);
                return Ok(SyntaxKind::LetKw);
            }
        }

        // Check for `in` keyword (used in `{#for x in items}`)
        if remaining.starts_with("in") {
            let next_char = remaining.chars().nth(2);
            if next_char
                .map(|c| !c.is_alphanumeric() && c != '_')
                .unwrap_or(true)
            {
                self.advance(2);
                return Ok(SyntaxKind::InKw);
            }
        }

        // Tokenize granularly for the condition/expression
        if let Some(c) = self.peek() {
            if c.is_alphabetic() || c == '_' {
                // Identifier
                self.consume_while(|c| c.is_alphanumeric() || c == '_');
                Ok(SyntaxKind::Ident)
            } else {
                // Single character operators/punctuation
                match c {
                    '(' | ')' | '[' | ']' | '{' | ',' | '.' | ':' | ';' | '+' | '-' | '*' | '/'
                    | '%' | '&' | '|' | '^' | '!' | '=' | '<' | '>' | '?' | '@' | '#' | '$'
                    | '~' => {
                        self.advance(1);
                        Ok(SyntaxKind::Text)
                    }
                    '"' => {
                        // String literal
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
                        Ok(SyntaxKind::Text)
                    }
                    '\'' => {
                        // Char literal
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
                        Ok(SyntaxKind::Text)
                    }
                    _ => {
                        // Numbers or other characters
                        if c.is_numeric() {
                            self.consume_while(|c| c.is_alphanumeric() || c == '_' || c == '.');
                        } else {
                            self.advance(c.len_utf8());
                        }
                        Ok(SyntaxKind::Text)
                    }
                }
            }
        } else {
            Err(LexError::unexpected_eof(
                self.pos,
                "control block condition",
            ))
        }
    }
}
