use convert_case::{Case, Casing};

use crate::builtin::derive_common::{DefaultFieldOptions, get_type_default, has_known_default};
use crate::macros::{ts_macro_derive, ts_template};
use crate::swc_ecma_ast::{Expr, Ident};
use crate::ts_syn::ts_ident;
use crate::ts_syn::{
    Data, DeriveInput, MacroforgeError, TsStream, emit_expr, parse_ts_expr, parse_ts_macro_input,
};

use super::types::{DefaultField, validate_default_fields};

#[ts_macro_derive(
    Default,
    description = "Generates a static defaultValue() factory method",
    attributes(default)
)]
pub fn derive_default_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);
    let type_registry = input.context.type_registry.as_ref();

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let class_ident = ts_ident!(class_name);
            let class_expr: Expr = class_ident.clone().into();

            // Validate fields against type registry (emit warnings for types missing Default)
            let fields_to_validate: Vec<(String, String)> = class
                .fields()
                .iter()
                .filter(|f| {
                    !f.optional && !DefaultFieldOptions::from_decorators(&f.decorators).has_default
                })
                .map(|f| (f.name.clone(), f.ts_type.clone()))
                .collect();
            validate_default_fields(&fields_to_validate, class_name, type_registry);

            // Check for required non-primitive fields missing @default (like Rust's derive(Default))
            let missing_defaults: Vec<&str> = class
                .fields()
                .iter()
                .filter(|field| {
                    // Skip optional fields
                    if field.optional {
                        return false;
                    }
                    // Skip if has explicit @default
                    if DefaultFieldOptions::from_decorators(&field.decorators).has_default {
                        return false;
                    }
                    // Skip if type has known default (primitives, collections, nullable)
                    if has_known_default(&field.ts_type) {
                        return false;
                    }
                    // This field needs @default but doesn't have it
                    true
                })
                .map(|f| f.name.as_str())
                .collect();

            if !missing_defaults.is_empty() {
                return Err(MacroforgeError::new(
                    input.decorator_span(),
                    format!(
                        "@derive(Default) cannot determine default for non-primitive fields. Add @default(value) to: {}",
                        missing_defaults.join(", ")
                    ),
                ));
            }

            // Build defaults for ALL non-optional fields by parsing expressions and generating class body
            // Parse all field default expressions upfront for validation (before template generation)
            let field_data: Vec<(Ident, Expr)> = class
                .fields()
                .iter()
                .filter(|field| !field.optional)
                .map(|field| {
                    let opts = DefaultFieldOptions::from_decorators(&field.decorators);
                    let default_value = opts
                        .value
                        .unwrap_or_else(|| get_type_default(&field.ts_type));

                    let value_expr = parse_ts_expr(&default_value).map_err(|err| {
                        MacroforgeError::new(
                            input.decorator_span(),
                            format!(
                                "@derive(Default): invalid default expression for '{}': {err:?}",
                                field.name
                            ),
                        )
                    })?;
                    Ok((ts_ident!(field.name.as_str()), *value_expr))
                })
                .collect::<Result<_, MacroforgeError>>()?;

            // Generate the method body using parsed field data
            // Note: field_data is consumed by the body! macro below
            let _ = &field_data; // Explicitly mark as used to satisfy clippy
            let class_body = ts_template!(Within {
                static defaultValue(): @{class_ident.clone()} {
                    const instance = new @{class_expr.clone()}();
                    {#for (name_ident, value_expr) in field_data}
                        instance.@{name_ident} = @{value_expr};
                    {/for}
                    return instance;
                }
            });

            // Also generate standalone function for consistency
            // Using {$typescript} to compose TsStream objects
            let fn_name_ident = ts_ident!("{}DefaultValue", class_name.to_case(Case::Camel));
            Ok(ts_template! {
                {$typescript class_body}

                export function @{fn_name_ident}(): @{class_ident.clone()} {
                    return @{class_expr.clone()}.defaultValue();
                }
            })
        }
        Data::Enum(enum_data) => {
            let enum_name = input.name();
            let enum_ident = ts_ident!(enum_name);

            // Find variant with @default attribute (like Rust's #[default] on enums)
            let default_variant = enum_data.variants().iter().find(|v| {
                v.decorators
                    .iter()
                    .any(|d| d.name.eq_ignore_ascii_case("default"))
            });

            match default_variant {
                Some(variant) => {
                    let variant_name = &variant.name;
                    let fn_name_ident = ts_ident!("{}DefaultValue", enum_name.to_case(Case::Camel));
                    let enum_expr: Expr = ts_ident!(enum_name).into();
                    let variant_ident = ts_ident!(variant_name.as_str());
                    Ok(ts_template! {
                        export function @{fn_name_ident}(): @{enum_ident} {
                            return @{enum_expr}.@{variant_ident};
                        }
                    })
                }
                None => Err(MacroforgeError::new(
                    input.decorator_span(),
                    format!(
                        "@derive(Default) on enum requires exactly one variant with @default attribute. \
                        Add @default to one variant of {}",
                        enum_name
                    ),
                )),
            }
        }
        Data::Interface(interface) => {
            let interface_name = input.name();
            let interface_ident = ts_ident!(interface_name);

            // Validate fields against type registry (emit warnings for types missing Default)
            let fields_to_validate: Vec<(String, String)> = interface
                .fields()
                .iter()
                .filter(|f| {
                    !f.optional && !DefaultFieldOptions::from_decorators(&f.decorators).has_default
                })
                .map(|f| (f.name.clone(), f.ts_type.clone()))
                .collect();
            validate_default_fields(&fields_to_validate, interface_name, type_registry);

            // Check for required non-primitive fields missing @default (like Rust's derive(Default))
            let missing_defaults: Vec<&str> = interface
                .fields()
                .iter()
                .filter(|field| {
                    // Skip optional fields
                    if field.optional {
                        return false;
                    }
                    // Skip if has explicit @default
                    if DefaultFieldOptions::from_decorators(&field.decorators).has_default {
                        return false;
                    }
                    // Skip if type has known default (primitives, collections, nullable)
                    if has_known_default(&field.ts_type) {
                        return false;
                    }
                    // This field needs @default but doesn't have it
                    true
                })
                .map(|f| f.name.as_str())
                .collect();

            if !missing_defaults.is_empty() {
                return Err(MacroforgeError::new(
                    input.decorator_span(),
                    format!(
                        "@derive(Default) cannot determine default for non-primitive fields. Add @default(value) to: {}",
                        missing_defaults.join(", ")
                    ),
                ));
            }

            // Build defaults for ALL non-optional fields
            let default_fields: Vec<DefaultField> = interface
                .fields()
                .iter()
                .filter(|field| !field.optional)
                .map(|field| {
                    let opts = DefaultFieldOptions::from_decorators(&field.decorators);
                    DefaultField {
                        name: field.name.clone(),
                        value: opts
                            .value
                            .unwrap_or_else(|| get_type_default(&field.ts_type)),
                    }
                })
                .collect();

            let has_defaults = !default_fields.is_empty();

            let fn_name_ident = ts_ident!("{}DefaultValue", interface_name.to_case(Case::Camel));

            if has_defaults {
                let object_fields: Vec<(Ident, Expr)> = default_fields
                    .iter()
                    .map(|f| {
                        let value_expr = parse_ts_expr(&f.value).map_err(|err| {
                            MacroforgeError::new(
                                input.decorator_span(),
                                format!(
                                    "@derive(Default): invalid default expression for '{}': {err:?}",
                                    f.name
                                ),
                            )
                        })?;
                        Ok((ts_ident!(f.name.as_str()), *value_expr))
                    })
                    .collect::<Result<_, MacroforgeError>>()?;

                let mut props = String::new();
                for (name_ident, value_expr) in &object_fields {
                    let name: &str = name_ident.sym.as_ref();
                    let value = emit_expr(value_expr);
                    props.push_str(&format!("{name}: {value},\n"));
                }

                let return_stmt = format!("return {{\n{props}}} as {interface_name};");
                let return_stmt_stream = TsStream::from_string(return_stmt);

                Ok(ts_template! {
                    export function @{fn_name_ident}(): @{interface_ident.clone()} {
                        {$typescript return_stmt_stream}
                    }
                })
            } else {
                let return_stmt = format!("return {{}} as {interface_name};");
                let return_stmt_stream = TsStream::from_string(return_stmt);

                Ok(ts_template! {
                    export function @{fn_name_ident}(): @{interface_ident.clone()} {
                        {$typescript return_stmt_stream}
                    }
                })
            }
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();

            // Build generic type signature if type has type params
            let type_params = type_alias.type_params();
            let (generic_decl, generic_args) = if type_params.is_empty() {
                (String::new(), String::new())
            } else {
                let params = type_params.join(", ");
                (format!("<{}>", params), format!("<{}>", params))
            };
            let full_type_name = format!("{}{}", type_name, generic_args);
            let full_type_ident = ts_ident!(full_type_name.as_str());
            let generic_decl_ident = ts_ident!(generic_decl.as_str());

            if type_alias.is_object() {
                let fields = type_alias.as_object().unwrap();

                // Validate fields against type registry (emit warnings for types missing Default)
                let fields_to_validate: Vec<(String, String)> = fields
                    .iter()
                    .filter(|f| {
                        !f.optional
                            && !DefaultFieldOptions::from_decorators(&f.decorators).has_default
                    })
                    .map(|f| (f.name.clone(), f.ts_type.clone()))
                    .collect();
                validate_default_fields(&fields_to_validate, type_name, type_registry);

                // Check for required non-primitive fields missing @default (like Rust's derive(Default))
                let missing_defaults: Vec<&str> = fields
                    .iter()
                    .filter(|field| {
                        // Skip optional fields
                        if field.optional {
                            return false;
                        }
                        // Skip if has explicit @default
                        if DefaultFieldOptions::from_decorators(&field.decorators).has_default {
                            return false;
                        }
                        // Skip if type has known default (primitives, collections, nullable)
                        if has_known_default(&field.ts_type) {
                            return false;
                        }
                        // This field needs @default but doesn't have it
                        true
                    })
                    .map(|f| f.name.as_str())
                    .collect();

                if !missing_defaults.is_empty() {
                    return Err(MacroforgeError::new(
                        input.decorator_span(),
                        format!(
                            "@derive(Default) cannot determine default for non-primitive fields. Add @default(value) to: {}",
                            missing_defaults.join(", ")
                        ),
                    ));
                }

                // Build defaults for ALL non-optional fields
                let default_fields: Vec<DefaultField> = fields
                    .iter()
                    .filter(|field| !field.optional)
                    .map(|field| {
                        let opts = DefaultFieldOptions::from_decorators(&field.decorators);
                        DefaultField {
                            name: field.name.clone(),
                            value: opts
                                .value
                                .unwrap_or_else(|| get_type_default(&field.ts_type)),
                        }
                    })
                    .collect();

                let has_defaults = !default_fields.is_empty();

                let fn_name_ident = ts_ident!("{}DefaultValue", type_name.to_case(Case::Camel));

                if has_defaults {
                    let object_fields: Vec<(Ident, Expr)> = default_fields
                        .iter()
                        .map(|f| {
                            let value_expr = parse_ts_expr(&f.value).map_err(|err| {
                                MacroforgeError::new(
                                    input.decorator_span(),
                                    format!(
                                        "@derive(Default): invalid default expression for '{}': {err:?}",
                                        f.name
                                    ),
                                )
                            })?;
                            Ok((ts_ident!(f.name.as_str()), *value_expr))
                        })
                        .collect::<Result<_, MacroforgeError>>()?;

                    let mut props = String::new();
                    for (name_ident, value_expr) in &object_fields {
                        let name: &str = name_ident.sym.as_ref();
                        let value = emit_expr(value_expr);
                        props.push_str(&format!("{name}: {value},\n"));
                    }

                    let return_stmt = format!("return {{\n{props}}} as {full_type_name};");
                    let return_stmt_stream = TsStream::from_string(return_stmt);

                    Ok(ts_template! {
                        export function @{fn_name_ident}@{generic_decl_ident}(): @{full_type_ident.clone()} {
                            {$typescript return_stmt_stream}
                        }
                    })
                } else {
                    let return_stmt = format!("return {{}} as {full_type_name};");
                    let return_stmt_stream = TsStream::from_string(return_stmt);

                    Ok(ts_template! {
                        export function @{fn_name_ident}@{generic_decl_ident}(): @{full_type_ident.clone()} {
                            {$typescript return_stmt_stream}
                        }
                    })
                }
            } else if type_alias.is_union() {
                // Union type: check for @default on a variant OR @default(...) on the type
                let members = type_alias.as_union().unwrap();

                // Helper: build an object literal default from an inline object variant's fields
                fn build_object_default(fields: &[crate::ts_syn::InterfaceFieldIR]) -> String {
                    let props: Vec<String> = fields
                        .iter()
                        .map(|f| {
                            let opts = DefaultFieldOptions::from_decorators(&f.decorators);
                            let value = opts.value.unwrap_or_else(|| get_type_default(&f.ts_type));
                            format!("{}: {}", f.name, value)
                        })
                        .collect();
                    format!("({{ {} }})", props.join(", "))
                }

                // Check for parenthesized union members - can't place @default inside parens
                // e.g., `(string | Product) | (string | Service)` is not allowed
                let parenthesized: Vec<&str> = members
                    .iter()
                    .filter_map(|m| m.as_type_ref())
                    .filter(|t| t.trim().starts_with('('))
                    .collect();

                if !parenthesized.is_empty() {
                    return Err(MacroforgeError::new(
                        input.decorator_span(),
                        format!(
                            "@derive(Default): Parenthesized union expressions ({}) are not supported. \
                             Formatters cannot preserve doc comments inside parentheses. \
                             Create a named type alias for each variant instead \
                             (e.g., use `RecordLink<Product>` instead of `(string | Product)`).",
                            parenthesized.join(", ")
                        ),
                    ));
                }

                // First, look for a variant with @default decorator
                let default_variant_from_member = members.iter().find_map(|member| {
                    if member.has_decorator("default") {
                        // Named type (TypeRef or Literal) — use the type name
                        if let Some(name) = member.type_name() {
                            return Some(name.to_string());
                        }
                        // Object type (tagged union variant) — build an object literal
                        // with default values for each field
                        if let Some(fields) = member.as_object() {
                            return Some(build_object_default(fields));
                        }
                        None
                    } else {
                        None
                    }
                });

                // Fallback for tagged object unions where @default may not be
                // attached to the member: use the first object variant.
                let default_variant_from_member = default_variant_from_member.or_else(|| {
                    let all_objects = members.iter().all(|m| m.is_object());
                    if all_objects {
                        members
                            .first()
                            .and_then(|m| m.as_object())
                            .map(build_object_default)
                    } else {
                        None
                    }
                });

                // Fall back to @default(...) on the type alias itself
                let default_variant = default_variant_from_member.or_else(|| {
                    let default_opts = DefaultFieldOptions::from_decorators(
                        &input
                            .attrs
                            .iter()
                            .map(|a| a.inner.clone())
                            .collect::<Vec<_>>(),
                    );
                    default_opts.value
                });

                if let Some(variant) = default_variant {
                    if variant.is_empty() {
                        return Err(MacroforgeError::new(
                            input.decorator_span(),
                            format!(
                                "@derive(Default): resolved an empty default expression for union type '{}'. \
                                 Add @default on a variant or @default(expression) on the type.",
                                type_name
                            ),
                        ));
                    }
                    // Determine the default expression based on variant type
                    // Use as-is if it's already an expression, a literal, or a primitive value
                    let is_expression = variant.contains('.') || variant.contains('(');
                    let is_string_literal = variant.starts_with('"')
                        || variant.starts_with('\'')
                        || variant.starts_with('`');
                    let is_primitive_value = variant.parse::<f64>().is_ok()
                        || variant == "true"
                        || variant == "false"
                        || variant == "null";

                    let default_expr = if is_expression || is_string_literal || is_primitive_value {
                        variant // Use as-is
                    } else {
                        // Use get_type_default which properly handles all types:
                        // - Primitives (string, number, boolean, bigint)
                        // - Generic types (RecordLink<Service>)
                        // - Named types (CompanyName, PersonName - interfaces/classes)
                        get_type_default(&variant)
                    };

                    // Handle generic type aliases (e.g., type RecordLink<T> = ...)
                    let type_params = type_alias.type_params();
                    let has_generics = !type_params.is_empty();
                    let generic_params = if has_generics {
                        format!("<{}>", type_params.join(", "))
                    } else {
                        String::new()
                    };
                    let return_type = if has_generics {
                        format!("{}<{}>", type_name, type_params.join(", "))
                    } else {
                        type_name.to_string()
                    };
                    let return_type_ident = ts_ident!(return_type.as_str());
                    let generic_params_ident = ts_ident!(generic_params.as_str());

                    let fn_name_ident = ts_ident!("{}DefaultValue", type_name.to_case(Case::Camel));
                    let return_expr = parse_ts_expr(&default_expr).map_err(|err| {
                        MacroforgeError::new(
                            input.decorator_span(),
                            format!(
                                "@derive(Default): invalid default expression for '{}': {err:?}",
                                type_name
                            ),
                        )
                    })?;
                    Ok(ts_template! {
                        export function @{fn_name_ident}@{generic_params_ident}(): @{return_type_ident} {
                            return @{return_expr};
                        }
                    })
                } else {
                    Err(MacroforgeError::new(
                        input.decorator_span(),
                        format!(
                            "@derive(Default) on union type '{}' requires @default on one variant \
                            or @default(VariantName.defaultValue()) on the type.",
                            type_name
                        ),
                    ))
                }
            } else {
                // Tuple or simple alias: check for explicit @default(value)
                let default_opts = DefaultFieldOptions::from_decorators(
                    &input
                        .attrs
                        .iter()
                        .map(|a| a.inner.clone())
                        .collect::<Vec<_>>(),
                );

                if let Some(default_variant) = default_opts.value {
                    let fn_name_ident = ts_ident!("{}DefaultValue", type_name.to_case(Case::Camel));
                    let return_expr = parse_ts_expr(&default_variant).map_err(|err| {
                        MacroforgeError::new(
                            input.decorator_span(),
                            format!(
                                "@derive(Default): invalid default expression for '{}': {err:?}",
                                type_name
                            ),
                        )
                    })?;
                    Ok(ts_template! {
                        export function @{fn_name_ident}@{generic_decl_ident}(): @{full_type_ident.clone()} {
                            return @{return_expr};
                        }
                    })
                } else {
                    Err(MacroforgeError::new(
                        input.decorator_span(),
                        format!(
                            "@derive(Default) on type '{}' requires @default(value) to specify the default.",
                            type_name
                        ),
                    ))
                }
            }
        }
    }
}
