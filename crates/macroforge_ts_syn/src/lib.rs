//! # macroforge_ts_syn
//!
//! TypeScript syntax types for compile-time macro code generation.
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
//! - [`derive`] - Derive input types that mirror Rust's `syn::DeriveInput`
//! - [`errors`] - Error types and diagnostics for macro expansion
//! - [`lower`] - AST lowering from SWC types to IR representations
//! - [`parse`] - TypeScript parsing utilities wrapping SWC
//! - [`quote_helpers`] - Macros for ergonomic code generation
//! - [`stream`] - Parsing stream abstraction similar to `syn::parse::ParseBuffer`
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
//! │  (lower_classes, lower_interfaces, lower_enums, ...)        │
//! ├─────────────────────────────────────────────────────────────┤
//! │                    IR Types (ABI Stable)                    │
//! │  (ClassIR, InterfaceIR, EnumIR, TypeAliasIR, ...)           │
//! ├─────────────────────────────────────────────────────────────┤
//! │                    SWC Parser                               │
//! │  (swc_core for TypeScript/JavaScript parsing)               │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Usage Example
//!
//! Here's how to use this crate in a derive macro:
//!
//! ```rust,ignore
//! use macroforge_ts_syn::{parse_ts_macro_input, DeriveInput, MacroResult, Patch, Data, MacroContextIR};
//!
//! // This function signature shows a typical derive macro entry point
//! pub fn my_derive_macro(ctx: MacroContextIR) -> MacroResult {
//!     // Parse the input using the syn-like API
//!     let input = parse_ts_macro_input!(ctx);
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
//!     // Generate code and return patches
//!     MacroResult::ok()
//! }
//! ```
//!
//! ## Helper Macros
//!
//! This crate provides several helper macros for working with SWC AST nodes:
//!
//! - [`ident!`] - Create an identifier with optional formatting
//! - [`private_ident!`] - Create a private (marked) identifier
//! - [`stmt_block!`] - Create a block statement from statements
//! - [`fn_expr!`] - Create an anonymous function expression
//! - [`member_expr!`] - Create a member access expression (obj.prop)
//! - [`assign_stmt!`] - Create an assignment statement
//! - [`fn_assign!`] - Create a function assignment (obj.prop = function() {...})
//! - [`proto_method!`] - Create a prototype method assignment
//!
//! ## Feature Flags
//!
//! - `swc` - Enables SWC integration and the helper macros (enabled by default)
//!
//! ## Re-exports
//!
//! For convenience, the crate re-exports commonly used SWC types when the `swc` feature
//! is enabled:
//!
//! - [`swc_core`] - The full SWC core crate
//! - [`swc_common`] - Common SWC types (Span, SourceMap, etc.)
//! - [`swc_ecma_ast`] - ECMAScript/TypeScript AST types
//! - `quote!` - SWC's quote macro for AST generation

pub mod abi;
pub mod derive;
pub mod errors;
pub mod lower;
pub mod parse;
pub mod quote_helpers;
pub mod stream;

pub use abi::*;
pub use derive::*;
pub use errors::*;
pub use lower::*;
pub use stream::*;
#[cfg(feature = "swc")]
pub use swc_core::quote;

// Re-export swc_core for convenience
#[cfg(feature = "swc")]
pub use swc_core;

// Re-export common swc modules at top level for ergonomics
#[cfg(feature = "swc")]
pub use swc_core::common as swc_common;
#[cfg(feature = "swc")]
pub use swc_core::ecma::ast as swc_ecma_ast;

// Helper macros for creating AST nodes

// =============================================================================
// Expression conversion helpers
// =============================================================================

/// Converts common Rust values into SWC [`Expr`](swc_core::ecma::ast::Expr) nodes.
///
/// This enables ergonomic interpolation in template literals and other AST
/// construction contexts where a string or identifier should be treated as a
/// TypeScript expression.
#[cfg(feature = "swc")]
pub trait ToTsExpr {
    fn to_ts_expr(self) -> swc_core::ecma::ast::Expr;
}

#[cfg(feature = "swc")]
impl ToTsExpr for swc_core::ecma::ast::Expr {
    fn to_ts_expr(self) -> swc_core::ecma::ast::Expr {
        self
    }
}

#[cfg(feature = "swc")]
impl ToTsExpr for Box<swc_core::ecma::ast::Expr> {
    fn to_ts_expr(self) -> swc_core::ecma::ast::Expr {
        *self
    }
}

#[cfg(feature = "swc")]
impl ToTsExpr for &swc_core::ecma::ast::Expr {
    fn to_ts_expr(self) -> swc_core::ecma::ast::Expr {
        self.clone()
    }
}

#[cfg(feature = "swc")]
impl ToTsExpr for swc_core::ecma::ast::Ident {
    fn to_ts_expr(self) -> swc_core::ecma::ast::Expr {
        swc_core::ecma::ast::Expr::Ident(self)
    }
}

#[cfg(feature = "swc")]
impl ToTsExpr for &swc_core::ecma::ast::Ident {
    fn to_ts_expr(self) -> swc_core::ecma::ast::Expr {
        swc_core::ecma::ast::Expr::Ident(self.clone())
    }
}

#[cfg(feature = "swc")]
impl ToTsExpr for String {
    fn to_ts_expr(self) -> swc_core::ecma::ast::Expr {
        swc_core::ecma::ast::Expr::Lit(swc_core::ecma::ast::Lit::Str(
            swc_core::ecma::ast::Str {
                span: swc_core::common::DUMMY_SP,
                value: self.into(),
                raw: None,
            },
        ))
    }
}

#[cfg(feature = "swc")]
impl ToTsExpr for &String {
    fn to_ts_expr(self) -> swc_core::ecma::ast::Expr {
        self.clone().to_ts_expr()
    }
}

#[cfg(feature = "swc")]
impl ToTsExpr for &str {
    fn to_ts_expr(self) -> swc_core::ecma::ast::Expr {
        swc_core::ecma::ast::Expr::Lit(swc_core::ecma::ast::Lit::Str(
            swc_core::ecma::ast::Str {
                span: swc_core::common::DUMMY_SP,
                value: self.into(),
                raw: None,
            },
        ))
    }
}

/// Convert a value into a TypeScript [`Expr`](swc_core::ecma::ast::Expr).
///
/// This is a convenience wrapper for [`ToTsExpr`].
#[cfg(feature = "swc")]
pub fn to_ts_expr<T: ToTsExpr>(value: T) -> swc_core::ecma::ast::Expr {
    value.to_ts_expr()
}

// =============================================================================
// Type conversion helpers
// =============================================================================

/// Converts common Rust values into SWC [`TsType`](swc_core::ecma::ast::TsType) nodes.
///
/// This enables ergonomic interpolation in template literals where a type
/// annotation is expected.
#[cfg(feature = "swc")]
pub trait ToTsType {
    fn to_ts_type(&self) -> swc_core::ecma::ast::TsType;
}

#[cfg(feature = "swc")]
impl ToTsType for swc_core::ecma::ast::TsType {
    fn to_ts_type(&self) -> swc_core::ecma::ast::TsType {
        self.clone()
    }
}

#[cfg(feature = "swc")]
impl ToTsType for Box<swc_core::ecma::ast::TsType> {
    fn to_ts_type(&self) -> swc_core::ecma::ast::TsType {
        (**self).clone()
    }
}

/// Convert a string to a TsTypeRef (type reference by name).
#[cfg(feature = "swc")]
impl ToTsType for String {
    fn to_ts_type(&self) -> swc_core::ecma::ast::TsType {
        swc_core::ecma::ast::TsType::TsTypeRef(swc_core::ecma::ast::TsTypeRef {
            span: swc_core::common::DUMMY_SP,
            type_name: swc_core::ecma::ast::TsEntityName::Ident(
                swc_core::ecma::ast::Ident::new_no_ctxt(self.clone().into(), swc_core::common::DUMMY_SP),
            ),
            type_params: None,
        })
    }
}

