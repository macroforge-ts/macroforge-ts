//! Error types for the lexer.
//!
//! Provides detailed, context-rich error messages for debugging lexer failures.

use std::fmt;

/// The kind of lexer error that occurred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexErrorKind {
    /// Unterminated string literal - missing closing quote.
    UnterminatedString,
    /// Unterminated template literal - missing closing backtick.
    UnterminatedTemplateLiteral,
    /// Unterminated interpolation - missing closing brace.
    UnterminatedInterpolation,
    /// Unterminated control block - missing closing brace.
    UnterminatedControlBlock,
    /// Unterminated directive - missing closing brace.
    UnterminatedDirective,
    /// Unterminated ident block - missing closing `|}`.
    UnterminatedIdentBlock,
    /// Invalid escape sequence in string.
    InvalidEscapeSequence,
    /// Invalid character in input.
    InvalidCharacter,
    /// Invalid unicode escape sequence.
    InvalidUnicodeEscape,
    /// Unexpected end of input.
    UnexpectedEof,
    /// Unbalanced braces in interpolation.
    UnbalancedBraces,
    /// Invalid control block syntax.
    InvalidControlBlock,
    /// Invalid directive syntax.
    InvalidDirective,
    /// Invalid template expression.
    InvalidTemplateExpression,
    /// Nested interpolation depth exceeded.
    InterpolationTooDeep,
}

impl LexErrorKind {
    /// Returns a human-readable description of this error kind.
    pub fn description(&self) -> &'static str {
        match self {
            Self::UnterminatedString => "unterminated string literal",
            Self::UnterminatedTemplateLiteral => "unterminated template literal",
            Self::UnterminatedInterpolation => "unterminated interpolation",
            Self::UnterminatedControlBlock => "unterminated control block",
            Self::UnterminatedDirective => "unterminated directive",
            Self::UnterminatedIdentBlock => "unterminated ident block",
            Self::InvalidEscapeSequence => "invalid escape sequence",
            Self::InvalidCharacter => "invalid character",
            Self::InvalidUnicodeEscape => "invalid unicode escape sequence",
            Self::UnexpectedEof => "unexpected end of input",
            Self::UnbalancedBraces => "unbalanced braces in interpolation",
            Self::InvalidControlBlock => "invalid control block syntax",
            Self::InvalidDirective => "invalid directive syntax",
            Self::InvalidTemplateExpression => "invalid template expression",
            Self::InterpolationTooDeep => "interpolation nesting too deep",
        }
    }

    /// Returns a suggested fix for this error kind.
    pub fn suggestion(&self) -> Option<&'static str> {
        match self {
            Self::UnterminatedString => Some("add a closing quote \" or '"),
            Self::UnterminatedTemplateLiteral => Some("add a closing backtick `"),
            Self::UnterminatedInterpolation => Some("add a closing brace } to @{...}"),
            Self::UnterminatedControlBlock => Some("add a closing brace } to control block"),
            Self::UnterminatedDirective => Some("add a closing brace } to directive {$...}"),
            Self::UnterminatedIdentBlock => Some("add closing |} to ident block {|...|}")
            ,
            Self::InvalidEscapeSequence => Some("use valid escape: \\n, \\t, \\r, \\\\, \\\", \\'"),
            Self::UnbalancedBraces => Some("ensure all { have matching }"),
            Self::InvalidControlBlock => Some("use {#if}, {#for}, {#while}, or {#match}"),
            Self::InvalidDirective => Some("use {$directive_name}"),
            _ => None,
        }
    }
}

/// A detailed lexer error with context information.
#[derive(Debug, Clone)]
pub struct LexError {
    /// The kind of error.
    pub kind: LexErrorKind,
    /// Byte position in the input where the error occurred.
    pub position: usize,
    /// The lexer mode when the error occurred.
    pub mode: String,
    /// Context describing what was being lexed.
    pub context: String,
    /// A snippet of the input around the error position.
    pub snippet: String,
    /// What was expected at this position.
    pub expected: Vec<String>,
    /// What was actually found.
    pub found: Option<String>,
    /// Optional help text for fixing the error.
    pub help: Option<String>,
    /// Position where a construct was opened (for unterminated errors).
    pub opened_at: Option<usize>,
}

impl LexError {
    /// Creates a new lexer error.
    pub fn new(kind: LexErrorKind, position: usize) -> Self {
        Self {
            kind,
            position,
            mode: String::new(),
            context: String::new(),
            snippet: String::new(),
            expected: Vec::new(),
            found: None,
            help: None,
            opened_at: None,
        }
    }

