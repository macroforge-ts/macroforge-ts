//! IR for the macro directives written in JSDoc comments.
//!
//! A directive is a `@name` or `@name(args)` inside the `/** ... */` comment
//! above a declaration or field. Native TypeScript decorators are not macro
//! syntax: they are left in the source as ordinary TypeScript.
//!
//! ## Example
//!
//! ```typescript
//! /** @derive(Debug, Clone) */
//! class User {
//!     /** @serde(rename = "user_name") */
//!     name: string;
//! }
//! ```
//!
//! This would produce:
//! - Class-level: `DecoratorIR { name: "Derive", args_src: "Debug, Clone", ... }`
//! - Field-level: `DecoratorIR { name: "serde", args_src: "rename = \"user_name\"", ... }`

use serde::{Deserialize, Serialize};

use crate::abi::SpanIR;

#[cfg(feature = "swc")]
use crate::abi::swc_ast;

/// Intermediate representation of a JSDoc macro directive.
///
/// Captures the decorator name and its arguments as raw source text,
/// allowing macro authors to parse arguments according to their own schema.
///
/// # Name Normalization
///
/// Names are preserved with their original casing, with one exception: a
/// `@derive` directive (matched case-insensitively) is normalized to
/// `"Derive"` during lowering. All other names are stored exactly as written
/// (`@serde` -> `"serde"`, `@Entity` -> `"Entity"`).
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

    /// Always `None`: it carried a native decorator's AST, and native
    /// decorators are not lowered.
    #[cfg(feature = "swc")]
    #[serde(skip)]
    pub node: Option<swc_ast::Decorator>,
}
