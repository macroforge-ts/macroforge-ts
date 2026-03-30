use convert_case::{Case, Casing};

use crate::macros::{ts_macro_derive, ts_template};
use crate::swc_ecma_ast::Expr;
use crate::ts_syn::ts_ident;
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

use super::debug_generation::debug_value_expr;
use super::types::{DebugField, DebugFieldOptions};

#[ts_macro_derive(
    Debug,
    description = "Generates a toString() method for debugging",
    attributes((debug, "Configure debug output for this field. Options: skip (exclude from output), rename (custom label)"))
)]
pub fn derive_debug_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);
    let resolved_fields = input.context.resolved_fields.as_ref();
    let type_registry = input.context.type_registry.as_ref();

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let class_ident = ts_ident!(class_name);

            // Collect fields that should be included in debug output
            let debug_fields: Vec<DebugField> = class
                .fields()
                .iter()
                .filter_map(|field| {
                    let opts = DebugFieldOptions::from_decorators(&field.decorators);
                    if opts.skip {
                        return None;
                    }
                    let label = opts.rename.unwrap_or_else(|| field.name.clone());
                    Some((label, field.name.clone(), field.ts_type.clone()))
                })
                .collect();

            // Generate function name (always prefix style)
            let fn_name_ident = ts_ident!("{}ToString", class_name.to_case(Case::Camel));
            let fn_name_expr: Expr = fn_name_ident.clone().into();

            // Build push statements with type-aware value expressions
            let mut push_stmts = String::new();
            for (label, name, ts_type) in &debug_fields {
                let resolved = resolved_fields.and_then(|rf| rf.get(name));
                let val_expr = debug_value_expr(name, ts_type, "value", resolved, type_registry);
                push_stmts.push_str(&format!("parts.push(\"{label}: \" + {val_expr});\n"));
            }

            // Generate standalone function with value parameter
            let standalone = if debug_fields.is_empty() {
                ts_template! {
                    export function @{fn_name_ident}(value: @{class_ident.clone()}): string {
                        return "@{class_name} {}";
                    }
                }
            } else {
                ts_template! {
                    export function @{fn_name_ident}(value: @{class_ident.clone()}): string {
                        const parts: string[] = [];
                        {$typescript TsStream::from_string(push_stmts)}
                        return "@{class_name} { " + parts.join(", ") + " }";
                    }
                }
            };

            // Generate static wrapper method that delegates to standalone function
            let class_body = ts_template!(Within {
                static toString(value: @{class_ident.clone()}): string {
                    return @{fn_name_expr}(value);
                }
            });

            // Combine standalone function with class body using {$typescript}
            // The standalone output (no marker) must come FIRST so it defaults to "below" (after class)
            // The body! output has /* @macroforge:body */ marker for class body insertion
            Ok(ts_template! {
                {$typescript standalone}
                {$typescript class_body}
            })
        }
        Data::Enum(enum_data) => {
            let enum_name = input.name();
            let enum_ident = ts_ident!(enum_name);
            let variants: Vec<String> = enum_data
                .variants()
                .iter()
                .map(|v| v.name.clone())
                .collect();

            let fn_name_ident = ts_ident!("{}ToString", enum_name.to_case(Case::Camel));
            // Convert ident to expression for array access
            let enum_expr: Expr = enum_ident.clone().into();
            Ok(ts_template! {
                export function @{fn_name_ident}(value: @{enum_ident.clone()}): string {
                    {#if !variants.is_empty()}
                        const key = @{enum_expr.clone()}[value as unknown as keyof typeof @{enum_ident.clone()}];
                        if (key !== undefined) {
                            return "@{enum_name}." + key;
                        }
                        return "@{enum_name}(" + String(value) + ")";
                    {:else}
                        return "@{enum_name}(" + String(value) + ")";
                    {/if}
                }
            })
        }
        Data::Interface(interface) => {
            let interface_name = input.name();
            let interface_ident = ts_ident!(interface_name);

            // Collect fields that should be included in debug output
            let debug_fields: Vec<DebugField> = interface
                .fields()
                .iter()
                .filter_map(|field| {
                    let opts = DebugFieldOptions::from_decorators(&field.decorators);
                    if opts.skip {
                        return None;
                    }
                    let label = opts.rename.unwrap_or_else(|| field.name.clone());
                    Some((label, field.name.clone(), field.ts_type.clone()))
                })
                .collect();

            let fn_name_ident = ts_ident!("{}ToString", interface_name.to_case(Case::Camel));

            if debug_fields.is_empty() {
                Ok(ts_template! {
                    export function @{fn_name_ident}(value: @{interface_ident.clone()}): string {
                        return "@{interface_name} {}";
                    }
                })
            } else {
                let mut push_stmts = String::new();
                for (label, name, ts_type) in &debug_fields {
                    let resolved = resolved_fields.and_then(|rf| rf.get(name));
                    let val_expr =
                        debug_value_expr(name, ts_type, "value", resolved, type_registry);
                    push_stmts.push_str(&format!("parts.push(\"{label}: \" + {val_expr});\n"));
                }

                Ok(ts_template! {
                    export function @{fn_name_ident}(value: @{interface_ident.clone()}): string {
                        const parts: string[] = [];
                        {$typescript TsStream::from_string(push_stmts)}
                        return "@{interface_name} { " + parts.join(", ") + " }";
                    }
                })
            }
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();
            let type_ident = ts_ident!(type_name);

            // Generate different output based on type body
            if type_alias.is_object() {
                // Object type: show fields
                let debug_fields: Vec<DebugField> = type_alias
                    .as_object()
                    .unwrap()
                    .iter()
                    .filter_map(|field| {
                        let opts = DebugFieldOptions::from_decorators(&field.decorators);
                        if opts.skip {
                            return None;
                        }
                        let label = opts.rename.unwrap_or_else(|| field.name.clone());
                        Some((label, field.name.clone(), field.ts_type.clone()))
                    })
                    .collect();

                let fn_name_ident = ts_ident!("{}ToString", type_name.to_case(Case::Camel));

                if debug_fields.is_empty() {
                    Ok(ts_template! {
                        export function @{fn_name_ident}(value: @{type_ident.clone()}): string {
                            return "@{type_name} {}";
                        }
                    })
                } else {
                    let mut push_stmts = String::new();
                    for (label, name, ts_type) in &debug_fields {
                        let resolved = resolved_fields.and_then(|rf| rf.get(name));
                        let val_expr =
                            debug_value_expr(name, ts_type, "value", resolved, type_registry);
                        push_stmts.push_str(&format!("parts.push(\"{label}: \" + {val_expr});\n"));
                    }

                    Ok(ts_template! {
                        export function @{fn_name_ident}(value: @{type_ident.clone()}): string {
                            const parts: string[] = [];
                            {$typescript TsStream::from_string(push_stmts)}
                            return "@{type_name} { " + parts.join(", ") + " }";
                        }
                    })
                }
            } else {
                // Union, intersection, tuple, or simple alias: use JSON.stringify
                let fn_name_ident = ts_ident!("{}ToString", type_name.to_case(Case::Camel));

                Ok(ts_template! {
                    export function @{fn_name_ident}(value: @{type_ident.clone()}): string {
                        return "@{type_name}(" + JSON.stringify(value) + ")";
                    }
                })
            }
        }
    }
}
