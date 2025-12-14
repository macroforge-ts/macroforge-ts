//! Patch-based code modification types.
//!
//! This module provides the types used by macros to express code modifications.
//! Instead of returning complete transformed source code, macros return a list
//! of [`Patch`] operations that describe insertions, replacements, and deletions.
//!
//! ## Why Patches?
//!
//! The patch-based approach has several advantages:
//!
//! 1. **Composable**: Multiple macros can contribute patches to the same file
//! 2. **Precise**: Each patch targets a specific span, preserving surrounding code
//! 3. **Traceable**: Patches can track which macro generated them
//! 4. **Efficient**: Only changed regions need to be processed
//!
//! ## Patch Types
//!
//! | Variant | Description |
//! |---------|-------------|
//! | [`Patch::Insert`] | Insert code at a position (zero-width span) |
//! | [`Patch::Replace`] | Replace code in a span with new code |
//! | [`Patch::Delete`] | Remove code in a span |
//! | [`Patch::InsertRaw`] | Insert raw text with optional context |
//! | [`Patch::ReplaceRaw`] | Replace with raw text and context |
//!
//! ## Example
//!
//! ```rust,ignore
//! use macroforge_ts_syn::{Patch, PatchCode, SpanIR, MacroResult};
//!
//! fn add_method_to_class(body_span: SpanIR) -> MacroResult {
//!     // Insert a method just before the closing brace
//!     let insert_point = SpanIR::new(body_span.end - 1, body_span.end - 1);
//!
//!     let patch = Patch::Insert {
//!         at: insert_point,
//!         code: "toString() { return 'MyClass'; }".into(),
//!         source_macro: Some("Debug".to_string()),
//!     };
//!
//!     MacroResult {
//!         runtime_patches: vec![patch],
//!         ..Default::default()
//!     }
//! }
//! ```

use serde::{Deserialize, Serialize};

use crate::abi::{SpanIR, swc_ast};
#[cfg(feature = "swc")]
use swc_core::common::{DUMMY_SP, SyntaxContext};

/// A code modification operation returned by macros.
///
/// Patches describe how to transform the original source code. They are
/// applied in order by the macro host to produce the final output.
///
/// # Source Macro Tracking
///
/// Patches can optionally track which macro generated them via the
/// `source_macro` field. This is useful for:
/// - Debugging macro output
/// - Source mapping in generated code
/// - Error attribution
///
/// Use [`with_source_macro()`](Self::with_source_macro) to set this field.
///
/// # Example
///
/// ```rust,ignore
/// use macroforge_ts_syn::{Patch, SpanIR};
///
/// // Insert new code
/// let insert = Patch::Insert {
///     at: SpanIR::new(100, 100),  // Zero-width span = insertion point
///     code: "// generated code".into(),
///     source_macro: Some("MyMacro".to_string()),
/// };
///
/// // Replace existing code
/// let replace = Patch::Replace {
///     span: SpanIR::new(50, 75),  // The span to replace
///     code: "newCode()".into(),
///     source_macro: None,
/// };
///
/// // Delete code
/// let delete = Patch::Delete {
///     span: SpanIR::new(200, 250),
/// };
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Patch {
    /// Insert code at a specific position.
    ///
    /// The `at` span should typically be zero-width (`start == end`)
    /// to mark an insertion point. The code is inserted at that position.
    Insert {
        /// The position to insert at (use zero-width span for pure insertion).
        at: SpanIR,
        /// The code to insert.
        code: PatchCode,
        /// Which macro generated this patch (e.g., "Debug", "Clone").
        #[serde(default)]
        source_macro: Option<String>,
    },

    /// Replace code in a span with new code.
    ///
    /// The original code in `span` is removed and replaced with `code`.
    Replace {
        /// The span of code to replace.
        span: SpanIR,
        /// The replacement code.
        code: PatchCode,
        /// Which macro generated this patch.
        #[serde(default)]
        source_macro: Option<String>,
    },

    /// Delete code in a span.
    ///
    /// The code in `span` is removed entirely.
    Delete {
        /// The span of code to delete.
        span: SpanIR,
    },

    /// Insert raw text with optional context.
    ///
    /// Similar to `Insert`, but takes a raw string and optional context
    /// for more control over formatting.
    InsertRaw {
        /// The position to insert at.
        at: SpanIR,
        /// The raw code string to insert.
        code: String,
        /// Optional context hint (e.g., "class_body", "module_level").
        context: Option<String>,
        /// Which macro generated this patch.
        #[serde(default)]
        source_macro: Option<String>,
    },

    /// Replace code with raw text and optional context.
    ///
    /// Similar to `Replace`, but takes a raw string and optional context.
    ReplaceRaw {
        /// The span of code to replace.
        span: SpanIR,
        /// The raw replacement code string.
        code: String,
        /// Optional context hint for formatting.
        context: Option<String>,
        /// Which macro generated this patch.
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
            Patch::InsertRaw {
                at, code, context, ..
            } => Patch::InsertRaw {
                at,
                code,
                context,
                source_macro: Some(macro_name.to_string()),
            },
            Patch::ReplaceRaw {
                span,
                code,
                context,
                ..
            } => Patch::ReplaceRaw {
                span,
                code,
                context,
                source_macro: Some(macro_name.to_string()),
            },
        }
    }
}

