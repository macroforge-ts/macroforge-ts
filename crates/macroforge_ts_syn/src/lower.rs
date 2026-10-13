//! AST lowering from Oxc types to IR representations.

use crate::LoweredTarget;
use crate::TsSynError;
use crate::abi::ir::{ModuleReExport, TypeParamIR};
use crate::abi::{
    ClassIR, DecoratorIR, EnumIR, EnumValue, EnumVariantIR, FieldIR, FunctionIR, FunctionParamIR,
    InterfaceFieldIR, InterfaceIR, InterfaceMethodIR, MemberKey, MethodSigIR, NEWTYPE_BRAND_PREFIX,
    NEWTYPE_MACRO, SpanIR, TypeAliasIR, TypeBody, TypeMember, TypeMemberKind, Visibility,
};
use ::oxc::ast::ast::{
    BindingPattern, Class, ClassElement, Declaration, ExportDefaultDeclarationKind, Expression,
    FormalParameter, Function, MethodDefinitionKind, Program, PropertyKey, Statement,
    TSAccessibility, TSEnumDeclaration, TSEnumMemberName, TSInterfaceDeclaration,
    TSMethodSignatureKind, TSSignature, TSType, TSTypeAliasDeclaration, TSTypeAnnotation,
    TSTypeLiteral, TSTypeName, TSTypeOperatorOperator, TSTypeParameterDeclaration, UnaryOperator,
};
use oxc::span::GetSpan;
use std::collections::HashSet;

/// Convert a 0-based Oxc span to a 1-based SpanIR (matching the patch applicator convention).
fn oxc_span_ir(span: oxc::span::Span) -> SpanIR {
    SpanIR::new(span.start + 1, span.end + 1)
}

/// Collect JSDoc field-level decorators (e.g. `/** @endec({ rename: "user_id" }) */`)
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

/// The name a property key gives its property, and how the key is written:
/// an identifier's own name, the value of a string or whole-number literal,
/// as `'first-name'` and `0` are written, or a `#private` name with its `#`.
///
/// A key that is worked out as the code runs gives no name to read off, so
/// its member is carried under the key's source in brackets, `[KEY]`. A macro
/// that cannot reach such a member has to say so rather than never see it.
fn property_name(key: &PropertyKey<'_>, computed: bool, source: &str) -> (String, MemberKey) {
    let named = |name: String| (name, MemberKey::Named);
    match key {
        _ if computed => {}
        PropertyKey::StaticIdentifier(ident) => return named(ident.name.to_string()),
        PropertyKey::PrivateIdentifier(private) => {
            return (format!("#{}", private.name), MemberKey::Private);
        }
        PropertyKey::StringLiteral(literal) => return named(literal.value.to_string()),
        PropertyKey::NumericLiteral(literal) => {
            // The largest whole number a property name spells without an exponent.
            const LARGEST_PLAIN: f64 = 1e21;
            let value = literal.value;
            if value.fract() == 0.0 && (0.0..LARGEST_PLAIN).contains(&value) {
                return named(format!("{value:.0}"));
            }
        }
        _ => {}
    }
    let span = key.span();
    (
        format!("[{}]", &source[span.start as usize..span.end as usize]),
        MemberKey::Unnamed,
    )
}

/// The name an index signature is carried under: its bracketed parameter, as
/// `[key: string]`.
fn index_signature_name(span: oxc::span::Span, source: &str) -> String {
    let text = &source[span.start as usize..span.end as usize];
    match (text.find('['), text.find(']')) {
        (Some(open), Some(close)) if open < close => text[open..=close].to_string(),
        _ => format!("[{text}]"),
    }
}

/// The type an annotation gives, as written, or `any` without one.
fn annotated_type(
    annotation: Option<&oxc::ast::ast::TSTypeAnnotation<'_>>,
    source: &str,
) -> String {
    annotation.map_or_else(
        || "any".to_string(),
        |annotation| {
            let span = annotation.type_annotation.span();
            source[span.start as usize..span.end as usize].to_string()
        },
    )
}

fn source_text(span: oxc::span::Span, source: &str) -> String {
    source[span.start as usize..span.end as usize].to_string()
}

fn visibility(accessibility: Option<TSAccessibility>) -> Visibility {
    match accessibility {
        Some(TSAccessibility::Private) => Visibility::Private,
        Some(TSAccessibility::Protected) => Visibility::Protected,
        Some(TSAccessibility::Public) | None => Visibility::Public,
    }
}

/// A field's type, and whether its declaration leaves the type unsaid: what
/// its annotation gives, else what its initializer makes plain, else `any`.
fn declared_type(
    annotation: Option<&TSTypeAnnotation<'_>>,
    initializer: Option<&Expression<'_>>,
    readonly: bool,
    source: &str,
) -> (String, bool) {
    if annotation.is_some() {
        return (annotated_type(annotation, source), false);
    }
    initializer
        .and_then(|value| initializer_type(value, readonly, source))
        .map_or_else(|| ("any".to_string(), true), |ts_type| (ts_type, false))
}

