use crate::abi::*;

use crate::TsSynError;

/// A lowered target that can be a class, interface, enum, or type alias
#[derive(Clone, Debug)]
pub enum LoweredTarget {
    Class(ClassIR),
    Interface(InterfaceIR),
    Enum(EnumIR),
    TypeAlias(TypeAliasIR),
}

#[cfg(feature = "swc")]
use swc_core::common::{Span, Spanned};
#[cfg(feature = "swc")]
use swc_core::ecma::ast::*;
#[cfg(feature = "swc")]
use swc_core::ecma::visit::{Visit, VisitWith};

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

/// Lower a module into InterfaceIR list (derive targets).
#[cfg(feature = "swc")]
pub fn lower_interfaces(module: &Module, source: &str) -> Result<Vec<InterfaceIR>, TsSynError> {
    let mut v = InterfaceCollector {
        out: vec![],
        source,
    };
    module.visit_with(&mut v);
    Ok(v.out)
}

/// Lower a module into all derive targets (classes and interfaces).
#[cfg(feature = "swc")]
pub fn lower_targets(module: &Module, source: &str) -> Result<Vec<LoweredTarget>, TsSynError> {
    let mut v = TargetCollector {
        out: vec![],
        source,
    };
    module.visit_with(&mut v);
    Ok(v.out)
}

/// Lower a module into EnumIR list (derive targets).
#[cfg(feature = "swc")]
pub fn lower_enums(module: &Module, source: &str) -> Result<Vec<EnumIR>, TsSynError> {
    let mut v = EnumCollector {
        out: vec![],
        source,
    };
    module.visit_with(&mut v);
    Ok(v.out)
}

/// Lower a module into TypeAliasIR list (derive targets).
#[cfg(feature = "swc")]
pub fn lower_type_aliases(module: &Module, source: &str) -> Result<Vec<TypeAliasIR>, TsSynError> {
    let mut v = TypeAliasCollector {
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

        let class_source = snippet(self.source, n.class.span);
        let body_span = if let (Some(open_brace), Some(close_brace)) =
            (class_source.find('{'), class_source.rfind('}'))
        {
            SpanIR::new(
                n.class.span.lo.0 + open_brace as u32,
                n.class.span.lo.0 + close_brace as u32 + 1,
            )
        } else {
            // fallback for classes without a body, though they can't have macros.
            span
        };

        let mut decorators = lower_decorators(&n.class.decorators, self.source);
        decorators.extend(collect_leading_macro_directives(
            self.source,
            n.class.span.lo.0 as usize,
        ));

        let (fields, methods) = lower_members(&n.class.body, self.source);

        self.out.push(ClassIR {
            name,
            span,
            body_span,
            is_abstract: n.class.is_abstract,
            type_params: vec![],
            heritage: vec![], // TODO: lower extends/implements
            decorators,
            decorators_ast: n.class.decorators.clone(),
            fields,
            methods,
            members: n.class.body.clone(),
        });
    }
}

#[cfg(feature = "swc")]
struct InterfaceCollector<'a> {
    out: Vec<InterfaceIR>,
    source: &'a str,
}

#[cfg(feature = "swc")]
impl<'a> Visit for InterfaceCollector<'a> {
    fn visit_ts_interface_decl(&mut self, n: &TsInterfaceDecl) {
        if let Some(ir) = lower_interface(n, self.source) {
            self.out.push(ir);
        }
    }
}

#[cfg(feature = "swc")]
struct EnumCollector<'a> {
    out: Vec<EnumIR>,
    source: &'a str,
}

#[cfg(feature = "swc")]
impl<'a> Visit for EnumCollector<'a> {
    fn visit_ts_enum_decl(&mut self, n: &TsEnumDecl) {
        if let Some(ir) = lower_enum(n, self.source) {
            self.out.push(ir);
        }
    }
}

#[cfg(feature = "swc")]
struct TypeAliasCollector<'a> {
    out: Vec<TypeAliasIR>,
    source: &'a str,
}

#[cfg(feature = "swc")]
impl<'a> Visit for TypeAliasCollector<'a> {
    fn visit_ts_type_alias_decl(&mut self, n: &TsTypeAliasDecl) {
        if let Some(ir) = lower_type_alias(n, self.source) {
            self.out.push(ir);
        }
    }
}

#[cfg(feature = "swc")]
struct TargetCollector<'a> {
    out: Vec<LoweredTarget>,
    source: &'a str,
}

#[cfg(feature = "swc")]
impl<'a> Visit for TargetCollector<'a> {
    fn visit_class_decl(&mut self, n: &ClassDecl) {
        let name = n.ident.sym.to_string();
        let span = swc_span_to_ir(n.class.span);

        let class_source = snippet(self.source, n.class.span);
        let body_span = if let (Some(open_brace), Some(close_brace)) =
            (class_source.find('{'), class_source.rfind('}'))
        {
            SpanIR::new(
                n.class.span.lo.0 + open_brace as u32,
                n.class.span.lo.0 + close_brace as u32 + 1,
            )
        } else {
            span
        };

        let mut decorators = lower_decorators(&n.class.decorators, self.source);
        decorators.extend(collect_leading_macro_directives(
            self.source,
            n.class.span.lo.0 as usize,
        ));

        let (fields, methods) = lower_members(&n.class.body, self.source);

        self.out.push(LoweredTarget::Class(ClassIR {
            name,
            span,
            body_span,
            is_abstract: n.class.is_abstract,
            type_params: vec![],
            heritage: vec![],
            decorators,
            decorators_ast: n.class.decorators.clone(),
            fields,
            methods,
            members: n.class.body.clone(),
        }));
    }

