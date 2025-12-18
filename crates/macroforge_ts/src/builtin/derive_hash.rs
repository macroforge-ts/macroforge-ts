//! # Hash Macro Implementation
//!
//! The `Hash` macro generates a `hashCode()` method for computing numeric hash codes.
//! This is analogous to Rust's `Hash` trait and Java's `hashCode()` method, enabling
//! objects to be used as keys in hash-based collections.
//!
//! ## Generated Output
//!
//! | Type | Generated Code | Description |
//! |------|----------------|-------------|
//! | Class | `classNameHashCode(value)` + `static hashCode(value)` | Standalone function + static wrapper method |
//! | Enum | `enumNameHashCode(value: EnumName): number` | Standalone function hashing by enum value |
//! | Interface | `interfaceNameHashCode(value: InterfaceName): number` | Standalone function computing hash |
//! | Type Alias | `typeNameHashCode(value: TypeName): number` | Standalone function computing hash |
//!
//!
//! ## Hash Algorithm
//!
//! Uses the standard polynomial rolling hash algorithm:
//!
//! ```text
//! hash = 17  // Initial seed
//! for each field:
//!     hash = (hash * 31 + fieldHash) | 0  // Bitwise OR keeps it 32-bit integer
//! ```
//!
//! This algorithm is consistent with Java's `Objects.hash()` implementation.
//!
//! ## Type-Specific Hashing
//!
//! | Type | Hash Strategy |
//! |------|---------------|
//! | `number` | Integer: direct value; Float: string hash of decimal |
//! | `bigint` | String hash of decimal representation |
//! | `string` | Character-by-character polynomial hash |
//! | `boolean` | 1231 for true, 1237 for false (Java convention) |
//! | `Date` | `getTime()` timestamp |
//! | Arrays | Element-by-element hash combination |
//! | `Map` | Entry-by-entry key+value hash |
//! | `Set` | Element-by-element hash |
//! | Objects | Calls `hashCode()` if available, else JSON string hash |
//!
//! ## Field-Level Options
//!
//! The `@hash` decorator supports:
//!
//! - `skip` - Exclude the field from hash calculation
//!
//! ## Example
//!
//! ```typescript
//! /** @derive(Hash, PartialEq) */
//! class User {
//!     id: number;
//!     name: string;
//!
//!     /** @hash({ skip: true }) */
//!     cachedScore: number;
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! class User {
//!     id: number;
//!     name: string;
//! 
//!     cachedScore: number;
//! 
//!     static hashCode(value: User): number {
//!         return userHashCode(value);
//!     }
//! 
//!     static equals(a: User, b: User): boolean {
//!         return userEquals(a, b);
//!     }
//! }
//! 
//! export function userHashCode(value: User): number {
//!     let hash = 17;
//!     hash =
//!         (hash * 31 +
//!             (Number.isInteger(value.id)
//!                 ? value.id | 0
//!                 : value.id
//!                       .toString()
//!                       .split('')
//!                       .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
//!         0;
//!     hash =
//!         (hash * 31 +
//!             (value.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
//!         0;
//!     return hash;
//! }
//! 
//! export function userEquals(a: User, b: User): boolean {
//!     if (a === b) return true;
//!     return a.id === b.id && a.name === b.name && a.cachedScore === b.cachedScore;
//! }
//! ```
//!
//! ## Hash Contract
//!
//! Objects that are equal (`PartialEq`) should produce the same hash code.
//! When using `@hash(skip)`, ensure the same fields are skipped in both
//! `Hash` and `PartialEq` to maintain this contract.

use convert_case::{Case, Casing};

use crate::builtin::derive_common::{CompareFieldOptions, is_primitive_type};
use crate::macros::{body, ts_macro_derive, ts_template};
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

/// Contains field information needed for hash code generation.
///
/// Each field that participates in hashing is represented by this struct,
/// which captures both the field name (for access) and its TypeScript type
/// (to select the appropriate hashing strategy).
pub struct HashField {
    /// The field name as it appears in the source TypeScript class.
    /// Used to generate property access expressions like `this.name`.
    pub name: String,

    /// The TypeScript type annotation for this field.
    /// Used to determine which hashing strategy to apply
    /// (e.g., numeric comparison, string hashing, recursive hashCode call).
    pub ts_type: String,
}

/// Generates JavaScript code that computes a hash contribution for a single field.
///
/// This function produces an expression that evaluates to an integer hash value.
/// The generated code handles different TypeScript types with appropriate
/// hashing strategies.
///
/// # Arguments
///
/// * `field` - The field to generate hash code for
/// * `var` - The variable name to use for field access (e.g., "self", "value")
///
/// # Returns
///
/// A string containing a JavaScript expression that evaluates to an integer hash value.
/// Field access uses the provided variable name: `var.fieldName`.
///
/// # Type-Specific Strategies
///
/// - **number**: Integer values used directly; floats hashed as strings
/// - **bigint**: String hash of decimal representation
/// - **string**: Character-by-character polynomial hash
/// - **boolean**: 1231 for true, 1237 for false (Java convention)
/// - **Date**: `getTime()` timestamp
/// - **Arrays**: Element-by-element hash combination
/// - **Map**: Entry-by-entry key+value hash
/// - **Set**: Element-by-element hash
/// - **Objects**: Calls `hashCode()` if available, else JSON string hash
///
/// # Example
///
/// ```rust
/// use macroforge_ts::builtin::derive_hash::{HashField, generate_field_hash_for_interface};
///
/// let field = HashField { name: "name".to_string(), ts_type: "string".to_string() };
/// let code = generate_field_hash_for_interface(&field, "self");
/// assert!(code.contains("self.name"));
/// assert!(code.contains("reduce"));
/// ```
pub fn generate_field_hash_for_interface(field: &HashField, var: &str) -> String {
    let field_name = &field.name;
    let ts_type = &field.ts_type;

    if is_primitive_type(ts_type) {
        match ts_type.as_str() {
            "number" => {
                format!(
                    "(Number.isInteger({var}.{field_name}) \
                        ? {var}.{field_name} | 0 \
                        : {var}.{field_name}.toString().split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))"
                )
            }
            "bigint" => {
                format!(
                    "{var}.{field_name}.toString().split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)"
                )
            }
            "string" => {
                format!(
                    "({var}.{field_name} ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)"
                )
            }
            "boolean" => {
                format!("({var}.{field_name} ? 1231 : 1237)")
            }
            _ => {
                format!("({var}.{field_name} != null ? 1 : 0)")
            }
        }
    } else if ts_type.ends_with("[]") || ts_type.starts_with("Array<") {
        format!(
            "(Array.isArray({var}.{field_name}) \
                ? {var}.{field_name}.reduce((h, v) => \
                    (h * 31 + (typeof (v as any)?.hashCode === 'function' \
                        ? (v as any).hashCode() \
                        : (v != null ? String(v).split('').reduce((hh, c) => (hh * 31 + c.charCodeAt(0)) | 0, 0) : 0))) | 0, 0) \
                : 0)"
        )
    } else if ts_type == "Date" {
        format!("({var}.{field_name} instanceof Date ? {var}.{field_name}.getTime() | 0 : 0)")
    } else if ts_type.starts_with("Map<") {
        format!(
            "({var}.{field_name} instanceof Map \
                ? Array.from({var}.{field_name}.entries()).reduce((h, [k, v]) => \
                    (h * 31 + String(k).split('').reduce((hh, c) => (hh * 31 + c.charCodeAt(0)) | 0, 0) + \
                    (typeof (v as any)?.hashCode === 'function' ? (v as any).hashCode() : 0)) | 0, 0) \
                : 0)"
        )
    } else if ts_type.starts_with("Set<") {
        format!(
            "({var}.{field_name} instanceof Set \
                ? Array.from({var}.{field_name}).reduce((h, v) => \
                    (h * 31 + (typeof (v as any)?.hashCode === 'function' \
                        ? (v as any).hashCode() \
                        : (v != null ? String(v).split('').reduce((hh, c) => (hh * 31 + c.charCodeAt(0)) | 0, 0) : 0))) | 0, 0) \
                : 0)"
        )
    } else {
        format!(
            "(typeof ({var}.{field_name} as any)?.hashCode === 'function' \
                ? ({var}.{field_name} as any).hashCode() \
                : ({var}.{field_name} != null \
                    ? JSON.stringify({var}.{field_name}).split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0) \
                    : 0))"
        )
    }
}

#[ts_macro_derive(
    Hash,
    description = "Generates a hashCode() method for hashing",
    attributes(hash)
)]
pub fn derive_hash_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();

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
                    })
                })
                .collect();

            let has_fields = !hash_fields.is_empty();

            // Generate function name (always prefix style)
            let fn_name = format!("{}HashCode", class_name.to_case(Case::Camel));

            // Build hash computation using value parameter instead of this
            let hash_body = if has_fields {
                hash_fields
                    .iter()
                    .map(|f| {
                        format!(
                            "hash = (hash * 31 + {}) | 0;",
                            generate_field_hash_for_interface(f, "value")
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n                    ")
            } else {
                String::new()
            };

            // Generate standalone function with value parameter
            let standalone = ts_template! {
                export function @{fn_name}(value: @{class_name}): number {
                    let hash = 17;
                    {#if has_fields}
                        @{hash_body}
                    {/if}
                    return hash;
                }
            };

            // Generate static wrapper method that delegates to standalone function
            let class_body = body! {
                static hashCode(value: @{class_name}): number {
                    return @{fn_name}(value);
                }
            };

            // Combine standalone function with class body by concatenating sources
            // The standalone output (no marker) must come FIRST so it defaults to "below" (after class)
            let combined_source = format!("{}\n{}", standalone.source(), class_body.source());
            let mut combined = TsStream::from_string(combined_source);
            combined.runtime_patches = standalone.runtime_patches;
            combined.runtime_patches.extend(class_body.runtime_patches);

            Ok(combined)
        }
        Data::Enum(enum_data) => {
            let enum_name = input.name();
            let fn_name = format!("{}HashCode", enum_name.to_case(Case::Camel));

            // Check if all variants are string values
            let is_string_enum = enum_data
                .variants()
                .iter()
                .all(|v| v.value.is_string());

            if is_string_enum {
                // String enum: hash the string value
                Ok(ts_template! {
                    export function @{fn_name}(value: @{enum_name}): number {
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
                    export function @{fn_name}(value: @{enum_name}): number {
                        return value as number;
                    }
                })
            }
        }
        Data::Interface(interface) => {
            let interface_name = input.name();

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
                    })
                })
                .collect();

            let has_fields = !hash_fields.is_empty();

            let hash_body = if has_fields {
                hash_fields
                    .iter()
                    .map(|f| {
                        format!(
                            "hash = (hash * 31 + {}) | 0;",
                            generate_field_hash_for_interface(f, "value")
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n                        ")
            } else {
                String::new()
            };

            let fn_name = format!("{}HashCode", interface_name.to_case(Case::Camel));

            Ok(ts_template! {
                export function @{fn_name}(value: @{interface_name}): number {
                    let hash = 17;
                    {#if has_fields}
                        @{hash_body}
                    {/if}
                    return hash;
                }
            })
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();

            if type_alias.is_object() {
                let hash_fields: Vec<HashField> = type_alias
                    .as_object()
                    .unwrap()
                    .iter()
                    .filter_map(|field| {
                        let opts = CompareFieldOptions::from_decorators(&field.decorators, "hash");
                        if opts.skip {
                            return None;
                        }
                        Some(HashField {
                            name: field.name.clone(),
                            ts_type: field.ts_type.clone(),
                        })
                    })
                    .collect();

                let has_fields = !hash_fields.is_empty();

                let hash_body = if has_fields {
                    hash_fields
                        .iter()
                        .map(|f| {
                            format!(
                                "hash = (hash * 31 + {}) | 0;",
                                generate_field_hash_for_interface(f, "value")
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n                        ")
                } else {
                    String::new()
                };

                let fn_name = format!("{}HashCode", type_name.to_case(Case::Camel));

                Ok(ts_template! {
                    export function @{fn_name}(value: @{type_name}): number {
                        let hash = 17;
                        {#if has_fields}
                            @{hash_body}
                        {/if}
                        return hash;
                    }
                })
            } else {
                // Union, tuple, or simple alias: use JSON hash
                let fn_name = format!("{}HashCode", type_name.to_case(Case::Camel));

                Ok(ts_template! {
                    export function @{fn_name}(value: @{type_name}): number {
                        const str = JSON.stringify(value);
                        let hash = 0;
                        for (let i = 0; i < str.length; i++) {
                            hash = (hash * 31 + str.charCodeAt(i)) | 0;
                        }
                        return hash;
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
    fn test_hash_macro_output() {
        let hash_fields: Vec<HashField> = vec![HashField {
            name: "id".to_string(),
            ts_type: "number".to_string(),
        }];
        let has_fields = !hash_fields.is_empty();

        let hash_body = hash_fields
            .iter()
            .map(|f| {
                format!(
                    "hash = (hash * 31 + {}) | 0;",
                    generate_field_hash_for_interface(f, "value")
                )
            })
            .collect::<Vec<_>>()
            .join("\n                    ");

        let output = body! {
            hashCode(): number {
                let hash = 17;
                {#if has_fields}
                    @{hash_body}
                {/if}
                return hash;
            }
        };

        let source = output.source();
        let body_content = source
            .strip_prefix("/* @macroforge:body */")
            .unwrap_or(source);
        let wrapped = format!("class __Temp {{ {} }}", body_content);

        assert!(
            macroforge_ts_syn::parse_ts_stmt(&wrapped).is_ok(),
            "Generated Hash macro output should parse as class members"
        );
        assert!(
            source.contains("hashCode"),
            "Should contain hashCode method"
        );
    }

    #[test]
    fn test_field_hash_number() {
        let field = HashField {
            name: "id".to_string(),
            ts_type: "number".to_string(),
        };
        let result = generate_field_hash_for_interface(&field, "value");
        assert!(result.contains("Number.isInteger"));
    }

    #[test]
    fn test_field_hash_string() {
        let field = HashField {
            name: "name".to_string(),
            ts_type: "string".to_string(),
        };
        let result = generate_field_hash_for_interface(&field, "value");
        assert!(result.contains("split"));
        assert!(result.contains("charCodeAt"));
    }

    #[test]
    fn test_field_hash_boolean() {
        let field = HashField {
            name: "active".to_string(),
            ts_type: "boolean".to_string(),
        };
        let result = generate_field_hash_for_interface(&field, "value");
        assert!(result.contains("1231")); // Java's Boolean.hashCode() constants
        assert!(result.contains("1237"));
    }

    #[test]
    fn test_field_hash_date() {
        let field = HashField {
            name: "createdAt".to_string(),
            ts_type: "Date".to_string(),
        };
        let result = generate_field_hash_for_interface(&field, "value");
        assert!(result.contains("getTime"));
    }

    #[test]
    fn test_field_hash_object() {
        let field = HashField {
            name: "user".to_string(),
            ts_type: "User".to_string(),
        };
        let result = generate_field_hash_for_interface(&field, "value");
        assert!(result.contains("hashCode"));
    }
}