#[cfg(feature = "swc")]
impl ToTsType for str {
    fn to_ts_type(&self) -> swc_core::ecma::ast::TsType {
        self.to_string().to_ts_type()
    }
}

/// Convert an Ident to a TsTypeRef.
#[cfg(feature = "swc")]
impl ToTsType for swc_core::ecma::ast::Ident {
    fn to_ts_type(&self) -> swc_core::ecma::ast::TsType {
        swc_core::ecma::ast::TsType::TsTypeRef(swc_core::ecma::ast::TsTypeRef {
            span: swc_core::common::DUMMY_SP,
            type_name: swc_core::ecma::ast::TsEntityName::Ident(self.clone()),
            type_params: None,
        })
    }
}

/// Convert an Expr to a TsType. Only works for identifier expressions.
#[cfg(feature = "swc")]
impl ToTsType for swc_core::ecma::ast::Expr {
    fn to_ts_type(&self) -> swc_core::ecma::ast::TsType {
        match self {
            swc_core::ecma::ast::Expr::Ident(ident) => ident.to_ts_type(),
            // For other expressions, create a typeof type
            other => swc_core::ecma::ast::TsType::TsTypeQuery(swc_core::ecma::ast::TsTypeQuery {
                span: swc_core::common::DUMMY_SP,
                expr_name: swc_core::ecma::ast::TsTypeQueryExpr::TsEntityName(
                    swc_core::ecma::ast::TsEntityName::Ident(swc_core::ecma::ast::Ident::new_no_ctxt(
                        format!("{:?}", other).into(),
                        swc_core::common::DUMMY_SP,
                    )),
                ),
                type_args: None,
            }),
        }
    }
}

/// Convert a value into a TypeScript [`TsType`](swc_core::ecma::ast::TsType).
///
/// This is a convenience wrapper for [`ToTsType`].
#[cfg(feature = "swc")]
pub fn to_ts_type<T: ToTsType>(value: &T) -> swc_core::ecma::ast::TsType {
    value.to_ts_type()
}

// =============================================================================
// Identifier conversion helpers
// =============================================================================

/// Converts common Rust values into SWC [`Ident`](swc_core::ecma::ast::Ident) nodes.
///
/// This enables ergonomic interpolation in template literals where an identifier
/// is expected.
#[cfg(feature = "swc")]
pub trait ToTsIdent {
    fn to_ts_ident(&self) -> swc_core::ecma::ast::Ident;
}

#[cfg(feature = "swc")]
impl ToTsIdent for swc_core::ecma::ast::Ident {
    fn to_ts_ident(&self) -> swc_core::ecma::ast::Ident {
        self.clone()
    }
}

#[cfg(feature = "swc")]
impl ToTsIdent for String {
    fn to_ts_ident(&self) -> swc_core::ecma::ast::Ident {
        swc_core::ecma::ast::Ident::new_no_ctxt(self.clone().into(), swc_core::common::DUMMY_SP)
    }
}

#[cfg(feature = "swc")]
impl ToTsIdent for str {
    fn to_ts_ident(&self) -> swc_core::ecma::ast::Ident {
        swc_core::ecma::ast::Ident::new_no_ctxt(self.into(), swc_core::common::DUMMY_SP)
    }
}

/// Convert an Expr to an Ident. Only works for identifier expressions.
#[cfg(feature = "swc")]
impl ToTsIdent for swc_core::ecma::ast::Expr {
    fn to_ts_ident(&self) -> swc_core::ecma::ast::Ident {
        match self {
            swc_core::ecma::ast::Expr::Ident(ident) => ident.clone(),
            // For non-ident expressions, create a placeholder identifier
            _ => swc_core::ecma::ast::Ident::new_no_ctxt(
                "__expr__".into(),
                swc_core::common::DUMMY_SP,
            ),
        }
    }
}

/// Convert a TsType to an Ident. Only works for type references with simple names.
#[cfg(feature = "swc")]
impl ToTsIdent for swc_core::ecma::ast::TsType {
    fn to_ts_ident(&self) -> swc_core::ecma::ast::Ident {
        match self {
            swc_core::ecma::ast::TsType::TsTypeRef(swc_core::ecma::ast::TsTypeRef {
                type_name: swc_core::ecma::ast::TsEntityName::Ident(ident),
                ..
            }) => ident.clone(),
            // For other types, create a placeholder
            _ => swc_core::ecma::ast::Ident::new_no_ctxt(
                "__type__".into(),
                swc_core::common::DUMMY_SP,
            ),
        }
    }
}

#[cfg(feature = "swc")]
impl ToTsIdent for Box<swc_core::ecma::ast::TsType> {
    fn to_ts_ident(&self) -> swc_core::ecma::ast::Ident {
        (**self).to_ts_ident()
    }
}

/// Convert a value into a TypeScript [`Ident`](swc_core::ecma::ast::Ident).
///
/// This is a convenience wrapper for [`ToTsIdent`].
#[cfg(feature = "swc")]
pub fn to_ts_ident<T: ToTsIdent>(value: &T) -> swc_core::ecma::ast::Ident {
    value.to_ts_ident()
}

// =============================================================================
// Statement conversion helpers
// =============================================================================

/// Converts common Rust values into SWC [`Stmt`](swc_core::ecma::ast::Stmt) nodes.
///
/// This enables ergonomic interpolation in template literals where a statement
/// is expected.
#[cfg(feature = "swc")]
pub trait ToTsStmt {
    fn to_ts_stmt(self) -> swc_core::ecma::ast::Stmt;
}

#[cfg(feature = "swc")]
impl ToTsStmt for swc_core::ecma::ast::Stmt {
    fn to_ts_stmt(self) -> swc_core::ecma::ast::Stmt {
        self
    }
}

#[cfg(feature = "swc")]
impl ToTsStmt for Box<swc_core::ecma::ast::Stmt> {
    fn to_ts_stmt(self) -> swc_core::ecma::ast::Stmt {
        *self
    }
}

#[cfg(feature = "swc")]
impl ToTsStmt for &swc_core::ecma::ast::Stmt {
    fn to_ts_stmt(self) -> swc_core::ecma::ast::Stmt {
        self.clone()
    }
}

/// Convert an expression to an expression statement.
#[cfg(feature = "swc")]
impl ToTsStmt for swc_core::ecma::ast::Expr {
    fn to_ts_stmt(self) -> swc_core::ecma::ast::Stmt {
        swc_core::ecma::ast::Stmt::Expr(swc_core::ecma::ast::ExprStmt {
            span: swc_core::common::DUMMY_SP,
            expr: Box::new(self),
        })
    }
}

#[cfg(feature = "swc")]
impl ToTsStmt for Box<swc_core::ecma::ast::Expr> {
    fn to_ts_stmt(self) -> swc_core::ecma::ast::Stmt {
        swc_core::ecma::ast::Stmt::Expr(swc_core::ecma::ast::ExprStmt {
            span: swc_core::common::DUMMY_SP,
            expr: self,
        })
    }
}

/// Convert a value into a TypeScript [`Stmt`](swc_core::ecma::ast::Stmt).
///
/// This is a convenience wrapper for [`ToTsStmt`].
#[cfg(feature = "swc")]
pub fn to_ts_stmt<T: ToTsStmt>(value: T) -> swc_core::ecma::ast::Stmt {
    value.to_ts_stmt()
}

/// Trait for converting values to type name strings.
///
/// This is used for type placeholder substitution in templates.
/// The key difference from `ToString` is that for `Ident`, this uses
/// the symbol directly (`.sym`) rather than the Display impl, which
/// avoids including SyntaxContext markers like `#0`.
#[cfg(feature = "swc")]
pub trait ToTsTypeName {
    fn to_ts_type_name(&self) -> String;
}

