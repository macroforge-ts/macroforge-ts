use std::{fs, path::Path};
use swc_core::{
    common::{DUMMY_SP, Span, SyntaxContext},
    ecma::{
        ast::*,
        visit::{VisitMut, VisitMutWith},
    },
};

pub struct MacroTransformer {
    filepath: String,
}

impl MacroTransformer {
    pub fn new(filepath: String) -> Self {
        Self { filepath }
    }

    /// Transform IncludeStr macro calls
    fn transform_include_str(&mut self, call: &mut CallExpr) -> Option<Expr> {
        // Check if this is an IncludeStr call
        if let Callee::Expr(expr) = &call.callee
            && let Expr::Ident(ident) = &**expr
            && ident.sym.as_ref() == "IncludeStr"
        {
            // Get the first argument (file path)
            if let Some(arg) = call.args.first()
                && let Expr::Lit(Lit::Str(str_lit)) = &*arg.expr
            {
                let relative_path = str_lit.value.to_string_lossy().into_owned();

                // Resolve path relative to the current file
                let current_dir = Path::new(&self.filepath).parent().unwrap_or(Path::new("."));
                let file_path = current_dir.join(relative_path);

                // Read the file contents
                match fs::read_to_string(&file_path) {
                    Ok(contents) => {
                        // Return a string literal with the file contents
                        return Some(Expr::Lit(Lit::Str(Str {
                            span: call.span,
                            value: contents.into(),
                            raw: None,
                        })));
                    }
                    Err(err) => {
                        eprintln!(
                            "Failed to read file '{}' for IncludeStr macro: {}",
                            file_path.display(),
                            err
                        );
                    }
                }
            }
        }
        None
    }

    /// Transform @Derive decorator
    fn transform_derive_decorator(&mut self, class: &mut Class) {
        let mut generated_methods = Vec::new();
        let mut remaining_decorators = Vec::with_capacity(class.decorators.len());

        for decorator in class.decorators.drain(..) {
            if let Expr::Call(call) = decorator.expr.as_ref()
                && let Callee::Expr(expr) = &call.callee
                && let Expr::Ident(ident) = expr.as_ref()
                && ident.sym.as_ref() == "Derive"
            {
                for arg in &call.args {
                    if let Expr::Lit(Lit::Str(str_lit)) = arg.expr.as_ref() {
                        let feature = str_lit.value.to_string_lossy().into_owned();
                        match feature.as_str() {
                            "Debug" => generated_methods.push(Self::generate_to_string_method()),
                            "JSON" => generated_methods.push(Self::generate_to_json_method()),
                            _ => {}
                        }
                    }
                }
                continue;
            }

            remaining_decorators.push(decorator);
        }

        class.decorators = remaining_decorators;
        class.body.extend(generated_methods);
    }

    fn span() -> Span {
        DUMMY_SP
    }

    fn ctxt() -> SyntaxContext {
        SyntaxContext::empty()
    }

    fn ident(name: &str) -> Ident {
        Ident::new(name.into(), Self::span(), Self::ctxt())
    }

    fn ident_name(name: &str) -> IdentName {
        Self::ident(name).into()
    }

    fn this_expr() -> Expr {
        Expr::This(ThisExpr { span: Self::span() })
    }

    fn generate_to_string_method() -> ClassMember {
        // this.constructor
        let ctor_expr = Expr::Member(MemberExpr {
            span: Self::span(),
            obj: Box::new(Self::this_expr()),
            prop: MemberProp::Ident(Self::ident_name("constructor")),
        });

        // JSON.stringify(this)
        let stringify_expr = Expr::Call(CallExpr {
            span: Self::span(),
            ctxt: Self::ctxt(),
            callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                span: Self::span(),
                obj: Box::new(Expr::Ident(Self::ident("JSON"))),
                prop: MemberProp::Ident(Self::ident_name("stringify")),
            }))),
            args: vec![ExprOrSpread {
                spread: None,
                expr: Box::new(Self::this_expr()),
            }],
            type_args: None,
        });

        let tpl = Expr::Tpl(Tpl {
            span: Self::span(),
            exprs: vec![Box::new(ctor_expr), Box::new(stringify_expr)],
            quasis: vec![
                TplElement {
                    span: Self::span(),
                    tail: false,
                    cooked: Some("".into()),
                    raw: "".into(),
                },
                TplElement {
                    span: Self::span(),
                    tail: false,
                    cooked: Some(" ".into()),
                    raw: " ".into(),
                },
                TplElement {
                    span: Self::span(),
                    tail: true,
                    cooked: Some("".into()),
                    raw: "".into(),
                },
            ],
        });

        let return_stmt = Stmt::Return(ReturnStmt {
            span: Self::span(),
            arg: Some(Box::new(tpl)),
        });

        ClassMember::Method(ClassMethod {
            span: Self::span(),
            key: PropName::Ident(Self::ident_name("toString")),
            function: Box::new(Function {
                params: vec![],
                decorators: vec![],
                span: Self::span(),
                ctxt: Self::ctxt(),
                body: Some(BlockStmt {
                    span: Self::span(),
                    ctxt: Self::ctxt(),
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
            span: Self::span(),
            key: PropName::Ident(Self::ident_name("toJSON")),
            function: Box::new(Function {
                params: vec![],
                decorators: vec![],
                span: Self::span(),
                ctxt: Self::ctxt(),
                body: Some(BlockStmt {
                    span: Self::span(),
                    ctxt: Self::ctxt(),
                    stmts: vec![Stmt::Return(ReturnStmt {
                        span: Self::span(),
                        arg: Some(Box::new(Expr::Object(ObjectLit {
                            span: Self::span(),
                            props: vec![PropOrSpread::Spread(SpreadElement {
                                dot3_token: Self::span(),
                                expr: Box::new(Self::this_expr()),
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
}

impl VisitMut for MacroTransformer {
    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        // Visit children first
        expr.visit_mut_children_with(self);

        // Transform macro calls
        if let Expr::Call(call) = expr
            && let Some(new_expr) = self.transform_include_str(call)
        {
            *expr = new_expr;
        }
    }

    fn visit_mut_class(&mut self, class: &mut Class) {
        // Visit children first
        class.visit_mut_children_with(self);

        // Process decorators if present
        if !class.decorators.is_empty() {
            self.transform_derive_decorator(class);
        }
    }

    fn visit_mut_class_decl(&mut self, decl: &mut ClassDecl) {
        // Visit the class
        decl.class.visit_mut_with(self);
    }
}
