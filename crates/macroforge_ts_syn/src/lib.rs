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
//! use macroforge_ts_syn::{parse_ts_macro_input, DeriveInput, MacroResult, Patch};
//!
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
//!         Data::Interface(iface) => {
//!             // Handle interface...
//!         }
//!         Data::Enum(enum_) => {
//!             // Handle enum...
//!         }
//!         Data::TypeAlias(alias) => {
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

/// Creates an SWC [`Ident`](swc_core::ecma::ast::Ident) with a dummy span.
///
/// This macro provides a convenient way to create identifier AST nodes for
/// code generation. It supports both simple string names and format strings.
///
/// # Examples
///
/// Simple identifier:
///
/// ```rust,ignore
/// use macroforge_ts_syn::ident;
///
/// let id = ident!("myVariable");
/// assert_eq!(id.sym.as_str(), "myVariable");
/// ```
///
/// Formatted identifier:
///
/// ```rust,ignore
/// use macroforge_ts_syn::ident;
///
/// let field_name = "age";
/// let getter = ident!("get{}", field_name.to_uppercase());
/// assert_eq!(getter.sym.as_str(), "getAGE");
/// ```
///
/// Using in code generation:
///
/// ```rust,ignore
/// use macroforge_ts_syn::{ident, quote};
///
/// let class_name = ident!("MyClass");
/// let code = quote!("class $class_name {}" as Stmt, class_name: Ident = class_name);
/// ```
#[cfg(feature = "swc")]
#[macro_export]
macro_rules! ident {
    // Single argument - direct string
    ($name:expr) => {
        swc_core::ecma::ast::Ident::new_no_ctxt($name.into(), swc_core::common::DUMMY_SP)
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
/// ```rust,ignore
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
/// ```rust,ignore
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
/// ```rust,ignore
/// use macroforge_ts_syn::stmt_block;
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
/// ```rust,ignore
/// use macroforge_ts_syn::{StmtVec, stmt_vec};
/// use swc_core::ecma::ast::Stmt;
///
/// let statements: Vec<Stmt> = vec![/* ... */];
/// let wrapped = StmtVec(statements);
/// // Or using the macro:
/// let wrapped = stmt_vec!(statements);
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
/// ```rust,ignore
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
/// ```rust,ignore
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
///
/// let params = vec![
///     Param {
///         span: DUMMY_SP,
///         decorators: vec![],
///         pat: Pat::Ident(ident!("x").into()),
///     }
/// ];
/// let body: Vec<Stmt> = vec![/* ... */];
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
/// use swc_core::ecma::ast::Expr;
///
/// let this_expr = Expr::This(ThisExpr { span: DUMMY_SP });
/// let access = member_expr!(this_expr, "name");
/// // Generates: this.name
/// ```
///
/// Chaining member access:
///
/// ```rust,ignore
/// use macroforge_ts_syn::{member_expr, ident};
/// use swc_core::ecma::ast::Expr;
///
/// let obj = Expr::Ident(ident!("obj"));
/// let nested = member_expr!(member_expr!(obj, "foo"), "bar");
/// // Generates: obj.foo.bar
/// ```
///
/// Accessing prototype:
///
/// ```rust,ignore
/// use macroforge_ts_syn::{member_expr, ident};
///
/// let class = Expr::Ident(ident!("MyClass"));
/// let proto = member_expr!(class, "prototype");
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
/// use swc_core::ecma::ast::{AssignTarget, SimpleAssignTarget, Expr};
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
/// use swc_core::ecma::ast::{AssignTarget, SimpleAssignTarget};
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
/// ```rust,ignore
/// use macroforge_ts_syn::{fn_assign, ident, member_expr};
/// use swc_core::ecma::ast::{Expr, Stmt};
///
/// let class_expr = Expr::Ident(ident!("MyClass"));
/// let proto = member_expr!(class_expr, "prototype");
///
/// let body: Vec<Stmt> = vec![/* return statements */];
/// let stmt = fn_assign!(proto, "toString", body);
/// // Generates: MyClass.prototype.toString = function() { ... };
/// ```
///
/// Adding a method with parameters:
///
/// ```rust,ignore
/// use macroforge_ts_syn::{fn_assign, ident, member_expr};
/// use swc_core::ecma::ast::{Expr, Param, Pat, Stmt};
///
/// let class_expr = Expr::Ident(ident!("MyClass"));
/// let proto = member_expr!(class_expr, "prototype");
///
/// let params = vec![
///     Param { span: DUMMY_SP, decorators: vec![], pat: Pat::Ident(ident!("value").into()) }
/// ];
/// let body: Vec<Stmt> = vec![/* method body */];
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
