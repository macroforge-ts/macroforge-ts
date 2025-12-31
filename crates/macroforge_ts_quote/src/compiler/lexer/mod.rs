//! Lexer for the template language.
//!
//! The lexer tokenizes template input into a stream of tokens.
//! It handles the hybrid nature of templates: TypeScript code with embedded
//! control flow and interpolations.

mod consume;
mod control_block;
mod directive;
mod ident;
mod interpolation;
mod keyword;
mod literals;
mod normalize;
#[cfg(test)]
mod tests;

use normalize::normalize_template;

use super::syntax::SyntaxKind;

/// A token produced by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The kind of token.
    pub kind: SyntaxKind,
    /// The text of the token.
    pub text: String,
    /// The byte offset where this token starts.
    pub start: usize,
}

/// Lexer state for context-sensitive tokenization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LexerMode {
    /// Normal template mode - looking for TypeScript and template constructs
    Normal,
    /// Inside a control block `{#...}`, `{/...}`, `{:...}`
    ControlBlock,
    /// Inside a directive `{$...}`
    Directive,
    /// Inside an interpolation `@{...}`
    Interpolation,
    /// Inside an ident block `...`
    IdentBlock,
    /// Inside a string literal
    StringLiteral,
    /// Inside a template literal (backticks)
    TemplateLiteral,
}

/// The lexer for template input.
pub struct Lexer {
    /// The normalized input text.
    input: String,
    /// Current byte position in the input.
    pos: usize,
    /// Stack of lexer modes for nested contexts.
    mode_stack: Vec<LexerMode>,
    /// Brace depth for tracking nested braces in interpolations.
    brace_depth: usize,
}

impl Lexer {
    /// Creates a new lexer for the given input.
    /// The input is normalized to collapse Rust tokenizer spacing.
    pub fn new(input: &str) -> Self {
        Self {
            input: normalize_template(input),
            pos: 0,
            mode_stack: vec![LexerMode::Normal],
            brace_depth: 0,
        }
    }

    /// Returns the current lexer mode.
    fn mode(&self) -> LexerMode {
        *self.mode_stack.last().unwrap_or(&LexerMode::Normal)
    }

    /// Pushes a new lexer mode onto the stack.
    fn push_mode(&mut self, mode: LexerMode) {
        self.mode_stack.push(mode);
    }

    /// Pops the current lexer mode from the stack.
    fn pop_mode(&mut self) {
        if self.mode_stack.len() > 1 {
            self.mode_stack.pop();
        }
    }

    /// Returns the remaining input from the current position.
    fn remaining(&self) -> &str {
        &self.input[self.pos..]
    }

    /// Peeks at the next character without consuming it.
    fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    /// Advances the position by n bytes.
    fn advance(&mut self, n: usize) {
        self.pos += n;
    }

    /// Tokenizes the entire input.
    pub fn tokenize(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.pos < self.input.len() {
            if let Some(token) = self.next_token() {
                tokens.push(token);
            }
        }
        tokens
    }

    /// Returns the next token, or None if at EOF.
    fn next_token(&mut self) -> Option<Token> {
        if self.pos >= self.input.len() {
            return None;
        }

        let start = self.pos;

        // Check for Rust doc attr first (needs special text handling)
        if let Some((len, content)) = self.try_match_rust_doc_attr() {
            self.advance(len);
            return Some(Token {
                kind: SyntaxKind::RustDocAttr,
                text: content,
                start,
            });
        }

        let kind = match self.mode() {
            LexerMode::Normal => self.lex_normal(),
            LexerMode::ControlBlock => self.lex_control_block(),
            LexerMode::Directive => self.lex_directive(),
            LexerMode::Interpolation => self.lex_interpolation(),
            LexerMode::IdentBlock => self.lex_ident_block(),
            LexerMode::StringLiteral => self.lex_string_literal(),
            LexerMode::TemplateLiteral => self.lex_template_literal(),
        };

        let text = self.input[start..self.pos].to_string();
        Some(Token { kind, text, start })
    }

