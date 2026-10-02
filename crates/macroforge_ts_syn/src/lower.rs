//! AST lowering from Oxc types to IR representations.

use crate::LoweredTarget;
use crate::TsSynError;
use crate::abi::*;
use ::oxc::ast::ast::*;
use oxc::span::GetSpan;
use std::collections::HashSet;

/// Convert a 0-based Oxc span to a 1-based SpanIR (matching the patch applicator convention).
fn oxc_span_ir(span: oxc::span::Span) -> SpanIR {
    SpanIR::new(span.start + 1, span.end + 1)
}

/// Collect JSDoc field-level decorators (e.g. `/** @serde({ rename: "user_id" }) */`)
/// from the source text preceding a given 0-based byte offset. `floor` is
/// where the previous sibling ends: no comment of this declaration's starts
/// before it, so the search stops there instead of scanning the file.
fn collect_leading_decorators(
    source: &str,
    floor: usize,
    target_start_0: usize,
    valid_annotations: Option<&HashSet<String>>,
) -> Vec<DecoratorIR> {
    use crate::jsdoc::{
        adjacent_jsdoc_after, is_macro_import_comment, parse_all_macro_directives,
        stacked_jsdoc_above,
    };

    let mut all_directives = Vec::new();
    let mut block = adjacent_jsdoc_after(source, floor, target_start_0);

    // Walk back over every JSDoc block stacked directly above the target.
    while let Some(current) = block {
        let comment_body = current.body(source);
        if !is_macro_import_comment(comment_body) {
            for (name, args_src) in parse_all_macro_directives(comment_body, valid_annotations) {
                // Use 1-based spans for SpanIR (matching the convention)
                all_directives.push(DecoratorIR {
                    name,
                    args_src,
                    span: SpanIR::new(current.start as u32 + 1, current.end as u32 + 1),
                });
            }
        }
        block = stacked_jsdoc_above(source, floor, current);
    }

    all_directives
}

pub fn lower_classes(
    program: &Program<'_>,
    source: &str,
    filter: Option<&HashSet<String>>,
) -> Result<Vec<ClassIR>, TsSynError> {
    let mut classes = Vec::new();
    for stmt in &program.body {
        match stmt {
            Statement::ClassDeclaration(decl) => {
                if let Some(class_ir) = lower_class(decl, source, filter) {
                    classes.push(class_ir);
                }
            }
            Statement::ExportDeclaration(decl) => {
                if let Declaration::ClassDeclaration(class_decl) = &decl.declaration
                    && let Some(class_ir) = lower_class(class_decl, source, filter)
                {
                    classes.push(class_ir);
                }
            }
            Statement::ExportDefaultDeclaration(decl) => {
                if let ExportDefaultDeclarationKind::ClassDeclaration(class_decl) =
                    &decl.declaration
                    && let Some(class_ir) = lower_class(class_decl, source, filter)
                {
                    classes.push(class_ir);
                }
            }
            _ => {}
        }
    }
    Ok(classes)
}

