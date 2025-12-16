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
//! export function userEquals(a: User, b: User): boolean {
//!     if (a === b) return true;
//!     return a.id === b.id && a.name === b.name;
//! }
//!
//! class User {
//!     id: number;
//!     name: string;
//!     cachedScore: number;
//!
//!     static equals(a: User, b: User): boolean {
//!         return userEquals(a, b);
//!     }
//! }
//! ```
//!
//! ## Equality Contract
//!
//! ```typescript
//! class User {
//!     id: number;
//!     name: string;
//! 
//!     // Don't compare cached values
//!     /** @hash({ skip: true }) */
//!     cachedScore: number;
//! 
//!     equals(other: unknown): boolean {
//!         if (this === other) return true;
//!         if (!(other instanceof User)) return false;
//!         const typedOther = other as User;
//!         return this.id === typedOther.id && this.name === typedOther.name;
//!     }
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
//!         hash =
//!             (hash * 31 +
//!                 (Number.isInteger(this.cachedScore)
//!                     ? this.cachedScore | 0
//!                     : this.cachedScore
//!                           .toString()
//!                           .split('')
//!                           .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
//!             0;
//!         return hash;
//!     }
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
//!     // Don't compare cached values
//!     /** @hash({ skip: true }) */
//!     cachedScore: number;
//! 
//!     equals(other: unknown): boolean {
//!         if (this === other) return true;
//!         if (!(other instanceof User)) return false;
//!         const typedOther = other as User;
//!         return this.id === typedOther.id && this.name === typedOther.name;
//!     }
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
//!         hash =
//!             (hash * 31 +
//!                 (Number.isInteger(this.cachedScore)
//!                     ? this.cachedScore | 0
//!                     : this.cachedScore
//!                           .toString()
//!                           .split('')
//!                           .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
//!             0;
//!         return hash;
//!     }
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
//!     // Don't compare cached values
//!     /** @hash({ skip: true }) */
//!     cachedScore: number;
//! 
//!     equals(other: unknown): boolean {
//!         if (this === other) return true;
//!         if (!(other instanceof User)) return false;
//!         const typedOther = other as User;
//!         return this.id === typedOther.id && this.name === typedOther.name;
//!     }
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
//!         hash =
//!             (hash * 31 +
//!                 (Number.isInteger(this.cachedScore)
//!                     ? this.cachedScore | 0
//!                     : this.cachedScore
//!                           .toString()
//!                           .split('')
//!                           .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
//!             0;
//!         return hash;
//!     }
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
//!     // Don't compare cached values
//!     /** @hash({ skip: true }) */
//!     cachedScore: number;
//! 
//!     equals(other: unknown): boolean {
//!         if (this === other) return true;
//!         if (!(other instanceof User)) return false;
//!         const typedOther = other as User;
//!         return this.id === typedOther.id && this.name === typedOther.name;
//!     }
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
//!         hash =
//!             (hash * 31 +
//!                 (Number.isInteger(this.cachedScore)
//!                     ? this.cachedScore | 0
//!                     : this.cachedScore
//!                           .toString()
//!                           .split('')
//!                           .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
//!             0;
//!         return hash;
//!     }
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
//!     // Don't compare cached values
//!     /** @hash({ skip: true }) */
//!     cachedScore: number;
//! 
//!     equals(other: unknown): boolean {
//!         if (this === other) return true;
//!         if (!(other instanceof User)) return false;
//!         const typedOther = other as User;
//!         return this.id === typedOther.id && this.name === typedOther.name;
//!     }
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
//!         hash =
//!             (hash * 31 +
//!                 (Number.isInteger(this.cachedScore)
//!                     ? this.cachedScore | 0
//!                     : this.cachedScore
//!                           .toString()
//!                           .split('')
//!                           .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
//!             0;
//!         return hash;
//!     }
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
//!     // Don't compare cached values
//!     /** @hash({ skip: true }) */
//!     cachedScore: number;
//! 
//!     equals(other: unknown): boolean {
//!         if (this === other) return true;
//!         if (!(other instanceof User)) return false;
//!         const typedOther = other as User;
//!         return this.id === typedOther.id && this.name === typedOther.name;
//!     }
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
//!         hash =
//!             (hash * 31 +
//!                 (Number.isInteger(this.cachedScore)
//!                     ? this.cachedScore | 0
//!                     : this.cachedScore
//!                           .toString()
//!                           .split('')
//!                           .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
//!             0;
//!         return hash;
//!     }
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
//!     // Don't compare cached values
//!     /** @hash({ skip: true }) */
//!     cachedScore: number;
//! 
//!     equals(other: unknown): boolean {
//!         if (this === other) return true;
//!         if (!(other instanceof User)) return false;
//!         const typedOther = other as User;
//!         return this.id === typedOther.id && this.name === typedOther.name;
//!     }
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
//!         hash =
//!             (hash * 31 +
//!                 (Number.isInteger(this.cachedScore)
//!                     ? this.cachedScore | 0
//!                     : this.cachedScore
//!                           .toString()
//!                           .split('')
//!                           .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
//!             0;
//!         return hash;
//!     }
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
//!     // Don't compare cached values
//!     /** @hash({ skip: true }) */
//!     cachedScore: number;
//! 
//!     equals(other: unknown): boolean {
//!         if (this === other) return true;
//!         if (!(other instanceof User)) return false;
//!         const typedOther = other as User;
//!         return this.id === typedOther.id && this.name === typedOther.name;
//!     }
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
//!         hash =
//!             (hash * 31 +
//!                 (Number.isInteger(this.cachedScore)
//!                     ? this.cachedScore | 0
//!                     : this.cachedScore
//!                           .toString()
//!                           .split('')
//!                           .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
//!             0;
//!         return hash;
//!     }
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