#[cfg(feature = "swc")]
impl ToTsTypeName for swc_core::ecma::ast::Ident {
    fn to_ts_type_name(&self) -> String {
        self.sym.to_string()
    }
}

#[cfg(feature = "swc")]
impl ToTsTypeName for &swc_core::ecma::ast::Ident {
    fn to_ts_type_name(&self) -> String {
        self.sym.to_string()
    }
}

#[cfg(feature = "swc")]
impl ToTsTypeName for String {
    fn to_ts_type_name(&self) -> String {
        self.clone()
    }
}

#[cfg(feature = "swc")]
impl ToTsTypeName for &String {
    fn to_ts_type_name(&self) -> String {
        (*self).clone()
    }
}

#[cfg(feature = "swc")]
impl ToTsTypeName for &str {
    fn to_ts_type_name(&self) -> String {
        (*self).to_string()
    }
}

/// Creates an SWC [`Ident`](swc_core::ecma::ast::Ident) with a dummy span.
///
/// This macro provides a convenient way to create identifier AST nodes for
/// code generation. It supports both simple string names and format strings.
///
/// # Examples
///
/// Simple identifier:
///
/// ```rust,no_run
/// use macroforge_ts_syn::ident;
///
/// let id = ident!("myVariable");
/// assert_eq!(id.sym.as_str(), "myVariable");
/// ```
///
/// Formatted identifier:
///
/// ```rust,no_run
/// use macroforge_ts_syn::ident;
///
/// let field_name = "age";
/// let getter = ident!("get{}", field_name.to_uppercase());
/// assert_eq!(getter.sym.as_str(), "getAGE");
/// ```
///
/// Using in code generation:
///
/// ```rust,no_run
/// use macroforge_ts_syn::{ident, quote};
///
/// let class_name = ident!("MyClass");
/// let code = quote!("class $class_name {}" as Stmt, class_name: Ident = class_name);
/// ```
#[cfg(feature = "swc")]
#[macro_export]
macro_rules! ident {
    // Single argument - direct string
    // Use AsRef<str> to handle String, &String, &str, etc.
    ($name:expr) => {
        swc_core::ecma::ast::Ident::new_no_ctxt(
            AsRef::<str>::as_ref(&$name).into(),
            swc_core::common::DUMMY_SP,
        )
    };
    // Format string with arguments
    ($fmt:expr, $($args:expr),+ $(,)?) => {
        swc_core::ecma::ast::Ident::new_no_ctxt(format!($fmt, $($args),+).into(), swc_core::common::DUMMY_SP)
    };
}

/// Creates a private (marked) SWC [`Ident`](swc_core::ecma::ast::Ident).
///
/// Unlike [`ident!`], this macro creates an identifier with a fresh hygiene mark,
/// making it unique and preventing name collisions with user code. Use this when
/// generating temporary variables or internal identifiers that shouldn't conflict
/// with existing names in scope.
///
/// # Examples
///
/// Creating a unique temporary variable:
///
/// ```rust,no_run
/// use macroforge_ts_syn::private_ident;
///
/// // Each call creates a unique identifier that won't clash
/// let temp1 = private_ident!("temp");
/// let temp2 = private_ident!("temp");
/// // temp1 and temp2 have different syntax contexts
/// ```
///
/// Generating internal helper code:
///
/// ```rust,no_run
/// use macroforge_ts_syn::{private_ident, quote};
///
/// let internal_var = private_ident!("__internal");
/// // This won't conflict with any user-defined __internal variable
/// let code = quote!("let $var = {};" as Stmt, var: Ident = internal_var);
/// ```
#[cfg(feature = "swc")]
#[macro_export]
macro_rules! private_ident {
    ($name:expr) => {{
        let mark = swc_core::common::Mark::fresh(swc_core::common::Mark::root());
        swc_core::ecma::ast::Ident::new(
            $name.into(),
            swc_core::common::DUMMY_SP,
            swc_core::common::SyntaxContext::empty().apply_mark(mark),
        )
    }};
}

/// Creates a block statement (`{ ... }`) from a vector of statements.
///
/// This macro wraps a `Vec<Stmt>` into a [`BlockStmt`](swc_core::ecma::ast::BlockStmt),
/// which represents a `{ ... }` block in JavaScript/TypeScript. Useful when you need
/// to group multiple statements into a single statement.
///
/// # Examples
///
/// Wrapping multiple statements in a block:
///
/// ```rust,ignore
/// use macroforge_ts_syn::{stmt_block, parse_ts_str};
/// use swc_core::ecma::ast::Stmt;
///
/// let stmt1: Stmt = parse_ts_str("console.log('hello');").unwrap();
/// let stmt2: Stmt = parse_ts_str("console.log('world');").unwrap();
///
/// let block = stmt_block!(vec![stmt1, stmt2]);
/// // Generates: { console.log('hello'); console.log('world'); }
/// ```
///
/// Using with generated code:
///
/// ```rust
/// use macroforge_ts_syn::stmt_block;
/// use macroforge_ts_syn::swc_ecma_ast::Stmt;
///
/// fn wrap_in_block(statements: Vec<Stmt>) -> Stmt {
///     stmt_block!(statements)
/// }
/// ```
#[cfg(feature = "swc")]
#[macro_export]
macro_rules! stmt_block {
    ($stmts:expr) => {
        swc_core::ecma::ast::Stmt::Block(swc_core::ecma::ast::BlockStmt {
            span: swc_core::common::DUMMY_SP,
            ctxt: swc_core::common::SyntaxContext::empty(),
            stmts: $stmts,
        })
    };
}

/// A wrapper type for passing a `Vec<Stmt>` to be used inline in function bodies.
///
/// This is a marker type that `ts_quote!` can detect and handle specially,
/// allowing you to interpolate multiple statements where the macro expects them.
///
/// # Examples
///
/// ```rust
/// use macroforge_ts_syn::{StmtVec, stmt_vec};
/// use macroforge_ts_syn::swc_ecma_ast::Stmt;
///
/// // Create directly with constructor
/// let statements1: Vec<Stmt> = vec![];
/// let _wrapped1 = StmtVec(statements1);
///
/// // Or using the macro
/// let statements2: Vec<Stmt> = vec![];
/// let _wrapped2 = stmt_vec!(statements2);
/// ```
#[cfg(feature = "swc")]
pub struct StmtVec(pub Vec<swc_core::ecma::ast::Stmt>);

/// Wraps a `Vec<Stmt>` in a [`StmtVec`] for use with `ts_quote!`.
///
/// This macro is a convenience wrapper around creating a `StmtVec` directly.
/// Use it when you need to pass multiple statements to a quote macro that
/// expects a statement vector.
///
/// # Examples
///
/// ```rust,no_run
/// use macroforge_ts_syn::stmt_vec;
/// use swc_core::ecma::ast::Stmt;
///
/// fn generate_method_body() -> Vec<Stmt> {
///     // ... generate statements
///     vec![]
/// }
///
/// let body = stmt_vec!(generate_method_body());
/// ```
#[cfg(feature = "swc")]
#[macro_export]
macro_rules! stmt_vec {
    ($stmts:expr) => {
        macroforge_ts_syn::StmtVec($stmts)
    };
}

