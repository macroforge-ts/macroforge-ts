//! Thin wrappers around SWC's `quote!` machinery tailored for ts-macros.
//!
//! Macro authors can depend on this crate to access the familiar `quote!`
//! macro (re-exported from `swc_core`) plus a couple of helpers for creating
//! identifiers with predictable hygiene. The goal is to decouple codegen
//! utilities from the heavier parsing utilities that live in `ts_syn`.

#![deny(missing_docs)]

#[cfg(feature = "swc")]
pub use swc_core::quote;

#[cfg(feature = "swc")]
pub use swc_core::{common, ecma::ast};

/// Create an identifier without syntax context, mirroring `syn::Ident::new`.
#[cfg(feature = "swc")]
#[macro_export]
macro_rules! ident {
    ($name:expr) => {
        swc_core::ecma::ast::Ident::new_no_ctxt($name.into(), swc_core::common::DUMMY_SP)
    };
}

/// Create a unique private identifier using a fresh mark.
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

#[cfg(all(test, feature = "swc"))]
mod tests {
    use super::*;
    use crate::quote;
    use swc_core::common::{GLOBALS, Globals};

    #[test]
    fn quote_roundtrip() {
        use swc_core::ecma::ast::Stmt;

        GLOBALS.set(&Globals::new(), || {
            let name = ident!("myVar");
            let stmt: Stmt = quote!("const $name = 42;" as Stmt, name = name);
            match stmt {
                Stmt::Decl(_) => {}
                _ => panic!("expected declaration"),
            }
        });
    }

    #[test]
    fn private_ident_is_unique() {
        GLOBALS.set(&Globals::new(), || {
            let first = private_ident!("_foo");
            let second = private_ident!("_foo");
            assert_ne!(first.to_id(), second.to_id());
        });
    }
}
