use ts_macro_abi::*;

use crate::TsSynError;

#[cfg(feature = "swc")]
use swc_common::{DUMMY_SP, Span, Spanned};
#[cfg(feature = "swc")]
use swc_ecma_ast::*;
#[cfg(feature = "swc")]
use swc_ecma_visit::{Visit, VisitWith};

/// Lower a module into ClassIR list (derive targets).
#[cfg(feature = "swc")]
pub fn lower_classes(module: &Module, source: &str) -> Result<Vec<ClassIR>, TsSynError> {
    let mut v = ClassCollector {
        out: vec![],
        source,
    };
    module.visit_with(&mut v);
    Ok(v.out)
}

#[cfg(feature = "swc")]
struct ClassCollector<'a> {
    out: Vec<ClassIR>,
    source: &'a str,
}

#[cfg(feature = "swc")]
impl<'a> Visit for ClassCollector<'a> {
    fn visit_class_decl(&mut self, n: &ClassDecl) {
        let name = n.ident.sym.to_string();
        let span = swc_span_to_ir(n.class.span);

        let decorators = lower_decorators(&n.class.decorators, self.source);

        let (fields, methods) = lower_members(&n.class.body, self.source);

        self.out.push(ClassIR {
            name,
            span,
            is_abstract: n.class.is_abstract,
            type_params: vec![],
            heritage: vec![], // TODO: lower extends/implements
            decorators,
            fields,
            methods,
        });
    }
}

#[cfg(feature = "swc")]
fn lower_members(body: &[ClassMember], source: &str) -> (Vec<FieldIR>, Vec<MethodSigIR>) {
    let mut fields = vec![];
    let mut methods = vec![];

    for m in body {
        match m {
            ClassMember::ClassProp(p) => {
                let name = match &p.key {
                    PropName::Ident(i) => i.sym.to_string(),
                    _ => continue,
                };

                let ts_type = p
                    .type_ann
                    .as_ref()
                    .map(|t| snippet(source, t.span()))
                    .unwrap_or_else(|| "any".into());

                fields.push(FieldIR {
                    name,
                    span: swc_span_to_ir(p.span),
                    ts_type,
                    optional: p.is_optional, // Changed from p.optional
                    readonly: p.readonly,
                    visibility: lower_visibility(p.accessibility),
                    decorators: lower_decorators(&p.decorators, source),
                });
            }
            ClassMember::Method(meth) => {
                let name = match &meth.key {
                    PropName::Ident(i) => i.sym.to_string(),
                    _ => continue,
                };

                methods.push(MethodSigIR {
                    name,
                    span: swc_span_to_ir(meth.span),
                    params_src: snippet(source, params_span(&meth.function.params)),
                    return_type_src: meth
                        .function
                        .return_type
                        .as_ref()
                        .map(|t| snippet(source, t.span()))
                        .unwrap_or_else(|| "void".into()),
                    is_static: meth.is_static,
                    visibility: lower_visibility(meth.accessibility),
                    decorators: lower_decorators(&meth.function.decorators, source),
                });
            }
            _ => {}
        }
    }

    (fields, methods)
}

#[cfg(feature = "swc")]
fn params_span(params: &[Param]) -> Span {
    let mut span = DUMMY_SP;
    for p in params {
        span = match span.is_dummy() {
            true => p.span(),
            false => Span::new(span.lo, p.span().hi),
        };
    }
    span
}

#[cfg(feature = "swc")]
fn lower_decorators(decs: &[Decorator], source: &str) -> Vec<DecoratorIR> {
    decs.iter()
        .filter_map(|d| {
            let span = adjust_decorator_span(d.span, source);
            let (name, args_src) = match &*d.expr {
                Expr::Ident(i) => (i.sym.to_string(), String::new()),
                Expr::Call(call) => {
                    let callee = match &call.callee {
                        Callee::Expr(e) => match &**e {
                            Expr::Ident(i) => i.sym.to_string(),
                            _ => return None,
                        },
                        _ => return None,
                    };
                    (callee, call_args_src(call, source))
                }
                _ => return None,
            };
            Some(DecoratorIR {
                name,
                args_src,
                span,
            })
        })
        .collect()
}