    /// Lexes in normal mode (TypeScript + template constructs).
    fn lex_normal(&mut self) -> SyntaxKind {
        let remaining = self.remaining();

        // Check for template constructs first
        if remaining.starts_with("@@{") {
            self.advance(3);
            return SyntaxKind::AtAt;
        }

        if remaining.starts_with("@{") {
            self.advance(2);
            self.push_mode(LexerMode::Interpolation);
            self.brace_depth = 1;
            return SyntaxKind::At;
        }

        if remaining.starts_with("{#") {
            self.advance(2);
            self.push_mode(LexerMode::ControlBlock);
            return SyntaxKind::HashOpen;
        }

        if remaining.starts_with("{/") {
            self.advance(2);
            self.push_mode(LexerMode::ControlBlock);
            return SyntaxKind::SlashOpen;
        }

        if remaining.starts_with("{:") {
            self.advance(2);
            self.push_mode(LexerMode::ControlBlock);
            return SyntaxKind::ColonOpen;
        }

        if remaining.starts_with("{$") {
            self.advance(2);
            self.push_mode(LexerMode::Directive);
            return SyntaxKind::DollarOpen;
        }

        if remaining.starts_with("{|") {
            self.advance(2);
            self.push_mode(LexerMode::IdentBlock);
            return SyntaxKind::PipeOpen;
        }

        if remaining.starts_with("{>>") {
            self.advance(3);
            return SyntaxKind::CommentBlockOpen;
        }

        if remaining.starts_with("{>") {
            self.advance(2);
            return SyntaxKind::CommentLineOpen;
        }

        if remaining.starts_with("<<}") {
            self.advance(3);
            return SyntaxKind::CommentBlockClose;
        }

        if remaining.starts_with("<}") {
            self.advance(2);
            return SyntaxKind::CommentLineClose;
        }

        // Check for doc comments
        if remaining.starts_with("///") {
            self.advance(3);
            return SyntaxKind::DocCommentPrefix;
        }

        if remaining.starts_with("/**") {
            self.advance(3);
            return SyntaxKind::JsDocOpen;
        }

        if remaining.starts_with("*/") {
            self.advance(2);
            return SyntaxKind::JsDocClose;
        }

        // Check for multi-character operators first (before single-char matching)
        if remaining.starts_with("...") {
            self.advance(3);
            return SyntaxKind::DotDotDot;
        }

        if remaining.starts_with("?.") {
            self.advance(2);
            return SyntaxKind::QuestionDot;
        }

        if remaining.starts_with("++") {
            self.advance(2);
            return SyntaxKind::PlusPlus;
        }

        if remaining.starts_with("--") {
            self.advance(2);
            return SyntaxKind::MinusMinus;
        }

        // Check for TypeScript keywords (for type position detection)
        if let Some(kw) = self.try_lex_ts_keyword() {
            kw
        } else {
            // Single character tokens
            let c = self.peek().unwrap();
            match c {
                '{' => {
                    self.advance(1);
                    SyntaxKind::LBrace
                }
                '}' => {
                    self.advance(1);
                    SyntaxKind::RBrace
                }
                '(' => {
                    self.advance(1);
                    SyntaxKind::LParen
                }
                ')' => {
                    self.advance(1);
                    SyntaxKind::RParen
                }
                '[' => {
                    self.advance(1);
                    SyntaxKind::LBracket
                }
                ']' => {
                    self.advance(1);
                    SyntaxKind::RBracket
                }
                '<' => {
                    self.advance(1);
                    SyntaxKind::Lt
                }
                '>' => {
                    self.advance(1);
                    SyntaxKind::Gt
                }
                ':' => {
                    self.advance(1);
                    SyntaxKind::Colon
                }
                ';' => {
                    self.advance(1);
                    SyntaxKind::Semicolon
                }
                ',' => {
                    self.advance(1);
                    SyntaxKind::Comma
                }
                '=' => {
                    self.advance(1);
                    SyntaxKind::Eq
                }
                '?' => {
                    self.advance(1);
                    SyntaxKind::Question
                }
                '.' => {
                    self.advance(1);
                    SyntaxKind::Dot
                }
                '*' => {
                    self.advance(1);
                    SyntaxKind::Star
                }
                '#' => {
                    self.advance(1);
                    SyntaxKind::Hash
                }
                '!' => {
                    self.advance(1);
                    SyntaxKind::Exclaim
                }
                '&' => {
                    self.advance(1);
                    SyntaxKind::Ampersand
                }
                '"' => {
                    self.advance(1);
                    self.push_mode(LexerMode::StringLiteral);
                    SyntaxKind::DoubleQuote
                }
                '\'' => {
                    self.advance(1);
                    SyntaxKind::SingleQuote
                }
                '`' => {
                    self.advance(1);
                    self.push_mode(LexerMode::TemplateLiteral);
                    SyntaxKind::Backtick
                }
                '@' => {
                    // Check what follows @ (note: @{ is handled earlier in this function)
                    // Look at the character after @
                    let after_at = self.input.get(self.pos + 1..).and_then(|s| s.chars().next());
                    match after_at {
                        Some(c) if c.is_alphabetic() || c == '_' => {
                            // `@identifier` - TypeScript decorator
                            self.advance(1);
                            SyntaxKind::DecoratorAt
                        }
                        _ => {
                            // Single @ not followed by identifier - just text
                            self.advance(1);
                            SyntaxKind::Text
                        }
                    }
                }
                _ if c.is_whitespace() => {
                    self.consume_while(|c| c.is_whitespace());
                    SyntaxKind::Whitespace
                }
                _ if c.is_alphabetic() || c == '_' => {
                    self.consume_while(|c| c.is_alphanumeric() || c == '_');
                    SyntaxKind::Ident
                }
                _ => {
                    // Consume as text until we hit a special character
                    self.consume_text_until_special();
                    SyntaxKind::Text
                }
            }
        }
    }

