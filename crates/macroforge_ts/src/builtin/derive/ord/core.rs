use convert_case::{Case, Casing};

use crate::ast::{Expr, Ident};
use crate::builtin::derive::common::TypeNames;
use crate::builtin::derive::common::{ClassUse, derive_class_fields, derive_interface_fields};
use crate::builtin::derive::common::{
    CompareFieldOptions, EnumValueKinds, OrdField, Ordering, generate_field_order,
    generate_value_order, rendered, structural_call, tuple_compare_statements,
};
use crate::macros::{ts_macro_derive, ts_template};
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input, ts_ident};

#[ts_macro_derive(
    Ord,
    description = "Generates a compare() method for total ordering (returns -1, 0, or 1, never null)",
    attributes(ord)
)]
pub fn derive_ord_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);
    let resolved_fields = input.context.resolved_fields.as_ref();
    let type_registry = &input.context.type_registry;

    match &input.data {
        Data::Class(class) => {
            let class_fields = derive_class_fields(&input, class, "Ord", ClassUse::Reads)?;
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

            let body = if !ord_fields.is_empty() {
                let compare_steps: Vec<(Ident, Expr)> = ord_fields
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        let cmp_ident = ts_ident!(format!("cmp{}", i));
                        let resolved = resolved_fields.and_then(|rf| rf.get(&f.name));
                        let expr_src = generate_field_order(
                            Ordering::Total,
                            f,
                            "a",
                            "b",
                            resolved,
                            type_registry,
                        );
                        let expr = Expr::parse(&expr_src).map_err(|err| {
                            MacroforgeError::new(
                                input.decorator_span(),
                                format!(
                                    "@derive(Ord): invalid comparison expression for '{}': {err:?}",
                                    f.name
                                ),
                            )
                        })?;
                        Ok((cmp_ident, expr))
                    })
                    .collect::<Result<_, MacroforgeError>>()?;

                ts_template! {
                    if (a === b) return 0;
                    {#for (cmp_ident, cmp_expr) in &compare_steps}
                        const @{cmp_ident.clone()} = @{cmp_expr.clone()};
                        if (@{cmp_ident.clone()} !== 0) return @{cmp_ident.clone()};
                    {/for}
                    return 0;
                }
            } else {
                ts_template! {
                    return 0;
                }
            };

            let both = rendered(ts_template! { a: @{&class_ident}, b: @{&class_ident} });
            Ok(class_names.class_function("compare", "Compare", &both, "a, b", "number", body))
        }
        Data::Enum(enum_data) => {
            let enum_name = input.name();
            let fn_name_ident = ts_ident!("{}Compare", enum_name.to_case(Case::Camel));
            let value_kinds = EnumValueKinds::of(enum_data.variants());

            Ok(ts_template! {
                export function @{fn_name_ident}(a: @{ts_ident!(enum_name)}, b: @{ts_ident!(enum_name)}): number {
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
                        // A number and a string: numbers order first.
                        return typeof a === "number" ? -1 : 1;
                    {:else}
                        return 0;
                    {/if}
                }
            })
        }
        Data::Interface(interface) => {
            let interface_fields = derive_interface_fields(&input, interface, "Ord")?;
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

            let fn_name_ident = interface_names.generic_function("Compare");

            if !ord_fields.is_empty() {
                let compare_steps: Vec<(Ident, Expr)> = ord_fields
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        let cmp_ident = ts_ident!(format!("cmp{}", i));
                        let resolved = resolved_fields.and_then(|rf| rf.get(&f.name));
                        let expr_src = generate_field_order(
                            Ordering::Total,
                            f,
                            "a",
                            "b",
                            resolved,
                            type_registry,
                        );
                        let expr = Expr::parse(&expr_src).map_err(|err| {
                            MacroforgeError::new(
                                input.decorator_span(),
                                format!(
                                    "@derive(Ord): invalid comparison expression for '{}': {err:?}",
                                    f.name
                                ),
                            )
                        })?;
                        Ok((cmp_ident, expr))
                    })
                    .collect::<Result<_, MacroforgeError>>()?;

                Ok(ts_template! {
                    export function @{fn_name_ident}(a: @{interface_ident}, b: @{interface_ident}): number {
                        if (a === b) return 0;
                        {#for (cmp_ident, cmp_expr) in &compare_steps}
                            const @{cmp_ident.clone()} = @{cmp_expr.clone()};
                            if (@{cmp_ident.clone()} !== 0) return @{cmp_ident.clone()};
                        {/for}
                        return 0;
                    }
                })
            } else {
                Ok(ts_template! {
                    export function @{fn_name_ident}(a: @{interface_ident}, b: @{interface_ident}): number {
                        if (a === b) return 0;
                        return 0;
                    }
                })
            }
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();
            let type_names = TypeNames::new(type_name, type_alias.type_params());
            let type_ident = type_names.full_type_ident();

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

                let fn_name_ident = type_names.generic_function("Compare");

                if !ord_fields.is_empty() {
                    let compare_steps: Vec<(Ident, Expr)> = ord_fields
                        .iter()
                        .enumerate()
                        .map(|(i, f)| {
                            let cmp_ident = ts_ident!(format!("cmp{}", i));
                            let resolved = resolved_fields.and_then(|rf| rf.get(&f.name));
                        let expr_src = generate_field_order(Ordering::Total, f, "a", "b", resolved, type_registry,
                        );
                            let expr = Expr::parse(&expr_src).map_err(|err| {
                                MacroforgeError::new(
                                    input.decorator_span(),
                                    format!(
                                        "@derive(Ord): invalid comparison expression for '{}': {err:?}",
                                        f.name
                                    ),
                                )
                            })?;
                            Ok((cmp_ident, expr))
                        })
                        .collect::<Result<_, MacroforgeError>>()?;

                    Ok(ts_template! {
                        export function @{fn_name_ident}(a: @{type_ident}, b: @{type_ident}): number {
                            if (a === b) return 0;
                            {#for (cmp_ident, cmp_expr) in &compare_steps}
                                const @{cmp_ident.clone()} = @{cmp_expr.clone()};
                                if (@{cmp_ident.clone()} !== 0) return @{cmp_ident.clone()};
                            {/for}
                            return 0;
                        }
                    })
                } else {
                    Ok(ts_template! {
                        export function @{fn_name_ident}(a: @{type_ident}, b: @{type_ident}): number {
                            if (a === b) return 0;
                            return 0;
                        }
                    })
                }
            } else if let Some(base) = type_alias.body().primitive_base() {
                let fn_name_ident = type_names.generic_function("Compare");
                let compare_body =
                    crate::builtin::derive::common::primitive_compare_statements(base, false);
                Ok(ts_template! {
                    export function @{fn_name_ident}(a: @{type_ident}, b: @{type_ident}): number {
                        {$typescript TsStream::from_string(compare_body)}
                    }
                })
            } else if let Some(elements) = type_alias.body().as_tuple() {
                let fn_name_ident = type_names.generic_function("Compare");
                let compare_body = tuple_compare_statements(elements, |ts_type, left, right| {
                    generate_value_order(Ordering::Total, ts_type, left, right, None, type_registry)
                });
                Ok(ts_template! {
                    export function @{fn_name_ident}(a: @{type_ident}, b: @{type_ident}): number {
                        if (a === b) return 0;
                        {$typescript TsStream::from_string(compare_body)}
                    }
                })
            } else {
                let fn_name_ident = type_names.generic_function("Compare");
                let order_src = match type_alias.body().as_alias() {
                    Some(ts_type) => generate_value_order(
                        Ordering::Total,
                        ts_type,
                        "a",
                        "b",
                        None,
                        type_registry,
                    ),
                    None => structural_call("structuralCompare", "a, b"),
                };
                let order = Expr::parse(&order_src).map_err(|err| {
                    MacroforgeError::new(
                        input.decorator_span(),
                        format!("@derive(Ord): invalid comparison expression: {err:?}"),
                    )
                })?;
                Ok(ts_template! {
                    export function @{fn_name_ident}(a: @{type_ident}, b: @{type_ident}): number {
                        return @{order};
                    }
                })
            }
        }
    }
}
