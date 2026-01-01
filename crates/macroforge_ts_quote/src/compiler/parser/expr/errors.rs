//! Error types for the expression parser.
//!
//! Provides detailed, context-rich error messages for debugging parse failures.
//! No Raw fallbacks - all errors are explicit and actionable.

use crate::compiler::syntax::SyntaxKind;
use std::fmt;

/// The kind of parse error that occurred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorKind {
    /// Encountered an unexpected token.
    UnexpectedToken,
    /// Reached end of input unexpectedly.
    UnexpectedEof,
    /// Invalid operator in this context.
    InvalidOperator,
    /// Missing operand for an operator.
    MissingOperand,
    /// Expression was not properly terminated.
    UnterminatedExpression,
    /// Left-hand side of assignment is not valid.
    InvalidAssignmentTarget,
    /// Missing closing parenthesis ')'.
    MissingClosingParen,
    /// Missing closing bracket ']'.
    MissingClosingBracket,
    /// Missing closing brace '}'.
    MissingClosingBrace,
    /// Missing arrow function body after '=>'.
    MissingArrowBody,
    /// Invalid arrow function parameters.
    InvalidArrowParams,
    /// Missing ':' in conditional expression.
    MissingConditionalColon,
    /// Invalid property name in object literal.
    InvalidPropertyName,
    /// Multiple spread operators not allowed.
    DuplicateSpread,
    /// Rest element must be last.
    InvalidRestPosition,
    /// Type annotation found in expression context.
    TypeAnnotationInExpression,
    /// Invalid prefix operator.
    InvalidPrefixOperator,
    /// Invalid postfix operator.
    InvalidPostfixOperator,
    /// Invalid binary operator.
    InvalidBinaryOperator,
    /// Missing property name after '.'.
    MissingPropertyName,
    /// Invalid computed property.
    InvalidComputedProperty,
    /// Missing template literal closing backtick.
    UnterminatedTemplateLiteral,
    /// Missing string literal closing quote.
    UnterminatedStringLiteral,
    /// Invalid numeric literal.
    InvalidNumericLiteral,
    /// Invalid BigInt literal.
    InvalidBigIntLiteral,
    /// Expected identifier.
    ExpectedIdentifier,
    /// Expected expression.
    ExpectedExpression,
    /// Expected type annotation.
    ExpectedTypeAnnotation,
    /// Invalid 'new' expression.
    InvalidNewExpression,
    /// Invalid 'await' expression.
    InvalidAwaitExpression,
    /// Invalid 'yield' expression.
    InvalidYieldExpression,
    /// Missing function body.
    MissingFunctionBody,
    /// Invalid function parameter.
    InvalidFunctionParameter,
    /// Duplicate parameter name.
    DuplicateParameter,
    /// Invalid destructuring pattern.
    InvalidDestructuringPattern,
    /// Missing initializer.
    MissingInitializer,
    /// Invalid optional chaining.
    InvalidOptionalChaining,
    /// Invalid tagged template.
    InvalidTaggedTemplate,
    /// Invalid class expression.
    InvalidClassExpression,
    /// Reserved word used as identifier.
    ReservedWordAsIdentifier,
}

impl ParseErrorKind {
    /// Returns a human-readable description of this error kind.
    pub fn description(&self) -> &'static str {
        match self {
            Self::UnexpectedToken => "unexpected token",
            Self::UnexpectedEof => "unexpected end of input",
            Self::InvalidOperator => "invalid operator in this context",
            Self::MissingOperand => "missing operand for operator",
            Self::UnterminatedExpression => "expression was not properly terminated",
            Self::InvalidAssignmentTarget => "invalid assignment target",
            Self::MissingClosingParen => "missing closing parenthesis ')'",
            Self::MissingClosingBracket => "missing closing bracket ']'",
            Self::MissingClosingBrace => "missing closing brace '}'",
            Self::MissingArrowBody => "missing arrow function body after '=>'",
            Self::InvalidArrowParams => "invalid arrow function parameters",
            Self::MissingConditionalColon => "missing ':' in conditional expression",
            Self::InvalidPropertyName => "invalid property name",
            Self::DuplicateSpread => "multiple spread operators not allowed in this position",
            Self::InvalidRestPosition => "rest element must be last",
            Self::TypeAnnotationInExpression => "type annotation not allowed in expression",
            Self::InvalidPrefixOperator => "invalid prefix operator",
            Self::InvalidPostfixOperator => "invalid postfix operator",
            Self::InvalidBinaryOperator => "invalid binary operator",
            Self::MissingPropertyName => "missing property name after '.'",
            Self::InvalidComputedProperty => "invalid computed property",
            Self::UnterminatedTemplateLiteral => "unterminated template literal",
            Self::UnterminatedStringLiteral => "unterminated string literal",
            Self::InvalidNumericLiteral => "invalid numeric literal",
            Self::InvalidBigIntLiteral => "invalid BigInt literal",
            Self::ExpectedIdentifier => "expected identifier",
            Self::ExpectedExpression => "expected expression",
            Self::ExpectedTypeAnnotation => "expected type annotation",
            Self::InvalidNewExpression => "invalid 'new' expression",
            Self::InvalidAwaitExpression => "invalid 'await' expression",
            Self::InvalidYieldExpression => "invalid 'yield' expression",
            Self::MissingFunctionBody => "missing function body",
            Self::InvalidFunctionParameter => "invalid function parameter",
            Self::DuplicateParameter => "duplicate parameter name",
            Self::InvalidDestructuringPattern => "invalid destructuring pattern",
            Self::MissingInitializer => "missing initializer",
            Self::InvalidOptionalChaining => "invalid optional chaining",
            Self::InvalidTaggedTemplate => "invalid tagged template",
            Self::InvalidClassExpression => "invalid class expression",
            Self::ReservedWordAsIdentifier => "reserved word cannot be used as identifier",
        }
    }
}

