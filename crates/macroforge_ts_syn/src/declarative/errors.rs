//! Errors produced by the declarative macro parser.

use crate::abi::SpanIR;
use crate::errors::MacroforgeError;
use thiserror::Error;

/// Error produced while parsing a declarative macro template body.
///
/// Carries a source span relative to the original file (not the template body),
/// so diagnostics can point at the exact character that triggered the failure.
///
/// Also carries optional structured help — a one-line hint with a
/// concrete suggestion, plus zero or more free-form notes — so that
/// when the error bubbles up to the host [`crate::ts_syn::abi::Diagnostic`]
/// the richer diagnostic fields aren't empty. Use
/// [`DeclarativeError::with_help`] and [`DeclarativeError::with_note`]
/// at the point of error construction to attach these.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("{message}")]
pub struct DeclarativeError {
    /// Human-readable error message.
    pub message: String,
    /// Location of the problem in the source file.
    pub span: SpanIR,
    /// Optional one-line hint explaining how to fix the problem.
    /// Typically phrased as an imperative: "add `expand:` to your
    /// `macroRules({...})` call" or "wrap the statement in an arrow
    /// function". Consumed by the host's `Diagnostic.help` field so
    /// IDE integrations can show it as a structured hint rather than
    /// burying it in the message.
    pub help: Option<String>,
    /// Additional non-fatal notes — extra context, related spans, or
    /// worked examples. Concatenated into the host `Diagnostic.notes`
    /// vector when the error bubbles up.
    pub notes: Vec<String>,
}

impl DeclarativeError {
    pub fn new(span: SpanIR, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            span,
            help: None,
            notes: Vec::new(),
        }
    }

    /// Attach a structured help hint. Chainable at the construction
    /// site: `DeclarativeError::new(span, "missing expand").with_help("add `expand:` ...")`.
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Attach a free-form note. Chainable; multiple `with_note` calls
    /// append to the notes vector in order.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }
}

impl From<DeclarativeError> for MacroforgeError {
    fn from(err: DeclarativeError) -> Self {
        MacroforgeError::new(err.span, err.message)
    }
}
