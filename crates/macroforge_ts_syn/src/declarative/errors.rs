//! Errors produced by the declarative macro parser.

use crate::abi::SpanIR;
use crate::errors::MacroforgeError;
use thiserror::Error;

/// Error produced while parsing a declarative macro template body.
///
/// Carries a source span relative to the original file (not the template body),
/// so diagnostics can point at the exact character that triggered the failure.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("{message}")]
pub struct DeclarativeError {
    /// Human-readable error message.
    pub message: String,
    /// Location of the problem in the source file.
    pub span: SpanIR,
}

impl DeclarativeError {
    pub fn new(span: SpanIR, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }
}

impl From<DeclarativeError> for MacroforgeError {
    fn from(err: DeclarativeError) -> Self {
        MacroforgeError::new(err.span, err.message)
    }
}
