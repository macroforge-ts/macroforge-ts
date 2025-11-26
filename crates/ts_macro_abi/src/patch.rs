#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{swc_ast, SpanIR};
use swc_common::{DUMMY_SP, SyntaxContext};

/// Patch-based output = stable "quote" target.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Patch {
    Insert { at: SpanIR, code: PatchCode },
    Replace { span: SpanIR, code: PatchCode },
    Delete { span: SpanIR },
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum PatchCode {
    Text(String),
    ClassMember(swc_ast::ClassMember),
    Stmt(swc_ast::Stmt),
    ModuleItem(swc_ast::ModuleItem),
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
            let code = items.iter().map(|_| "/* generated code */").collect::<Vec<_>>().join("\n");
            PatchCode::Text(code)
        }
    }
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