    fn visit_ts_interface_decl(&mut self, n: &TsInterfaceDecl) {
        if let Some(ir) = lower_interface(n, self.source) {
            self.out.push(LoweredTarget::Interface(ir));
        }
    }

    fn visit_ts_enum_decl(&mut self, n: &TsEnumDecl) {
        if let Some(ir) = lower_enum(n, self.source) {
            self.out.push(LoweredTarget::Enum(ir));
        }
    }

    fn visit_ts_type_alias_decl(&mut self, n: &TsTypeAliasDecl) {
        if let Some(ir) = lower_type_alias(n, self.source) {
            self.out.push(LoweredTarget::TypeAlias(ir));
        }
    }
}

#[cfg(feature = "swc")]
fn lower_interface(n: &TsInterfaceDecl, source: &str) -> Option<InterfaceIR> {
    let name = n.id.sym.to_string();
    let span = swc_span_to_ir(n.span);

    let interface_source = snippet(source, n.span);
    let body_span = if let (Some(open_brace), Some(close_brace)) =
        (interface_source.find('{'), interface_source.rfind('}'))
    {
        SpanIR::new(
            n.span.lo.0 + open_brace as u32,
            n.span.lo.0 + close_brace as u32 + 1,
        )
    } else {
        span
    };

    // Collect decorators from leading JSDoc comments
    let decorators = collect_leading_macro_directives(source, n.span.lo.0 as usize);

    let (fields, methods) = lower_interface_members(&n.body.body, source);

    Some(InterfaceIR {
        name,
        span,
        body_span,
        type_params: vec![], // TODO: extract type params
        heritage: vec![],    // TODO: extract extends
        decorators,
        fields,
        methods,
    })
}

#[cfg(feature = "swc")]
fn lower_enum(n: &TsEnumDecl, source: &str) -> Option<EnumIR> {
    let name = n.id.sym.to_string();
    let span = swc_span_to_ir(n.span);

    let enum_source = snippet(source, n.span);
    let body_span = if let (Some(open_brace), Some(close_brace)) =
        (enum_source.find('{'), enum_source.rfind('}'))
    {
        SpanIR::new(
            n.span.lo.0 + open_brace as u32,
            n.span.lo.0 + close_brace as u32 + 1,
        )
    } else {
        span
    };

    // Collect decorators from leading JSDoc comments
    let decorators = collect_leading_macro_directives(source, n.span.lo.0 as usize);

    // Lower enum members with values
    let variants = lower_enum_members(&n.members, source);

    Some(EnumIR {
        name,
        span,
        body_span,
        decorators,
        variants,
        is_const: n.is_const,
    })
}

#[cfg(feature = "swc")]
fn lower_enum_members(members: &[TsEnumMember], source: &str) -> Vec<EnumVariantIR> {
    let mut variants = vec![];
    let mut next_auto_value: f64 = 0.0;

    for member in members {
        let name = match &member.id {
            TsEnumMemberId::Ident(i) => i.sym.to_string(),
            TsEnumMemberId::Str(s) => String::from_utf8_lossy(s.value.as_bytes()).to_string(),
        };

        // Collect field-level decorators from JSDoc comments
        let decorators = collect_leading_macro_directives(source, member.span.lo.0 as usize);

        // Extract the value from the initializer
        let value = if let Some(init) = &member.init {
            match &**init {
                // String literal: "ACTIVE"
                Expr::Lit(Lit::Str(s)) => {
                    // String enums reset auto-increment behavior
                    EnumValue::String(String::from_utf8_lossy(s.value.as_bytes()).to_string())
                }
                // Numeric literal: 42
                Expr::Lit(Lit::Num(n)) => {
                    let val = n.value;
                    next_auto_value = val + 1.0;
                    EnumValue::Number(val)
                }
                // Unary expression: -1, +5
                Expr::Unary(unary) if matches!(unary.op, swc_core::ecma::ast::UnaryOp::Minus | swc_core::ecma::ast::UnaryOp::Plus) => {
                    if let Expr::Lit(Lit::Num(n)) = &*unary.arg {
                        let val = if matches!(unary.op, swc_core::ecma::ast::UnaryOp::Minus) {
                            -n.value
                        } else {
                            n.value
                        };
                        next_auto_value = val + 1.0;
                        EnumValue::Number(val)
                    } else {
                        // Complex unary expression, store as expression
                        let expr_src = snippet(source, init.span());
                        EnumValue::Expr(expr_src)
                    }
                }
                // Any other expression (function call, binary op, etc.)
                _ => {
                    let expr_src = snippet(source, init.span());
                    // Try to evaluate simple expressions like A + 1 where A is known
                    EnumValue::Expr(expr_src)
                }
            }
        } else {
            // No initializer - use auto-increment
            let val = next_auto_value;
            next_auto_value += 1.0;
            // Check if this is truly auto (first member with no init) or follows string enum
            // For simplicity, we track numeric auto-increment
            EnumValue::Number(val)
        };

        variants.push(EnumVariantIR {
            name,
            span: swc_span_to_ir(member.span),
            value,
            decorators,
        });
    }

    variants
}

