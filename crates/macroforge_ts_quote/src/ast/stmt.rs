use swc_core::ecma::ast::*;

impl_enum!(
    Stmt,
    [
        Block, Empty, Debugger, With, Return, Labeled, Break, Continue, If, Switch, Throw, Try,
        While, DoWhile, For, ForIn, ForOf, Decl, Expr
    ]
);

impl_struct!(EmptyStmt, [span]);
impl_struct!(BlockStmt, [span, ctxt, stmts]);
impl_struct!(DebuggerStmt, [span]);
impl_struct!(WithStmt, [span, obj, body]);
impl_struct!(LabeledStmt, [span, label, body]);
impl_struct!(BreakStmt, [span, label]);
impl_struct!(ContinueStmt, [span, label]);
impl_struct!(IfStmt, [span, test, cons, alt]);
impl_struct!(SwitchStmt, [span, discriminant, cases]);
impl_struct!(ThrowStmt, [span, arg]);
impl_struct!(TryStmt, [span, block, handler, finalizer]);
impl_struct!(WhileStmt, [span, test, body]);
impl_struct!(DoWhileStmt, [span, test, body]);
impl_struct!(ForStmt, [span, init, test, update, body]);
impl_struct!(ForInStmt, [span, left, right, body]);
impl_struct!(ForOfStmt, [span, is_await, left, right, body]);
impl_struct!(ReturnStmt, [span, arg]);
impl_struct!(ExprStmt, [span, expr]);

impl_enum!(VarDeclOrExpr, [VarDecl, Expr]);
impl_enum!(ForHead, [VarDecl, UsingDecl, Pat]);

impl_struct!(SwitchCase, [span, test, cons]);

