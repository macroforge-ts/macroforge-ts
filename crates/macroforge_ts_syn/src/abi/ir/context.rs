//! Macro execution context types.
//!
//! This module provides the context that is passed to macro functions during
//! execution. The [`MacroContextIR`] contains all the information a macro needs
//! to process its input and generate output.
//!
//! ## Context Flow
//!
//! ```text
//! TypeScript Source
//!        │
//!        ▼
//! ┌─────────────────┐
//! │  Parse & Lower  │  (SWC parser → IR types)
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │ MacroContextIR  │  (Serialized to macro)
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │  Macro Function │  (Your code!)
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │   MacroResult   │  (Patches to apply)
//! └─────────────────┘
//! ```
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use macroforge_ts_syn::{MacroContextIR, MacroResult, DeriveInput};
//!
//! pub fn my_derive_macro(ctx: MacroContextIR) -> MacroResult {
//!     // Access macro metadata
//!     println!("Macro: {} from {}", ctx.macro_name, ctx.module_path);
//!     println!("File: {}", ctx.file_name);
//!
//!     // Work with the target
//!     if let Some(class) = ctx.as_class() {
//!         println!("Processing class: {}", class.name);
//!     }
//!
//!     MacroResult::ok()
//! }
//! ```

use crate::abi::{ClassIR, EnumIR, InterfaceIR, SpanIR, TypeAliasIR};
use serde::{Deserialize, Serialize};

/// The kind of macro being executed.
///
/// Different macro kinds have different invocation syntax and capabilities:
///
/// - **Derive**: Attached to types, generates additional code alongside the type
/// - **Attribute**: Attached to declarations, can transform the declaration
/// - **Call**: Invoked inline, generates code at the call site
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MacroKind {
    /// A derive macro attached via JSDoc.
    ///
    /// Example: `/** @derive(Debug, Clone) */`
    ///
    /// Derive macros receive the type definition and generate additional
    /// code (methods, traits, etc.) without modifying the original type.
    Derive,

    /// An attribute macro attached to a declaration.
    ///
    /// Example: `@log`, `@sqlTable("users")`
    ///
    /// Attribute macros can transform or augment the target declaration.
    Attribute,

    /// A call macro invoked inline.
    ///
    /// Example: `sql!("SELECT * FROM users")`, `html!(<div>...</div>)`
    ///
    /// Call macros generate code at the invocation site.
    Call,
}

/// The target declaration of a macro application.
///
/// Wraps the IR type of the declaration that the macro is applied to,
/// allowing macros to handle different target types uniformly.
///
/// # Example
///
/// ```rust,ignore
/// use macroforge_ts_syn::TargetIR;
///
/// fn get_target_name(target: &TargetIR) -> &str {
///     match target {
///         TargetIR::Class(c) => &c.name,
///         TargetIR::Interface(i) => &i.name,
///         TargetIR::Enum(e) => &e.name,
///         TargetIR::TypeAlias(t) => &t.name,
///         _ => "<unknown>",
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetIR {
    /// Macro applied to a class declaration.
    Class(ClassIR),

    /// Macro applied to an enum declaration.
    Enum(EnumIR),

    /// Macro applied to an interface declaration.
    Interface(InterfaceIR),

    /// Macro applied to a type alias declaration.
    TypeAlias(TypeAliasIR),

    /// Macro applied to a function (reserved for future use).
    Function,

    /// Macro applied to an unsupported construct.
    Other,
}

/// Context provided to macros during execution.
///
/// This is the primary input to all macro functions. It contains:
/// - Metadata about the macro invocation (name, kind, source location)
/// - The target declaration ([`TargetIR`]) the macro is applied to
/// - Source spans for accurate error reporting
/// - The raw source code of the target for custom parsing
///
/// # ABI Stability
///
/// The `abi_version` field allows for backwards compatibility checking.
/// Macros can verify they're compatible with the runtime version.
///
/// # Error Reporting
///
/// Use [`error_span()`](Self::error_span) to get the best span for error
/// messages. It prefers `macro_name_span` (pointing to the specific macro)
/// over `decorator_span` (pointing to the entire decorator).
///
/// # Example
///
/// ```rust,ignore
/// use macroforge_ts_syn::{MacroContextIR, MacroResult, Patch};
///
/// pub fn debug_derive(ctx: MacroContextIR) -> MacroResult {
///     let class = ctx.as_class().expect("Debug requires a class");
///
///     // Generate a debug method
///     let method_code = format!(
///         r#"debug(): string {{
///             return "{}({{}})";
///         }}"#,
///         class.name
///     );
///
///     // Insert at end of class body
///     MacroResult::ok().with_patches(vec![
///         Patch::insert_into_class(class.body_span, method_code)
///     ])
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroContextIR {
    /// ABI version for compatibility checking
    pub abi_version: u32,

    /// The kind of macro being executed
    pub macro_kind: MacroKind,

    /// The name of the macro (e.g., "Debug")
    pub macro_name: String,

    /// The module path the macro comes from (e.g., "@macro/derive")
    pub module_path: String,

    /// Span of the decorator/macro invocation (entire @derive(...))
    pub decorator_span: SpanIR,

    /// Span of just the macro name within the decorator (e.g., "Debug" in @derive(Debug))
    /// Used for error reporting to point to the specific macro that caused the error
    #[serde(default)]
    pub macro_name_span: Option<SpanIR>,

    /// Span of the target (class, enum, etc.)
    pub target_span: SpanIR,

    /// The file being processed
    pub file_name: String,

    /// The target of the macro application
    pub target: TargetIR,

    /// The source code of the target (class, enum, etc.)
    /// This enables macros to parse the source themselves using TsStream
    pub target_source: String,
}

