//! Error types for code generation.
//!
//! Provides detailed, context-rich error messages for debugging codegen failures.
//! No silent fallbacks - all errors are explicit and actionable.

use crate::compiler::ir::IrNode;
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
        }
    }

    /// Creates an "unexpected IR node" error.
    pub fn unexpected_node(context: &str, node: &IrNode, expected: &[&str]) -> Self {
        Self {
            kind: GenErrorKind::UnexpectedIrNode,
            context: context.to_string(),
            ir_node: Some(format!("{:?}", std::mem::discriminant(node))),
            expected: expected.iter().map(|s| (*s).to_string()).collect(),
            found: Some(node_variant_name(node)),
            help: None,
            source: None,
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
        }
    }

    /// Adds context to the error.
    pub fn with_context(mut self, context: &str) -> Self {
        self.context = context.to_string();
        self
    }

    /// Adds the IR node to the error.
    pub fn with_ir_node(mut self, node: &IrNode) -> Self {
        self.ir_node = Some(format!("{:?}", std::mem::discriminant(node)));
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
        let node = IrNode::NumLit("42".to_string());
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
}
