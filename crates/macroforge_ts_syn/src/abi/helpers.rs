//! Helper utilities for working with patches and spans.
//!
//! This module provides convenience functions for common patching operations.
//! These helpers abstract away the details of span arithmetic and patch
//! construction for typical use cases.
//!
//! ## Common Operations
//!
//! | Function | Use Case |
//! |----------|----------|
//! | [`insert_into_class`] | Add a method or property to a class |
//! | [`insert_class_member`] | Insert an SWC ClassMember AST node |
//!
//! ## Example
//!
//! ```rust,no_run
//! use macroforge_ts_syn::{insert_into_class, ClassIR, MacroResult};
//!
//! fn generate_debug_method(class: &ClassIR) -> MacroResult {
//!     let method = format!(
//!         r#"debug(): string {{
//!             return "{}";
//!         }}"#,
//!         class.name
//!     );
//!
//!     MacroResult {
//!         runtime_patches: vec![insert_into_class(class.body_span, method)],
//!         ..Default::default()
//!     }
//! }
//! ```

use crate::abi::*;

/// Creates a patch to insert code inside a class body.
///
/// This helper calculates the correct insertion point (just before the
/// closing `}` brace) and creates an `Insert` patch. It's the recommended
/// way to add methods, properties, or other members to a class.
///
/// # Arguments
///
/// - `class_span` - The `body_span` of the class (from [`ClassIR::body_span`])
/// - `code` - The code to insert (string, `PatchCode`, or any type implementing `Into<PatchCode>`)
///
/// # Returns
///
/// A [`Patch::Insert`] positioned at `class_span.end - 1` (before the closing brace).
///
/// # Example
///
/// ```rust
/// use macroforge_ts_syn::{insert_into_class, Patch, SpanIR};
///
/// // Create a patch to insert code into a class body
/// let class_body_span = SpanIR::new(10, 50);
/// let _patch: Patch = insert_into_class(class_body_span, "toString() { return 'MyClass'; }");
///
/// // Given a class like:
/// // class Foo {
/// //     name: string;
/// // }
/// //
/// // The patch inserts at position marked with |:
/// // class Foo {
/// //     name: string;
/// //     |
/// // }
/// ```
///
/// # Note
///
/// The `source_macro` field of the returned patch is `None`. Use
/// [`Patch::with_source_macro`] to set it for debugging purposes:
///
/// ```rust
/// use macroforge_ts_syn::{insert_into_class, SpanIR};
///
/// let class_body_span = SpanIR::new(10, 50);
/// let code = "toString() { return 'MyClass'; }";
/// let _patch = insert_into_class(class_body_span, code)
///     .with_source_macro("Debug");
/// ```
pub fn insert_into_class(class_span: SpanIR, code: impl Into<PatchCode>) -> Patch {
    let insert_at = class_span.end.saturating_sub(1);
    Patch::Insert {
        at: SpanIR::new(insert_at, insert_at),
        code: code.into(),
        source_macro: None,
    }
}

/// Creates a patch to insert an SWC `ClassMember` into a class body.
///
/// This is a convenience wrapper around [`insert_into_class`] that accepts
/// an SWC [`ClassMember`](swc_ast::ClassMember) AST node directly.
///
/// # Arguments
///
/// - `class_span` - The `body_span` of the class
/// - `member` - An SWC `ClassMember` (method, property, constructor, etc.)
///
/// # Example
///
/// ```rust,ignore
/// use macroforge_ts_syn::{insert_class_member, ident, ClassIR};
/// use swc_core::ecma::ast::*;
/// use swc_core::common::DUMMY_SP;
///
/// fn add_property(class: &ClassIR) -> Patch {
///     let prop = ClassMember::ClassProp(ClassProp {
///         span: DUMMY_SP,
///         key: PropName::Ident(ident!("generated")),
///         value: Some(Box::new(Expr::Lit(Lit::Bool(Bool {
///             span: DUMMY_SP,
///             value: true,
///         })))),
///         // ... other fields
///     });
///
///     insert_class_member(class.body_span, prop)
/// }
/// ```
pub fn insert_class_member(class_span: SpanIR, member: swc_ast::ClassMember) -> Patch {
    insert_into_class(class_span, PatchCode::from(member))
}
