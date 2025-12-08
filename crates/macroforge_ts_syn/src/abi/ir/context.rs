//! Macro execution context

use crate::abi::{ClassIR, EnumIR, InterfaceIR, SpanIR, TypeAliasIR};
use serde::{Deserialize, Serialize};

/// The kind of macro being executed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MacroKind {
    /// Derive macro: /** @derive(Debug, Clone) */
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
    /// Macro applied to an interface
    Interface(InterfaceIR),
    /// Macro applied to a type alias
    TypeAlias(TypeAliasIR),
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
