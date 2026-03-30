use convert_case::{Case, Casing};

use crate::builtin::derive_common::CompareFieldOptions;
use crate::macros::{ts_macro_derive, ts_template};
use crate::swc_ecma_ast::{Expr, Ident};
use crate::ts_syn::{
    Data, DeriveInput, MacroforgeError, TsStream, parse_ts_expr, parse_ts_macro_input, ts_ident,
};

use super::comparison::generate_field_compare_for_interface;
use super::types::OrdField;

#[ts_macro_derive(
    Ord,
    description = "Generates a compareTo() method for total ordering (returns -1, 0, or 1, never null)",
    attributes(ord)
)]
pub fn derive_ord_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);
    let resolved_fields = input.context.resolved_fields.as_ref();
    let type_registry = input.context.type_registry.as_ref();

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let class_ident = ts_ident!(class_name);

            // Collect fields for comparison
            let ord_fields: Vec<OrdField> = class
                .fields()
                .iter()
                .filter_map(|field| {
                    let opts = CompareFieldOptions::from_decorators(&field.decorators, "ord");
                    if opts.skip {
                        return None;
                    }
                    Some(OrdField {
                        name: field.name.clone(),
                        ts_type: field.ts_type.clone(),
                    })
                })
                .collect();

            // Generate function name (always prefix style)
            let fn_name_ident = ts_ident!("{}Compare", class_name.to_case(Case::Camel));
            let fn_name_expr: Expr = fn_name_ident.clone().into();

            // Generate standalone function with two parameters
            let standalone = if !ord_fields.is_empty() {
                let compare_steps: Vec<(Ident, Expr)> = ord_fields
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        let cmp_ident = ts_ident!(format!("cmp{}", i));
                        let resolved = resolved_fields.and_then(|rf| rf.get(&f.name));
                        let expr_src = generate_field_compare_for_interface(
                            f,
                            "a",
                            "b",
                            resolved,
                            type_registry,
                        );
                        let expr = parse_ts_expr(&expr_src).map_err(|err| {
                            MacroforgeError::new(
                                input.decorator_span(),
                                format!(
                                    "@derive(Ord): invalid comparison expression for '{}': {err:?}",
                                    f.name
                                ),
                            )
                        })?;
                        Ok((cmp_ident, *expr))
                    })
                    .collect::<Result<_, MacroforgeError>>()?;

                // Explicitly use compare_steps to satisfy clippy (it's consumed by ts_template! macro)
                let _ = &compare_steps;

                ts_template! {
                    export function @{fn_name_ident}(a: @{class_ident}, b: @{class_ident}): number {
                        if (a === b) return 0;
                        {#for (cmp_ident, cmp_expr) in &compare_steps}
                            const @{cmp_ident.clone()} = @{cmp_expr.clone()};
                            if (@{cmp_ident.clone()} !== 0) return @{cmp_ident.clone()};
                        {/for}
                        return 0;
                    }
                }
            } else {
                ts_template! {
                    export function @{fn_name_ident}(a: @{class_ident}, b: @{class_ident}): number {
                        if (a === b) return 0;
                        return 0;
                    }
                }
            };

            // Generate static wrapper method that delegates to standalone function
            let class_body = ts_template!(Within {
                static compareTo(a: @{class_ident}, b: @{class_ident}): number {
                    return @{fn_name_expr}(a, b);
                }
            });

            // Combine standalone function with class body
            // The standalone output (no marker) must come FIRST so it defaults to "below" (after class)
            Ok(standalone.merge(class_body))
        }
        Data::Enum(_) => {
            let enum_name = input.name();
            let fn_name_ident = ts_ident!("{}Compare", enum_name.to_case(Case::Camel));

            Ok(ts_template! {
                export function @{fn_name_ident}(a: @{ts_ident!(enum_name)}, b: @{ts_ident!(enum_name)}): number {
                    // For enums, compare by value (numeric enums) or string
                    if (typeof a === "number" && typeof b === "number") {
                        return a < b ? -1 : a > b ? 1 : 0;
                    }
                    if (typeof a === "string" && typeof b === "string") {
                        const cmp = a.localeCompare(b);
                        return cmp < 0 ? -1 : cmp > 0 ? 1 : 0;
                    }
                    return 0;
                }
            })
        }
        Data::Interface(interface) => {
            let interface_name = input.name();
            let interface_ident = ts_ident!(interface_name);

            let ord_fields: Vec<OrdField> = interface
                .fields()
                .iter()
                .filter_map(|field| {
                    let opts = CompareFieldOptions::from_decorators(&field.decorators, "ord");
                    if opts.skip {
                        return None;
                    }
                    Some(OrdField {
                        name: field.name.clone(),
                        ts_type: field.ts_type.clone(),
                    })
                })
                .collect();

            let fn_name_ident = ts_ident!("{}Compare", interface_name.to_case(Case::Camel));

            if !ord_fields.is_empty() {
                let compare_steps: Vec<(Ident, Expr)> = ord_fields
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        let cmp_ident = ts_ident!(format!("cmp{}", i));
                        let resolved = resolved_fields.and_then(|rf| rf.get(&f.name));
                        let expr_src = generate_field_compare_for_interface(
                            f,
                            "a",
                            "b",
                            resolved,
                            type_registry,
                        );
                        let expr = parse_ts_expr(&expr_src).map_err(|err| {
                            MacroforgeError::new(
                                input.decorator_span(),
                                format!(
                                    "@derive(Ord): invalid comparison expression for '{}': {err:?}",
                                    f.name
                                ),
                            )
                        })?;
                        Ok((cmp_ident, *expr))
                    })
                    .collect::<Result<_, MacroforgeError>>()?;

                // Explicitly use compare_steps to satisfy clippy (it's consumed by ts_template! macro)
                let _ = &compare_steps;

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
            let type_ident = ts_ident!(type_name);

            if type_alias.is_object() {
                let ord_fields: Vec<OrdField> = type_alias
                    .as_object()
                    .unwrap()
                    .iter()
                    .filter_map(|field| {
                        let opts = CompareFieldOptions::from_decorators(&field.decorators, "ord");
                        if opts.skip {
                            return None;
                        }
                        Some(OrdField {
                            name: field.name.clone(),
                            ts_type: field.ts_type.clone(),
                        })
                    })
                    .collect();

                let fn_name_ident = ts_ident!("{}Compare", type_name.to_case(Case::Camel));

                if !ord_fields.is_empty() {
                    let compare_steps: Vec<(Ident, Expr)> = ord_fields
                        .iter()
                        .enumerate()
                        .map(|(i, f)| {
                            let cmp_ident = ts_ident!(format!("cmp{}", i));
                            let resolved = resolved_fields.and_then(|rf| rf.get(&f.name));
                        let expr_src = generate_field_compare_for_interface(
                            f, "a", "b", resolved, type_registry,
                        );
                            let expr = parse_ts_expr(&expr_src).map_err(|err| {
                                MacroforgeError::new(
                                    input.decorator_span(),
                                    format!(
                                        "@derive(Ord): invalid comparison expression for '{}': {err:?}",
                                        f.name
                                    ),
                                )
                            })?;
                            Ok((cmp_ident, *expr))
                        })
                        .collect::<Result<_, MacroforgeError>>()?;

                    // Explicitly use compare_steps to satisfy clippy (it's consumed by ts_template! macro)
                    let _ = &compare_steps;

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
            } else {
                // Union, tuple, or simple alias: basic comparison
                let fn_name_ident = ts_ident!("{}Compare", type_name.to_case(Case::Camel));

                Ok(ts_template! {
                    export function @{fn_name_ident}(a: @{type_ident}, b: @{type_ident}): number {
                        if (a === b) return 0;
                        // For unions/tuples, try primitive comparison
                        if (typeof a === "number" && typeof b === "number") {
                            return a < b ? -1 : a > b ? 1 : 0;
                        }
                        if (typeof a === "string" && typeof b === "string") {
                            const cmp = a.localeCompare(b);
                            return cmp < 0 ? -1 : cmp > 0 ? 1 : 0;
                        }
                        return 0;
                    }
                })
            }
        }
    }
}
