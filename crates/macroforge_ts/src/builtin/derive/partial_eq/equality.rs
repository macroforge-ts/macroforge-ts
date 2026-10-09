use crate::builtin::derive::common::{
    Builtin, ValueType, classify_value_type, collection_element_type, derived_function,
    field_value_type, rendered, structural_helper, tuple_element,
};
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::type_registry::{ResolvedTypeRef, TypeRegistry};
use crate::ts_syn::ts_ident;

use super::types::EqField;

/// Generates JavaScript code that compares fields for equality.
///
/// This function produces an expression that evaluates to a boolean indicating
/// whether the field values are equal, dispatching on the field's declared
/// type.
///
/// # Arguments
///
/// * `field` - The field to generate comparison code for
/// * `self_var` - Variable name for the first object (e.g., "self", "a")
/// * `other_var` - Variable name for the second object (e.g., "other", "b")
///
/// # Returns
///
/// A string containing a JavaScript boolean expression comparing `self_var.field`
/// with `other_var.field`. The expression can be combined with `&&` for
/// multiple fields.
///
/// # Type-Specific Strategies
///
/// - **Primitives and literals**: strict equality (`===`)
/// - **Arrays**: same length, each element compared by its own type
/// - **Map**: same size, same keys, each value compared by its own type
/// - **Set**: same size, each element matched by an equal element
/// - **Date, RegExp, URL, typed arrays, errors**: by value
/// - **Types deriving `PartialEq`**: their `equals` function
/// - **Anything else** (unions, object literals, unresolved types):
///   `structuralEquals` from `@macroforge/core/structural`
///
/// # Example
///
/// ```rust
/// use macroforge_ts::builtin::derive::partial_eq::{EqField, generate_field_equality_for_interface};
///
/// let field = EqField {
///     name: "name".to_string(),
///     ts_type: "string".to_string(),
///     optional: false,
/// };
/// let registry = macroforge_ts::ts_syn::abi::ir::TypeRegistry::default();
/// let code = generate_field_equality_for_interface(&field, "self", "other", None, &registry);
/// assert_eq!(code, "self.name === other.name");
/// ```
pub fn generate_field_equality_for_interface(
    field: &EqField,
    self_var: &str,
    other_var: &str,
    resolved: Option<&ResolvedTypeRef>,
    registry: &TypeRegistry,
) -> String {
    generate_value_equality(
        &field_value_type(&field.ts_type, field.optional),
        &rendered(ts_template! { @{self_var}.@{&field.name} }),
        &rendered(ts_template! { @{other_var}.@{&field.name} }),
        resolved,
        registry,
    )
}

/// Generates the equality of two values of `ts_type`, given as the
/// expressions `left` and `right`, with the strategies of
/// [`generate_field_equality_for_interface`].
pub(crate) fn generate_value_equality(
    ts_type: &str,
    left: &str,
    right: &str,
    resolved: Option<&ResolvedTypeRef>,
    registry: &TypeRegistry,
) -> String {
    value_equality(ts_type, left, right, resolved, registry, 0)
}

/// Statements returning whether the fixed-length tuples `a` and `b` are equal,
/// element by element.
pub(crate) fn tuple_equality_statements(elements: &[String], registry: &TypeRegistry) -> String {
    let comparisons: Vec<String> = elements
        .iter()
        .enumerate()
        .map(|(index, source)| {
            generate_value_equality(
                tuple_element(source).ts_type,
                &rendered(ts_template! { a[@{index}] }),
                &rendered(ts_template! { b[@{index}] }),
                None,
                registry,
            )
        })
        .collect();
    let all = comparisons.join(" && ");
    rendered(ts_template! { return @{all}; })
}

fn value_equality(
    ts_type: &str,
    left: &str,
    right: &str,
    resolved: Option<&ResolvedTypeRef>,
    registry: &TypeRegistry,
    depth: usize,
) -> String {
    let classified = classify_value_type(ts_type, resolved);
    let element = || resolved.and_then(collection_element_type);
    let item = ts_ident!("__item{}", depth);
    let equality = match classified.kind {
        // `===` and the structural helper already treat `null` and `undefined`.
        ValueType::Primitive(_) | ValueType::Opaque => {
            return rendered(ts_template! { @{left} === @{right} });
        }
        ValueType::Structural => return structural_equality(left, right),
        ValueType::Named(_) => match derived_function(resolved, registry, "PartialEq", "Equals") {
            Some(function) => rendered(ts_template! { @{function}(@{left}, @{right}) }),
            None => return structural_equality(left, right),
        },
        ValueType::Array(element_type) => {
            let index = ts_ident!("__index{}", depth);
            let other = ts_ident!("__other{}", depth);
            let inner = value_equality(
                element_type,
                &item.sym,
                &other.sym,
                element(),
                registry,
                depth + 1,
            );
            // The other element is read into a local so an index the type
            // checker cannot prove in bounds narrows before it is used.
            rendered(
                ts_template! { (@{left}.length === @{right}.length && @{left}.every((@{&item}, @{&index}) => { const @{&other} = @{right}[@{&index}]; return @{&other} === undefined ? @{&item} === undefined : @{inner}; })) },
            )
        }
        ValueType::Map { value, .. } => {
            let key = ts_ident!("__key{}", depth);
            let other = ts_ident!("__other{}", depth);
            let inner =
                value_equality(value, &item.sym, &other.sym, element(), registry, depth + 1);
            rendered(
                ts_template! { (@{left}.size === @{right}.size && Array.from(@{left}).every(([@{&key}, @{&item}]) => { const @{&other} = @{right}.get(@{&key}); return @{right}.has(@{&key}) && (@{&other} === undefined ? @{&item} === undefined : @{inner}); })) },
            )
        }
        ValueType::Set(element_type) => {
            let other = ts_ident!("__other{}", depth);
            let inner = value_equality(
                element_type,
                &item.sym,
                &other.sym,
                element(),
                registry,
                depth + 1,
            );
            rendered(
                ts_template! { (@{left}.size === @{right}.size && Array.from(@{left}).every((@{&item}) => @{right}.has(@{&item}) || Array.from(@{right}).some((@{&other}) => @{inner}))) },
            )
        }
        ValueType::Builtin(builtin) => builtin_equality(builtin, left, right),
    };
    if classified.nullable {
        rendered(ts_template! {
            (@{left} == null || @{right} == null ? @{left} === @{right} : @{equality})
        })
    } else {
        equality
    }
}

fn builtin_equality(builtin: Builtin, left: &str, right: &str) -> String {
    rendered(match builtin {
        Builtin::Date => ts_template! { @{left}.getTime() === @{right}.getTime() },
        Builtin::RegExp => ts_template! {
            (@{left}.source === @{right}.source && @{left}.flags === @{right}.flags)
        },
        Builtin::Url => ts_template! { @{left}.href === @{right}.href },
        Builtin::UrlSearchParams => ts_template! { @{left}.toString() === @{right}.toString() },
        Builtin::TypedArray => ts_template! {
            (@{left}.length === @{right}.length && @{left}.every((__byte, __at) => __byte === @{right}[__at]))
        },
        Builtin::ArrayBuffer => return structural_equality(left, right),
        Builtin::Error => ts_template! {
            (@{left}.message === @{right}.message && @{left}.constructor === @{right}.constructor)
        },
    })
}

fn structural_equality(left: &str, right: &str) -> String {
    let helper = structural_helper("structuralEquals");
    rendered(ts_template! { @{helper}(@{left}, @{right}) })
}
