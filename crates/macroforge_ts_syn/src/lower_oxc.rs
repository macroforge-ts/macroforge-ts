//! AST lowering from Oxc types to IR representations.

use crate::LoweredTarget;
use crate::TsSynError;
use crate::abi::*;
use ::oxc_ast::ast::*;
use oxc_span::GetSpan;
use std::collections::HashSet;

/// Convert a 0-based Oxc span to a 1-based SpanIR (matching the patch applicator convention).
fn oxc_span_ir(span: oxc_span::Span) -> SpanIR {
    SpanIR::new(span.start + 1, span.end + 1)
}

/// Collect JSDoc field-level decorators (e.g. `/** @serde({ rename: "user_id" }) */`)
/// from the source text preceding a given 0-based byte offset.
///
/// This is the Oxc equivalent of the SWC `collect_leading_macro_directives`.
fn collect_leading_decorators_oxc(
    source: &str,
    target_start_0: usize,
    valid_annotations: Option<&HashSet<String>>,
) -> Vec<DecoratorIR> {
    use crate::jsdoc::parse_all_macro_directives;

    if target_start_0 == 0 || target_start_0 > source.len() {
        return Vec::new();
    }

    let search_area = &source[..target_start_0];

    let Some(end_idx) = search_area.rfind("*/") else {
        return Vec::new();
    };

    let Some(start_idx) = search_area[..end_idx].rfind("/**") else {
        return Vec::new();
    };

    let end_of_comment_block = end_idx + 2;

    // Only accept if the comment is directly adjacent (modulo allowed modifiers)
    let between = &search_area[end_of_comment_block..];
    let between_trimmed = between.trim();
    if !between_trimmed.is_empty() {
        let allowed_modifiers = ["export", "declare", "abstract", "default", "async"];
        let remaining: String = between_trimmed
            .split_whitespace()
            .filter(|word| !allowed_modifiers.contains(word))
            .collect::<Vec<_>>()
            .join(" ");
        if !remaining.is_empty() {
            return Vec::new();
        }
    }

    let mut all_directives = Vec::new();
    let mut current_start = start_idx;
    let mut current_end = end_idx;

    loop {
        let comment_body = &search_area[current_start + 3..current_end];

        // Skip macro import comments
        let body_lower = comment_body.to_ascii_lowercase();
        let is_macro_import = body_lower.contains("import") && body_lower.contains("macro");

        let directives = if is_macro_import {
            Vec::new()
        } else {
            parse_all_macro_directives(comment_body, valid_annotations)
        };

        for (name, args_src) in directives {
            // Use 1-based spans for SpanIR (matching the convention)
            all_directives.push(DecoratorIR {
                name,
                args_src,
                span: SpanIR::new(current_start as u32 + 1, (current_end + 2) as u32 + 1),
                #[cfg(feature = "swc")]
                node: None,
            });
        }

        // Check for adjacent preceding comment
        let before_comment = &search_area[..current_start];
        let before_trimmed = before_comment.trim_end();

        if !before_trimmed.ends_with("*/") {
            break;
        }

        let prev_end = before_trimmed.len() - 2;
        let Some(prev_start) = before_trimmed[..prev_end].rfind("/**") else {
            break;
        };

        current_start = prev_start;
        current_end = prev_end;
    }

    all_directives
}