#[cfg(feature = "swc")]
fn lower_type_alias(n: &TsTypeAliasDecl, source: &str) -> Option<TypeAliasIR> {
    let name = n.id.sym.to_string();
    let span = swc_span_to_ir(n.span);

    // Collect decorators from leading JSDoc comments
    let decorators = collect_leading_macro_directives(source, n.span.lo.0 as usize);

    // Extract type parameters
    let type_params = n
        .type_params
        .as_ref()
        .map(|tp| {
            tp.params
                .iter()
                .map(|p| p.name.sym.to_string())
                .collect()
        })
        .unwrap_or_default();

    // Lower the type body
    let body = lower_type_body(&n.type_ann, source);

    Some(TypeAliasIR {
        name,
        span,
        decorators,
        type_params,
        body,
    })
}

#[cfg(feature = "swc")]
fn lower_type_body(ts_type: &TsType, source: &str) -> TypeBody {
    use swc_core::ecma::ast::TsType::*;

    match ts_type {
        // Union type: A | B | C
        TsUnionOrIntersectionType(swc_core::ecma::ast::TsUnionOrIntersectionType::TsUnionType(
            union,
        )) => {
            let members: Vec<TypeMember> = union
                .types
                .iter()
                .map(|t| lower_type_member(t, source))
                .collect();
            TypeBody::Union(members)
        }

        // Intersection type: A & B & C
        TsUnionOrIntersectionType(
            swc_core::ecma::ast::TsUnionOrIntersectionType::TsIntersectionType(intersection),
        ) => {
            let members: Vec<TypeMember> = intersection
                .types
                .iter()
                .map(|t| lower_type_member(t, source))
                .collect();
            TypeBody::Intersection(members)
        }

        // Type literal (object type): { x: number; y: number }
        TsTypeLit(lit) => {
            let (fields, _methods) = lower_interface_members(&lit.members, source);
            TypeBody::Object { fields }
        }

        // Tuple type: [string, number]
        TsTupleType(tuple) => {
            let elements: Vec<String> = tuple
                .elem_types
                .iter()
                .map(|elem| snippet(source, elem.span()))
                .collect();
            TypeBody::Tuple(elements)
        }

        // Type reference: string, number, Array<T>, etc.
        TsTypeRef(type_ref) => {
            let type_str = snippet(source, type_ref.span());
            TypeBody::Alias(type_str)
        }

        // Keyword types: string, number, boolean, etc.
        TsKeywordType(kw) => {
            let type_str = snippet(source, kw.span);
            TypeBody::Alias(type_str)
        }

        // Literal type: "active", 42
        TsLitType(lit) => {
            let type_str = snippet(source, lit.span);
            TypeBody::Alias(type_str)
        }

        // Array type: string[]
        TsArrayType(arr) => {
            let type_str = snippet(source, arr.span);
            TypeBody::Alias(type_str)
        }

        // Fallback for other types (mapped, conditional, etc.)
        _ => {
            let type_str = snippet(source, ts_type.span());
            TypeBody::Other(type_str)
        }
    }
}

#[cfg(feature = "swc")]
fn lower_type_member(ts_type: &TsType, source: &str) -> TypeMember {
    use crate::abi::ir::type_alias::TypeMemberKind;
    use swc_core::ecma::ast::TsType::*;

    // Parse any leading JSDoc comment decorators (e.g., /** @default */)
    let decorators = collect_leading_macro_directives(source, ts_type.span().lo.0 as usize);

    let kind = match ts_type {
        // Literal type: "active", 42, true
        TsLitType(lit) => {
            let lit_str = snippet(source, lit.span);
            TypeMemberKind::Literal(lit_str)
        }

        // Type literal (inline object): { role: string }
        TsTypeLit(lit) => {
            let (fields, _methods) = lower_interface_members(&lit.members, source);
            TypeMemberKind::Object { fields }
        }

        // Type reference or other: User, string, Array<T>
        _ => {
            let type_str = snippet(source, ts_type.span());
            TypeMemberKind::TypeRef(type_str)
        }
    };

    TypeMember::with_decorators(kind, decorators)
}

