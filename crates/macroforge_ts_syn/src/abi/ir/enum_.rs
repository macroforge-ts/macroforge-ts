//! Enum-related IR types for TypeScript enum declarations.
//!
//! This module provides the intermediate representation for TypeScript enums,
//! including both numeric and string enums.
//!
//! ## Example
//!
//! Given this TypeScript enum:
//!
//! ```typescript
//! /** @derive(Debug) */
//! enum Status {
//!     /** @default */
//!     Active = "ACTIVE",
//!     Inactive = "INACTIVE",
//!     Pending = 0,
//! }
//! ```
//!
//! The resulting [`EnumIR`] would contain:
//! - `name`: `"Status"`
//! - `variants`: Three [`EnumVariantIR`] entries with different [`EnumValue`] types

use serde::{Deserialize, Serialize};

use crate::abi::{DecoratorIR, SpanIR};

/// Represents the value/initializer of an enum variant.
///
/// TypeScript enums can have various types of initializers:
/// - String literals: `Status = "ACTIVE"`
/// - Numeric literals: `Priority = 1`
/// - Auto-incremented (no initializer): `First,`
/// - Computed expressions: `Value = CONST + 1`
///
/// # Examples
///
/// ```typescript
/// enum Example {
///     A,           // EnumValue::Auto
///     B = 10,      // EnumValue::Number(10.0)
///     C = "hello", // EnumValue::String("hello")
///     D = A + B,   // EnumValue::Expr("A + B")
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum EnumValue {
    /// A string literal value.
    ///
    /// Example: `Status = "ACTIVE"` -> `EnumValue::String("ACTIVE")`
    String(String),

    /// A numeric literal value.
    ///
    /// Example: `Priority = 42` -> `EnumValue::Number(42.0)`
    Number(f64),

    /// No explicit initializer; value is auto-incremented from the previous variant.
    ///
    /// For the first variant, this defaults to `0`. For subsequent variants,
    /// it increments from the previous numeric value.
    #[default]
    Auto,

    /// A computed expression stored as raw source code.
    ///
    /// Example: `Value = BASE + OFFSET` -> `EnumValue::Expr("BASE + OFFSET")`
    Expr(String),
}

impl EnumValue {
    /// Returns true if this is a string literal value
    pub fn is_string(&self) -> bool {
        matches!(self, EnumValue::String(_))
    }

    /// Returns true if this is a numeric literal value
    pub fn is_number(&self) -> bool {
        matches!(self, EnumValue::Number(_))
    }

    /// Returns true if this is auto-incremented
    pub fn is_auto(&self) -> bool {
        matches!(self, EnumValue::Auto)
    }

    /// Returns true if this is a computed expression
    pub fn is_expr(&self) -> bool {
        matches!(self, EnumValue::Expr(_))
    }

    /// Returns the string value if this is a String variant
    pub fn as_string(&self) -> Option<&str> {
        match self {
            EnumValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the numeric value if this is a Number variant
    pub fn as_number(&self) -> Option<f64> {
        match self {
            EnumValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Returns the expression source if this is an Expr variant
    pub fn as_expr(&self) -> Option<&str> {
        match self {
            EnumValue::Expr(s) => Some(s),
            _ => None,
        }
    }
}

/// Intermediate representation of a TypeScript enum declaration.
///
/// Represents both regular enums and `const` enums. Captures all variants
/// with their names, values, and any decorators.
///
/// # Const Enums
///
/// When `is_const` is `true`, the enum is declared with `const enum`.
/// Const enums are completely inlined at compile time and don't exist
/// at runtime.
///
/// # Example
///
/// ```rust,no_run
/// use macroforge_ts_syn::{EnumIR, EnumValue};
///
/// fn is_string_enum(enum_ir: &EnumIR) -> bool {
///     enum_ir.variants.iter().all(|v| v.value.is_string())
/// }
///
/// fn generate_values_array(enum_ir: &EnumIR) -> String {
///     let values: Vec<String> = enum_ir.variants.iter()
///         .map(|v| format!("{}.{}", enum_ir.name, v.name))
///         .collect();
///
///     format!("const {}Values = [{}] as const;",
///         enum_ir.name,
///         values.join(", ")
///     )
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EnumIR {
    /// The enum name (identifier).
    pub name: String,

    /// Source span covering the entire enum declaration.
    pub span: SpanIR,

    /// Source span of the enum body (between `{` and `}`).
    pub body_span: SpanIR,

    /// Decorators applied to the enum (from JSDoc).
    pub decorators: Vec<DecoratorIR>,

    /// The enum variants/members.
    pub variants: Vec<EnumVariantIR>,

    /// Whether this is a `const enum`.
    /// Const enums are inlined at compile time.
    pub is_const: bool,
}

/// Intermediate representation of an enum variant/member.
///
/// Each variant has a name, optional initializer value, and can have
/// decorators applied via JSDoc comments.
///
/// # Example
///
/// For the TypeScript enum variant:
///
/// ```typescript
/// /** @default */
/// Active = "ACTIVE",
/// ```
///
/// The `EnumVariantIR` would have:
/// - `name`: `"Active"`
/// - `value`: `EnumValue::String("ACTIVE")`
/// - `decorators`: Contains the `@default` decorator
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EnumVariantIR {
    /// The variant name (identifier).
    pub name: String,

    /// Source span of the variant declaration.
    pub span: SpanIR,

    /// The initializer value for this variant.
    /// See [`EnumValue`] for the different value types.
    pub value: EnumValue,

    /// Decorators applied to this variant (from JSDoc comments).
    /// Useful for variant-level metadata like `@default` or `@deprecated`.
    pub decorators: Vec<DecoratorIR>,
}
