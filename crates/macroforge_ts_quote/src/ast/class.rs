use swc_core::ecma::ast::*;

impl_struct!(
    Class,
    [
        span,
        ctxt,
        decorators,
        body,
        super_class,
        is_abstract,
        type_params,
        super_type_params,
        implements
    ]
);

impl_struct!(
    Constructor,
    [span, ctxt, key, params, body, accessibility, is_optional]
);

impl_struct!(
    ClassMethod,
    [
        span,
        key,
        function,
        kind,
        is_static,
        accessibility,
        is_abstract,
        is_optional,
        is_override
    ]
);

impl_struct!(
    PrivateMethod,
    [
        span,
        key,
        function,
        kind,
        is_static,
        accessibility,
        is_abstract,
        is_optional,
        is_override
    ]
);

impl_struct!(
    ClassProp,
    [
        span,
        key,
        value,
        type_ann,
        is_static,
        decorators,
        accessibility,
        is_abstract,
        is_optional,
        is_override,
        readonly,
        declare,
        definite
    ]
);

impl_struct!(StaticBlock, [span, body]);

impl_struct!(
    AutoAccessor,
    [
        span,
        is_abstract,
        is_override,
        definite,
        key,
        value,
        type_ann,
        is_static,
        decorators,
        accessibility
    ]
);

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

    // ==================== Class Tests ====================

    #[test]
    fn test_class_empty() {
        let cx = empty_ctx();
        let class = Class {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            decorators: vec![],
            body: vec![],
            super_class: None,
            is_abstract: false,
            type_params: None,
            super_type_params: None,
            implements: vec![],
        };
        let code = class.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Class"));
        assert!(code_str.contains("body"));
        assert!(code_str.contains("is_abstract"));
    }

    #[test]
    fn test_class_abstract() {
        let cx = empty_ctx();
        let class = Class {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            decorators: vec![],
            body: vec![],
            super_class: None,
            is_abstract: true,
            type_params: None,
            super_type_params: None,
            implements: vec![],
        };
        let code = class.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Class"));
        assert!(code_str.contains("is_abstract"));
    }

    #[test]
    fn test_class_with_super() {
        let cx = empty_ctx();
        let class = Class {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            decorators: vec![],
            body: vec![],
            super_class: Some(Box::new(Expr::Ident(Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("BaseClass"),
                optional: false,
            }))),
            is_abstract: false,
            type_params: None,
            super_type_params: None,
            implements: vec![],
        };
        let code = class.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Class"));
        assert!(code_str.contains("super_class"));
    }

    // ==================== Constructor Tests ====================

    #[test]
    fn test_constructor_to_code() {
        let cx = empty_ctx();
        let ctor = Constructor {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            key: PropName::Ident(IdentName {
                span: Span::default(),
                sym: Atom::from("constructor"),
            }),
            params: vec![],
            body: None,
            accessibility: None,
            is_optional: false,
        };
        let code = ctor.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Constructor"));
        assert!(code_str.contains("key"));
        assert!(code_str.contains("params"));
    }

    #[test]
    fn test_constructor_with_accessibility() {
        let cx = empty_ctx();
        let ctor = Constructor {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            key: PropName::Ident(IdentName {
                span: Span::default(),
                sym: Atom::from("constructor"),
            }),
            params: vec![],
            body: None,
            accessibility: Some(Accessibility::Public),
            is_optional: false,
        };
        let code = ctor.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Constructor"));
        assert!(code_str.contains("accessibility"));
    }

    // ==================== ClassMethod Tests ====================

    #[test]
    fn test_class_method_to_code() {
        let cx = empty_ctx();
        let method = ClassMethod {
            span: Span::default(),
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
            kind: MethodKind::Method,
            is_static: false,
            accessibility: None,
            is_abstract: false,
            is_optional: false,
            is_override: false,
        };
        let code = method.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ClassMethod"));
        assert!(code_str.contains("key"));
        assert!(code_str.contains("function"));
    }

    #[test]
    fn test_class_method_static() {
        let cx = empty_ctx();
        let method = ClassMethod {
            span: Span::default(),
            key: PropName::Ident(IdentName {
                span: Span::default(),
                sym: Atom::from("staticMethod"),
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
            kind: MethodKind::Method,
            is_static: true,
            accessibility: None,
            is_abstract: false,
            is_optional: false,
            is_override: false,
        };
        let code = method.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ClassMethod"));
        assert!(code_str.contains("is_static"));
    }

    // ==================== ClassProp Tests ====================

    #[test]
    fn test_class_prop_to_code() {
        let cx = empty_ctx();
        let prop = ClassProp {
            span: Span::default(),
            key: PropName::Ident(IdentName {
                span: Span::default(),
                sym: Atom::from("myProp"),
            }),
            value: None,
            type_ann: None,
            is_static: false,
            decorators: vec![],
            accessibility: None,
            is_abstract: false,
            is_optional: false,
            is_override: false,
            readonly: false,
            declare: false,
            definite: false,
        };
        let code = prop.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ClassProp"));
        assert!(code_str.contains("key"));
        assert!(code_str.contains("value"));
    }

    #[test]
    fn test_class_prop_with_value() {
        let cx = empty_ctx();
        let prop = ClassProp {
            span: Span::default(),
            key: PropName::Ident(IdentName {
                span: Span::default(),
                sym: Atom::from("count"),
            }),
            value: Some(Box::new(Expr::Lit(Lit::Num(Number {
                span: Span::default(),
                value: 0.0,
                raw: None,
            })))),
            type_ann: None,
            is_static: false,
            decorators: vec![],
            accessibility: None,
            is_abstract: false,
            is_optional: false,
            is_override: false,
            readonly: false,
            declare: false,
            definite: false,
        };
        let code = prop.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ClassProp"));
    }

    // ==================== StaticBlock Tests ====================

    #[test]
    fn test_static_block_to_code() {
        let cx = empty_ctx();
        let block = StaticBlock {
            span: Span::default(),
            body: BlockStmt {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            },
        };
        let code = block.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("StaticBlock"));
        assert!(code_str.contains("body"));
    }

    // ==================== PrivateMethod Tests ====================

    #[test]
    fn test_private_method_to_code() {
        let cx = empty_ctx();
        let method = PrivateMethod {
            span: Span::default(),
            key: PrivateName {
                span: Span::default(),
                name: Atom::from("privateMethod"),
            },
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
            kind: MethodKind::Method,
            is_static: false,
            accessibility: None,
            is_abstract: false,
            is_optional: false,
            is_override: false,
        };
        let code = method.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("PrivateMethod"));
        assert!(code_str.contains("key"));
    }

    // ==================== AutoAccessor Tests ====================

    #[test]
    fn test_auto_accessor_to_code() {
        let cx = empty_ctx();
        let accessor = AutoAccessor {
            span: Span::default(),
            is_abstract: false,
            is_override: false,
            definite: false,
            key: Key::Public(PropName::Ident(IdentName {
                span: Span::default(),
                sym: Atom::from("myAccessor"),
            })),
            value: None,
            type_ann: None,
            is_static: false,
            decorators: vec![],
            accessibility: None,
        };
        let code = accessor.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("AutoAccessor"));
        assert!(code_str.contains("key"));
    }
}
