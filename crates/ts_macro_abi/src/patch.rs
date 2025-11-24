#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::SpanIR;

/// Patch-based output = stable "quote" target.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Patch {
    Insert { at: SpanIR, code: String },
    Replace { span: SpanIR, code: String },
    Delete { span: SpanIR },
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default)]
pub struct MacroResult {
    pub patches: Vec<Patch>,
    pub diagnostics: Vec<Diagnostic>,
    pub debug: Option<String>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub span: Option<SpanIR>,
    pub notes: Vec<String>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
}
