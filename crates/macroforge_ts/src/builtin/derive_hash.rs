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
//! | Class | `hashCode(): number` | Instance method computing hash from all fields |
//! | Enum | `hashCodeEnumName(value: EnumName): number` | Standalone function hashing by enum value |
//! | Interface | `hashCodeInterfaceName(value: InterfaceName): number` | Standalone function computing hash |
//! | Type Alias | `hashCodeTypeName(value: TypeName): number` | Standalone function computing hash |
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
//!     hashCode(): number {
//!         let hash = 17;
//!         hash =
//!             (hash * 31 +
//!                 (Number.isInteger(this.id)
//!                     ? this.id | 0
//!                     : this.id
//!                           .toString()
//!                           .split('')
//!                           .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
//!             0;
//!         hash =
//!             (hash * 31 +
//!                 (this.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
//!             0;
//!         return hash;
//!     }
//! 
//!     equals(other: unknown): boolean {
//!         if (this === other) return true;
//!         if (!(other instanceof User)) return false;
//!         const typedOther = other as User;
//!         return (
//!             this.id === typedOther.id &&
//!             this.name === typedOther.name &&
//!             this.cachedScore === typedOther.cachedScore
//!         );
//!     }
//! }
//! ```
//!
//! ## Hash Contract
//!
//! Objects that are equal (`PartialEq`) should produce the same hash code.
//! When using `@hash(skip)`, ensure the same fields are skipped in both
//! `Hash` and `PartialEq` to maintain this contract.

use crate::builtin::derive_common::{CompareFieldOptions, is_primitive_type};
use crate::macros::{body, ts_macro_derive, ts_template};
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

