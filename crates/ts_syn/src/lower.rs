use ts_macro_abi::*;

use crate::TsSynError;

#[cfg(feature = "swc")]
use swc_common::{Span, Spanned, DUMMY_SP};
#[cfg(feature = "swc")]
use swc_ecma_ast::*;
#[cfg(feature = "swc")]
use swc_ecma_visit::{Visit, VisitWith};

/// Lower a module into ClassIR list (derive targets).
#[cfg(feature = "swc")]
pub fn lower_classes(module: &Module, source: &str) -> Result<Vec<ClassIR>, TsSynError> {
    let mut v = ClassCollector { out: vec![], source };
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

        let decorators = lower_decorators(&n.class.decorators);

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
                    optional: p.is_optional,  // Changed from p.optional
                    readonly: p.readonly,
                    visibility: lower_visibility(p.accessibility),
                    decorators: lower_decorators(&p.decorators),
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
                    decorators: lower_decorators(&meth.function.decorators),
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
fn lower_decorators(decs: &[Decorator]) -> Vec<DecoratorIR> {
    decs
        .iter()
        .filter_map(|d| {
            let span = swc_span_to_ir(d.span);
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
                    (callee, String::new()) // TODO: args snippet if needed
                }
                _ => return None,
            };
            Some(DecoratorIR { name, args_src, span })
        })
        .collect()
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
