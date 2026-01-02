//! Error types for the expression parser.
//!
//! Provides detailed, context-rich error messages for debugging parse failures.
//! No Raw fallbacks - all errors are explicit and actionable.

use crate::compiler::syntax::SyntaxKind;
use std::fmt;

/// Line and column information for error reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceLocation {
    /// 1-indexed line number.
    pub line: usize,
    /// 1-indexed column number (in UTF-16 code units for LSP/editor compatibility).
    pub column: usize,
    /// Byte offset in the source.
    pub offset: usize,
}

impl SourceLocation {
    /// Calculate line/column from source and byte offset.
    ///
    /// Column is calculated in UTF-16 code units (not bytes or chars) for
    /// compatibility with editors like Zed that use UTF-16 internally.
    pub fn from_offset(source: &str, offset: usize) -> Self {
        let offset = offset.min(source.len());
        let before = &source[..offset];

        // Count lines (1-indexed)
        let line = before.chars().filter(|&c| c == '\n').count() + 1;

        // Find start of current line
        let line_start = before.rfind('\n').map(|pos| pos + 1).unwrap_or(0);

        // Calculate column in UTF-16 code units (1-indexed)
        // This matches LSP and editors like Zed that use UTF-16 internally
        let line_content = &source[line_start..offset];
        let column = line_content.chars().map(|c| c.len_utf16()).sum::<usize>() + 1;

        Self {
            line,
            column,
            offset,
        }
    }
}

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
    /// Invalid interpolation syntax (e.g., missing @{ or } delimiters).
    InvalidInterpolation,
    /// Invalid Rust expression inside interpolation.
    InvalidRustExpression,
    /// If-expression requires an else branch to produce a value.
    MissingElseBranch,
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
            Self::InvalidInterpolation => "invalid interpolation syntax",
            Self::InvalidRustExpression => "invalid Rust expression in interpolation",
            Self::MissingElseBranch => "if-expression requires an {:else} branch to produce a value",
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

    /// Formats the error with source context for display.
    ///
    /// Produces an error message with source context:
    /// ```text
    /// error: message
    ///  --> input:8:13
    /// `8 | const x = foo                    `
    /// `  |           ^ found: 'bar'         `
    /// help: suggestion
    /// ```
    pub fn format_with_source(&self, source: &str) -> String {
        self.format_with_source_and_file(source, "input", 0)
    }

    /// Formats the error with source context and a custom filename.
    /// `line_offset` is added to convert relative template lines to absolute file lines.
    pub fn format_with_source_and_file(&self, source: &str, filename: &str, line_offset: usize) -> String {
        let loc = SourceLocation::from_offset(source, self.position);
        let absolute_line = loc.line + line_offset;

        // Header: error: message
        let mut msg = format!("error: {}\n", self.kind.description());

        // Location: --> file:line:column (UTF-16 column for editor compatibility)
        msg.push_str(&format!(" --> {}:{}:{}\n", filename, absolute_line, loc.column));

        // Extract and show the problematic line with caret
        let lines: Vec<&str> = source.lines().collect();
        if loc.line > 0 && loc.line <= lines.len() {
            let line_content = lines[loc.line - 1];
            // Expand tabs to spaces for consistent alignment, trim any trailing whitespace
            let expanded_content = line_content.replace('\t', "    ").trim_end().to_string();

            // Calculate byte offset within the line for visual caret positioning
            let line_start_offset = source[..self.position]
                .rfind('\n')
                .map(|pos| pos + 1)
                .unwrap_or(0);
            let byte_offset_in_line = self.position.saturating_sub(line_start_offset);

            // Calculate visual column (accounting for tabs) from byte offset
            let visual_column: usize = line_content
                .char_indices()
                .take_while(|(i, _)| *i < byte_offset_in_line)
                .map(|(_, c)| if c == '\t' { 4 } else { 1 })
                .sum();

            // Caret line annotation
            let annotation = if let Some(ref found) = self.found {
                format!("found: {}", found)
            } else if !self.expected.is_empty() {
                format!("expected: {}", self.expected.join(" or "))
            } else {
                String::new()
            };

            // Truncate long lines to show context around the error
            const MAX_LINE_LEN: usize = 80;
            const CONTEXT_CHARS: usize = 30;

            let (display_content, display_caret_col, truncated) = if expanded_content.len() > MAX_LINE_LEN {
                // Find a window around the error position
                let start = visual_column.saturating_sub(CONTEXT_CHARS);
                let end = (visual_column + CONTEXT_CHARS).min(expanded_content.len());

                // Adjust to char boundaries
                let content_chars: Vec<char> = expanded_content.chars().collect();
                let start = start.min(content_chars.len());
                let end = end.min(content_chars.len());

                let prefix = if start > 0 { "..." } else { "" };
                let suffix = if end < content_chars.len() { "..." } else { "" };

                let snippet: String = content_chars[start..end].iter().collect();
                let new_caret_col = visual_column - start + prefix.len();

                (format!("{}{}{}", prefix, snippet, suffix), new_caret_col, true)
            } else {
                (expanded_content.clone(), visual_column, false)
            };

            // Line number width for alignment
            let line_num_width = absolute_line.to_string().len();

            // Source line with line number and pipe
            let source_line = format!("{:>width$} | {}", absolute_line, display_content, width = line_num_width);
            // Caret line with pipe (no line number, just spaces)
            let caret_content = format!("{:>col$}^ {}", "", annotation, col = display_caret_col);
            let caret_line = format!("{:>width$} | {}", "", caret_content, width = line_num_width);

            // Pad both lines to equal length for alignment
            let max_len = source_line.len().max(caret_line.len());
            let padded_source = format!("{:<width$}", source_line, width = max_len);
            let padded_caret = format!("{:<width$}", caret_line, width = max_len);

            msg.push_str(&format!("`{}`\n", padded_source));
            msg.push_str(&format!("`{}`\n", padded_caret));

            let _ = truncated; // silence unused warning
        }

        // Help note if available
        if let Some(ref help) = self.help {
            msg.push_str(&format!("help: {}\n", help));
        }

        msg
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

/// Outcome of attempting to parse a single node.
///
/// Used by `parse_node()` to distinguish between:
/// - Successfully parsed a node
/// - Nothing to parse at current position (skip token and continue)
#[derive(Debug)]
pub enum ParseOutcome<T> {
    /// Successfully parsed a node.
    Node(T),
    /// Nothing to parse at current position (skip token).
    Skip,
}

impl<T> ParseOutcome<T> {
    /// Returns true if this outcome contains a node.
    pub fn is_node(&self) -> bool {
        matches!(self, Self::Node(_))
    }

    /// Returns true if this outcome is a skip.
    pub fn is_skip(&self) -> bool {
        matches!(self, Self::Skip)
    }

    /// Converts the outcome to an Option.
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Node(node) => Some(node),
            Self::Skip => None,
        }
    }
}

