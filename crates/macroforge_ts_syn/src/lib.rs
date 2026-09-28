//! # macroforge_ts_syn
//!
//! TypeScript syntax types for build-time macro code generation.
//!
//! This crate provides a [`syn`](https://docs.rs/syn)-like API for parsing and manipulating
//! TypeScript code, enabling macro authors to work with TypeScript AST in a familiar way.
//! It is the core infrastructure crate for the Macroforge TypeScript macro system.
//!
//! ## Overview
//!
//! The crate is organized into several modules:
//!
//! - [`abi`] - Application Binary Interface types for stable macro communication
//! - [`ast`] - Source-backed expression and identifier values for templates
//! - [`config`] - Serializable configuration types shared between host and macro processes
//! - [`context_registry`] - Thread-local storage for the active [`MacroContextIR`]
//! - [`declarative`] - Grammar and parser for declarative (pattern-matching) macros
//! - [`derive`] - Derive input types that mirror Rust's `syn::DeriveInput`
//! - [`errors`] - Error types and diagnostics for macro expansion
//! - [`import_registry`] - Unified import registry built during IR lowering
//! - [`jsdoc`] - JSDoc directive parsing used by lowering
//! - [`lower`] - AST lowering from OXC types to IR representations
//! - [`stream`] - Parsing stream abstraction similar to `syn::parse::ParseBuffer`
//! - [`type_normalize`] - Helpers for splitting TS type-string snippets into structural pieces
//!
//! ## Architecture
//!
//! The crate follows a layered architecture:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    User-Facing API                          │
//! │  (DeriveInput, TsStream, parse_ts_macro_input!)             │
//! ├─────────────────────────────────────────────────────────────┤
//! │                    Lowering Layer                           │
//! │  (lower_classes, lower_interfaces, ...)                     │
//! ├─────────────────────────────────────────────────────────────┤
//! │                    IR Types (ABI Stable)                    │
//! │  (ClassIR, InterfaceIR, EnumIR, TypeAliasIR, ...)           │
//! ├─────────────────────────────────────────────────────────────┤
//! │                    Parser Backend                           │
//! │  (OXC)                                                      │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Usage Example
//!
//! Here's how to use this crate in a derive macro:
//!
//! ```rust
//! use macroforge_ts_syn::{parse_ts_macro_input, Data, DeriveInput, MacroforgeError, TsStream};
//!
//! // A typical derive macro entry point (normally annotated with
//! // `#[ts_macro_derive(...)]` from the `macroforge_ts_macros` crate)
//! pub fn my_derive_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
//!     // Parse the input using the syn-like API
//!     let input = parse_ts_macro_input!(input as DeriveInput);
//!
//!     // Access type information
//!     println!("Processing type: {}", input.name());
//!
//!     // Match on the type kind
//!     match &input.data {
//!         Data::Class(class) => {
//!             for field in class.fields() {
//!                 println!("Field: {}", field.name);
//!             }
//!         }
//!         Data::Interface(_iface) => {
//!             // Handle interface...
//!         }
//!         Data::Enum(_enum_) => {
//!             // Handle enum...
//!         }
//!         Data::TypeAlias(_alias) => {
//!             // Handle type alias...
//!         }
//!     }
//!
//!     // Return the generated code as a TsStream
//!     Ok(TsStream::from_string(String::new()))
//! }
//! ```
//!
//! ## Re-exports
//!
//! - [`oxc`] - The OXC crate, for macros that work on the parsed AST directly

pub mod abi;
pub mod ast;
pub mod config;
pub mod context_registry;
pub mod declarative;
pub mod derive;
pub mod errors;
pub mod import_registry;
pub mod jsdoc;
pub mod lower;
mod quote_helpers;
pub mod stream;
pub mod type_normalize;

pub use abi::*;
pub use derive::*;
pub use errors::*;
pub use import_registry::{
    ImportRegistry, clear_registry, collect_file_imports, install_registry, take_registry,
    with_registry, with_registry_mut,
};
pub use lower::{
    collect_exported_names, lower_classes, lower_enums, lower_functions, lower_interfaces,
    lower_targets, lower_type_aliases,
};
pub use quote_helpers::{
    QuoteArena, ToAssignTargetSource, ToExprSource, ToIdentSource, ToPatSource,
    ToStringLiteralSource, ToTypeSource, assignment_target_to_string, binding_pattern_to_string,
    expr_to_string, parse_assignment_target, parse_binding_pattern, parse_expr, parse_module_item,
    parse_program, parse_prop_or_spread, parse_statement, parse_type, stmt_to_string,
    string_literal_to_string, type_to_string,
};
pub use stream::*;

pub use ::oxc;

