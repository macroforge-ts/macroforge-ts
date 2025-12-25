//! # PartialEq Macro Implementation
//!
//! The `PartialEq` macro generates an `equals()` method for field-by-field
//! structural equality comparison. This is analogous to Rust's `PartialEq` trait,
//! enabling value-based equality semantics instead of reference equality.
//!
//! ## Generated Output
//!
//! | Type | Generated Code | Description |
//! |------|----------------|-------------|
//! | Class | `classNameEquals(a, b)` + `static equals(a, b)` | Standalone function + static wrapper method |
//! | Enum | `enumNameEquals(a: EnumName, b: EnumName): boolean` | Standalone function using strict equality |
//! | Interface | `interfaceNameEquals(a: InterfaceName, b: InterfaceName): boolean` | Standalone function comparing fields |
//! | Type Alias | `typeNameEquals(a: TypeName, b: TypeName): boolean` | Standalone function with type-appropriate comparison |
//!
//! ## Comparison Strategy
//!
//! The generated equality check:
//!
//! 1. **Identity check**: `a === b` returns true immediately
//! 2. **Field comparison**: Compares each non-skipped field
//!
//! ## Type-Specific Comparisons
//!
//! | Type | Comparison Method |
//! |------|-------------------|
//! | Primitives | Strict equality (`===`) |
//! | Arrays | Length + element-by-element (recursive) |
//! | `Date` | `getTime()` comparison |
//! | `Map` | Size + entry-by-entry comparison |
//! | `Set` | Size + membership check |
//! | Objects | Calls `equals()` if available, else `===` |
//!
//! ## Field-Level Options
//!
//! The `@partialEq` decorator supports:
//!
//! - `skip` - Exclude the field from equality comparison
//!
//! ## Example
//!
//! ```typescript
//! /** @derive(PartialEq, Hash) */
//! class User {
//!     id: number;
//!     name: string;
//!
//!     /** @partialEq({ skip: true }) @hash({ skip: true }) */
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
//!     static equals(a: User, b: User): boolean {
//!         return userEquals(a, b);
//!     }
//!
//!     static hashCode(value: User): number {
//!         return userHashCode(value);
//!     }
//! }
//!
//! export function userEquals(a: User, b: User): boolean {
//!     if (a === b) return true;
//!     return a.id === b.id && a.name === b.name;
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
//! ```
//!
//! ## Equality Contract
//!
//! When implementing `PartialEq`, consider also implementing `Hash`:
//!
//! - **Reflexivity**: `a.equals(a)` is always true
//! - **Symmetry**: `a.equals(b)` implies `b.equals(a)`
//! - **Hash consistency**: Equal objects must have equal hash codes
//!
//! To maintain the hash contract, skip the same fields in both `PartialEq` and `Hash`:
//!
//! ```typescript
//! /** @derive(PartialEq, Hash) */
//! class User {
//!     id: number;
//!     name: string;
//!
//!     /** @partialEq({ skip: true }) @hash({ skip: true }) */
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
//!     static equals(a: User, b: User): boolean {
//!         return userEquals(a, b);
//!     }
//!
//!     static hashCode(value: User): number {
//!         return userHashCode(value);
//!     }
//! }
//!
//! export function userEquals(a: User, b: User): boolean {
//!     if (a === b) return true;
//!     return a.id === b.id && a.name === b.name;
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
//! ```

use convert_case::{Case, Casing};

use crate::builtin::derive_common::{CompareFieldOptions, is_primitive_type};
use crate::macros::{ts_macro_derive, ts_template};
use crate::swc_ecma_ast::Expr;
use crate::ts_syn::{
    Data, DeriveInput, MacroforgeError, TsStream, ident, parse_ts_expr, parse_ts_macro_input,
};

/// Contains field information needed for equality comparison generation.
///
/// Each field that participates in equality checking is represented by this struct,
/// which captures both the field name (for access) and its TypeScript type
/// (to select the appropriate comparison strategy).
pub struct EqField {
    /// The field name as it appears in the source TypeScript class.
    /// Used to generate property access expressions like `this.name`.
    pub name: String,

    /// The TypeScript type annotation for this field.
    /// Used to determine which comparison strategy to apply
    /// (e.g., strict equality for primitives, recursive equals for objects).
    pub ts_type: String,
}

