use crate::builtin::derive::common::function_prefix;

use crate::ast::Expr;
use crate::builtin::derive::common::TypeNames;
use crate::builtin::derive::common::structural_callee;
use crate::builtin::derive::common::{
    ClassUse, derive_class_fields, derive_interface_fields, inherited_statics,
};
use crate::builtin::derive::common::{
    CompareFieldOptions, fixed_tuple, is_primitive_union, parameter_name, rendered, structural_call,
};
use crate::macros::{ts_macro_derive, ts_template};
use crate::ts_syn::abi::ir::TypeBody;
use crate::ts_syn::abi::ir::type_registry::{ResolvedTypeRef, TypeRegistry};
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input, ts_ident};
use std::collections::HashMap;

use super::equality::{
    generate_field_equality_for_interface, generate_value_equality, tuple_equality_statements,
};
use super::types::EqField;

/// Statements returning whether `a` and `b` are equal in each of `fields`,
/// each field compared in a function of its own: one expression over them
/// all nests as deep as the type is wide, and one body comparing each in turn
/// grows past what the checker reads.
fn fields_equal(
    fields: &[EqField],
    resolved_fields: Option<&HashMap<String, ResolvedTypeRef>>,
    type_registry: &TypeRegistry,
    input: &DeriveInput,
) -> Result<TsStream, MacroforgeError> {
    let mut comparisons = Vec::with_capacity(fields.len());
    for field in fields {
        let resolved = resolved_fields.and_then(|resolved| resolved.get(&field.name));
        let source =
            generate_field_equality_for_interface(field, "a", "b", resolved, type_registry)
                .map_err(|message| refused(input, message))?;
        comparisons.push(Expr::parse(&source).map_err(|err| {
            MacroforgeError::new(
                input.decorator_span(),
                format!("@derive(PartialEq): invalid comparison expression: {err}"),
            )
        })?);
    }
    Ok(match comparisons.as_slice() {
        [] => ts_template! { return true; },
        [comparison] => ts_template! { return @{comparison.clone()}; },
        _ => {
            let each = TsStream::merge_all(
                comparisons
                    .into_iter()
                    .map(|comparison| ts_template! { () => @{comparison}, }),
            );
            ts_template! {
                return [
                    {$typescript each}
                ].every((same) => same());
            }
        }
    })
}

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
            let class_fields = derive_class_fields(&input, class, "PartialEq", ClassUse::Reads)?;
            let class_name = input.name();
            let class_names = TypeNames::new(class_name, class.type_params())
                .inheriting(inherited_statics(&input, class, "PartialEq"));
            let class_ident = class_names.full_type_ident();

            // Collect fields that should be included in equality comparison
            let eq_fields: Vec<EqField> = class_fields
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

            let comparisons = fields_equal(&eq_fields, resolved_fields, type_registry, &input)?;

            let body = ts_template! {
                if (a === b) return true;
                return @{structural_callee("equalWithin")}(a, b, () => {
                        {$typescript comparisons}
                    });
            };
            let both = rendered(ts_template! { a: @{&class_ident}, b: @{&class_ident} });
            Ok(class_names.class_function("equals", "Equals", &both, "a, b", "boolean", body))
        }
        Data::Enum(_) => {
            // Enums: direct comparison with ===
            let enum_name = input.name();
            let fn_name_ident = ts_ident!("{}Equals", function_prefix(enum_name));

            Ok(ts_template! {
                export function @{fn_name_ident}(a: @{ts_ident!(enum_name)}, b: @{ts_ident!(enum_name)}): boolean {
                    return a === b;
                }
            })
        }
        Data::Interface(interface) => {
            let interface_fields = derive_interface_fields(&input, interface, "PartialEq")?;
            let interface_names = TypeNames::new(input.name(), interface.type_params());
            let interface_ident = interface_names.full_type_ident();

            // Collect fields for comparison
            let eq_fields: Vec<EqField> = interface_fields
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

            let comparisons = fields_equal(&eq_fields, resolved_fields, type_registry, &input)?;

            let fn_name_ident = interface_names.generic_function("Equals");

            Ok(ts_template! {
                export function @{fn_name_ident}(a: @{interface_ident}, b: @{interface_ident}): boolean {
                    if (a === b) return true;
                    return @{structural_callee("equalWithin")}(a, b, () => {
                        {$typescript comparisons}
                    });
                }
            })
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();
            let type_names = TypeNames::new(type_name, type_alias.type_params());
            let type_ident = type_names.full_type_ident();

            let effective_fields =
                crate::builtin::derive::common::derive_alias_fields(&input, type_alias);
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

                let comparisons = fields_equal(&eq_fields, resolved_fields, type_registry, &input)?;

                let fn_name_ident = type_names.generic_function("Equals");

                Ok(ts_template! {
                    export function @{fn_name_ident}(a: @{type_ident}, b: @{type_ident}): boolean {
                        if (a === b) return true;
                        return @{structural_callee("equalWithin")}(a, b, () => {
                        {$typescript comparisons}
                    });
                    }
                })
            } else {
                let fn_name_ident = type_names.generic_function("Equals");
                let body = alias_equality_statements(type_alias.body(), type_registry)
                    .map_err(|message| refused(&input, message))?;
                let left = ts_ident!(parameter_name("a", &body));
                let right = ts_ident!(parameter_name("b", &body));
                Ok(ts_template! {
                    export function @{fn_name_ident}(@{left}: @{type_ident}, @{right}: @{type_ident}): boolean {
                        {$typescript TsStream::from_string(body)}
                    }
                })
            }
        }
    }
}

/// Statements returning whether `a` and `b`, values of a non-object alias
/// with `body`, are equal.
fn alias_equality_statements(body: &TypeBody, registry: &TypeRegistry) -> Result<String, String> {
    if let Some(elements) = body.as_tuple().filter(|elements| fixed_tuple(elements)) {
        return tuple_equality_statements(elements, registry);
    }
    let equality = match body.as_alias() {
        Some(ts_type) => generate_value_equality(ts_type, "a", "b", None, registry)?,
        None if is_primitive_union(body) || body.primitive_base().is_some() => {
            "a === b".to_string()
        }
        None => structural_call("structuralEquals", "a, b"),
    };
    // Identity settles it before anything is compared, where it is not all
    // there is to compare.
    if equality == "a === b" {
        return Ok(rendered(ts_template! { return a === b; }));
    }
    Ok(rendered(ts_template! {
        if (a === b) return true;
        return @{equality};
    }))
}

/// The error `@derive(PartialEq)` on `input` gives for what it cannot compare.
fn refused(input: &DeriveInput, message: String) -> MacroforgeError {
    MacroforgeError::new(
        input.decorator_span(),
        format!("@derive(PartialEq): {message}"),
    )
}