fn adjust_decorator_span(span: swc_common::Span, source: &str) -> SpanIR {
    let mut ir = swc_span_to_ir(span);
    if ir.start > 0 {
        let start = ir.start as usize;
        if start <= source.len() {
            let bytes = source.as_bytes();
            if start > 0 && bytes[start - 1] == b'@' {
                ir.start -= 1;
            }
        }
    }
    ir
}

#[cfg(feature = "swc")]
fn call_args_src(call: &CallExpr, source: &str) -> String {
    if call.args.is_empty() {
        return String::new();
    }

    let call_src = snippet(source, call.span);
    if let (Some(open), Some(close)) = (call_src.find('('), call_src.rfind(')')) {
        if open + 1 <= close {
            return call_src[open + 1..close].trim().to_string();
        }
    }
    String::new()
}

#[cfg(feature = "swc")]
fn lower_visibility(acc: Option<Accessibility>) -> Visibility {
    match acc {
        Some(Accessibility::Public) => Visibility::Public,
        Some(Accessibility::Protected) => Visibility::Protected,
        Some(Accessibility::Private) => Visibility::Private,
        None => Visibility::Public,
    }
}

#[cfg(feature = "swc")]
fn swc_span_to_ir(sp: swc_common::Span) -> SpanIR {
    SpanIR::new(sp.lo.0, sp.hi.0)
}

#[cfg(feature = "swc")]
fn snippet(source: &str, sp: swc_common::Span) -> String {
    if sp.is_dummy() {
        return String::new();
    }
    let lo = sp.lo.0 as usize;
    let hi = sp.hi.0 as usize;
    source.get(lo..hi).unwrap_or("").to_string()
}

#[cfg(not(feature = "swc"))]
pub fn lower_classes(_module: &(), _source: &str) -> Result<Vec<ClassIR>, TsSynError> {
    Err(TsSynError::Unsupported("swc feature disabled".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "swc")]
    use swc_common::{FileName, GLOBALS, Globals, SourceMap, sync::Lrc};
    #[cfg(feature = "swc")]
    use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};

    #[cfg(feature = "swc")]
    fn parse_module(source: &str) -> Module {
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(
            FileName::Custom("test.ts".into()).into(),
            source.to_string(),
        );
        let lexer = Lexer::new(
            Syntax::Typescript(TsSyntax {
                tsx: false,
                decorators: true,
                ..Default::default()
            }),
            Default::default(),
            StringInput::from(&*fm),
            None,
        );
        let mut parser = Parser::new_from(lexer);
        parser.parse_module().expect("module to parse")
    }

    #[cfg(feature = "swc")]
    #[test]
    fn lowers_decorator_arguments_for_fields() {
        GLOBALS.set(&Globals::new(), || {
            let source = r#"
            class User {
                @Debug({ rename: "identifier", skip: false })
                id: string;
            }
            "#;
            let module = parse_module(source);
            let classes = lower_classes(&module, source).expect("lowering to succeed");
            let first = classes.first().expect("class");
            let field = first.fields.first().expect("field");
            let decorator = field.decorators.first().expect("decorator");
            assert_eq!(decorator.name, "Debug");
            assert_eq!(
                decorator.args_src.trim(),
                r#"{ rename: "identifier", skip: false }"#
            );
        });
    }

    #[cfg(feature = "swc")]
    #[test]
    fn class_decorator_span_captures_at_symbol() {
        GLOBALS.set(&Globals::new(), || {
            let source = r#"
            @Derive("Debug")
            class User {}
            "#;
            let module = parse_module(source);
            let classes = lower_classes(&module, source).expect("lowering to succeed");
            let class = classes.first().expect("class");
            let decorator = class.decorators.first().expect("decorator");
            let snippet =
                &source.as_bytes()[decorator.span.start as usize..decorator.span.end as usize];
            assert!(
                std::str::from_utf8(snippet).unwrap().starts_with("@Derive"),
                "decorator span should include '@Derive', got {:?}",
                std::str::from_utf8(snippet).unwrap()
            );
        });
    }
}
