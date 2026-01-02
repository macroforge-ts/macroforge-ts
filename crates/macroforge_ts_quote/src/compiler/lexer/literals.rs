use super::errors::LexResult;
use super::*;

impl Lexer {
    /// Lexes inside a JSDoc comment `/** ... */`.
    /// Collects content as Text until `*/` is encountered.
    pub(super) fn lex_jsdoc(&mut self) -> SyntaxKind {
        let remaining = self.remaining();

        // Check for closing */
        if remaining.starts_with("*/") {
            self.advance(2);
            self.pop_mode();
            return SyntaxKind::JsDocClose;
        }

        // Consume text until */
        let text_start = self.pos;
        while self.pos < self.input.len() {
            let r = self.remaining();
            if r.starts_with("*/") {
                break;
            }
            if let Some(c) = self.peek() {
                self.advance(c.len_utf8());
            }
        }

        if self.pos > text_start {
            SyntaxKind::Text
        } else {
            // EOF reached without closing - return what we have
            SyntaxKind::Text
        }
    }

    /// Lexes inside a string literal.
    pub(super) fn lex_string_literal(&mut self) -> LexResult<SyntaxKind> {
        let start_pos = self.pos;
        let remaining = self.remaining();

        // Check for closing quote
        if remaining.starts_with("\"") {
            self.advance(1);
            self.pop_mode();
            return Ok(SyntaxKind::DoubleQuote);
        }

        // Check for interpolation
        if remaining.starts_with("@{") {
            self.advance(2);
            self.push_mode(LexerMode::Interpolation);
            self.brace_depth = 1;
            return Ok(SyntaxKind::At);
        }

        // Check for escape
        if remaining.starts_with("\\") && remaining.len() >= 2 {
            self.advance(2);
            return Ok(SyntaxKind::Text);
        }

        // Check for unterminated escape at EOF
        if remaining.starts_with("\\") && remaining.len() < 2 {
            let sequence = remaining.chars().nth(1).map_or(String::new(), |c| c.to_string());
            return Err(LexError::invalid_escape(self.pos, &sequence));
        }

        // Consume text until @{, ", or escape
        let text_start = self.pos;
        while self.pos < self.input.len() {
            let r = self.remaining();
            if r.starts_with("\"") || r.starts_with("@{") || r.starts_with("\\") {
                break;
            }
            if let Some(c) = self.peek() {
                self.advance(c.len_utf8());
            }
        }

        if self.pos > text_start {
            Ok(SyntaxKind::Text)
        } else if self.pos >= self.input.len() {
            // Reached EOF without finding closing quote
            Err(LexError::unterminated_string(self.pos, start_pos, '"'))
        } else {
            Err(LexError::unterminated_string(self.pos, start_pos, '"'))
        }
    }

    /// Lexes inside a template literal.
    pub(super) fn lex_template_literal(&mut self) -> LexResult<SyntaxKind> {
        let start_pos = self.pos;
        let remaining = self.remaining();

        // Check for closing backtick
        if remaining.starts_with("`") {
            self.advance(1);
            self.pop_mode();
            return Ok(SyntaxKind::Backtick);
        }

        // Check for ${...} expression - JS template interpolation (tokenized normally)
        if remaining.starts_with("${") {
            self.advance(2);
            self.push_mode(LexerMode::TemplateExpr);
            self.brace_depth = 1;
            return Ok(SyntaxKind::DollarBrace);
        }

        // Check for @{...} interpolation
        if remaining.starts_with("@{") {
            self.advance(2);
            self.push_mode(LexerMode::Interpolation);
            self.brace_depth = 1;
            return Ok(SyntaxKind::At);
        }

        // Check for escape
        if remaining.starts_with("\\") && remaining.len() >= 2 {
            self.advance(2);
            return Ok(SyntaxKind::Text);
        }

        // Check for unterminated escape at EOF
        if remaining.starts_with("\\") && remaining.len() < 2 {
            let sequence = remaining.chars().nth(1).map_or(String::new(), |c| c.to_string());
            return Err(LexError::invalid_escape(self.pos, &sequence));
        }

        // Consume text until ${, @{, `, or escape
        let text_start = self.pos;
        while self.pos < self.input.len() {
            let r = self.remaining();
            if r.starts_with("`") || r.starts_with("${") || r.starts_with("@{") || r.starts_with("\\") {
                break;
            }
            if let Some(c) = self.peek() {
                self.advance(c.len_utf8());
            }
        }

        if self.pos > text_start {
            Ok(SyntaxKind::Text)
        } else if self.pos >= self.input.len() {
            // Reached EOF without finding closing backtick
            Err(LexError::unterminated_template(self.pos, start_pos))
        } else {
            Err(LexError::unterminated_template(self.pos, start_pos))
        }
    }
}
