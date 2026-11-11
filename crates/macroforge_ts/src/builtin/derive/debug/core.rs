use crate::builtin::derive::common::function_prefix;

use crate::ast::Expr;
use crate::builtin::derive::common::TypeNames;
use crate::builtin::derive::common::structural_callee;
use crate::builtin::derive::common::{
    ClassUse, derive_class_fields, derive_interface_fields, inherited_statics,
};
use crate::macros::{ts_macro_derive, ts_template};
use crate::ts_syn::ts_ident;
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

use crate::builtin::derive::common::{
    is_primitive_union, js_string, parameter_name, rendered, structural_call,
};

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
            let class_fields = derive_class_fields(&input, class, "Debug", ClassUse::Reads)?;
            let class_name = input.name();
            let class_names = TypeNames::new(class_name, class.type_params())
                .inheriting(inherited_statics(&input, class, "Debug"));
            let class_ident = class_names.full_type_ident();

            let debug_fields: Vec<DebugField> = class_fields
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

            let push_stmts = push_statements(&debug_fields, resolved_fields, type_registry)
                .map_err(|message| refused(&input, message))?;
            let body = if debug_fields.is_empty() {
                ts_template! {
                    return "@{class_name} {}";
                }
            } else {
                ts_template! {
                    return @{structural_callee("debuggedWithin")}(value, () => {
                        const parts: string[] = [];
                    {$typescript TsStream::from_string(push_stmts)}
                        return "@{class_name} { " + parts.join(", ") + " }";
                    });
                }
            };
            let parameter = parameter_name("value", body.source());
            let value = rendered(ts_template! { @{ts_ident!(parameter.as_str())}: @{class_ident} });
            Ok(class_names
                .class_function("toString", "ToString", &value, &parameter, "string", body))
        }
        Data::Enum(enum_data) => {
            let enum_name = input.name();
            let enum_ident = ts_ident!(enum_name);
            // A member is found by its value: a string enum has no lookup
            // from value to name, and a `const enum` no object to look in.
            let members: Vec<(Expr, String)> = enum_data
                .variants()
                .iter()
                .map(|variant| {
                    let name = js_string(&variant.name);
                    (
                        Expr::Source(rendered(ts_template! { @{enum_name}[@{name}] })),
                        js_string(&format!("{enum_name}.{}", variant.name)),
                    )
                })
                .collect();

            let fn_name_ident = ts_ident!("{}ToString", function_prefix(enum_name));
            Ok(ts_template! {
                export function @{fn_name_ident}(value: @{enum_ident.clone()}): string {
                    {#if !members.is_empty()}
                        // Members that share a value are one type, which a
                        // test of `value` itself would narrow away.
                        const held: unknown = value;
                        {#for (member, label) in &members}
                            if (held === @{member}) {
                                return @{label};
                            }
                        {/for}
                    {/if}
                    return "@{enum_name}(" + String(value) + ")";
                }
            })
        }
        Data::Interface(interface) => {
            let interface_fields = derive_interface_fields(&input, interface, "Debug")?;
            let interface_name = input.name();
            let interface_names = TypeNames::new(input.name(), interface.type_params());
            let interface_ident = interface_names.full_type_ident();

            let debug_fields: Vec<DebugField> = interface_fields
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
                    export function @{fn_name_ident}(_value: @{interface_ident.clone()}): string {
                        return "@{interface_name} {}";
                    }
                })
            } else {
                let push_stmts = push_statements(&debug_fields, resolved_fields, type_registry)
                    .map_err(|message| refused(&input, message))?;

                Ok(ts_template! {
                    export function @{fn_name_ident}(value: @{interface_ident.clone()}): string {
                        return @{structural_callee("debuggedWithin")}(value, () => {
                            const parts: string[] = [];
                        {$typescript TsStream::from_string(push_stmts)}
                            return "@{interface_name} { " + parts.join(", ") + " }";
                        });
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
                crate::builtin::derive::common::derive_alias_fields(&input, type_alias);
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
                        export function @{fn_name_ident}(_value: @{type_ident.clone()}): string {
                            return "@{type_name} {}";
                        }
                    })
                } else {
                    let push_stmts = push_statements(&debug_fields, resolved_fields, type_registry)
                        .map_err(|message| refused(&input, message))?;

                    Ok(ts_template! {
                        export function @{fn_name_ident}(value: @{type_ident.clone()}): string {
                            return @{structural_callee("debuggedWithin")}(value, () => {
                                const parts: string[] = [];
                            {$typescript TsStream::from_string(push_stmts)}
                                return "@{type_name} { " + parts.join(", ") + " }";
                            });
                        }
                    })
                }
            } else {
                let fn_name_ident = type_names.generic_function("ToString");
                let rendered_src = match type_alias.body().as_alias() {
                    Some(ts_type) => debug_value_expr(ts_type, "value", None, type_registry)
                        .map_err(|message| refused(&input, message))?,
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
                        format!("@derive(Debug): invalid debug expression: {err}"),
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

/// The error `@derive(Debug)` on `input` gives for what it cannot print.
fn refused(input: &DeriveInput, message: String) -> MacroforgeError {
    MacroforgeError::new(input.decorator_span(), format!("@derive(Debug): {message}"))
}
