use std::collections::HashMap;

use crate::builtin::derive::common::{
    Member, ValueType, classify_value_type, collection_element_type, derived_function,
    foreign_handler, js_string, rendered, structural_helper,
};
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::type_registry::{ResolvedTypeRef, TypeRegistry};
use crate::ts_syn::config::ForeignHandler;

use super::types::DebugField;

/// Statements pushing `"label: value"` onto `parts` for each field of `value`.
pub(super) fn push_statements(
    fields: &[DebugField],
    resolved_fields: Option<&HashMap<String, ResolvedTypeRef>>,
    registry: &TypeRegistry,
) -> Result<String, String> {
    let statements = fields
        .iter()
        .map(|field| {
            let resolved = resolved_fields.and_then(|fields| fields.get(&field.name));
            let shown = debug_value_expr(
                &field.ts_type,
                &rendered(ts_template! { value@{Member::of(&field.name)} }),
                resolved,
                registry,
            )?;
            let label = js_string(&format!("{}: ", field.label));
            Ok(rendered(ts_template! { parts.push(@{label} + @{shown}); }))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(statements.join("\n"))
}

/// A string rendering `value`, an expression of type `ts_type`: primitives
/// and built-ins as `String` renders them, types deriving `Debug` through
/// their `toString` function, configured foreign types through their
/// `toString` handler, which they have to configure, and anything else
/// through `structuralDebug` from `@macroforge/core/structural`.
pub(super) fn debug_value_expr(
    ts_type: &str,
    value: &str,
    resolved: Option<&ResolvedTypeRef>,
    registry: &TypeRegistry,
) -> Result<String, String> {
    let classified = classify_value_type(ts_type, resolved);
    let foreign = foreign_handler(classified.ts_type, ForeignHandler::ToString).transpose()?;
    let shown = match (foreign, classified.kind) {
        (Some(handler), _) => rendered(ts_template! { @{handler}(@{value}) }),
        (None, ValueType::Primitive(_) | ValueType::Builtin(_) | ValueType::Opaque) => {
            return Ok(rendered(ts_template! { String(@{value}) }));
        }
        (None, ValueType::Named(_)) => {
            match derived_function(resolved, registry, "Debug", "ToString") {
                Some(function) => rendered(ts_template! { @{function}(@{value}) }),
                None => return Ok(structural_debug(value)),
            }
        }
        (None, ValueType::Array(element)) => {
            let element_function = match foreign_handler(
                classify_value_type(element, None).ts_type,
                ForeignHandler::ToString,
            ) {
                Some(handler) => Some(handler?),
                None => derived_function(
                    resolved.and_then(collection_element_type),
                    registry,
                    "Debug",
                    "ToString",
                ),
            };
            match element_function {
                // The element's own `toString` takes no `null` or `undefined`.
                Some(function) if classify_value_type(element, None).nullable => {
                    rendered(ts_template! {
                        "[" + @{value}.map((item) => item == null ? String(item) : @{function}(item)).join(", ") + "]"
                    })
                }
                Some(function) => {
                    rendered(ts_template! { "[" + @{value}.map(@{function}).join(", ") + "]" })
                }
                None => return Ok(structural_debug(value)),
            }
        }
        (None, ValueType::Map { .. } | ValueType::Set(_) | ValueType::Structural) => {
            return Ok(structural_debug(value));
        }
    };
    Ok(if classified.nullable {
        rendered(ts_template! { (@{value} == null ? String(@{value}) : @{shown}) })
    } else {
        shown
    })
}

fn structural_debug(value: &str) -> String {
    let helper = structural_helper("structuralDebug");
    rendered(ts_template! { @{helper}(@{value}) })
}