/// Convert a PascalCase name to camelCase (for prefix naming style)
fn to_camel_case(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// Contains field information needed for hash code generation.
///
/// Each field that participates in hashing is represented by this struct,
/// which captures both the field name (for access) and its TypeScript type
/// (to select the appropriate hashing strategy).
struct HashField {
    /// The field name as it appears in the source TypeScript class.
    /// Used to generate property access expressions like `this.name`.
    name: String,

    /// The TypeScript type annotation for this field.
    /// Used to determine which hashing strategy to apply
    /// (e.g., numeric comparison, string hashing, recursive hashCode call).
    ts_type: String,
}

/// Generates JavaScript code that computes a hash contribution for a single class field.
///
/// This function produces an expression that evaluates to an integer hash value
/// for the given field when accessed via `this.fieldName`. The generated code
/// handles different TypeScript types with appropriate hashing strategies.
///
/// # Arguments
///
/// * `field` - The field to generate hash code for, containing name and type info
///
/// # Returns
///
/// A string containing a JavaScript expression that evaluates to an integer.
/// The expression is designed to be combined with other field hashes using
/// the polynomial rolling hash algorithm: `hash = (hash * 31 + fieldHash) | 0`.
///
/// # Type-Specific Strategies
///
/// - **number**: Uses direct bit manipulation for integers, string-based hash for floats
/// - **bigint**: Converts to string and hashes character by character
/// - **string**: Polynomial hash of each character code
/// - **boolean**: Returns 1231 for true, 1237 for false (Java's Boolean.hashCode constants)
/// - **null/undefined**: Returns 1 if non-null, 0 if null
/// - **Array**: Recursively hashes each element, combining with polynomial formula
/// - **Date**: Uses `getTime()` timestamp as hash
/// - **Map**: Hashes all key-value pairs
/// - **Set**: Hashes all elements
/// - **Objects**: Calls `hashCode()` method if available, falls back to JSON string hash
fn generate_field_hash(field: &HashField) -> String {
    let field_name = &field.name;
    let ts_type = &field.ts_type;

    if is_primitive_type(ts_type) {
        match ts_type.as_str() {
            "number" => {
                // For numbers, use bit manipulation if integer, otherwise hash string
                format!(
                    "(Number.isInteger(this.{field_name}) \
                        ? this.{field_name} | 0 \
                        : this.{field_name}.toString().split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))"
                )
            }
            "bigint" => {
                // For bigint, convert to string and hash
                format!(
                    "this.{field_name}.toString().split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)"
                )
            }
            "string" => {
                // For strings, hash each character
                format!(
                    "(this.{field_name} ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)"
                )
            }
            "boolean" => {
                // For booleans, use 1 for true, 0 for false
                format!("(this.{field_name} ? 1231 : 1237)")
            }
            _ => {
                // null/undefined
                format!("(this.{field_name} != null ? 1 : 0)")
            }
        }
    } else if ts_type.ends_with("[]") || ts_type.starts_with("Array<") {
        // For arrays, hash each element and combine
        format!(
            "(Array.isArray(this.{field_name}) \
                ? this.{field_name}.reduce((h, v) => \
                    (h * 31 + (typeof (v as any)?.hashCode === 'function' \
                        ? (v as any).hashCode() \
                        : (v != null ? String(v).split('').reduce((hh, c) => (hh * 31 + c.charCodeAt(0)) | 0, 0) : 0))) | 0, 0) \
                : 0)"
        )
    } else if ts_type == "Date" {
        // For Date, hash the timestamp
        format!("(this.{field_name} instanceof Date ? this.{field_name}.getTime() | 0 : 0)")
    } else if ts_type.starts_with("Map<") {
        // For Map, hash entries
        format!(
            "(this.{field_name} instanceof Map \
                ? Array.from(this.{field_name}.entries()).reduce((h, [k, v]) => \
                    (h * 31 + String(k).split('').reduce((hh, c) => (hh * 31 + c.charCodeAt(0)) | 0, 0) + \
                    (typeof (v as any)?.hashCode === 'function' ? (v as any).hashCode() : 0)) | 0, 0) \
                : 0)"
        )
    } else if ts_type.starts_with("Set<") {
        // For Set, hash elements
        format!(
            "(this.{field_name} instanceof Set \
                ? Array.from(this.{field_name}).reduce((h, v) => \
                    (h * 31 + (typeof (v as any)?.hashCode === 'function' \
                        ? (v as any).hashCode() \
                        : (v != null ? String(v).split('').reduce((hh, c) => (hh * 31 + c.charCodeAt(0)) | 0, 0) : 0))) | 0, 0) \
                : 0)"
        )
    } else {
        // For objects, check for hashCode method first
        format!(
            "(typeof (this.{field_name} as any)?.hashCode === 'function' \
                ? (this.{field_name} as any).hashCode() \
                : (this.{field_name} != null \
                    ? JSON.stringify(this.{field_name}).split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0) \
                    : 0))"
        )
    }
}

/// Generates JavaScript code that computes a hash contribution for interface/type alias fields.
///
/// Similar to [`generate_field_hash`], but uses a variable name parameter instead of `this`,
/// making it suitable for namespace functions that take the object as a parameter.
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
/// # Example
///
/// ```ignore
/// let code = generate_field_hash_for_interface(&field, "self");
/// // Generates: "(self.name ?? '').split('').reduce(...)"
/// ```
fn generate_field_hash_for_interface(field: &HashField, var: &str) -> String {
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

            // Build hash computation
            let hash_body = if has_fields {
                hash_fields
                    .iter()
                    .map(|f| format!("hash = (hash * 31 + {}) | 0;", generate_field_hash(f)))
                    .collect::<Vec<_>>()
                    .join("\n                    ")
            } else {
                String::new()
            };

            Ok(body! {
                hashCode(): number {
                    let hash = 17;
                    {#if has_fields}
                        @{hash_body}
                    {/if}
                    return hash;
                }
            })
        }
        Data::Enum(_) => {
            let enum_name = input.name();
            let fn_name = format!("{}HashCode", to_camel_case(enum_name));

            Ok(ts_template! {
                export function @{fn_name}(value: @{enum_name}): number {
                    if (typeof value === "string") {
                        let hash = 0;
                        for (let i = 0; i < value.length; i++) {
                            hash = (hash * 31 + value.charCodeAt(i)) | 0;
                        }
                        return hash;
                    }
                    return value as number;
                }
            })
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

            let fn_name = format!("{}HashCode", to_camel_case(interface_name));

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

                let fn_name = format!("{}HashCode", to_camel_case(type_name));

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
                let fn_name = format!("{}HashCode", to_camel_case(type_name));

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
            .map(|f| format!("hash = (hash * 31 + {}) | 0;", generate_field_hash(f)))
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
        let result = generate_field_hash(&field);
        assert!(result.contains("Number.isInteger"));
    }

    #[test]
    fn test_field_hash_string() {
        let field = HashField {
            name: "name".to_string(),
            ts_type: "string".to_string(),
        };
        let result = generate_field_hash(&field);
        assert!(result.contains("split"));
        assert!(result.contains("charCodeAt"));
    }

    #[test]
    fn test_field_hash_boolean() {
        let field = HashField {
            name: "active".to_string(),
            ts_type: "boolean".to_string(),
        };
        let result = generate_field_hash(&field);
        assert!(result.contains("1231")); // Java's Boolean.hashCode() constants
        assert!(result.contains("1237"));
    }

    #[test]
    fn test_field_hash_date() {
        let field = HashField {
            name: "createdAt".to_string(),
            ts_type: "Date".to_string(),
        };
        let result = generate_field_hash(&field);
        assert!(result.contains("getTime"));
    }

    #[test]
    fn test_field_hash_object() {
        let field = HashField {
            name: "user".to_string(),
            ts_type: "User".to_string(),
        };
        let result = generate_field_hash(&field);
        assert!(result.contains("hashCode"));
    }
}
