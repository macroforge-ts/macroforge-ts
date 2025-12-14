use thiserror::Error;
use crate::abi::{Diagnostic, DiagnosticLevel, MacroResult, SpanIR};

#[derive(Error, Debug)]
pub enum TsSynError {
    #[error("parse error: {0}")]
    Parse(String),

    #[error("unsupported TS syntax: {0}")]
    Unsupported(String),
}

#[derive(Debug)]
pub struct MacroforgeError {
    message: String,
    span: Option<SpanIR>,
}

impl MacroforgeError {
    pub fn new(span: SpanIR, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            span: Some(span),
        }
    }

    pub fn new_global(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            span: None,
        }
    }

    pub fn to_diagnostic(self) -> Diagnostic {
        Diagnostic {
            level: DiagnosticLevel::Error,
            message: self.message,
            span: self.span,
            notes: vec![],
            help: None,
        }
    }
}

impl From<MacroforgeError> for MacroResult {
    fn from(err: MacroforgeError) -> Self {
        MacroResult {
            diagnostics: vec![err.to_diagnostic()],
            ..Default::default()
        }
    }
}

impl std::fmt::Display for MacroforgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MacroforgeError {}

/// Error type that can carry multiple diagnostics (e.g., from multiple field validation errors)
#[derive(Debug)]
pub struct MacroforgeErrors {
    pub diagnostics: Vec<Diagnostic>,
}

impl MacroforgeErrors {
    /// Create from a vector of diagnostics
    pub fn new(diagnostics: Vec<Diagnostic>) -> Self {
        Self { diagnostics }
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.level == DiagnosticLevel::Error)
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }
}

impl From<MacroforgeErrors> for MacroResult {
    fn from(err: MacroforgeErrors) -> Self {
        MacroResult {
            diagnostics: err.diagnostics,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for MacroforgeErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_count = self.diagnostics.iter().filter(|d| d.level == DiagnosticLevel::Error).count();
        write!(f, "{} error(s)", error_count)
    }
}

impl std::error::Error for MacroforgeErrors {}

impl From<MacroforgeErrors> for MacroforgeError {
    fn from(errors: MacroforgeErrors) -> Self {
        // Take the first error, noting if there are more
        let first_error = errors.diagnostics.into_iter()
            .find(|d| d.level == DiagnosticLevel::Error);

        if let Some(diag) = first_error {
            MacroforgeError {
                message: diag.message,
                span: diag.span,
            }
        } else {
            MacroforgeError::new_global("Multiple validation errors occurred")
        }
    }
}