/// Converts a `Vec<Stmt>` into a single block statement for use in `ts_quote!`.
///
/// This macro is useful when you need to interpolate multiple statements in a
/// context where only a single statement is expected. The statements are wrapped
/// in a block `{ ... }`.
///
/// This is an alias for [`stmt_block!`] and behaves identically.
///
/// # Examples
///
/// ```rust,no_run
/// use macroforge_ts_syn::{stmt_block_from_vec, quote};
/// use swc_core::ecma::ast::Stmt;
///
/// fn generate_body() -> Vec<Stmt> {
///     // Generate multiple statements
///     vec![]
/// }
///
/// let body_block = stmt_block_from_vec!(generate_body());
/// // Can now use body_block where a single Stmt is expected
/// ```
#[cfg(feature = "swc")]
#[macro_export]
macro_rules! stmt_block_from_vec {
    ($stmts:expr) => {
        swc_core::ecma::ast::Stmt::Block(swc_core::ecma::ast::BlockStmt {
            span: swc_core::common::DUMMY_SP,
            ctxt: swc_core::common::SyntaxContext::empty(),
            stmts: $stmts,
        })
    };
}

/// Creates an anonymous function expression with the given body statements.
///
/// This macro generates a [`FnExpr`](swc_core::ecma::ast::FnExpr) representing
/// `function() { ... }` or `function(params) { ... }` in JavaScript/TypeScript.
///
/// # Variants
///
/// - `fn_expr!(body_stmts)` - Creates a function with no parameters
/// - `fn_expr!(params, body_stmts)` - Creates a function with the specified parameters
///
/// # Examples
///
/// Function with no parameters:
///
/// ```rust,ignore
/// use macroforge_ts_syn::{fn_expr, parse_ts_str};
/// use swc_core::ecma::ast::Stmt;
///
/// let body: Vec<Stmt> = vec![
///     parse_ts_str("return 42;").unwrap()
/// ];
///
/// let func = fn_expr!(body);
/// // Generates: function() { return 42; }
/// ```
///
/// Function with parameters:
///
/// ```rust,ignore
/// use macroforge_ts_syn::{fn_expr, ident};
/// use swc_core::ecma::ast::{Param, Pat, Stmt};
/// use swc_core::common::DUMMY_SP;
///
/// let params = vec![
///     Param {
///         span: DUMMY_SP,
///         decorators: vec![],
///         pat: Pat::Ident(ident!("x").into()),
///     }
/// ];
/// let body: Vec<Stmt> = vec![];
///
/// let func = fn_expr!(params, body);
/// // Generates: function(x) { ... }
/// ```
#[cfg(feature = "swc")]
#[macro_export]
macro_rules! fn_expr {
    ($body_stmts:expr) => {
        swc_core::ecma::ast::Expr::Fn(swc_core::ecma::ast::FnExpr {
            ident: None,
            function: Box::new(swc_core::ecma::ast::Function {
                params: vec![],
                decorators: vec![],
                span: swc_core::common::DUMMY_SP,
                ctxt: swc_core::common::SyntaxContext::empty(),
                body: Some(swc_core::ecma::ast::BlockStmt {
                    span: swc_core::common::DUMMY_SP,
                    ctxt: swc_core::common::SyntaxContext::empty(),
                    stmts: $body_stmts,
                }),
                is_generator: false,
                is_async: false,
                type_params: None,
                return_type: None,
            }),
        })
    };
    ($params:expr, $body_stmts:expr) => {
        swc_core::ecma::ast::Expr::Fn(swc_core::ecma::ast::FnExpr {
            ident: None,
            function: Box::new(swc_core::ecma::ast::Function {
                params: $params,
                decorators: vec![],
                span: swc_core::common::DUMMY_SP,
                ctxt: swc_core::common::SyntaxContext::empty(),
                body: Some(swc_core::ecma::ast::BlockStmt {
                    span: swc_core::common::DUMMY_SP,
                    ctxt: swc_core::common::SyntaxContext::empty(),
                    stmts: $body_stmts,
                }),
                is_generator: false,
                is_async: false,
                type_params: None,
                return_type: None,
            }),
        })
    };
}

/// Creates a member access expression (`obj.prop`).
///
/// This macro generates a [`MemberExpr`](swc_core::ecma::ast::MemberExpr) representing
/// property access in JavaScript/TypeScript, such as `obj.property` or `this.field`.
///
/// # Arguments
///
/// - `$obj` - The object expression to access the property on
/// - `$prop` - The property name as a string or something convertible to `JsWord`
///
/// # Examples
///
/// Accessing a property:
///
/// ```rust,ignore
/// use macroforge_ts_syn::{member_expr, ident};
/// use swc_core::ecma::ast::{Expr, ThisExpr};
/// use swc_core::common::DUMMY_SP;
///
/// let this_expr = Expr::This(ThisExpr { span: DUMMY_SP });
/// let access = member_expr!(this_expr, "name");
/// // Generates: this.name
/// ```
///
/// Chaining member access:
///
/// ```rust
/// use macroforge_ts_syn::{member_expr, ident};
/// use macroforge_ts_syn::swc_ecma_ast::Expr;
///
/// let obj = Expr::Ident(ident!("obj"));
/// let _nested = member_expr!(member_expr!(obj, "foo"), "bar");
/// // Generates: obj.foo.bar
/// ```
///
/// Accessing prototype:
///
/// ```rust
/// use macroforge_ts_syn::{member_expr, ident};
/// use macroforge_ts_syn::swc_ecma_ast::Expr;
///
/// let class = Expr::Ident(ident!("MyClass"));
/// let _proto = member_expr!(class, "prototype");
/// // Generates: MyClass.prototype
/// ```
#[cfg(feature = "swc")]
#[macro_export]
macro_rules! member_expr {
    ($obj:expr, $prop:expr) => {
        swc_core::ecma::ast::Expr::Member(swc_core::ecma::ast::MemberExpr {
            span: swc_core::common::DUMMY_SP,
            obj: Box::new($obj),
            prop: swc_core::ecma::ast::MemberProp::Ident(swc_core::ecma::ast::IdentName {
                span: swc_core::common::DUMMY_SP,
                sym: $prop.into(),
            }),
        })
    };
}

/// Creates an assignment expression statement (`lhs = rhs;`).
///
/// This macro generates a statement that assigns a value to a target. The left-hand
/// side must be an [`AssignTarget`](swc_core::ecma::ast::AssignTarget) (typically a
/// variable, property access, or destructuring pattern), and the right-hand side
/// is any expression.
///
/// # Arguments
///
/// - `$lhs` - The assignment target (left-hand side)
/// - `$rhs` - The expression to assign (right-hand side)
///
/// # Examples
///
/// Simple variable assignment:
///
/// ```rust,ignore
/// use macroforge_ts_syn::{assign_stmt, ident};
/// use swc_core::ecma::ast::{AssignTarget, SimpleAssignTarget, Expr, Lit, Number};
/// use swc_core::common::DUMMY_SP;
///
/// let target = AssignTarget::Simple(SimpleAssignTarget::Ident(ident!("x").into()));
/// let value = Expr::Lit(Lit::Num(Number { span: DUMMY_SP, value: 42.0, raw: None }));
///
/// let stmt = assign_stmt!(target, value);
/// // Generates: x = 42;
/// ```
///
/// Property assignment:
///
/// ```rust,ignore
/// use macroforge_ts_syn::{assign_stmt, member_expr, ident};
/// use swc_core::ecma::ast::{AssignTarget, SimpleAssignTarget, Expr, Lit, ThisExpr};
/// use swc_core::common::DUMMY_SP;
///
/// let this_name = member_expr!(Expr::This(ThisExpr { span: DUMMY_SP }), "name");
/// let target = /* convert to AssignTarget */;
/// let value = Expr::Lit(Lit::Str("John".into()));
///
/// let stmt = assign_stmt!(target, value);
/// // Generates: this.name = "John";
/// ```
#[cfg(feature = "swc")]
#[macro_export]
macro_rules! assign_stmt {
    ($lhs:expr, $rhs:expr) => {
        swc_core::ecma::ast::Stmt::Expr(swc_core::ecma::ast::ExprStmt {
            span: swc_core::common::DUMMY_SP,
            expr: Box::new(swc_core::ecma::ast::Expr::Assign(
                swc_core::ecma::ast::AssignExpr {
                    span: swc_core::common::DUMMY_SP,
                    op: swc_core::ecma::ast::AssignOp::Assign,
                    left: $lhs,
                    right: Box::new($rhs),
                },
            )),
        })
    };
}

