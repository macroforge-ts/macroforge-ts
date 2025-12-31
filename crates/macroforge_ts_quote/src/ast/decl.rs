use swc_core::ecma::ast::*;

impl_enum!(
    Decl,
    [
        Class,
        Fn,
        Var,
        TsInterface,
        TsTypeAlias,
        TsEnum,
        TsModule,
        Using
    ]
);

impl_struct!(ClassDecl, [ident, declare, class]);
impl_struct!(FnDecl, [ident, declare, function]);
impl_struct!(VarDecl, [span, ctxt, kind, declare, decls]);
impl_struct!(VarDeclarator, [span, name, init, definite]);
impl_struct!(UsingDecl, [span, is_await, decls]);

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

    // ==================== Decl Enum Tests ====================

    #[test]
    fn test_decl_var_to_code() {
        let cx = empty_ctx();
        let decl = Decl::Var(Box::new(VarDecl {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            kind: VarDeclKind::Const,
            declare: false,
            decls: vec![],
        }));
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Decl"));
        assert!(code_str.contains("Var"));
    }

    #[test]
    fn test_decl_fn_to_code() {
        let cx = empty_ctx();
        let decl = Decl::Fn(FnDecl {
            ident: Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("myFunc"),
                optional: false,
            },
            declare: false,
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
        });
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Decl"));
        assert!(code_str.contains("Fn"));
    }

    // ==================== VarDecl Tests ====================

    #[test]
    fn test_var_decl_const() {
        let cx = empty_ctx();
        let decl = VarDecl {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            kind: VarDeclKind::Const,
            declare: false,
            decls: vec![],
        };
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("VarDecl"));
        assert!(code_str.contains("kind"));
        assert!(code_str.contains("decls"));
    }

    #[test]
    fn test_var_decl_let() {
        let cx = empty_ctx();
        let decl = VarDecl {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            kind: VarDeclKind::Let,
            declare: false,
            decls: vec![],
        };
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("VarDecl"));
    }

    #[test]
    fn test_var_decl_with_declarator() {
        let cx = empty_ctx();
        let decl = VarDecl {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            kind: VarDeclKind::Const,
            declare: false,
            decls: vec![VarDeclarator {
                span: Span::default(),
                name: Pat::Ident(BindingIdent {
                    id: Ident {
                        span: Span::default(),
                        ctxt: SyntaxContext::empty(),
                        sym: Atom::from("x"),
                        optional: false,
                    },
                    type_ann: None,
                }),
                init: Some(Box::new(Expr::Lit(Lit::Num(Number {
                    span: Span::default(),
                    value: 42.0,
                    raw: None,
                })))),
                definite: false,
            }],
        };
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("VarDecl"));
    }

    // ==================== VarDeclarator Tests ====================

    #[test]
    fn test_var_declarator_basic() {
        let cx = empty_ctx();
        let decl = VarDeclarator {
            span: Span::default(),
            name: Pat::Ident(BindingIdent {
                id: Ident {
                    span: Span::default(),
                    ctxt: SyntaxContext::empty(),
                    sym: Atom::from("myVar"),
                    optional: false,
                },
                type_ann: None,
            }),
            init: None,
            definite: false,
        };
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("VarDeclarator"));
        assert!(code_str.contains("name"));
        assert!(code_str.contains("init"));
    }

    #[test]
    fn test_var_declarator_with_init() {
        let cx = empty_ctx();
        let decl = VarDeclarator {
            span: Span::default(),
            name: Pat::Ident(BindingIdent {
                id: Ident {
                    span: Span::default(),
                    ctxt: SyntaxContext::empty(),
                    sym: Atom::from("x"),
                    optional: false,
                },
                type_ann: None,
            }),
            init: Some(Box::new(Expr::Lit(Lit::Str(Str {
                span: Span::default(),
                value: Atom::from("hello").into(),
                raw: None,
            })))),
            definite: false,
        };
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("VarDeclarator"));
    }

    // ==================== FnDecl Tests ====================

    #[test]
    fn test_fn_decl_to_code() {
        let cx = empty_ctx();
        let decl = FnDecl {
            ident: Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("myFunction"),
                optional: false,
            },
            declare: false,
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
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("FnDecl"));
        assert!(code_str.contains("ident"));
        assert!(code_str.contains("function"));
    }

    #[test]
    fn test_fn_decl_async() {
        let cx = empty_ctx();
        let decl = FnDecl {
            ident: Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("asyncFn"),
                optional: false,
            },
            declare: false,
            function: Box::new(Function {
                ctxt: SyntaxContext::empty(),
                params: vec![],
                decorators: vec![],
                span: Span::default(),
                body: None,
                is_generator: false,
                is_async: true,
                type_params: None,
                return_type: None,
            }),
        };
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("FnDecl"));
    }

    // ==================== ClassDecl Tests ====================

    #[test]
    fn test_class_decl_to_code() {
        let cx = empty_ctx();
        let decl = ClassDecl {
            ident: Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("MyClass"),
                optional: false,
            },
            declare: false,
            class: Box::new(Class {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                decorators: vec![],
                body: vec![],
                super_class: None,
                is_abstract: false,
                type_params: None,
                super_type_params: None,
                implements: vec![],
            }),
        };
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ClassDecl"));
        assert!(code_str.contains("ident"));
        assert!(code_str.contains("class"));
    }

    // ==================== UsingDecl Tests ====================

    #[test]
    fn test_using_decl_to_code() {
        let cx = empty_ctx();
        let decl = UsingDecl {
            span: Span::default(),
            is_await: false,
            decls: vec![],
        };
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("UsingDecl"));
        assert!(code_str.contains("is_await"));
        assert!(code_str.contains("decls"));
    }

    #[test]
    fn test_using_decl_await() {
        let cx = empty_ctx();
        let decl = UsingDecl {
            span: Span::default(),
            is_await: true,
            decls: vec![],
        };
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("UsingDecl"));
    }
}