/// A detailed parse error with context information.
#[derive(Debug, Clone)]
pub struct ParseError {
    /// The kind of error.
    pub kind: ParseErrorKind,
    /// Position in the input where the error occurred.
    pub position: usize,
    /// Context describing what was being parsed.
    pub context: String,
    /// What tokens/constructs were expected.
    pub expected: Vec<String>,
    /// What was actually found.
    pub found: Option<String>,
    /// Optional help text for fixing the error.
    pub help: Option<String>,
}

impl ParseError {
    /// Creates a new parse error.
    pub fn new(kind: ParseErrorKind, position: usize) -> Self {
        Self {
            kind,
            position,
            context: String::new(),
            expected: Vec::new(),
            found: None,
            help: None,
        }
    }

    /// Creates an "unexpected token" error.
    pub fn unexpected_token(position: usize, expected: &[&str], found: SyntaxKind) -> Self {
        Self {
            kind: ParseErrorKind::UnexpectedToken,
            position,
            context: String::new(),
            expected: expected.iter().map(|s| (*s).to_string()).collect(),
            found: Some(format!("{:?}", found)),
            help: None,
        }
    }

    /// Creates an "unexpected token" error with text.
    pub fn unexpected_token_text(position: usize, expected: &[&str], found: &str) -> Self {
        Self {
            kind: ParseErrorKind::UnexpectedToken,
            position,
            context: String::new(),
            expected: expected.iter().map(|s| (*s).to_string()).collect(),
            found: Some(format!("'{}'", found)),
            help: None,
        }
    }

    /// Creates an "unexpected EOF" error.
    pub fn unexpected_eof(position: usize, context: &str) -> Self {
        Self {
            kind: ParseErrorKind::UnexpectedEof,
            position,
            context: context.to_string(),
            expected: Vec::new(),
            found: None,
            help: None,
        }
    }

    /// Creates an "expected expression" error.
    pub fn expected_expression(position: usize) -> Self {
        Self {
            kind: ParseErrorKind::ExpectedExpression,
            position,
            context: String::new(),
            expected: vec!["expression".to_string()],
            found: None,
            help: None,
        }
    }

    /// Creates an "expected expression" error with found token.
    pub fn expected_expression_found(position: usize, found: SyntaxKind) -> Self {
        Self {
            kind: ParseErrorKind::ExpectedExpression,
            position,
            context: String::new(),
            expected: vec!["expression".to_string()],
            found: Some(format!("{:?}", found)),
            help: None,
        }
    }

    /// Creates an "invalid assignment target" error.
    pub fn invalid_assignment_target(position: usize) -> Self {
        Self {
            kind: ParseErrorKind::InvalidAssignmentTarget,
            position,
            context: String::new(),
            expected: vec!["identifier or member expression".to_string()],
            found: None,
            help: Some(
                "The left-hand side of an assignment must be a variable, property, or pattern"
                    .to_string(),
            ),
        }
    }

    /// Creates a "missing closing" error for the given delimiter.
    pub fn missing_closing(kind: ParseErrorKind, position: usize, opened_at: usize) -> Self {
        let delimiter = match kind {
            ParseErrorKind::MissingClosingParen => "')'",
            ParseErrorKind::MissingClosingBracket => "']'",
            ParseErrorKind::MissingClosingBrace => "'}'",
            _ => "delimiter",
        };
        Self {
            kind,
            position,
            context: format!("opened at position {}", opened_at),
            expected: vec![delimiter.to_string()],
            found: None,
            help: None,
        }
    }