/// Generates JavaScript code that compares fields for equality.
///
/// This function produces an expression that evaluates to a boolean indicating
/// whether the field values are equal. The generated code handles different
/// TypeScript types with appropriate comparison strategies.
///
/// # Arguments
///
/// * `field` - The field to generate comparison code for
/// * `self_var` - Variable name for the first object (e.g., "self", "a")
/// * `other_var` - Variable name for the second object (e.g., "other", "b")
///
/// # Returns
///
/// A string containing a JavaScript boolean expression comparing `self_var.field`
/// with `other_var.field`. The expression can be combined with `&&` for
/// multiple fields.
///
/// # Type-Specific Strategies
///
/// - **Primitives**: Uses strict equality (`===`)
/// - **Arrays**: Checks length, then compares elements (calls `equals` if available)
/// - **Date**: Compares via `getTime()` timestamps
/// - **Map**: Checks size, then compares all entries
/// - **Set**: Checks size, then verifies all elements present in both
/// - **Objects**: Calls `equals()` method if available, falls back to `===`
///
/// # Example
///
/// ```rust
/// use macroforge_ts::builtin::derive_partial_eq::{EqField, generate_field_equality_for_interface};
///
/// let field = EqField { name: "name".to_string(), ts_type: "string".to_string() };
/// let code = generate_field_equality_for_interface(&field, "self", "other");
/// assert_eq!(code, "self.name === other.name");
/// ```
pub fn generate_field_equality_for_interface(
    field: &EqField,
    self_var: &str,
    other_var: &str,
) -> String {
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
            let class_ident = ident!(class_name);

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

            // Generate function name (always prefix style)
            let fn_name_ident = ident!("{}Equals", class_name.to_case(Case::Camel));
            let fn_name_expr: Expr = fn_name_ident.clone().into();

            let comparison_src = if eq_fields.is_empty() {
                "true".to_string()
            } else {
                eq_fields
                    .iter()
                    .map(|f| generate_field_equality_for_interface(f, "a", "b"))
                    .collect::<Vec<_>>()
                    .join(" && ")
            };
            let comparison_expr = parse_ts_expr(&comparison_src).map_err(|err| {
                MacroforgeError::new(
                    input.decorator_span(),
                    format!("@derive(PartialEq): invalid comparison expression: {err:?}"),
                )
            })?;

            // Generate standalone function with two parameters
            let standalone = ts_template! {
                export function @{fn_name_ident}(a: @{class_ident}, b: @{class_ident}): boolean {
                    if (a === b) return true;
                    return @{comparison_expr};
                }
            };

            // Generate static wrapper method that delegates to standalone function
            let class_body = ts_template!(Within {
                static equals(a: @{class_ident}, b: @{class_ident}): boolean {
                    return @{fn_name_expr}(a, b);
                }
            });

            // Combine standalone function with class body using {$typescript}
            // The standalone output (no marker) must come FIRST so it defaults to "below" (after class)
            Ok(ts_template! {
                {$typescript standalone}
                {$typescript class_body}
            })
        }
        Data::Enum(_) => {
            // Enums: direct comparison with ===
            let enum_name = input.name();
            let fn_name_ident = ident!("{}Equals", enum_name.to_case(Case::Camel));

            Ok(ts_template! {
                export function @{fn_name_ident}(a: @{ident!(enum_name)}, b: @{ident!(enum_name)}): boolean {
                    return a === b;
                }
            })
        }
        Data::Interface(interface) => {
            let interface_name = input.name();
            let interface_ident = ident!(interface_name);

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

            let comparison_src = if eq_fields.is_empty() {
                "true".to_string()
            } else {
                eq_fields
                    .iter()
                    .map(|f| generate_field_equality_for_interface(f, "a", "b"))
                    .collect::<Vec<_>>()
                    .join(" && ")
            };
            let comparison_expr = parse_ts_expr(&comparison_src).map_err(|err| {
                MacroforgeError::new(
                    input.decorator_span(),
                    format!("@derive(PartialEq): invalid comparison expression: {err:?}"),
                )
            })?;

            let fn_name_ident = ident!("{}Equals", interface_name.to_case(Case::Camel));

            Ok(ts_template! {
                export function @{fn_name_ident}(a: @{interface_ident}, b: @{interface_ident}): boolean {
                    if (a === b) return true;
                    return @{comparison_expr};
                }
            })
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();
            let type_ident = ident!(type_name);

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

                let comparison_src = if eq_fields.is_empty() {
                    "true".to_string()
                } else {
                    eq_fields
                        .iter()
                        .map(|f| generate_field_equality_for_interface(f, "a", "b"))
                        .collect::<Vec<_>>()
                        .join(" && ")
                };
                let comparison_expr = parse_ts_expr(&comparison_src).map_err(|err| {
                    MacroforgeError::new(
                        input.decorator_span(),
                        format!("@derive(PartialEq): invalid comparison expression: {err:?}"),
                    )
                })?;

                let fn_name_ident = ident!("{}Equals", type_name.to_case(Case::Camel));

                Ok(ts_template! {
                    export function @{fn_name_ident}(a: @{type_ident}, b: @{type_ident}): boolean {
                        if (a === b) return true;
                        return @{comparison_expr};
                    }
                })
            } else {
                // Union, tuple, or simple alias: use strict equality and JSON fallback
                let fn_name_ident = ident!("{}Equals", type_name.to_case(Case::Camel));

                Ok(ts_template! {
                    export function @{fn_name_ident}(a: @{type_ident}, b: @{type_ident}): boolean {
                        if (a === b) return true;
                        if (typeof a === "object" && typeof b === "object" && a !== null && b !== null) {
                            return JSON.stringify(a) === JSON.stringify(b);
                        }
                        return false;
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
            .map(|f| generate_field_equality_for_interface(f, "a", "b"))
            .collect::<Vec<_>>()
            .join(" && ");
        let comparison_expr = parse_ts_expr(&comparison).expect("comparison expr should parse");

        let output = ts_template!(Within {
            equals(other: unknown): boolean {
                if (a === b) return true;
                return @{comparison_expr};
            }
        });

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
        let result = generate_field_equality_for_interface(&field, "a", "b");
        assert!(result.contains("a.id === b.id"));
    }

    #[test]
    fn test_field_equality_object() {
        let field = EqField {
            name: "user".to_string(),
            ts_type: "User".to_string(),
        };
        let result = generate_field_equality_for_interface(&field, "a", "b");
        assert!(result.contains("equals"));
    }

    #[test]
    fn test_field_equality_array() {
        let field = EqField {
            name: "items".to_string(),
            ts_type: "string[]".to_string(),
        };
        let result = generate_field_equality_for_interface(&field, "a", "b");
        assert!(result.contains("Array.isArray"));
        assert!(result.contains("every"));
    }

    #[test]
    fn test_field_equality_date() {
        let field = EqField {
            name: "createdAt".to_string(),
            ts_type: "Date".to_string(),
        };
        let result = generate_field_equality_for_interface(&field, "a", "b");
        assert!(result.contains("getTime"));
    }
}