#[cfg(feature = "swc")]
fn lower_interface_members(
    body: &[TsTypeElement],
    source: &str,
) -> (Vec<InterfaceFieldIR>, Vec<InterfaceMethodIR>) {
    let mut fields = vec![];
    let mut methods = vec![];

    for elem in body {
        match elem {
            TsTypeElement::TsPropertySignature(prop) => {
                let name = match &*prop.key {
                    Expr::Ident(i) => i.sym.to_string(),
                    _ => continue,
                };

                let ts_type = prop
                    .type_ann
                    .as_ref()
                    .map(|t| snippet(source, t.type_ann.span()))
                    .unwrap_or_else(|| "any".into());

                // Collect decorators from leading JSDoc comments
                let decorators = collect_leading_macro_directives(source, prop.span.lo.0 as usize);

                fields.push(InterfaceFieldIR {
                    name,
                    span: swc_span_to_ir(prop.span),
                    ts_type,
                    optional: prop.optional,
                    readonly: prop.readonly,
                    decorators,
                });
            }
            TsTypeElement::TsMethodSignature(meth) => {
                let name = match &*meth.key {
                    Expr::Ident(i) => i.sym.to_string(),
                    _ => continue,
                };

                let method_src = snippet(source, meth.span);
                let params_src = extract_params_from_source(&method_src);
                let type_params_src = extract_type_params_from_source(&method_src, &name);

                let return_type_src = meth
                    .type_ann
                    .as_ref()
                    .map(|t| snippet(source, t.type_ann.span()).trim().to_string())
                    .unwrap_or_else(|| "void".into());

                let decorators = collect_leading_macro_directives(source, meth.span.lo.0 as usize);

                methods.push(InterfaceMethodIR {
                    name,
                    span: swc_span_to_ir(meth.span),
                    type_params_src,
                    params_src,
                    return_type_src,
                    optional: meth.optional,
                    decorators,
                });
            }
            _ => {}
        }
    }

    (fields, methods)
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
                    .map(|t| snippet(source, t.type_ann.span()))
                    .unwrap_or_else(|| "any".into());

                let mut decorators = lower_decorators(&p.decorators, source);
                decorators.extend(collect_leading_macro_directives(
                    source,
                    p.span.lo.0 as usize,
                ));

                fields.push(FieldIR {
                    name,
                    span: swc_span_to_ir(p.span),
                    ts_type,
                    type_ann: p.type_ann.as_ref().map(|ann| ann.type_ann.clone()),
                    optional: p.is_optional, // Changed from p.optional
                    readonly: p.readonly,
                    visibility: lower_visibility(p.accessibility),
                    decorators,
                    prop_ast: Some(p.clone()),
                });
            }
            ClassMember::Method(meth) => {
                let name = match &meth.key {
                    PropName::Ident(i) => i.sym.to_string(),
                    _ => continue,
                };

                let method_span = if let Some(body) = &meth.function.body {
                    Span::new(meth.span.lo, body.span.hi)
                } else {
                    meth.span
                };

                // Extract parameters and type parameters from method source
                let method_src = snippet(source, meth.span);
                let params_src = extract_params_from_source(&method_src);
                let type_params_src = extract_type_params_from_source(&method_src, &name);

                // Adjust span to find the actual start (handles modifiers like public, static, async)
                let adjusted_span = adjust_method_span(
                    source,
                    method_span,
                    &name,
                    meth.is_static,
                    meth.accessibility,
                );

                methods.push(MethodSigIR {
                    name,
                    span: swc_span_to_ir(adjusted_span),
                    type_params_src,
                    params_src,
                    return_type_src: meth
                        .function
                        .return_type
                        .as_ref()
                        .map(|t| snippet(source, t.span()).trim().to_string())
                        .unwrap_or_else(|| "void".into()),
                    is_static: meth.is_static,
                    is_async: meth.function.is_async,
                    visibility: lower_visibility(meth.accessibility),
                    decorators: lower_decorators(&meth.function.decorators, source),
                    member_ast: Some(MethodAstIR::Method(meth.clone())),
                });
            }
            ClassMember::Constructor(c) => {
                let constructor_span = if let Some(body) = &c.body {
                    Span::new(c.span.lo, body.span.hi)
                } else {
                    c.span
                };

                // Extract parameters from constructor source
                let constructor_src = snippet(source, c.span);
                let params_src = extract_params_from_source(&constructor_src);

                // Adjust span to find the actual start (handles modifiers like private, protected, public)
                let adjusted_span =
                    adjust_constructor_span(source, constructor_span, c.accessibility);

                methods.push(MethodSigIR {
                    name: "constructor".into(),
                    span: swc_span_to_ir(adjusted_span),
                    type_params_src: String::new(), // constructors don't have type parameters
                    params_src,
                    return_type_src: String::new(), // constructors don't have return types
                    is_static: false,
                    is_async: false, // constructors can't be async
                    visibility: lower_visibility(c.accessibility),
                    decorators: vec![], // Constructors don't have decorators
                    member_ast: Some(MethodAstIR::Constructor(c.clone())),
                });
            }
            _ => {}
        }
    }

    (fields, methods)
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
                node: Some(d.clone()),
            })
        })
        .collect()
}