/// Creates a function assignment statement (`obj.prop = function(params) { body };`).
///
/// This macro is particularly useful for adding methods to prototypes or assigning
/// function implementations to object properties. It generates the complete assignment
/// statement including the anonymous function.
///
/// # Variants
///
/// - `fn_assign!(obj, prop, body_stmts)` - Function with no parameters
/// - `fn_assign!(obj, prop, params, body_stmts)` - Function with specified parameters
///
/// # Arguments
///
/// - `$obj` - The object expression to assign to
/// - `$prop` - The property name (as a string)
/// - `$params` - Optional: vector of function parameters
/// - `$body_stmts` - Vector of statements for the function body
///
/// # Examples
///
/// Adding a method to a prototype (no params):
///
/// ```rust
/// use macroforge_ts_syn::{fn_assign, ident, member_expr};
/// use macroforge_ts_syn::swc_ecma_ast::{Expr, Stmt};
///
/// let class_expr = Expr::Ident(ident!("MyClass"));
/// let proto = member_expr!(class_expr, "prototype");
///
/// let body: Vec<Stmt> = vec![];
/// let _stmt = fn_assign!(proto, "toString", body);
/// // Generates: MyClass.prototype.toString = function() { ... };
/// ```
///
/// Adding a method with parameters:
///
/// ```rust,ignore
/// use macroforge_ts_syn::{fn_assign, ident, member_expr};
/// use swc_core::ecma::ast::{Expr, Param, Pat, Stmt};
/// use swc_core::common::DUMMY_SP;
///
/// let class_expr = Expr::Ident(ident!("MyClass"));
/// let proto = member_expr!(class_expr, "prototype");
///
/// let params = vec![
///     Param { span: DUMMY_SP, decorators: vec![], pat: Pat::Ident(ident!("value").into()) }
/// ];
/// let body: Vec<Stmt> = vec![];
///
/// let stmt = fn_assign!(proto, "setValue", params, body);
/// // Generates: MyClass.prototype.setValue = function(value) { ... };
/// ```
///
/// # See Also
///
/// - [`proto_method!`] - A higher-level macro for prototype methods
/// - [`fn_expr!`] - For creating standalone function expressions
#[cfg(feature = "swc")]
#[macro_export]
macro_rules! fn_assign {
    ($obj:expr, $prop:expr, $body_stmts:expr) => {{
        use swc_core::common::{DUMMY_SP, SyntaxContext};
        use swc_core::ecma::ast::*;

        Stmt::Expr(ExprStmt {
            span: DUMMY_SP,
            expr: Box::new(Expr::Assign(AssignExpr {
                span: DUMMY_SP,
                op: AssignOp::Assign,
                left: AssignTarget::Simple(SimpleAssignTarget::Member(MemberExpr {
                    span: DUMMY_SP,
                    obj: Box::new($obj),
                    prop: MemberProp::Ident(IdentName {
                        span: DUMMY_SP,
                        sym: $prop.into(),
                    }),
                })),
                right: Box::new(Expr::Fn(FnExpr {
                    ident: None,
                    function: Box::new(Function {
                        params: vec![],
                        decorators: vec![],
                        span: DUMMY_SP,
                        ctxt: SyntaxContext::empty(),
                        body: Some(BlockStmt {
                            span: DUMMY_SP,
                            ctxt: SyntaxContext::empty(),
                            stmts: $body_stmts,
                        }),
                        is_generator: false,
                        is_async: false,
                        type_params: None,
                        return_type: None,
                    }),
                })),
            })),
        })
    }};
    ($obj:expr, $prop:expr, $params:expr, $body_stmts:expr) => {{
        use swc_core::common::{DUMMY_SP, SyntaxContext};
        use swc_core::ecma::ast::*;

        Stmt::Expr(ExprStmt {
            span: DUMMY_SP,
            expr: Box::new(Expr::Assign(AssignExpr {
                span: DUMMY_SP,
                op: AssignOp::Assign,
                left: AssignTarget::Simple(SimpleAssignTarget::Member(MemberExpr {
                    span: DUMMY_SP,
                    obj: Box::new($obj),
                    prop: MemberProp::Ident(IdentName {
                        span: DUMMY_SP,
                        sym: $prop.into(),
                    }),
                })),
                right: Box::new(Expr::Fn(FnExpr {
                    ident: None,
                    function: Box::new(Function {
                        params: $params,
                        decorators: vec![],
                        span: DUMMY_SP,
                        ctxt: SyntaxContext::empty(),
                        body: Some(BlockStmt {
                            span: DUMMY_SP,
                            ctxt: SyntaxContext::empty(),
                            stmts: $body_stmts,
                        }),
                        is_generator: false,
                        is_async: false,
                        type_params: None,
                        return_type: None,
                    }),
                })),
            })),
        })
    }};
}

// =============================================================================
// AST to string conversion helpers (for template codegen)
// =============================================================================

/// Converts an expression to its TypeScript string representation.
///
/// This is used by the template compiler to convert AST nodes back to strings
/// for runtime parsing.
#[cfg(feature = "swc")]
pub fn expr_to_string(expr: &swc_core::ecma::ast::Expr) -> String {
    use swc_core::common::sync::Lrc;
    use swc_core::ecma::ast::{Module, ModuleItem, Stmt, ExprStmt};
    use swc_core::ecma::codegen::{text_writer::JsWriter, Config, Emitter};

    // Wrap expression in a module as expression statement
    let module = Module {
        span: swc_core::common::DUMMY_SP,
        body: vec![ModuleItem::Stmt(Stmt::Expr(ExprStmt {
            span: swc_core::common::DUMMY_SP,
            expr: Box::new(expr.clone()),
        }))],
        shebang: None,
    };

    let cm = Lrc::new(swc_core::common::SourceMap::default());
    let mut buf = Vec::new();
    {
        let mut emitter = Emitter {
            cfg: Config::default(),
            cm: cm.clone(),
            comments: None,
            wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
        };
        let _ = emitter.emit_module(&module);
    }
    // The output will be "expr;\n" - trim the semicolon and newline
    let s = String::from_utf8(buf).unwrap_or_default();
    s.trim_end().trim_end_matches(';').to_string()
}

/// Converts a type to its TypeScript string representation.
#[cfg(feature = "swc")]
pub fn type_to_string(ty: &swc_core::ecma::ast::TsType) -> String {
    use swc_core::common::sync::Lrc;
    use swc_core::ecma::ast::*;
    use swc_core::ecma::codegen::{text_writer::JsWriter, Config, Emitter};

    // Wrap type in a type alias declaration: type __T = <type>;
    let module = Module {
        span: swc_core::common::DUMMY_SP,
        body: vec![ModuleItem::Stmt(Stmt::Decl(Decl::TsTypeAlias(Box::new(
            TsTypeAliasDecl {
                span: swc_core::common::DUMMY_SP,
                declare: false,
                id: Ident::new_no_ctxt("__T".into(), swc_core::common::DUMMY_SP),
                type_params: None,
                type_ann: Box::new(ty.clone()),
            },
        ))))],
        shebang: None,
    };

    let cm = Lrc::new(swc_core::common::SourceMap::default());
    let mut buf = Vec::new();
    {
        let mut emitter = Emitter {
            cfg: Config::default(),
            cm: cm.clone(),
            comments: None,
            wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
        };
        let _ = emitter.emit_module(&module);
    }
    // Output: "type __T = <type>;\n" - extract the type part
    let s = String::from_utf8(buf).unwrap_or_default();
    // Extract between "= " and ";"
    if let Some(eq_pos) = s.find("= ") {
        let after_eq = &s[eq_pos + 2..];
        if let Some(semi_pos) = after_eq.rfind(';') {
            return after_eq[..semi_pos].to_string();
        }
    }
    s.trim().to_string()
}

