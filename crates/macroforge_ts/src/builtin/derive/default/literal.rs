//! A written default that is data in its wire form, decoded into the field's
//! type. A literal cannot be a newtype's brand, a `RecordLink`'s record id or
//! an object holding either, so it goes through the type's own decoder: only
//! a type's own module brands raw values, as a Rust newtype's private
//! constructor would.

use convert_case::{Case, Casing};

use super::values::DefaultSources;
use crate::builtin::derive::common::{rendered, type_has_derive};
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::{
    TypeBody, TypeDefinitionIR, is_primitive_keyword, split_top_level_union,
};
use crate::ts_syn::import_registry::with_registry_mut;
use crate::ts_syn::ts_ident;

/// `value`, a field's written default, decoded through the field's type when
/// it is plain data the type has to decode: a literal for a newtype or a
/// generic alias such as `RecordLink<User>`, or an object literal for an
/// object type. Anything else is returned as written.
pub(super) fn decode_literal_default(
    value: String,
    ts_type: &str,
    sources: &DefaultSources,
) -> String {
    let literal = value.trim();
    let Some(shape) = data_shape(literal) else {
        return value;
    };
    let members = split_top_level_union(ts_type).unwrap_or_else(|| vec![ts_type.trim()]);
    let present: Vec<&str> = members
        .into_iter()
        .filter(|member| !matches!(*member, "null" | "undefined"))
        .collect();
    let [name] = present.as_slice() else {
        return value;
    };
    if is_primitive_keyword(name) {
        return value;
    }
    let (base, type_args) = match name.split_once('<') {
        Some((base, args)) => (base.trim(), format!("<{args}")),
        None => (*name, String::new()),
    };
    let Some(entry) =
        sources
            .registry
            .resolve_in_file(base, sources.caller_file_path, sources.file_imports)
    else {
        return value;
    };
    let decodes = match &entry.definition {
        TypeDefinitionIR::TypeAlias(alias) => match &alias.body {
            TypeBody::Newtype(_) => true,
            TypeBody::Object { .. } => shape == Shape::Object,
            TypeBody::Union(_)
            | TypeBody::Intersection(_)
            | TypeBody::Alias(_)
            | TypeBody::Tuple(_)
            | TypeBody::Other(_) => !type_args.is_empty(),
        },
        TypeDefinitionIR::Interface(_) | TypeDefinitionIR::Class(_) => shape == Shape::Object,
        TypeDefinitionIR::Enum(_) => false,
    };
    if !decodes {
        return value;
    }
    if !type_has_derive(sources.registry, base, "Decode") {
        sources.rejected.borrow_mut().push(format!(
            "@default({literal}) on a '{name}' field needs '{base}' to derive Decode, which turns the literal into a '{base}'"
        ));
        return value;
    }
    let decode = ts_ident!("{}Decode", base.to_case(Case::Camel));
    let decoded_literal = ts_ident!("__mf_decodedLiteral");
    with_registry_mut(|imports| {
        imports.request_import(
            "__mf_decodedLiteral",
            Some("decodedLiteral"),
            crate::package::ENDEC,
            false,
        );
    });
    rendered(ts_template! { @{decoded_literal}(@{decode}@{type_args}(@{literal})) })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    Scalar,
    Object,
}

/// Whether `source` is plain data, with no expression to evaluate: a string,
/// number, boolean or null literal, or an object or array holding only those.
fn data_shape(source: &str) -> Option<Shape> {
    use oxc::allocator::Allocator;
    use oxc::ast::ast::{ArrayExpressionElement, Expression, ObjectPropertyKind, UnaryOperator};

    fn plain(expression: &Expression<'_>) -> bool {
        match expression {
            Expression::StringLiteral(_)
            | Expression::NumericLiteral(_)
            | Expression::BooleanLiteral(_)
            | Expression::NullLiteral(_) => true,
            Expression::TemplateLiteral(template) => template.expressions.is_empty(),
            Expression::UnaryExpression(unary) => {
                unary.operator == UnaryOperator::UnaryNegation
                    && matches!(unary.argument, Expression::NumericLiteral(_))
            }
            Expression::ParenthesizedExpression(inner) => plain(&inner.expression),
            Expression::ArrayExpression(array) => array.elements.iter().all(|element| {
                element.as_expression().is_some_and(plain)
                    && !matches!(element, ArrayExpressionElement::SpreadElement(_))
            }),
            Expression::ObjectExpression(object) => object.properties.iter().all(|property| {
                matches!(property, ObjectPropertyKind::ObjectProperty(property)
                    if !property.computed && !property.method && plain(&property.value))
            }),
            _ => false,
        }
    }

    let allocator = Allocator::default();
    let parsed = oxc::parser::Parser::new(&allocator, source, oxc::span::SourceType::ts())
        .parse_expression()
        .ok()?;
    let mut expression = &parsed;
    while let Expression::ParenthesizedExpression(inner) = expression {
        expression = &inner.expression;
    }
    if !plain(expression) {
        return None;
    }
    Some(match expression {
        Expression::ObjectExpression(_) => Shape::Object,
        _ => Shape::Scalar,
    })
}