fn collect_leading_macro_directives(source: &str, target_start: usize) -> Vec<DecoratorIR> {
    // target_start is 1-based (from SWC BytePos), convert to 0-based for slicing
    let target_start_0 = target_start.saturating_sub(1);
    if target_start_0 == 0 || target_start_0 > source.len() {
        return Vec::new();
    }

    let search_area = &source[..target_start_0];

    // Find the last "*/" in the search_area
    let Some(end_idx_in_search_area) = search_area.rfind("*/") else {
        return Vec::new();
    };

    // Find the matching "/**" before that "*/"
    let Some(start_idx) = search_area[..end_idx_in_search_area].rfind("/**") else {
        return Vec::new();
    };

    let end_of_comment_block = end_idx_in_search_area + 2; // +2 for "*/"

    // Only accept if the comment is directly adjacent to the target
    // Allow common modifiers between comment and target (export, declare, abstract, etc.)
    let between = &search_area[end_of_comment_block..];
    let between_trimmed = between.trim();
    if !between_trimmed.is_empty() {
        // Check if only allowed modifiers are between comment and target
        let allowed_modifiers = ["export", "declare", "abstract", "default", "async"];
        let remaining: String = between_trimmed
            .split_whitespace()
            .filter(|word| !allowed_modifiers.contains(word))
            .collect::<Vec<_>>()
            .join(" ");
        if !remaining.is_empty() {
            // There's non-whitespace, non-modifier content between the comment and target
            return Vec::new();
        }
    }

    let comment_body = &search_area[start_idx + 3 .. end_idx_in_search_area];

    // Parse ALL macro directives from the comment
    let directives = parse_all_macro_directives(comment_body);

    directives
        .into_iter()
        .map(|(name, args_src)| {
            let final_span_ir = adjust_decorator_span(
                swc_core::common::Span::new(
                    swc_core::common::BytePos(start_idx as u32 + 1), // Convert to 1-based BytePos
                    swc_core::common::BytePos(end_of_comment_block as u32 + 1), // Convert to 1-based BytePos
                ),
                source,
            );

            DecoratorIR {
                name,
                args_src,
                span: final_span_ir,
                node: None,
            }
        })
        .collect()
}

/// Parse ALL macro directives from a JSDoc comment body.
/// Returns a Vec of (name, args) tuples for each @directive or @directive(...) found.
/// Supports multiple directives on the same line: `@derive(X) @default(Y)`
/// Also supports directives without parens: `@default` (treated as empty args)
#[cfg(feature = "swc")]
fn parse_all_macro_directives(comment_body: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();

    // Process line by line to handle multiple directives
    for line in comment_body.lines() {
        let line = line.trim().trim_start_matches('*').trim();

        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        // Parse all directives on this line
        let mut remaining = line;
        while let Some(at_idx) = remaining.find('@') {
            let after_at = &remaining[at_idx + 1..];

            // Extract the directive name (alphanumeric chars until space, paren, or end)
            let name_end = after_at
                .find(|c: char| !c.is_alphanumeric() && c != '_')
                .unwrap_or(after_at.len());

            if name_end == 0 {
                // No valid name found, skip
                remaining = &remaining[at_idx + 1..];
                continue;
            }

            let name = &after_at[..name_end];
            let after_name = &after_at[name_end..];

            // Check if there's an opening paren
            let trimmed_after_name = after_name.trim_start();
            if trimmed_after_name.starts_with('(') {
                // Has arguments - parse balanced parens
                let paren_start = after_name.len() - trimmed_after_name.len();
                let args_start = paren_start + 1;

                // Parse balanced parentheses to find the closing one
                let mut depth: i32 = 1;
                let mut brace_depth: i32 = 0;
                let mut bracket_depth: i32 = 0;
                let mut close_idx = None;

                for (i, c) in after_name[args_start..].char_indices() {
                    match c {
                        '(' => depth += 1,
                        ')' => {
                            depth -= 1;
                            if depth == 0 && brace_depth == 0 && bracket_depth == 0 {
                                close_idx = Some(args_start + i);
                                break;
                            }
                        }
                        '{' => brace_depth += 1,
                        '}' => brace_depth = brace_depth.saturating_sub(1),
                        '[' => bracket_depth += 1,
                        ']' => bracket_depth = bracket_depth.saturating_sub(1),
                        _ => {}
                    }
                }

                if let Some(close) = close_idx {
                    let args = after_name[args_start..close].trim();

                    let normalized_name = if name.eq_ignore_ascii_case("derive") {
                        "Derive".to_string()
                    } else {
                        name.to_string()
                    };

                    results.push((normalized_name, args.to_string()));

                    // Continue searching after this directive's closing paren
                    let end_of_directive = at_idx + 1 + name_end + close + 1;
                    if end_of_directive < remaining.len() {
                        remaining = &remaining[end_of_directive..];
                    } else {
                        break;
                    }
                } else {
                    // No matching close paren, skip to next @ symbol
                    remaining = &remaining[at_idx + 1..];
                }
            } else {
                // No parens - directive without arguments (like @default)
                let normalized_name = if name.eq_ignore_ascii_case("derive") {
                    "Derive".to_string()
                } else {
                    name.to_string()
                };

                results.push((normalized_name, String::new()));

                // Continue after the directive name
                let end_of_directive = at_idx + 1 + name_end;
                if end_of_directive < remaining.len() {
                    remaining = &remaining[end_of_directive..];
                } else {
                    break;
                }
            }
        }
    }

    results
}


