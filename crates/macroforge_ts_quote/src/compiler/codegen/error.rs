//! Error types for code generation.
//!
//! Provides detailed, context-rich error messages for debugging codegen failures.
//! No silent fallbacks - all errors are explicit and actionable.

use crate::compiler::ir::{IrNode, IrSpan};
use std::fmt;

/// The kind of codegen error that occurred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenErrorKind {
    /// Encountered an unexpected IR node type for this context.
    UnexpectedIrNode,
    /// Missing required field in IR node.
    MissingRequiredField,
    /// Invalid placeholder kind for this position.
    InvalidPlaceholderKind,
    /// Failed to generate expression.
    ExpressionGenerationFailed,
    /// Failed to generate statement.
    StatementGenerationFailed,
    /// Failed to generate type annotation.
    TypeGenerationFailed,
    /// Failed to generate pattern.
    PatternGenerationFailed,
    /// Failed to generate identifier.
    IdentifierGenerationFailed,
    /// Failed to generate property.
    PropertyGenerationFailed,
    /// Failed to generate parameter.
    ParameterGenerationFailed,
    /// Failed to generate class member.
    ClassMemberGenerationFailed,
    /// Failed to generate interface member.
    InterfaceMemberGenerationFailed,
    /// Failed to generate module item.
    ModuleItemGenerationFailed,
    /// Failed to generate declaration.
    DeclarationGenerationFailed,
    /// Invalid numeric literal format.
    InvalidNumericLiteral,
    /// Invalid BigInt literal format.
    InvalidBigIntLiteral,
    /// Invalid operator for this context.
    InvalidOperator,
    /// Control flow node in unsupported position.
    UnsupportedControlFlowPosition,
    /// Spread element in unsupported position.
    UnsupportedSpreadPosition,
    /// Empty block where content is required.
    EmptyBlockNotAllowed,
    /// Invalid entity name.
    InvalidEntityName,
    /// Invalid property name.
    InvalidPropertyName,
    /// Invalid method kind.
    InvalidMethodKind,
    /// Invalid accessibility modifier.
    InvalidAccessibility,
    /// Internal codegen bug - should never happen.
    InternalError,
}

impl GenErrorKind {
    /// Returns a human-readable description of this error kind.
    pub fn description(&self) -> &'static str {
        match self {
            Self::UnexpectedIrNode => "unexpected IR node type for this context",
            Self::MissingRequiredField => "missing required field in IR node",
            Self::InvalidPlaceholderKind => "invalid placeholder kind for this position",
            Self::ExpressionGenerationFailed => "failed to generate expression",
            Self::StatementGenerationFailed => "failed to generate statement",
            Self::TypeGenerationFailed => "failed to generate type annotation",
            Self::PatternGenerationFailed => "failed to generate pattern",
            Self::IdentifierGenerationFailed => "failed to generate identifier",
            Self::PropertyGenerationFailed => "failed to generate property",
            Self::ParameterGenerationFailed => "failed to generate parameter",
            Self::ClassMemberGenerationFailed => "failed to generate class member",
            Self::InterfaceMemberGenerationFailed => "failed to generate interface member",
            Self::ModuleItemGenerationFailed => "failed to generate module item",
            Self::DeclarationGenerationFailed => "failed to generate declaration",
            Self::InvalidNumericLiteral => "invalid numeric literal format",
            Self::InvalidBigIntLiteral => "invalid BigInt literal format",
            Self::InvalidOperator => "invalid operator for this context",
            Self::UnsupportedControlFlowPosition => "control flow node in unsupported position",
            Self::UnsupportedSpreadPosition => "spread element in unsupported position",
            Self::EmptyBlockNotAllowed => "empty block where content is required",
            Self::InvalidEntityName => "invalid entity name",
            Self::InvalidPropertyName => "invalid property name",
            Self::InvalidMethodKind => "invalid method kind",
            Self::InvalidAccessibility => "invalid accessibility modifier",
            Self::InternalError => "internal codegen error (this is a bug)",
        }
    }
}

/// A detailed codegen error with context information.
#[derive(Debug, Clone)]
pub struct GenError {
    /// The kind of error.
    pub kind: GenErrorKind,
    /// Context describing what was being generated.
    pub context: String,
    /// The IR node that caused the error (as debug string).
    pub ir_node: Option<String>,
    /// What was expected.
    pub expected: Vec<String>,
    /// What was actually found.
    pub found: Option<String>,
    /// Optional help text for fixing the error.
    pub help: Option<String>,
    /// Source chain for nested errors.
    pub source: Option<Box<GenError>>,
    /// Byte offset span in template source for error highlighting.
    pub span: Option<IrSpan>,
}

impl GenError {
    /// Creates a new codegen error.
    pub fn new(kind: GenErrorKind) -> Self {
        Self {
            kind,
            context: String::new(),
            ir_node: None,
            expected: Vec::new(),
            found: None,
            help: None,
            source: None,
            span: None,
        }
    }