    /// Creates an "unterminated string" error.
    pub fn unterminated_string(position: usize, opened_at: usize, quote_char: char) -> Self {
        Self {
            kind: LexErrorKind::UnterminatedString,
            position,
            mode: "StringLiteral".to_string(),
            context: format!("string starting with {}", quote_char),
            snippet: String::new(),
            expected: vec![format!("closing {}", quote_char)],
            found: Some("end of input".to_string()),
            help: Some(format!("add {} to close the string", quote_char)),
            opened_at: Some(opened_at),
        }
    }

    /// Creates an "unterminated template literal" error.
    pub fn unterminated_template(position: usize, opened_at: usize) -> Self {
        Self {
            kind: LexErrorKind::UnterminatedTemplateLiteral,
            position,
            mode: "TemplateLiteral".to_string(),
            context: "template literal".to_string(),
            snippet: String::new(),
            expected: vec!["`".to_string()],
            found: Some("end of input".to_string()),
            help: Some("add ` to close the template literal".to_string()),
            opened_at: Some(opened_at),
        }
    }

    /// Creates an "unterminated interpolation" error.
    pub fn unterminated_interpolation(position: usize, opened_at: usize, depth: usize) -> Self {
        Self {
            kind: LexErrorKind::UnterminatedInterpolation,
            position,
            mode: "Interpolation".to_string(),
            context: format!("interpolation at depth {}", depth),
            snippet: String::new(),
            expected: vec!["}".to_string()],
            found: Some("end of input".to_string()),
            help: Some("add } to close the @{...} interpolation".to_string()),
            opened_at: Some(opened_at),
        }
    }

    /// Creates an "unterminated control block" error.
    pub fn unterminated_control_block(position: usize, opened_at: usize, block_type: &str) -> Self {
        Self {
            kind: LexErrorKind::UnterminatedControlBlock,
            position,
            mode: "ControlBlock".to_string(),
            context: format!("{} control block", block_type),
            snippet: String::new(),
            expected: vec!["}".to_string()],
            found: Some("end of input".to_string()),
            help: Some(format!("add }} to close the {{#{} block", block_type)),
            opened_at: Some(opened_at),
        }
    }

    /// Creates an "unexpected EOF" error.
    pub fn unexpected_eof(position: usize, context: &str) -> Self {
        Self {
            kind: LexErrorKind::UnexpectedEof,
            position,
            mode: String::new(),
            context: context.to_string(),
            snippet: String::new(),
            expected: Vec::new(),
            found: None,
            help: None,
            opened_at: None,
        }
    }

    /// Creates an "invalid character" error.
    pub fn invalid_character(position: usize, char: char, context: &str) -> Self {
        Self {
            kind: LexErrorKind::InvalidCharacter,
            position,
            mode: String::new(),
            context: context.to_string(),
            snippet: String::new(),
            expected: Vec::new(),
            found: Some(format!("'{}'", char)),
            help: None,
            opened_at: None,
        }
    }

    /// Creates an "invalid escape sequence" error.
    pub fn invalid_escape(position: usize, sequence: &str) -> Self {
        Self {
            kind: LexErrorKind::InvalidEscapeSequence,
            position,
            mode: "StringLiteral".to_string(),
            context: "escape sequence".to_string(),
            snippet: String::new(),
            expected: vec![
                "\\n".to_string(),
                "\\t".to_string(),
                "\\r".to_string(),
                "\\\\".to_string(),
            ],
            found: Some(format!("\\{}", sequence)),
            help: Some("use a valid escape sequence".to_string()),
            opened_at: None,
        }
    }

    /// Sets the lexer mode when error occurred.
    pub fn in_mode(mut self, mode: &str) -> Self {
        self.mode = mode.to_string();
        self
    }

    /// Adds context to the error.
    pub fn with_context(mut self, context: &str) -> Self {
        self.context = context.to_string();
        self
    }

    /// Adds a snippet of the input around the error.
    pub fn with_snippet(mut self, snippet: &str) -> Self {
        self.snippet = snippet.to_string();
        self
    }

    /// Adds expected tokens to the error.
    pub fn with_expected(mut self, expected: &[&str]) -> Self {
        self.expected = expected.iter().map(|s| (*s).to_string()).collect();
        self
    }

