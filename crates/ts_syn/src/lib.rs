pub mod derive;
pub mod errors;
pub mod lower;
pub mod parse;
pub mod quote_helpers;
pub mod stream;

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

/// Create a block statement from a Vec<Stmt>
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

/// Helper to pass Vec<Stmt> to be used inline in function bodies
/// This is a marker type that ts_quote! can detect and handle specially
#[cfg(feature = "swc")]
pub struct StmtVec(pub Vec<swc_core::ecma::ast::Stmt>);

#[cfg(feature = "swc")]
#[macro_export]
macro_rules! stmt_vec {
    ($stmts:expr) => {
        ts_syn::StmtVec($stmts)
    };
}

/// Convert a Vec<Stmt> into a single block statement that can be used in ts_quote!
/// This allows you to interpolate multiple statements where a single statement is expected.
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

/// Create a function expression with the given body statements
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

/// Create a member expression: obj.prop
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

/// Create an assignment expression statement: lhs = rhs;
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

/// Create a function assignment: obj.prop = function(params) { body };
/// Usage: fn_assign!(ClassName.prototype.methodName, params, body_stmts)
/// Or: fn_assign!(ClassName.prototype.methodName, body_stmts) for no params
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