    /// Creates an "unexpected IR node" error with span from the node.
    pub fn unexpected_node(context: &str, node: &IrNode, expected: &[&str]) -> Self {
        Self {
            kind: GenErrorKind::UnexpectedIrNode,
            context: context.to_string(),
            ir_node: Some(format!("{:?}", std::mem::discriminant(node))),
            expected: expected.iter().map(|s| (*s).to_string()).collect(),
            found: Some(node_variant_name(node)),
            help: None,
            source: None,
            span: Some(node.span()),
        }
    }

    /// Creates an "invalid placeholder kind" error.
    pub fn invalid_placeholder(
        context: &str,
        found_kind: &str,
        expected_kinds: &[&str],
    ) -> Self {
        Self {
            kind: GenErrorKind::InvalidPlaceholderKind,
            context: context.to_string(),
            ir_node: None,
            expected: expected_kinds.iter().map(|s| (*s).to_string()).collect(),
            found: Some(found_kind.to_string()),
            help: Some(format!(
                "In {} position, placeholders must be one of: {}",
                context,
                expected_kinds.join(", ")
            )),
            source: None,
            span: None,
        }
    }

    /// Creates an "invalid placeholder kind" error with span.
    pub fn invalid_placeholder_at(
        context: &str,
        found_kind: &str,
        expected_kinds: &[&str],
        span: IrSpan,
    ) -> Self {
        Self {
            kind: GenErrorKind::InvalidPlaceholderKind,
            context: context.to_string(),
            ir_node: None,
            expected: expected_kinds.iter().map(|s| (*s).to_string()).collect(),
            found: Some(found_kind.to_string()),
            help: Some(format!(
                "In {} position, placeholders must be one of: {}",
                context,
                expected_kinds.join(", ")
            )),
            source: None,
            span: Some(span),
        }
    }

    /// Creates a "missing required field" error.
    pub fn missing_field(context: &str, field_name: &str) -> Self {
        Self {
            kind: GenErrorKind::MissingRequiredField,
            context: context.to_string(),
            ir_node: None,
            expected: vec![field_name.to_string()],
            found: Some("None".to_string()),
            help: Some(format!("The {} field is required for {}", field_name, context)),
            source: None,
            span: None,
        }
    }

    /// Creates an internal error (should never happen).
    pub fn internal(message: &str) -> Self {
        Self {
            kind: GenErrorKind::InternalError,
            context: String::new(),
            ir_node: None,
            expected: Vec::new(),
            found: None,
            help: Some(format!(
                "This is a bug in the codegen. Please report it with this message: {}",
                message
            )),
            source: None,
            span: None,
        }
    }

    /// Adds context to the error.
    pub fn with_context(mut self, context: &str) -> Self {
        self.context = context.to_string();
        self
    }

    /// Adds the IR node to the error (also captures span for highlighting).
    pub fn with_ir_node(mut self, node: &IrNode) -> Self {
        self.ir_node = Some(format!("{:?}", std::mem::discriminant(node)));
        if self.span.is_none() {
            self.span = Some(node.span());
        }
        self
    }

    /// Adds expected items to the error.
    pub fn with_expected(mut self, expected: &[&str]) -> Self {
        self.expected = expected.iter().map(|s| (*s).to_string()).collect();
        self
    }

    /// Adds the found item to the error.
    pub fn with_found(mut self, found: &str) -> Self {
        self.found = Some(found.to_string());
        self
    }

    /// Adds help text to the error.
    pub fn with_help(mut self, help: &str) -> Self {
        self.help = Some(help.to_string());
        self
    }

    /// Adds a source error (for error chaining).
    pub fn with_source(mut self, source: GenError) -> Self {
        self.source = Some(Box::new(source));
        self
    }

    /// Adds a span for error highlighting.
    pub fn with_span(mut self, span: IrSpan) -> Self {
        self.span = Some(span);
        self
    }

    /// Adds a span from an IR node for error highlighting.
    pub fn with_node_span(mut self, node: &IrNode) -> Self {
        self.span = Some(node.span());
        self
    }

    /// Returns the position (start byte offset) for error highlighting.
    pub fn position(&self) -> Option<usize> {
        self.span.map(|s| s.start)
    }

