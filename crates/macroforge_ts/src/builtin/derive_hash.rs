//! /** @derive(Hash) */ macro implementation
//!
//! Generates a `hashCode()` method for hashing.
//! Supports @hash(skip) decorator on fields to exclude them from the hash.

use crate::builtin::derive_common::{is_primitive_type, CompareFieldOptions};
use crate::macros::{body, ts_macro_derive, ts_template};
use crate::ts_syn::{parse_ts_macro_input, Data, DeriveInput, MacroforgeError, TsStream};

/// Field info for hashing: (field_name, ts_type)
struct HashField {
    name: String,
    ts_type: String,
}

/// Generate hash contribution code for a single field (class method version)
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
        format!(
            "(this.{field_name} instanceof Date ? this.{field_name}.getTime() | 0 : 0)"
        )
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

/// Generate hash contribution code for interface/type alias fields
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
        format!(
            "({var}.{field_name} instanceof Date ? {var}.{field_name}.getTime() | 0 : 0)"
        )
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
            Ok(ts_template! {
                export namespace @{enum_name} {
                    export function hashCode(value: @{enum_name}): number {
                        // For numeric enums, use the value directly
                        // For string enums, hash the string
                        if (typeof value === "string") {
                            let hash = 0;
                            for (let i = 0; i < value.length; i++) {
                                hash = (hash * 31 + value.charCodeAt(i)) | 0;
                            }
                            return hash;
                        }
                        return value as number;
                    }
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
                            generate_field_hash_for_interface(f, "self")
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n                        ")
            } else {
                String::new()
            };

            Ok(ts_template! {
                export namespace @{interface_name} {
                    export function hashCode(self: @{interface_name}): number {
                        let hash = 17;
                        {#if has_fields}
                            @{hash_body}
                        {/if}
                        return hash;
                    }
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

                Ok(ts_template! {
                    export namespace @{type_name} {
                        export function hashCode(value: @{type_name}): number {
                            let hash = 17;
                            {#if has_fields}
                                @{hash_body}
                            {/if}
                            return hash;
                        }
                    }
                })
            } else {
                // Union, tuple, or simple alias: use JSON hash
                Ok(ts_template! {
                    export namespace @{type_name} {
                        export function hashCode(value: @{type_name}): number {
                            const str = JSON.stringify(value);
                            let hash = 0;
                            for (let i = 0; i < str.length; i++) {
                                hash = (hash * 31 + str.charCodeAt(i)) | 0;
                            }
                            return hash;
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