fn adjust_decorator_span(span: Span, source: &str) -> SpanIR {
    let mut ir = swc_span_to_ir(span);
    let bytes = source.as_bytes();
    let mut start = ir.start.saturating_sub(1) as usize;
    let mut end = ir.end.saturating_sub(1) as usize;

    // Extend backward to include '@' symbol and any leading whitespace on the same line
    if start > 0 && bytes[start - 1] == b'@' {
        start -= 1;
        // Continue backward to include leading whitespace, but stop at newline
        while start > 0 && (bytes[start - 1] == b' ' || bytes[start - 1] == b'\t') {
            start -= 1;
        }
        ir.start = start as u32;
    }

    // Extend forward to include only one trailing newline character if it exists
    if end < bytes.len() && bytes[end] == b'\n' {
        end += 1; // Include the newline
        ir.end = end as u32;
    }
    ir
}

#[cfg(feature = "swc")]
fn call_args_src(call: &CallExpr, source: &str) -> String {
    if call.args.is_empty() {
        return String::new();
    }

    let call_src = snippet(source, call.span);
    if let (Some(open), Some(close)) = (call_src.find('('), call_src.rfind(')'))
        && open < close
    {
        return call_src[open + 1..close].trim().to_string();
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
fn swc_span_to_ir(sp: Span) -> SpanIR {
    SpanIR::new(sp.lo.0, sp.hi.0)
}

#[cfg(feature = "swc")]
fn snippet(source: &str, sp: Span) -> String {
    if sp.is_dummy() {
        return String::new();
    }
    // SWC BytePos is 1-based by default for the first file.
    // We subtract 1 to map to 0-based string index.
    // TODO: This assumes the span is from a file starting at 1.
    // For robust multi-file support, lower_classes needs the file start position.
    let lo = (sp.lo.0 as usize).saturating_sub(1);
    let hi = (sp.hi.0 as usize).saturating_sub(1);

    if lo >= source.len() {
        return String::new();
    }
    let end = std::cmp::min(hi, source.len());

    source.get(lo..end).unwrap_or("").to_string()
}

#[cfg(feature = "swc")]
fn extract_params_from_source(method_src: &str) -> String {
    // Find the first '(' which starts the parameter list
    let Some(open) = method_src.find('(') else {
        return String::new();
    };

    // Now find the matching closing ')', accounting for nested parentheses
    let chars: Vec<char> = method_src.chars().collect();
    let mut depth = 0;
    let mut close_pos = None;

    for (i, &ch) in chars.iter().enumerate().skip(open) {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    close_pos = Some(i);
                    break;
                }
            }
            _ => {}
        }
    }

    if let Some(close) = close_pos
        && open < close
    {
        return chars[open + 1..close].iter().collect();
    }

    String::new()
}

#[cfg(feature = "swc")]
fn extract_type_params_from_source(method_src: &str, method_name: &str) -> String {
    // Find the method name in the source
    let Some(name_pos) = method_src.find(method_name) else {
        return String::new();
    };

    // Look for '<' after the method name
    let after_name = &method_src[name_pos + method_name.len()..];
    let chars: Vec<char> = after_name.chars().collect();

    // Skip whitespace
    let mut i = 0;
    while i < chars.len() && chars[i].is_whitespace() {
        i += 1;
    }

    // Check if we have a '<'
    if i >= chars.len() || chars[i] != '<' {
        return String::new();
    }

    let start = i;
    i += 1; // skip the '<'
    let mut depth = 1;

    // Find matching '>'
    while i < chars.len() && depth > 0 {
        match chars[i] {
            '<' => depth += 1,
            '>' => depth -= 1,
            _ => {}
        }
        i += 1;
    }

    if depth == 0 {
        return chars[start..i].iter().collect();
    }

    String::new()
}