    /// Converts the error to a user-friendly message.
    pub fn to_message(&self) -> String {
        let mut msg = String::new();

        // Main error description
        msg.push_str("Codegen error: ");
        msg.push_str(self.kind.description());

        // Context
        if !self.context.is_empty() {
            msg.push_str(&format!(" (while generating {})", self.context));
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

        // IR node info
        if let Some(ref ir_node) = self.ir_node {
            msg.push_str(&format!("\n  IR node: {}", ir_node));
        }

        // Span info (byte offsets for error highlighting)
        if let Some(span) = self.span {
            msg.push_str(&format!("\n  at: bytes {}..{}", span.start, span.end));
        }

        // Help text
        if let Some(ref help) = self.help {
            msg.push_str(&format!("\n  help: {}", help));
        }

        // Source chain
        if let Some(ref source) = self.source {
            msg.push_str(&format!("\n  caused by: {}", source.to_message()));
        }

        msg
    }

    /// Formats the error with source context, showing the problematic line with a caret.
    ///
    /// Output format:
    /// ```text
    /// error: error message
    ///  --> file:line:column
    ///   |
    /// 5 | source line here
    ///   |      ^ found: X
    ///   |
    ///   = help: suggestion
    /// ```
    pub fn format_with_source(&self, source: &str) -> String {
        self.format_with_source_and_file(source, "template", 0)
    }

    /// Formats the error with source context and a custom filename.
    /// `line_offset` is added to convert relative template lines to absolute file lines.
    pub fn format_with_source_and_file(&self, source: &str, filename: &str, line_offset: usize) -> String {
        use crate::compiler::error_fmt::{build_annotation, ErrorFormat};

        // If no span is available, fall back to basic message
        let Some(span) = self.span else {
            return self.to_message();
        };

        let annotation = build_annotation(
            self.found.as_deref(),
            &self.expected.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        );

        let mut fmt = ErrorFormat::new(self.kind.description(), source, span.start)
            .filename(filename)
            .line_offset(line_offset);

        if let Some(ann) = annotation {
            fmt = fmt.annotation(ann);
        }

        if let Some(ref help) = self.help {
            fmt = fmt.help(help);
        }

        fmt.format()
    }
}

impl fmt::Display for GenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_message())
    }
}

impl std::error::Error for GenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

/// Result type for code generation.
pub type GenResult<T> = Result<T, GenError>;

