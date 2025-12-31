use swc_core::ecma::ast::*;

impl_enum!(AssignTarget, [Simple, Pat], true);
impl_enum!(
    SimpleAssignTarget,
    [
        Ident,
        Member,
        SuperProp,
        Paren,
        OptChain,
        TsAs,
        TsNonNull,
        TsSatisfies,
        TsTypeAssertion,
        TsInstantiation,
        Invalid
    ]
);
impl_enum!(AssignTargetPat, [Array, Object, Invalid]);
impl_enum!(
    Expr,
    [
        This,
        Array,
        Object,
        Fn,
        Unary,
        Update,
        Bin,
        Assign,
        Member,
        SuperProp,
        Cond,
        Call,
        New,
        Seq,
        Ident,
        Lit,
        Tpl,
        TaggedTpl,
        Arrow,
        Class,
        Yield,
        MetaProp,
        Await,
        Paren,
        JSXMember,
        JSXNamespacedName,
        JSXEmpty,
        JSXElement,
        JSXFragment,
        TsTypeAssertion,
        TsConstAssertion,
        TsNonNull,
        TsAs,
        TsInstantiation,
        TsSatisfies,
        PrivateName,
        OptChain,
        Invalid
    ],
    true
);

impl_struct!(ThisExpr, [span]);
impl_struct!(ArrayLit, [span, elems]);
impl_struct!(ObjectLit, [span, props]);
impl_struct!(FnExpr, [ident, function]);
impl_struct!(
    ArrowExpr,
    [
        span,
        ctxt,
        params,
        body,
        is_async,
        is_generator,
        type_params,
        return_type
    ]
);
impl_struct!(ClassExpr, [ident, class]);
impl_struct!(Tpl, [span, exprs, quasis]);
impl_struct!(UnaryExpr, [span, op, arg]);
impl_struct!(UpdateExpr, [span, op, prefix, arg]);
impl_struct!(BinExpr, [span, op, left, right]);
impl_struct!(AssignExpr, [span, op, left, right]);
impl_struct!(MemberExpr, [span, obj, prop]);
impl_struct!(SuperPropExpr, [span, obj, prop]);
impl_struct!(CondExpr, [span, test, cons, alt]);

impl_struct!(CallExpr, [span, ctxt, callee, args, type_args]);
impl_struct!(ExprOrSpread, [spread, expr]);
impl_struct!(Super, [span]);
impl_struct!(Import, [span, phase]);
impl_struct!(NewExpr, [span, ctxt, callee, args, type_args]);
impl_struct!(SeqExpr, [span, exprs]);

impl_struct!(TaggedTpl, [span, ctxt, tag, type_params, tpl]);
impl_struct!(YieldExpr, [span, arg, delegate]);
impl_struct!(MetaPropExpr, [span, kind]);
impl_struct!(AwaitExpr, [span, arg]);
impl_struct!(JSXMemberExpr, [span, obj, prop]);
impl_struct!(JSXNamespacedName, [span, ns, name]);
impl_struct!(JSXEmptyExpr, [span]);
impl_struct!(JSXElement, [span, opening, closing, children]);
impl_struct!(JSXFragment, [span, opening, closing, children]);
impl_struct!(OptChainExpr, [span, optional, base]);

impl_struct!(ParenExpr, [span, expr]);
impl_struct!(
    Function,
    [
        ctxt,
        params,
        decorators,
        span,
        body,
        is_generator,
        is_async,
        type_params,
        return_type
    ]
);
impl_struct!(Decorator, [span, expr]);

impl_struct!(TplElement, [span, tail, cooked, raw]);

impl_struct!(
    JSXOpeningElement,
    [name, span, attrs, self_closing, type_args]
);
impl_struct!(JSXClosingElement, [name, span]);

impl_struct!(JSXOpeningFragment, [span]);
impl_struct!(JSXClosingFragment, [span]);

impl_struct!(SpreadElement, [dot3_token, expr]);

impl_struct!(JSXExprContainer, [span, expr]);
impl_struct!(JSXSpreadChild, [span, expr]);

impl_struct!(JSXAttr, [span, name, value]);

impl_enum!(
    JSXAttrValue,
    [Str, JSXExprContainer, JSXElement, JSXFragment]
);

impl_enum!(JSXAttrName, [Ident, JSXNamespacedName]);

impl_enum!(JSXExpr, [Expr, JSXEmptyExpr]);

impl_struct!(OptCall, [span, ctxt, callee, args, type_args]);

