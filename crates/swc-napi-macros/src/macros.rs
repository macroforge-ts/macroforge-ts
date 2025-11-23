use std::fs;
use std::path::Path;
use swc_common::{SourceMap, sync::Lrc};
use swc_ecma_ast::*;
use swc_ecma_visit::{VisitMut, VisitMutWith};

pub struct MacroTransformer {
    cm: Lrc<SourceMap>,
    filepath: String,
}

impl MacroTransformer {
    pub fn new(cm: Lrc<SourceMap>, filepath: String) -> Self {
        Self { cm, filepath }
    }

    /// Transform IncludeStr macro calls
    fn transform_include_str(&mut self, call: &mut CallExpr) -> Option<Expr> {
        // Check if this is an IncludeStr call
        if let Callee::Expr(expr) = &call.callee {
            if let Expr::Ident(ident) = &**expr {
                if ident.sym.as_ref() == "IncludeStr" {
                    // Get the first argument (file path)
                    if let Some(arg) = call.args.first() {
                        if let Expr::Lit(Lit::Str(str_lit)) = &*arg.expr {
                            let relative_path = str_lit.value.as_ref();

                            // Resolve path relative to the current file
                            let current_dir =
                                Path::new(&self.filepath).parent().unwrap_or(Path::new("."));
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
                }
            }
        }
        None
    }

    /// Transform @Derive decorator
    fn transform_derive_decorator(&mut self, class: &mut Class, decorators: &mut Vec<Decorator>) {
        let mut decorators_to_remove = vec![];
        let mut methods_to_add = vec![];

        for (idx, decorator) in decorators.iter().enumerate() {
            if let Expr::Call(call) = &*decorator.expr {
                if let Callee::Expr(expr) = &call.callee {
                    if let Expr::Ident(ident) = &**expr {
                        if ident.sym.as_ref() == "Derive" {
                            decorators_to_remove.push(idx);

                            // Process each feature argument
                            for arg in &call.args {
                                if let Expr::Lit(Lit::Str(str_lit)) = &*arg.expr {
                                    match str_lit.value.as_ref() {
                                        "Debug" => {
                                            methods_to_add.push(self.generate_to_string_method());
                                        }
                                        "JSON" => {
                                            methods_to_add.push(self.generate_to_json_method());
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Remove processed decorators (in reverse to maintain indices)
        for idx in decorators_to_remove.into_iter().rev() {
            decorators.remove(idx);
        }

        // Add generated methods to the class
        for method in methods_to_add {
            class.body.push(method);
        }
    }

    fn generate_to_string_method(&self) -> ClassMember {
        let span = || Default::default();
        let ident = |s: &str| Ident::new(s.into(), span());
        let this_expr = || Expr::This(ThisExpr { span: span() });

        // this.constructor
        let ctor_expr = Expr::Member(MemberExpr {
            span: span(),
            obj: Box::new(this_expr()),
            prop: MemberProp::Ident(ident("constructor")),
        });

        // JSON.stringify(this)
        let stringify_expr = Expr::Call(CallExpr {
            span: span(),
            callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                span: span(),
                obj: Box::new(Expr::Ident(ident("JSON"))),
                prop: MemberProp::Ident(ident("stringify")),
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
            key: PropName::Ident(ident("toString")),
            function: Box::new(Function {
                params: vec![],
                decorators: vec![],
                span: span(),
                body: Some(BlockStmt {
                    span: span(),
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

    fn generate_to_json_method(&self) -> ClassMember {
        ClassMember::Method(ClassMethod {
            span: Default::default(),
            key: PropName::Ident(Ident::new("toJSON".into(), Default::default())),
            function: Box::new(Function {
                params: vec![],
                decorators: vec![],
                span: Default::default(),
                body: Some(BlockStmt {
                    span: Default::default(),
                    stmts: vec![Stmt::Return(ReturnStmt {
                        span: Default::default(),
                        arg: Some(Box::new(Expr::Object(ObjectLit {
                            span: Default::default(),
                            props: vec![PropOrSpread::Spread(SpreadElement {
                                dot3_token: Default::default(),
                                expr: Box::new(Expr::This(ThisExpr {
                                    span: Default::default(),
                                })),
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
        if let Expr::Call(call) = expr {
            if let Some(new_expr) = self.transform_include_str(call) {
                *expr = new_expr;
            }
        }
    }

    fn visit_mut_class(&mut self, class: &mut Class) {
        // Visit children first
        class.visit_mut_children_with(self);

        // Process decorators if present
        if !class.decorators.is_empty() {
            self.transform_derive_decorator(class, &mut class.decorators);
        }
    }

    fn visit_mut_class_decl(&mut self, decl: &mut ClassDecl) {
        // Visit the class
        decl.class.visit_mut_with(self);
    }
}