impl_struct!(CatchClause, [span, param, body]);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ToCode;
    use crate::ctxt::Ctx;
    use quote::ToTokens;
    use rustc_hash::FxHashMap;
    use swc_core::atoms::Atom;
    use swc_core::common::{Span, SyntaxContext};

    /// Helper to create an empty context for testing
    fn empty_ctx() -> Ctx {
        Ctx {
            vars: FxHashMap::default(),
        }
    }

    // ==================== Stmt Enum Tests ====================

    #[test]
    fn test_stmt_empty() {
        let cx = empty_ctx();
        let stmt = Stmt::Empty(EmptyStmt {
            span: Span::default(),
        });
        let code = stmt.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Stmt"));
        assert!(code_str.contains("Empty"));
    }

    #[test]
    fn test_stmt_debugger() {
        let cx = empty_ctx();
        let stmt = Stmt::Debugger(DebuggerStmt {
            span: Span::default(),
        });
        let code = stmt.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Debugger"));
    }

    // ==================== EmptyStmt Tests ====================

    #[test]
    fn test_empty_stmt_to_code() {
        let cx = empty_ctx();
        let stmt = EmptyStmt {
            span: Span::default(),
        };
        let code = stmt.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("EmptyStmt"));
        assert!(code_str.contains("span"));
    }

    // ==================== BlockStmt Tests ====================

    #[test]
    fn test_block_stmt_empty() {
        let cx = empty_ctx();
        let stmt = BlockStmt {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            stmts: vec![],
        };
        let code = stmt.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("BlockStmt"));
        assert!(code_str.contains("stmts"));
    }

    #[test]
    fn test_block_stmt_with_statements() {
        let cx = empty_ctx();
        let inner_stmt = Stmt::Empty(EmptyStmt {
            span: Span::default(),
        });
        let stmt = BlockStmt {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            stmts: vec![inner_stmt],
        };
        let code = stmt.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("BlockStmt"));
    }

    // ==================== IfStmt Tests ====================

    #[test]
    fn test_if_stmt_without_else() {
        let cx = empty_ctx();
        let stmt = IfStmt {
            span: Span::default(),
            test: Box::new(Expr::Lit(Lit::Bool(Bool {
                span: Span::default(),
                value: true,
            }))),
            cons: Box::new(Stmt::Empty(EmptyStmt {
                span: Span::default(),
            })),
            alt: None,
        };
        let code = stmt.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("IfStmt"));
        assert!(code_str.contains("test"));
        assert!(code_str.contains("cons"));
    }

    #[test]
    fn test_if_stmt_with_else() {
        let cx = empty_ctx();
        let stmt = IfStmt {
            span: Span::default(),
            test: Box::new(Expr::Lit(Lit::Bool(Bool {
                span: Span::default(),
                value: true,
            }))),
            cons: Box::new(Stmt::Empty(EmptyStmt {
                span: Span::default(),
            })),
            alt: Some(Box::new(Stmt::Empty(EmptyStmt {
                span: Span::default(),
            }))),
        };
        let code = stmt.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("alt"));
    }

    // ==================== ReturnStmt Tests ====================

    #[test]
    fn test_return_stmt_without_value() {
        let cx = empty_ctx();
        let stmt = ReturnStmt {
            span: Span::default(),
            arg: None,
        };
        let code = stmt.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ReturnStmt"));
    }

    #[test]
    fn test_return_stmt_with_value() {
        let cx = empty_ctx();
        let stmt = ReturnStmt {
            span: Span::default(),
            arg: Some(Box::new(Expr::Lit(Lit::Num(Number {
                span: Span::default(),
                value: 42.0,
                raw: Some(Atom::from("42")),
            })))),
        };
        let code = stmt.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("arg"));
    }

    // ==================== ThrowStmt Tests ====================

    #[test]
    fn test_throw_stmt() {
        let cx = empty_ctx();
        let stmt = ThrowStmt {
            span: Span::default(),
            arg: Box::new(Expr::Lit(Lit::Str(Str {
                span: Span::default(),
                value: Atom::from("error").into(),
                raw: None,
            }))),
        };
        let code = stmt.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ThrowStmt"));
        assert!(code_str.contains("arg"));
    }

    // ==================== WhileStmt Tests ====================

    #[test]
    fn test_while_stmt() {
        let cx = empty_ctx();
        let stmt = WhileStmt {
            span: Span::default(),
            test: Box::new(Expr::Lit(Lit::Bool(Bool {
                span: Span::default(),
                value: true,
            }))),
            body: Box::new(Stmt::Empty(EmptyStmt {
                span: Span::default(),
            })),
        };
        let code = stmt.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("WhileStmt"));
        assert!(code_str.contains("test"));
        assert!(code_str.contains("body"));
    }

    // ==================== ForStmt Tests ====================

    #[test]
    fn test_for_stmt() {
        let cx = empty_ctx();
        let stmt = ForStmt {
            span: Span::default(),
            init: None,
            test: None,
            update: None,
            body: Box::new(Stmt::Empty(EmptyStmt {
                span: Span::default(),
            })),
        };
        let code = stmt.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ForStmt"));
    }

    // ==================== ExprStmt Tests ====================

    #[test]
    fn test_expr_stmt() {
        let cx = empty_ctx();
        let stmt = ExprStmt {
            span: Span::default(),
            expr: Box::new(Expr::Lit(Lit::Num(Number {
                span: Span::default(),
                value: 1.0,
                raw: None,
            }))),
        };
        let code = stmt.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ExprStmt"));
        assert!(code_str.contains("expr"));
    }

    // ==================== TryStmt Tests ====================

    #[test]
    fn test_try_stmt_basic() {
        let cx = empty_ctx();
        let stmt = TryStmt {
            span: Span::default(),
            block: BlockStmt {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            },
            handler: None,
            finalizer: None,
        };
        let code = stmt.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TryStmt"));
        assert!(code_str.contains("block"));
    }

    // ==================== SwitchCase Tests ====================

    #[test]
    fn test_switch_case() {
        let cx = empty_ctx();
        let case = SwitchCase {
            span: Span::default(),
            test: Some(Box::new(Expr::Lit(Lit::Num(Number {
                span: Span::default(),
                value: 1.0,
                raw: None,
            })))),
            cons: vec![],
        };
        let code = case.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("SwitchCase"));
        assert!(code_str.contains("test"));
        assert!(code_str.contains("cons"));
    }

    // ==================== CatchClause Tests ====================

    #[test]
    fn test_catch_clause() {
        let cx = empty_ctx();
        let clause = CatchClause {
            span: Span::default(),
            param: None,
            body: BlockStmt {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            },
        };
        let code = clause.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("CatchClause"));
        assert!(code_str.contains("body"));
    }

    // ==================== VarDeclOrExpr Tests ====================

    #[test]
    fn test_var_decl_or_expr_expr() {
        let cx = empty_ctx();
        let val = VarDeclOrExpr::Expr(Box::new(Expr::Lit(Lit::Num(Number {
            span: Span::default(),
            value: 1.0,
            raw: None,
        }))));
        let code = val.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("VarDeclOrExpr"));
        assert!(code_str.contains("Expr"));
    }
}