/// Converts an identifier to its TypeScript string representation.
#[cfg(feature = "swc")]
pub fn ident_to_string(ident: &swc_core::ecma::ast::Ident) -> String {
    ident.sym.to_string()
}

/// Emits a list of module items to a TypeScript source string.
///
/// This function takes AST nodes and optional comments, and produces
/// properly formatted TypeScript source code.
///
/// # Example
///
/// ```ignore
/// use macroforge_ts_syn::emit_module_items;
/// use swc_core::ecma::ast::ModuleItem;
/// use swc_core::common::comments::SingleThreadedComments;
///
/// let items: Vec<ModuleItem> = vec![/* ... */];
/// let comments = SingleThreadedComments::default();
/// let source = emit_module_items(&items, &comments);
/// ```
#[cfg(feature = "swc")]
pub fn emit_module_items(
    items: &[swc_core::ecma::ast::ModuleItem],
    comments: &swc_core::common::comments::SingleThreadedComments,
) -> String {
    use swc_core::common::sync::Lrc;
    use swc_core::ecma::ast::Module;
    use swc_core::ecma::codegen::{text_writer::JsWriter, Config, Emitter};

    let module = Module {
        span: swc_core::common::DUMMY_SP,
        body: items.to_vec(),
        shebang: None,
    };

    let cm = Lrc::new(swc_core::common::SourceMap::default());
    let mut buf = Vec::new();
    {
        let mut emitter = Emitter {
            cfg: Config::default().with_minify(false),
            cm: cm.clone(),
            comments: Some(comments),
            wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
        };
        let _ = emitter.emit_module(&module);
    }
    String::from_utf8(buf).unwrap_or_default()
}

// =============================================================================
// Internal helpers for virtual completion (used by generated code)
// =============================================================================

/// Internal module for virtual completion helpers.
///
/// These functions are used by the generated code from `ts_quote!` macro
/// when templates contain control flow inside function bodies. The template
/// gets split into incomplete chunks that are validated individually, and
/// these helpers reassemble the complete AST at runtime.
///
/// **DO NOT USE DIRECTLY** - These are implementation details of the macro system.
#[cfg(feature = "swc")]
pub mod __internal {
    use std::collections::HashMap;
    use swc_core::ecma::ast::*;
    use swc_core::ecma::visit::{VisitMut, VisitMutWith};

    // =========================================================================
    // Context-aware collector system
    // =========================================================================

    /// Collector for items generated inside control flow, parameterized by context.
    ///
    /// When a for-loop or if-block appears inside an object literal, array literal,
    /// class body, or type literal, the items generated need to be collected and
    /// later merged into the parent structure.
    #[derive(Debug, Clone)]
    pub enum ContextCollector {
        /// Object literal properties: `{ key: value, ... }`
        ObjectProps(Vec<PropOrSpread>),
        /// Array literal elements: `[elem, ...]`
        ArrayElems(Vec<ExprOrSpread>),
        /// Class body members: `class { member... }`
        ClassMembers(Vec<ClassMember>),
        /// Type literal properties: `{ prop: Type, ... }`
        TypeProps(Vec<TsTypeElement>),
        /// Statements (for function/block bodies) - handled separately
        Stmts(Vec<Stmt>),
    }

    impl ContextCollector {
        /// Creates a new empty collector for the given context type.
        ///
        /// Context values: 0=ObjectLiteral, 1=ArrayLiteral, 2=ClassBody, 3=TypeObjectLiteral, _=Stmts
        pub fn new(context_type: u8) -> Self {
            match context_type {
                0 => Self::ObjectProps(vec![]),
                1 => Self::ArrayElems(vec![]),
                2 => Self::ClassMembers(vec![]),
                3 => Self::TypeProps(vec![]),
                _ => Self::Stmts(vec![]),
            }
        }

        /// Creates a collector for object literal properties.
        pub fn for_object() -> Self {
            Self::ObjectProps(vec![])
        }

        /// Creates a collector for array literal elements.
        pub fn for_array() -> Self {
            Self::ArrayElems(vec![])
        }

        /// Creates a collector for class body members.
        pub fn for_class() -> Self {
            Self::ClassMembers(vec![])
        }

        /// Creates a collector for type literal properties.
        pub fn for_type_object() -> Self {
            Self::TypeProps(vec![])
        }
    }

    /// Pushes an object property to a collector.
    ///
    /// Panics if the collector is not an ObjectProps variant.
    pub fn push_object_prop(collector: &mut ContextCollector, prop: PropOrSpread) {
        match collector {
            ContextCollector::ObjectProps(props) => props.push(prop),
            _ => panic!("push_object_prop called on non-ObjectProps collector"),
        }
    }

    /// Pushes an array element to a collector.
    ///
    /// Panics if the collector is not an ArrayElems variant.
    pub fn push_array_elem(collector: &mut ContextCollector, elem: ExprOrSpread) {
        match collector {
            ContextCollector::ArrayElems(elems) => elems.push(elem),
            _ => panic!("push_array_elem called on non-ArrayElems collector"),
        }
    }

    /// Pushes a class member to a collector.
    ///
    /// Panics if the collector is not a ClassMembers variant.
    pub fn push_class_member(collector: &mut ContextCollector, member: ClassMember) {
        match collector {
            ContextCollector::ClassMembers(members) => members.push(member),
            _ => panic!("push_class_member called on non-ClassMembers collector"),
        }
    }

    /// Pushes a type property to a collector.
    ///
    /// Panics if the collector is not a TypeProps variant.
    pub fn push_type_prop(collector: &mut ContextCollector, prop: TsTypeElement) {
        match collector {
            ContextCollector::TypeProps(props) => props.push(prop),
            _ => panic!("push_type_prop called on non-TypeProps collector"),
        }
    }

    /// Extracts a PropOrSpread from a wrapped expression `({ prop })`.
    ///
    /// This is used when parsing object literal properties in a for-loop body.
    /// The property is wrapped in `({ ... })` to make it parseable, then we
    /// extract the actual property.
    pub fn extract_prop_from_wrapped_expr(expr: &Expr) -> Option<PropOrSpread> {
        // Expected structure: Paren(Object({ props: [prop] }))
        if let Expr::Paren(ParenExpr { expr: inner, .. }) = expr {
            if let Expr::Object(ObjectLit { props, .. }) = inner.as_ref() {
                // Skip __mf_dummy property if present, return the real property
                for prop in props {
                    match prop {
                        PropOrSpread::Prop(p) => {
                            if let Prop::KeyValue(KeyValueProp { key, .. }) = p.as_ref() {
                                if let PropName::Ident(ident) = key {
                                    if ident.sym.as_ref() == "__mf_dummy" {
                                        continue;
                                    }
                                }
                            }
                            return Some(prop.clone());
                        }
                        PropOrSpread::Spread(_) => return Some(prop.clone()),
                    }
                }
            }
        }
        None
    }

    /// Extracts an ExprOrSpread from a wrapped array expression `[elem]`.
    ///
    /// This is used when parsing array literal elements in a for-loop body.
    pub fn extract_elem_from_wrapped_expr(expr: &Expr) -> Option<ExprOrSpread> {
        // Expected structure: Array({ elems: [Some(elem)] })
        if let Expr::Array(ArrayLit { elems, .. }) = expr {
            for elem in elems.iter().flatten() {
                return Some(elem.clone());
            }
        }
        None
    }