/// The code content of a patch, supporting both text and AST representations.
///
/// `PatchCode` can hold code in different forms:
///
/// - **Text**: Raw source code strings (serializable)
/// - **AST nodes**: SWC AST types for programmatic construction (not serializable)
///
/// When serialized, AST variants are converted to placeholder text since
/// SWC AST nodes are not directly serializable.
///
/// # Example
///
/// ```rust,ignore
/// use macroforge_ts_syn::PatchCode;
///
/// // From a string
/// let text_code: PatchCode = "myMethod() {}".into();
///
/// // From an AST node
/// let ast_code: PatchCode = some_class_member.into();
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum PatchCode {
    /// Raw source code as text.
    ///
    /// This is the most portable form, as it can be serialized
    /// and processed by any consumer.
    Text(String),

    /// An SWC class member AST node.
    ///
    /// Use this when constructing class members programmatically
    /// using SWC's AST types or the `quote!` macro.
    ClassMember(swc_ast::ClassMember),

    /// An SWC statement AST node.
    ///
    /// Use for standalone statements or function/method bodies.
    Stmt(swc_ast::Stmt),

    /// An SWC module item AST node.
    ///
    /// Use for top-level declarations like imports, exports,
    /// or function/class declarations.
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

/// The result returned by a macro function.
///
/// `MacroResult` is the standard return type for all macro functions.
/// It contains patches to apply to the code, diagnostic messages,
/// and optional debug information.
///
/// # Patch Separation
///
/// Patches are separated into two categories:
///
/// - `runtime_patches`: Applied to the `.js`/`.ts` runtime code
/// - `type_patches`: Applied to `.d.ts` type declaration files
///
/// This separation allows macros to generate different code for
/// runtime and type-checking contexts.
///
/// # Example
///
/// ```rust,ignore
/// use macroforge_ts_syn::{MacroResult, Patch, Diagnostic, DiagnosticLevel};
///
/// fn my_macro() -> MacroResult {
///     // Success with patches
///     MacroResult {
///         runtime_patches: vec![/* ... */],
///         type_patches: vec![],
///         diagnostics: vec![],
///         tokens: None,
///         debug: Some("Generated 2 methods".to_string()),
///     }
/// }
///
/// fn my_macro_error() -> MacroResult {
///     // Error result
///     MacroResult {
///         diagnostics: vec![Diagnostic {
///             level: DiagnosticLevel::Error,
///             message: "Invalid input".to_string(),
///             span: None,
///             notes: vec![],
///             help: Some("Try using @derive(Debug) instead".to_string()),
///         }],
///         ..Default::default()
///     }
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MacroResult {
    /// Patches to apply to the runtime JS/TS code.
    pub runtime_patches: Vec<Patch>,

    /// Patches to apply to the `.d.ts` type declarations.
    pub type_patches: Vec<Patch>,

    /// Diagnostic messages (errors, warnings, info).
    /// If any error diagnostics are present, the macro expansion is considered failed.
    pub diagnostics: Vec<Diagnostic>,

    /// Optional raw token stream (source code) returned by the macro.
    /// Used for macros that generate complete output rather than patches.
    pub tokens: Option<String>,

    /// Optional debug information for development.
    /// Can be displayed in verbose mode or logged for debugging.
    pub debug: Option<String>,
}

/// A diagnostic message from macro expansion.
///
/// Diagnostics provide feedback about the macro expansion process,
/// including errors, warnings, and informational messages. Each
/// diagnostic can include a source span, additional notes, and
/// help text.
///
/// # Example
///
/// ```rust,ignore
/// use macroforge_ts_syn::{Diagnostic, DiagnosticLevel, SpanIR};
///
/// let error = Diagnostic {
///     level: DiagnosticLevel::Error,
///     message: "Field 'password' cannot be serialized".to_string(),
///     span: Some(SpanIR::new(100, 115)),
///     notes: vec!["Sensitive fields should use @serde(skip)".to_string()],
///     help: Some("Add @serde(skip) decorator to this field".to_string()),
/// };
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Diagnostic {
    /// The severity level of the diagnostic.
    pub level: DiagnosticLevel,

    /// The main diagnostic message.
    pub message: String,

    /// Optional source span where the issue occurred.
    /// If `None`, the diagnostic applies to the whole macro invocation.
    pub span: Option<SpanIR>,

    /// Additional notes providing context.
    pub notes: Vec<String>,

    /// Optional help text suggesting how to fix the issue.
    pub help: Option<String>,
}

/// The severity level of a diagnostic message.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticLevel {
    /// An error that prevents successful macro expansion.
    /// Any error diagnostic causes the macro result to be considered failed.
    Error,

    /// A warning that doesn't prevent expansion but indicates potential issues.
    Warning,

    /// Informational message for debugging or user guidance.
    Info,
}

/// A builder for collecting diagnostics during macro expansion.
///
/// `DiagnosticCollector` provides a convenient way to accumulate
/// multiple diagnostics and then convert them to a `Vec<Diagnostic>`
/// for inclusion in a `MacroResult`.
///
/// # Example
///
/// ```rust,ignore
/// use macroforge_ts_syn::{DiagnosticCollector, SpanIR};
///
/// let mut collector = DiagnosticCollector::new();
///
/// // Add diagnostics as you encounter issues
/// for field in fields {
///     if field.ts_type == "any" {
///         collector.warning(field.span, "Avoid using 'any' type");
///     }
/// }
///
/// // Check for errors before continuing
/// if collector.has_errors() {
///     return MacroResult {
///         diagnostics: collector.into_vec(),
///         ..Default::default()
///     };
/// }
/// ```
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
    pub fn error_with_help(
        &mut self,
        span: SpanIR,
        message: impl Into<String>,
        help: impl Into<String>,
    ) {
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
        self.diagnostics
            .iter()
            .any(|d| d.level == DiagnosticLevel::Error)
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
