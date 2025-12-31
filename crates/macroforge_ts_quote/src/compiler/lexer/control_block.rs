use super::*;

impl Lexer {
    /// Lexes inside a control block `{#...}`, `{/...}`, or `{:...}`.
    pub(super) fn lex_control_block(&mut self) -> SyntaxKind {
        let remaining = self.remaining();

        // Skip whitespace
        if let Some(c) = self.peek()
            && c.is_whitespace()
        {
            self.consume_while(|c| c.is_whitespace());
            return SyntaxKind::Whitespace;
        }

        // Check for closing brace
        if remaining.starts_with("}") {
            self.advance(1);
            self.pop_mode();
            return SyntaxKind::RBrace;
        }

        // Check for control keywords
        let keywords = [
            ("if", SyntaxKind::IfKw),
            ("else", SyntaxKind::ElseKw),
            ("for", SyntaxKind::ForKw),
            ("while", SyntaxKind::WhileKw),
            ("match", SyntaxKind::MatchKw),
            ("case", SyntaxKind::CaseKw),
            ("let", SyntaxKind::LetKw),
            ("in", SyntaxKind::InKw),
        ];

        for (kw, kind) in keywords {
            if remaining.starts_with(kw) {
                let next_char = remaining.chars().nth(kw.len());
                if next_char
                    .map(|c| !c.is_alphanumeric() && c != '_')
                    .unwrap_or(true)
                {
                    self.advance(kw.len());
                    return kind;
                }
            }
        }

        // Tokenize granularly so we can detect keywords like "in"
        // Only consume one "unit" at a time (identifier, operator, etc.)
        if let Some(c) = self.peek() {
            if c.is_alphabetic() || c == '_' {
                // Identifier - consume just this one identifier
                self.consume_while(|c| c.is_alphanumeric() || c == '_');
                SyntaxKind::Ident
            } else {
                // Single character operators/punctuation
                match c {
                    '(' | ')' | '[' | ']' | '{' | ',' | '.' | ':' | ';' | '+' | '-' | '*' | '/'
                    | '%' | '&' | '|' | '^' | '!' | '=' | '<' | '>' | '?' | '@' | '#' | '$' | '~' => {
                        self.advance(1);
                        SyntaxKind::Text
                    }
                    '"' => {
                        // String literal - consume the whole thing
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
                        SyntaxKind::Text
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
                        SyntaxKind::Text
                    }
                    _ => {
                        // Numbers or other characters
                        if c.is_numeric() {
                            self.consume_while(|c| c.is_alphanumeric() || c == '_' || c == '.');
                        } else {
                            self.advance(c.len_utf8());
                        }
                        SyntaxKind::Text
                    }
                }
            }
        } else {
            SyntaxKind::Error
        }
    }
}