    /// Adds the found token to the error.
    pub fn with_found(mut self, found: &str) -> Self {
        self.found = Some(found.to_string());
        self
    }

    /// Adds help text to the error.
    pub fn with_help(mut self, help: &str) -> Self {
        self.help = Some(help.to_string());
        self
    }

    /// Sets the position where the construct was opened.
    pub fn opened_at(mut self, pos: usize) -> Self {
        self.opened_at = Some(pos);
        self
    }

    /// Converts the error to a user-friendly message.
    pub fn to_message(&self) -> String {
        let mut msg = format!("Lex error at position {}: ", self.position);

        // Main error description
        msg.push_str(self.kind.description());

        // Mode context
        if !self.mode.is_empty() {
            msg.push_str(&format!(" (in {} mode)", self.mode));
        }

        // What was found
        if let Some(ref found) = self.found {
            msg.push_str(&format!(", found {}", found));
        }

        // What was expected
        if !self.expected.is_empty() {
            if self.expected.len() == 1 {
                msg.push_str(&format!(", expected {}", self.expected[0]));
            } else {
                msg.push_str(&format!(
                    ", expected one of: {}",
                    self.expected.join(", ")
                ));
            }
        }

        // Context
        if !self.context.is_empty() {
            msg.push_str(&format!(" while lexing {}", self.context));
        }

        // Opened at position
        if let Some(opened) = self.opened_at {
            msg.push_str(&format!(" (opened at position {})", opened));
        }

        // Snippet
        if !self.snippet.is_empty() {
            msg.push_str(&format!("\n  --> {}", self.snippet));
        }

        // Help text - use custom help or suggestion from error kind
        if let Some(ref help) = self.help {
            msg.push_str(&format!("\n  help: {}", help));
        } else if let Some(suggestion) = self.kind.suggestion() {
            msg.push_str(&format!("\n  help: {}", suggestion));
        }

        msg
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_message())
    }
}

impl std::error::Error for LexError {}

/// Result type for lexer operations.
pub type LexResult<T> = Result<T, LexError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unterminated_string_error() {
        let err = LexError::unterminated_string(50, 10, '"');
        let msg = err.to_message();
        assert!(msg.contains("position 50"));
        assert!(msg.contains("unterminated string"));
        assert!(msg.contains("opened at position 10"));
    }

    #[test]
    fn test_unterminated_template_error() {
        let err = LexError::unterminated_template(100, 20);
        let msg = err.to_message();
        assert!(msg.contains("position 100"));
        assert!(msg.contains("unterminated template literal"));
    }

    #[test]
    fn test_unterminated_interpolation_error() {
        let err = LexError::unterminated_interpolation(75, 30, 2);
        let msg = err.to_message();
        assert!(msg.contains("position 75"));
        assert!(msg.contains("depth 2"));
    }

    #[test]
    fn test_invalid_escape_error() {
        let err = LexError::invalid_escape(42, "q");
        let msg = err.to_message();
        assert!(msg.contains("position 42"));
        assert!(msg.contains("invalid escape"));
        assert!(msg.contains("\\q"));
    }

    #[test]
    fn test_error_with_snippet() {
        let err = LexError::new(LexErrorKind::InvalidCharacter, 10)
            .with_snippet("const x = @invalid")
            .with_found("@")
            .with_context("identifier");
        let msg = err.to_message();
        assert!(msg.contains("const x = @invalid"));
    }

    #[test]
    fn test_all_error_kinds_have_descriptions() {
        let kinds = [
            LexErrorKind::UnterminatedString,
            LexErrorKind::UnterminatedTemplateLiteral,
            LexErrorKind::UnterminatedInterpolation,
            LexErrorKind::UnterminatedControlBlock,
            LexErrorKind::UnterminatedDirective,
            LexErrorKind::UnterminatedIdentBlock,
            LexErrorKind::InvalidEscapeSequence,
            LexErrorKind::InvalidCharacter,
            LexErrorKind::InvalidUnicodeEscape,
            LexErrorKind::UnexpectedEof,
            LexErrorKind::UnbalancedBraces,
            LexErrorKind::InvalidControlBlock,
            LexErrorKind::InvalidDirective,
            LexErrorKind::InvalidTemplateExpression,
            LexErrorKind::InterpolationTooDeep,
        ];

        for kind in kinds {
            let desc = kind.description();
            assert!(!desc.is_empty(), "{:?} has empty description", kind);
        }
    }
}
