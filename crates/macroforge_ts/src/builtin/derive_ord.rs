//! # Ord Macro Implementation
//!
//! The `Ord` macro generates a `compareTo()` method for **total ordering** comparison.
//! This is analogous to Rust's `Ord` trait, enabling objects to be sorted and
//! compared with a guaranteed ordering relationship.
//!
//! ## Generated Output
//!
//! | Type | Generated Code | Description |
//! |------|----------------|-------------|
//! | Class | `compareTo(other): number` | Instance method returning -1, 0, or 1 |
//! | Enum | `compareEnumName(a: EnumName, b: EnumName): number` | Standalone function comparing enum values |
//! | Interface | `compareInterfaceName(a: InterfaceName, b: InterfaceName): number` | Standalone function comparing fields |
//! | Type Alias | `compareTypeName(a: TypeName, b: TypeName): number` | Standalone function with type-appropriate comparison |
//!
//!
//! ## Return Values
//!
//! Unlike `PartialOrd`, `Ord` provides **total ordering** - every pair of values
//! can be compared:
//!
//! - **-1**: `this` is less than `other`
//! - **0**: `this` is equal to `other`
//! - **1**: `this` is greater than `other`
//!
//! The method **never returns null** - all values must be comparable.
//!
//! ## Comparison Strategy
//!
//! Fields are compared **lexicographically** in declaration order:
//!
//! 1. Compare first field
//! 2. If not equal, return that result
//! 3. Otherwise, compare next field
//! 4. Continue until a difference is found or all fields are equal
//!
//! ## Type-Specific Comparisons
//!
//! | Type | Comparison Method |
//! |------|-------------------|
//! | `number`/`bigint` | Direct `<` and `>` comparison |
//! | `string` | `localeCompare()` (clamped to -1, 0, 1) |
//! | `boolean` | false < true |
//! | Arrays | Lexicographic element-by-element |
//! | `Date` | `getTime()` timestamp comparison |
//! | Objects | Calls `compareTo()` if available, else 0 |
//!
//! ## Field-Level Options
//!
//! The `@ord` decorator supports:
//!
//! - `skip` - Exclude the field from ordering comparison
//!
//! ## Example
//!
//! ```typescript
//! /** @derive(Ord) */
//! class Version {
//!     major: number;
//!     minor: number;
//!     patch: number;
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! class Version {
//!     major: number;
//!     minor: number;
//!     patch: number;
//! 
//!     compareTo(other: Version): number {
//!         if (this === other) return 0;
//!         const typedOther = other;
//!         const cmp0 = this.major < typedOther.major ? -1 : this.major > typedOther.major ? 1 : 0;
//!         if (cmp0 !== 0) return cmp0;
//!         const cmp1 = this.minor < typedOther.minor ? -1 : this.minor > typedOther.minor ? 1 : 0;
//!         if (cmp1 !== 0) return cmp1;
//!         const cmp2 = this.patch < typedOther.patch ? -1 : this.patch > typedOther.patch ? 1 : 0;
//!         if (cmp2 !== 0) return cmp2;
//!         return 0;
//!     }
//! }
//! ```
//!
//! ## Ord vs PartialOrd
//!
//! - Use **Ord** when all values are comparable (total ordering)
//! - Use **PartialOrd** when some values may be incomparable (returns `Option<number>`)

use crate::builtin::derive_common::{CompareFieldOptions, is_numeric_type, is_primitive_type};
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

/// Contains field information needed for ordering comparison generation.
///
/// Each field that participates in ordering is represented by this struct,
/// which captures both the field name (for access) and its TypeScript type
/// (to select the appropriate comparison strategy).
struct OrdField {
    /// The field name as it appears in the source TypeScript class.
    /// Used to generate property access expressions like `this.name`.
    name: String,

    /// The TypeScript type annotation for this field.
    /// Used to determine which comparison strategy to apply
    /// (e.g., numeric comparison, string localeCompare, recursive compareTo).
    ts_type: String,
}

/// Generates JavaScript code that compares a single class field for ordering.
///
/// This function produces an expression that evaluates to -1, 0, or 1 indicating
/// the ordering relationship between two values. The generated code handles
/// different TypeScript types with appropriate comparison strategies.
///
/// Unlike `PartialOrd`, this function guarantees a total ordering - it **never
/// returns null**. For incomparable values, it falls back to 0 (equal).
///
/// # Arguments
///
/// * `field` - The field to generate comparison code for
///
/// # Returns
///
/// A string containing a JavaScript expression that evaluates to -1, 0, or 1.
/// The expression compares `this.field` with `typedOther.field`.
///
/// # Type-Specific Strategies
///
/// - **number/bigint**: Direct `<` and `>` comparison
/// - **string**: Uses `localeCompare()`, result clamped to -1, 0, 1
/// - **boolean**: false is less than true
/// - **null/undefined**: Treated as equal (returns 0)
/// - **Arrays**: Lexicographic comparison with fallback to 0 for incomparable elements
/// - **Date**: Timestamp comparison via `getTime()`
/// - **Objects**: Calls `compareTo()` if available (with `?? 0` fallback), else 0
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

/// Generates JavaScript code that compares interface/type alias fields for ordering.
///
/// Similar to [`generate_field_compare`], but uses variable name parameters instead
/// of `this`, making it suitable for namespace functions that take objects as parameters.
///
/// # Arguments
///
/// * `field` - The field to generate comparison code for
/// * `self_var` - Variable name for the first object (e.g., "self", "a")
/// * `other_var` - Variable name for the second object (e.g., "other", "b")
///
/// # Returns
///
/// A string containing a JavaScript expression that evaluates to -1, 0, or 1.
/// Field access uses the provided variable names: `self_var.field` vs `other_var.field`.
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
            let fn_name = format!("{}Compare", to_camel_case(enum_name));

            Ok(ts_template! {
                export function @{fn_name}(a: @{enum_name}, b: @{enum_name}): number {
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
                            generate_field_compare_for_interface(f, "a", "b")
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n                        ")
            } else {
                String::new()
            };

            let fn_name = format!("{}Compare", to_camel_case(interface_name));

            Ok(ts_template! {
                export function @{fn_name}(a: @{interface_name}, b: @{interface_name}): number {
                    if (a === b) return 0;
                    {#if has_fields}
                        @{compare_body}
                    {/if}
                    return 0;
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

                let fn_name = format!("{}Compare", to_camel_case(type_name));

                Ok(ts_template! {
                    export function @{fn_name}(a: @{type_name}, b: @{type_name}): number {
                        if (a === b) return 0;
                        {#if has_fields}
                            @{compare_body}
                        {/if}
                        return 0;
                    }
                })
            } else {
                // Union, tuple, or simple alias: basic comparison
                let fn_name = format!("{}Compare", to_camel_case(type_name));

                Ok(ts_template! {
                    export function @{fn_name}(a: @{type_name}, b: @{type_name}): number {
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