/// Helper to get the variant name of an IrNode for error messages.
/// Uses Debug formatting to extract the variant name without listing all variants.
fn node_variant_name(node: &IrNode) -> String {
    let debug_str = format!("{:?}", node);
    // Extract just the variant name (before any { or ()
    debug_str
        .split(|c| c == '{' || c == '(')
        .next()
        .unwrap_or("Unknown")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unexpected_node_error() {
        let node = IrNode::NumLit { span: IrSpan::empty(), value: "42".to_string() };
        let err = GenError::unexpected_node("statement", &node, &["VarDecl", "FnDecl", "ExprStmt"]);
        let msg = err.to_message();
        assert!(msg.contains("unexpected IR node"));
        assert!(msg.contains("statement"));
        assert!(msg.contains("NumLit"));
    }

    #[test]
    fn test_invalid_placeholder_error() {
        let err = GenError::invalid_placeholder("property name", "Expr", &["Ident"]);
        let msg = err.to_message();
        assert!(msg.contains("invalid placeholder"));
        assert!(msg.contains("property name"));
        assert!(msg.contains("Expr"));
        assert!(msg.contains("Ident"));
    }

    #[test]
    fn test_error_with_source_chain() {
        let inner = GenError::new(GenErrorKind::InvalidNumericLiteral)
            .with_found("123abc");
        let outer = GenError::new(GenErrorKind::ExpressionGenerationFailed)
            .with_context("numeric literal")
            .with_source(inner);
        let msg = outer.to_message();
        assert!(msg.contains("failed to generate expression"));
        assert!(msg.contains("caused by:"));
        assert!(msg.contains("invalid numeric literal"));
    }

    #[test]
    fn test_all_error_kinds_have_descriptions() {
        let kinds = [
            GenErrorKind::UnexpectedIrNode,
            GenErrorKind::MissingRequiredField,
            GenErrorKind::InvalidPlaceholderKind,
            GenErrorKind::ExpressionGenerationFailed,
            GenErrorKind::StatementGenerationFailed,
            GenErrorKind::TypeGenerationFailed,
            GenErrorKind::PatternGenerationFailed,
            GenErrorKind::IdentifierGenerationFailed,
            GenErrorKind::PropertyGenerationFailed,
            GenErrorKind::ParameterGenerationFailed,
            GenErrorKind::ClassMemberGenerationFailed,
            GenErrorKind::InterfaceMemberGenerationFailed,
            GenErrorKind::ModuleItemGenerationFailed,
            GenErrorKind::DeclarationGenerationFailed,
            GenErrorKind::InvalidNumericLiteral,
            GenErrorKind::InvalidBigIntLiteral,
            GenErrorKind::InvalidOperator,
            GenErrorKind::UnsupportedControlFlowPosition,
            GenErrorKind::UnsupportedSpreadPosition,
            GenErrorKind::EmptyBlockNotAllowed,
            GenErrorKind::InvalidEntityName,
            GenErrorKind::InvalidPropertyName,
            GenErrorKind::InvalidMethodKind,
            GenErrorKind::InvalidAccessibility,
            GenErrorKind::InternalError,
        ];

        for kind in kinds {
            let desc = kind.description();
            assert!(!desc.is_empty(), "{:?} has empty description", kind);
        }
    }

    #[test]
    fn test_format_with_source_shows_line_and_caret() {
        let err = GenError::new(GenErrorKind::InvalidNumericLiteral)
            .with_context("numeric literal")
            .with_found("123abc")
            .with_span(IrSpan::new(10, 16)); // Points to "123abc"

        let source = "let x = 123abc;";
        let formatted = err.format_with_source_and_file(source, "test.ts", 0);

        // Should show line number
        assert!(formatted.contains("1 |"), "should show line 1");
        // Should show the source line
        assert!(formatted.contains("let x = 123abc;"), "should show source line");
        // Should show caret pointing to error
        assert!(formatted.contains("^"), "should show caret");
        // Should show filename
        assert!(formatted.contains("test.ts"), "should show filename");
    }

    #[test]
    fn test_format_with_source_multiline() {
        let source = "function foo() {\n  return invalid;\n}";
        // Error at "invalid" which starts at byte 26
        let err = GenError::new(GenErrorKind::UnexpectedIrNode)
            .with_context("return expression")
            .with_found("invalid")
            .with_span(IrSpan::new(26, 33));

        let formatted = err.format_with_source_and_file(source, "test.ts", 0);

        // Should show line 2
        assert!(formatted.contains("2 |"), "should show line 2");
        // Should show the source line with "invalid"
        assert!(formatted.contains("return invalid;"), "should show source line");
    }

    #[test]
    fn test_format_with_source_with_line_offset() {
        let source = "let bad = 42abc;";
        let err = GenError::new(GenErrorKind::InvalidNumericLiteral)
            .with_context("numeric literal")
            .with_span(IrSpan::new(10, 15));

        // With line_offset=9, the error should show as line 10
        let formatted = err.format_with_source_and_file(source, "template", 9);

        assert!(formatted.contains("10 |"), "should show line 10 with offset");
    }

    #[test]
    fn test_format_with_source_caret_at_span_start() {
        let source = "let longIdentifier = bad;";
        // Span covers "longIdentifier" (positions 4-18, 14 chars)
        let err = GenError::new(GenErrorKind::UnexpectedIrNode)
            .with_context("identifier")
            .with_span(IrSpan::new(4, 18));

        let formatted = err.format_with_source_and_file(source, "test.ts", 0);

        // Should have caret pointing to span start
        assert!(formatted.contains("^"), "should show caret at span start");
        // Should show the source line
        assert!(formatted.contains("longIdentifier"), "should show source line");
    }

    #[test]
    fn test_format_without_span_shows_error_only() {
        let err = GenError::new(GenErrorKind::InvalidNumericLiteral)
            .with_context("numeric literal")
            .with_found("bad");

        let source = "let x = bad;";
        let formatted = err.format_with_source_and_file(source, "test.ts", 0);

        // Without span, should not show backtick-wrapped source context
        assert!(!formatted.contains("`"), "should not show backtick-wrapped source without span");
        // But should still show error message
        assert!(formatted.contains("invalid numeric literal"), "should show error message");
    }

    #[test]
    fn test_format_uses_backticks() {
        let err = GenError::new(GenErrorKind::InvalidNumericLiteral)
            .with_context("numeric literal")
            .with_found("bad")
            .with_span(IrSpan::new(8, 11));

        let source = "let x = bad;";
        let formatted = err.format_with_source_and_file(source, "test.ts", 0);

        // Should wrap source and caret lines in backticks
        assert!(formatted.contains("`1 |"), "should have backtick before line number");
        // Count backticks - should have at least 4 (2 per line, 2 lines)
        let backtick_count = formatted.chars().filter(|&c| c == '`').count();
        assert!(backtick_count >= 4, "should have at least 4 backticks, found {}", backtick_count);
        // Verify closing backticks exist (at end of lines)
        assert!(formatted.contains("`\n"), "should have backtick at end of lines");
    }

    #[test]
    fn test_format_long_line_truncation() {
        // Create a line longer than 80 chars with error in the middle
        let long_line = "let veryLongVariableName = someFunction(anotherLongArgument, yetAnotherArgument, andEvenMoreArguments, finalArgument);";
        let err = GenError::new(GenErrorKind::UnexpectedIrNode)
            .with_context("expression")
            .with_span(IrSpan::new(50, 55));

        let formatted = err.format_with_source_and_file(long_line, "test.ts", 0);

        // Should have ellipsis for truncation
        assert!(formatted.contains("..."), "long lines should be truncated with ellipsis, got:\n{}", formatted);
    }
}
