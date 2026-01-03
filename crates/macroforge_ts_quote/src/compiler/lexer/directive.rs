use super::errors::LexResult;
use super::*;

impl Lexer {
    /// Lexes inside a directive `{$...}`.
    pub(super) fn lex_directive(&mut self) -> LexResult<SyntaxKind> {
        // Skip whitespace
        if let Some(c) = self.peek()
            && c.is_whitespace()
        {
            self.consume_while(|c| c.is_whitespace());
            return Ok(SyntaxKind::Whitespace);
        }

        // Check for closing brace
        if self.remaining().starts_with("}") {
            self.advance(1);
            self.pop_mode();
            return Ok(SyntaxKind::RBrace);
        }

        // Check for directive keywords
        let keywords = [
            ("let", SyntaxKind::LetKw),
            ("mut", SyntaxKind::MutKw),
            ("do", SyntaxKind::DoKw),
            ("typescript", SyntaxKind::TypeScriptKw),
        ];

        for (kw, kind) in keywords {
            if self.remaining().starts_with(kw) {
                let next_char = self.remaining().chars().nth(kw.len());
                if next_char
                    .map(|c| !c.is_alphanumeric() && c != '_')
                    .unwrap_or(true)
                {
                    self.advance(kw.len());
                    return Ok(kind);
                }
            }
        }

        // Everything else is Rust tokens
        self.consume_rust_tokens();
        Ok(SyntaxKind::RustTokens)
    }
}