impl_enum!(Callee, [Super, Import, Expr]);

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

    // ==================== ThisExpr Tests ====================

    #[test]
    fn test_this_expr_to_code() {
        let cx = empty_ctx();
        let expr = ThisExpr {
            span: Span::default(),
        };
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ThisExpr"));
        assert!(code_str.contains("span"));
    }

    // ==================== ArrayLit Tests ====================

    #[test]
    fn test_array_lit_empty() {
        let cx = empty_ctx();
        let expr = ArrayLit {
            span: Span::default(),
            elems: vec![],
        };
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ArrayLit"));
        assert!(code_str.contains("elems"));
    }

    #[test]
    fn test_array_lit_with_elements() {
        let cx = empty_ctx();
        let expr = ArrayLit {
            span: Span::default(),
            elems: vec![Some(ExprOrSpread {
                spread: None,
                expr: Box::new(Expr::Lit(Lit::Null(Null {
                    span: Span::default(),
                }))),
            })],
        };
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ArrayLit"));
    }

    // ==================== ObjectLit Tests ====================

    #[test]
    fn test_object_lit_empty() {
        let cx = empty_ctx();
        let expr = ObjectLit {
            span: Span::default(),
            props: vec![],
        };
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ObjectLit"));
        assert!(code_str.contains("props"));
    }

    // ==================== UnaryExpr Tests ====================

    #[test]
    fn test_unary_expr_to_code() {
        let cx = empty_ctx();
        let expr = UnaryExpr {
            span: Span::default(),
            op: UnaryOp::Bang,
            arg: Box::new(Expr::Lit(Lit::Bool(Bool {
                span: Span::default(),
                value: true,
            }))),
        };
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("UnaryExpr"));
        assert!(code_str.contains("op"));
        assert!(code_str.contains("arg"));
    }

    // ==================== BinExpr Tests ====================

    #[test]
    fn test_bin_expr_to_code() {
        let cx = empty_ctx();
        let expr = BinExpr {
            span: Span::default(),
            op: BinaryOp::Add,
            left: Box::new(Expr::Lit(Lit::Num(Number {
                span: Span::default(),
                value: 1.0,
                raw: None,
            }))),
            right: Box::new(Expr::Lit(Lit::Num(Number {
                span: Span::default(),
                value: 2.0,
                raw: None,
            }))),
        };
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("BinExpr"));
        assert!(code_str.contains("left"));
        assert!(code_str.contains("right"));
    }

    // ==================== AssignExpr Tests ====================

    #[test]
    fn test_assign_expr_to_code() {
        let cx = empty_ctx();
        let expr = AssignExpr {
            span: Span::default(),
            op: AssignOp::Assign,
            left: AssignTarget::Simple(SimpleAssignTarget::Ident(BindingIdent {
                id: Ident {
                    span: Span::default(),
                    ctxt: SyntaxContext::empty(),
                    sym: Atom::from("x"),
                    optional: false,
                },
                type_ann: None,
            })),
            right: Box::new(Expr::Lit(Lit::Num(Number {
                span: Span::default(),
                value: 42.0,
                raw: None,
            }))),
        };
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("AssignExpr"));
    }

    // ==================== CondExpr Tests ====================

    #[test]
    fn test_cond_expr_to_code() {
        let cx = empty_ctx();
        let expr = CondExpr {
            span: Span::default(),
            test: Box::new(Expr::Lit(Lit::Bool(Bool {
                span: Span::default(),
                value: true,
            }))),
            cons: Box::new(Expr::Lit(Lit::Num(Number {
                span: Span::default(),
                value: 1.0,
                raw: None,
            }))),
            alt: Box::new(Expr::Lit(Lit::Num(Number {
                span: Span::default(),
                value: 2.0,
                raw: None,
            }))),
        };
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("CondExpr"));
        assert!(code_str.contains("test"));
        assert!(code_str.contains("cons"));
        assert!(code_str.contains("alt"));
    }

    // ==================== CallExpr Tests ====================

    #[test]
    fn test_call_expr_to_code() {
        let cx = empty_ctx();
        let expr = CallExpr {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            callee: Callee::Expr(Box::new(Expr::Ident(Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("foo"),
                optional: false,
            }))),
            args: vec![],
            type_args: None,
        };
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("CallExpr"));
        assert!(code_str.contains("callee"));
        assert!(code_str.contains("args"));
    }

    // ==================== NewExpr Tests ====================

    #[test]
    fn test_new_expr_to_code() {
        let cx = empty_ctx();
        let expr = NewExpr {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            callee: Box::new(Expr::Ident(Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("MyClass"),
                optional: false,
            })),
            args: None,
            type_args: None,
        };
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("NewExpr"));
        assert!(code_str.contains("callee"));
    }

    // ==================== ArrowExpr Tests ====================

    #[test]
    fn test_arrow_expr_to_code() {
        let cx = empty_ctx();
        let expr = ArrowExpr {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            params: vec![],
            body: Box::new(BlockStmtOrExpr::Expr(Box::new(Expr::Lit(Lit::Num(
                Number {
                    span: Span::default(),
                    value: 42.0,
                    raw: None,
                },
            ))))),
            is_async: false,
            is_generator: false,
            type_params: None,
            return_type: None,
        };
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ArrowExpr"));
        assert!(code_str.contains("params"));
        assert!(code_str.contains("body"));
    }

    // ==================== YieldExpr Tests ====================

    #[test]
    fn test_yield_expr_to_code() {
        let cx = empty_ctx();
        let expr = YieldExpr {
            span: Span::default(),
            arg: Some(Box::new(Expr::Lit(Lit::Num(Number {
                span: Span::default(),
                value: 1.0,
                raw: None,
            })))),
            delegate: false,
        };
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("YieldExpr"));
        assert!(code_str.contains("delegate"));
    }

    // ==================== AwaitExpr Tests ====================

    #[test]
    fn test_await_expr_to_code() {
        let cx = empty_ctx();
        let expr = AwaitExpr {
            span: Span::default(),
            arg: Box::new(Expr::Ident(Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("promise"),
                optional: false,
            })),
        };
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("AwaitExpr"));
        assert!(code_str.contains("arg"));
    }

    // ==================== ParenExpr Tests ====================

    #[test]
    fn test_paren_expr_to_code() {
        let cx = empty_ctx();
        let expr = ParenExpr {
            span: Span::default(),
            expr: Box::new(Expr::Lit(Lit::Num(Number {
                span: Span::default(),
                value: 42.0,
                raw: None,
            }))),
        };
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ParenExpr"));
        assert!(code_str.contains("expr"));
    }

    // ==================== SeqExpr Tests ====================

    #[test]
    fn test_seq_expr_to_code() {
        let cx = empty_ctx();
        let expr = SeqExpr {
            span: Span::default(),
            exprs: vec![
                Box::new(Expr::Lit(Lit::Num(Number {
                    span: Span::default(),
                    value: 1.0,
                    raw: None,
                }))),
                Box::new(Expr::Lit(Lit::Num(Number {
                    span: Span::default(),
                    value: 2.0,
                    raw: None,
                }))),
            ],
        };
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("SeqExpr"));
        assert!(code_str.contains("exprs"));
    }

    // ==================== Super Tests ====================

    #[test]
    fn test_super_to_code() {
        let cx = empty_ctx();
        let s = Super {
            span: Span::default(),
        };
        let code = s.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Super"));
        assert!(code_str.contains("span"));
    }

    // ==================== Callee Enum Tests ====================

    #[test]
    fn test_callee_super_to_code() {
        let cx = empty_ctx();
        let callee = Callee::Super(Super {
            span: Span::default(),
        });
        let code = callee.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Callee"));
        assert!(code_str.contains("Super"));
    }

    #[test]
    fn test_callee_expr_to_code() {
        let cx = empty_ctx();
        let callee = Callee::Expr(Box::new(Expr::Ident(Ident {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            sym: Atom::from("fn"),
            optional: false,
        })));
        let code = callee.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Callee"));
        assert!(code_str.contains("Expr"));
    }

    // ==================== ExprOrSpread Tests ====================

    #[test]
    fn test_expr_or_spread_to_code() {
        let cx = empty_ctx();
        let expr = ExprOrSpread {
            spread: None,
            expr: Box::new(Expr::Lit(Lit::Num(Number {
                span: Span::default(),
                value: 42.0,
                raw: None,
            }))),
        };
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ExprOrSpread"));
        assert!(code_str.contains("spread"));
        assert!(code_str.contains("expr"));
    }

    // ==================== SpreadElement Tests ====================

    #[test]
    fn test_spread_element_to_code() {
        let cx = empty_ctx();
        let spread = SpreadElement {
            dot3_token: Span::default(),
            expr: Box::new(Expr::Ident(Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("arr"),
                optional: false,
            })),
        };
        let code = spread.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("SpreadElement"));
        assert!(code_str.contains("dot3_token"));
    }

    // ==================== Expr Enum Tests ====================

    #[test]
    fn test_expr_this_to_code() {
        let cx = empty_ctx();
        let expr = Expr::This(ThisExpr {
            span: Span::default(),
        });
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Expr"));
        assert!(code_str.contains("This"));
    }

    #[test]
    fn test_expr_lit_to_code() {
        let cx = empty_ctx();
        let expr = Expr::Lit(Lit::Null(Null {
            span: Span::default(),
        }));
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Expr"));
        assert!(code_str.contains("Lit"));
    }

    #[test]
    fn test_expr_ident_to_code() {
        let cx = empty_ctx();
        let expr = Expr::Ident(Ident {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            sym: Atom::from("myVar"),
            optional: false,
        });
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Expr"));
        assert!(code_str.contains("Ident"));
    }
}
