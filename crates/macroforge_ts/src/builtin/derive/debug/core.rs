use convert_case::{Case, Casing};

use crate::ast::Expr;
use crate::builtin::derive::common::TypeNames;
use crate::macros::{ts_macro_derive, ts_template};
use crate::ts_syn::ts_ident;
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

use crate::builtin::derive::common::{is_primitive_union, structural_call};

use super::debug_generation::{debug_value_expr, push_statements};
use super::types::DebugField;

#[ts_macro_derive(
    Debug,
    description = "Generates a toString() method for debugging",
    attributes((debug, "Configure debug output for this field. Options: skip (exclude from output), rename (custom label)"))
)]
pub fn derive_debug_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);
    let resolved_fields = input.context.resolved_fields.as_ref();
    let type_registry = &input.context.type_registry;

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let class_names = TypeNames::new(class_name, class.type_params());
            let class_ident = class_names.full_type_ident();

            let debug_fields: Vec<DebugField> = class
                .fields()
                .iter()
                .filter_map(|field| {
                    DebugField::shown(
                        &field.name,
                        &field.ts_type,
                        field.optional,
                        &field.decorators,
                    )
                })
                .collect();

            // Generate function name (always prefix style)
            let fn_name_ident = class_names.generic_function("ToString");
            let fn_name_expr: Expr = ts_ident!(class_names.function("ToString")).into();

            let push_stmts = push_statements(&debug_fields, resolved_fields, type_registry);

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
                static @{class_names.generic_method("toString")}(value: @{class_ident.clone()}): string {
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
            let interface_names = TypeNames::new(input.name(), interface.type_params());
            let interface_ident = interface_names.full_type_ident();

            let debug_fields: Vec<DebugField> = interface
                .fields()
                .iter()
                .filter_map(|field| {
                    DebugField::shown(
                        &field.name,
                        &field.ts_type,
                        field.optional,
                        &field.decorators,
                    )
                })
                .collect();

            let fn_name_ident = interface_names.generic_function("ToString");

            if debug_fields.is_empty() {
                Ok(ts_template! {
                    export function @{fn_name_ident}(value: @{interface_ident.clone()}): string {
                        return "@{interface_name} {}";
                    }
                })
            } else {
                let push_stmts = push_statements(&debug_fields, resolved_fields, type_registry);

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
            let type_names = TypeNames::new(type_name, type_alias.type_params());
            let type_ident = type_names.full_type_ident();

            // Generate different output based on type body
            let effective_fields =
                crate::builtin::derive::common::get_effective_fields(type_alias, type_registry);
            if let Some(ref effective_fields) = effective_fields {
                // Object-like type (Object or flattened Intersection): show fields
                let debug_fields: Vec<DebugField> = effective_fields
                    .iter()
                    .filter_map(|field| {
                        DebugField::shown(
                            &field.name,
                            &field.ts_type,
                            field.optional,
                            &field.decorators,
                        )
                    })
                    .collect();

                let fn_name_ident = type_names.generic_function("ToString");

                if debug_fields.is_empty() {
                    Ok(ts_template! {
                        export function @{fn_name_ident}(value: @{type_ident.clone()}): string {
                            return "@{type_name} {}";
                        }
                    })
                } else {
                    let push_stmts = push_statements(&debug_fields, resolved_fields, type_registry);

                    Ok(ts_template! {
                        export function @{fn_name_ident}(value: @{type_ident.clone()}): string {
                            const parts: string[] = [];
                            {$typescript TsStream::from_string(push_stmts)}
                            return "@{type_name} { " + parts.join(", ") + " }";
                        }
                    })
                }
            } else {
                let fn_name_ident = type_names.generic_function("ToString");
                let rendered_src = match type_alias.body().as_alias() {
                    Some(ts_type) => debug_value_expr(ts_type, "value", None, type_registry),
                    None if type_alias.body().primitive_base().is_some()
                        || is_primitive_union(type_alias.body()) =>
                    {
                        "String(value)".to_string()
                    }
                    None => structural_call("structuralDebug", "value"),
                };
                let rendered = Expr::parse(&rendered_src).map_err(|err| {
                    MacroforgeError::new(
                        input.decorator_span(),
                        format!("@derive(Debug): invalid debug expression: {err:?}"),
                    )
                })?;
                Ok(ts_template! {
                    export function @{fn_name_ident}(value: @{type_ident.clone()}): string {
                        return "@{type_name}(" + @{rendered} + ")";
                    }
                })
            }
        }
    }
}
