//! Decorator/attribute IR types for TypeScript decorators.
//!
//! This module provides the intermediate representation for decorators
//! and JSDoc-based attributes applied to TypeScript declarations.
//!
//! ## Decorator Sources
//!
//! Decorators can come from two sources:
//!
//! 1. **TypeScript decorators**: `@Decorator(args)`
//! 2. **JSDoc comments**: `/** @decorator(args) */`
//!
//! Both are represented uniformly using [`DecoratorIR`].
//!
//! ## Example
//!
//! ```typescript
//! /** @derive(Debug, Clone) */
//! @Entity("users")
//! class User {
//!     /** @serde(rename = "user_name") */
//!     name: string;
//! }
//! ```
//!
//! This would produce:
//! - Class-level: `DecoratorIR { name: "derive", args_src: "Debug, Clone", ... }`
//! - Class-level: `DecoratorIR { name: "Entity", args_src: "\"users\"", ... }`
//! - Field-level: `DecoratorIR { name: "serde", args_src: "rename = \"user_name\"", ... }`

use serde::{Deserialize, Serialize};

use crate::abi::{SpanIR, swc_ast};

/// Intermediate representation of a decorator or JSDoc attribute.
///
/// Captures the decorator name and its arguments as raw source text,
/// allowing macro authors to parse arguments according to their own schema.
///
/// # Name Normalization
///
/// Decorator names from JSDoc are typically lowercase (`@derive`, `@serde`),
/// while TypeScript decorators preserve their original casing (`@Entity`).
/// Macro systems may normalize these for case-insensitive matching.
///
/// # Arguments
///
/// Arguments are stored as raw source text (`args_src`) rather than
/// being parsed into a structured format. This allows each macro to
/// define its own argument syntax.
///
/// # Example
///
/// ```rust,no_run
/// use macroforge_ts_syn::DecoratorIR;
///
/// fn has_skip_decorator(decorators: &[DecoratorIR]) -> bool {
///     decorators.iter().any(|d| {
///         d.name.eq_ignore_ascii_case("serde") &&
///         d.args_src.contains("skip")
///     })
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DecoratorIR {
    /// The decorator name without the `@` prefix.
    ///
    /// Examples: `"derive"`, `"serde"`, `"Entity"`, `"deprecated"`
    pub name: String,

    /// Raw arguments text (everything inside the parentheses).
    ///
    /// Examples:
    /// - `@derive(Debug, Clone)` -> `"Debug, Clone"`
    /// - `@serde(rename = "id")` -> `"rename = \"id\""`
    /// - `@Entity("users")` -> `"\"users\""`
    /// - `@deprecated` -> `""` (no arguments)
    pub args_src: String,

    /// Source span of the decorator.
    pub span: SpanIR,

    /// The raw SWC decorator AST node (not serialized).
    /// Only available for TypeScript decorator syntax, not JSDoc.
    #[serde(skip)]
    pub node: Option<swc_ast::Decorator>,
}
