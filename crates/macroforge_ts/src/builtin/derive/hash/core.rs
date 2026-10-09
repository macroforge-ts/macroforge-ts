use convert_case::{Case, Casing};

use crate::ast::Expr;
use crate::builtin::derive::common::TypeNames;
use crate::builtin::derive::common::{CompareFieldOptions, fixed_tuple, structural_call};
use crate::macros::{ts_macro_derive, ts_template};
use crate::ts_syn::abi::ir::TypeBody;
use crate::ts_syn::abi::ir::type_registry::TypeRegistry;
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input, ts_ident};

use super::hash_generation::{
    generate_field_hash_for_interface, generate_value_hash, primitive_hash_expr, tuple_hash_expr,
};
use super::types::HashField;

#[ts_macro_derive(
    Hash,
    description = "Generates a hashCode() method for hashing",
    attributes(hash)
)]
pub fn derive_hash_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);
    let resolved_fields = input.context.resolved_fields.as_ref();
    let type_registry = &input.context.type_registry;

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let class_names = TypeNames::new(class_name, class.type_params());
            let class_ident = class_names.full_type_ident();

            // Collect fields that should be included in hash
            let hash_fields: Vec<HashField> = class
                .fields()
                .iter()
                .filter_map(|field| {
                    let opts = CompareFieldOptions::from_decorators(&field.decorators, "hash");
                    if opts.skip {
                        return None;
                    }
                    Some(HashField {
                        name: field.name.clone(),
                        ts_type: field.ts_type.clone(),
                        optional: field.optional,
                    })
                })
                .collect();

            let has_fields = !hash_fields.is_empty();

            // Generate function name (always prefix style)
            let fn_name_ident = class_names.generic_function("HashCode");
            let fn_name_expr: Expr = ts_ident!(class_names.function("HashCode")).into();

            // Generate hash expressions for each field
            let hash_exprs: Vec<Expr> = hash_fields
                .iter()
                .map(|f| {
                    let resolved = resolved_fields.and_then(|rf| rf.get(&f.name));
                    let expr_src =
                        generate_field_hash_for_interface(f, "value", resolved, type_registry);
                    let expr = Expr::parse(&expr_src).map_err(|err| {
                        MacroforgeError::new(
                            input.decorator_span(),
                            format!(
                                "@derive(Hash): invalid hash expression for '{}': {err:?}",
                                f.name
                            ),
                        )
                    })?;
                    Ok(expr)
                })
                .collect::<Result<_, MacroforgeError>>()?;

            // Generate standalone function with value parameter
            let standalone = ts_template! {
                export function @{fn_name_ident}(value: @{class_ident.clone()}): number {
                    let hash = 17;
                    {#if has_fields}
                        {#for hash_expr in hash_exprs}
                            hash = (hash * 31 + (@{hash_expr})) | 0;
                        {/for}
                    {/if}
                    return hash;
                }
            };

            // Generate static wrapper method that delegates to standalone function
            let class_body = ts_template!(Within {
                static @{class_names.generic_method("hashCode")}(value: @{class_ident.clone()}): number {
                    return @{fn_name_expr}(value);
                }
            });

            // Combine standalone function with class body using {$typescript} for proper composition
            // The standalone output (no marker) must come FIRST so it defaults to "below" (after class)
            Ok(ts_template! {
                {$typescript standalone}
                {$typescript class_body}
            })
        }
        Data::Enum(enum_data) => {
            let enum_name = input.name();
            let fn_name_ident = ts_ident!("{}HashCode", enum_name.to_case(Case::Camel));

            // Check if all variants are string values
            let is_string_enum = enum_data.variants().iter().all(|v| v.value.is_string());

            if is_string_enum {
                // String enum: hash the string value
                Ok(ts_template! {
                    export function @{fn_name_ident}(value: @{ts_ident!(enum_name)}): number {
                        let hash = 0;
                        for (let i = 0; i < value.length; i++) {
                            hash = (hash * 31 + value.charCodeAt(i)) | 0;
                        }
                        return hash;
                    }
                })
            } else {
                // Numeric enum: use the number value directly
                Ok(ts_template! {
                    export function @{fn_name_ident}(value: @{ts_ident!(enum_name)}): number {
                        return value as number;
                    }
                })
            }
        }
        Data::Interface(interface) => {
            let interface_names = TypeNames::new(input.name(), interface.type_params());

            let hash_fields: Vec<HashField> = interface
                .fields()
                .iter()
                .filter_map(|field| {
                    let opts = CompareFieldOptions::from_decorators(&field.decorators, "hash");
                    if opts.skip {
                        return None;
                    }
                    Some(HashField {
                        name: field.name.clone(),
                        ts_type: field.ts_type.clone(),
                        optional: field.optional,
                    })
                })
                .collect();

            let has_fields = !hash_fields.is_empty();

            // Generate hash expressions for each field
            let hash_exprs: Vec<Expr> = hash_fields
                .iter()
                .map(|f| {
                    let resolved = resolved_fields.and_then(|rf| rf.get(&f.name));
                    let expr_src =
                        generate_field_hash_for_interface(f, "value", resolved, type_registry);
                    let expr = Expr::parse(&expr_src).map_err(|err| {
                        MacroforgeError::new(
                            input.decorator_span(),
                            format!(
                                "@derive(Hash): invalid hash expression for '{}': {err:?}",
                                f.name
                            ),
                        )
                    })?;
                    Ok(expr)
                })
                .collect::<Result<_, MacroforgeError>>()?;

            let fn_name_ident = interface_names.generic_function("HashCode");

            Ok(ts_template! {
                export function @{fn_name_ident}(value: @{interface_names.full_type_ident()}): number {
                    let hash = 17;
                    {#if has_fields}
                        {#for hash_expr in hash_exprs}
                            hash = (hash * 31 + (@{hash_expr})) | 0;
                        {/for}
                    {/if}
                    return hash;
                }
            })
        }
        Data::TypeAlias(type_alias) => {
            let type_names = TypeNames::new(input.name(), type_alias.type_params());

            let effective_fields =
                crate::builtin::derive::common::get_effective_fields(type_alias, type_registry);
            if let Some(ref effective_fields) = effective_fields {
                let hash_fields: Vec<HashField> = effective_fields
                    .iter()
                    .filter_map(|field| {
                        let opts = CompareFieldOptions::from_decorators(&field.decorators, "hash");
                        if opts.skip {
                            return None;
                        }
                        Some(HashField {
                            name: field.name.clone(),
                            ts_type: field.ts_type.clone(),
                            optional: field.optional,
                        })
                    })
                    .collect();

                let has_fields = !hash_fields.is_empty();

                // Generate hash expressions for each field
                let hash_exprs: Vec<Expr> = hash_fields
                    .iter()
                    .map(|f| {
                        let resolved = resolved_fields.and_then(|rf| rf.get(&f.name));
                        let expr_src =
                            generate_field_hash_for_interface(f, "value", resolved, type_registry);
                        let expr = Expr::parse(&expr_src).map_err(|err| {
                            MacroforgeError::new(
                                input.decorator_span(),
                                format!(
                                    "@derive(Hash): invalid hash expression for '{}': {err:?}",
                                    f.name
                                ),
                            )
                        })?;
                        Ok(expr)
                    })
                    .collect::<Result<_, MacroforgeError>>()?;

                let fn_name_ident = type_names.generic_function("HashCode");

                Ok(ts_template! {
                    export function @{fn_name_ident}(value: @{type_names.full_type_ident()}): number {
                        let hash = 17;
                        {#if has_fields}
                            {#for hash_expr in hash_exprs}
                                hash = (hash * 31 + (@{hash_expr})) | 0;
                            {/for}
                        {/if}
                        return hash;
                    }
                })
            } else {
                let fn_name_ident = type_names.generic_function("HashCode");
                let hash_src = alias_hash_expr(type_alias.body(), type_registry);
                let hash_expr = Expr::parse(&hash_src).map_err(|err| {
                    MacroforgeError::new(
                        input.decorator_span(),
                        format!("@derive(Hash): invalid hash expression: {err:?}"),
                    )
                })?;
                Ok(ts_template! {
                    export function @{fn_name_ident}(value: @{type_names.full_type_ident()}): number {
                        return @{hash_expr};
                    }
                })
            }
        }
    }
}

/// The hash of `value`, a value of a non-object alias with `body`.
fn alias_hash_expr(body: &TypeBody, registry: &TypeRegistry) -> String {
    if let Some(base) = body.primitive_base() {
        return primitive_hash_expr(base, "value");
    }
    if let Some(elements) = body.as_tuple().filter(|elements| fixed_tuple(elements)) {
        return tuple_hash_expr(elements, registry);
    }
    match body.as_alias() {
        Some(ts_type) => generate_value_hash(ts_type, "value", None, registry),
        None => structural_call("structuralHash", "value"),
    }
}
