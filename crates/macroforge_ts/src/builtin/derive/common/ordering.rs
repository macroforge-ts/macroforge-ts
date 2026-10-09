//! The comparison of two values for the ordering derives, `PartialOrd` and
//! `Ord`, which must agree with `PartialEq`: a comparison is `0` exactly when
//! the values are equal.

use crate::macros::ts_template;
use crate::ts_syn::abi::ir::type_registry::{ResolvedTypeRef, TypeRegistry};
use crate::ts_syn::ts_ident;

use super::registry_helpers::collection_element_type;
use super::value_type::{
    Builtin, ValueType, classify_value_type, derived_function, field_value_type, rendered,
    structural_helper,
};

/// A field taking part in an ordering comparison.
pub struct OrdField {
    /// The field name, as accessed on the compared values.
    pub name: String,
    /// The field's declared type, which selects its comparison.
    pub ts_type: String,
    /// Whether the field is declared optional (`name?: T`), so its value may
    /// be `undefined` whatever `ts_type` says.
    pub optional: bool,
}

/// Generates the comparison of `field` between the values `self_var` and
/// `other_var`, with the strategies of [`generate_value_order`].
pub fn generate_field_order(
    ordering: Ordering,
    field: &OrdField,
    self_var: &str,
    other_var: &str,
    resolved: Option<&ResolvedTypeRef>,
    registry: &TypeRegistry,
) -> String {
    generate_value_order(
        ordering,
        &field_value_type(&field.ts_type, field.optional),
        &rendered(ts_template! { @{self_var}.@{&field.name} }),
        &rendered(ts_template! { @{other_var}.@{&field.name} }),
        resolved,
        registry,
    )
}

/// Which ordering derive a comparison is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ordering {
    /// `PartialOrd`: `-1`, `0`, `1`, or `null` for an unordered pair (`NaN`).
    Partial,
    /// `Ord`: always `-1`, `0` or `1`.
    Total,
}

impl Ordering {
    fn derive(self) -> &'static str {
        match self {
            Self::Partial => "PartialOrd",
            Self::Total => "Ord",
        }
    }

    fn suffix(self) -> &'static str {
        match self {
            Self::Partial => "PartialCompare",
            Self::Total => "Compare",
        }
    }
}

/// Generates the comparison of two values of `ts_type`, given as the
/// expressions `left` and `right`.
///
/// - **number, bigint, string**: `<` and `>`, strings by UTF-16 code units as
///   `===` distinguishes them; under `Partial`, `NaN` is unordered (`null`)
/// - **boolean**: `false` before `true`
/// - **Date, URL, URLSearchParams**: by timestamp or text
/// - **Arrays**: lexicographic, each element compared by its own type
/// - **Types deriving the trait**: their `compare` or `partialCompare`
/// - **Anything else** (unions, object literals, maps, sets, unresolved
///   types): `structuralCompare` from `@macroforge/core/structural`
pub fn generate_value_order(
    ordering: Ordering,
    ts_type: &str,
    left: &str,
    right: &str,
    resolved: Option<&ResolvedTypeRef>,
    registry: &TypeRegistry,
) -> String {
    value_order(ordering, ts_type, left, right, resolved, registry, 0)
}

fn value_order(
    ordering: Ordering,
    ts_type: &str,
    left: &str,
    right: &str,
    resolved: Option<&ResolvedTypeRef>,
    registry: &TypeRegistry,
    depth: usize,
) -> String {
    let classified = classify_value_type(ts_type, resolved);
    let compared = match classified.kind {
        ValueType::Primitive("number") => number_order(ordering, left, right),
        ValueType::Primitive("boolean") => {
            rendered(ts_template! { (@{left} === @{right} ? 0 : @{left} ? 1 : -1) })
        }
        // Strings and bigints have no unordered pair.
        ValueType::Primitive(_) => text_order(left, right),
        ValueType::Named(_) => {
            match derived_function(resolved, registry, ordering.derive(), ordering.suffix()) {
                Some(function) => rendered(ts_template! { @{function}(@{left}, @{right}) }),
                None => return structural_order(left, right),
            }
        }
        ValueType::Array(element_type) => {
            let (left_items, right_items) =
                (ts_ident!("__left{}", depth), ts_ident!("__right{}", depth));
            let (item, other) = (ts_ident!("__item{}", depth), ts_ident!("__other{}", depth));
            let (index, order) = (ts_ident!("__index{}", depth), ts_ident!("__order{}", depth));
            let shared = ts_ident!("__shared{}", depth);
            let inner = value_order(
                ordering,
                element_type,
                &item.sym,
                &other.sym,
                resolved.and_then(collection_element_type),
                registry,
                depth + 1,
            );
            // Elements are read into locals so an index the type checker
            // cannot prove in bounds narrows before it is used.
            let structural = structural_order(&item.sym, &other.sym);
            rendered(ts_template! {
                (() => {
                    const @{&left_items} = @{left};
                    const @{&right_items} = @{right};
                    const @{&shared} = Math.min(@{&left_items}.length, @{&right_items}.length);
                    for (let @{&index} = 0; @{&index} < @{&shared}; @{&index}++) {
                        const @{&item} = @{&left_items}[@{&index}];
                        const @{&other} = @{&right_items}[@{&index}];
                        const @{&order} = @{&item} === undefined || @{&other} === undefined
                            ? @{structural}
                            : @{inner};
                        if (@{&order} !== 0) return @{&order};
                    }
                    return @{&left_items}.length < @{&right_items}.length ? -1
                        : @{&left_items}.length > @{&right_items}.length ? 1 : 0;
                })()
            })
        }
        ValueType::Builtin(Builtin::Date) => number_order(
            ordering,
            &rendered(ts_template! { @{left}.getTime() }),
            &rendered(ts_template! { @{right}.getTime() }),
        ),
        ValueType::Builtin(Builtin::Url) => text_order(
            &rendered(ts_template! { @{left}.href }),
            &rendered(ts_template! { @{right}.href }),
        ),
        ValueType::Builtin(Builtin::UrlSearchParams) => text_order(
            &rendered(ts_template! { @{left}.toString() }),
            &rendered(ts_template! { @{right}.toString() }),
        ),
        ValueType::Builtin(
            Builtin::RegExp | Builtin::TypedArray | Builtin::ArrayBuffer | Builtin::Error,
        )
        | ValueType::Opaque
        | ValueType::Structural
        | ValueType::Map { .. }
        | ValueType::Set(_) => return structural_order(left, right),
    };
    if classified.nullable {
        // An absent value orders as the structural order ranks it: before
        // every present one.
        let structural = structural_order(left, right);
        rendered(
            ts_template! { (@{left} == null || @{right} == null ? @{structural} : @{compared}) },
        )
    } else {
        compared
    }
}

fn number_order(ordering: Ordering, left: &str, right: &str) -> String {
    match ordering {
        Ordering::Partial => rendered(ts_template! {
            (@{left} < @{right} ? -1 : @{left} > @{right} ? 1 : @{left} === @{right} ? 0 : null)
        }),
        Ordering::Total => text_order(left, right),
    }
}

fn text_order(left: &str, right: &str) -> String {
    rendered(ts_template! { (@{left} < @{right} ? -1 : @{left} > @{right} ? 1 : 0) })
}

fn structural_order(left: &str, right: &str) -> String {
    let helper = structural_helper("structuralCompare");
    rendered(ts_template! { @{helper}(@{left}, @{right}) })
}
