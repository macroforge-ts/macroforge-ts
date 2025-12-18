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
//! | Class | `classNamePartialCompare(a, b)` + `static compareTo(a, b)` | Standalone function + static wrapper method |
//! | Enum | `enumNamePartialCompare(a: EnumName, b: EnumName): Option<number>` | Standalone function returning Option |
//! | Interface | `interfaceNamePartialCompare(a: InterfaceName, b: InterfaceName): Option<number>` | Standalone function with Option |
//! | Type Alias | `typeNamePartialCompare(a: TypeName, b: TypeName): Option<number>` | Standalone function with Option |
//!
//! ## Return Values
//!
//! Unlike `Ord`, `PartialOrd` returns an `Option<number>` to handle incomparable values:
//!
//! - **Option.some(-1)**: `a` is less than `b`
//! - **Option.some(0)**: `a` is equal to `b`
//! - **Option.some(1)**: `a` is greater than `b`
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
//!     static compareTo(a: Temperature, b: Temperature): number | null {
//!         return temperaturePartialCompare(a, b);
//!     }
//! }
//! 
//! export function temperaturePartialCompare(a: Temperature, b: Temperature): number | null {
//!     if (a === b) return 0;
//!     const cmp0 = (() => {
//!         if (typeof (a.value as any)?.compareTo === 'function') {
//!             const optResult = (a.value as any).compareTo(b.value);
//!             return optResult === null ? null : optResult;
//!         }
//!         return a.value === b.value ? 0 : null;
//!     })();
//!     if (cmp0 === null) return null;
//!     if (cmp0 !== 0) return cmp0;
//!     const cmp1 = a.unit.localeCompare(b.unit);
//!     if (cmp1 === null) return null;
//!     if (cmp1 !== 0) return cmp1;
//!     return 0;
//! }
//! ```
//!
//! ## Required Import
//!
//! The generated code automatically adds an import for `Option` from `macroforge/reexports`.

use convert_case::{Case, Casing};

use crate::builtin::derive_common::{CompareFieldOptions, is_numeric_type, is_primitive_type};
use crate::builtin::return_types::{
    is_none_check, partial_ord_return_type, unwrap_option_or_null, wrap_none, wrap_some,
};
use crate::builtin::serde::get_return_types_mode;
use crate::macros::{body, ts_macro_derive, ts_template};
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

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

