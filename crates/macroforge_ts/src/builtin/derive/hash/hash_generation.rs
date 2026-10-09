use crate::builtin::derive::common::{
    Builtin, ValueType, classify_value_type, collection_element_type, derived_function,
    field_value_type, rendered, structural_helper, tuple_element,
};
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::type_registry::{ResolvedTypeRef, TypeRegistry};
use crate::ts_syn::ts_ident;

use super::types::HashField;

/// The hash of a primitive value given as the expression `value`: integers as
/// themselves, other numbers, bigints and strings by their characters, and
/// booleans by the Java constants.
pub(crate) fn primitive_hash_expr(ts_type: &str, value: &str) -> String {
    match ts_type {
        "number" => {
            let as_text = string_hash(&rendered(ts_template! { @{value}.toString() }));
            rendered(ts_template! { (Number.isInteger(@{value}) ? @{value} | 0 : @{as_text}) })
        }
        "bigint" => string_hash(&rendered(ts_template! { @{value}.toString() })),
        "string" => string_hash(&rendered(ts_template! { (@{value} ?? "") })),
        "boolean" => rendered(ts_template! { (@{value} ? 1231 : 1237) }),
        _ => rendered(ts_template! { (@{value} != null ? 1 : 0) }),
    }
}

/// Generates JavaScript code that computes a hash contribution for a single field.
///
/// This function produces an expression that evaluates to an integer hash
/// value, dispatching on the field's declared type. Values that compare equal
/// under the derived `PartialEq` hash the same.
///
/// # Arguments
///
/// * `field` - The field to generate hash code for
/// * `var` - The variable name to use for field access (e.g., "self", "value")
///
/// # Returns
///
/// A string containing a JavaScript expression that evaluates to an integer hash value.
/// Field access uses the provided variable name: `var.fieldName`.
///
/// # Type-Specific Strategies
///
/// - **number**: Integer values used directly; floats hashed as strings
/// - **bigint**: String hash of decimal representation
/// - **string**: Character-by-character polynomial hash
/// - **boolean**: 1231 for true, 1237 for false (Java convention)
/// - **Date**: `getTime()` timestamp
/// - **Arrays**: Element hashes combined in order
/// - **Map, Set**: Entry hashes summed, since equality ignores their order
/// - **Types deriving `Hash`**: their `hashCode` function
/// - **Anything else** (unions, object literals, unresolved types):
///   `structuralHash` from `@macroforge/core/structural`
///
/// # Example
///
/// ```rust
/// use macroforge_ts::builtin::derive::hash::{HashField, generate_field_hash_for_interface};
///
/// let field = HashField {
///     name: "name".to_string(),
///     ts_type: "string".to_string(),
///     optional: false,
/// };
/// let registry = macroforge_ts::ts_syn::abi::ir::TypeRegistry::default();
/// let code = generate_field_hash_for_interface(&field, "self", None, &registry);
/// assert!(code.contains("self.name"));
/// assert!(code.contains("reduce"));
/// ```
pub fn generate_field_hash_for_interface(
    field: &HashField,
    var: &str,
    resolved: Option<&ResolvedTypeRef>,
    registry: &TypeRegistry,
) -> String {
    generate_value_hash(
        &field_value_type(&field.ts_type, field.optional),
        &rendered(ts_template! { @{var}.@{&field.name} }),
        resolved,
        registry,
    )
}

/// Generates the hash of `value`, an expression of type `ts_type`, with the
/// strategies of [`generate_field_hash_for_interface`].
pub(crate) fn generate_value_hash(
    ts_type: &str,
    value: &str,
    resolved: Option<&ResolvedTypeRef>,
    registry: &TypeRegistry,
) -> String {
    value_hash(ts_type, value, resolved, registry, 0)
}

/// An expression hashing the fixed-length tuple `value` element by element.
pub(crate) fn tuple_hash_expr(elements: &[String], registry: &TypeRegistry) -> String {
    elements
        .iter()
        .enumerate()
        .fold("1".to_string(), |hash, (index, source)| {
            let element = generate_value_hash(
                tuple_element(source).ts_type,
                &rendered(ts_template! { value[@{index}] }),
                None,
                registry,
            );
            rendered(ts_template! { ((@{hash}) * 31 + (@{element})) | 0 })
        })
}

fn value_hash(
    ts_type: &str,
    value: &str,
    resolved: Option<&ResolvedTypeRef>,
    registry: &TypeRegistry,
    depth: usize,
) -> String {
    let classified = classify_value_type(ts_type, resolved);
    let element = || resolved.and_then(collection_element_type);
    let item = ts_ident!("__item{}", depth);
    let hash = ts_ident!("__hash{}", depth);
    let hashed = match classified.kind {
        ValueType::Primitive(keyword) => primitive_hash_expr(keyword, value),
        ValueType::Opaque => return "0".to_string(),
        ValueType::Structural => return structural_hash(value),
        ValueType::Named(_) => match derived_function(resolved, registry, "Hash", "HashCode") {
            Some(function) => rendered(ts_template! { @{function}(@{value}) }),
            None => return structural_hash(value),
        },
        ValueType::Array(element_type) => {
            let inner = value_hash(element_type, &item.sym, element(), registry, depth + 1);
            rendered(ts_template! {
                @{value}.reduce((@{&hash}, @{&item}) => (@{&hash} * 31 + (@{inner})) | 0, 1)
            })
        }
        ValueType::Map {
            key,
            value: value_type,
        } => {
            let key_var = ts_ident!("__key{}", depth);
            let key_hash = value_hash(key, &key_var.sym, None, registry, depth + 1);
            let inner = value_hash(value_type, &item.sym, element(), registry, depth + 1);
            rendered(ts_template! {
                Array.from(@{value}).reduce((@{&hash}, [@{&key_var}, @{&item}]) =>
                    (@{&hash} + (((@{key_hash}) * 31 + (@{inner})) | 0)) | 0, 0)
            })
        }
        ValueType::Set(element_type) => {
            let inner = value_hash(element_type, &item.sym, element(), registry, depth + 1);
            rendered(ts_template! {
                Array.from(@{value}).reduce((@{&hash}, @{&item}) => (@{&hash} + (@{inner})) | 0, 0)
            })
        }
        ValueType::Builtin(builtin) => builtin_hash(builtin, value),
    };
    if classified.nullable {
        rendered(ts_template! { (@{value} == null ? 0 : @{hashed}) })
    } else {
        hashed
    }
}

fn string_hash(text: &str) -> String {
    rendered(ts_template! { @{text}.split("").reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0) })
}

fn builtin_hash(builtin: Builtin, value: &str) -> String {
    match builtin {
        Builtin::Date => rendered(ts_template! { (@{value}.getTime() | 0) }),
        Builtin::RegExp => string_hash(&rendered(
            ts_template! { (@{value}.source + @{value}.flags) },
        )),
        Builtin::Url => string_hash(&rendered(ts_template! { @{value}.href })),
        Builtin::UrlSearchParams => string_hash(&rendered(ts_template! { @{value}.toString() })),
        Builtin::TypedArray => rendered(ts_template! {
            Array.from(@{value}, Number).reduce((h, v) => (h * 31 + (v | 0)) | 0, 0)
        }),
        Builtin::ArrayBuffer => structural_hash(value),
        Builtin::Error => string_hash(&rendered(ts_template! { @{value}.message })),
    }
}

fn structural_hash(value: &str) -> String {
    let helper = structural_helper("structuralHash");
    rendered(ts_template! { @{helper}(@{value}) })
}
