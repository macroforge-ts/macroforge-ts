use convert_case::{Case, Casing};

use crate::ast::Expr;
use crate::builtin::derive::common::TypeNames;
use crate::builtin::derive::common::{ClassUse, derive_class_fields, derive_interface_fields};
use crate::builtin::derive::common::{
    CompareFieldOptions, EnumValueKinds, OrdField, Ordering, generate_field_order,
    generate_value_order, rendered, structural_helper, tuple_compare_statements,
};
use crate::builtin::return_types::partial_ord_return_type;
use crate::macros::{ts_macro_derive, ts_template};
use crate::ts_syn::abi::ir::type_registry::{ResolvedTypeRef, TypeRegistry};
use crate::ts_syn::ts_ident;
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};
use std::collections::HashMap;

#[ts_macro_derive(
    PartialOrd,
    description = "Generates a partialCompare() method for partial ordering (returns number | null: -1, 0, 1, or null)",
    attributes(ord)
)]
pub fn derive_partial_ord_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);
    let resolved_fields = input.context.resolved_fields.as_ref();
    let type_registry = &input.context.type_registry;

    match &input.data {
        Data::Class(class) => {
            let class_fields = derive_class_fields(&input, class, "PartialOrd", ClassUse::Reads)?;
            let class_name = input.name();
            let class_names = TypeNames::new(class_name, class.type_params());
            let class_ident = class_names.full_type_ident();

            // Collect fields for comparison
            let ord_fields: Vec<OrdField> = class_fields
                .iter()
                .filter_map(|field| {
                    let opts = CompareFieldOptions::from_decorators(&field.decorators, "ord");
                    if opts.skip {
                        return None;
                    }
                    Some(OrdField {
                        name: field.name.clone(),
                        ts_type: field.ts_type.clone(),
                        optional: field.optional,
                    })
                })
                .collect();

            let has_fields = !ord_fields.is_empty();

            let body = if has_fields {
                let compare_body =
                    partial_compare_steps(&ord_fields, resolved_fields, type_registry);
                ts_template! {
                    if (a === b) return 0;
                    {$typescript compare_body}
                    return 0;
                }
            } else {
                ts_template! {
                    return 0;
                }
            };
            let both = rendered(ts_template! { a: @{&class_ident}, b: @{&class_ident} });
            Ok(class_names.class_function(
                "partialCompare",
                "PartialCompare",
                &both,
                "a, b",
                partial_ord_return_type(),
                body,
            ))
        }
        Data::Enum(enum_data) => {
            let enum_name = input.name();
            let fn_name_ident = ts_ident!("{}PartialCompare", enum_name.to_case(Case::Camel));
            let value_kinds = EnumValueKinds::of(enum_data.variants());

            // Get return type
            let return_type = partial_ord_return_type();
            let return_type_ident = ts_ident!(return_type);

            let result = ts_template! {
                export function @{fn_name_ident}(a: @{ts_ident!(enum_name)}, b: @{ts_ident!(enum_name)}): @{return_type_ident} {
                    {#if value_kinds.numbers}
                        if (typeof a === "number" && typeof b === "number") {
                            return a < b ? -1 : a > b ? 1 : 0;
                        }
                    {/if}
                    {#if value_kinds.strings}
                        if (typeof a === "string" && typeof b === "string") {
                            return a < b ? -1 : a > b ? 1 : 0;
                        }
                    {/if}
                    {#if value_kinds.strings && value_kinds.numbers}
                        // A number and a string: numbers order first, as `Ord` has them.
                        return typeof a === "number" ? -1 : 1;
                    {:else}
                        return 0;
                    {/if}
                }
            };

            Ok(result)
        }
        Data::Interface(interface) => {
            let interface_fields = derive_interface_fields(&input, interface, "PartialOrd")?;
            let interface_names = TypeNames::new(input.name(), interface.type_params());
            let interface_ident = interface_names.full_type_ident();

            let ord_fields: Vec<OrdField> = interface_fields
                .iter()
                .filter_map(|field| {
                    let opts = CompareFieldOptions::from_decorators(&field.decorators, "ord");
                    if opts.skip {
                        return None;
                    }
                    Some(OrdField {
                        name: field.name.clone(),
                        ts_type: field.ts_type.clone(),
                        optional: field.optional,
                    })
                })
                .collect();

            let has_fields = !ord_fields.is_empty();

            // Get return type
            let return_type = partial_ord_return_type();
            let return_type_ident = ts_ident!(return_type);

            let fn_name_ident = interface_names.generic_function("PartialCompare");

            let result = if has_fields {
                let compare_body =
                    partial_compare_steps(&ord_fields, resolved_fields, type_registry);

                ts_template! {
                    export function @{fn_name_ident}(a: @{interface_ident}, b: @{interface_ident}): @{return_type_ident} {
                        if (a === b) return 0;
                        {$typescript compare_body}
                        return 0;
                    }
                }
            } else {
                ts_template! {
                    export function @{fn_name_ident}(a: @{interface_ident}, b: @{interface_ident}): @{return_type_ident} {
                        if (a === b) return 0;
                        return 0;
                    }
                }
            };

            Ok(result)
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();
            let type_names = TypeNames::new(type_name, type_alias.type_params());
            let type_ident = type_names.full_type_ident();

            // Get return type
            let return_type = partial_ord_return_type();
            let return_type_ident = ts_ident!(return_type);

            let effective_fields =
                crate::builtin::derive::common::get_effective_fields(type_alias, type_registry);
            if let Some(ref effective_fields) = effective_fields {
                let ord_fields: Vec<OrdField> = effective_fields
                    .iter()
                    .filter_map(|field| {
                        let opts = CompareFieldOptions::from_decorators(&field.decorators, "ord");
                        if opts.skip {
                            return None;
                        }
                        Some(OrdField {
                            name: field.name.clone(),
                            ts_type: field.ts_type.clone(),
                            optional: field.optional,
                        })
                    })
                    .collect();

                let has_fields = !ord_fields.is_empty();

                let fn_name_ident = type_names.generic_function("PartialCompare");

                let result = if has_fields {
                    let compare_body =
                        partial_compare_steps(&ord_fields, resolved_fields, type_registry);

                    ts_template! {
                        export function @{fn_name_ident}(a: @{type_ident}, b: @{type_ident}): @{return_type_ident} {
                            if (a === b) return 0;
                            {$typescript compare_body}
                            return 0;
                        }
                    }
                } else {
                    ts_template! {
                        export function @{fn_name_ident}(a: @{type_ident}, b: @{type_ident}): @{return_type_ident} {
                            if (a === b) return 0;
                            return 0;
                        }
                    }
                };

                Ok(result)
            } else if let Some(base) = type_alias.body().primitive_base() {
                let fn_name_ident = type_names.generic_function("PartialCompare");
                let compare_body =
                    crate::builtin::derive::common::primitive_compare_statements(base, true);
                Ok(ts_template! {
                    export function @{fn_name_ident}(a: @{type_ident}, b: @{type_ident}): @{return_type_ident} {
                        {$typescript TsStream::from_string(compare_body)}
                    }
                })
            } else if let Some(elements) = type_alias.body().as_tuple() {
                let fn_name_ident = type_names.generic_function("PartialCompare");
                let compare_body = tuple_compare_statements(elements, |ts_type, left, right| {
                    generate_value_order(
                        Ordering::Partial,
                        ts_type,
                        left,
                        right,
                        None,
                        type_registry,
                    )
                });
                Ok(ts_template! {
                    export function @{fn_name_ident}(a: @{type_ident}, b: @{type_ident}): @{return_type_ident} {
                        if (a === b) return 0;
                        {$typescript TsStream::from_string(compare_body)}
                    }
                })
            } else {
                let fn_name_ident = type_names.generic_function("PartialCompare");
                let order_src = match type_alias.body().as_alias() {
                    Some(ts_type) => generate_value_order(
                        Ordering::Partial,
                        ts_type,
                        "a",
                        "b",
                        None,
                        type_registry,
                    ),
                    None => {
                        let helper = structural_helper("structuralCompare");
                        rendered(ts_template! { @{helper}(a, b) })
                    }
                };
                let order = Expr::parse(&order_src).map_err(|err| {
                    MacroforgeError::new(
                        input.decorator_span(),
                        format!("@derive(PartialOrd): invalid comparison expression: {err:?}"),
                    )
                })?;
                Ok(ts_template! {
                    export function @{fn_name_ident}(a: @{type_ident}, b: @{type_ident}): @{return_type_ident} {
                        return @{order};
                    }
                })
            }
        }
    }
}

/// Statements returning the first field comparison of `a` and `b` that is not
/// `0`, or `null` as soon as one is.
fn partial_compare_steps(
    ord_fields: &[OrdField],
    resolved_fields: Option<&HashMap<String, ResolvedTypeRef>>,
    registry: &TypeRegistry,
) -> TsStream {
    TsStream::merge_all(ord_fields.iter().enumerate().map(|(index, field)| {
        let cmp = ts_ident!("cmp{}", index);
        let resolved = resolved_fields.and_then(|fields| fields.get(&field.name));
        let order = generate_field_order(Ordering::Partial, field, "a", "b", resolved, registry);
        ts_template! {
            const @{&cmp} = @{order};
            if (@{&cmp} === null) return null;
            if (@{&cmp} !== 0) return @{&cmp};
        }
    }))
}
