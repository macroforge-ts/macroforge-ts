use serde::{Deserialize, Serialize};

use crate::abi::{swc_ast, SpanIR};
#[cfg(feature = "swc")]
use swc_core::common::{SyntaxContext, DUMMY_SP};

/// Patch-based output = stable "quote" target.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq)]
pub enum Patch {
    Insert {
        at: SpanIR,
        code: PatchCode,
        /// Which macro generated this patch (e.g., "Debug", "Clone")
        #[serde(default)]
        source_macro: Option<String>,
    },
    Replace {
        span: SpanIR,
        code: PatchCode,
        /// Which macro generated this patch (e.g., "Debug", "Clone")
        #[serde(default)]
        source_macro: Option<String>,
    },
    Delete {
        span: SpanIR,
    },
    InsertRaw {
        at: SpanIR,
        code: String,
        context: Option<String>,
        /// Which macro generated this patch (e.g., "Debug", "Clone")
        #[serde(default)]
        source_macro: Option<String>,
    },
    ReplaceRaw {
        span: SpanIR,
        code: String,
        context: Option<String>,
        /// Which macro generated this patch (e.g., "Debug", "Clone")
        #[serde(default)]
        source_macro: Option<String>,
    },
}

impl Patch {
    /// Get the source macro name for this patch, if set
    pub fn source_macro(&self) -> Option<&str> {
        match self {
            Patch::Insert { source_macro, .. } => source_macro.as_deref(),
            Patch::Replace { source_macro, .. } => source_macro.as_deref(),
            Patch::Delete { .. } => None,
            Patch::InsertRaw { source_macro, .. } => source_macro.as_deref(),
            Patch::ReplaceRaw { source_macro, .. } => source_macro.as_deref(),
        }
    }

    /// Set the source macro name for this patch
    pub fn with_source_macro(self, macro_name: &str) -> Self {
        match self {
            Patch::Insert { at, code, .. } => Patch::Insert {
                at,
                code,
                source_macro: Some(macro_name.to_string()),
            },
            Patch::Replace { span, code, .. } => Patch::Replace {
                span,
                code,
                source_macro: Some(macro_name.to_string()),
            },
            Patch::Delete { span } => Patch::Delete { span },
            Patch::InsertRaw { at, code, context, .. } => Patch::InsertRaw {
                at,
                code,
                context,
                source_macro: Some(macro_name.to_string()),
            },
            Patch::ReplaceRaw { span, code, context, .. } => Patch::ReplaceRaw {
                span,
                code,
                context,
                source_macro: Some(macro_name.to_string()),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PatchCode {
    Text(String),
    ClassMember(swc_ast::ClassMember),
    Stmt(swc_ast::Stmt),
    ModuleItem(swc_ast::ModuleItem),
}

// Custom serde for PatchCode - only serialize Text variant, skip AST variants
impl serde::Serialize for PatchCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            PatchCode::Text(s) => serializer.serialize_str(s),
            _ => serializer.serialize_str("/* AST node - cannot serialize */"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for PatchCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(PatchCode::Text(s))
    }
}

impl From<String> for PatchCode {
    fn from(value: String) -> Self {
        PatchCode::Text(value)
    }
}

impl From<&str> for PatchCode {
    fn from(value: &str) -> Self {
        PatchCode::Text(value.to_string())
    }
}

impl From<swc_ast::ClassMember> for PatchCode {
    fn from(member: swc_ast::ClassMember) -> Self {
        PatchCode::ClassMember(member)
    }
}

impl From<swc_ast::Stmt> for PatchCode {
    fn from(stmt: swc_ast::Stmt) -> Self {
        PatchCode::Stmt(stmt)
    }
}

impl From<swc_ast::ModuleItem> for PatchCode {
    fn from(item: swc_ast::ModuleItem) -> Self {
        PatchCode::ModuleItem(item)
    }
}

impl From<Vec<swc_ast::Stmt>> for PatchCode {
    fn from(stmts: Vec<swc_ast::Stmt>) -> Self {
        // For Vec<Stmt>, wrap in a block and convert to a single Stmt
        if stmts.len() == 1 {
            PatchCode::Stmt(stmts.into_iter().next().unwrap())
        } else {
            PatchCode::Stmt(swc_ast::Stmt::Block(swc_ast::BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts,
            }))
        }
    }
}

impl From<Vec<swc_ast::ModuleItem>> for PatchCode {
    fn from(items: Vec<swc_ast::ModuleItem>) -> Self {
        // For Vec<ModuleItem>, take the first if there's only one
        if items.len() == 1 {
            PatchCode::ModuleItem(items.into_iter().next().unwrap())
        } else {
            // Multiple items - convert to a string representation
            // This is a limitation since PatchCode doesn't have a Vec variant
            let code = items
                .iter()
                .map(|_| "/* generated code */")
                .collect::<Vec<_>>()
                .join("\n");
            PatchCode::Text(code)
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Default)]
pub struct MacroResult {
    /// Patches to apply to the runtime JS/TS code
    pub runtime_patches: Vec<Patch>,
    /// Patches to apply to the .d.ts type declarations
    pub type_patches: Vec<Patch>,
    /// Diagnostic messages (errors, warnings, info)
    pub diagnostics: Vec<Diagnostic>,
    /// Optional raw token stream (source code) returned by the macro
    pub tokens: Option<String>,
    /// Optional debug information for development
    pub debug: Option<String>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
}

/// Collects diagnostics during macro expansion with field/decorator context
#[derive(Default, Clone, Debug)]
pub struct DiagnosticCollector {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticCollector {
    /// Create a new empty collector
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a diagnostic to the collection
    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Add an error diagnostic with span
    pub fn error(&mut self, span: SpanIR, message: impl Into<String>) {
        self.push(Diagnostic {
            level: DiagnosticLevel::Error,
            message: message.into(),
            span: Some(span),
            notes: vec![],
            help: None,
        });
    }

    /// Add an error diagnostic with span and help text
    pub fn error_with_help(&mut self, span: SpanIR, message: impl Into<String>, help: impl Into<String>) {
        self.push(Diagnostic {
            level: DiagnosticLevel::Error,
            message: message.into(),
            span: Some(span),
            notes: vec![],
            help: Some(help.into()),
        });
    }

    /// Add a warning diagnostic with span
    pub fn warning(&mut self, span: SpanIR, message: impl Into<String>) {
        self.push(Diagnostic {
            level: DiagnosticLevel::Warning,
            message: message.into(),
            span: Some(span),
            notes: vec![],
            help: None,
        });
    }

    /// Merge diagnostics from another collector
    pub fn extend(&mut self, other: DiagnosticCollector) {
        self.diagnostics.extend(other.diagnostics);
    }

    /// Check if there are any errors in the collection
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.level == DiagnosticLevel::Error)
    }

    /// Check if the collection is empty
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Get the number of diagnostics
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Convert to a Vec of Diagnostics
    pub fn into_vec(self) -> Vec<Diagnostic> {
        self.diagnostics
    }

    /// Get a reference to the diagnostics
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}
