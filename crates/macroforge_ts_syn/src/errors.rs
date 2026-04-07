//! Error types for the macro system.
//!
//! This module provides error types for handling failures in parsing,
//! macro expansion, and validation. These errors integrate with the
//! macro result system to provide rich diagnostics to users.
//!
//! ## Error Types
//!
//! | Type | Use Case |
//! |------|----------|
//! | [`TsSynError`] | Low-level parsing and syntax errors |
//! | [`MacroforgeError`] | Single-error macro failures with span |
//! | [`MacroforgeErrors`] | Multiple errors (e.g., validation failures) |
//!
//! ## Usage Patterns
//!
//! ### Returning Errors from Macros
//!
//! ```rust
//! use macroforge_ts_syn::{MacroforgeError, MacroResult, DeriveInput, SpanIR};
//!
//! fn my_macro(input: &DeriveInput, some_condition: bool, other_condition: bool) -> Result<MacroResult, MacroforgeError> {
//!     // Error with source span
//!     if some_condition {
//!         return Err(MacroforgeError::new(
//!             input.error_span(),
//!             "Unsupported type for this macro"
//!         ));
//!     }
//!
//!     // Global error (no specific span)
//!     if other_condition {
//!         return Err(MacroforgeError::new_global("Configuration error"));
//!     }
//!
//!     Ok(MacroResult::default())
//! }
//! ```
//!
//! ### Converting Errors to MacroResult
//!
//! Both [`MacroforgeError`] and [`MacroforgeErrors`] implement `Into<MacroResult>`,
//! allowing direct conversion:
//!
//! ```rust
//! use macroforge_ts_syn::{MacroforgeError, MacroforgeErrors, MacroResult, Diagnostic, DiagnosticLevel, SpanIR};
//!
//! // Single error
//! let span = SpanIR::new(0, 10);
//! let error = MacroforgeError::new(span, "Something went wrong");
//! let _result: MacroResult = error.into();
//!
//! // Multiple errors
//! let diagnostics = vec![Diagnostic {
//!     level: DiagnosticLevel::Error,
//!     message: "First error".into(),
//!     span: None,
//!     notes: vec![],
//!     help: None,
//! }];
//! let errors = MacroforgeErrors::new(diagnostics);
//! let _result: MacroResult = errors.into();
//! ```

use crate::abi::{Diagnostic, DiagnosticLevel, MacroResult, SpanIR};
use thiserror::Error;

/// Low-level error type for parsing and syntax issues.
///
/// This error type is used for failures in the parsing layer, before
/// macro logic runs. For errors in macro implementations, prefer
/// [`MacroforgeError`].
///
/// # Variants
///
/// - `Parse` - SWC parsing failures (syntax errors)
/// - `Unsupported` - Valid syntax that isn't supported by the lowering layer
///
/// # Example
///
/// ```rust
/// use macroforge_ts_syn::TsSynError;
///
/// fn check_feature(is_supported: bool) -> Result<(), TsSynError> {
///     if !is_supported {
///         return Err(TsSynError::Unsupported(
///             "Async generators are not supported".into()
///         ));
///     }
///     Ok(())
/// }
/// ```
#[derive(Error, Debug)]
pub enum TsSynError {
    /// A parsing error from SWC.
    #[error("parse error: {0}")]
    Parse(String),

    /// Valid TypeScript syntax that isn't supported by the macro system.
    #[error("unsupported TS syntax: {0}")]
    Unsupported(String),
}

/// A macro error with an optional source span.
///
/// This is the primary error type for macro implementations. It carries
/// an error message and optionally a source span for precise error reporting.
/// When converted to a [`MacroResult`], it becomes an error-level diagnostic.
///
/// # Creating Errors
///
/// ```rust
/// use macroforge_ts_syn::{MacroforgeError, SpanIR};
///
/// // With a span (preferred for user-facing errors)
/// let field_span = SpanIR::new(10, 25);
/// let field_name = "email";
/// let _error = MacroforgeError::new(
///     field_span,
///     format!("Field '{}' has unsupported type", field_name)
/// );
///
/// // Without a span (for internal/configuration errors)
/// let _error = MacroforgeError::new_global("Missing required configuration");
/// ```
///
/// # Converting to MacroResult
///
/// `MacroforgeError` implements `Into<MacroResult>` for easy error returns:
///
/// ```rust
/// use macroforge_ts_syn::{MacroforgeError, MacroResult, SpanIR};
///
/// fn my_macro(has_error: bool) -> Result<MacroResult, MacroforgeError> {
///     if has_error {
///         let span = SpanIR::new(0, 10);
///         return Err(MacroforgeError::new(span, "Something went wrong"));
///     }
///     Ok(MacroResult::default())
/// }
///
/// // Or convert directly:
/// let _result: MacroResult = MacroforgeError::new_global("Error").into();
/// ```
#[derive(Debug)]
pub struct MacroforgeError {
    message: String,
    span: Option<SpanIR>,
}

