use crate::builtin::derive::common::{
    Builtin, ValueType, classify_value_type, collection_element_type, derived_function, rendered,
    structural_helper, tuple_element,
};
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::type_registry::{ResolvedTypeRef, TypeRegistry};
use crate::ts_syn::ts_ident;

/// Generate a clone expression for the field `field_name` of `var`,
/// dispatching on the field's declared type.
///
/// - **Primitives and literals**: the value itself
/// - **Arrays, Map, Set**: rebuilt with each element cloned by its own type
///   (a map's keys are shared, since lookups match them by identity)
/// - **Date, RegExp, URL, typed arrays**: copied by value
/// - **Types deriving `Clone`**: their `clone` function
/// - **Anything else** (unions, object literals, unresolved types):
///   `structuralClone` from `@macroforge/core/structural`
pub(crate) fn generate_clone_expr(
    field_name: &str,
    ts_type: &str,
    var: &str,
    resolved: Option<&ResolvedTypeRef>,
    registry: &TypeRegistry,
) -> String {
    let field = rendered(ts_template! { @{var}.@{field_name} });
    generate_value_clone(ts_type, &field, resolved, registry)
}

/// Generates a clone of `value`, an expression of type `ts_type`, with the
/// strategies of [`generate_clone_expr`].
pub(crate) fn generate_value_clone(
    ts_type: &str,
    value: &str,
    resolved: Option<&ResolvedTypeRef>,
    registry: &TypeRegistry,
) -> String {
    value_clone(ts_type, value, resolved, registry, 0)
}

/// An array literal cloning the fixed-length tuple `value` element by element.
pub(crate) fn tuple_clone_expr(elements: &[String], registry: &TypeRegistry) -> String {
    let clones: Vec<String> = elements
        .iter()
        .enumerate()
        .map(|(index, source)| {
            generate_value_clone(
                tuple_element(source).ts_type,
                &rendered(ts_template! { value[@{index}] }),
                None,
                registry,
            )
        })
        .collect();
    let clones = clones.join(", ");
    rendered(ts_template! { [@{clones}] })
}

fn value_clone(
    ts_type: &str,
    value: &str,
    resolved: Option<&ResolvedTypeRef>,
    registry: &TypeRegistry,
    depth: usize,
) -> String {
    let classified = classify_value_type(ts_type, resolved);
    let element = || resolved.and_then(collection_element_type);
    let item = ts_ident!("__item{}", depth);
    let cloned = match classified.kind {
        ValueType::Primitive(_) | ValueType::Opaque => return value.to_string(),
        ValueType::Structural => return structural_clone(value),
        ValueType::Named(_) => match derived_function(resolved, registry, "Clone", "Clone") {
            Some(function) => rendered(ts_template! { @{function}(@{value}) }),
            None => return structural_clone(value),
        },
        ValueType::Array(element_type) => {
            let inner = value_clone(element_type, &item.sym, element(), registry, depth + 1);
            if inner == item.sym {
                rendered(ts_template! { [...@{value}] })
            } else {
                rendered(ts_template! { @{value}.map((@{&item}) => @{inner}) })
            }
        }
        ValueType::Map {
            value: value_type, ..
        } => {
            let inner = value_clone(value_type, &item.sym, element(), registry, depth + 1);
            if inner == item.sym {
                rendered(ts_template! { new Map(@{value}) })
            } else {
                let key = ts_ident!("__key{}", depth);
                let copy = ts_ident!("__copy{}", depth);
                rendered(ts_template! {
                    (() => {
                        const @{&copy} = new Map(@{value});
                        for (const [@{&key}, @{&item}] of @{value}) @{&copy}.set(@{&key}, @{inner});
                        return @{&copy};
                    })()
                })
            }
        }
        ValueType::Set(element_type) => {
            let inner = value_clone(element_type, &item.sym, element(), registry, depth + 1);
            if inner == item.sym {
                rendered(ts_template! { new Set(@{value}) })
            } else {
                rendered(ts_template! { new Set(Array.from(@{value}, (@{&item}) => @{inner})) })
            }
        }
        ValueType::Builtin(builtin) => match builtin {
            Builtin::Date => rendered(ts_template! { new Date(@{value}.getTime()) }),
            Builtin::RegExp => {
                rendered(ts_template! { new RegExp(@{value}.source, @{value}.flags) })
            }
            Builtin::Url => rendered(ts_template! { new URL(@{value}.href) }),
            Builtin::UrlSearchParams => rendered(ts_template! { new URLSearchParams(@{value}) }),
            Builtin::TypedArray => rendered(ts_template! { @{value}.slice() }),
            Builtin::ArrayBuffer => rendered(ts_template! { @{value}.slice(0) }),
            // Errors carry a stack and identity; equality compares their
            // message and class, which a shared reference keeps.
            Builtin::Error => return value.to_string(),
        },
    };
    if classified.nullable {
        rendered(ts_template! { (@{value} == null ? @{value} : @{cloned}) })
    } else {
        cloned
    }
}

fn structural_clone(value: &str) -> String {
    let helper = structural_helper("structuralClone");
    rendered(ts_template! { @{helper}(@{value}) })
}