    /// Merges collected object properties into an opener's object literal.
    ///
    /// Finds the object literal in the opener (typically inside a return statement)
    /// and adds the collected properties to it, removing any dummy properties.
    pub fn merge_object_props(item: &mut ModuleItem, props: Vec<PropOrSpread>) {
        let mut merger = ObjectPropMerger { props_to_add: props };
        item.visit_mut_with(&mut merger);
    }

    /// Merges collected array elements into an opener's array literal.
    pub fn merge_array_elems(item: &mut ModuleItem, elems: Vec<ExprOrSpread>) {
        let mut merger = ArrayElemMerger { elems_to_add: elems };
        item.visit_mut_with(&mut merger);
    }

    /// Merges collected class members into an opener's class body.
    pub fn merge_class_members(item: &mut ModuleItem, members: Vec<ClassMember>) {
        let mut merger = ClassMemberMerger { members_to_add: members };
        item.visit_mut_with(&mut merger);
    }

    /// Merges collected type properties into an opener's type literal.
    pub fn merge_type_props(item: &mut ModuleItem, props: Vec<TsTypeElement>) {
        let mut merger = TypePropMerger { props_to_add: props };
        item.visit_mut_with(&mut merger);
    }

    /// Visitor that merges properties into the first object literal found.
    struct ObjectPropMerger {
        props_to_add: Vec<PropOrSpread>,
    }

    impl VisitMut for ObjectPropMerger {
        fn visit_mut_object_lit(&mut self, obj: &mut ObjectLit) {
            if !self.props_to_add.is_empty() {
                // Remove __mf_dummy property if present
                obj.props.retain(|prop| {
                    if let PropOrSpread::Prop(p) = prop {
                        if let Prop::KeyValue(KeyValueProp { key: PropName::Ident(ident), .. }) = p.as_ref() {
                            if ident.sym.as_ref() == "__mf_dummy" {
                                return false;
                            }
                        }
                    }
                    true
                });
                // Add collected properties
                obj.props.extend(std::mem::take(&mut self.props_to_add));
            }
        }
    }

    /// Visitor that merges elements into the first array literal found.
    struct ArrayElemMerger {
        elems_to_add: Vec<ExprOrSpread>,
    }

    impl VisitMut for ArrayElemMerger {
        fn visit_mut_array_lit(&mut self, arr: &mut ArrayLit) {
            if !self.elems_to_add.is_empty() {
                // Remove dummy element (first element with value 0) if present
                if let Some(first) = arr.elems.first() {
                    if let Some(ExprOrSpread { expr, .. }) = first {
                        if let Expr::Lit(Lit::Num(Number { value, .. })) = expr.as_ref() {
                            if *value == 0.0 {
                                arr.elems.remove(0);
                            }
                        }
                    }
                }
                // Add collected elements
                for elem in std::mem::take(&mut self.elems_to_add) {
                    arr.elems.push(Some(elem));
                }
            }
        }
    }

    /// Visitor that merges members into the first class found.
    struct ClassMemberMerger {
        members_to_add: Vec<ClassMember>,
    }

    impl VisitMut for ClassMemberMerger {
        fn visit_mut_class(&mut self, class: &mut Class) {
            if !self.members_to_add.is_empty() {
                class.body.extend(std::mem::take(&mut self.members_to_add));
            }
        }
    }

    /// Visitor that merges type properties into the first type literal found.
    struct TypePropMerger {
        props_to_add: Vec<TsTypeElement>,
    }

    impl VisitMut for TypePropMerger {
        fn visit_mut_ts_type_lit(&mut self, lit: &mut TsTypeLit) {
            if !self.props_to_add.is_empty() {
                lit.members.extend(std::mem::take(&mut self.props_to_add));
            }
        }
    }

    // =========================================================================
    // Existing type replacement helpers
    // =========================================================================

    /// Visitor that replaces marker type references with actual types.
    struct TypeReplacer {
        /// Mapping from placeholder names to actual types
        replacements: HashMap<String, TsType>,
    }

    impl VisitMut for TypeReplacer {
        fn visit_mut_ts_type(&mut self, ty: &mut TsType) {
            // First, check if this is a type reference to one of our markers
            if let TsType::TsTypeRef(TsTypeRef {
                type_name: TsEntityName::Ident(ident),
                type_params: None,
                ..
            }) = ty
            {
                let name = ident.sym.to_string();
                if let Some(replacement) = self.replacements.get(&name) {
                    *ty = replacement.clone();
                    return;
                }
            }
            // Continue visiting children
            ty.visit_mut_children_with(self);
        }
    }

    /// Replaces marker type references in a ModuleItem with actual types.
    ///
    /// This is used for type placeholder substitution. SWC quote! doesn't
    /// support `$placeholder` in type positions, so we use marker names like
    /// `__ph_0` and replace them after parsing.
    ///
    /// # Arguments
    /// * `item` - The ModuleItem to transform (modified in-place)
    /// * `replacements` - Map of placeholder names to actual TsType values
    pub fn replace_type_markers(item: &mut ModuleItem, replacements: HashMap<String, TsType>) {
        let mut replacer = TypeReplacer { replacements };
        item.visit_mut_with(&mut replacer);
    }

    /// Replaces marker type references in a Statement with actual types.
    pub fn replace_type_markers_stmt(stmt: &mut Stmt, replacements: HashMap<String, TsType>) {
        let mut replacer = TypeReplacer { replacements };
        stmt.visit_mut_with(&mut replacer);
    }

    /// Handle an opener chunk (template with unclosed braces).
    ///
    /// This is called when a template like `export function foo() { const x = 1;`
    /// was parsed with virtual closing braces. We push the item to output and
    /// return the index so the closer can find it.
    ///
    /// # Arguments
    /// * `item` - The parsed ModuleItem (function/class with virtual body)
    /// * `output` - The accumulator Vec<ModuleItem>
    /// * `depth` - Number of virtual closing braces that were added
    ///
    /// # Returns
    /// The index of the pushed item in output
    pub fn push_opener(item: ModuleItem, output: &mut Vec<ModuleItem>, _depth: usize) -> usize {
        let idx = output.len();
        output.push(item);
        idx
    }

    /// Handle a closer chunk (template with unmatched closing braces).
    ///
    /// This is called when a template like `return x; }` was parsed with a
    /// virtual function wrapper. We extract the statements and add them to
    /// the opener's body.
    ///
    /// # Arguments
    /// * `item` - The parsed ModuleItem (virtual wrapper containing actual statements)
    /// * `output` - The accumulator Vec<ModuleItem>
    /// * `opener_idx` - The index of the opener item (from push_opener)
    /// * `_depth` - Number of virtual opening braces that were added
    pub fn finalize_closer(
        item: ModuleItem,
        output: &mut Vec<ModuleItem>,
        opener_idx: usize,
    ) {
        // Extract statements from the virtual wrapper (function __mf_virtual() { ... })
        let closer_stmts = extract_function_body_statements(&item);

        // Collect all items pushed after the opener (these are statements from control flow)
        let intermediate_items: Vec<ModuleItem> = output.drain((opener_idx + 1)..).collect();

        // Convert intermediate ModuleItems to Statements
        let intermediate_stmts: Vec<Stmt> = intermediate_items
            .into_iter()
            .filter_map(|mi| match mi {
                ModuleItem::Stmt(stmt) => Some(stmt),
                _ => None, // Skip module declarations (shouldn't happen in function body)
            })
            .collect();

        // Extend the opener's body with intermediate + closer statements
        if let Some(opener) = output.get_mut(opener_idx) {
            let all_stmts: Vec<Stmt> =
                intermediate_stmts.into_iter().chain(closer_stmts).collect();
            extend_item_body(opener, all_stmts);
        }
    }

