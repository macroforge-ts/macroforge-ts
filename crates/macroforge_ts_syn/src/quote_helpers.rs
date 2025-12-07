//! Helper macros for ergonomic code generation with ts_quote!

/// Create a method assignment on a prototype with Vec<Stmt> body.
/// This is a more ergonomic wrapper around fn_assign! for the common case.
///
/// # Examples
///
/// ```ignore
/// let body_stmts = vec![
///     ts_quote!( const result = {}; as Stmt ),
///     ts_quote!( return result; as Stmt ),
/// ];
///
/// // Simple: Class.prototype.method = function() { body }
/// let stmt = proto_method!(MyClass, "toJSON", body_stmts);
///
/// // With dynamic class name:
/// let class_ident = ident!("User");
/// let stmt = proto_method!(class_ident, "toString", body_stmts);
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