/// Generates JavaScript code that compares fields for partial ordering.
///
/// This function produces an expression that evaluates to -1, 0, 1, or `null`.
/// The `null` value indicates incomparable values (the caller wraps results in `Option`).
///
/// # Arguments
///
/// * `field` - The field to generate comparison code for
/// * `self_var` - Variable name for the first object (e.g., "self", "a")
/// * `other_var` - Variable name for the second object (e.g., "other", "b")
/// * `allow_null` - Whether to return `null` for incomparable values (true for
///   PartialOrd, false for Ord which uses 0 instead)
///
/// # Returns
///
/// A string containing a JavaScript expression that evaluates to -1, 0, 1, or null.
/// Field access uses the provided variable names: `self_var.field` vs `other_var.field`.
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
        // Handle nested compareTo calls that return Option<number> or number | null
        let unwrap_opt = unwrap_option_or_null("optResult");
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
                        cmp = {unwrap_opt}; \
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
        // For objects, check for compareTo method that returns Option<number> or number | null
        let unwrap_opt = unwrap_option_or_null("optResult");
        let is_none = is_none_check("optResult");
        format!(
            "(() => {{ \
                if (typeof ({self_var}.{field_name} as any)?.compareTo === 'function') {{ \
                    const optResult = ({self_var}.{field_name} as any).compareTo({other_var}.{field_name}); \
                    return {is_none} ? {null_return} : {unwrap_opt}; \
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

            // Generate function name (always prefix style)
            let fn_name = format!("{}PartialCompare", class_name.to_case(Case::Camel));

            // Get mode-specific return type and wrappers
            let return_type = partial_ord_return_type();
            let some_zero = wrap_some("0");
            let none_val = wrap_none();

            // Build comparison logic using a and b parameters
            let compare_body = if has_fields {
                ord_fields
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        let var_name = format!("cmp{}", i);
                        let some_var = wrap_some(&var_name);
                        format!(
                            "const {var_name} = {};\n                    if ({var_name} === null) return {none_val};\n                    if ({var_name} !== 0) return {some_var};",
                            generate_field_compare_for_interface(f, "a", "b", true)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n                    ")
            } else {
                String::new()
            };

            // Generate standalone function with two parameters
            let mut standalone = ts_template! {
                export function @{fn_name}(a: @{class_name}, b: @{class_name}): @{return_type} {
                    if (a === b) return @{some_zero};
                    {#if has_fields}
                        @{compare_body}
                    {/if}
                    return @{some_zero};
                }
            };

            // Add imports based on mode
            standalone.add_imports(get_return_types_mode().partial_ord_imports());

            // Generate static wrapper method that delegates to standalone function
            let class_body = body! {
                static compareTo(a: @{class_name}, b: @{class_name}): @{return_type} {
                    return @{fn_name}(a, b);
                }
            };

            // Combine standalone function with class body by concatenating sources
            // The standalone output (no marker) must come FIRST so it defaults to "below" (after class)
            let combined_source = format!("{}\n{}", standalone.source(), class_body.source());
            let mut combined = TsStream::from_string(combined_source);
            combined.runtime_patches = standalone.runtime_patches;
            combined.runtime_patches.extend(class_body.runtime_patches);

            // Add imports based on mode
            combined.add_imports(get_return_types_mode().partial_ord_imports());

            Ok(combined)
        }
        Data::Enum(_) => {
            let enum_name = input.name();
            let fn_name = format!("{}PartialCompare", enum_name.to_case(Case::Camel));

            // Get mode-specific return type and wrappers
            let return_type = partial_ord_return_type();
            let some_zero = wrap_some("0");
            let some_cmp_num = wrap_some("a < b ? -1 : a > b ? 1 : 0");
            let some_locale = wrap_some("a.localeCompare(b)");
            let none_val = wrap_none();

            let mut result = ts_template! {
                export function @{fn_name}(a: @{enum_name}, b: @{enum_name}): @{return_type} {
                    // For enums, compare by value (numeric enums) or string
                    if (typeof a === "number" && typeof b === "number") {
                        return @{some_cmp_num};
                    }
                    if (typeof a === "string" && typeof b === "string") {
                        return @{some_locale};
                    }
                    return a === b ? @{some_zero} : @{none_val};
                }
            };

            // Add imports based on mode
            result.add_imports(get_return_types_mode().partial_ord_imports());

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

            // Get mode-specific return type and wrappers
            let return_type = partial_ord_return_type();
            let some_zero = wrap_some("0");
            let none_val = wrap_none();

            let compare_body = if has_fields {
                ord_fields
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        let var_name = format!("cmp{}", i);
                        let some_var = wrap_some(&var_name);
                        format!(
                            "const {var_name} = {};\n                        if ({var_name} === null) return {none_val};\n                        if ({var_name} !== 0) return {some_var};",
                            generate_field_compare_for_interface(f, "a", "b", true)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n                        ")
            } else {
                String::new()
            };

            let fn_name = format!("{}PartialCompare", interface_name.to_case(Case::Camel));

            let mut result = ts_template! {
                export function @{fn_name}(a: @{interface_name}, b: @{interface_name}): @{return_type} {
                    if (a === b) return @{some_zero};
                    {#if has_fields}
                        @{compare_body}
                    {/if}
                    return @{some_zero};
                }
            };

            // Add imports based on mode
            result.add_imports(get_return_types_mode().partial_ord_imports());

            Ok(result)
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();

            // Get mode-specific return type and wrappers
            let return_type = partial_ord_return_type();
            let some_zero = wrap_some("0");
            let none_val = wrap_none();

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
                            let some_var = wrap_some(&var_name);
                            format!(
                                "const {var_name} = {};\n                        if ({var_name} === null) return {none_val};\n                        if ({var_name} !== 0) return {some_var};",
                                generate_field_compare_for_interface(f, "a", "b", true)
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n                        ")
                } else {
                    String::new()
                };

                let fn_name = format!("{}PartialCompare", type_name.to_case(Case::Camel));

                let mut result = ts_template! {
                    export function @{fn_name}(a: @{type_name}, b: @{type_name}): @{return_type} {
                        if (a === b) return @{some_zero};
                        {#if has_fields}
                            @{compare_body}
                        {/if}
                        return @{some_zero};
                    }
                };

                // Add imports based on mode
                result.add_imports(get_return_types_mode().partial_ord_imports());

                Ok(result)
            } else {
                // Union, tuple, or simple alias: limited comparison
                let fn_name = format!("{}PartialCompare", type_name.to_case(Case::Camel));
                let some_cmp_num = wrap_some("a < b ? -1 : a > b ? 1 : 0");
                let some_locale = wrap_some("a.localeCompare(b)");

                let mut result = ts_template! {
                    export function @{fn_name}(a: @{type_name}, b: @{type_name}): @{return_type} {
                        if (a === b) return @{some_zero};
                        // For unions/tuples, try primitive comparison
                        if (typeof a === "number" && typeof b === "number") {
                            return @{some_cmp_num};
                        }
                        if (typeof a === "string" && typeof b === "string") {
                            return @{some_locale};
                        }
                        return @{none_val};
                    }
                };

                // Add imports based on mode
                result.add_imports(get_return_types_mode().partial_ord_imports());

                Ok(result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtin::serde::{clear_return_types_mode, set_return_types_mode};
    use crate::host::config::ReturnTypesMode;
    use crate::macros::body;

    #[test]
    fn test_partial_ord_macro_output_vanilla() {
        clear_return_types_mode();
        let ord_fields: Vec<OrdField> = vec![OrdField {
            name: "id".to_string(),
            ts_type: "number".to_string(),
        }];
        let has_fields = !ord_fields.is_empty();

        let none_val = wrap_none();
        let compare_body = ord_fields
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let var_name = format!("cmp{}", i);
                let some_var = wrap_some(&var_name);
                format!(
                    "const {var_name} = {};\n                    if ({var_name} === null) return {none_val};\n                    if ({var_name} !== 0) return {some_var};",
                    generate_field_compare_for_interface(f, "a", "b", true)
                )
            })
            .collect::<Vec<_>>()
            .join("\n                    ");

        let return_type = partial_ord_return_type();
        let some_zero = wrap_some("0");
        let output = body! {
            compareTo(other: unknown): @{return_type} {
                if (a === b) return @{some_zero};
                {#if has_fields}
                    @{compare_body}
                {/if}
                return @{some_zero};
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
        // In vanilla mode, return type is "number | null"
        assert!(
            source.contains("number | null"),
            "Should have number | null return type in vanilla mode"
        );
    }

    #[test]
    fn test_partial_ord_macro_output_custom() {
        set_return_types_mode(ReturnTypesMode::Custom);
        let return_type = partial_ord_return_type();
        assert_eq!(return_type, "Option<number>");
        assert_eq!(wrap_some("0"), "Option.some(0)");
        assert_eq!(wrap_none(), "Option.none()");
        clear_return_types_mode();
    }

    #[test]
    fn test_field_compare_number() {
        clear_return_types_mode();
        let field = OrdField {
            name: "id".to_string(),
            ts_type: "number".to_string(),
        };
        let result = generate_field_compare_for_interface(&field, "a", "b", true);
        assert!(result.contains("a.id < b.id"));
        assert!(result.contains("a.id > b.id"));
    }

    #[test]
    fn test_field_compare_string() {
        clear_return_types_mode();
        let field = OrdField {
            name: "name".to_string(),
            ts_type: "string".to_string(),
        };
        let result = generate_field_compare_for_interface(&field, "a", "b", true);
        assert!(result.contains("localeCompare"));
    }

    #[test]
    fn test_field_compare_boolean() {
        clear_return_types_mode();
        let field = OrdField {
            name: "active".to_string(),
            ts_type: "boolean".to_string(),
        };
        let result = generate_field_compare_for_interface(&field, "a", "b", true);
        // false < true: false returns -1, true returns 1
        assert!(result.contains("-1"));
        assert!(result.contains("1"));
    }

    #[test]
    fn test_field_compare_date() {
        clear_return_types_mode();
        let field = OrdField {
            name: "createdAt".to_string(),
            ts_type: "Date".to_string(),
        };
        let result = generate_field_compare_for_interface(&field, "a", "b", true);
        assert!(result.contains("getTime"));
    }

    #[test]
    fn test_field_compare_object_vanilla() {
        clear_return_types_mode();
        let field = OrdField {
            name: "user".to_string(),
            ts_type: "User".to_string(),
        };
        let result = generate_field_compare_for_interface(&field, "a", "b", true);
        assert!(result.contains("compareTo"));
        // In vanilla mode, we check for null directly
        assert!(result.contains("=== null"));
    }

    #[test]
    fn test_field_compare_object_custom() {
        set_return_types_mode(ReturnTypesMode::Custom);
        let field = OrdField {
            name: "user".to_string(),
            ts_type: "User".to_string(),
        };
        let result = generate_field_compare_for_interface(&field, "a", "b", true);
        assert!(result.contains("compareTo"));
        // In custom mode, we use Option.isNone
        assert!(result.contains("Option.isNone"));
        clear_return_types_mode();
    }

    #[test]
    fn test_field_compare_array_vanilla() {
        clear_return_types_mode();
        let field = OrdField {
            name: "items".to_string(),
            ts_type: "Item[]".to_string(),
        };
        let result = generate_field_compare_for_interface(&field, "a", "b", true);
        // In vanilla mode, optResult is already the value
        assert!(result.contains("cmp = optResult"));
    }

    #[test]
    fn test_field_compare_array_custom() {
        set_return_types_mode(ReturnTypesMode::Custom);
        let field = OrdField {
            name: "items".to_string(),
            ts_type: "Item[]".to_string(),
        };
        let result = generate_field_compare_for_interface(&field, "a", "b", true);
        // In custom mode, we unwrap the Option
        assert!(result.contains("Option.isNone(optResult)"));
        assert!(result.contains("optResult.value"));
        clear_return_types_mode();
    }
}
