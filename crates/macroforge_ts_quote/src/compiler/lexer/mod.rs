//! Lexer for the template language.
//!
//! The lexer tokenizes template input into a stream of tokens.
//! It handles the hybrid nature of templates: TypeScript code with embedded
//! control flow and interpolations.

mod consume;
mod control_block;
mod directive;
pub mod errors;
mod ident;
mod interpolation;
mod keyword;
mod literals;
mod normalize;
#[cfg(test)]
mod tests;

use errors::LexResult;
use normalize::normalize_template;

use super::syntax::SyntaxKind;

pub use errors::{LexError, LexErrorKind};

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

impl Token {
    /// Get the end byte offset of this token.
    #[inline]
    pub fn end(&self) -> usize {
        self.start + self.text.len()
    }

    /// Get the IR span for this token.
    #[inline]
    pub fn ir_span(&self) -> super::ir::IrSpan {
        super::ir::IrSpan::new(self.start, self.start + self.text.len())
    }
}

impl super::ir::IntoIrNode for Token {
    #[inline]
    fn text(&self) -> &str {
        &self.text
    }

    #[inline]
    fn ir_span(&self) -> super::ir::IrSpan {
        super::ir::IrSpan::from_pos_len(self.start, self.text.len())
    }
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
    /// Inside an interpolation `@{...}` - collects raw Rust tokens
    Interpolation,
    /// Inside a JS template expression `${...}` - tokenizes normally
    TemplateExpr,
    /// Inside an ident block `...`
    IdentBlock,
    /// Inside a string literal
    StringLiteral,
    /// Inside a template literal (backticks)
    TemplateLiteral,
    /// Inside a JSDoc comment `/** ... */` - collects as text until `*/`
    JsDoc,
}

impl LexerMode {
    /// Returns a human-readable name for this mode.
    fn as_str(&self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::ControlBlock => "ControlBlock",
            Self::Directive => "Directive",
            Self::Interpolation => "Interpolation",
            Self::TemplateExpr => "TemplateExpr",
            Self::IdentBlock => "IdentBlock",
            Self::StringLiteral => "StringLiteral",
            Self::TemplateLiteral => "TemplateLiteral",
            Self::JsDoc => "JsDoc",
        }
    }
}

