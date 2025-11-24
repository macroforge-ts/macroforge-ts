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
    /// Patches to apply to the runtime JS/TS code
    pub runtime_patches: Vec<Patch>,
    /// Patches to apply to the .d.ts type declarations
    pub type_patches: Vec<Patch>,
    /// Diagnostic messages (errors, warnings, info)
    pub diagnostics: Vec<Diagnostic>,
    /// Optional debug information for development
    pub debug: Option<String>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub span: Option<SpanIR>,
    /// Additional notes about the diagnostic
    pub notes: Vec<String>,
    /// Optional help text suggesting fixes
    pub help: Option<String>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
}