/// Constructors of the standard library whose instances have one type,
/// whatever they are given.
const PLAIN_CONSTRUCTORS: &[&str] = &["Date", "Error", "RegExp", "URL", "URLSearchParams"];

/// The type TypeScript gives a field for the initializer `value`, where the
/// initializer alone says it: a literal, an assertion, or a `new` whose type
/// takes no inference. A `readonly` field keeps a literal's own type, and
/// any other field the widened one. `source` is the text `value` was parsed
/// from.
pub fn initializer_type(value: &Expression<'_>, readonly: bool, source: &str) -> Option<String> {
    let value = value.without_parentheses();
    let literal = |widened: &str| {
        Some(if readonly {
            source_text(value.span(), source)
        } else {
            widened.to_string()
        })
    };
    match value {
        Expression::NumericLiteral(_) => literal("number"),
        Expression::StringLiteral(_) => literal("string"),
        Expression::BooleanLiteral(_) => literal("boolean"),
        Expression::BigIntLiteral(_) => literal("bigint"),
        Expression::UnaryExpression(negated)
            if negated.operator == UnaryOperator::UnaryNegation =>
        {
            match &negated.argument {
                Expression::NumericLiteral(_) => literal("number"),
                Expression::BigIntLiteral(_) => literal("bigint"),
                _ => None,
            }
        }
        // A template with no holes is a literal, whose own type a `readonly`
        // field keeps.
        Expression::TemplateLiteral(template) if !readonly || !template.expressions.is_empty() => {
            Some("string".to_string())
        }
        Expression::RegExpLiteral(_) => Some("RegExp".to_string()),
        Expression::TSAsExpression(asserted) => asserted_type(&asserted.type_annotation, source),
        Expression::TSTypeAssertion(asserted) => asserted_type(&asserted.type_annotation, source),
        Expression::TSSatisfiesExpression(checked) => {
            initializer_type(&checked.expression, readonly, source)
        }
        Expression::NewExpression(created) => {
            let Expression::Identifier(constructor) = &created.callee else {
                return None;
            };
            match &created.type_arguments {
                Some(arguments) => Some(format!(
                    "{}{}",
                    constructor.name,
                    source_text(arguments.span, source)
                )),
                None => PLAIN_CONSTRUCTORS
                    .contains(&constructor.name.as_str())
                    .then(|| constructor.name.to_string()),
            }
        }
        _ => None,
    }
}

/// The type an assertion gives, unless it is `as const`, whose type is read
/// off the value.
fn asserted_type(asserted: &TSType<'_>, source: &str) -> Option<String> {
    let ts_type = source_text(asserted.span(), source);
    (ts_type != "const").then_some(ts_type)
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
        .map(|tp| lower_type_params(tp, source))
        .unwrap_or_default();

    let mut heritage = Vec::new();
    let superclass = match &decl.heritage {
        Some(super_class) => match &super_class.expression {
            Expression::Identifier(ident) => Some(ident.name.to_string()),
            _ => None,
        },
        None => None,
    };
    heritage.extend(superclass.clone());
    for item in &decl.implements {
        if let TSTypeName::IdentifierReference(ident) = &item.expression {
            heritage.push(ident.name.to_string());
        }
    }

    // JSDoc class-level decorators (e.g. @endec({ denyUnknownFields: true }))
    let decorators = collect_leading_decorators(source, 0, decl.span.start as usize, filter);

    let mut fields = Vec::new();
    let mut methods = Vec::new();

    let mut previous_end = decl.body.span.start as usize + 1;
    for element in &decl.body.body {
        let floor = std::mem::replace(&mut previous_end, element.span().end as usize);
        match element {
            // A static property belongs to the class, not to its instances.
            ClassElement::PropertyDefinition(prop) if !prop.r#static => {
                let (name, key) = property_name(&prop.key, prop.computed, source);
                let (ts_type, untyped) = declared_type(
                    prop.type_annotation.as_deref(),
                    prop.value.as_ref(),
                    prop.readonly,
                    source,
                );
                fields.push(FieldIR {
                    name,
                    key,
                    span: oxc_span_ir(prop.span),
                    ts_type,
                    optional: prop.optional,
                    readonly: prop.readonly,
                    visibility: visibility(prop.accessibility),
                    decorators: collect_leading_decorators(
                        source,
                        floor,
                        prop.span.start as usize,
                        filter,
                    ),
                    initializer: prop
                        .value
                        .as_ref()
                        .map(|value| source_text(value.span(), source)),
                    accessor: false,
                    untyped,
                });
            }
            // An `accessor` is a field too, kept behind a getter and a setter.
            ClassElement::AccessorProperty(prop) if !prop.r#static => {
                let (name, key) = property_name(&prop.key, prop.computed, source);
                let (ts_type, untyped) = declared_type(
                    prop.type_annotation.as_deref(),
                    prop.value.as_ref(),
                    false,
                    source,
                );
                fields.push(FieldIR {
                    name,
                    key,
                    span: oxc_span_ir(prop.span),
                    ts_type,
                    optional: false,
                    readonly: false,
                    visibility: visibility(prop.accessibility),
                    decorators: collect_leading_decorators(
                        source,
                        floor,
                        prop.span.start as usize,
                        filter,
                    ),
                    initializer: prop
                        .value
                        .as_ref()
                        .map(|value| source_text(value.span(), source)),
                    accessor: true,
                    untyped,
                });
            }
            ClassElement::TSIndexSignature(signature) if !signature.r#static => {
                fields.push(FieldIR {
                    name: index_signature_name(signature.span, source),
                    key: MemberKey::Unnamed,
                    span: oxc_span_ir(signature.span),
                    ts_type: annotated_type(Some(&*signature.type_annotation), source),
                    optional: false,
                    readonly: signature.readonly,
                    visibility: Visibility::Public,
                    decorators: collect_leading_decorators(
                        source,
                        floor,
                        signature.span.start as usize,
                        filter,
                    ),
                    initializer: None,
                    accessor: false,
                    untyped: false,
                });
            }
            ClassElement::MethodDefinition(method) => {
                // A constructor parameter with a modifier declares a field.
                if method.kind == MethodDefinitionKind::Constructor {
                    fields.extend(
                        method
                            .value
                            .params
                            .items
                            .iter()
                            .filter_map(|param| parameter_property(param, source)),
                    );
                }
                if let PropertyKey::StaticIdentifier(ident) = &method.key {
                    let func = &method.value;

                    let type_params = func
                        .type_parameters
                        .as_ref()
                        .map(|tp| lower_type_params(tp, source))
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
                        type_params,
                        params_src,
                        return_type_src,
                        is_static: method.r#static,
                        is_async: method.value.r#async,
                        visibility: visibility(method.accessibility),
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
        is_declare: decl.declare,
        type_params,
        heritage,
        superclass,
        decorators,
        fields,
        methods,
    })
}