/// Result type for parse_node() - can succeed with a node, skip, or error.
pub type ParseNodeResult<T> = Result<ParseOutcome<T>, ParseError>;

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
    fn test_source_location_from_offset() {
        // Test basic case: position on line 3
        let source = "line1\nline2\n            = @{raw_var_ident} as @{inner}[];";

        // Calculate offset of ']' character
        // "line1\n" = 6, "line2\n" = 6, then 43 chars to get to ']'
        let line3 = "            = @{raw_var_ident} as @{inner}[]";
        let offset_of_bracket = 6 + 6 + line3.len() - 1; // -1 because we want ']' not ';'

        let loc = SourceLocation::from_offset(source, offset_of_bracket);

        println!("Source: {:?}", source);
        println!("Offset: {}", offset_of_bracket);
        println!("Line: {}, Column: {}", loc.line, loc.column);

        assert_eq!(loc.line, 3, "Should be line 3");
        // Column should be position of ']' on line 3 (1-indexed)
        // "            = @{raw_var_ident} as @{inner}[]"
        //  123456789012345678901234567890123456789012345
        //            1         2         3         4
        // ']' is at position 44
        assert_eq!(loc.column, 44, "Should be column 44");
    }

    #[test]
    fn test_source_location_first_line() {
        let source = "hello world";
        let loc = SourceLocation::from_offset(source, 6); // 'w'
        assert_eq!(loc.line, 1);
        assert_eq!(loc.column, 7); // 1-indexed
    }

    #[test]
    fn test_source_location_after_newline() {
        let source = "hello\nworld";
        let loc = SourceLocation::from_offset(source, 6); // 'w' after newline
        assert_eq!(loc.line, 2);
        assert_eq!(loc.column, 1); // First char on line 2
    }

    #[test]
    fn test_caret_with_placeholder_and_array_type() {
        // This is the actual template content that shows wrong caret position
        let source = r#"instance.@{field._field_ident} = @{raw_var_ident} as @{inner}[];"#;

        // Let's see how the lexer tokenizes this and what positions we get
        use crate::compiler::lexer::Lexer;

        let tokens: Vec<_> = Lexer::new(source).tokenize().unwrap();
        println!("Tokens:");
        for (i, t) in tokens.iter().enumerate() {
            println!("  {}: {:?} '{}' at byte {}", i, t.kind, t.text, t.start);
        }

        // Find the RBracket token
        let rbracket = tokens.iter().find(|t| t.text == "]").unwrap();
        println!("\nRBracket token at byte offset: {}", rbracket.start);

        // Now check what SourceLocation gives us
        let loc = SourceLocation::from_offset(source, rbracket.start);
        println!("SourceLocation: line {}, column {}", loc.line, loc.column);

        // Verify the character at that position in the source
        let char_at_offset = source.chars().nth(rbracket.start).unwrap();
        println!("Character at offset {}: '{}'", rbracket.start, char_at_offset);

        // The character should be ']'
        assert_eq!(char_at_offset, ']', "Token start should point to ']'");
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
            ParseErrorKind::InvalidInterpolation,
            ParseErrorKind::InvalidRustExpression,
            ParseErrorKind::MissingElseBranch,
        ];

        for kind in kinds {
            let desc = kind.description();
            assert!(!desc.is_empty(), "{:?} has empty description", kind);
        }
    }
}
