//! Function declaration IR types for attribute macros.
//!
//! These types represent TypeScript function declarations in a form
//! that attribute macros can work with. The layout mirrors [`ClassIR`]
//! and [`InterfaceIR`] so the macro author's mental model carries over.

use serde::{Deserialize, Serialize};

use crate::abi::ir::decorators::DecoratorIR;
use crate::abi::span::SpanIR;

/// IR representation of a TypeScript function declaration.
///
/// Covers `function`, `async function`, `function*`, and exported variants.
/// Arrow functions and function expressions are NOT represented here — they
/// are expressions, not items, and have no natural place for a JSDoc decorator.
///
/// # Spans
///
/// ```text
/// export async function foo<T>(x: T, y: number): Promise<T> { return x; }
/// ╰─────────────────────── span ──────────────────────────────────────────╯
/// ╰──────────── signature_span ─────────────────────────╯
///                                                         ╰─ body_span ─╯
/// ```
///
/// - `span`: entire declaration including leading `export`/`async` keywords
///   through the closing `}`.
/// - `signature_span`: everything before the body's opening `{`.
/// - `body_span`: the `{ … }` block (inclusive of braces).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FunctionIR {
    /// The function name.
    pub name: String,

    /// Span of the entire declaration.
    pub span: SpanIR,

    /// Span of the body `{ … }` block (inclusive of braces).
    pub body_span: SpanIR,

    /// Span of everything before the opening `{`.
    pub signature_span: SpanIR,

    /// Whether the function is `async`.
    pub is_async: bool,

    /// Whether the function is a generator (`function*`).
    pub is_generator: bool,

    /// Whether the function has an `export` keyword.
    pub is_exported: bool,

    /// Whether the function is `export default`.
    pub is_default_export: bool,

    /// Generic type parameters (e.g., `["T", "U"]`).
    pub type_params: Vec<String>,

    /// Formal parameters.
    pub params: Vec<FunctionParamIR>,

    /// Return type annotation as source string (e.g., `"Promise<T>"`).
    /// Empty string if no explicit return type.
    pub return_type_src: String,

    /// Raw source of the body (between but not including the braces).
    pub body_src: String,

    /// Decorators applied to this function (JSDoc or native).
    pub decorators: Vec<DecoratorIR>,
}

/// IR representation of a single function parameter.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FunctionParamIR {
    /// The parameter name.
    pub name: String,

    /// Source span of the parameter.
    pub span: SpanIR,

    /// Type annotation as source string. Empty if no annotation.
    pub type_src: String,

    /// Default value as source string, if present.
    pub default_src: Option<String>,

    /// Whether the parameter is optional (`?`).
    pub is_optional: bool,

    /// Whether the parameter is a rest parameter (`...`).
    pub is_rest: bool,

    /// Decorators applied to this parameter.
    pub decorators: Vec<DecoratorIR>,
}
