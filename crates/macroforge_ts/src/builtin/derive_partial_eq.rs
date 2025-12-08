//! /** @derive(PartialEq) */ macro implementation
//!
//! Generates an `equals()` method for field-by-field comparison.
//! Supports @partialEq(skip) decorator on fields to exclude them from comparison.

use crate::builtin::derive_common::{is_primitive_type, CompareFieldOptions};
use crate::macros::{body, ts_macro_derive, ts_template};
use crate::ts_syn::{parse_ts_macro_input, Data, DeriveInput, MacroforgeError, TsStream};

/// Field info for equality comparison: (field_name, ts_type)
struct EqField {
    name: String,
    ts_type: String,
}

/// Generate equality comparison code for a single field
fn generate_field_equality(field: &EqField) -> String {
    let field_name = &field.name;
    let ts_type = &field.ts_type;

    if is_primitive_type(ts_type) {
        // For primitives, use strict equality
        format!("this.{field_name} === typedOther.{field_name}")
    } else if ts_type.ends_with("[]") || ts_type.starts_with("Array<") {
        // For arrays, compare element by element
        format!(
            "(Array.isArray(this.{field_name}) && Array.isArray(typedOther.{field_name}) && \
             this.{field_name}.length === typedOther.{field_name}.length && \
             this.{field_name}.every((v, i) => \
                typeof (v as any)?.equals === 'function' \
                    ? (v as any).equals(typedOther.{field_name}[i]) \
                    : v === typedOther.{field_name}[i]))"
        )
    } else if ts_type == "Date" {
        // For Date, compare timestamps
        format!(
            "(this.{field_name} instanceof Date && typedOther.{field_name} instanceof Date \
             ? this.{field_name}.getTime() === typedOther.{field_name}.getTime() \
             : this.{field_name} === typedOther.{field_name})"
        )
    } else if ts_type.starts_with("Map<") {
        // For Map, check size and all entries
        format!(
            "(this.{field_name} instanceof Map && typedOther.{field_name} instanceof Map && \
             this.{field_name}.size === typedOther.{field_name}.size && \
             Array.from(this.{field_name}.entries()).every(([k, v]) => \
                typedOther.{field_name}.has(k) && \
                (typeof (v as any)?.equals === 'function' \
                    ? (v as any).equals(typedOther.{field_name}.get(k)) \
                    : v === typedOther.{field_name}.get(k))))"
        )
    } else if ts_type.starts_with("Set<") {
        // For Set, compare by converting to arrays and checking membership
        format!(
            "(this.{field_name} instanceof Set && typedOther.{field_name} instanceof Set && \
             this.{field_name}.size === typedOther.{field_name}.size && \
             Array.from(this.{field_name}).every(v => typedOther.{field_name}.has(v)))"
        )
    } else {
        // For objects, check for equals method first, then fallback to ===
        format!(
            "(typeof (this.{field_name} as any)?.equals === 'function' \
                ? (this.{field_name} as any).equals(typedOther.{field_name}) \
                : this.{field_name} === typedOther.{field_name})"
        )
    }
}

/// Generate equality comparison code for interface/type alias (using parameter names instead of `this`)
fn generate_field_equality_for_interface(field: &EqField, self_var: &str, other_var: &str) -> String {
    let field_name = &field.name;
    let ts_type = &field.ts_type;

    if is_primitive_type(ts_type) {
        format!("{self_var}.{field_name} === {other_var}.{field_name}")
    } else if ts_type.ends_with("[]") || ts_type.starts_with("Array<") {
        format!(
            "(Array.isArray({self_var}.{field_name}) && Array.isArray({other_var}.{field_name}) && \
             {self_var}.{field_name}.length === {other_var}.{field_name}.length && \
             {self_var}.{field_name}.every((v, i) => \
                typeof (v as any)?.equals === 'function' \
                    ? (v as any).equals({other_var}.{field_name}[i]) \
                    : v === {other_var}.{field_name}[i]))"
        )
    } else if ts_type == "Date" {
        format!(
            "({self_var}.{field_name} instanceof Date && {other_var}.{field_name} instanceof Date \
             ? {self_var}.{field_name}.getTime() === {other_var}.{field_name}.getTime() \
             : {self_var}.{field_name} === {other_var}.{field_name})"
        )
    } else if ts_type.starts_with("Map<") {
        format!(
            "({self_var}.{field_name} instanceof Map && {other_var}.{field_name} instanceof Map && \
             {self_var}.{field_name}.size === {other_var}.{field_name}.size && \
             Array.from({self_var}.{field_name}.entries()).every(([k, v]) => \
                {other_var}.{field_name}.has(k) && \
                (typeof (v as any)?.equals === 'function' \
                    ? (v as any).equals({other_var}.{field_name}.get(k)) \
                    : v === {other_var}.{field_name}.get(k))))"
        )
    } else if ts_type.starts_with("Set<") {
        format!(
            "({self_var}.{field_name} instanceof Set && {other_var}.{field_name} instanceof Set && \
             {self_var}.{field_name}.size === {other_var}.{field_name}.size && \
             Array.from({self_var}.{field_name}).every(v => {other_var}.{field_name}.has(v)))"
        )
    } else {
        format!(
            "(typeof ({self_var}.{field_name} as any)?.equals === 'function' \
                ? ({self_var}.{field_name} as any).equals({other_var}.{field_name}) \
                : {self_var}.{field_name} === {other_var}.{field_name})"
        )
    }
}