    /// Adds context to the error.
    pub fn with_context(mut self, context: &str) -> Self {
        self.context = context.to_string();
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

    /// Converts the error to a user-friendly message.
    pub fn to_message(&self) -> String {
        let mut msg = format!("Parse error at position {}: ", self.position);

        // Main error description
        msg.push_str(self.kind.description());

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
            msg.push_str(&format!(" (while parsing {})", self.context));
        }

        // Help text
        if let Some(ref help) = self.help {
            msg.push_str(&format!("\n  help: {}", help));
        }

        msg
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_message())
    }
}

impl std::error::Error for ParseError {}

/// Result type for expression parsing.
pub type ParseResult<T> = Result<T, ParseError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unexpected_token_error() {
        let err = ParseError::unexpected_token(42, &["identifier", "number"], SyntaxKind::LParen);
        let msg = err.to_message();
        assert!(msg.contains("position 42"));
        assert!(msg.contains("unexpected token"));
        assert!(msg.contains("identifier"));
        assert!(msg.contains("number"));
    }

    #[test]
    fn test_unexpected_eof_error() {
        let err = ParseError::unexpected_eof(100, "function body");
        let msg = err.to_message();
        assert!(msg.contains("position 100"));
        assert!(msg.contains("unexpected end of input"));
        assert!(msg.contains("function body"));
    }

    #[test]
    fn test_invalid_assignment_target() {
        let err = ParseError::invalid_assignment_target(50);
        let msg = err.to_message();
        assert!(msg.contains("invalid assignment target"));
        assert!(msg.contains("help:"));
    }

    #[test]
    fn test_error_with_context() {
        let err = ParseError::new(ParseErrorKind::MissingOperand, 10)
            .with_context("binary expression")
            .with_expected(&["+", "-", "*", "/"])
            .with_found("EOF");
        let msg = err.to_message();
        assert!(msg.contains("binary expression"));
        assert!(msg.contains("EOF"));
    }

    #[test]
    fn test_all_error_kinds_have_descriptions() {
        // Ensure every error kind has a non-empty description
        let kinds = [
            ParseErrorKind::UnexpectedToken,
            ParseErrorKind::UnexpectedEof,
            ParseErrorKind::InvalidOperator,
            ParseErrorKind::MissingOperand,
            ParseErrorKind::UnterminatedExpression,
            ParseErrorKind::InvalidAssignmentTarget,
            ParseErrorKind::MissingClosingParen,
            ParseErrorKind::MissingClosingBracket,
            ParseErrorKind::MissingClosingBrace,
            ParseErrorKind::MissingArrowBody,
            ParseErrorKind::InvalidArrowParams,
            ParseErrorKind::MissingConditionalColon,
            ParseErrorKind::InvalidPropertyName,
            ParseErrorKind::DuplicateSpread,
            ParseErrorKind::InvalidRestPosition,
            ParseErrorKind::TypeAnnotationInExpression,
            ParseErrorKind::InvalidPrefixOperator,
            ParseErrorKind::InvalidPostfixOperator,
            ParseErrorKind::InvalidBinaryOperator,
            ParseErrorKind::MissingPropertyName,
            ParseErrorKind::InvalidComputedProperty,
            ParseErrorKind::UnterminatedTemplateLiteral,
            ParseErrorKind::UnterminatedStringLiteral,
            ParseErrorKind::InvalidNumericLiteral,
            ParseErrorKind::InvalidBigIntLiteral,
            ParseErrorKind::ExpectedIdentifier,
            ParseErrorKind::ExpectedExpression,
            ParseErrorKind::ExpectedTypeAnnotation,
            ParseErrorKind::InvalidNewExpression,
            ParseErrorKind::InvalidAwaitExpression,
            ParseErrorKind::InvalidYieldExpression,
            ParseErrorKind::MissingFunctionBody,
            ParseErrorKind::InvalidFunctionParameter,
            ParseErrorKind::DuplicateParameter,
            ParseErrorKind::InvalidDestructuringPattern,
            ParseErrorKind::MissingInitializer,
            ParseErrorKind::InvalidOptionalChaining,
            ParseErrorKind::InvalidTaggedTemplate,
            ParseErrorKind::InvalidClassExpression,
            ParseErrorKind::ReservedWordAsIdentifier,
        ];

        for kind in kinds {
            let desc = kind.description();
            assert!(!desc.is_empty(), "{:?} has empty description", kind);
        }
    }
}
