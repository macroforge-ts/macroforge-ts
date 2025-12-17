//! # Default Macro Implementation
//!
//! The `Default` macro generates a static `defaultValue()` factory method that creates
//! instances with default values. This is analogous to Rust's `Default` trait, providing
//! a standard way to create "zero" or "empty" instances of types.
//!
//! ## Generated Output
//!
//! | Type | Generated Code | Description |
//! |------|----------------|-------------|
//! | Class | `static defaultValue(): ClassName` | Static factory method |
//! | Enum | `defaultValueEnumName(): EnumName` | Standalone function returning marked variant |
//! | Interface | `defaultValueInterfaceName(): InterfaceName` | Standalone function returning object literal |
//! | Type Alias | `defaultValueTypeName(): TypeName` | Standalone function with type-appropriate default |
//!
//!
//! ## Default Values by Type
//!
//! The macro uses Rust-like default semantics:
//!
//! | Type | Default Value |
//! |------|---------------|
//! | `string` | `""` (empty string) |
//! | `number` | `0` |
//! | `boolean` | `false` |
//! | `bigint` | `0n` |
//! | `T[]` | `[]` (empty array) |
//! | `Array<T>` | `[]` (empty array) |
//! | `Map<K,V>` | `new Map()` |
//! | `Set<T>` | `new Set()` |
//! | `Date` | `new Date()` (current time) |
//! | `T \| null` | `null` |
//! | `CustomType` | `CustomType.defaultValue()` (recursive) |
//!
//! ## Field-Level Options
//!
//! The `@default` decorator allows specifying explicit default values:
//!
//! - `@default(42)` - Use 42 as the default
//! - `@default("hello")` - Use "hello" as the default
//! - `@default([])` - Use empty array as the default
//! - `@default({ value: "test" })` - Named form for complex values
//!
//! ## Example
//!
//! ```typescript
//! /** @derive(Default) */
//! class UserSettings {
//!     /** @default("light") */
//!     theme: string;
//!
//!     /** @default(10) */
//!     pageSize: number;
//!
//!     notifications: boolean;  // Uses type default: false
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! class UserSettings {
//!     theme: string;
//! 
//!     pageSize: number;
//! 
//!     notifications: boolean; // Uses type default: false
//! 
//!     static defaultValue(): UserSettings {
//!         const instance = new UserSettings();
//!         instance.theme = 'light';
//!         instance.pageSize = 10;
//!         instance.notifications = false;
//!         return instance;
//!     }
//! }
//! ```
//!
//! ## Enum Defaults
//!
//! For enums, mark one variant with `@default`:
//!
//! ```typescript
//! /** @derive(Default) */
//! enum Status {
//!     /** @default */
//!     Pending,
//!     Active,
//!     Completed
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! enum Status {
//!     /** @default */
//!     Pending,
//!     Active,
//!     Completed
//! }
//! 
//! export function statusDefaultValue(): Status {
//!     return Status.Pending;
//! }
//! 
//! namespace Status {
//!     export const defaultValue = statusDefaultValue;
//! }
//! ```
//!
//! ## Error Handling
//!
//! The macro will return an error if:
//!
//! - A non-primitive field lacks `@default` and has no known default
//! - An enum has no variant marked with `@default`
//! - A union type has no `@default` on a variant

use convert_case::{Case, Casing};

use crate::builtin::derive_common::{DefaultFieldOptions, get_type_default, has_known_default};
use crate::macros::{body, ts_macro_derive, ts_template};
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

/// Contains field information needed for default value generation.
///
/// Each non-optional field that needs a default value is represented by this struct,
/// capturing both the field name and the expression to use as its default value.
struct DefaultField {
    /// The field name as it appears in the source TypeScript class.
    /// Used to generate assignment statements like `instance.name = value`.
    name: String,

    /// The JavaScript expression for the default value.
    /// This can be a literal (`0`, `""`, `[]`), a constructor call (`new Date()`),
    /// or a recursive `defaultValue()` call for custom types.
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
                        value: opts.value.unwrap_or_else(|| get_type_default(&field.ts_type)),
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

            let mut class_body = body! {
                static defaultValue(): @{class_name} {
                    const instance = new @{class_name}();
                    {#if has_defaults}
                        @{assignments}
                    {/if}
                    return instance;
                }
            };

            // Also generate standalone function for consistency
            let fn_name = format!("{}DefaultValue", class_name.to_case(Case::Camel));
            let sibling = ts_template! {
                export function @{fn_name}(): @{class_name} {
                    return @{class_name}.defaultValue();
                }
            };
            class_body.runtime_patches.extend(sibling.runtime_patches);

            Ok(class_body)
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
                    let fn_name = format!("{}DefaultValue", enum_name.to_case(Case::Camel));
                    Ok(ts_template! {
                        export function @{fn_name}(): @{enum_name} {
                            return @{enum_name}.@{variant_name};
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
                        value: opts.value.unwrap_or_else(|| get_type_default(&field.ts_type)),
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

            let fn_name = format!("{}DefaultValue", interface_name.to_case(Case::Camel));
            Ok(ts_template! {
                export function @{fn_name}(): @{interface_name} {
                    return {
                        {#if has_defaults}
                            @{object_body}
                        {/if}
                    } as @{interface_name};
                }
            })
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

                let fn_name = format!("{}DefaultValue", type_name.to_case(Case::Camel));
                Ok(ts_template! {
                    export function {|@{fn_name}@{generic_decl}|}(): @{full_type_name} {
                        return {
                            {#if has_defaults}
                                @{object_body}
                            {/if}
                        } as @{full_type_name};
                    }
                })
            } else if type_alias.is_union() {
                // Union type: check for @default on a variant OR @default(...) on the type
                let members = type_alias.as_union().unwrap();

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

                    let fn_name = format!("{}DefaultValue", type_name.to_case(Case::Camel));
                    Ok(ts_template! {
                        export function @{fn_name}@{generic_params}(): @{return_type} {
                            return @{default_expr};
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
                    let fn_name = format!("{}DefaultValue", type_name.to_case(Case::Camel));
                    Ok(ts_template! {
                        export function {|@{fn_name}@{generic_decl}|}(): @{full_type_name} {
                            return @{default_variant};
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
        assert!(
            source.contains("defaultValue"),
            "Should contain defaultValue method"
        );
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
