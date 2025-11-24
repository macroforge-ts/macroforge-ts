//! Macro execution context

use crate::{ClassIR, EnumIR, SpanIR};
use serde::{Deserialize, Serialize};

/// The kind of macro being executed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MacroKind {
    /// Derive macro: @Derive(Debug, Clone)
    Derive,
    /// Attribute macro: @log, @sqlTable
    Attribute,
    /// Call macro: macro.sql, Foo!()
    Call,
}

/// Target of a macro application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetIR {
    /// Macro applied to a class
    Class(ClassIR),
    /// Macro applied to an enum
    Enum(EnumIR),
    /// Macro applied to an interface (future)
    Interface,
    /// Macro applied to a function (future)
    Function,
    /// Macro applied to other construct
    Other,
}

/// Context provided to macros during execution
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

    /// Span of the decorator/macro invocation
    pub decorator_span: SpanIR,

    /// Span of the target (class, enum, etc.)
    pub target_span: SpanIR,

    /// The file being processed
    pub file_name: String,

    /// The target of the macro application
    pub target: TargetIR,
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
    ) -> Self {
        Self {
            abi_version: 1,
            macro_kind: MacroKind::Derive,
            macro_name,
            module_path,
            decorator_span,
            target_span,
            file_name,
            target: TargetIR::Class(class),
        }
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
}