/// Creates an [`ast::Ident`] from a name or a format string.
///
/// # Examples
///
/// ```rust
/// use macroforge_ts_syn::ts_ident;
///
/// let id = ts_ident!("myVariable");
/// assert_eq!(id.sym, "myVariable");
///
/// let field_name = "age";
/// let getter = ts_ident!("get{}", field_name.to_uppercase());
/// assert_eq!(getter.sym, "getAGE");
/// ```
#[macro_export]
macro_rules! ts_ident {
    ($name:expr) => {
        $crate::ast::Ident::new(AsRef::<str>::as_ref(&$name))
    };
    ($fmt:expr, $($args:expr),+ $(,)?) => {
        $crate::ast::Ident::new(format!($fmt, $($args),+))
    };
}

// =============================================================================
// ToTsString trait for string-based template interpolation
// =============================================================================

/// Trait for converting values to TypeScript string representations.
///
/// This trait is used by `ts_template!` macro for interpolating values into
/// string-based templates.
///
/// # Implementations
///
/// - Primitive types (`String`, `&str`, numbers, `bool`) use their standard string representation
/// - [`ast::Expr`], [`ast::Ident`] and [`TsStream`] emit their source text
/// - References and smart pointers delegate to the pointee
pub trait ToTsString {
    /// Convert this value to a TypeScript string representation.
    fn to_ts_string(&self) -> String;
}

// Implementations for common Rust types
// Note: Strings are output as-is (without quotes) for identifier concatenation.
// For string literals in expressions, use expr_str() or similar helper.
impl ToTsString for String {
    fn to_ts_string(&self) -> String {
        self.clone()
    }
}

impl ToTsString for str {
    fn to_ts_string(&self) -> String {
        self.to_owned()
    }
}

impl ToTsString for bool {
    fn to_ts_string(&self) -> String {
        self.to_string()
    }
}

impl ToTsString for i8 {
    fn to_ts_string(&self) -> String {
        self.to_string()
    }
}

impl ToTsString for i16 {
    fn to_ts_string(&self) -> String {
        self.to_string()
    }
}

impl ToTsString for i32 {
    fn to_ts_string(&self) -> String {
        self.to_string()
    }
}

impl ToTsString for i64 {
    fn to_ts_string(&self) -> String {
        self.to_string()
    }
}

impl ToTsString for i128 {
    fn to_ts_string(&self) -> String {
        self.to_string()
    }
}

impl ToTsString for isize {
    fn to_ts_string(&self) -> String {
        self.to_string()
    }
}

impl ToTsString for u8 {
    fn to_ts_string(&self) -> String {
        self.to_string()
    }
}

impl ToTsString for u16 {
    fn to_ts_string(&self) -> String {
        self.to_string()
    }
}

impl ToTsString for u32 {
    fn to_ts_string(&self) -> String {
        self.to_string()
    }
}

impl ToTsString for u64 {
    fn to_ts_string(&self) -> String {
        self.to_string()
    }
}

impl ToTsString for u128 {
    fn to_ts_string(&self) -> String {
        self.to_string()
    }
}

impl ToTsString for usize {
    fn to_ts_string(&self) -> String {
        self.to_string()
    }
}

impl ToTsString for f32 {
    fn to_ts_string(&self) -> String {
        self.to_string()
    }
}

impl ToTsString for f64 {
    fn to_ts_string(&self) -> String {
        self.to_string()
    }
}

impl ToTsString for char {
    fn to_ts_string(&self) -> String {
        self.to_string()
    }
}

impl ToTsString for crate::ast::Expr {
    fn to_ts_string(&self) -> String {
        self.source().to_string()
    }
}

impl ToTsString for crate::ast::Ident {
    fn to_ts_string(&self) -> String {
        self.sym.clone()
    }
}

impl ToTsString for crate::TsStream {
    fn to_ts_string(&self) -> String {
        self.source().to_string()
    }
}

// Reference implementations
impl<T: ToTsString + ?Sized> ToTsString for &T {
    fn to_ts_string(&self) -> String {
        (*self).to_ts_string()
    }
}

impl<T: ToTsString + ?Sized> ToTsString for &mut T {
    fn to_ts_string(&self) -> String {
        (**self).to_ts_string()
    }
}

impl<T: ToTsString + ?Sized> ToTsString for Box<T> {
    fn to_ts_string(&self) -> String {
        (**self).to_ts_string()
    }
}

impl<T: ToTsString + Clone> ToTsString for std::borrow::Cow<'_, T> {
    fn to_ts_string(&self) -> String {
        self.as_ref().to_ts_string()
    }
}

impl<T: ToTsString> ToTsString for std::rc::Rc<T> {
    fn to_ts_string(&self) -> String {
        (**self).to_ts_string()
    }
}

impl<T: ToTsString> ToTsString for std::sync::Arc<T> {
    fn to_ts_string(&self) -> String {
        (**self).to_ts_string()
    }
}
