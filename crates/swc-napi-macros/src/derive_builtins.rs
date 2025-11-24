use swc_core::{
    common::{Span, DUMMY_SP, SyntaxContext},
    ecma::ast::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltInDerive {
    Debug,
    Json,
}

impl BuiltInDerive {
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "Debug" => Some(Self::Debug),
            "JSON" => Some(Self::Json),
            _ => None,
        }
    }
}

/// Remove @Derive decorators from the class and return the derived macros that should run.
pub fn extract_builtin_derives(class: &mut Class) -> Vec<BuiltInDerive> {
    let mut derives = Vec::new();
    let mut remaining = Vec::with_capacity(class.decorators.len());

    for decorator in class.decorators.drain(..) {
        if let Expr::Call(call) = decorator.expr.as_ref()
            && let Callee::Expr(expr) = &call.callee
            && let Expr::Ident(ident) = expr.as_ref()
            && &*ident.sym == "Derive"
        {
            for arg in &call.args {
                if let Expr::Lit(Lit::Str(str_lit)) = arg.expr.as_ref() {
                    // Wtf8 might not be valid UTF-8, so use to_string_lossy
                    let str_value = str_lit.value.to_string_lossy();
                    if let Some(kind) = BuiltInDerive::from_str(&str_value) {
                        derives.push(kind);
                    }
                }
            }
            continue;
        }

        remaining.push(decorator);
    }

    class.decorators = remaining;
    derives
}

/// Generate the AST class members for the applicable built-in derives.
pub fn expand_builtin_derives(kinds: &[BuiltInDerive]) -> Vec<ClassMember> {
    let mut members = Vec::new();

    for kind in kinds {
        match kind {
            BuiltInDerive::Debug => members.push(generate_to_string_method()),
            BuiltInDerive::Json => members.push(generate_to_json_method()),
        }
    }

    members
}

fn span() -> Span {
    DUMMY_SP
}

fn ctxt() -> SyntaxContext {
    SyntaxContext::empty()
}

fn ident(name: &str) -> Ident {
    Ident::new(name.into(), span(), ctxt())
}

fn ident_name(name: &str) -> IdentName {
    ident(name).into()
}

fn this_expr() -> Expr {
    Expr::This(ThisExpr { span: span() })
}

fn generate_to_string_method() -> ClassMember {
    // this.constructor
    let ctor_expr = Expr::Member(MemberExpr {
        span: span(),
        obj: Box::new(this_expr()),
        prop: MemberProp::Ident(ident_name("constructor")),
    });

    // JSON.stringify(this)
    let stringify_expr = Expr::Call(CallExpr {
        span: span(),
        ctxt: ctxt(),
        callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
            span: span(),
            obj: Box::new(Expr::Ident(ident("JSON"))),
            prop: MemberProp::Ident(ident_name("stringify")),
        }))),
        args: vec![ExprOrSpread {
            spread: None,
            expr: Box::new(this_expr()),
        }],
        type_args: None,
    });

    let tpl = Expr::Tpl(Tpl {
        span: span(),
        exprs: vec![Box::new(ctor_expr), Box::new(stringify_expr)],
        quasis: vec![
            TplElement {
                span: span(),
                tail: false,
                cooked: Some("".into()),
                raw: "".into(),
            },
            TplElement {
                span: span(),
                tail: false,
                cooked: Some(" ".into()),
                raw: " ".into(),
            },
            TplElement {
                span: span(),
                tail: true,
                cooked: Some("".into()),
                raw: "".into(),
            },
        ],
    });

    let return_stmt = Stmt::Return(ReturnStmt {
        span: span(),
        arg: Some(Box::new(tpl)),
    });

    ClassMember::Method(ClassMethod {
        span: span(),
        key: PropName::Ident(ident_name("toString")),
        function: Box::new(Function {
            params: vec![],
            decorators: vec![],
            span: span(),
            ctxt: ctxt(),
            body: Some(BlockStmt {
                span: span(),
                ctxt: ctxt(),
                stmts: vec![return_stmt],
            }),
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
    })
}

fn generate_to_json_method() -> ClassMember {
    ClassMember::Method(ClassMethod {
        span: span(),
        key: PropName::Ident(ident_name("toJSON")),
        function: Box::new(Function {
            params: vec![],
            decorators: vec![],
            span: span(),
            ctxt: ctxt(),
            body: Some(BlockStmt {
                span: span(),
                ctxt: ctxt(),
                stmts: vec![Stmt::Return(ReturnStmt {
                    span: span(),
                    arg: Some(Box::new(Expr::Object(ObjectLit {
                        span: span(),
                        props: vec![PropOrSpread::Spread(SpreadElement {
                            dot3_token: span(),
                            expr: Box::new(this_expr()),
                        })],
                    }))),
                })],
            }),
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
    })
}
