//! /** @derive(Ord) */ macro implementation
//!
//! Generates a `compareTo()` method for total ordering.
//! Returns -1 (less), 0 (equal), or 1 (greater) - never null.
//! Supports @ord(skip) decorator on fields to exclude them from comparison.

use crate::builtin::derive_common::{is_numeric_type, is_primitive_type, CompareFieldOptions};
use crate::macros::{body, ts_macro_derive, ts_template};
use crate::ts_syn::{parse_ts_macro_input, Data, DeriveInput, MacroforgeError, TsStream};

/// Field info for ordering comparison: (field_name, ts_type)
struct OrdField {
    name: String,
    ts_type: String,
}

/// Generate comparison code for a single field (class method version)
/// Returns code that evaluates to -1, 0, or 1 (never null - total ordering)
fn generate_field_compare(field: &OrdField) -> String {
    let field_name = &field.name;
    let ts_type = &field.ts_type;

    if is_numeric_type(ts_type) {
        // For numbers/bigint, use direct comparison
        format!(
            "(this.{field_name} < typedOther.{field_name} ? -1 : \
             this.{field_name} > typedOther.{field_name} ? 1 : 0)"
        )
    } else if ts_type == "string" {
        // For strings, use localeCompare (clamped to -1, 0, 1)
        format!(
            "((cmp => cmp < 0 ? -1 : cmp > 0 ? 1 : 0)(this.{field_name}.localeCompare(typedOther.{field_name})))"
        )
    } else if ts_type == "boolean" {
        // For booleans, false < true
        format!(
            "(this.{field_name} === typedOther.{field_name} ? 0 : \
             this.{field_name} ? 1 : -1)"
        )
    } else if is_primitive_type(ts_type) {
        // For other primitives (null/undefined), treat as equal
        "0".to_string()
    } else if ts_type.ends_with("[]") || ts_type.starts_with("Array<") {
        // For arrays, lexicographic comparison
        format!(
            "(() => {{ \
                const a = this.{field_name} ?? []; \
                const b = typedOther.{field_name} ?? []; \
                const minLen = Math.min(a.length, b.length); \
                for (let i = 0; i < minLen; i++) {{ \
                    const cmp = typeof (a[i] as any)?.compareTo === 'function' \
                        ? (a[i] as any).compareTo(b[i]) ?? 0 \
                        : (a[i] < b[i] ? -1 : a[i] > b[i] ? 1 : 0); \
                    if (cmp !== 0) return cmp; \
                }} \
                return a.length < b.length ? -1 : a.length > b.length ? 1 : 0; \
            }})()"
        )
    } else if ts_type == "Date" {
        // For Date, compare timestamps
        format!(
            "(() => {{ \
                const ta = this.{field_name}?.getTime() ?? 0; \
                const tb = typedOther.{field_name}?.getTime() ?? 0; \
                return ta < tb ? -1 : ta > tb ? 1 : 0; \
            }})()"
        )
    } else {
        // For objects, check for compareTo method, fallback to 0
        format!(
            "(typeof (this.{field_name} as any)?.compareTo === 'function' \
                ? (this.{field_name} as any).compareTo(typedOther.{field_name}) ?? 0 \
                : 0)"
        )
    }
}