fn lower_class(
    decl: &Class<'_>,
    source: &str,
    filter: Option<&HashSet<String>>,
) -> Option<ClassIR> {
    let name = decl.id.as_ref()?.name.to_string();
    let span = oxc_span_ir(decl.span);
    let body_span = oxc_span_ir(decl.body.span);

    let type_params = decl
        .type_parameters
        .as_ref()
        .map(|tp| tp.params.iter().map(|p| p.name.to_string()).collect())
        .unwrap_or_default();

    let mut heritage = Vec::new();
    if let Some(super_class) = &decl.heritage
        && let Expression::Identifier(ident) = &super_class.expression
    {
        heritage.push(ident.name.to_string());
    }
    for item in &decl.implements {
        if let TSTypeName::IdentifierReference(ident) = &item.expression {
            heritage.push(ident.name.to_string());
        }
    }

    // JSDoc class-level decorators (e.g. @serde({ denyUnknownFields: true }))
    let decorators = collect_leading_decorators(source, 0, decl.span.start as usize, filter);

    let mut fields = Vec::new();
    let mut methods = Vec::new();

    let mut previous_end = decl.body.span.start as usize + 1;
    for element in &decl.body.body {
        let floor = std::mem::replace(&mut previous_end, element.span().end as usize);
        match element {
            ClassElement::PropertyDefinition(prop) => {
                if let PropertyKey::StaticIdentifier(ident) = &prop.key {
                    let field_decorators =
                        collect_leading_decorators(source, floor, prop.span.start as usize, filter);
                    fields.push(FieldIR {
                        name: ident.name.to_string(),
                        span: oxc_span_ir(prop.span),
                        ts_type: prop
                            .type_annotation
                            .as_ref()
                            .map(|ann| {
                                let sp = ann.type_annotation.span();
                                source[sp.start as usize..sp.end as usize].to_string()
                            })
                            .unwrap_or_else(|| "any".to_string()),
                        optional: prop.optional,
                        readonly: prop.readonly,
                        visibility: match prop.accessibility {
                            Some(TSAccessibility::Private) => Visibility::Private,
                            Some(TSAccessibility::Protected) => Visibility::Protected,
                            _ => Visibility::Public,
                        },
                        decorators: field_decorators,
                    });
                }
            }
            ClassElement::MethodDefinition(method) => {
                if let PropertyKey::StaticIdentifier(ident) = &method.key {
                    let func = &method.value;

                    let type_params_src = func
                        .type_parameters
                        .as_ref()
                        .map(|tp| source[tp.span.start as usize..tp.span.end as usize].to_string())
                        .unwrap_or_default();

                    let params_src = {
                        let sp = func.params.span;
                        let raw = &source[sp.start as usize..sp.end as usize];
                        raw.strip_prefix('(')
                            .and_then(|s| s.strip_suffix(')'))
                            .unwrap_or(raw)
                            .trim()
                            .to_string()
                    };

                    let return_type_src = func
                        .return_type
                        .as_ref()
                        .map(|ann| {
                            let sp = ann.type_annotation.span();
                            source[sp.start as usize..sp.end as usize].to_string()
                        })
                        .unwrap_or_default();

                    let method_decorators = collect_leading_decorators(
                        source,
                        floor,
                        method.span.start as usize,
                        filter,
                    );

                    let (method_body_span, method_body_src) = if let Some(body) = &func.body {
                        let bspan = oxc_span_ir(body.span);
                        let inner_start = body.span.start as usize + 1;
                        let inner_end = body.span.end as usize - 1;
                        let src = source.get(inner_start..inner_end).unwrap_or("").to_string();
                        (Some(bspan), Some(src))
                    } else {
                        (None, None)
                    };

                    methods.push(MethodSigIR {
                        name: ident.name.to_string(),
                        span: oxc_span_ir(method.span),
                        type_params_src,
                        params_src,
                        return_type_src,
                        is_static: method.r#static,
                        is_async: method.value.r#async,
                        visibility: match method.accessibility {
                            Some(TSAccessibility::Private) => Visibility::Private,
                            Some(TSAccessibility::Protected) => Visibility::Protected,
                            _ => Visibility::Public,
                        },
                        decorators: method_decorators,
                        body_span: method_body_span,
                        body_src: method_body_src,
                    });
                }
            }
            _ => {}
        }
    }

    Some(ClassIR {
        name,
        span,
        body_span,
        is_abstract: decl.r#abstract,
        type_params,
        heritage,
        decorators,
        fields,
        methods,
    })
}

pub fn lower_interfaces(
    program: &Program<'_>,
    source: &str,
    filter: Option<&HashSet<String>>,
) -> Result<Vec<InterfaceIR>, TsSynError> {
    let mut interfaces = Vec::new();
    for stmt in &program.body {
        let decl = match stmt {
            Statement::TSInterfaceDeclaration(d) => Some(d.as_ref()),
            Statement::ExportDeclaration(d) => match &d.declaration {
                Declaration::TSInterfaceDeclaration(d) => Some(d.as_ref()),
                _ => None,
            },
            _ => None,
        };
        if let Some(d) = decl {
            interfaces.push(lower_interface(d, source, filter));
        }
    }
    Ok(interfaces)
}

