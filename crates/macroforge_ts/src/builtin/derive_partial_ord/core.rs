use convert_case::{Case, Casing};

use crate::builtin::derive_common::CompareFieldOptions;
use crate::builtin::return_types::partial_ord_return_type;
use crate::macros::{ts_macro_derive, ts_template};
use crate::swc_ecma_ast::Expr;
use crate::ts_syn::ts_ident;
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

use super::comparison::generate_field_compare_for_interface;
use super::types::OrdField;

#[ts_macro_derive(
    PartialOrd,
    description = "Generates a compareTo() method for partial ordering (returns number | null: -1, 0, 1, or null)",
    attributes(ord)
)]
pub fn derive_partial_ord_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
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

            let has_fields = !ord_fields.is_empty();

            // Generate function name (always prefix style)
            let fn_name_ident = ts_ident!("{}PartialCompare", class_name.to_case(Case::Camel));
            let fn_name_expr: Expr = fn_name_ident.clone().into();

            // Get return type
            let return_type = partial_ord_return_type();
            let return_type_ident = ts_ident!(return_type);

            // Generate standalone function with two parameters
            let standalone = if has_fields {
                // Build comparison steps for each field
                let mut compare_body = String::new();
                for (i, f) in ord_fields.iter().enumerate() {
                    let cmp_var = format!("cmp{}", i);
                    let resolved = resolved_fields.and_then(|rf| rf.get(&f.name));
                    let expr_src = generate_field_compare_for_interface(
                        f,
                        "a",
                        "b",
                        true,
                        resolved,
                        type_registry,
                    );
                    compare_body.push_str(&format!(
                        "const {} = {};\nif ({} === null) return null;\nif ({} !== 0) return {};\n",
                        cmp_var, expr_src, cmp_var, cmp_var, cmp_var
                    ));
                }

                ts_template! {
                    export function @{fn_name_ident}(a: @{class_ident}, b: @{class_ident}): @{return_type_ident} {
                        if (a === b) return 0;
                        {$typescript TsStream::from_string(compare_body)}
                        return 0;
                    }
                }
            } else {
                ts_template! {
                    export function @{fn_name_ident}(a: @{class_ident}, b: @{class_ident}): @{return_type_ident} {
                        if (a === b) return 0;
                        return 0;
                    }
                }
            };

            // Generate static wrapper method that delegates to standalone function
            let class_body = ts_template!(Within {
                static compareTo(a: @{class_ident}, b: @{class_ident}): @{return_type_ident} {
                    return @{fn_name_expr}(a, b);
                }
            });

            // Combine standalone function with class body
            // The standalone output (no marker) must come FIRST so it defaults to "below" (after class)
            Ok(standalone.merge(class_body))
        }
        Data::Enum(_) => {
            let enum_name = input.name();
            let fn_name_ident = ts_ident!("{}PartialCompare", enum_name.to_case(Case::Camel));

            // Get return type
            let return_type = partial_ord_return_type();
            let return_type_ident = ts_ident!(return_type);

            let result = ts_template! {
                export function @{fn_name_ident}(a: @{ts_ident!(enum_name)}, b: @{ts_ident!(enum_name)}): @{return_type_ident} {
                    // For enums, compare by value (numeric enums) or string
                    if (typeof a === "number" && typeof b === "number") {
                        return a < b ? -1 : a > b ? 1 : 0;
                    }
                    if (typeof a === "string" && typeof b === "string") {
                        return a.localeCompare(b);
                    }
                    return a === b ? 0 : null;
                }
            };

            Ok(result)
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

            let has_fields = !ord_fields.is_empty();

            // Get return type
            let return_type = partial_ord_return_type();
            let return_type_ident = ts_ident!(return_type);

            let fn_name_ident = ts_ident!("{}PartialCompare", interface_name.to_case(Case::Camel));

            let result = if has_fields {
                // Build comparison steps for each field
                let mut compare_body = String::new();
                for (i, f) in ord_fields.iter().enumerate() {
                    let cmp_var = format!("cmp{}", i);
                    let resolved = resolved_fields.and_then(|rf| rf.get(&f.name));
                    let expr_src = generate_field_compare_for_interface(
                        f,
                        "a",
                        "b",
                        true,
                        resolved,
                        type_registry,
                    );
                    compare_body.push_str(&format!(
                        "const {} = {};\nif ({} === null) return null;\nif ({} !== 0) return {};\n",
                        cmp_var, expr_src, cmp_var, cmp_var, cmp_var
                    ));
                }

                ts_template! {
                    export function @{fn_name_ident}(a: @{interface_ident}, b: @{interface_ident}): @{return_type_ident} {
                        if (a === b) return 0;
                        {$typescript TsStream::from_string(compare_body)}
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
            let type_ident = ts_ident!(type_name);

            // Get return type
            let return_type = partial_ord_return_type();
            let return_type_ident = ts_ident!(return_type);

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

                let has_fields = !ord_fields.is_empty();

                let fn_name_ident = ts_ident!("{}PartialCompare", type_name.to_case(Case::Camel));

                let result = if has_fields {
                    // Build comparison steps for each field
                    let mut compare_body = String::new();
                    for (i, f) in ord_fields.iter().enumerate() {
                        let cmp_var = format!("cmp{}", i);
                        let resolved = resolved_fields.and_then(|rf| rf.get(&f.name));
                        let expr_src = generate_field_compare_for_interface(
                            f,
                            "a",
                            "b",
                            true,
                            resolved,
                            type_registry,
                        );
                        compare_body.push_str(&format!(
                            "const {} = {};\nif ({} === null) return null;\nif ({} !== 0) return {};\n",
                            cmp_var, expr_src, cmp_var, cmp_var, cmp_var
                        ));
                    }

                    ts_template! {
                        export function @{fn_name_ident}(a: @{type_ident}, b: @{type_ident}): @{return_type_ident} {
                            if (a === b) return 0;
                            {$typescript TsStream::from_string(compare_body)}
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
            } else {
                // Union, tuple, or simple alias: limited comparison
                let fn_name_ident = ts_ident!("{}PartialCompare", type_name.to_case(Case::Camel));

                let result = ts_template! {
                    export function @{fn_name_ident}(a: @{type_ident}, b: @{type_ident}): @{return_type_ident} {
                        if (a === b) return 0;
                        // For unions/tuples, try primitive comparison
                        if (typeof a === "number" && typeof b === "number") {
                            return a < b ? -1 : a > b ? 1 : 0;
                        }
                        if (typeof a === "string" && typeof b === "string") {
                            return a.localeCompare(b);
                        }
                        return null;
                    }
                };

                Ok(result)
            }
        }
    }
}