    /// Handle a middle chunk (template with both unclosed and unmatched braces).
    ///
    /// This is for cases like `} else {` which close one block and open another.
    /// We extract the statements and add them to the opener's body.
    ///
    /// # Arguments
    /// * `item` - The parsed ModuleItem (virtual wrapper)
    /// * `output` - The accumulator Vec<ModuleItem>
    /// * `opener_idx` - The index of the opener item
    pub fn push_middle(item: ModuleItem, output: &mut Vec<ModuleItem>, opener_idx: usize) {
        // Extract statements from the virtual wrapper
        let stmts = extract_function_body_statements(&item);

        // Collect intermediate items
        let intermediate_items: Vec<ModuleItem> = output.drain((opener_idx + 1)..).collect();

        let intermediate_stmts: Vec<Stmt> = intermediate_items
            .into_iter()
            .filter_map(|mi| match mi {
                ModuleItem::Stmt(stmt) => Some(stmt),
                _ => None,
            })
            .collect();

        // Extend the opener's body
        if let Some(opener) = output.get_mut(opener_idx) {
            let all_stmts: Vec<Stmt> = intermediate_stmts.into_iter().chain(stmts).collect();
            extend_item_body(opener, all_stmts);
        }
    }

    /// Extract the body statements from a function declaration.
    fn extract_function_body_statements(item: &ModuleItem) -> Vec<Stmt> {
        match item {
            // function foo() { ... }
            ModuleItem::Stmt(Stmt::Decl(Decl::Fn(FnDecl { function, .. }))) => {
                if let Some(body) = &function.body {
                    body.stmts.clone()
                } else {
                    vec![]
                }
            }
            // export function foo() { ... }
            ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
                decl: Decl::Fn(FnDecl { function, .. }),
                ..
            })) => {
                if let Some(body) = &function.body {
                    body.stmts.clone()
                } else {
                    vec![]
                }
            }
            _ => vec![],
        }
    }

    /// Extend the body of a function or class method with additional statements.
    fn extend_item_body(item: &mut ModuleItem, stmts: Vec<Stmt>) {
        match item {
            // export function foo() { ... }
            ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
                decl: Decl::Fn(FnDecl { function, .. }),
                ..
            })) => {
                if let Some(ref mut body) = function.body {
                    body.stmts.extend(stmts);
                }
            }

            // export default function() { ... }
            ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(ExportDefaultDecl {
                decl: DefaultDecl::Fn(FnExpr { function, .. }),
                ..
            })) => {
                if let Some(ref mut body) = function.body {
                    body.stmts.extend(stmts);
                }
            }

            // function foo() { ... }
            ModuleItem::Stmt(Stmt::Decl(Decl::Fn(FnDecl { function, .. }))) => {
                if let Some(ref mut body) = function.body {
                    body.stmts.extend(stmts);
                }
            }

            // export class Foo { method() { ... } }
            ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
                decl: Decl::Class(ClassDecl { class, .. }),
                ..
            })) => {
                // Find the last method and extend its body
                for member in class.body.iter_mut().rev() {
                    if let ClassMember::Method(ClassMethod { function, .. }) = member {
                        if let Some(ref mut body) = function.body {
                            body.stmts.extend(stmts);
                            return;
                        }
                    }
                }
            }

            // class Foo { ... }
            ModuleItem::Stmt(Stmt::Decl(Decl::Class(ClassDecl { class, .. }))) => {
                for member in class.body.iter_mut().rev() {
                    if let ClassMember::Method(ClassMethod { function, .. }) = member {
                        if let Some(ref mut body) = function.body {
                            body.stmts.extend(stmts);
                            return;
                        }
                    }
                }
            }

            _ => {
                // For other cases, we can't extend the body
            }
        }
    }
}

/// Converts a statement to its TypeScript string representation.
#[cfg(feature = "swc")]
pub fn stmt_to_string(stmt: &swc_core::ecma::ast::Stmt) -> String {
    use swc_core::common::sync::Lrc;
    use swc_core::ecma::ast::{Module, ModuleItem};
    use swc_core::ecma::codegen::{text_writer::JsWriter, Config, Emitter};

    let module = Module {
        span: swc_core::common::DUMMY_SP,
        body: vec![ModuleItem::Stmt(stmt.clone())],
        shebang: None,
    };

    let cm = Lrc::new(swc_core::common::SourceMap::default());
    let mut buf = Vec::new();
    {
        let mut emitter = Emitter {
            cfg: Config::default(),
            cm: cm.clone(),
            comments: None,
            wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
        };
        let _ = emitter.emit_module(&module);
    }
    String::from_utf8(buf).unwrap_or_default().trim().to_string()
}

/// Parses a TypeScript string into a Vec of ModuleItems.
///
/// This is used by the template compiler to parse generated code at runtime.
#[cfg(feature = "swc")]
pub fn parse_ts_to_module_items(source: &str) -> Vec<swc_core::ecma::ast::ModuleItem> {
    use swc_core::common::{FileName, SourceMap, sync::Lrc};
    use swc_core::ecma::parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};

    let cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
    let fm = cm.new_source_file(
        FileName::Custom("template.ts".into()).into(),
        source.to_string(),
    );
    let syntax = Syntax::Typescript(TsSyntax {
        tsx: true,
        decorators: true,
        ..Default::default()
    });
    let lexer = Lexer::new(
        syntax,
        swc_core::ecma::ast::EsVersion::latest(),
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    match parser.parse_module() {
        Ok(module) => module.body,
        Err(_) => vec![],
    }
}

/// Extends the body of a function or class with additional statements.
///
/// This helper is used by the generated code for virtually-completed chunks.
/// When a template has control flow inside a function body, the function is
/// parsed with an empty body and this function extends it with the actual
/// statements collected at runtime.
///
/// # Arguments
///
/// * `item` - The `ModuleItem` containing a function or class declaration
/// * `stmts` - The statements to add to the function/class body
///
/// # Supported Structures
///
/// - `export function ...` - Extends the function body
/// - `export class ... { method() {} }` - Extends the last method's body
/// - `function ...` (non-export) - Extends the function body
#[cfg(feature = "swc")]
pub fn extend_module_item_body(
    item: &mut swc_core::ecma::ast::ModuleItem,
    stmts: Vec<swc_core::ecma::ast::Stmt>,
) {
    use swc_core::ecma::ast::*;

    match item {
        // export function foo() { ... }
        ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
            decl: Decl::Fn(FnDecl { function, .. }),
            ..
        })) => {
            if let Some(ref mut body) = function.body {
                body.stmts.extend(stmts);
            }
        }

        // export default function() { ... }
        ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(ExportDefaultDecl {
            decl: DefaultDecl::Fn(FnExpr { function, .. }),
            ..
        })) => {
            if let Some(ref mut body) = function.body {
                body.stmts.extend(stmts);
            }
        }

        // function foo() { ... }
        ModuleItem::Stmt(Stmt::Decl(Decl::Fn(FnDecl { function, .. }))) => {
            if let Some(ref mut body) = function.body {
                body.stmts.extend(stmts);
            }
        }

        // export class Foo { method() { ... } }
        ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
            decl: Decl::Class(ClassDecl { class, .. }),
            ..
        })) => {
            // Find the last method and extend its body
            for member in class.body.iter_mut().rev() {
                if let ClassMember::Method(ClassMethod { function, .. }) = member {
                    if let Some(ref mut body) = function.body {
                        body.stmts.extend(stmts);
                        return;
                    }
                }
            }
        }

        // class Foo { ... }
        ModuleItem::Stmt(Stmt::Decl(Decl::Class(ClassDecl { class, .. }))) => {
            // Find the last method and extend its body
            for member in class.body.iter_mut().rev() {
                if let ClassMember::Method(ClassMethod { function, .. }) = member {
                    if let Some(ref mut body) = function.body {
                        body.stmts.extend(stmts);
                        return;
                    }
                }
            }
        }

        // Arrow function expression or other constructs
        _ => {
            // For other cases, we can't easily extend the body
            // Just ignore - the statements won't be added
        }
    }
}