impl MacroContextIR {
    /// Create a new macro context for a derive macro on a class
    pub fn new_derive_class(
        macro_name: String,
        module_path: String,
        decorator_span: SpanIR,
        target_span: SpanIR,
        file_name: String,
        class: ClassIR,
        target_source: String,
    ) -> Self {
        Self {
            abi_version: 1,
            macro_kind: MacroKind::Derive,
            macro_name,
            module_path,
            decorator_span,
            macro_name_span: None,
            target_span,
            file_name,
            target: TargetIR::Class(class),
            target_source,
        }
    }

    /// Set the macro name span (builder pattern)
    pub fn with_macro_name_span(mut self, span: SpanIR) -> Self {
        self.macro_name_span = Some(span);
        self
    }

    /// Get the best span for error reporting - prefers macro_name_span if available
    pub fn error_span(&self) -> SpanIR {
        self.macro_name_span.unwrap_or(self.decorator_span)
    }

    /// Get the class IR if the target is a class
    pub fn as_class(&self) -> Option<&ClassIR> {
        match &self.target {
            TargetIR::Class(class) => Some(class),
            _ => None,
        }
    }

    /// Get the enum IR if the target is an enum
    pub fn as_enum(&self) -> Option<&EnumIR> {
        match &self.target {
            TargetIR::Enum(enum_ir) => Some(enum_ir),
            _ => None,
        }
    }

    /// Get the interface IR if the target is an interface
    pub fn as_interface(&self) -> Option<&InterfaceIR> {
        match &self.target {
            TargetIR::Interface(interface_ir) => Some(interface_ir),
            _ => None,
        }
    }

    /// Get the type alias IR if the target is a type alias
    pub fn as_type_alias(&self) -> Option<&TypeAliasIR> {
        match &self.target {
            TargetIR::TypeAlias(type_alias_ir) => Some(type_alias_ir),
            _ => None,
        }
    }

    /// Create a new macro context for a derive macro on an interface
    pub fn new_derive_interface(
        macro_name: String,
        module_path: String,
        decorator_span: SpanIR,
        target_span: SpanIR,
        file_name: String,
        interface: InterfaceIR,
        target_source: String,
    ) -> Self {
        Self {
            abi_version: 1,
            macro_kind: MacroKind::Derive,
            macro_name,
            module_path,
            decorator_span,
            macro_name_span: None,
            target_span,
            file_name,
            target: TargetIR::Interface(interface),
            target_source,
        }
    }

    /// Create a new macro context for a derive macro on a type alias
    pub fn new_derive_type_alias(
        macro_name: String,
        module_path: String,
        decorator_span: SpanIR,
        target_span: SpanIR,
        file_name: String,
        type_alias: TypeAliasIR,
        target_source: String,
    ) -> Self {
        Self {
            abi_version: 1,
            macro_kind: MacroKind::Derive,
            macro_name,
            module_path,
            decorator_span,
            macro_name_span: None,
            target_span,
            file_name,
            target: TargetIR::TypeAlias(type_alias),
            target_source,
        }
    }

    /// Create a new macro context for a derive macro on an enum
    pub fn new_derive_enum(
        macro_name: String,
        module_path: String,
        decorator_span: SpanIR,
        target_span: SpanIR,
        file_name: String,
        enum_ir: EnumIR,
        target_source: String,
    ) -> Self {
        Self {
            abi_version: 1,
            macro_kind: MacroKind::Derive,
            macro_name,
            module_path,
            decorator_span,
            macro_name_span: None,
            target_span,
            file_name,
            target: TargetIR::Enum(enum_ir),
            target_source,
        }
    }
}
