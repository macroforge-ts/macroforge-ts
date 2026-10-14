//! A field or element type, classified by how the derived value traits
//! (`PartialEq`, `Hash`, `Clone`) treat its values.

use crate::ts_syn::abi::ir::type_registry::{ResolvedTypeRef, TypeRegistry};
use crate::ts_syn::abi::ir::{
    TypeBody, TypeMemberKind, is_primitive_keyword, split_top_level_union,
};

use crate::ts_syn::TsStream;

use super::registry_helpers::{resolved_type_has_derive, standalone_fn_name};
use super::type_utils::{array_element_type, parse_generic_type, tuple_element};

/// How values of a type are compared, hashed and copied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType<'a> {
    /// An immutable primitive compared with `===`, by its `typeof` keyword:
    /// `string`, `number`, `boolean` or `bigint`. Literal types map to theirs.
    Primitive(&'a str),
    /// A value held by reference and only ever compared by identity:
    /// `symbol`, functions, `null`, `undefined`, `void` and `never`.
    Opaque,
    Array(&'a str),
    Map {
        key: &'a str,
        value: &'a str,
    },
    Set(&'a str),
    Builtin(Builtin),
    /// A named type, which derives the trait itself when the registry says so.
    Named(&'a str),
    /// A type with no single runtime shape (a union, object literal, tuple,
    /// intersection, `unknown` or `object`), handled by its runtime structure.
    Structural,
}

/// A JavaScript built-in class with its own notion of equality and copying.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Builtin {
    Date,
    RegExp,
    Url,
    UrlSearchParams,
    TypedArray,
    ArrayBuffer,
    Error,
}

/// A type with its `null` and `undefined` members split off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassifiedType<'a> {
    /// The type with `| null` and `| undefined` removed.
    pub ts_type: &'a str,
    /// Whether the type admits `null` or `undefined`.
    pub nullable: bool,
    pub kind: ValueType<'a>,
}

/// A field's type as its values have it: an optional field (`name?: T`) may
/// also be `undefined`, whatever its annotation says.
pub fn field_value_type(ts_type: &str, optional: bool) -> String {
    if optional {
        rendered(crate::macros::ts_template! { @{ts_type} | undefined })
    } else {
        ts_type.to_string()
    }
}

/// Classifies `ts_type`. `resolved` adds nullability the declaration carries
/// outside the type (an optional field).
pub fn classify_value_type<'a>(
    ts_type: &'a str,
    resolved: Option<&ResolvedTypeRef>,
) -> ClassifiedType<'a> {
    let (ts_type, nullable) = strip_nullable(ts_type.trim());
    ClassifiedType {
        ts_type,
        nullable: nullable || resolved.is_some_and(|resolved| resolved.is_optional),
        kind: value_kind(ts_type),
    }
}

/// `ts_type` without its `null` and `undefined` members, and whether it had
/// any. A union of several other members stays whole.
fn strip_nullable(ts_type: &str) -> (&str, bool) {
    let Some(members) = split_top_level_union(ts_type) else {
        return (ts_type, false);
    };
    let mut present = members
        .iter()
        .filter(|member| !matches!(**member, "null" | "undefined"));
    match (present.next(), present.next()) {
        (Some(only), None) if members.len() > 1 => (only, true),
        (Some(_), Some(_)) | (Some(_), None) | (None, _) => (ts_type, false),
    }
}

fn value_kind(ts_type: &str) -> ValueType<'_> {
    // A union is the primitive all of its members are, and has no single
    // shape when they differ: `'a' | 3` starts with a quote and is no string.
    if let Some(members) = split_top_level_union(ts_type) {
        let mut keywords = members.iter().map(|member| primitive_keyword(member));
        return match keywords.next().flatten() {
            Some(first) if keywords.all(|keyword| keyword == Some(first)) => {
                ValueType::Primitive(first)
            }
            _ => ValueType::Structural,
        };
    }
    if let Some(keyword) = primitive_keyword(ts_type) {
        return ValueType::Primitive(keyword);
    }
    if let Some(element) = array_element_type(ts_type) {
        return ValueType::Array(element);
    }
    if let Some(builtin) = builtin(ts_type) {
        return ValueType::Builtin(builtin);
    }
    match ts_type {
        "symbol" | "Function" | "null" | "undefined" | "void" | "never" => {
            return ValueType::Opaque;
        }
        "unknown" | "any" | "object" => return ValueType::Structural,
        _ => {}
    }
    if split_top_level_union(ts_type).is_some()
        || ts_type.contains('&')
        || ts_type.starts_with(['{', '['])
    {
        return ValueType::Structural;
    }
    if ts_type.contains("=>") {
        return ValueType::Opaque;
    }
    match parse_generic_type(ts_type) {
        Some(("Map" | "ReadonlyMap", args)) => match split_type_args(args) {
            Some((key, value)) => ValueType::Map { key, value },
            None => ValueType::Structural,
        },
        Some(("Set" | "ReadonlySet", element)) => ValueType::Set(element),
        Some((name, _)) => ValueType::Named(name),
        None => ValueType::Named(ts_type),
    }
}

/// The `typeof` keyword of a primitive type or literal type.
fn primitive_keyword(ts_type: &str) -> Option<&'static str> {
    match ts_type {
        "string" => Some("string"),
        "number" => Some("number"),
        "boolean" | "true" | "false" => Some("boolean"),
        "bigint" => Some("bigint"),
        // A literal is quoted from end to end: `'a'[]` only starts as one.
        _ if ['"', '\'', '`'].into_iter().any(|quote| {
            ts_type.len() > 1 && ts_type.starts_with(quote) && ts_type.ends_with(quote)
        }) =>
        {
            Some("string")
        }
        _ => crate::ts_syn::abi::ir::number_literal(ts_type).map(|kind| match kind {
            crate::ts_syn::abi::ir::NumberLiteral::Number => "number",
            crate::ts_syn::abi::ir::NumberLiteral::BigInt => "bigint",
        }),
    }
}

