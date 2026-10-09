use std::collections::HashMap;

use crate::builtin::derive::common::{
    ValueType, classify_value_type, collection_element_type, derived_function, js_string, rendered,
    structural_helper,
};
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::type_registry::{ResolvedTypeRef, TypeRegistry};

use super::types::DebugField;

/// Statements pushing `"label: value"` onto `parts` for each field of `value`.
pub(super) fn push_statements(
    fields: &[DebugField],
    resolved_fields: Option<&HashMap<String, ResolvedTypeRef>>,
    registry: &TypeRegistry,
) -> String {
    fields
        .iter()
        .map(|field| {
            let resolved = resolved_fields.and_then(|fields| fields.get(&field.name));
            let shown = debug_value_expr(
                &field.ts_type,
                &rendered(ts_template! { value.@{&field.name} }),
                resolved,
                registry,
            );
            let label = js_string(&format!("{}: ", field.label));
            rendered(ts_template! { parts.push(@{label} + @{shown}); })
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// A string rendering `value`, an expression of type `ts_type`: primitives
/// and built-ins as `String` renders them, types deriving `Debug` through
/// their `toString` function, and anything else through `structuralDebug`
/// from `@macroforge/core/structural`.
pub(super) fn debug_value_expr(
    ts_type: &str,
    value: &str,
    resolved: Option<&ResolvedTypeRef>,
    registry: &TypeRegistry,
) -> String {
    let classified = classify_value_type(ts_type, resolved);
    let shown = match classified.kind {
        ValueType::Primitive(_) | ValueType::Builtin(_) | ValueType::Opaque => {
            return rendered(ts_template! { String(@{value}) });
        }
        ValueType::Named(_) => match derived_function(resolved, registry, "Debug", "ToString") {
            Some(function) => rendered(ts_template! { @{function}(@{value}) }),
            None => return structural_debug(value),
        },
        ValueType::Array(_) => {
            match derived_function(
                resolved.and_then(collection_element_type),
                registry,
                "Debug",
                "ToString",
            ) {
                Some(function) => {
                    rendered(ts_template! { "[" + @{value}.map(@{function}).join(", ") + "]" })
                }
                None => return structural_debug(value),
            }
        }
        ValueType::Map { .. } | ValueType::Set(_) | ValueType::Structural => {
            return structural_debug(value);
        }
    };
    if classified.nullable {
        rendered(ts_template! { (@{value} == null ? String(@{value}) : @{shown}) })
    } else {
        shown
    }
}

fn structural_debug(value: &str) -> String {
    let helper = structural_helper("structuralDebug");
    rendered(ts_template! { @{helper}(@{value}) })
}
