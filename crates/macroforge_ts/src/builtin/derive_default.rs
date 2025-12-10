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

            // For enums, return the first variant as default
            let first_variant = enum_data
                .variants()
                .first()
                .map(|v| v.name.clone())
                .unwrap_or_else(|| "0 as any".to_string());

            Ok(ts_template! {
                export namespace @{enum_name} {
                    export function defaultValue(): @{enum_name} {
                        return @{enum_name}.@{first_variant};
                    }
                }
            })
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
            } else {
                // Union, tuple, or simple alias: return null/undefined as placeholder
                Ok(ts_template! {
                    export namespace @{type_name} {
                        export function defaultValue(): @{type_name} {
                            return null as unknown as @{type_name};
                        }
                    }
                })
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
