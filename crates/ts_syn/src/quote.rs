//! Quote macro for generating TypeScript/JavaScript AST.
//!
//! This module re-exports `swc_core::quote` to provide a `quote!`-like
//! experience similar to Rust's `quote` crate.
//!
//! # Example
//! ```ignore
//! use ts_syn::quote;
//! use swc_core::ecma::ast::*;
//! use swc_core::common::DUMMY_SP;
//!
//! // Generate: const myVar = 42;
//! let stmt: Stmt = quote!(
//!     "const $name = $value;" as Stmt,
//!     name = Ident::new_no_ctxt("myVar".into(), DUMMY_SP),
//!     value: Expr = 42.into(),
//! );
//! ```
//!
//! # Interpolation Syntax
//!
//! Variables are prefixed with `$`:
//! - `$name` - Substitutes an `Ident`
//! - `$expr: Expr` - Substitutes an expression with explicit type
//! - `$str: Str` - Substitutes a string literal
//!
//! # Supported Output Types
//! - `Expr` / `Box<Expr>`
//! - `Stmt`
//! - `ModuleItem`
//! - `Pat`
//! - `AssignTarget`

#[cfg(feature = "swc")]
pub use swc_core::quote;

/// Helper to create an identifier with a dummy span (no syntax context).
///
/// # Example
/// ```ignore
/// use ts_syn::ident;
///
/// let id = ident!("myVariable");
/// ```
#[macro_export]
macro_rules! ident {
    ($name:expr) => {
        $crate::swc_core::ecma::ast::Ident::new_no_ctxt($name.into(), $crate::swc_core::common::DUMMY_SP)
    };
}

/// Helper to create a private identifier (for use in generated code).
///
/// This creates an identifier that won't conflict with user code.
#[macro_export]
macro_rules! private_ident {
    ($name:expr) => {{
        use $crate::swc_core::common::Mark;
        $crate::swc_core::ecma::ast::Ident::new(
            $name.into(),
            $crate::swc_core::common::DUMMY_SP,
            $crate::swc_core::common::SyntaxContext::empty().apply_mark(Mark::new()),
        )
    }};
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "swc")]
    #[test]
    fn test_quote_stmt() {
        use crate::swc_core::common::DUMMY_SP;
        use crate::swc_core::ecma::ast::{Ident, Stmt};
        use crate::quote;

        let name = Ident::new_no_ctxt("myVar".into(), DUMMY_SP);
        let _stmt: Stmt = quote!("const $name = 42;" as Stmt, name = name);
    }

    #[cfg(feature = "swc")]
    #[test]
    fn test_ident_macro() {
        use crate::ident;
        let id = ident!("testIdent");
        assert_eq!(id.sym.as_ref(), "testIdent");
    }
}