fn lower_interface(
    decl: &TSInterfaceDeclaration<'_>,
    source: &str,
    filter: Option<&HashSet<String>>,
) -> InterfaceIR {
    let name = decl.id.name.to_string();
    let span = oxc_span_ir(decl.span);
    let body_span = oxc_span_ir(decl.body.span);

    let type_params = decl
        .type_parameters
        .as_ref()
        .map(|tp| tp.params.iter().map(|p| p.name.to_string()).collect())
        .unwrap_or_default();

    let mut heritage = Vec::new();
    for ext in &decl.extends {
        if let TSTypeName::IdentifierReference(ident) = &ext.type_name {
            heritage.push(ident.name.to_string());
        }
    }

    let (fields, methods) =
        lower_interface_members(&decl.body.body, decl.body.span.start, source, filter);

    // JSDoc interface-level decorators
    let decorators = collect_leading_decorators(source, 0, decl.span.start as usize, filter);

    InterfaceIR {
        name,
        span,
        body_span,
        type_params,
        heritage,
        decorators,
        fields,
        methods,
    }
}

/// The members of an interface or type literal whose body opens with the `{`
/// at `body_start`.
fn lower_interface_members(
    body: &[TSSignature<'_>],
    body_start: u32,
    source: &str,
    filter: Option<&HashSet<String>>,
) -> (Vec<InterfaceFieldIR>, Vec<InterfaceMethodIR>) {
    let mut fields = Vec::new();
    let mut methods = Vec::new();

    let mut previous_end = body_start as usize + 1;
    for elem in body {
        let floor = std::mem::replace(&mut previous_end, elem.span().end as usize);
        match elem {
            TSSignature::TSPropertySignature(prop) => {
                if let PropertyKey::StaticIdentifier(ident) = &prop.key {
                    let ts_type = prop
                        .type_annotation
                        .as_ref()
                        .map(|ann| {
                            let sp = ann.type_annotation.span();
                            source[sp.start as usize..sp.end as usize].to_string()
                        })
                        .unwrap_or_else(|| "any".to_string());

                    let field_decorators =
                        collect_leading_decorators(source, floor, prop.span.start as usize, filter);
                    fields.push(InterfaceFieldIR {
                        name: ident.name.to_string(),
                        span: oxc_span_ir(prop.span),
                        ts_type,
                        optional: prop.optional,
                        readonly: prop.readonly,
                        decorators: field_decorators,
                    });
                }
            }
            TSSignature::TSMethodSignature(meth) => {
                if let PropertyKey::StaticIdentifier(ident) = &meth.key {
                    let params_src = {
                        let sp = meth.params.span;
                        let raw = &source[sp.start as usize..sp.end as usize];
                        raw.strip_prefix('(')
                            .and_then(|s| s.strip_suffix(')'))
                            .unwrap_or(raw)
                            .trim()
                            .to_string()
                    };

                    let type_params_src = meth
                        .type_parameters
                        .as_ref()
                        .map(|tp| source[tp.span.start as usize..tp.span.end as usize].to_string())
                        .unwrap_or_default();

                    let return_type_src = meth
                        .return_type
                        .as_ref()
                        .map(|ann| {
                            let sp = ann.type_annotation.span();
                            source[sp.start as usize..sp.end as usize].to_string()
                        })
                        .unwrap_or_else(|| "void".to_string());

                    let meth_decorators =
                        collect_leading_decorators(source, floor, meth.span.start as usize, filter);
                    methods.push(InterfaceMethodIR {
                        name: ident.name.to_string(),
                        span: oxc_span_ir(meth.span),
                        type_params_src,
                        params_src,
                        return_type_src,
                        optional: meth.optional,
                        decorators: meth_decorators,
                    });
                }
            }
            _ => {}
        }
    }

    (fields, methods)
}

pub fn lower_enums(
    program: &Program<'_>,
    source: &str,
    _filter: Option<&HashSet<String>>,
) -> Result<Vec<EnumIR>, TsSynError> {
    let mut enums = Vec::new();
    for stmt in &program.body {
        let decl = match stmt {
            Statement::TSEnumDeclaration(d) => Some(d.as_ref()),
            Statement::ExportDeclaration(d) => match &d.declaration {
                Declaration::TSEnumDeclaration(d) => Some(d.as_ref()),
                _ => None,
            },
            _ => None,
        };
        if let Some(d) = decl {
            enums.push(lower_enum(d, source));
        }
    }
    Ok(enums)
}

fn lower_enum(decl: &TSEnumDeclaration<'_>, source: &str) -> EnumIR {
    let name = decl.id.name.to_string();
    let span = oxc_span_ir(decl.span);

    let enum_source = &source[decl.span.start as usize..decl.span.end as usize];
    let body_span =
        if let (Some(open), Some(close)) = (enum_source.find('{'), enum_source.rfind('}')) {
            SpanIR::new(
                decl.span.start + open as u32 + 1,
                decl.span.start + close as u32 + 2,
            )
        } else {
            span
        };

    let mut variants = Vec::new();
    let mut next_auto_value: f64 = 0.0;

    let mut previous_end = body_span.start as usize;
    for member in &decl.body.members {
        let floor = std::mem::replace(&mut previous_end, member.span.end as usize);
        let member_name = match &member.id {
            TSEnumMemberName::Identifier(i) => i.name.to_string(),
            TSEnumMemberName::String(s) | TSEnumMemberName::ComputedString(s) => {
                s.value.to_string()
            }
            TSEnumMemberName::ComputedTemplateString(_) => continue,
        };

        let value = if let Some(init) = &member.initializer {
            match init {
                Expression::StringLiteral(s) => EnumValue::String(s.value.to_string()),
                Expression::NumericLiteral(n) => {
                    next_auto_value = n.value + 1.0;
                    EnumValue::Number(n.value)
                }
                Expression::UnaryExpression(unary)
                    if matches!(
                        unary.operator,
                        UnaryOperator::UnaryNegation | UnaryOperator::UnaryPlus
                    ) =>
                {
                    if let Expression::NumericLiteral(n) = &unary.argument {
                        let val = if matches!(unary.operator, UnaryOperator::UnaryNegation) {
                            -n.value
                        } else {
                            n.value
                        };
                        next_auto_value = val + 1.0;
                        EnumValue::Number(val)
                    } else {
                        let sp = init.span();
                        EnumValue::Expr(source[sp.start as usize..sp.end as usize].to_string())
                    }
                }
                _ => {
                    let sp = init.span();
                    EnumValue::Expr(source[sp.start as usize..sp.end as usize].to_string())
                }
            }
        } else {
            let val = next_auto_value;
            next_auto_value += 1.0;
            EnumValue::Number(val)
        };

        let variant_decorators =
            collect_leading_decorators(source, floor, member.span.start as usize, None);
        variants.push(EnumVariantIR {
            name: member_name,
            span: oxc_span_ir(member.span),
            value,
            decorators: variant_decorators,
        });
    }

    let decorators = collect_leading_decorators(source, 0, decl.span.start as usize, None);

    EnumIR {
        name,
        span,
        body_span,
        decorators,
        variants,
        is_const: decl.r#const,
    }
}

pub fn lower_type_aliases(
    program: &Program<'_>,
    source: &str,
    _filter: Option<&HashSet<String>>,
) -> Result<Vec<TypeAliasIR>, TsSynError> {
    let mut aliases = Vec::new();
    for stmt in &program.body {
        let decl = match stmt {
            Statement::TSTypeAliasDeclaration(d) => Some(d.as_ref()),
            Statement::ExportDeclaration(d) => match &d.declaration {
                Declaration::TSTypeAliasDeclaration(d) => Some(d.as_ref()),
                _ => None,
            },
            _ => None,
        };
        if let Some(d) = decl {
            aliases.push(lower_type_alias(d, source));
        }
    }
    Ok(aliases)
}

fn lower_type_alias(decl: &TSTypeAliasDeclaration<'_>, source: &str) -> TypeAliasIR {
    let name = decl.id.name.to_string();
    let span = oxc_span_ir(decl.span);

    let type_params = decl
        .type_parameters
        .as_ref()
        .map(|tp| tp.params.iter().map(|p| p.name.to_string()).collect())
        .unwrap_or_default();

    let body = lower_type_body(&decl.type_annotation, source);

    let decorators = collect_leading_decorators(source, 0, decl.span.start as usize, None);

    TypeAliasIR {
        name,
        span,
        decorators,
        type_params,
        body,
    }
}

/// Lowers each of a union's or intersection's members, bounding each one's
/// JSDoc search at the member before it.
fn lower_union_members(members: &[TSType<'_>], source: &str) -> Vec<TypeMember> {
    let mut previous_end = 0;
    members
        .iter()
        .map(|member| {
            let floor = std::mem::replace(&mut previous_end, member.span().end as usize);
            lower_union_member(member, floor, source)
        })
        .collect()
}

fn lower_union_member(t: &TSType<'_>, floor: usize, source: &str) -> TypeMember {
    let sp = t.span();
    let text = source[sp.start as usize..sp.end as usize].to_string();
    let decorators = collect_leading_decorators(source, floor, sp.start as usize, None);
    let kind = match t {
        TSType::TSLiteralType(_) => TypeMemberKind::Literal(text),
        TSType::TSTypeLiteral(lit) => {
            let (fields, _) = lower_interface_members(&lit.members, lit.span.start, source, None);
            TypeMemberKind::Object { fields }
        }
        TSType::TSIntersectionType(inter) => {
            TypeMemberKind::Intersection(lower_union_members(&inter.types, source))
        }
        TSType::TSParenthesizedType(paren) => {
            let mut inner = lower_union_member(
                &paren.type_annotation,
                paren.span.start as usize + 1,
                source,
            );
            if inner.decorators.is_empty() && !decorators.is_empty() {
                inner.decorators = decorators;
            }
            return inner;
        }
        _ => TypeMemberKind::TypeRef(text),
    };
    TypeMember::with_decorators(kind, decorators)
}

fn lower_type_body(ts_type: &TSType<'_>, source: &str) -> TypeBody {
    match ts_type {
        TSType::TSUnionType(union) => TypeBody::Union(lower_union_members(&union.types, source)),
        TSType::TSIntersectionType(inter) => {
            TypeBody::Intersection(lower_union_members(&inter.types, source))
        }
        TSType::TSTypeLiteral(lit) => {
            let (fields, _methods) =
                lower_interface_members(&lit.members, lit.span.start, source, None);
            TypeBody::Object { fields }
        }
        TSType::TSTupleType(tuple) => {
            let elements = tuple
                .element_types
                .iter()
                .map(|elem| {
                    let sp = elem.span();
                    source[sp.start as usize..sp.end as usize].to_string()
                })
                .collect();
            TypeBody::Tuple(elements)
        }
        _ => {
            let sp = ts_type.span();
            TypeBody::Other(source[sp.start as usize..sp.end as usize].to_string())
        }
    }
}

pub fn lower_functions(
    program: &Program<'_>,
    source: &str,
    _filter: Option<&HashSet<String>>,
) -> Result<Vec<FunctionIR>, TsSynError> {
    let mut functions = Vec::new();
    for stmt in &program.body {
        match stmt {
            Statement::FunctionDeclaration(decl) => {
                if let Some(func_ir) = lower_function(decl, source, false, false, None) {
                    functions.push(func_ir);
                }
            }
            Statement::ExportDeclaration(decl) => {
                if let Declaration::FunctionDeclaration(func_decl) = &decl.declaration
                    && let Some(func_ir) =
                        lower_function(func_decl, source, true, false, Some(decl.span))
                {
                    functions.push(func_ir);
                }
            }
            Statement::ExportDefaultDeclaration(decl) => {
                if let ExportDefaultDeclarationKind::FunctionDeclaration(func_decl) =
                    &decl.declaration
                    && let Some(func_ir) =
                        lower_function(func_decl, source, true, true, Some(decl.span))
                {
                    functions.push(func_ir);
                }
            }
            _ => {}
        }
    }
    Ok(functions)
}

fn lower_function(
    decl: &Function<'_>,
    source: &str,
    is_exported: bool,
    is_default_export: bool,
    export_span: Option<oxc::span::Span>,
) -> Option<FunctionIR> {
    let name = decl.id.as_ref()?.name.to_string();

    let body = decl.body.as_ref()?;

    // Use the export statement span if present, otherwise the function's own span.
    let outer_span = export_span.unwrap_or(decl.span);
    let span = oxc_span_ir(outer_span);
    let body_span = oxc_span_ir(body.span);

    // Signature span: from the start of the outer span to just before the body's `{`.
    let sig_end = body.span.start;
    let signature_span = SpanIR::new(outer_span.start + 1, sig_end + 1);

    let type_params = decl
        .type_parameters
        .as_ref()
        .map(|tp| tp.params.iter().map(|p| p.name.to_string()).collect())
        .unwrap_or_default();

    let mut params = Vec::new();
    for param in &decl.params.items {
        let p_span = param.span;
        let p_src = &source[p_span.start as usize..p_span.end as usize];

        let is_rest = p_src.starts_with("...");

        let (p_name, type_src, default_src, is_optional) = match &param.pattern {
            BindingPattern::BindingIdentifier(ident) => {
                let ts = param
                    .type_annotation
                    .as_ref()
                    .map(|ann| {
                        let sp = ann.type_annotation.span();
                        source[sp.start as usize..sp.end as usize].to_string()
                    })
                    .unwrap_or_default();
                (ident.name.to_string(), ts, None, param.optional)
            }
            BindingPattern::AssignmentPattern(assign) => {
                let id_name = match &assign.left {
                    BindingPattern::BindingIdentifier(ident) => ident.name.to_string(),
                    _ => p_src.to_string(),
                };
                let ts = param
                    .type_annotation
                    .as_ref()
                    .map(|ann| {
                        let sp = ann.type_annotation.span();
                        source[sp.start as usize..sp.end as usize].to_string()
                    })
                    .unwrap_or_default();
                let default = {
                    let sp = assign.right.span();
                    source[sp.start as usize..sp.end as usize].to_string()
                };
                (id_name, ts, Some(default), false)
            }
            _ => (p_src.to_string(), String::new(), None, false),
        };

        params.push(FunctionParamIR {
            name: p_name,
            span: oxc_span_ir(p_span),
            type_src,
            default_src,
            is_optional,
            is_rest,
            decorators: vec![],
        });
    }

    let return_type_src = decl
        .return_type
        .as_ref()
        .map(|ann| {
            let sp = ann.type_annotation.span();
            source[sp.start as usize..sp.end as usize].to_string()
        })
        .unwrap_or_default();

    // Body source: between the braces (exclusive).
    let inner_start = body.span.start as usize + 1;
    let inner_end = body.span.end as usize - 1;
    let body_src = source.get(inner_start..inner_end).unwrap_or("").to_string();

    // Collect leading JSDoc decorators, unfiltered: every decorator is kept.
    let decorator_start = export_span
        .map(|s| s.start as usize)
        .unwrap_or(decl.span.start as usize);
    let decorators = collect_leading_decorators(source, 0, decorator_start, None);

    Some(FunctionIR {
        name,
        span,
        body_span,
        signature_span,
        is_async: decl.r#async,
        is_generator: decl.generator,
        is_exported,
        is_default_export,
        type_params,
        params,
        return_type_src,
        body_src,
        decorators,
    })
}

pub fn lower_targets(
    program: &Program<'_>,
    source: &str,
    filter: Option<&HashSet<String>>,
) -> Result<Vec<LoweredTarget>, TsSynError> {
    let mut targets = Vec::new();
    for class in lower_classes(program, source, filter)? {
        targets.push(LoweredTarget::Class(class));
    }
    for iface in lower_interfaces(program, source, filter)? {
        targets.push(LoweredTarget::Interface(iface));
    }
    for e in lower_enums(program, source, filter)? {
        targets.push(LoweredTarget::Enum(e));
    }
    for ta in lower_type_aliases(program, source, filter)? {
        targets.push(LoweredTarget::TypeAlias(ta));
    }
    for func in lower_functions(program, source, filter)? {
        targets.push(LoweredTarget::Function(func));
    }
    Ok(targets)
}

/// Collect names of exported declarations from a parsed Oxc program.
pub fn collect_exported_names(program: &Program<'_>) -> HashSet<String> {
    let mut names = HashSet::new();

    for stmt in &program.body {
        match stmt {
            Statement::ExportDeclaration(decl) => match &decl.declaration {
                Declaration::ClassDeclaration(c) => {
                    if let Some(id) = &c.id {
                        names.insert(id.name.to_string());
                    }
                }
                Declaration::TSInterfaceDeclaration(i) => {
                    names.insert(i.id.name.to_string());
                }
                Declaration::TSEnumDeclaration(e) => {
                    names.insert(e.id.name.to_string());
                }
                Declaration::TSTypeAliasDeclaration(t) => {
                    names.insert(t.id.name.to_string());
                }
                _ => {}
            },
            Statement::ExportNamedDeclaration(decl) => {
                // Handle export { foo, bar }
                for spec in &decl.specifiers {
                    names.insert(spec.local.name().to_string());
                }
            }
            Statement::ExportFromDeclaration(decl) => {
                for spec in &decl.specifiers {
                    names.insert(spec.local.name().to_string());
                }
            }
            Statement::ExportDefaultDeclaration(decl) => {
                if let ExportDefaultDeclarationKind::ClassDeclaration(c) = &decl.declaration
                    && let Some(id) = &c.id
                {
                    names.insert(id.name.to_string());
                }
            }
            _ => {}
        }
    }

    names
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc::allocator::Allocator;
    use oxc::parser::Parser;
    use oxc::span::SourceType;

    fn parse<'a>(allocator: &'a Allocator, source: &'a str) -> Program<'a> {
        let parsed = Parser::new(allocator, source, SourceType::ts()).parse();
        assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
        parsed.program
    }

    /// The source text a 1-based `SpanIR` covers.
    fn text_at(source: &str, span: SpanIR) -> &str {
        &source[span.start as usize - 1..span.end as usize - 1]
    }

    fn single_class(source: &str) -> ClassIR {
        let allocator = Allocator::default();
        let classes =
            lower_classes(&parse(&allocator, source), source, None).expect("classes should lower");
        classes.into_iter().next().expect("source declares a class")
    }

    fn single_interface(source: &str) -> InterfaceIR {
        let allocator = Allocator::default();
        let interfaces = lower_interfaces(&parse(&allocator, source), source, None)
            .expect("interfaces should lower");
        interfaces
            .into_iter()
            .next()
            .expect("source declares an interface")
    }

    fn single_type_alias(source: &str) -> TypeAliasIR {
        let allocator = Allocator::default();
        let aliases = lower_type_aliases(&parse(&allocator, source), source, None)
            .expect("type aliases should lower");
        aliases
            .into_iter()
            .next()
            .expect("source declares a type alias")
    }

    #[test]
    fn regular_method_params() {
        let class = single_class(
            "class User { getName(prefix: string, suffix: string): string { return ''; } }",
        );
        let method = class
            .methods
            .iter()
            .find(|method| method.name == "getName")
            .expect("getName method");

        assert!(method.params_src.contains("prefix: string"));
        assert!(method.params_src.contains("suffix: string"));
    }

    #[test]
    fn method_span_starts_at_modifier() {
        let source =
            "class User { public static async getUser(): Promise<User> { return null!; } }";
        let class = single_class(source);
        let method = class.methods.first().expect("method");

        assert!(
            text_at(source, method.span).starts_with("public static async getUser"),
            "method span should start at its first modifier, got {:?}",
            text_at(source, method.span)
        );
    }

    #[test]
    fn class_decorator_span_covers_the_directive() {
        let source = r#"
            /** @derive(Debug) */
            class User {}
            "#;
        let class = single_class(source);
        let decorator = class.decorators.first().expect("decorator");

        assert_eq!(decorator.name, "Derive");
        assert!(
            text_at(source, decorator.span).contains("@derive"),
            "decorator span should include '@derive', got {:?}",
            text_at(source, decorator.span)
        );
    }

    #[test]
    fn import_macro_comment_not_parsed_as_decorator() {
        // The `@playground/macro` inside an import-macro comment is a module
        // path, not a `@playground` directive.
        let iface = single_interface(
            r#"/** import macro {Gigaform} from "@playground/macro"; */
/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface PhoneNumber {
    label: string;
    number: string;
}"#,
        );

        assert_eq!(
            iface.decorators.len(),
            1,
            "Expected exactly 1 decorator (Derive), got {:?}",
            iface.decorators
        );
        assert_eq!(iface.decorators[0].name, "Derive");
        assert!(iface.decorators[0].args_src.contains("Gigaform"));
    }

    #[test]
    fn type_alias_with_multiple_decorators() {
        let alias = single_type_alias(
            r#"/**
 * @derive(Default, Deserialize)
 * @default(DailyRecurrenceRule.defaultValue())
 */
export type Interval = DailyRecurrenceRule | WeeklyRecurrenceRule;"#,
        );

        assert_eq!(alias.name, "Interval");
        assert!(
            alias
                .decorators
                .iter()
                .any(|decorator| decorator.name == "Derive"),
            "Expected @derive decorator, got {:?}",
            alias.decorators
        );
        let default = alias
            .decorators
            .iter()
            .find(|decorator| decorator.name == "default")
            .unwrap_or_else(|| panic!("Expected @default decorator, got {:?}", alias.decorators));
        assert_eq!(default.args_src, "DailyRecurrenceRule.defaultValue()");
    }

    #[test]
    fn union_variant_with_default_decorator() {
        let alias = single_type_alias(
            r#"/** @derive(Default) */
export type UnionWithDefault =
  | /** @default */ VariantA
  | VariantB;"#,
        );

        assert_eq!(alias.name, "UnionWithDefault");
        let TypeBody::Union(members) = &alias.body else {
            panic!("Expected Union type body, got {:?}", alias.body);
        };
        assert_eq!(members.len(), 2, "Should have 2 union members");
        assert!(
            members[0].has_decorator("default"),
            "First variant should have @default. Decorators: {:?}",
            members[0].decorators
        );
        assert!(
            !members[1].has_decorator("default"),
            "Second variant should NOT have @default"
        );
    }

    #[test]
    fn interface_field_jsdoc_decorators() {
        let iface = single_interface(
            r#"
interface UserProfile {
    /** @serde(email) */
    email: string;

    /** @serde(minLength(2), maxLength(50)) */
    username: string;
}
"#,
        );

        assert_eq!(iface.name, "UserProfile");
        assert_eq!(iface.fields.len(), 2, "Should have 2 fields");
        for name in ["email", "username"] {
            let field = iface
                .fields
                .iter()
                .find(|field| field.name == name)
                .unwrap_or_else(|| panic!("{name} field"));
            assert!(
                field
                    .decorators
                    .iter()
                    .any(|decorator| decorator.name == "serde"),
                "{name} should have @serde decorator. Got: {:?}",
                field.decorators
            );
        }
    }

    #[test]
    fn type_alias_object_field_jsdoc_decorators() {
        let alias = single_type_alias(
            r#"
type ContactInfo = {
    /** @serde(email) */
    primaryEmail: string;

    /** @serde(minLength(1)) */
    address: string;
};
"#,
        );

        assert_eq!(alias.name, "ContactInfo");
        let TypeBody::Object { fields } = &alias.body else {
            panic!("Expected Object type body, got {:?}", alias.body);
        };
        assert_eq!(fields.len(), 2, "Should have 2 fields");
        for name in ["primaryEmail", "address"] {
            let field = fields
                .iter()
                .find(|field| field.name == name)
                .unwrap_or_else(|| panic!("{name} field"));
            assert!(
                field
                    .decorators
                    .iter()
                    .any(|decorator| decorator.name == "serde"),
                "{name} should have @serde decorator. Got: {:?}",
                field.decorators
            );
        }
    }
}