#[ts_macro_derive(
    PartialEq,
    description = "Generates an equals() method for field-by-field comparison",
    attributes(partialEq)
)]
pub fn derive_partial_eq_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();

            // Collect fields that should be included in equality comparison
            let eq_fields: Vec<EqField> = class
                .fields()
                .iter()
                .filter_map(|field| {
                    let opts = CompareFieldOptions::from_decorators(&field.decorators, "partialEq");
                    if opts.skip {
                        return None;
                    }
                    Some(EqField {
                        name: field.name.clone(),
                        ts_type: field.ts_type.clone(),
                    })
                })
                .collect();

            // Build comparison expression
            let comparison = if eq_fields.is_empty() {
                "true".to_string()
            } else {
                eq_fields
                    .iter()
                    .map(generate_field_equality)
                    .collect::<Vec<_>>()
                    .join(" && ")
            };

            Ok(body! {
                equals(other: unknown): boolean {
                    if (this === other) return true;
                    if (!(other instanceof @{class_name})) return false;
                    const typedOther = other as @{class_name};
                    return @{comparison};
                }
            })
        }
        Data::Enum(_) => {
            // Enums: direct comparison with ===
            let enum_name = input.name();
            Ok(ts_template! {
                export namespace @{enum_name} {
                    export function equals(a: @{enum_name}, b: @{enum_name}): boolean {
                        return a === b;
                    }
                }
            })
        }
        Data::Interface(interface) => {
            let interface_name = input.name();

            // Collect fields for comparison
            let eq_fields: Vec<EqField> = interface
                .fields()
                .iter()
                .filter_map(|field| {
                    let opts = CompareFieldOptions::from_decorators(&field.decorators, "partialEq");
                    if opts.skip {
                        return None;
                    }
                    Some(EqField {
                        name: field.name.clone(),
                        ts_type: field.ts_type.clone(),
                    })
                })
                .collect();

            // Build comparison expression
            let comparison = if eq_fields.is_empty() {
                "true".to_string()
            } else {
                eq_fields
                    .iter()
                    .map(|f| generate_field_equality_for_interface(f, "self", "other"))
                    .collect::<Vec<_>>()
                    .join(" && ")
            };

            Ok(ts_template! {
                export namespace @{interface_name} {
                    export function equals(self: @{interface_name}, other: @{interface_name}): boolean {
                        if (self === other) return true;
                        return @{comparison};
                    }
                }
            })
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();

            if type_alias.is_object() {
                // Object type: field-by-field comparison
                let eq_fields: Vec<EqField> = type_alias
                    .as_object()
                    .unwrap()
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
                        })
                    })
                    .collect();

                let comparison = if eq_fields.is_empty() {
                    "true".to_string()
                } else {
                    eq_fields
                        .iter()
                        .map(|f| generate_field_equality_for_interface(f, "a", "b"))
                        .collect::<Vec<_>>()
                        .join(" && ")
                };

                Ok(ts_template! {
                    export namespace @{type_name} {
                        export function equals(a: @{type_name}, b: @{type_name}): boolean {
                            if (a === b) return true;
                            return @{comparison};
                        }
                    }
                })
            } else {
                // Union, tuple, or simple alias: use strict equality and JSON fallback
                Ok(ts_template! {
                    export namespace @{type_name} {
                        export function equals(a: @{type_name}, b: @{type_name}): boolean {
                            if (a === b) return true;
                            if (typeof a === "object" && typeof b === "object" && a !== null && b !== null) {
                                return JSON.stringify(a) === JSON.stringify(b);
                            }
                            return false;
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
    fn test_partial_eq_macro_output() {
        // Test that the template compiles and produces valid output
        let class_name = "User";
        let eq_fields: Vec<EqField> = vec![
            EqField {
                name: "id".to_string(),
                ts_type: "number".to_string(),
            },
            EqField {
                name: "name".to_string(),
                ts_type: "string".to_string(),
            },
        ];

        let comparison = eq_fields
            .iter()
            .map(generate_field_equality)
            .collect::<Vec<_>>()
            .join(" && ");

        let output = body! {
            equals(other: unknown): boolean {
                if (this === other) return true;
                if (!(other instanceof @{class_name})) return false;
                const typedOther = other as @{class_name};
                return @{comparison};
            }
        };

        let source = output.source();
        let body_content = source
            .strip_prefix("/* @macroforge:body */")
            .unwrap_or(source);
        let wrapped = format!("class __Temp {{ {} }}", body_content);

        assert!(
            macroforge_ts_syn::parse_ts_stmt(&wrapped).is_ok(),
            "Generated PartialEq macro output should parse as class members"
        );
        assert!(source.contains("equals"), "Should contain equals method");
    }

    #[test]
    fn test_field_equality_primitive() {
        let field = EqField {
            name: "id".to_string(),
            ts_type: "number".to_string(),
        };
        let result = generate_field_equality(&field);
        assert!(result.contains("this.id === typedOther.id"));
    }

    #[test]
    fn test_field_equality_object() {
        let field = EqField {
            name: "user".to_string(),
            ts_type: "User".to_string(),
        };
        let result = generate_field_equality(&field);
        assert!(result.contains("equals"));
    }

    #[test]
    fn test_field_equality_array() {
        let field = EqField {
            name: "items".to_string(),
            ts_type: "string[]".to_string(),
        };
        let result = generate_field_equality(&field);
        assert!(result.contains("Array.isArray"));
        assert!(result.contains("every"));
    }

    #[test]
    fn test_field_equality_date() {
        let field = EqField {
            name: "createdAt".to_string(),
            ts_type: "Date".to_string(),
        };
        let result = generate_field_equality(&field);
        assert!(result.contains("getTime"));
    }
}