#[cfg(feature = "swc")]
fn adjust_constructor_span(source: &str, span: Span, accessibility: Option<Accessibility>) -> Span {
    let search_start = span.lo.0 as usize;
    let bytes = source.as_bytes();

    // Build list of possible keywords
    let mut keywords = vec!["constructor"];
    match accessibility {
        Some(Accessibility::Public) => keywords.insert(0, "public"),
        Some(Accessibility::Protected) => keywords.insert(0, "protected"),
        Some(Accessibility::Private) => keywords.insert(0, "private"),
        None => {}
    }

    // Search backwards for the earliest matching keyword
    let mut earliest_start = search_start;

    for keyword in &keywords {
        let keyword_bytes = keyword.as_bytes();
        let search_region_start = search_start.saturating_sub(keyword.len() + 20);
        let search_region_end = (search_start + keyword.len()).min(source.len());

        if search_region_start >= search_region_end {
            continue;
        }

        let search_region = &bytes[search_region_start..search_region_end];

        for i in 0..search_region.len() {
            if i + keyword_bytes.len() <= search_region.len() {
                let candidate = &search_region[i..i + keyword_bytes.len()];
                if candidate == keyword_bytes {
                    let abs_pos = search_region_start + i;
                    if abs_pos < earliest_start {
                        let is_word_boundary_before = abs_pos == 0
                            || !bytes[abs_pos - 1].is_ascii_alphanumeric()
                                && bytes[abs_pos - 1] != b'_';
                        let is_word_boundary_after = abs_pos + keyword_bytes.len() >= bytes.len()
                            || !bytes[abs_pos + keyword_bytes.len()].is_ascii_alphanumeric()
                                && bytes[abs_pos + keyword_bytes.len()] != b'_';

                        if is_word_boundary_before && is_word_boundary_after {
                            earliest_start = abs_pos;
                        }
                    }
                }
            }
        }
    }

    Span::new(swc_core::common::BytePos(earliest_start as u32), span.hi)
}

#[cfg(feature = "swc")]
fn adjust_method_span(
    source: &str,
    span: Span,
    method_name: &str,
    is_static: bool,
    accessibility: Option<Accessibility>,
) -> Span {
    let search_start = span.lo.0 as usize;
    let bytes = source.as_bytes();

    // Build list of possible keywords that might precede the method name
    let mut keywords = vec![method_name];
    if is_static {
        keywords.insert(0, "static");
    }
    match accessibility {
        Some(Accessibility::Public) => keywords.insert(0, "public"),
        Some(Accessibility::Protected) => keywords.insert(0, "protected"),
        Some(Accessibility::Private) => keywords.insert(0, "private"),
        None => {}
    }
    // Also check for async keyword
    keywords.push("async");

    // Search backwards for the earliest matching keyword
    let mut earliest_start = search_start;

    for keyword in &keywords {
        let keyword_bytes = keyword.as_bytes();
        let search_region_start = search_start.saturating_sub(keyword.len() + 20); // Extra buffer for whitespace
        let search_region_end = (search_start + keyword.len()).min(source.len());

        if search_region_start >= search_region_end {
            continue;
        }

        let search_region = &bytes[search_region_start..search_region_end];

        // Find all occurrences of this keyword
        for i in 0..search_region.len() {
            if i + keyword_bytes.len() <= search_region.len() {
                let candidate = &search_region[i..i + keyword_bytes.len()];
                if candidate == keyword_bytes {
                    let abs_pos = search_region_start + i;
                    if abs_pos < earliest_start {
                        // Verify it's a whole word (not part of a larger identifier)
                        let is_word_boundary_before = abs_pos == 0
                            || !bytes[abs_pos - 1].is_ascii_alphanumeric()
                                && bytes[abs_pos - 1] != b'_';
                        let is_word_boundary_after = abs_pos + keyword_bytes.len() >= bytes.len()
                            || !bytes[abs_pos + keyword_bytes.len()].is_ascii_alphanumeric()
                                && bytes[abs_pos + keyword_bytes.len()] != b'_';

                        if is_word_boundary_before && is_word_boundary_after {
                            earliest_start = abs_pos;
                        }
                    }
                }
            }
        }
    }

    Span::new(swc_core::common::BytePos(earliest_start as u32), span.hi)
}

