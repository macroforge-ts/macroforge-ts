use thiserror::Error;
use ts_macro_abi::{Diagnostic, DiagnosticLevel, MacroResult, SpanIR};

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