fn builtin(ts_type: &str) -> Option<Builtin> {
    Some(match ts_type {
        "Date" => Builtin::Date,
        "RegExp" => Builtin::RegExp,
        "URL" => Builtin::Url,
        "URLSearchParams" => Builtin::UrlSearchParams,
        "Uint8Array" | "Int8Array" | "Uint16Array" | "Int16Array" | "Uint32Array"
        | "Int32Array" | "Float32Array" | "Float64Array" | "BigInt64Array" | "BigUint64Array"
        | "Uint8ClampedArray" => Builtin::TypedArray,
        "ArrayBuffer" => Builtin::ArrayBuffer,
        "Error" | "TypeError" | "RangeError" | "SyntaxError" | "ReferenceError" | "URIError"
        | "EvalError" => Builtin::Error,
        _ => return None,
    })
}

/// The two arguments of `Map<K, V>`, split at the top-level comma.
fn split_type_args(args: &str) -> Option<(&str, &str)> {
    let mut depth = 0usize;
    for (index, byte) in args.bytes().enumerate() {
        match byte {
            b'<' | b'(' | b'[' | b'{' => depth += 1,
            b'>' | b')' | b']' | b'}' => depth = depth.saturating_sub(1),
            b',' if depth == 0 => {
                let key = args.get(..index)?.trim();
                let value = args.get(index + 1..)?.trim();
                return Some((key, value));
            }
            _ => {}
        }
    }
    None
}

/// The text of a generated fragment, for generators that compose
/// expressions as text.
pub fn rendered(stream: TsStream) -> String {
    stream.source().to_owned()
}

/// `parts` each in a function of its own, called in turn. The checker reads
/// the flow of one body back from each use to its start, so a body holding a
/// part a field outgrows what it reads with a few hundred fields; a function
/// called where it is written is read as part of the body around it.
pub fn each_on_its_own(parts: impl IntoIterator<Item = TsStream>) -> TsStream {
    let parts: Vec<TsStream> = parts
        .into_iter()
        .filter(|part| !part.source().trim().is_empty())
        .collect();
    if parts.len() < 2 {
        return TsStream::merge_all(parts);
    }
    let functions = TsStream::merge_all(parts.into_iter().map(|part| {
        crate::macros::ts_template! {
            () => {
                {$typescript part}
            },
        }
    }));
    crate::macros::ts_template! {
        for (const __part of [
            {$typescript functions}
        ]) {
            __part();
        }
    }
}

/// Statements returning the first of `orders` that is not `0`, or `0`, each
/// worked out in a function of its own as [`each_on_its_own`] runs its parts:
/// a body comparing each field in turn outgrows what the checker reads.
pub fn first_order(orders: Vec<crate::ast::Expr>) -> TsStream {
    match orders.as_slice() {
        [] => crate::macros::ts_template! { return 0; },
        [order] => crate::macros::ts_template! { return @{order.clone()}; },
        _ => {
            let functions = TsStream::merge_all(orders.into_iter().map(|order| {
                crate::macros::ts_template! { () => @{order}, }
            }));
            crate::macros::ts_template! {
                for (const __order of [
                    {$typescript functions}
                ]) {
                    const __result = __order();
                    if (__result !== 0) return __result;
                }
                return 0;
            }
        }
    }
}

/// A call of the `@macroforge/core/structural` helper `name` on `args`,
/// requesting its import for the file being expanded.
pub fn structural_call(name: &str, args: &str) -> String {
    let helper = structural_helper(name);
    rendered(crate::macros::ts_template! { @{helper}(@{args}) })
}

/// The local name of a `@macroforge/core/structural` helper, requesting its
/// import for the file being expanded.
pub fn structural_helper(name: &str) -> String {
    let alias = format!("__mf_{name}");
    crate::host::import_registry::with_registry_mut(|registry| {
        registry.request_import(&alias, Some(name), crate::package::STRUCTURAL, false);
    });
    alias
}

/// A `@macroforge/core/structural` helper as a callee, requesting its
/// import for the file being expanded.
pub fn structural_callee(name: &str) -> crate::ast::Expr {
    crate::ts_syn::ts_ident!(structural_helper(name)).into()
}

/// The standalone function (`userEquals`, `userHashCode`) of the type
/// `resolved` names, when the registry records it deriving `derive`.
pub fn derived_function(
    resolved: Option<&ResolvedTypeRef>,
    registry: &TypeRegistry,
    derive: &str,
    suffix: &str,
) -> Option<String> {
    let resolved = resolved?;
    (!resolved.is_collection && resolved_type_has_derive(registry, resolved, derive))
        .then(|| standalone_fn_name(&resolved.base_type_name, suffix))
}

/// Whether every element of a tuple is fixed: no optional or rest elements,
/// so the tuple's length is known and its elements can be handled one by one.
pub fn fixed_tuple(elements: &[String]) -> bool {
    elements.iter().all(|source| {
        let element = tuple_element(source);
        !element.rest && !element.optional
    })
}

/// Whether every member of a union body is a literal or primitive keyword, so
/// no value of the alias is an object.
pub fn is_primitive_union(body: &TypeBody) -> bool {
    body.as_union().is_some_and(|members| {
        members.iter().all(|member| match &member.kind {
            TypeMemberKind::Literal(_) => true,
            TypeMemberKind::TypeRef(name) => is_primitive_keyword(name) || name.trim() == "symbol",
            TypeMemberKind::Object { .. }
            | TypeMemberKind::Intersection(_)
            | TypeMemberKind::Brand(_) => false,
        })
    })
}
