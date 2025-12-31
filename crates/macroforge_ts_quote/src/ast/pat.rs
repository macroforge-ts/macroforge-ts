use swc_core::ecma::ast::*;

impl_struct!(BindingIdent, [id, type_ann]);
impl_struct!(ArrayPat, [span, elems, optional, type_ann]);
impl_struct!(ObjectPat, [span, props, optional, type_ann]);
impl_struct!(RestPat, [span, dot3_token, arg, type_ann]);
impl_struct!(AssignPat, [span, left, right]);
impl_struct!(Param, [span, decorators, pat]);

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

    // ==================== BindingIdent Tests ====================

    #[test]
    fn test_binding_ident_to_code() {
        let cx = empty_ctx();
        let binding = BindingIdent {
            id: Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("myVar"),
                optional: false,
            },
            type_ann: None,
        };
        let code = binding.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("BindingIdent"));
        assert!(code_str.contains("id"));
        assert!(code_str.contains("type_ann"));
    }

    #[test]
    fn test_binding_ident_with_type_ann() {
        let cx = empty_ctx();
        let binding = BindingIdent {
            id: Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("x"),
                optional: false,
            },
            type_ann: Some(Box::new(TsTypeAnn {
                span: Span::default(),
                type_ann: Box::new(TsType::TsKeywordType(TsKeywordType {
                    span: Span::default(),
                    kind: TsKeywordTypeKind::TsNumberKeyword,
                })),
            })),
        };
        let code = binding.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("BindingIdent"));
    }

    // ==================== ArrayPat Tests ====================

    #[test]
    fn test_array_pat_empty() {
        let cx = empty_ctx();
        let pat = ArrayPat {
            span: Span::default(),
            elems: vec![],
            optional: false,
            type_ann: None,
        };
        let code = pat.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ArrayPat"));
        assert!(code_str.contains("elems"));
        assert!(code_str.contains("optional"));
    }

    #[test]
    fn test_array_pat_with_elements() {
        let cx = empty_ctx();
        let pat = ArrayPat {
            span: Span::default(),
            elems: vec![
                Some(Pat::Ident(BindingIdent {
                    id: Ident {
                        span: Span::default(),
                        ctxt: SyntaxContext::empty(),
                        sym: Atom::from("a"),
                        optional: false,
                    },
                    type_ann: None,
                })),
                None, // hole in array destructuring
                Some(Pat::Ident(BindingIdent {
                    id: Ident {
                        span: Span::default(),
                        ctxt: SyntaxContext::empty(),
                        sym: Atom::from("b"),
                        optional: false,
                    },
                    type_ann: None,
                })),
            ],
            optional: false,
            type_ann: None,
        };
        let code = pat.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ArrayPat"));
    }

    // ==================== ObjectPat Tests ====================

    #[test]
    fn test_object_pat_empty() {
        let cx = empty_ctx();
        let pat = ObjectPat {
            span: Span::default(),
            props: vec![],
            optional: false,
            type_ann: None,
        };
        let code = pat.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ObjectPat"));
        assert!(code_str.contains("props"));
    }

    #[test]
    fn test_object_pat_with_properties() {
        let cx = empty_ctx();
        let pat = ObjectPat {
            span: Span::default(),
            props: vec![ObjectPatProp::Assign(AssignPatProp {
                span: Span::default(),
                key: BindingIdent {
                    id: Ident {
                        span: Span::default(),
                        ctxt: SyntaxContext::empty(),
                        sym: Atom::from("x"),
                        optional: false,
                    },
                    type_ann: None,
                },
                value: None,
            })],
            optional: false,
            type_ann: None,
        };
        let code = pat.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ObjectPat"));
    }

    // ==================== RestPat Tests ====================

    #[test]
    fn test_rest_pat_to_code() {
        let cx = empty_ctx();
        let pat = RestPat {
            span: Span::default(),
            dot3_token: Span::default(),
            arg: Box::new(Pat::Ident(BindingIdent {
                id: Ident {
                    span: Span::default(),
                    ctxt: SyntaxContext::empty(),
                    sym: Atom::from("rest"),
                    optional: false,
                },
                type_ann: None,
            })),
            type_ann: None,
        };
        let code = pat.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("RestPat"));
        assert!(code_str.contains("dot3_token"));
        assert!(code_str.contains("arg"));
    }

    // ==================== AssignPat Tests ====================

    #[test]
    fn test_assign_pat_to_code() {
        let cx = empty_ctx();
        let pat = AssignPat {
            span: Span::default(),
            left: Box::new(Pat::Ident(BindingIdent {
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
        let code = pat.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("AssignPat"));
        assert!(code_str.contains("left"));
        assert!(code_str.contains("right"));
    }

    // ==================== Param Tests ====================

    #[test]
    fn test_param_to_code() {
        let cx = empty_ctx();
        let param = Param {
            span: Span::default(),
            decorators: vec![],
            pat: Pat::Ident(BindingIdent {
                id: Ident {
                    span: Span::default(),
                    ctxt: SyntaxContext::empty(),
                    sym: Atom::from("arg"),
                    optional: false,
                },
                type_ann: None,
            }),
        };
        let code = param.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Param"));
        assert!(code_str.contains("decorators"));
        assert!(code_str.contains("pat"));
    }

    #[test]
    fn test_param_with_decorator() {
        let cx = empty_ctx();
        let param = Param {
            span: Span::default(),
            decorators: vec![Decorator {
                span: Span::default(),
                expr: Box::new(Expr::Ident(Ident {
                    span: Span::default(),
                    ctxt: SyntaxContext::empty(),
                    sym: Atom::from("Inject"),
                    optional: false,
                })),
            }],
            pat: Pat::Ident(BindingIdent {
                id: Ident {
                    span: Span::default(),
                    ctxt: SyntaxContext::empty(),
                    sym: Atom::from("service"),
                    optional: false,
                },
                type_ann: None,
            }),
        };
        let code = param.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Param"));
    }
}