/// Generate comparison code for interface/type alias fields
fn generate_field_compare_for_interface(
    field: &OrdField,
    self_var: &str,
    other_var: &str,
) -> String {
    let field_name = &field.name;
    let ts_type = &field.ts_type;

    if is_numeric_type(ts_type) {
        format!(
            "({self_var}.{field_name} < {other_var}.{field_name} ? -1 : \
             {self_var}.{field_name} > {other_var}.{field_name} ? 1 : 0)"
        )
    } else if ts_type == "string" {
        format!(
            "((cmp => cmp < 0 ? -1 : cmp > 0 ? 1 : 0)({self_var}.{field_name}.localeCompare({other_var}.{field_name})))"
        )
    } else if ts_type == "boolean" {
        format!(
            "({self_var}.{field_name} === {other_var}.{field_name} ? 0 : \
             {self_var}.{field_name} ? 1 : -1)"
        )
    } else if is_primitive_type(ts_type) {
        "0".to_string()
    } else if ts_type.ends_with("[]") || ts_type.starts_with("Array<") {
        format!(
            "(() => {{ \
                const a = {self_var}.{field_name} ?? []; \
                const b = {other_var}.{field_name} ?? []; \
                const minLen = Math.min(a.length, b.length); \
                for (let i = 0; i < minLen; i++) {{ \
                    const cmp = typeof (a[i] as any)?.compareTo === 'function' \
                        ? (a[i] as any).compareTo(b[i]) ?? 0 \
                        : (a[i] < b[i] ? -1 : a[i] > b[i] ? 1 : 0); \
                    if (cmp !== 0) return cmp; \
                }} \
                return a.length < b.length ? -1 : a.length > b.length ? 1 : 0; \
            }})()"
        )
    } else if ts_type == "Date" {
        format!(
            "(() => {{ \
                const ta = {self_var}.{field_name}?.getTime() ?? 0; \
                const tb = {other_var}.{field_name}?.getTime() ?? 0; \
                return ta < tb ? -1 : ta > tb ? 1 : 0; \
            }})()"
        )
    } else {
        format!(
            "(typeof ({self_var}.{field_name} as any)?.compareTo === 'function' \
                ? ({self_var}.{field_name} as any).compareTo({other_var}.{field_name}) ?? 0 \
                : 0)"
        )
    }
}

