//! # PartialOrd Macro Implementation
//!
//! The `PartialOrd` macro generates a `compareTo()` method for **partial ordering**
//! comparison. This is analogous to Rust's `PartialOrd` trait, enabling comparison
//! between values where some pairs may be incomparable.
//!
//! ## Generated Output
//!
//! | Type | Generated Code | Description |
//! |------|----------------|-------------|
//! | Class | `compareTo(other): Option<number>` | Instance method with optional result |
//! | Enum | `partialCompareEnumName(a: EnumName, b: EnumName): Option<number>` | Standalone function returning Option |
//! | Interface | `partialCompareInterfaceName(a: InterfaceName, b: InterfaceName): Option<number>` | Standalone function with Option |
//! | Type Alias | `partialCompareTypeName(a: TypeName, b: TypeName): Option<number>` | Standalone function with Option |
//!
//! ## Return Values
//!
//! Unlike `Ord`, `PartialOrd` returns an `Option<number>` to handle incomparable values:
//!
//! - **Option.some(-1)**: `this` is less than `other`
//! - **Option.some(0)**: `this` is equal to `other`
//! - **Option.some(1)**: `this` is greater than `other`
//! - **Option.none()**: Values are incomparable
//!
//! ## When to Use PartialOrd vs Ord
//!
//! - **PartialOrd**: When some values may not be comparable
//!   - Example: Floating-point NaN values
//!   - Example: Mixed-type unions
//!   - Example: Type mismatches between objects
//!
//! - **Ord**: When all values are guaranteed comparable (total ordering)
//!
//! ## Comparison Strategy
//!
//! Fields are compared **lexicographically** in declaration order:
//!
//! 1. Compare first field
//! 2. If incomparable, return `Option.none()`
//! 3. If not equal, return that result wrapped in `Option.some()`
//! 4. Otherwise, compare next field
//! 5. Continue until a difference is found or all fields are equal
//!
//! ## Type-Specific Comparisons
//!
//! | Type | Comparison Method |
//! |------|-------------------|
//! | `number`/`bigint` | Direct comparison, returns some() |
//! | `string` | `localeCompare()` wrapped in some() |
//! | `boolean` | false < true, wrapped in some() |
//! | null/undefined | Returns none() for mismatched nullability |
//! | Arrays | Lexicographic, propagates none() on incomparable elements |
//! | `Date` | Timestamp comparison, none() if invalid |
//! | Objects | Unwraps nested Option from compareTo() |
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
//! /** @derive(PartialOrd) */
//! class Temperature {
//!     value: number | null;
//!     unit: string;
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! class Temperature {
//!     value: number | null;
//!     unit: string;
//! 
//!     compareTo(other: unknown): Option<number> {
//!         if (this === other) return Option.some(0);
//!         if (!(other instanceof Temperature)) return Option.none();
//!         const typedOther = other as Temperature;
//!         const cmp0 = (() => {
//!             if (typeof (this.value as any)?.compareTo === 'function') {
//!                 const optResult = (this.value as any).compareTo(typedOther.value);
//!                 return Option.isNone(optResult) ? null : optResult.value;
//!             }
//!             return this.value === typedOther.value ? 0 : null;
//!         })();
//!         if (cmp0 === null) return Option.none();
//!         if (cmp0 !== 0) return Option.some(cmp0);
//!         const cmp1 = this.unit.localeCompare(typedOther.unit);
//!         if (cmp1 === null) return Option.none();
//!         if (cmp1 !== 0) return Option.some(cmp1);
//!         return Option.some(0);
//!     }
//! }
//! ```
//!
//! ## Required Import
//!
//! The generated code automatically adds an import for `Option` from `macroforge/utils`.

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

/// Contains field information needed for partial ordering comparison generation.
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

/// Generates JavaScript code that compares a single class field for partial ordering.
///
/// This function produces an expression that evaluates to -1, 0, 1, or `null`.
/// The `null` value indicates incomparable values (the caller wraps results in `Option`).
///
/// # Arguments
///
/// * `field` - The field to generate comparison code for
/// * `allow_null` - Whether to return `null` for incomparable values (true for
///   PartialOrd, false for Ord which uses 0 instead)
///
/// # Returns
///
/// A string containing a JavaScript expression that evaluates to -1, 0, 1, or null.
/// The expression compares `this.field` with `typedOther.field`.
///
/// # Type-Specific Strategies
///
/// - **number/bigint**: Direct comparison, never null
/// - **string**: `localeCompare()`, never null
/// - **boolean**: false < true, never null
/// - **null/undefined**: Returns `null_return` if values differ
/// - **Arrays**: Returns null if element comparison returns null
/// - **Date**: Returns null if either value is not a valid Date
/// - **Objects**: Unwraps `Option` from nested `compareTo()` calls
///
/// # Example Output
///
/// For a number field: `"(this.count < typedOther.count ? -1 : this.count > typedOther.count ? 1 : 0)"`
/// For an object field: `"(() => { if (typeof (this.user as any)?.compareTo === 'function') { ... } })()"`
fn generate_field_compare(field: &OrdField, allow_null: bool) -> String {
    let field_name = &field.name;
    let ts_type = &field.ts_type;
    let null_return = if allow_null { "null" } else { "0" };

    if is_numeric_type(ts_type) {
        // For numbers/bigint, use direct comparison
        format!(
            "(this.{field_name} < typedOther.{field_name} ? -1 : \
             this.{field_name} > typedOther.{field_name} ? 1 : 0)"
        )
    } else if ts_type == "string" {
        // For strings, use localeCompare
        format!("this.{field_name}.localeCompare(typedOther.{field_name})")
    } else if ts_type == "boolean" {
        // For booleans, false < true
        format!(
            "(this.{field_name} === typedOther.{field_name} ? 0 : \
             this.{field_name} ? 1 : -1)"
        )
    } else if is_primitive_type(ts_type) {
        // For other primitives (null/undefined), treat as equal if both same
        format!("(this.{field_name} === typedOther.{field_name} ? 0 : {null_return})")
    } else if ts_type.ends_with("[]") || ts_type.starts_with("Array<") {
        // For arrays, lexicographic comparison
        // Handle nested compareTo calls that return Option<number>
        format!(
            "(() => {{ \
                const a = this.{field_name}; \
                const b = typedOther.{field_name}; \
                if (!Array.isArray(a) || !Array.isArray(b)) return {null_return}; \
                const minLen = Math.min(a.length, b.length); \
                for (let i = 0; i < minLen; i++) {{ \
                    let cmp: number | null; \
                    if (typeof (a[i] as any)?.compareTo === 'function') {{ \
                        const optResult = (a[i] as any).compareTo(b[i]); \
                        cmp = Option.isNone(optResult) ? null : optResult.value; \
                    }} else {{ \
                        cmp = a[i] < b[i] ? -1 : a[i] > b[i] ? 1 : 0; \
                    }} \
                    if (cmp === null) return {null_return}; \
                    if (cmp !== 0) return cmp; \
                }} \
                return a.length < b.length ? -1 : a.length > b.length ? 1 : 0; \
            }})()"
        )
    } else if ts_type == "Date" {
        // For Date, compare timestamps
        format!(
            "(() => {{ \
                const a = this.{field_name}; \
                const b = typedOther.{field_name}; \
                if (!(a instanceof Date) || !(b instanceof Date)) return {null_return}; \
                const ta = a.getTime(); \
                const tb = b.getTime(); \
                return ta < tb ? -1 : ta > tb ? 1 : 0; \
            }})()"
        )
    } else {
        // For objects, check for compareTo method that returns Option<number>
        format!(
            "(() => {{ \
                if (typeof (this.{field_name} as any)?.compareTo === 'function') {{ \
                    const optResult = (this.{field_name} as any).compareTo(typedOther.{field_name}); \
                    return Option.isNone(optResult) ? {null_return} : optResult.value; \
                }} \
                return this.{field_name} === typedOther.{field_name} ? 0 : {null_return}; \
            }})()"
        )
    }
}

/// Generates JavaScript code that compares interface/type alias fields for partial ordering.
///
/// Similar to [`generate_field_compare`], but uses variable name parameters instead
/// of `this`, making it suitable for namespace functions that take objects as parameters.
///
/// # Arguments
///
/// * `field` - The field to generate comparison code for
/// * `self_var` - Variable name for the first object (e.g., "self", "a")
/// * `other_var` - Variable name for the second object (e.g., "other", "b")
/// * `allow_null` - Whether to return null for incomparable values
///
/// # Returns
///
/// A string containing a JavaScript expression that evaluates to -1, 0, 1, or null.
/// Field access uses the provided variable names.
fn generate_field_compare_for_interface(
    field: &OrdField,
    self_var: &str,
    other_var: &str,
    allow_null: bool,
) -> String {
    let field_name = &field.name;
    let ts_type = &field.ts_type;
    let null_return = if allow_null { "null" } else { "0" };

    if is_numeric_type(ts_type) {
        format!(
            "({self_var}.{field_name} < {other_var}.{field_name} ? -1 : \
             {self_var}.{field_name} > {other_var}.{field_name} ? 1 : 0)"
        )
    } else if ts_type == "string" {
        format!("{self_var}.{field_name}.localeCompare({other_var}.{field_name})")
    } else if ts_type == "boolean" {
        format!(
            "({self_var}.{field_name} === {other_var}.{field_name} ? 0 : \
             {self_var}.{field_name} ? 1 : -1)"
        )
    } else if is_primitive_type(ts_type) {
        format!("({self_var}.{field_name} === {other_var}.{field_name} ? 0 : {null_return})")
    } else if ts_type.ends_with("[]") || ts_type.starts_with("Array<") {
        // Handle nested compareTo calls that return Option<number>
        format!(
            "(() => {{ \
                const a = {self_var}.{field_name}; \
                const b = {other_var}.{field_name}; \
                if (!Array.isArray(a) || !Array.isArray(b)) return {null_return}; \
                const minLen = Math.min(a.length, b.length); \
                for (let i = 0; i < minLen; i++) {{ \
                    let cmp: number | null; \
                    if (typeof (a[i] as any)?.compareTo === 'function') {{ \
                        const optResult = (a[i] as any).compareTo(b[i]); \
                        cmp = Option.isNone(optResult) ? null : optResult.value; \
                    }} else {{ \
                        cmp = a[i] < b[i] ? -1 : a[i] > b[i] ? 1 : 0; \
                    }} \
                    if (cmp === null) return {null_return}; \
                    if (cmp !== 0) return cmp; \
                }} \
                return a.length < b.length ? -1 : a.length > b.length ? 1 : 0; \
            }})()"
        )
    } else if ts_type == "Date" {
        format!(
            "(() => {{ \
                const a = {self_var}.{field_name}; \
                const b = {other_var}.{field_name}; \
                if (!(a instanceof Date) || !(b instanceof Date)) return {null_return}; \
                const ta = a.getTime(); \
                const tb = b.getTime(); \
                return ta < tb ? -1 : ta > tb ? 1 : 0; \
            }})()"
        )
    } else {
        // For objects, check for compareTo method that returns Option<number>
        format!(
            "(() => {{ \
                if (typeof ({self_var}.{field_name} as any)?.compareTo === 'function') {{ \
                    const optResult = ({self_var}.{field_name} as any).compareTo({other_var}.{field_name}); \
                    return Option.isNone(optResult) ? {null_return} : optResult.value; \
                }} \
                return {self_var}.{field_name} === {other_var}.{field_name} ? 0 : {null_return}; \
            }})()"
        )
    }
}

#[ts_macro_derive(
    PartialOrd,
    description = "Generates a compareTo() method for partial ordering (returns Option<number>: some(-1), some(0), some(1), or none())",
    attributes(ord)
)]
pub fn derive_partial_ord_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
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
            // Internal comparisons use raw numbers, final result wrapped in Option
            let compare_body = if has_fields {
                ord_fields
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        let var_name = format!("cmp{}", i);
                        format!(
                            "const {var_name} = {};\n                    if ({var_name} === null) return Option.none();\n                    if ({var_name} !== 0) return Option.some({var_name});",
                            generate_field_compare(f, true)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n                    ")
            } else {
                String::new()
            };

            let mut result = body! {
                compareTo(other: unknown): Option<number> {
                    if (this === other) return Option.some(0);
                    if (!(other instanceof @{class_name})) return Option.none();
                    const typedOther = other as @{class_name};
                    {#if has_fields}
                        @{compare_body}
                    {/if}
                    return Option.some(0);
                }
            };
            result.add_import("Option", "macroforge/utils");
            Ok(result)
        }
        Data::Enum(_) => {
            let enum_name = input.name();
            let fn_name = format!("{}PartialCompare", to_camel_case(enum_name));

            let mut result = ts_template! {
                export function @{fn_name}(a: @{enum_name}, b: @{enum_name}): Option<number> {
                    // For enums, compare by value (numeric enums) or string
                    if (typeof a === "number" && typeof b === "number") {
                        return Option.some(a < b ? -1 : a > b ? 1 : 0);
                    }
                    if (typeof a === "string" && typeof b === "string") {
                        return Option.some(a.localeCompare(b));
                    }
                    return a === b ? Option.some(0) : Option.none();
                }
            };
            result.add_import("Option", "macroforge/utils");
            Ok(result)
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
                            "const {var_name} = {};\n                        if ({var_name} === null) return Option.none();\n                        if ({var_name} !== 0) return Option.some({var_name});",
                            generate_field_compare_for_interface(f, "a", "b", true)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n                        ")
            } else {
                String::new()
            };

            let fn_name = format!("{}PartialCompare", to_camel_case(interface_name));

            let mut result = ts_template! {
                export function @{fn_name}(a: @{interface_name}, b: @{interface_name}): Option<number> {
                    if (a === b) return Option.some(0);
                    {#if has_fields}
                        @{compare_body}
                    {/if}
                    return Option.some(0);
                }
            };
            result.add_import("Option", "macroforge/utils");
            Ok(result)
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
                                "const {var_name} = {};\n                        if ({var_name} === null) return Option.none();\n                        if ({var_name} !== 0) return Option.some({var_name});",
                                generate_field_compare_for_interface(f, "a", "b", true)
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n                        ")
                } else {
                    String::new()
                };

                let fn_name = format!("{}PartialCompare", to_camel_case(type_name));

                let mut result = ts_template! {
                    export function @{fn_name}(a: @{type_name}, b: @{type_name}): Option<number> {
                        if (a === b) return Option.some(0);
                        {#if has_fields}
                            @{compare_body}
                        {/if}
                        return Option.some(0);
                    }
                };
                result.add_import("Option", "macroforge/utils");
                Ok(result)
            } else {
                // Union, tuple, or simple alias: limited comparison
                let fn_name = format!("{}PartialCompare", to_camel_case(type_name));

                let mut result = ts_template! {
                    export function @{fn_name}(a: @{type_name}, b: @{type_name}): Option<number> {
                        if (a === b) return Option.some(0);
                        // For unions/tuples, try primitive comparison
                        if (typeof a === "number" && typeof b === "number") {
                            return Option.some(a < b ? -1 : a > b ? 1 : 0);
                        }
                        if (typeof a === "string" && typeof b === "string") {
                            return Option.some(a.localeCompare(b));
                        }
                        return Option.none();
                    }
                };
                result.add_import("Option", "macroforge/utils");
                Ok(result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::body;

    #[test]
    fn test_partial_ord_macro_output() {
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
                    "const {var_name} = {};\n                    if ({var_name} === null) return Option.none();\n                    if ({var_name} !== 0) return Option.some({var_name});",
                    generate_field_compare(f, true)
                )
            })
            .collect::<Vec<_>>()
            .join("\n                    ");

        let output = body! {
            compareTo(other: unknown): Option<number> {
                if (this === other) return Option.some(0);
                if (!(other instanceof @{class_name})) return Option.none();
                const typedOther = other as @{class_name};
                {#if has_fields}
                    @{compare_body}
                {/if}
                return Option.some(0);
            }
        };

        let source = output.source();
        let body_content = source
            .strip_prefix("/* @macroforge:body */")
            .unwrap_or(source);
        let wrapped = format!("class __Temp {{ {} }}", body_content);

        assert!(
            macroforge_ts_syn::parse_ts_stmt(&wrapped).is_ok(),
            "Generated PartialOrd macro output should parse as class members"
        );
        assert!(
            source.contains("compareTo"),
            "Should contain compareTo method"
        );
        assert!(
            source.contains("Option<number>"),
            "Should have Option<number> return type"
        );
    }

    #[test]
    fn test_field_compare_number() {
        let field = OrdField {
            name: "id".to_string(),
            ts_type: "number".to_string(),
        };
        let result = generate_field_compare(&field, true);
        assert!(result.contains("< typedOther.id"));
        assert!(result.contains("> typedOther.id"));
    }

    #[test]
    fn test_field_compare_string() {
        let field = OrdField {
            name: "name".to_string(),
            ts_type: "string".to_string(),
        };
        let result = generate_field_compare(&field, true);
        assert!(result.contains("localeCompare"));
    }

    #[test]
    fn test_field_compare_boolean() {
        let field = OrdField {
            name: "active".to_string(),
            ts_type: "boolean".to_string(),
        };
        let result = generate_field_compare(&field, true);
        // false < true: false returns -1, true returns 1
        assert!(result.contains("-1"));
        assert!(result.contains("1"));
    }

    #[test]
    fn test_field_compare_date() {
        let field = OrdField {
            name: "createdAt".to_string(),
            ts_type: "Date".to_string(),
        };
        let result = generate_field_compare(&field, true);
        assert!(result.contains("getTime"));
    }

    #[test]
    fn test_field_compare_object() {
        let field = OrdField {
            name: "user".to_string(),
            ts_type: "User".to_string(),
        };
        let result = generate_field_compare(&field, true);
        assert!(result.contains("compareTo"));
        assert!(result.contains("Option.isNone"));
    }

    #[test]
    fn test_field_compare_array() {
        let field = OrdField {
            name: "items".to_string(),
            ts_type: "Item[]".to_string(),
        };
        let result = generate_field_compare(&field, true);
        assert!(result.contains("Option.isNone"));
        assert!(result.contains("optResult.value"));
    }
}