/// Contains field information needed for equality comparison generation.
///
/// Each field that participates in equality checking is represented by this struct,
/// which captures both the field name (for access) and its TypeScript type
/// (to select the appropriate comparison strategy).
struct EqField {
    /// The field name as it appears in the source TypeScript class.
    /// Used to generate property access expressions like `this.name`.
    name: String,

    /// The TypeScript type annotation for this field.
    /// Used to determine which comparison strategy to apply
    /// (e.g., strict equality for primitives, recursive equals for objects).
    ts_type: String,
}

/// Generates JavaScript code that compares a single class field for equality.
///
/// This function produces an expression that evaluates to a boolean indicating
/// whether the field values are equal. The generated code handles different
/// TypeScript types with appropriate comparison strategies.
///
/// # Arguments
///
/// * `field` - The field to generate comparison code for
///
/// # Returns
///
/// A string containing a JavaScript boolean expression comparing `this.field`
/// with `typedOther.field`. The expression can be combined with `&&` for
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

/// Generates JavaScript code that compares interface/type alias fields for equality.
///
/// Similar to [`generate_field_equality`], but uses variable name parameters instead
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
/// A string containing a JavaScript boolean expression comparing `self_var.field`
/// with `other_var.field`.
///
/// # Example
///
/// ```ignore
/// let code = generate_field_equality_for_interface(&field, "self", "other");
/// // Generates: "self.name === other.name"
/// ```
fn generate_field_equality_for_interface(
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
            let naming_style = input.context.function_naming_style;

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

            // Generate function name based on naming style
            let fn_name = match naming_style {
                FunctionNamingStyle::Prefix => format!("{}Equals", to_camel_case(class_name)),
                FunctionNamingStyle::Suffix => format!("equals{}", class_name),
                FunctionNamingStyle::Generic => "equals".to_string(),
                FunctionNamingStyle::Namespace => format!("{}.equals", class_name),
            };

            // Build comparison expression using a and b parameters
            let comparison = if eq_fields.is_empty() {
                "true".to_string()
            } else {
                eq_fields
                    .iter()
                    .map(|f| generate_field_equality_for_interface(f, "a", "b"))
                    .collect::<Vec<_>>()
                    .join(" && ")
            };

            // Generate standalone function with two parameters
            let standalone = ts_template! {
                export function @{fn_name}(a: @{class_name}, b: @{class_name}): boolean {
                    if (a === b) return true;
                    return @{comparison};
                }
            };

            // Generate static wrapper method that delegates to standalone function
            let class_body = body! {
                static equals(a: @{class_name}, b: @{class_name}): boolean {
                    return @{fn_name}(a, b);
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
        Data::Enum(_) => {
            // Enums: direct comparison with ===
            let enum_name = input.name();
            let fn_name = format!("{}Equals", to_camel_case(enum_name));

            Ok(ts_template! {
                export function @{fn_name}(a: @{enum_name}, b: @{enum_name}): boolean {
                    return a === b;
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
                    .map(|f| generate_field_equality_for_interface(f, "a", "b"))
                    .collect::<Vec<_>>()
                    .join(" && ")
            };

            let fn_name = format!("{}Equals", to_camel_case(interface_name));

            Ok(ts_template! {
                export function @{fn_name}(a: @{interface_name}, b: @{interface_name}): boolean {
                    if (a === b) return true;
                    return @{comparison};
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

                let fn_name = format!("{}Equals", to_camel_case(type_name));

                Ok(ts_template! {
                    export function @{fn_name}(a: @{type_name}, b: @{type_name}): boolean {
                        if (a === b) return true;
                        return @{comparison};
                    }
                })
            } else {
                // Union, tuple, or simple alias: use strict equality and JSON fallback
                let fn_name = format!("{}Equals", to_camel_case(type_name));

                Ok(ts_template! {
                    export function @{fn_name}(a: @{type_name}, b: @{type_name}): boolean {
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