/// The lexer for template input.
pub struct Lexer {
    /// The normalized input text.
    input: String,
    /// Current byte position in the input.
    pos: usize,
    /// Stack of lexer modes for nested contexts.
    mode_stack: Vec<LexerMode>,
    /// Brace depth for tracking nested braces in current interpolation context.
    brace_depth: usize,
    /// Stack of brace depths to save/restore when entering nested interpolations.
    brace_depth_stack: Vec<usize>,
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
            brace_depth_stack: Vec::new(),
        }
    }

    /// Returns the current lexer mode.
    fn mode(&self) -> LexerMode {
        *self.mode_stack.last().unwrap_or(&LexerMode::Normal)
    }

    /// Pushes a new lexer mode onto the stack, saving brace depth if entering an interpolation context.
    fn push_mode(&mut self, mode: LexerMode) {
        // Save current brace depth when entering interpolation modes
        if matches!(mode, LexerMode::Interpolation | LexerMode::TemplateExpr) {
            self.brace_depth_stack.push(self.brace_depth);
        }
        self.mode_stack.push(mode);
    }

    /// Pops the current lexer mode from the stack, restoring brace depth if leaving an interpolation context.
    fn pop_mode(&mut self) {
        if self.mode_stack.len() > 1 {
            let popped = self.mode_stack.pop();
            // Restore brace depth when leaving interpolation modes
            if matches!(
                popped,
                Some(LexerMode::Interpolation | LexerMode::TemplateExpr)
            ) && let Some(saved) = self.brace_depth_stack.pop()
            {
                self.brace_depth = saved;
            }
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
    /// Returns all tokens on success, or stops at the first error.
    pub fn tokenize(mut self) -> LexResult<Vec<Token>> {
        let mut tokens = Vec::new();
        while self.pos < self.input.len() {
            match self.next_token() {
                Ok(Some(token)) => tokens.push(token),
                Ok(None) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(tokens)
    }

    /// Tokenizes the entire input, collecting errors instead of stopping.
    /// Returns all successfully lexed tokens and any errors encountered.
    pub fn tokenize_with_errors(mut self) -> (Vec<Token>, Vec<LexError>) {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();
        while self.pos < self.input.len() {
            match self.next_token() {
                Ok(Some(token)) => tokens.push(token),
                Ok(None) => break,
                Err(e) => {
                    errors.push(e);
                    // Skip one character to try to recover
                    self.advance(1);
                }
            }
        }
        (tokens, errors)
    }

    /// Returns the next token, or None if at EOF.
    fn next_token(&mut self) -> LexResult<Option<Token>> {
        if self.pos >= self.input.len() {
            return Ok(None);
        }

        let start = self.pos;

        // Check for Rust doc attr first (needs special text handling)
        if let Some((len, content)) = self.try_match_rust_doc_attr() {
            self.advance(len);
            return Ok(Some(Token {
                kind: SyntaxKind::RustDocAttr,
                text: content,
                start,
            }));
        }

        let kind = match self.mode() {
            LexerMode::Normal => self.lex_normal(), // Normal mode is infallible
            LexerMode::ControlBlock => self.lex_control_block()?,
            LexerMode::Directive => self.lex_directive()?,
            LexerMode::Interpolation => self.lex_interpolation()?,
            LexerMode::TemplateExpr => self.lex_template_expr()?,
            LexerMode::IdentBlock => self.lex_ident_block()?,
            LexerMode::StringLiteral => self.lex_string_literal()?,
            LexerMode::TemplateLiteral => self.lex_template_literal()?,
            LexerMode::JsDoc => self.lex_jsdoc(),
        };

        let text = self.input[start..self.pos].to_string();
        Ok(Some(Token { kind, text, start }))
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

        // Template control flow - opening constructs {#...
        // Check longest patterns first to avoid prefix matching issues
        if remaining.starts_with("{#while")
            && !remaining
                .chars()
                .nth(7)
                .map(|c| c.is_alphanumeric() || c == '_')
                .unwrap_or(false)
        {
            self.advance(7);
            self.push_mode(LexerMode::ControlBlock);
            return SyntaxKind::BraceHashWhile;
        }
        if remaining.starts_with("{#match")
            && !remaining
                .chars()
                .nth(7)
                .map(|c| c.is_alphanumeric() || c == '_')
                .unwrap_or(false)
        {
            self.advance(7);
            self.push_mode(LexerMode::ControlBlock);
            return SyntaxKind::BraceHashMatch;
        }
        if remaining.starts_with("{#for")
            && !remaining
                .chars()
                .nth(5)
                .map(|c| c.is_alphanumeric() || c == '_')
                .unwrap_or(false)
        {
            self.advance(5);
            self.push_mode(LexerMode::ControlBlock);
            return SyntaxKind::BraceHashFor;
        }
        if remaining.starts_with("{#if")
            && !remaining
                .chars()
                .nth(4)
                .map(|c| c.is_alphanumeric() || c == '_')
                .unwrap_or(false)
        {
            self.advance(4);
            self.push_mode(LexerMode::ControlBlock);
            return SyntaxKind::BraceHashIf;
        }

        // Template control flow - closing constructs {/...}
        if remaining.starts_with("{/while}") {
            self.advance(8);
            return SyntaxKind::BraceSlashWhileBrace;
        }
        if remaining.starts_with("{/match}") {
            self.advance(8);
            return SyntaxKind::BraceSlashMatchBrace;
        }
        if remaining.starts_with("{/for}") {
            self.advance(6);
            return SyntaxKind::BraceSlashForBrace;
        }
        if remaining.starts_with("{/if}") {
            self.advance(5);
            return SyntaxKind::BraceSlashIfBrace;
        }

        // Template control flow - continuation constructs {:...
        // Check {:else if before {:else to avoid prefix matching
        if remaining.starts_with("{:else if")
            && !remaining
                .chars()
                .nth(9)
                .map(|c| c.is_alphanumeric() || c == '_')
                .unwrap_or(false)
        {
            self.advance(9);
            self.push_mode(LexerMode::ControlBlock);
            return SyntaxKind::BraceColonElseIf;
        }
        if remaining.starts_with("{:else}") {
            self.advance(7);
            return SyntaxKind::BraceColonElseBrace;
        }
        if remaining.starts_with("{:case")
            && !remaining
                .chars()
                .nth(6)
                .map(|c| c.is_alphanumeric() || c == '_')
                .unwrap_or(false)
        {
            self.advance(6);
            self.push_mode(LexerMode::ControlBlock);
            return SyntaxKind::BraceColonCase;
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
            self.push_mode(LexerMode::JsDoc);
            return SyntaxKind::JsDocOpen;
        }

        // Note: */ is handled in JsDoc mode, not here in Normal mode

        // TypeScript line comment: // (but not /// which is doc comment, handled above)
        if remaining.starts_with("//") {
            // Consume // and everything until end of line
            self.advance(2);
            self.consume_while(|c| c != '\n');
            return SyntaxKind::TsLineComment;
        }

        // TypeScript block comment: /* (but not /** which is JSDoc, handled above)
        if remaining.starts_with("/*") {
            // Consume /* and everything until */
            self.advance(2);
            loop {
                if self.remaining().starts_with("*/") {
                    self.advance(2);
                    break;
                }
                if self.pos >= self.input.len() {
                    // Unterminated block comment - stop at EOF
                    break;
                }
                // Advance by one character
                if let Some(c) = self.peek() {
                    self.advance(c.len_utf8());
                } else {
                    break;
                }
            }
            return SyntaxKind::TsBlockComment;
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
                    // Check for === or ==
                    let rest = &self.input[self.pos + 1..];
                    if rest.starts_with("==") {
                        self.advance(3);
                        SyntaxKind::EqEqEq
                    } else if rest.starts_with('=') {
                        self.advance(2);
                        SyntaxKind::EqEq
                    } else {
                        self.advance(1);
                        SyntaxKind::Eq
                    }
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
                    // Check for !== or !=
                    let rest = &self.input[self.pos + 1..];
                    if rest.starts_with("==") {
                        self.advance(3);
                        SyntaxKind::NotEqEq
                    } else if rest.starts_with('=') {
                        self.advance(2);
                        SyntaxKind::NotEq
                    } else {
                        self.advance(1);
                        SyntaxKind::Exclaim
                    }
                }
                '&' => {
                    // Check for && or &=
                    let rest = &self.input[self.pos + 1..];
                    if rest.starts_with('&') {
                        self.advance(2);
                        SyntaxKind::AmpersandAmpersand
                    } else if rest.starts_with('=') {
                        self.advance(2);
                        SyntaxKind::AmpersandEq
                    } else {
                        self.advance(1);
                        SyntaxKind::Ampersand
                    }
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
                    let after_at = self
                        .input
                        .get(self.pos + 1..)
                        .and_then(|s| s.chars().next());
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