/// The field a constructor parameter declares, when it carries the modifier
/// that makes it one: `public`, `protected`, `private` or `readonly`. Its
/// default value is the field's initializer.
fn parameter_property(param: &FormalParameter<'_>, source: &str) -> Option<FieldIR> {
    if param.accessibility.is_none() && !param.readonly {
        return None;
    }
    let BindingPattern::BindingIdentifier(ident) = &param.pattern else {
        return None;
    };
    let (ts_type, untyped) = declared_type(
        param.type_annotation.as_deref(),
        param.initializer.as_deref(),
        param.readonly,
        source,
    );
    Some(FieldIR {
        name: ident.name.to_string(),
        key: MemberKey::Named,
        span: oxc_span_ir(param.span),
        ts_type,
        optional: param.optional,
        readonly: param.readonly,
        visibility: visibility(param.accessibility),
        decorators: Vec::new(),
        initializer: param
            .initializer
            .as_ref()
            .map(|value| source_text(value.span(), source)),
        accessor: false,
        untyped,
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
            Statement::ExportDefaultDeclaration(d) => match &d.declaration {
                ExportDefaultDeclarationKind::TSInterfaceDeclaration(d) => Some(d.as_ref()),
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
        .map(|tp| lower_type_params(tp, source))
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

/// The members of an object type written in place. A method it declares is
/// a member that holds a function, since the type has no list of methods
/// apart from its fields.
fn object_type_fields(literal: &TSTypeLiteral<'_>, source: &str) -> Vec<InterfaceFieldIR> {
    let (mut fields, methods) =
        lower_interface_members(&literal.members, literal.span.start, source, None);
    fields.extend(methods.iter().map(InterfaceMethodIR::as_field));
    fields
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
    // The properties declared by a `get` or a `set` signature, by name.
    let accessors = |kind: TSMethodSignatureKind| -> HashSet<String> {
        body.iter()
            .filter_map(|member| match member {
                TSSignature::TSMethodSignature(accessor) if accessor.kind == kind => {
                    Some(property_name(&accessor.key, accessor.computed, source).0)
                }
                _ => None,
            })
            .collect()
    };
    let (getters, setters) = (
        accessors(TSMethodSignatureKind::Get),
        accessors(TSMethodSignatureKind::Set),
    );

    let mut previous_end = body_start as usize + 1;
    for elem in body {
        let floor = std::mem::replace(&mut previous_end, elem.span().end as usize);
        match elem {
            // `get name(): T` declares a property of type `T`, which nothing
            // sets without a `set` beside it. A lone `set name(value: T)`
            // declares one of its parameter's type.
            TSSignature::TSMethodSignature(accessor)
                if accessor.kind != TSMethodSignatureKind::Method =>
            {
                let (name, key) = property_name(&accessor.key, accessor.computed, source);
                let is_getter = accessor.kind == TSMethodSignatureKind::Get;
                if !is_getter && getters.contains(&name) {
                    continue;
                }
                let annotation = if is_getter {
                    accessor.return_type.as_deref()
                } else {
                    accessor
                        .params
                        .items
                        .first()
                        .and_then(|param| param.type_annotation.as_deref())
                };
                fields.push(InterfaceFieldIR {
                    readonly: is_getter && !setters.contains(&name),
                    name,
                    key,
                    span: oxc_span_ir(accessor.span),
                    ts_type: annotated_type(annotation, source),
                    optional: false,
                    decorators: collect_leading_decorators(
                        source,
                        floor,
                        accessor.span.start as usize,
                        filter,
                    ),
                });
            }
            TSSignature::TSPropertySignature(prop) => {
                let (name, key) = property_name(&prop.key, prop.computed, source);
                fields.push(InterfaceFieldIR {
                    name,
                    key,
                    span: oxc_span_ir(prop.span),
                    ts_type: annotated_type(prop.type_annotation.as_deref(), source),
                    optional: prop.optional,
                    readonly: prop.readonly,
                    decorators: collect_leading_decorators(
                        source,
                        floor,
                        prop.span.start as usize,
                        filter,
                    ),
                });
            }
            TSSignature::TSIndexSignature(signature) => {
                fields.push(InterfaceFieldIR {
                    name: index_signature_name(signature.span, source),
                    key: MemberKey::Unnamed,
                    span: oxc_span_ir(signature.span),
                    ts_type: annotated_type(Some(&*signature.type_annotation), source),
                    optional: false,
                    readonly: signature.readonly,
                    decorators: collect_leading_decorators(
                        source,
                        floor,
                        signature.span.start as usize,
                        filter,
                    ),
                });
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

                    let type_params = meth
                        .type_parameters
                        .as_ref()
                        .map(|tp| lower_type_params(tp, source))
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
                        type_params,
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
        is_declare: decl.declare,
    }
}

pub fn lower_type_aliases(
    program: &Program<'_>,
    source: &str,
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
        .map(|tp| lower_type_params(tp, source))
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
        TSType::TSTypeLiteral(lit) => match brand_symbols(lit) {
            Some(symbols) => TypeMemberKind::Brand(symbols),
            None => TypeMemberKind::Object {
                fields: object_type_fields(lit, source),
            },
        },
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

/// Symbol identifiers of a type literal made only of computed `[X]` keys.
fn brand_symbols(lit: &TSTypeLiteral<'_>) -> Option<Vec<String>> {
    if lit.members.is_empty() {
        return None;
    }
    lit.members
        .iter()
        .map(|member| match member {
            TSSignature::TSPropertySignature(prop) if prop.computed => match &prop.key {
                PropertyKey::Identifier(ident) => Some(ident.name.to_string()),
                _ => None,
            },
            _ => None,
        })
        .collect()
}

fn lower_type_body(ts_type: &TSType<'_>, source: &str) -> TypeBody {
    if let Some((branded, depth)) = newtype_branded_type(ts_type) {
        let mut body = lower_type_body(branded, source);
        for _ in 0..depth {
            body = TypeBody::Newtype(Box::new(body));
        }
        return body;
    }
    match ts_type {
        TSType::TSUnionType(union) => TypeBody::Union(lower_union_members(&union.types, source)),
        TSType::TSIntersectionType(inter) => {
            TypeBody::Intersection(lower_union_members(&inter.types, source))
        }
        TSType::TSTypeLiteral(lit) => TypeBody::Object {
            fields: object_type_fields(lit, source),
        },
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
        // A reference to another type, by name or as an array or keyword.
        TSType::TSTypeReference(_)
        | TSType::TSArrayType(_)
        | TSType::TSLiteralType(_)
        | TSType::TSAnyKeyword(_)
        | TSType::TSBigIntKeyword(_)
        | TSType::TSBooleanKeyword(_)
        | TSType::TSNeverKeyword(_)
        | TSType::TSNullKeyword(_)
        | TSType::TSNumberKeyword(_)
        | TSType::TSObjectKeyword(_)
        | TSType::TSStringKeyword(_)
        | TSType::TSSymbolKeyword(_)
        | TSType::TSUndefinedKeyword(_)
        | TSType::TSUnknownKeyword(_)
        | TSType::TSVoidKeyword(_) => TypeBody::Alias(type_source(ts_type, source)),
        TSType::TSTypeOperatorType(operator)
            if operator.operator == TSTypeOperatorOperator::Readonly
                && matches!(operator.type_annotation, TSType::TSArrayType(_)) =>
        {
            TypeBody::Alias(type_source(ts_type, source))
        }
        _ => TypeBody::Other(type_source(ts_type, source)),
    }
}

/// The type a `$Newtype` brands and how many brands it carries, from the
/// macro call (`$Newtype<T>`) or from its expansion
/// (`T & { readonly [__mf_newtype_N]: true }`). A nested newtype expands to one
/// flat intersection with a brand per level.
fn newtype_branded_type<'a, 'b>(ts_type: &'b TSType<'a>) -> Option<(&'b TSType<'a>, usize)> {
    match ts_type {
        TSType::TSTypeReference(reference) => {
            let TSTypeName::IdentifierReference(ident) = &reference.type_name else {
                return None;
            };
            if ident.name != NEWTYPE_MACRO {
                return None;
            }
            match reference.type_arguments.as_deref()?.params.as_slice() {
                [branded] => Some((branded, 1)),
                _ => None,
            }
        }
        TSType::TSIntersectionType(inter) => {
            let (brands, rest): (Vec<_>, Vec<_>) = inter
                .types
                .iter()
                .partition(|member| is_newtype_brand(member));
            match (brands.len(), rest.as_slice()) {
                (0, _) => None,
                (depth, [branded]) => Some((unparenthesized(branded), depth)),
                _ => None,
            }
        }
        TSType::TSParenthesizedType(paren) => newtype_branded_type(&paren.type_annotation),
        _ => None,
    }
}

/// Whether `ts_type` is the brand a `$Newtype` expansion adds: a type literal
/// whose only keys are its `__mf_newtype_N` symbols.
fn is_newtype_brand(ts_type: &TSType<'_>) -> bool {
    let TSType::TSTypeLiteral(lit) = ts_type else {
        return false;
    };
    brand_symbols(lit).is_some_and(|symbols| {
        symbols
            .iter()
            .all(|symbol| symbol.starts_with(NEWTYPE_BRAND_PREFIX))
    })
}

fn unparenthesized<'a, 'b>(ts_type: &'b TSType<'a>) -> &'b TSType<'a> {
    match ts_type {
        TSType::TSParenthesizedType(paren) => unparenthesized(&paren.type_annotation),
        _ => ts_type,
    }
}

/// Each parameter of a type parameter list as declared, its constraint and
/// default kept as written.
fn lower_type_params(params: &TSTypeParameterDeclaration<'_>, source: &str) -> Vec<TypeParamIR> {
    params
        .params
        .iter()
        .map(|param| TypeParamIR {
            name: param.name.to_string(),
            constraint: param
                .constraint
                .as_ref()
                .map(|constraint| type_source(constraint, source)),
            default: param
                .default
                .as_ref()
                .map(|default| type_source(default, source)),
            is_const: param.r#const,
        })
        .collect()
}

fn type_source(ts_type: &TSType<'_>, source: &str) -> String {
    let sp = ts_type.span();
    source
        .get(sp.start as usize..sp.end as usize)
        .unwrap_or_default()
        .to_string()
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
        .map(|tp| lower_type_params(tp, source))
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
    for ta in lower_type_aliases(program, source)? {
        targets.push(LoweredTarget::TypeAlias(ta));
    }
    for func in lower_functions(program, source, filter)? {
        targets.push(LoweredTarget::Function(func));
    }
    Ok(targets)
}

/// The names a module exports its own declarations under besides their own:
/// `default` for a default export (`export default class User`, `export
/// default User`) and the alias of `export { User as Member }`, each as the
/// declared name and the exported one. A re-export from another module
/// exports no declaration of this one.
pub fn collect_export_aliases(program: &Program<'_>) -> Vec<(String, String)> {
    let mut aliases = Vec::new();
    for statement in &program.body {
        match statement {
            Statement::ExportNamedDeclaration(export) => {
                for specifier in &export.specifiers {
                    let (declared, exported) = (specifier.local.name(), specifier.exported.name());
                    if declared != exported {
                        aliases.push((declared.to_string(), exported.to_string()));
                    }
                }
            }
            Statement::ExportDefaultDeclaration(export) => {
                let declared = match &export.declaration {
                    ExportDefaultDeclarationKind::ClassDeclaration(class) => {
                        class.id.as_ref().map(|id| id.name.to_string())
                    }
                    ExportDefaultDeclarationKind::TSInterfaceDeclaration(interface) => {
                        Some(interface.id.name.to_string())
                    }
                    ExportDefaultDeclarationKind::Identifier(name) => Some(name.name.to_string()),
                    _ => None,
                };
                if let Some(declared) = declared {
                    aliases.push((declared, "default".to_string()));
                }
            }
            _ => {}
        }
    }
    aliases
}

/// What a module exports again from other modules: `export { A as B } from
/// './a'`, `export { default as A } from './a'` and `export * from './a'`,
/// and a name it imports and then exports (`import { A } from './a'; export
/// { A as B }`, `export default A`). `export * as space from './a'` and an
/// exported namespace import export a namespace, which names no type.
pub fn collect_re_exports(program: &Program<'_>) -> Vec<ModuleReExport> {
    let imports = crate::import_registry::collect_file_imports(program);
    let imported_as = |local: &str, exported: &str| {
        let import = imports.iter().find(|import| import.local_name == local)?;
        let imported = import.original_name.as_deref().unwrap_or(local);
        (imported != "*").then(|| ModuleReExport {
            module_specifier: import.module_specifier.clone(),
            imported: imported.to_string(),
            exported: exported.to_string(),
        })
    };
    let mut re_exports = Vec::new();
    for statement in &program.body {
        match statement {
            Statement::ExportNamedDeclaration(export) => {
                re_exports.extend(export.specifiers.iter().filter_map(|specifier| {
                    imported_as(&specifier.local.name(), &specifier.exported.name())
                }));
            }
            Statement::ExportDefaultDeclaration(export) => {
                if let ExportDefaultDeclarationKind::Identifier(name) = &export.declaration {
                    re_exports.extend(imported_as(&name.name, "default"));
                }
            }
            Statement::ExportFromDeclaration(export) => {
                for specifier in &export.specifiers {
                    re_exports.push(ModuleReExport {
                        module_specifier: export.source.value.to_string(),
                        imported: specifier.local.name().to_string(),
                        exported: specifier.exported.name().to_string(),
                    });
                }
            }
            Statement::ExportAllDeclaration(export) if export.exported.is_none() => {
                re_exports.push(ModuleReExport {
                    module_specifier: export.source.value.to_string(),
                    imported: "*".to_string(),
                    exported: "*".to_string(),
                });
            }
            _ => {}
        }
    }
    re_exports
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
            Statement::ExportDefaultDeclaration(decl) => match &decl.declaration {
                ExportDefaultDeclarationKind::ClassDeclaration(c) => {
                    if let Some(id) = &c.id {
                        names.insert(id.name.to_string());
                    }
                }
                ExportDefaultDeclarationKind::TSInterfaceDeclaration(i) => {
                    names.insert(i.id.name.to_string());
                }
                _ => {}
            },
            _ => {}
        }
    }

    names
}

#[cfg(test)]
mod tests {
    use super::{
        ClassIR, InterfaceIR, Program, SpanIR, TypeAliasIR, TypeBody, TypeMemberKind,
        lower_classes, lower_enums, lower_interfaces, lower_type_aliases,
    };
    use crate::abi::MemberKey;
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
        let aliases = lower_type_aliases(&parse(&allocator, source), source)
            .expect("type aliases should lower");
        aliases
            .into_iter()
            .next()
            .expect("source declares a type alias")
    }

    #[test]
    fn a_reference_to_another_type_lowers_to_an_alias() {
        for (source, target) in [
            ("type A = Ledger;", "Ledger"),
            ("type B = Map<string, number>;", "Map<string, number>"),
            ("type C = string[];", "string[]"),
            ("type D = readonly Date[];", "readonly Date[]"),
            ("type E = string;", "string"),
        ] {
            assert_eq!(
                single_type_alias(source).body.as_alias(),
                Some(target),
                "{source}"
            );
        }
        for source in ["type F = keyof User;", "type G = `on${string}`;"] {
            assert!(
                single_type_alias(source).body.as_alias().is_none(),
                "{source}"
            );
        }
    }

    #[test]
    fn a_field_takes_the_type_its_declaration_makes_plain() {
        let class = single_class(
            "class Shape { count = 0; label = 'a'; on = true; big = 1n; low = -1; note = `n${1}`; \
             at = new Date(); tags = new Set<string>(); ratio = (0.5 as number); held = <string>x; \
             checked = 2 satisfies number; pattern = /a/; readonly kind = 'k'; readonly rank = -2; \
             readonly mark = `m`; list = []; made = new Thing(); frozen = [1] as const; \
             said: string = x; accessor step = 1; static accessor all = 0; accessor open; \
             constructor(public id = '', readonly code = 7, private bag = {}) {} }",
        );
        let plain = |name: &'static str, ts_type: &'static str| (name, ts_type, false, false);
        let unsaid = |name: &'static str| (name, "any", true, false);
        assert_eq!(
            class
                .fields
                .iter()
                .map(|field| {
                    (
                        field.name.as_str(),
                        field.ts_type.as_str(),
                        field.untyped,
                        field.accessor,
                    )
                })
                .collect::<Vec<_>>(),
            vec![
                plain("count", "number"),
                plain("label", "string"),
                plain("on", "boolean"),
                plain("big", "bigint"),
                plain("low", "number"),
                plain("note", "string"),
                plain("at", "Date"),
                plain("tags", "Set<string>"),
                plain("ratio", "number"),
                plain("held", "string"),
                plain("checked", "number"),
                plain("pattern", "RegExp"),
                plain("kind", "'k'"),
                plain("rank", "-2"),
                unsaid("mark"),
                unsaid("list"),
                unsaid("made"),
                unsaid("frozen"),
                plain("said", "string"),
                ("step", "number", false, true),
                ("open", "any", true, true),
                plain("id", "string"),
                plain("code", "7"),
                unsaid("bag"),
            ]
        );
    }

    #[test]
    fn an_accessor_signature_declares_a_field_and_a_method_a_function() {
        let interface = single_interface(
            "interface Sized { id: string; get size(): number; set size(value: number); get stamp(): Date; set mark(value: string); run(): void; }",
        );
        assert_eq!(
            interface
                .fields
                .iter()
                .map(|field| (field.name.as_str(), field.ts_type.as_str(), field.readonly))
                .collect::<Vec<_>>(),
            vec![
                ("id", "string", false),
                ("size", "number", false),
                ("stamp", "Date", true),
                ("mark", "string", false),
            ]
        );
        assert_eq!(
            interface
                .methods
                .iter()
                .map(|method| method.name.as_str())
                .collect::<Vec<_>>(),
            vec!["run"]
        );

        let allocator = Allocator::default();
        let source = "type Sized = { id: string; get size(): number; run(count: number): void; };";
        let aliases =
            lower_type_aliases(&parse(&allocator, source), source).expect("aliases should lower");
        let fields = aliases
            .first()
            .and_then(|alias| alias.body.as_object())
            .expect("an object type");
        assert_eq!(
            fields
                .iter()
                .map(|field| (field.name.as_str(), field.ts_type.as_str()))
                .collect::<Vec<_>>(),
            vec![
                ("id", "string"),
                ("size", "number"),
                ("run", "(count: number) => void"),
            ]
        );
    }

    #[test]
    fn an_enum_is_carried_with_how_it_is_declared() {
        let declared = |source: &str| {
            let allocator = Allocator::default();
            let enums =
                lower_enums(&parse(&allocator, source), source, None).expect("enums should lower");
            let lowered = enums.first().expect("source declares an enum");
            (lowered.is_const, lowered.is_declare)
        };
        assert_eq!(declared("enum Plain { A }"), (false, false));
        assert_eq!(declared("const enum Inlined { A }"), (true, false));
        assert_eq!(declared("declare enum Elsewhere { A }"), (false, true));
    }

    #[test]
    fn a_member_is_carried_with_how_its_key_is_written() {
        let class = single_class(
            "class Bag { name = ''; 'first-name' = ''; #secret = 1; [KEY] = 2; [Symbol.iterator] = 3; 1.5 = 4; [key: string]: unknown; static [index: number]: string; }",
        );
        assert_eq!(
            class
                .fields
                .iter()
                .map(|field| (field.name.clone(), field.key))
                .collect::<Vec<_>>(),
            vec![
                ("name".to_string(), MemberKey::Named),
                ("first-name".to_string(), MemberKey::Named),
                ("#secret".to_string(), MemberKey::Private),
                ("[KEY]".to_string(), MemberKey::Unnamed),
                ("[Symbol.iterator]".to_string(), MemberKey::Unnamed),
                ("[1.5]".to_string(), MemberKey::Unnamed),
                ("[key: string]".to_string(), MemberKey::Unnamed),
            ]
        );
        assert_eq!(class.fields[6].ts_type, "unknown");

        let interface = single_interface(
            "interface Bag { id: string; '#tag': string; '[x]': string; [KEY]: number; readonly [key: string]: unknown; }",
        );
        assert_eq!(
            interface
                .fields
                .iter()
                .map(|field| (field.name.clone(), field.key))
                .collect::<Vec<_>>(),
            vec![
                ("id".to_string(), MemberKey::Named),
                ("#tag".to_string(), MemberKey::Named),
                ("[x]".to_string(), MemberKey::Named),
                ("[KEY]".to_string(), MemberKey::Unnamed),
                ("[key: string]".to_string(), MemberKey::Unnamed),
            ]
        );
        assert!(interface.fields[4].readonly);

        let alias = single_type_alias("type Bag = { id: string; [key: string]: unknown };");
        let fields = alias.body.as_object().expect("an object type");
        assert_eq!(fields[1].key, MemberKey::Unnamed);
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
        // The `@testground/macro` inside an import-macro comment is a module
        // path, not a `@testground` directive.
        let iface = single_interface(
            r#"/** import macro {Gigaform} from "@testground/macro"; */
/** @derive(Default, Encode, Decode, Gigaform) */
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
 * @derive(Default, Decode)
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
    /** @endec(email) */
    email: string;

    /** @endec(minLength(2), maxLength(50)) */
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
                    .any(|decorator| decorator.name == "endec"),
                "{name} should have @endec decorator. Got: {:?}",
                field.decorators
            );
        }
    }

    #[test]
    fn type_alias_object_field_jsdoc_decorators() {
        let alias = single_type_alias(
            r#"
type ContactInfo = {
    /** @endec(email) */
    primaryEmail: string;

    /** @endec(minLength(1)) */
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
                    .any(|decorator| decorator.name == "endec"),
                "{name} should have @endec decorator. Got: {:?}",
                field.decorators
            );
        }
    }

    #[test]
    fn class_fields_keep_their_initializers_and_skip_statics() {
        let class = single_class(
            "class Settings {\n  static instances = 0;\n  theme: string = \"dark\";\n  label?: string;\n}",
        );
        let fields: Vec<(&str, Option<&str>)> = class
            .fields
            .iter()
            .map(|field| (field.name.as_str(), field.initializer.as_deref()))
            .collect();
        assert_eq!(fields, [("theme", Some("\"dark\"")), ("label", None)]);
    }

    #[test]
    fn symbol_keyed_literal_lowers_to_brand() {
        let alias = single_type_alias(
            "declare const MetersBrand: unique symbol;\ntype Meters = number & { readonly [MetersBrand]: true };",
        );
        let TypeBody::Intersection(members) = &alias.body else {
            panic!("expected an intersection body, got {:?}", alias.body);
        };
        assert_eq!(members[0].kind, TypeMemberKind::TypeRef("number".into()));
        assert_eq!(
            members[1].kind,
            TypeMemberKind::Brand(vec!["MetersBrand".into()])
        );
        assert_eq!(alias.body.primitive_base(), Some("number"));
    }

    #[test]
    fn mixed_literal_is_still_a_branded_primitive() {
        let alias = single_type_alias(
            "declare const B: unique symbol;\ntype Tagged = number & { readonly [B]: true; label: string };",
        );
        let TypeBody::Intersection(members) = &alias.body else {
            panic!("expected an intersection body, got {:?}", alias.body);
        };
        assert!(members[1].is_object());
        assert_eq!(alias.body.primitive_base(), Some("number"));
    }

    #[test]
    fn newtype_call_lowers_to_newtype() {
        let alias = single_type_alias("type Meters = $Newtype<number>;");
        assert_eq!(
            alias.body,
            TypeBody::Newtype(Box::new(TypeBody::Alias("number".to_string())))
        );
        assert_eq!(alias.body.primitive_base(), Some("number"));
    }

    #[test]
    fn newtype_expansion_lowers_to_newtype() {
        let alias =
            single_type_alias("type Meters = number & { readonly [__mf_newtype_3]: true };");
        assert_eq!(
            alias.body,
            TypeBody::Newtype(Box::new(TypeBody::Alias("number".to_string())))
        );
    }

    #[test]
    fn nested_newtypes_lower_to_nested_newtypes() {
        let call = single_type_alias("type Id = $Newtype<$Newtype<string>>;");
        let expanded = single_type_alias(
            "type Id = string & { readonly [__mf_newtype_1]: true } & { readonly [__mf_newtype_2]: true };",
        );
        let parenthesized = single_type_alias(
            "type Id = (string & { readonly [__mf_newtype_1]: true }) & { readonly [__mf_newtype_2]: true };",
        );
        let nested = TypeBody::Newtype(Box::new(TypeBody::Newtype(Box::new(TypeBody::Alias(
            "string".to_string(),
        )))));
        assert_eq!(call.body, nested);
        assert_eq!(expanded.body, nested);
        assert_eq!(parenthesized.body, nested);
        assert_eq!(call.body.primitive_base(), Some("string"));
    }

    #[test]
    fn hand_written_brand_stays_an_intersection() {
        let alias = single_type_alias("type Meters = number & { readonly [Brand]: true };");
        assert!(alias.body.is_intersection());
        assert_eq!(alias.body.primitive_base(), Some("number"));
    }

    #[test]
    fn newtype_with_two_arguments_is_not_a_newtype() {
        let alias = single_type_alias("type Pair = $Newtype<string, number>;");
        assert_eq!(alias.body.as_newtype(), None);
    }

    #[test]
    fn string_keyed_brand_is_a_primitive_base() {
        let alias = single_type_alias("type Tagged = number & { readonly __brand: \"Tagged\" };");
        assert_eq!(alias.body.primitive_base(), Some("number"));
        let empty = single_type_alias("type Loose = string & {};");
        assert_eq!(empty.body.primitive_base(), Some("string"));
    }

    #[test]
    fn primitive_base_of_plain_and_non_primitive_aliases() {
        assert_eq!(
            single_type_alias("type Id = string;").body.primitive_base(),
            Some("string")
        );
        assert_eq!(
            single_type_alias("type Big = bigint;")
                .body
                .primitive_base(),
            Some("bigint")
        );
        assert_eq!(
            single_type_alias("type U = User;").body.primitive_base(),
            None
        );
        assert_eq!(
            single_type_alias(
                "declare const B: unique symbol;\ntype U = User & { readonly [B]: true };"
            )
            .body
            .primitive_base(),
            None
        );
        assert_eq!(
            single_type_alias("type T = number & string & {};")
                .body
                .primitive_base(),
            None
        );
    }
}
