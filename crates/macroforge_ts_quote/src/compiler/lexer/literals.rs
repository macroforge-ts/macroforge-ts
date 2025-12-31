use super::*;

impl Lexer {
    /// Lexes inside a string literal.
    pub(super) fn lex_string_literal(&mut self) -> SyntaxKind {
        let remaining = self.remaining();

        // Check for closing quote
        if remaining.starts_with("\"") {
            self.advance(1);
            self.pop_mode();
            return SyntaxKind::DoubleQuote;
        }

        // Check for interpolation
        if remaining.starts_with("@{") {
            self.advance(2);
            self.push_mode(LexerMode::Interpolation);
            self.brace_depth = 1;
            return SyntaxKind::At;
        }

        // Check for escape
        if remaining.starts_with("\\") && remaining.len() >= 2 {
            self.advance(2);
            return SyntaxKind::Text;
        }

        // Consume text until @{, ", or escape
        let start = self.pos;
        while self.pos < self.input.len() {
            let r = self.remaining();
            if r.starts_with("\"") || r.starts_with("@{") || r.starts_with("\\") {
                break;
            }
            if let Some(c) = self.peek() {
                self.advance(c.len_utf8());
            }
        }

        if self.pos > start {
            SyntaxKind::Text
        } else {
            SyntaxKind::Error
        }
    }

    /// Lexes inside a template literal.
    pub(super) fn lex_template_literal(&mut self) -> SyntaxKind {
        let remaining = self.remaining();

        // Check for closing backtick
        if remaining.starts_with("`") {
            self.advance(1);
            self.pop_mode();
            return SyntaxKind::Backtick;
        }

        // Check for ${...} expression
        if remaining.starts_with("${") {
            self.advance(2);
            self.push_mode(LexerMode::Interpolation);
            self.brace_depth = 1;
            return SyntaxKind::At; // Reuse AT for template expressions
        }

        // Check for @{...} interpolation
        if remaining.starts_with("@{") {
            self.advance(2);
            self.push_mode(LexerMode::Interpolation);
            self.brace_depth = 1;
            return SyntaxKind::At;
        }

        // Check for escape
        if remaining.starts_with("\\") && remaining.len() >= 2 {
            self.advance(2);
            return SyntaxKind::Text;
        }

        // Consume text until ${, @{, `, or escape
        let start = self.pos;
        while self.pos < self.input.len() {
            let r = self.remaining();
            if r.starts_with("`") || r.starts_with("${") || r.starts_with("@{") || r.starts_with("\\") {
                break;
            }
            if let Some(c) = self.peek() {
                self.advance(c.len_utf8());
            }
        }

        if self.pos > start {
            SyntaxKind::Text
        } else {
            SyntaxKind::Error
        }
    }
}
