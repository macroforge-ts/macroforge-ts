use convert_case::{Case, Casing};

use crate::builtin::derive_common::CompareFieldOptions;
use crate::macros::{ts_macro_derive, ts_template};
use crate::swc_ecma_ast::Expr;
use crate::ts_syn::{
    Data, DeriveInput, MacroforgeError, TsStream, parse_ts_expr, parse_ts_macro_input, ts_ident,
};

use super::equality::generate_field_equality_for_interface;
use super::types::EqField;

#[ts_macro_derive(
    PartialEq,
    description = "Generates an equals() method for field-by-field comparison",
    attributes(partialEq)
)]
pub fn derive_partial_eq_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);
    let resolved_fields = input.context.resolved_fields.as_ref();
    let type_registry = input.context.type_registry.as_ref();

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let class_ident = ts_ident!(class_name);

            // Collect fields that should be included in equality comparison
            let eq_fields: Vec<EqField> = class
                .fields()
                .iter()
                .filter_map(|field| {
                    let opts = CompareFieldOptions::from_decorators(&field.decorators, "partialEq");
                    if opts.skip {
                        return None;
                    }
                    Some(EqField {
                        name: field.name.clone(),
                        ts_type: field.ts_type.clone(),
                    })
                })
                .collect();

            // Generate function name (always prefix style)
            let fn_name_ident = ts_ident!("{}Equals", class_name.to_case(Case::Camel));
            let fn_name_expr: Expr = fn_name_ident.clone().into();

            let comparison_src = if eq_fields.is_empty() {
                "true".to_string()
            } else {
                eq_fields
                    .iter()
                    .map(|f| {
                        let resolved = resolved_fields.and_then(|rf| rf.get(&f.name));
                        generate_field_equality_for_interface(f, "a", "b", resolved, type_registry)
                    })
                    .collect::<Vec<_>>()
                    .join(" && ")
            };
            let comparison_expr = parse_ts_expr(&comparison_src).map_err(|err| {
                MacroforgeError::new(
                    input.decorator_span(),
                    format!("@derive(PartialEq): invalid comparison expression: {err:?}"),
                )
            })?;

            // Generate standalone function with two parameters
            let standalone = ts_template! {
                export function @{fn_name_ident}(a: @{class_ident}, b: @{class_ident}): boolean {
                    if (a === b) return true;
                    return @{comparison_expr};
                }
            };

            // Generate static wrapper method that delegates to standalone function
            let class_body = ts_template!(Within {
                static equals(a: @{class_ident}, b: @{class_ident}): boolean {
                    return @{fn_name_expr}(a, b);
                }
            });

            // Combine standalone function with class body
            // The standalone output (no marker) must come FIRST so it defaults to "below" (after class)
            Ok(standalone.merge(class_body))
        }
        Data::Enum(_) => {
            // Enums: direct comparison with ===
            let enum_name = input.name();
            let fn_name_ident = ts_ident!("{}Equals", enum_name.to_case(Case::Camel));

            Ok(ts_template! {
                export function @{fn_name_ident}(a: @{ts_ident!(enum_name)}, b: @{ts_ident!(enum_name)}): boolean {
                    return a === b;
                }
            })
        }
        Data::Interface(interface) => {
            let interface_name = input.name();
            let interface_ident = ts_ident!(interface_name);

            // Collect fields for comparison
            let eq_fields: Vec<EqField> = interface
                .fields()
                .iter()
                .filter_map(|field| {
                    let opts = CompareFieldOptions::from_decorators(&field.decorators, "partialEq");
                    if opts.skip {
                        return None;
                    }
                    Some(EqField {
                        name: field.name.clone(),
                        ts_type: field.ts_type.clone(),
                    })
                })
                .collect();

            let comparison_src = if eq_fields.is_empty() {
                "true".to_string()
            } else {
                eq_fields
                    .iter()
                    .map(|f| {
                        let resolved = resolved_fields.and_then(|rf| rf.get(&f.name));
                        generate_field_equality_for_interface(f, "a", "b", resolved, type_registry)
                    })
                    .collect::<Vec<_>>()
                    .join(" && ")
            };
            let comparison_expr = parse_ts_expr(&comparison_src).map_err(|err| {
                MacroforgeError::new(
                    input.decorator_span(),
                    format!("@derive(PartialEq): invalid comparison expression: {err:?}"),
                )
            })?;

            let fn_name_ident = ts_ident!("{}Equals", interface_name.to_case(Case::Camel));

            Ok(ts_template! {
                export function @{fn_name_ident}(a: @{interface_ident}, b: @{interface_ident}): boolean {
                    if (a === b) return true;
                    return @{comparison_expr};
                }
            })
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();
            let type_ident = ts_ident!(type_name);

            if type_alias.is_object() {
                // Object type: field-by-field comparison
                let eq_fields: Vec<EqField> = type_alias
                    .as_object()
                    .unwrap()
                    .iter()
                    .filter_map(|field| {
                        let opts =
                            CompareFieldOptions::from_decorators(&field.decorators, "partialEq");
                        if opts.skip {
                            return None;
                        }
                        Some(EqField {
                            name: field.name.clone(),
                            ts_type: field.ts_type.clone(),
                        })
                    })
                    .collect();

                let comparison_src = if eq_fields.is_empty() {
                    "true".to_string()
                } else {
                    eq_fields
                        .iter()
                        .map(|f| {
                            let resolved = resolved_fields.and_then(|rf| rf.get(&f.name));
                            generate_field_equality_for_interface(
                                f,
                                "a",
                                "b",
                                resolved,
                                type_registry,
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(" && ")
                };
                let comparison_expr = parse_ts_expr(&comparison_src).map_err(|err| {
                    MacroforgeError::new(
                        input.decorator_span(),
                        format!("@derive(PartialEq): invalid comparison expression: {err:?}"),
                    )
                })?;

                let fn_name_ident = ts_ident!("{}Equals", type_name.to_case(Case::Camel));

                Ok(ts_template! {
                    export function @{fn_name_ident}(a: @{type_ident}, b: @{type_ident}): boolean {
                        if (a === b) return true;
                        return @{comparison_expr};
                    }
                })
            } else {
                // Union, tuple, or simple alias: use strict equality and JSON fallback
                let fn_name_ident = ts_ident!("{}Equals", type_name.to_case(Case::Camel));

                Ok(ts_template! {
                    export function @{fn_name_ident}(a: @{type_ident}, b: @{type_ident}): boolean {
                        if (a === b) return true;
                        if (typeof a === "object" && typeof b === "object" && a !== null && b !== null) {
                            return JSON.stringify(a) === JSON.stringify(b);
                        }
                        return false;
                    }
                })
            }
        }
    }
}
