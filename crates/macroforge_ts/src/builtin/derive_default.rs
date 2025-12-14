//! /** @derive(Default) */ macro implementation
//!
//! Generates a `static default()` factory method that creates an instance with default values.
//! Requires @default(value) decorator on fields to specify their default values.

use crate::builtin::derive_common::{get_type_default, has_known_default, DefaultFieldOptions};
use crate::macros::{body, ts_macro_derive, ts_template};
use crate::ts_syn::{parse_ts_macro_input, Data, DeriveInput, MacroforgeError, TsStream};

/// Field info for default values: (field_name, default_value)
struct DefaultField {
    name: String,
    value: String,
}

#[ts_macro_derive(
    Default,
    description = "Generates a static defaultValue() factory method",
    attributes(default)
)]
pub fn derive_default_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();

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

            // Build defaults for ALL non-optional fields
            let default_fields: Vec<DefaultField> = class
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

            // Build field assignments
            let assignments = if has_defaults {
                default_fields
                    .iter()
                    .map(|f| format!("instance.{} = {};", f.name, f.value))
                    .collect::<Vec<_>>()
                    .join("\n                    ")
            } else {
                String::new()
            };

            Ok(body! {
                static defaultValue(): @{class_name} {
                    const instance = new @{class_name}();
                    {#if has_defaults}
                        @{assignments}
                    {/if}
                    return instance;
                }
            })
        }
        Data::Enum(enum_data) => {
            let enum_name = input.name();

            // Find variant with @default attribute (like Rust's #[default] on enums)
            let default_variant = enum_data.variants().iter().find(|v| {
                v.decorators
                    .iter()
                    .any(|d| d.name.eq_ignore_ascii_case("default"))
            });

            match default_variant {
                Some(variant) => {
                    let variant_name = &variant.name;
                    Ok(ts_template! {
                        export namespace @{enum_name} {
                            export function defaultValue(): @{enum_name} {
                                return @{enum_name}.@{variant_name};
                            }
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

            // Build object literal
            let object_body = if has_defaults {
                default_fields
                    .iter()
                    .map(|f| format!("{}: {},", f.name, f.value))
                    .collect::<Vec<_>>()
                    .join("\n                            ")
            } else {
                String::new()
            };

            Ok(ts_template! {
                export namespace @{interface_name} {
                    export function defaultValue(): @{interface_name} {
                        return {
                            {#if has_defaults}
                                @{object_body}
                            {/if}
                        } as @{interface_name};
                    }
                }
            })
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();

            if type_alias.is_object() {
                let fields = type_alias.as_object().unwrap();

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

                let object_body = if has_defaults {
                    default_fields
                        .iter()
                        .map(|f| format!("{}: {},", f.name, f.value))
                        .collect::<Vec<_>>()
                        .join("\n                            ")
                } else {
                    String::new()
                };

                Ok(ts_template! {
                    export namespace @{type_name} {
                        export function defaultValue(): @{type_name} {
                            return {
                                {#if has_defaults}
                                    @{object_body}
                                {/if}
                            } as @{type_name};
                        }
                    }
                })
            } else if type_alias.is_union() {
                // Union type: check for @default on a variant OR @default(...) on the type
                let members = type_alias.as_union().unwrap();

                // First, look for a variant with @default decorator
                let default_variant_from_member = members.iter().find_map(|member| {
                    if member.has_decorator("default") {
                        member.type_name().map(String::from)
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
                    // Determine the default expression based on variant type
                    // Use as-is if it's already an expression, a literal, or a primitive
                    let is_expression = variant.contains('.') || variant.contains('(');
                    let is_string_literal = variant.starts_with('"') || variant.starts_with('\'') || variant.starts_with('`');
                    let is_primitive = variant.parse::<f64>().is_ok() || variant == "true" || variant == "false" || variant == "null";

                    let default_expr = if is_expression || is_string_literal || is_primitive {
                        variant // Use as-is
                    } else {
                        format!("{}.defaultValue()", variant) // Type reference - call defaultValue()
                    };

                    Ok(ts_template! {
                        export namespace @{type_name} {
                            export function defaultValue(): @{type_name} {
                                return @{default_expr};
                            }
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
                    Ok(ts_template! {
                        export namespace @{type_name} {
                            export function defaultValue(): @{type_name} {
                                return @{default_variant};
                            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::body;

    #[test]
    fn test_default_macro_output() {
        let class_name = "User";
        let default_fields: Vec<DefaultField> = vec![
            DefaultField {
                name: "id".to_string(),
                value: "0".to_string(),
            },
            DefaultField {
                name: "name".to_string(),
                value: r#""""#.to_string(),
            },
        ];
        let has_defaults = !default_fields.is_empty();

        let assignments = default_fields
            .iter()
            .map(|f| format!("instance.{} = {};", f.name, f.value))
            .collect::<Vec<_>>()
            .join("\n                    ");

        let output = body! {
            static defaultValue(): @{class_name} {
                const instance = new @{class_name}();
                {#if has_defaults}
                    @{assignments}
                {/if}
                return instance;
            }
        };

        let source = output.source();
        let body_content = source
            .strip_prefix("/* @macroforge:body */")
            .unwrap_or(source);
        let wrapped = format!("class __Temp {{ {} }}", body_content);

        assert!(
            macroforge_ts_syn::parse_ts_stmt(&wrapped).is_ok(),
            "Generated Default macro output should parse as class members"
        );
        assert!(source.contains("defaultValue"), "Should contain defaultValue method");
        assert!(source.contains("static"), "Should be a static method");
    }

    #[test]
    fn test_default_field_assignment() {
        let fields: Vec<DefaultField> = vec![
            DefaultField {
                name: "count".to_string(),
                value: "42".to_string(),
            },
            DefaultField {
                name: "items".to_string(),
                value: "[]".to_string(),
            },
        ];

        let assignments = fields
            .iter()
            .map(|f| format!("instance.{} = {};", f.name, f.value))
            .collect::<Vec<_>>()
            .join("\n");

        assert!(assignments.contains("instance.count = 42;"));
        assert!(assignments.contains("instance.items = [];"));
    }
}