impl MacroforgeError {
    /// Creates a new error with a source span.
    ///
    /// Use this for errors that can be traced to a specific location in the
    /// source code. The span will be used for error highlighting.
    ///
    /// # Arguments
    ///
    /// - `span` - The source span where the error occurred
    /// - `message` - A human-readable error message
    pub fn new(span: SpanIR, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            span: Some(span),
        }
    }

    /// Creates a new error without a source span.
    ///
    /// Use this for errors that aren't tied to a specific source location,
    /// such as configuration errors or internal failures.
    ///
    /// # Arguments
    ///
    /// - `message` - A human-readable error message
    pub fn new_global(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            span: None,
        }
    }

    /// Converts this error into a [`Diagnostic`].
    ///
    /// The resulting diagnostic has [`DiagnosticLevel::Error`] and includes
    /// the message and span from this error.
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

/// A collection of multiple diagnostics, for reporting multiple errors at once.
///
/// Use this when a macro needs to report multiple errors simultaneously,
/// such as validating all fields in a class and collecting all validation
/// failures. This provides a better user experience than failing on the
/// first error, as users can fix all issues in one iteration.
///
/// # Example: Field Validation
///
/// ```rust
/// use macroforge_ts_syn::{MacroforgeErrors, Diagnostic, DiagnosticLevel, FieldIR, SpanIR, Visibility};
///
/// fn validate_fields(fields: &[FieldIR]) -> Result<(), MacroforgeErrors> {
///     let mut diagnostics = Vec::new();
///
///     for field in fields {
///         if field.ts_type == "any" {
///             diagnostics.push(Diagnostic {
///                 level: DiagnosticLevel::Error,
///                 message: format!("Field '{}' cannot use 'any' type", field.name),
///                 span: Some(field.span),
///                 notes: vec![],
///                 help: Some("Use a specific type instead".into()),
///             });
///         }
///     }
///
///     if diagnostics.is_empty() {
///         Ok(())
///     } else {
///         Err(MacroforgeErrors::new(diagnostics))
///     }
/// }
/// ```
///
/// # Converting to MacroResult
///
/// ```rust
/// use macroforge_ts_syn::{MacroforgeErrors, MacroResult, Diagnostic, DiagnosticLevel};
///
/// let diagnostics = vec![Diagnostic {
///     level: DiagnosticLevel::Error,
///     message: "Some error".into(),
///     span: None,
///     notes: vec![],
///     help: None,
/// }];
/// let errors = MacroforgeErrors::new(diagnostics);
/// let _result: MacroResult = errors.into();
/// // result.diagnostics contains all the errors
/// ```
#[derive(Debug)]
pub struct MacroforgeErrors {
    /// The collected diagnostics (errors, warnings, etc.).
    pub diagnostics: Vec<Diagnostic>,
}

impl MacroforgeErrors {
    /// Creates a new error collection from a vector of diagnostics.
    ///
    /// # Arguments
    ///
    /// - `diagnostics` - The diagnostics to include in this error
    pub fn new(diagnostics: Vec<Diagnostic>) -> Self {
        Self { diagnostics }
    }

    /// Returns `true` if any diagnostic has error level.
    ///
    /// Use this to check if the collection contains actual errors
    /// vs only warnings or info messages.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.level == DiagnosticLevel::Error)
    }

    /// Returns `true` if the collection contains no diagnostics.
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
        let error_count = self
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        write!(f, "{} error(s)", error_count)
    }
}

impl std::error::Error for MacroforgeErrors {}

impl From<MacroforgeErrors> for MacroforgeError {
    fn from(errors: MacroforgeErrors) -> Self {
        // Take the first error, noting if there are more
        let first_error = errors
            .diagnostics
            .into_iter()
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
