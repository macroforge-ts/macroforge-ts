//! Interface-related IR types for TypeScript interface declarations.
//!
//! This module provides the intermediate representation for TypeScript interfaces,
//! which define the shape of objects without implementation.
//!
//! ## Example
//!
//! Given this TypeScript interface:
//!
//! ```typescript
//! /** @derive(Serialize) */
//! interface User extends Entity {
//!     readonly id: string;
//!     name: string;
//!     email?: string;
//!     greet(message: string): void;
//! }
//! ```
//!
//! The resulting [`InterfaceIR`] would contain:
//! - `name`: `"User"`
//! - `heritage`: `["Entity"]`
//! - `fields`: Three [`InterfaceFieldIR`] entries
//! - `methods`: One [`InterfaceMethodIR`] entry for `greet`

use serde::{Deserialize, Serialize};

use crate::abi::{DecoratorIR, SpanIR};

/// Intermediate representation of a TypeScript interface declaration.
///
/// Similar to [`ClassIR`](crate::ClassIR) but for interface declarations.
/// Interfaces define the shape of objects and can extend other interfaces.
///
/// # Key Differences from ClassIR
///
/// - No visibility modifiers (all members are implicitly public)
/// - No abstract modifier
/// - No implementation bodies for methods
/// - Can only extend (not implement) other interfaces
///
/// # Example
///
/// ```rust
/// use macroforge_ts_syn::InterfaceIR;
///
/// fn generate_type_guard(iface: &InterfaceIR) -> String {
///     let mut checks = Vec::new();
///
///     for field in &iface.fields {
///         if !field.optional {
///             // Generate property existence checks
///             checks.push(format!(
///                 "'{}' in obj",
///                 field.name
///             ));
///         }
///     }
///
///     format!(
///         "function is{}(obj: unknown): obj is {} {{ return {}; }}",
///         iface.name, iface.name, checks.join(" && ")
///     )
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InterfaceIR {
    /// The interface name (identifier).
    pub name: String,

    /// Source span covering the entire interface declaration.
    pub span: SpanIR,

    /// Source span of the interface body (between `{` and `}`).
    pub body_span: SpanIR,

    /// Generic type parameters (e.g., `["T"]` for `interface Foo<T>`).
    pub type_params: Vec<String>,

    /// Extended interfaces (from `extends` clause).
    /// For `interface Foo extends Bar, Baz`, this would be `["Bar", "Baz"]`.
    pub heritage: Vec<String>,

    /// Decorators applied to the interface (from JSDoc).
    pub decorators: Vec<DecoratorIR>,

    /// Interface properties/fields.
    pub fields: Vec<InterfaceFieldIR>,

    /// Interface method signatures.
    pub methods: Vec<InterfaceMethodIR>,
}

/// Intermediate representation of an interface property.
///
/// Represents a property signature within a TypeScript interface.
/// Unlike class fields, interface fields cannot have visibility modifiers
/// (they're always public).
///
/// # Example
///
/// For the TypeScript property:
///
/// ```typescript
/// /** @serde(rename = "user_email") */
/// readonly email?: string;
/// ```
///
/// The `InterfaceFieldIR` would have:
/// - `name`: `"email"`
/// - `ts_type`: `"string"`
/// - `optional`: `true`
/// - `readonly`: `true`
/// - `decorators`: Contains the `@serde(rename = "user_email")` decorator
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InterfaceFieldIR {
    /// The property name (identifier).
    pub name: String,

    /// Source span of the property signature.
    pub span: SpanIR,

    /// The type annotation as a string.
    pub ts_type: String,

    /// Whether the property is optional (`?`).
    pub optional: bool,

    /// Whether the property is marked `readonly`.
    pub readonly: bool,

    /// Decorators applied to this property (from JSDoc).
    pub decorators: Vec<DecoratorIR>,
}

/// Intermediate representation of an interface method signature.
///
/// Captures the signature of an interface method without implementation.
/// Interface methods are always abstract by nature.
///
/// # Example
///
/// For the TypeScript method signature:
///
/// ```typescript
/// process<T>(input: T): Promise<Result<T>>;
/// ```
///
/// The `InterfaceMethodIR` would have:
/// - `name`: `"process"`
/// - `type_params_src`: `"<T>"`
/// - `params_src`: `"input: T"`
/// - `return_type_src`: `"Promise<Result<T>>"`
/// - `optional`: `false`
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InterfaceMethodIR {
    /// The method name (identifier).
    pub name: String,

    /// Source span of the method signature.
    pub span: SpanIR,

    /// Type parameters as source string (e.g., `"<T>"`).
    pub type_params_src: String,

    /// Method parameters as source string.
    pub params_src: String,

    /// Return type as source string.
    pub return_type_src: String,

    /// Whether the method is optional (rare in interfaces).
    pub optional: bool,

    /// Decorators applied to this method (from JSDoc).
    pub decorators: Vec<DecoratorIR>,
}
