//! Helper macros for ergonomic code generation.
//!
//! This module provides convenience macros for common code generation
//! patterns when building macro implementations. These macros work with
//! the AST construction macros from [`lib.rs`](crate) and SWC's AST types.
//!
//! ## Available Macros
//!
//! | Macro | Purpose |
//! |-------|---------|
//! | `proto_method!` | Assign a method to a class prototype |
//!
//! ## Common Patterns
//!
//! ### Adding Methods to Classes
//!
//! Use `proto_method!` to generate `Class.prototype.method = function() {...}`:
//!
//! ```rust,ignore
//! use macroforge_ts_syn::{proto_method, ident, stmt_vec};
//!
//! // Generate: User.prototype.toJSON = function() { return {...}; }
//! let class_name = ident!("User");
//! let body = stmt_vec![
//!     ts_quote!( const result = {}; as Stmt ),
//!     ts_quote!( return result; as Stmt ),
//! ];
//!
//! let stmt = proto_method!(class_name, "toJSON", body);
//! ```
//!
//! ## Related Macros
//!
//! For constructing AST nodes, see the macros in [`lib.rs`](crate):
//! - [`ident!`](crate::ident!) - Create identifier expressions
//! - [`member_expr!`](crate::member_expr!) - Create member access expressions
//! - [`fn_assign!`](crate::fn_assign!) - Create function assignment statements
//! - [`stmt_vec!`](crate::stmt_vec!) - Create vectors of statements

/// Creates a method assignment on a class prototype.
///
/// This macro generates code of the form:
/// ```typescript
/// ClassName.prototype.methodName = function() { /* body */ }
/// ```
///
/// This is the standard pattern for adding methods to classes in JavaScript/TypeScript
/// when you need to add methods outside the class declaration (e.g., in macro-generated code).
///
/// # Arguments
///
/// - `$class` - An identifier expression for the class name (use [`ident!`](crate::ident!))
/// - `$method` - A string literal for the method name
/// - `$body` - A `Vec<Stmt>` containing the function body
///
/// # Example: Basic Usage
///
/// ```rust,ignore
/// use macroforge_ts_syn::{proto_method, ident};
///
/// // Generate: MyClass.prototype.toString = function() { return "MyClass"; }
/// let body_stmts = vec![
///     ts_quote!( return "MyClass"; as Stmt ),
/// ];
///
/// let stmt = proto_method!(ident!("MyClass"), "toString", body_stmts);
/// ```
///
/// # Example: In a Derive Macro
///
/// ```rust,ignore
/// fn derive_debug(input: &DeriveInput) -> MacroResult {
///     let class = input.as_class().expect("Expected class");
///     let class_name = ident!(&input.name());
///
///     // Build the debug() method body
///     let body = vec![
///         ts_quote!( return `${#class_name} { ... }`; as Stmt ),
///     ];
///
///     let method_stmt = proto_method!(class_name, "debug", body);
///
///     // Return patch that inserts this method after the class
///     MacroResult {
///         runtime_patches: vec![
///             Patch::InsertAfter {
///                 target: input.target_span(),
///                 code: method_stmt.into(),
///                 source_macro: Some("Debug".into()),
///             }
///         ],
///         ..Default::default()
///     }
/// }
/// ```
///
/// # Under the Hood
///
/// This macro expands to:
/// ```rust,ignore
/// fn_assign!(
///     member_expr!(Expr::Ident(class), "prototype"),
///     method_name,
///     body
/// )
/// ```
#[cfg(feature = "swc")]
#[macro_export]
macro_rules! proto_method {
    ($class:expr, $method:expr, $body:expr) => {{
        $crate::fn_assign!(
            $crate::member_expr!(swc_core::ecma::ast::Expr::Ident($class), "prototype"),
            $method,
            $body
        )
    }};
}