    /// Tries to match a Rust doc attribute pattern: `# [ doc = "..." ]`
    /// Returns (total_length, content) if matched, None otherwise.
    fn try_match_rust_doc_attr(&self) -> Option<(usize, String)> {
        let remaining = self.remaining();
        let bytes = remaining.as_bytes();
        let mut i = 0;

        // Match `#`
        if bytes.get(i) != Some(&b'#') {
            return None;
        }
        i += 1;

        // Skip whitespace
        while bytes
            .get(i)
            .map(|b| b.is_ascii_whitespace())
            .unwrap_or(false)
        {
            i += 1;
        }

        // Match `[`
        if bytes.get(i) != Some(&b'[') {
            return None;
        }
        i += 1;

        // Skip whitespace
        while bytes
            .get(i)
            .map(|b| b.is_ascii_whitespace())
            .unwrap_or(false)
        {
            i += 1;
        }

        // Match `doc`
        if !remaining[i..].starts_with("doc") {
            return None;
        }
        i += 3;

        // Skip whitespace
        while bytes
            .get(i)
            .map(|b| b.is_ascii_whitespace())
            .unwrap_or(false)
        {
            i += 1;
        }

        // Match `=`
        if bytes.get(i) != Some(&b'=') {
            return None;
        }
        i += 1;

        // Skip whitespace
        while bytes
            .get(i)
            .map(|b| b.is_ascii_whitespace())
            .unwrap_or(false)
        {
            i += 1;
        }

        // Match `"` and consume string content
        if bytes.get(i) != Some(&b'"') {
            return None;
        }
        i += 1;
        let content_start = i;

        // Consume until closing `"`
        while i < bytes.len() {
            if bytes[i] == b'\\' && i + 1 < bytes.len() {
                i += 2; // Skip escape sequence
            } else if bytes[i] == b'"' {
                break;
            } else {
                i += 1;
            }
        }
        let content_end = i;
        let content = remaining[content_start..content_end].to_string();

        // Skip closing quote
        if bytes.get(i) == Some(&b'"') {
            i += 1;
        }

        // Skip whitespace
        while bytes
            .get(i)
            .map(|b| b.is_ascii_whitespace())
            .unwrap_or(false)
        {
            i += 1;
        }

        // Match `]`
        if bytes.get(i) != Some(&b']') {
            return None;
        }
        i += 1;

        Some((i, content))
    }
}
