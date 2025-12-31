use swc_core::ecma::ast::*;

impl_enum!(Prop, [Shorthand, KeyValue, Assign, Getter, Setter, Method]);

impl_struct!(
    PrivateProp,
    [
        span,
        ctxt,
        definite,
        key,
        value,
        type_ann,
        is_static,
        decorators,
        accessibility,
        is_optional,
        is_override,
        readonly
    ]
);

impl_struct!(KeyValueProp, [key, value]);

impl_struct!(AssignProp, [span, key, value]);

impl_struct!(GetterProp, [span, key, type_ann, body]);
impl_struct!(SetterProp, [span, key, param, this_param, body]);

impl_struct!(MethodProp, [key, function]);

impl_struct!(KeyValuePatProp, [key, value]);

impl_struct!(AssignPatProp, [span, key, value]);

impl_struct!(ComputedPropName, [span, expr]);

impl_enum!(Key, [Private, Public]);

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

    // ==================== Prop Enum Tests ====================

    #[test]
    fn test_prop_shorthand_to_code() {
        let cx = empty_ctx();
        let prop = Prop::Shorthand(Ident {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            sym: Atom::from("x"),
            optional: false,
        });
        let code = prop.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Prop"));
        assert!(code_str.contains("Shorthand"));
    }

    #[test]
    fn test_prop_key_value_to_code() {
        let cx = empty_ctx();
        let prop = Prop::KeyValue(KeyValueProp {
            key: PropName::Ident(IdentName {
                span: Span::default(),
                sym: Atom::from("name"),
            }),
            value: Box::new(Expr::Lit(Lit::Str(Str {
                span: Span::default(),
                value: Atom::from("value").into(),
                raw: None,
            }))),
        });
        let code = prop.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Prop"));
        assert!(code_str.contains("KeyValue"));
    }

    // ==================== KeyValueProp Tests ====================

    #[test]
    fn test_key_value_prop_to_code() {
        let cx = empty_ctx();
        let prop = KeyValueProp {
            key: PropName::Ident(IdentName {
                span: Span::default(),
                sym: Atom::from("foo"),
            }),
            value: Box::new(Expr::Lit(Lit::Num(Number {
                span: Span::default(),
                value: 42.0,
                raw: None,
            }))),
        };
        let code = prop.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("KeyValueProp"));
        assert!(code_str.contains("key"));
        assert!(code_str.contains("value"));
    }

    // ==================== AssignProp Tests ====================

    #[test]
    fn test_assign_prop_to_code() {
        let cx = empty_ctx();
        let prop = AssignProp {
            span: Span::default(),
            key: Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("x"),
                optional: false,
            },
            value: Box::new(Expr::Lit(Lit::Num(Number {
                span: Span::default(),
                value: 10.0,
                raw: None,
            }))),
        };
        let code = prop.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("AssignProp"));
        assert!(code_str.contains("key"));
        assert!(code_str.contains("value"));
    }

    // ==================== GetterProp Tests ====================

    #[test]
    fn test_getter_prop_to_code() {
        let cx = empty_ctx();
        let prop = GetterProp {
            span: Span::default(),
            key: PropName::Ident(IdentName {
                span: Span::default(),
                sym: Atom::from("value"),
            }),
            type_ann: None,
            body: None,
        };
        let code = prop.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("GetterProp"));
        assert!(code_str.contains("key"));
        assert!(code_str.contains("body"));
    }

    // ==================== SetterProp Tests ====================

    #[test]
    fn test_setter_prop_to_code() {
        let cx = empty_ctx();
        let prop = SetterProp {
            span: Span::default(),
            key: PropName::Ident(IdentName {
                span: Span::default(),
                sym: Atom::from("value"),
            }),
            param: Box::new(Pat::Ident(BindingIdent {
                id: Ident {
                    span: Span::default(),
                    ctxt: SyntaxContext::empty(),
                    sym: Atom::from("val"),
                    optional: false,
                },
                type_ann: None,
            })),
            this_param: None,
            body: None,
        };
        let code = prop.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("SetterProp"));
        assert!(code_str.contains("key"));
        assert!(code_str.contains("param"));
    }

    // ==================== MethodProp Tests ====================

    #[test]
    fn test_method_prop_to_code() {
        let cx = empty_ctx();
        let prop = MethodProp {
            key: PropName::Ident(IdentName {
                span: Span::default(),
                sym: Atom::from("myMethod"),
            }),
            function: Box::new(Function {
                ctxt: SyntaxContext::empty(),
                params: vec![],
                decorators: vec![],
                span: Span::default(),
                body: None,
                is_generator: false,
                is_async: false,
                type_params: None,
                return_type: None,
            }),
        };
        let code = prop.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("MethodProp"));
        assert!(code_str.contains("key"));
        assert!(code_str.contains("function"));
    }

    // ==================== PrivateProp Tests ====================

    #[test]
    fn test_private_prop_to_code() {
        let cx = empty_ctx();
        let prop = PrivateProp {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            definite: false,
            key: PrivateName {
                span: Span::default(),
                name: Atom::from("privateField"),
            },
            value: None,
            type_ann: None,
            is_static: false,
            decorators: vec![],
            accessibility: None,
            is_optional: false,
            is_override: false,
            readonly: false,
        };
        let code = prop.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("PrivateProp"));
        assert!(code_str.contains("key"));
    }

    // ==================== ComputedPropName Tests ====================

    #[test]
    fn test_computed_prop_name_to_code() {
        let cx = empty_ctx();
        let prop = ComputedPropName {
            span: Span::default(),
            expr: Box::new(Expr::Lit(Lit::Str(Str {
                span: Span::default(),
                value: Atom::from("dynamicKey").into(),
                raw: None,
            }))),
        };
        let code = prop.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ComputedPropName"));
        assert!(code_str.contains("expr"));
    }

    // ==================== Key Enum Tests ====================

    #[test]
    fn test_key_public_to_code() {
        let cx = empty_ctx();
        let key = Key::Public(PropName::Ident(IdentName {
            span: Span::default(),
            sym: Atom::from("publicKey"),
        }));
        let code = key.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Key"));
        assert!(code_str.contains("Public"));
    }

    #[test]
    fn test_key_private_to_code() {
        let cx = empty_ctx();
        let key = Key::Private(PrivateName {
            span: Span::default(),
            name: Atom::from("privateKey"),
        });
        let code = key.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Key"));
        assert!(code_str.contains("Private"));
    }

    // ==================== KeyValuePatProp Tests ====================

    #[test]
    fn test_key_value_pat_prop_to_code() {
        let cx = empty_ctx();
        let prop = KeyValuePatProp {
            key: PropName::Ident(IdentName {
                span: Span::default(),
                sym: Atom::from("x"),
            }),
            value: Box::new(Pat::Ident(BindingIdent {
                id: Ident {
                    span: Span::default(),
                    ctxt: SyntaxContext::empty(),
                    sym: Atom::from("localX"),
                    optional: false,
                },
                type_ann: None,
            })),
        };
        let code = prop.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("KeyValuePatProp"));
        assert!(code_str.contains("key"));
        assert!(code_str.contains("value"));
    }

    // ==================== AssignPatProp Tests ====================

    #[test]
    fn test_assign_pat_prop_to_code() {
        let cx = empty_ctx();
        let prop = AssignPatProp {
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
            value: Some(Box::new(Expr::Lit(Lit::Num(Number {
                span: Span::default(),
                value: 10.0,
                raw: None,
            })))),
        };
        let code = prop.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("AssignPatProp"));
        assert!(code_str.contains("key"));
        assert!(code_str.contains("value"));
    }
}