#[cfg(not(feature = "swc"))]
pub fn lower_classes(_module: &(), _source: &str) -> Result<Vec<ClassIR>, TsSynError> {
    Err(TsSynError::Unsupported("swc feature disabled".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "swc")]
    use swc_core::common::{FileName, GLOBALS, Globals, SourceMap, sync::Lrc};
    #[cfg(feature = "swc")]
    use swc_core::ecma::parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};

    #[cfg(feature = "swc")]
    #[test]
    fn test_regular_method_params() {
        GLOBALS.set(&Globals::new(), || {
            let source =
                "class User { getName(prefix: string, suffix: string): string { return ''; } }";
            let module = parse_module(source);
            let classes = lower_classes(&module, source).expect("lowering to succeed");
            let class = classes.first().expect("class");
            let method = class
                .methods
                .iter()
                .find(|m| m.name == "getName")
                .expect("getName method");

            // The params_src should include the commas and spacing
            assert!(method.params_src.contains("prefix: string"));
            assert!(method.params_src.contains("suffix: string"));
        });
    }

    #[cfg(feature = "swc")]
    #[test]
    fn test_method_with_modifiers() {
        GLOBALS.set(&Globals::new(), || {
            let source =
                "class User { public static async getUser(): Promise<User> { return null!; } }";
            let module = parse_module(source);
            let classes = lower_classes(&module, source).expect("lowering to succeed");
            let class = classes.first().expect("class");
            let method = class.methods.first().expect("method");

            // Check if the span starts at the correct position
            assert!(
                source[method.span.start as usize..].starts_with("public")
                    || source[method.span.start as usize..].starts_with("getUser"),
                "Method span should start at modifier or method name, got: {:?}",
                &source[method.span.start as usize..method.span.start as usize + 10]
            );
        });
    }

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
            /** @derive(Debug) */
            class User {}
            "#;
            let module = parse_module(source);
            let classes = lower_classes(&module, source).expect("lowering to succeed");
            let class = classes.first().expect("class");
            let decorator = class.decorators.first().expect("decorator");
            let snippet =
                &source.as_bytes()[decorator.span.start as usize..decorator.span.end as usize];
            let snippet_str = std::str::from_utf8(snippet).unwrap();

            // The span now includes leading whitespace for clean deletion
            assert!(
                snippet_str.contains("@derive"),
                "decorator span should include '@derive', got {:?}",
                snippet_str
            );

            // Verify it includes the trailing newline and next line's indentation
            assert!(
                snippet_str.contains('\n'),
                "decorator span should include trailing newline for clean deletion, got {:?}",
                snippet_str
            );
        });
    }

    #[cfg(feature = "swc")]
    #[test]
    fn parse_all_macro_directives_single_line() {
        let comment_body = " @derive(Default, Deserialize) ";
        let directives = parse_all_macro_directives(comment_body);
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].0, "Derive");
        assert_eq!(directives[0].1, "Default, Deserialize");
    }

    #[cfg(feature = "swc")]
    #[test]
    fn parse_all_macro_directives_multiline() {
        let comment_body = r#"
         * @derive(Default, Deserialize)
         * @default(Created.defaultValue())
         "#;
        let directives = parse_all_macro_directives(comment_body);
        assert_eq!(
            directives.len(),
            2,
            "Expected 2 directives, got {:?}",
            directives
        );
        assert_eq!(directives[0].0, "Derive");
        assert_eq!(directives[0].1, "Default, Deserialize");
        assert_eq!(directives[1].0, "default");
        assert_eq!(directives[1].1, "Created.defaultValue()");
    }

    #[cfg(feature = "swc")]
    #[test]
    fn parse_all_macro_directives_same_line() {
        let comment_body = " @derive(Default) @default(Foo.defaultValue()) ";
        let directives = parse_all_macro_directives(comment_body);
        // Should parse both directives on the same line
        assert_eq!(
            directives.len(),
            2,
            "Expected 2 directives on same line, got {:?}",
            directives
        );
        assert_eq!(directives[0].0, "Derive");
        assert_eq!(directives[0].1, "Default");
        assert_eq!(directives[1].0, "default");
        assert_eq!(directives[1].1, "Foo.defaultValue()");
    }

    #[cfg(feature = "swc")]
    #[test]
    fn type_alias_with_multiple_decorators() {
        GLOBALS.set(&Globals::new(), || {
            // Note: No leading newline, comment directly at start
            let source = r#"/**
 * @derive(Default, Deserialize)
 * @default(DailyRecurrenceRule.defaultValue())
 */
export type Interval = DailyRecurrenceRule | WeeklyRecurrenceRule;"#;
            let module = parse_module(source);
            let type_aliases = lower_type_aliases(&module, source).expect("lowering to succeed");
            let alias = type_aliases.first().expect("type alias");

            assert_eq!(alias.name, "Interval");
            assert!(
                alias.decorators.len() >= 2,
                "Expected at least 2 decorators, got {:?}",
                alias.decorators
            );

            // Check @derive is present
            let derive = alias.decorators.iter().find(|d| d.name == "Derive");
            assert!(derive.is_some(), "Expected @derive decorator");

            // Check @default is present
            let default = alias.decorators.iter().find(|d| d.name == "default");
            assert!(
                default.is_some(),
                "Expected @default decorator, got decorators: {:?}",
                alias.decorators
            );
            if let Some(d) = default {
                assert_eq!(d.args_src, "DailyRecurrenceRule.defaultValue()");
            }
        });
    }

    #[cfg(feature = "swc")]
    #[test]
    fn union_variant_with_default_decorator() {
        use crate::abi::ir::type_alias::TypeBody;

        GLOBALS.set(&Globals::new(), || {
            let source = r#"/** @derive(Default) */
export type UnionWithDefault =
  | /** @default */ VariantA
  | VariantB;"#;
            let module = parse_module(source);
            let type_aliases = lower_type_aliases(&module, source).expect("lowering to succeed");
            let alias = type_aliases.first().expect("type alias");

            assert_eq!(alias.name, "UnionWithDefault");

            // Check that we have a union type
            if let TypeBody::Union(members) = &alias.body {
                assert_eq!(members.len(), 2, "Should have 2 union members");

                // First member (VariantA) should have @default decorator
                let first = &members[0];
                eprintln!("First member: {:?}", first);
                eprintln!("First member decorators: {:?}", first.decorators);

                assert!(
                    first.has_decorator("default"),
                    "First variant should have @default. Decorators: {:?}",
                    first.decorators
                );

                // Second member (VariantB) should NOT have @default
                let second = &members[1];
                assert!(
                    !second.has_decorator("default"),
                    "Second variant should NOT have @default"
                );
            } else {
                panic!("Expected Union type body, got {:?}", alias.body);
            }
        });
    }
}