pub fn lower_classes_oxc(
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
            Statement::ExportNamedDeclaration(decl) => {
                if let Some(Declaration::ClassDeclaration(class_decl)) = &decl.declaration
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
    if let Some(super_class) = &decl.super_class
        && let Expression::Identifier(ident) = super_class
    {
        heritage.push(ident.name.to_string());
    }
    if let Some(implements) = &decl.implements {
        for item in implements {
            if let TSTypeName::IdentifierReference(ident) = &item.expression {
                heritage.push(ident.name.to_string());
            }
        }
    }

    let mut decorators = Vec::new();
    // 1. Native decorators
    for dec in &decl.decorators {
        if let Expression::CallExpression(call) = &dec.expression {
            if let Expression::Identifier(ident) = &call.callee {
                decorators.push(DecoratorIR {
                    name: ident.name.to_string(),
                    args_src: source[call.span.start as usize..call.span.end as usize].to_string(),
                    span: oxc_span_ir(dec.span),
                    #[cfg(feature = "swc")]
                    node: None,
                });
            }
        } else if let Expression::Identifier(ident) = &dec.expression {
            decorators.push(DecoratorIR {
                name: ident.name.to_string(),
                args_src: String::new(),
                span: oxc_span_ir(dec.span),
                #[cfg(feature = "swc")]
                node: None,
            });
        }
    }
    // 2. JSDoc class-level decorators (e.g. @serde({ denyUnknownFields: true }))
    decorators.extend(collect_leading_decorators_oxc(
        source,
        decl.span.start as usize,
        filter,
    ));

    let mut fields = Vec::new();
    let mut methods = Vec::new();

    for element in &decl.body.body {
        match element {
            ClassElement::PropertyDefinition(prop) => {
                if let PropertyKey::StaticIdentifier(ident) = &prop.key {
                    let field_decorators =
                        collect_leading_decorators_oxc(source, prop.span.start as usize, filter);
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
                        #[cfg(feature = "swc")]
                        type_ann: None,
                        optional: prop.optional,
                        readonly: prop.readonly,
                        visibility: match prop.accessibility {
                            Some(TSAccessibility::Private) => Visibility::Private,
                            Some(TSAccessibility::Protected) => Visibility::Protected,
                            _ => Visibility::Public,
                        },
                        decorators: field_decorators,
                        #[cfg(feature = "swc")]
                        prop_ast: None,
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

                    let method_decorators =
                        collect_leading_decorators_oxc(source, method.span.start as usize, filter);
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
                        #[cfg(feature = "swc")]
                        member_ast: None,
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
        #[cfg(feature = "swc")]
        decorators_ast: Vec::new(),
        fields,
        methods,
        #[cfg(feature = "swc")]
        members: Vec::new(),
    })
}

pub fn lower_interfaces_oxc(
    program: &Program<'_>,
    source: &str,
    filter: Option<&HashSet<String>>,
) -> Result<Vec<InterfaceIR>, TsSynError> {
    let mut interfaces = Vec::new();
    for stmt in &program.body {
        let decl = match stmt {
            Statement::TSInterfaceDeclaration(d) => Some(d.as_ref()),
            Statement::ExportNamedDeclaration(d) => match &d.declaration {
                Some(Declaration::TSInterfaceDeclaration(d)) => Some(d.as_ref()),
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
    if let Some(extends) = &decl.extends {
        for ext in extends {
            if let Expression::Identifier(ident) = &ext.expression {
                heritage.push(ident.name.to_string());
            }
        }
    }

    let (fields, methods) = lower_interface_members(&decl.body.body, source, filter);

    // JSDoc interface-level decorators
    let decorators = collect_leading_decorators_oxc(source, decl.span.start as usize, filter);

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

fn lower_interface_members(
    body: &[TSSignature<'_>],
    source: &str,
    filter: Option<&HashSet<String>>,
) -> (Vec<InterfaceFieldIR>, Vec<InterfaceMethodIR>) {
    let mut fields = Vec::new();
    let mut methods = Vec::new();

    for elem in body {
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
                        collect_leading_decorators_oxc(source, prop.span.start as usize, filter);
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
                        collect_leading_decorators_oxc(source, meth.span.start as usize, filter);
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

pub fn lower_enums_oxc(
    program: &Program<'_>,
    source: &str,
    _filter: Option<&HashSet<String>>,
) -> Result<Vec<EnumIR>, TsSynError> {
    let mut enums = Vec::new();
    for stmt in &program.body {
        let decl = match stmt {
            Statement::TSEnumDeclaration(d) => Some(d.as_ref()),
            Statement::ExportNamedDeclaration(d) => match &d.declaration {
                Some(Declaration::TSEnumDeclaration(d)) => Some(d.as_ref()),
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

    for member in &decl.members {
        let member_name = match &member.id {
            TSEnumMemberName::Identifier(i) => i.name.to_string(),
            TSEnumMemberName::String(s) => s.value.to_string(),
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
            collect_leading_decorators_oxc(source, member.span.start as usize, None);
        variants.push(EnumVariantIR {
            name: member_name,
            span: oxc_span_ir(member.span),
            value,
            decorators: variant_decorators,
        });
    }

    let decorators = collect_leading_decorators_oxc(source, decl.span.start as usize, None);

    EnumIR {
        name,
        span,
        body_span,
        decorators,
        variants,
        is_const: decl.r#const,
    }
}

pub fn lower_type_aliases_oxc(
    program: &Program<'_>,
    source: &str,
    _filter: Option<&HashSet<String>>,
) -> Result<Vec<TypeAliasIR>, TsSynError> {
    let mut aliases = Vec::new();
    for stmt in &program.body {
        let decl = match stmt {
            Statement::TSTypeAliasDeclaration(d) => Some(d.as_ref()),
            Statement::ExportNamedDeclaration(d) => match &d.declaration {
                Some(Declaration::TSTypeAliasDeclaration(d)) => Some(d.as_ref()),
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

    let body = lower_type_body_oxc(&decl.type_annotation, source);

    let decorators = collect_leading_decorators_oxc(source, decl.span.start as usize, None);

    TypeAliasIR {
        name,
        span,
        decorators,
        type_params,
        body,
    }
}

fn lower_union_member_oxc(t: &TSType<'_>, source: &str) -> TypeMember {
    let sp = t.span();
    let text = source[sp.start as usize..sp.end as usize].to_string();
    let decorators = collect_leading_decorators_oxc(source, sp.start as usize, None);
    let kind = match t {
        TSType::TSLiteralType(_) => TypeMemberKind::Literal(text),
        TSType::TSTypeLiteral(lit) => {
            let (fields, _) = lower_interface_members(&lit.members, source, None);
            TypeMemberKind::Object { fields }
        }
        _ => TypeMemberKind::TypeRef(text),
    };
    TypeMember::with_decorators(kind, decorators)
}

fn lower_type_body_oxc(ts_type: &TSType<'_>, source: &str) -> TypeBody {
    match ts_type {
        TSType::TSUnionType(union) => {
            let members = union
                .types
                .iter()
                .map(|t| lower_union_member_oxc(t, source))
                .collect();
            TypeBody::Union(members)
        }
        TSType::TSIntersectionType(inter) => {
            let members = inter
                .types
                .iter()
                .map(|t| lower_union_member_oxc(t, source))
                .collect();
            TypeBody::Intersection(members)
        }
        TSType::TSTypeLiteral(lit) => {
            let (fields, _methods) = lower_interface_members(&lit.members, source, None);
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

pub fn lower_targets_oxc(
    program: &Program<'_>,
    source: &str,
    filter: Option<&HashSet<String>>,
) -> Result<Vec<LoweredTarget>, TsSynError> {
    let mut targets = Vec::new();
    for class in lower_classes_oxc(program, source, filter)? {
        targets.push(LoweredTarget::Class(class));
    }
    for iface in lower_interfaces_oxc(program, source, filter)? {
        targets.push(LoweredTarget::Interface(iface));
    }
    for e in lower_enums_oxc(program, source, filter)? {
        targets.push(LoweredTarget::Enum(e));
    }
    for ta in lower_type_aliases_oxc(program, source, filter)? {
        targets.push(LoweredTarget::TypeAlias(ta));
    }
    Ok(targets)
}

/// Collect names of exported declarations from a parsed Oxc program.
pub fn collect_exported_names_oxc(program: &Program<'_>) -> HashSet<String> {
    let mut names = HashSet::new();

    for stmt in &program.body {
        match stmt {
            Statement::ExportNamedDeclaration(decl) => {
                if let Some(decl) = &decl.declaration {
                    match decl {
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
                    }
                }
                // Handle export { foo, bar }
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