#[ts_macro_derive(
    Ord,
    description = "Generates a compareTo() method for total ordering (returns -1, 0, or 1, never null)",
    attributes(ord)
)]
pub fn derive_ord_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();

            // Collect fields for comparison
            let ord_fields: Vec<OrdField> = class
                .fields()
                .iter()
                .filter_map(|field| {
                    let opts = CompareFieldOptions::from_decorators(&field.decorators, "ord");
                    if opts.skip {
                        return None;
                    }
                    Some(OrdField {
                        name: field.name.clone(),
                        ts_type: field.ts_type.clone(),
                    })
                })
                .collect();

            let has_fields = !ord_fields.is_empty();

            // Build comparison logic - lexicographic by field order
            let compare_body = if has_fields {
                ord_fields
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        let var_name = format!("cmp{}", i);
                        format!(
                            "const {var_name} = {};\n                    if ({var_name} !== 0) return {var_name};",
                            generate_field_compare(f)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n                    ")
            } else {
                String::new()
            };

            Ok(body! {
                compareTo(other: @{class_name}): number {
                    if (this === other) return 0;
                    const typedOther = other;
                    {#if has_fields}
                        @{compare_body}
                    {/if}
                    return 0;
                }
            })
        }
        Data::Enum(_) => {
            let enum_name = input.name();
            Ok(ts_template! {
                export namespace @{enum_name} {
                    export function compareTo(a: @{enum_name}, b: @{enum_name}): number {
                        // For enums, compare by value (numeric enums) or string
                        if (typeof a === "number" && typeof b === "number") {
                            return a < b ? -1 : a > b ? 1 : 0;
                        }
                        if (typeof a === "string" && typeof b === "string") {
                            const cmp = a.localeCompare(b);
                            return cmp < 0 ? -1 : cmp > 0 ? 1 : 0;
                        }
                        return 0;
                    }
                }
            })
        }
        Data::Interface(interface) => {
            let interface_name = input.name();

            let ord_fields: Vec<OrdField> = interface
                .fields()
                .iter()
                .filter_map(|field| {
                    let opts = CompareFieldOptions::from_decorators(&field.decorators, "ord");
                    if opts.skip {
                        return None;
                    }
                    Some(OrdField {
                        name: field.name.clone(),
                        ts_type: field.ts_type.clone(),
                    })
                })
                .collect();

            let has_fields = !ord_fields.is_empty();

            let compare_body = if has_fields {
                ord_fields
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        let var_name = format!("cmp{}", i);
                        format!(
                            "const {var_name} = {};\n                        if ({var_name} !== 0) return {var_name};",
                            generate_field_compare_for_interface(f, "self", "other")
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n                        ")
            } else {
                String::new()
            };

            Ok(ts_template! {
                export namespace @{interface_name} {
                    export function compareTo(self: @{interface_name}, other: @{interface_name}): number {
                        if (self === other) return 0;
                        {#if has_fields}
                            @{compare_body}
                        {/if}
                        return 0;
                    }
                }
            })
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();

            if type_alias.is_object() {
                let ord_fields: Vec<OrdField> = type_alias
                    .as_object()
                    .unwrap()
                    .iter()
                    .filter_map(|field| {
                        let opts = CompareFieldOptions::from_decorators(&field.decorators, "ord");
                        if opts.skip {
                            return None;
                        }
                        Some(OrdField {
                            name: field.name.clone(),
                            ts_type: field.ts_type.clone(),
                        })
                    })
                    .collect();

                let has_fields = !ord_fields.is_empty();

                let compare_body = if has_fields {
                    ord_fields
                        .iter()
                        .enumerate()
                        .map(|(i, f)| {
                            let var_name = format!("cmp{}", i);
                            format!(
                                "const {var_name} = {};\n                        if ({var_name} !== 0) return {var_name};",
                                generate_field_compare_for_interface(f, "a", "b")
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n                        ")
                } else {
                    String::new()
                };

                Ok(ts_template! {
                    export namespace @{type_name} {
                        export function compareTo(a: @{type_name}, b: @{type_name}): number {
                            if (a === b) return 0;
                            {#if has_fields}
                                @{compare_body}
                            {/if}
                            return 0;
                        }
                    }
                })
            } else {
                // Union, tuple, or simple alias: basic comparison
                Ok(ts_template! {
                    export namespace @{type_name} {
                        export function compareTo(a: @{type_name}, b: @{type_name}): number {
                            if (a === b) return 0;
                            // For unions/tuples, try primitive comparison
                            if (typeof a === "number" && typeof b === "number") {
                                return a < b ? -1 : a > b ? 1 : 0;
                            }
                            if (typeof a === "string" && typeof b === "string") {
                                const cmp = a.localeCompare(b);
                                return cmp < 0 ? -1 : cmp > 0 ? 1 : 0;
                            }
                            return 0;
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
    fn test_ord_macro_output() {
        let class_name = "User";
        let ord_fields: Vec<OrdField> = vec![OrdField {
            name: "id".to_string(),
            ts_type: "number".to_string(),
        }];
        let has_fields = !ord_fields.is_empty();

        let compare_body = ord_fields
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let var_name = format!("cmp{}", i);
                format!(
                    "const {var_name} = {};\n                    if ({var_name} !== 0) return {var_name};",
                    generate_field_compare(f)
                )
            })
            .collect::<Vec<_>>()
            .join("\n                    ");

        let output = body! {
            compareTo(other: @{class_name}): number {
                if (this === other) return 0;
                const typedOther = other;
                {#if has_fields}
                    @{compare_body}
                {/if}
                return 0;
            }
        };

        let source = output.source();
        let body_content = source
            .strip_prefix("/* @macroforge:body */")
            .unwrap_or(source);
        let wrapped = format!("class __Temp {{ {} }}", body_content);

        assert!(
            macroforge_ts_syn::parse_ts_stmt(&wrapped).is_ok(),
            "Generated Ord macro output should parse as class members"
        );
        assert!(
            source.contains("compareTo"),
            "Should contain compareTo method"
        );
        // Verify return type doesn't contain null
        assert!(
            !source.contains("number | null"),
            "Should not contain nullable return type"
        );
    }

    #[test]
    fn test_field_compare_number() {
        let field = OrdField {
            name: "id".to_string(),
            ts_type: "number".to_string(),
        };
        let result = generate_field_compare(&field);
        assert!(result.contains("< typedOther.id"));
        assert!(result.contains("> typedOther.id"));
        assert!(!result.contains("null")); // Total ordering - no null
    }

    #[test]
    fn test_field_compare_string() {
        let field = OrdField {
            name: "name".to_string(),
            ts_type: "string".to_string(),
        };
        let result = generate_field_compare(&field);
        assert!(result.contains("localeCompare"));
        // Should clamp localeCompare result to -1, 0, 1
        assert!(result.contains("-1"));
        assert!(result.contains("1"));
    }

    #[test]
    fn test_field_compare_object_no_null() {
        let field = OrdField {
            name: "user".to_string(),
            ts_type: "User".to_string(),
        };
        let result = generate_field_compare(&field);
        assert!(result.contains("compareTo"));
        // Should fallback to 0 instead of null
        assert!(result.contains("?? 0"));
    }
}
