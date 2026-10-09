use convert_case::{Case, Casing};

use crate::ast::Expr;
use crate::builtin::derive::common::TypeNames;
use crate::builtin::derive::common::{
    CompareFieldOptions, fixed_tuple, is_primitive_union, rendered, structural_call,
};
use crate::macros::{ts_macro_derive, ts_template};
use crate::ts_syn::abi::ir::TypeBody;
use crate::ts_syn::abi::ir::type_registry::TypeRegistry;
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input, ts_ident};

use super::equality::{
    generate_field_equality_for_interface, generate_value_equality, tuple_equality_statements,
};
use super::types::EqField;

#[ts_macro_derive(
    PartialEq,
    description = "Generates an equals() method for field-by-field comparison",
    attributes(partialEq)
)]
pub fn derive_partial_eq_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);
    let resolved_fields = input.context.resolved_fields.as_ref();
    let type_registry = &input.context.type_registry;

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let class_names = TypeNames::new(class_name, class.type_params());
            let class_ident = class_names.full_type_ident();

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
                        optional: field.optional,
                    })
                })
                .collect();

            // Generate function name (always prefix style)
            let fn_name_ident = class_names.generic_function("Equals");
            let fn_name_expr: Expr = ts_ident!(class_names.function("Equals")).into();

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
            let comparison_expr = Expr::parse(&comparison_src).map_err(|err| {
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
                static @{class_names.generic_method("equals")}(a: @{class_ident}, b: @{class_ident}): boolean {
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
            let interface_names = TypeNames::new(input.name(), interface.type_params());
            let interface_ident = interface_names.full_type_ident();

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
                        optional: field.optional,
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
            let comparison_expr = Expr::parse(&comparison_src).map_err(|err| {
                MacroforgeError::new(
                    input.decorator_span(),
                    format!("@derive(PartialEq): invalid comparison expression: {err:?}"),
                )
            })?;

            let fn_name_ident = interface_names.generic_function("Equals");

            Ok(ts_template! {
                export function @{fn_name_ident}(a: @{interface_ident}, b: @{interface_ident}): boolean {
                    if (a === b) return true;
                    return @{comparison_expr};
                }
            })
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();
            let type_names = TypeNames::new(type_name, type_alias.type_params());
            let type_ident = type_names.full_type_ident();

            let effective_fields =
                crate::builtin::derive::common::get_effective_fields(type_alias, type_registry);
            if let Some(ref effective_fields) = effective_fields {
                // Object-like type: field-by-field comparison
                let eq_fields: Vec<EqField> = effective_fields
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
                            optional: field.optional,
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
                let comparison_expr = Expr::parse(&comparison_src).map_err(|err| {
                    MacroforgeError::new(
                        input.decorator_span(),
                        format!("@derive(PartialEq): invalid comparison expression: {err:?}"),
                    )
                })?;

                let fn_name_ident = type_names.generic_function("Equals");

                Ok(ts_template! {
                    export function @{fn_name_ident}(a: @{type_ident}, b: @{type_ident}): boolean {
                        if (a === b) return true;
                        return @{comparison_expr};
                    }
                })
            } else {
                let fn_name_ident = type_names.generic_function("Equals");
                let body = alias_equality_statements(type_alias.body(), type_registry);
                Ok(ts_template! {
                    export function @{fn_name_ident}(a: @{type_ident}, b: @{type_ident}): boolean {
                        if (a === b) return true;
                        {$typescript TsStream::from_string(body)}
                    }
                })
            }
        }
    }
}

/// Statements returning whether `a` and `b`, values of a non-object alias
/// with `body`, are equal.
fn alias_equality_statements(body: &TypeBody, registry: &TypeRegistry) -> String {
    if let Some(elements) = body.as_tuple().filter(|elements| fixed_tuple(elements)) {
        return tuple_equality_statements(elements, registry);
    }
    let equality = match body.as_alias() {
        Some(ts_type) => generate_value_equality(ts_type, "a", "b", None, registry),
        None if is_primitive_union(body) || body.primitive_base().is_some() => {
            "a === b".to_string()
        }
        None => structural_call("structuralEquals", "a, b"),
    };
    rendered(ts_template! { return @{equality}; })
}
