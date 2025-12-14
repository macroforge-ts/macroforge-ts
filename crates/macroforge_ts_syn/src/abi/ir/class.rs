//! Class-related IR types for TypeScript class declarations.
//!
//! This module provides the intermediate representation for TypeScript classes,
//! including their fields, methods, and associated metadata.
//!
//! ## Example
//!
//! Given this TypeScript class:
//!
//! ```typescript
//! /** @derive(Debug, Clone) */
//! class User {
//!     public name: string;
//!     private age: number;
//!
//!     constructor(name: string, age: number) {
//!         this.name = name;
//!         this.age = age;
//!     }
//!
//!     greet(): string {
//!         return `Hello, ${this.name}!`;
//!     }
//! }
//! ```
//!
//! The resulting [`ClassIR`] would contain:
//! - `name`: `"User"`
//! - `fields`: Two [`FieldIR`] entries for `name` and `age`
//! - `methods`: Two [`MethodSigIR`] entries for `constructor` and `greet`

use serde::{Deserialize, Serialize};

use crate::abi::{DecoratorIR, SpanIR, swc_ast};

/// Intermediate representation of a TypeScript class declaration.
///
/// This struct captures all relevant information about a class that macro
/// authors typically need: the class name, its fields, methods, inheritance
/// chain, type parameters, and decorators.
///
/// # Fields
///
/// - `name` - The class identifier (e.g., `"User"`, `"MyComponent"`)
/// - `span` - Source span covering the entire class declaration
/// - `body_span` - Source span of just the class body (between `{` and `}`)
/// - `is_abstract` - Whether the class is declared as `abstract`
/// - `type_params` - Generic type parameters (e.g., `["T", "U"]` for `class Foo<T, U>`)
/// - `heritage` - Extended/implemented types (e.g., `["BaseClass", "ISerializable"]`)
/// - `decorators` - Decorators applied to the class
/// - `fields` - Class properties/fields
/// - `methods` - Class methods and constructor
///
/// # Example
///
/// ```rust,ignore
/// use macroforge_ts_syn::ClassIR;
///
/// fn generate_debug(class: &ClassIR) -> String {
///     let mut output = format!("class {} {{\n", class.name);
///
///     // Generate debug output for each field
///     for field in &class.fields {
///         output.push_str(&format!("  {}: {},\n", field.name, field.ts_type));
///     }
///
///     output.push_str("}\n");
///     output
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClassIR {
    /// The class name (identifier).
    pub name: String,

    /// Source span covering the entire class declaration.
    pub span: SpanIR,

    /// Source span of the class body (between `{` and `}`).
    /// Useful for inserting generated methods inside the class.
    pub body_span: SpanIR,

    /// Whether this is an `abstract class`.
    pub is_abstract: bool,

    /// Generic type parameters (e.g., `["T"]` for `class Foo<T>`).
    pub type_params: Vec<String>,

    /// Heritage clauses - extended classes and implemented interfaces.
    /// For `class Foo extends Bar implements IBaz`, this would be `["Bar", "IBaz"]`.
    pub heritage: Vec<String>,

    /// Decorators applied to the class (from JSDoc or decorator syntax).
    pub decorators: Vec<DecoratorIR>,

    /// The raw SWC decorator AST nodes (not serialized).
    #[serde(skip)]
    pub decorators_ast: Vec<swc_ast::Decorator>,

    /// Class fields/properties.
    pub fields: Vec<FieldIR>,

    /// Class methods (including constructor).
    pub methods: Vec<MethodSigIR>,

    /// Raw SWC class members (not serialized).
    /// Includes all members for advanced use cases.
    #[serde(skip)]
    pub members: Vec<swc_ast::ClassMember>,
}

/// Intermediate representation of a class field/property.
///
/// Represents a property declaration within a TypeScript class, including
/// its type annotation, modifiers, and any decorators.
///
/// # Example
///
/// For the TypeScript field:
///
/// ```typescript
/// /** @serde(skip) */
/// private readonly email?: string;
/// ```
///
/// The `FieldIR` would have:
/// - `name`: `"email"`
/// - `ts_type`: `"string"`
/// - `optional`: `true`
/// - `readonly`: `true`
/// - `visibility`: `Visibility::Private`
/// - `decorators`: Contains the `@serde(skip)` decorator
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FieldIR {
    /// The field name (identifier).
    pub name: String,

    /// Source span of the field declaration.
    pub span: SpanIR,

    /// The type annotation as a string (e.g., `"string"`, `"number[]"`, `"Map<K, V>"`).
    /// Stored as a string for simplicity in v0.
    pub ts_type: String,

    /// The raw SWC type annotation AST (not serialized).
    /// Available for advanced type analysis.
    #[serde(skip)]
    pub type_ann: Option<Box<swc_ast::TsType>>,

    /// Whether the field is optional (`?`).
    pub optional: bool,

    /// Whether the field is marked `readonly`.
    pub readonly: bool,

    /// The access modifier (`public`, `protected`, or `private`).
    pub visibility: Visibility,

    /// Decorators applied to this field.
    pub decorators: Vec<DecoratorIR>,

    /// The raw SWC class property AST (not serialized).
    #[serde(skip)]
    pub prop_ast: Option<swc_ast::ClassProp>,
}

/// Intermediate representation of a class method signature.
///
/// Captures the signature of a class method (including constructors) without
/// the method body. The signature components are stored as source strings
/// for easy code generation.
///
/// # Example
///
/// For the TypeScript method:
///
/// ```typescript
/// async fetchUser<T>(id: string): Promise<T> {
///     // ...
/// }
/// ```
///
/// The `MethodSigIR` would have:
/// - `name`: `"fetchUser"`
/// - `type_params_src`: `"<T>"`
/// - `params_src`: `"id: string"`
/// - `return_type_src`: `"Promise<T>"`
/// - `is_async`: `true`
/// - `is_static`: `false`
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MethodSigIR {
    /// The method name (or `"constructor"` for constructors).
    pub name: String,

    /// Source span of the method declaration.
    pub span: SpanIR,

    /// Type parameters as source string (e.g., `"<T>"`, `"<T, U extends Foo>"`).
    /// Empty string if no type parameters.
    pub type_params_src: String,

    /// Method parameters as source string (e.g., `"name: string, age: number"`).
    pub params_src: String,

    /// Return type as source string (e.g., `"void"`, `"Promise<User>"`).
    /// Empty string if no explicit return type.
    pub return_type_src: String,

    /// Whether this is a static method.
    pub is_static: bool,

    /// Whether this is an async method.
    pub is_async: bool,

    /// The access modifier (`public`, `protected`, or `private`).
    pub visibility: Visibility,

    /// Decorators applied to this method.
    pub decorators: Vec<DecoratorIR>,

    /// The raw SWC method AST (not serialized).
    /// Provides access to the full method including body.
    #[serde(skip)]
    pub member_ast: Option<MethodAstIR>,
}

/// Container for method AST variants.
///
/// TypeScript/JavaScript classes have two kinds of method-like members:
/// regular methods and constructors. This enum allows unified handling
/// while preserving the distinction.
#[derive(Clone, Debug, PartialEq)]
pub enum MethodAstIR {
    /// A regular class method.
    Method(swc_ast::ClassMethod),
    /// The class constructor.
    Constructor(swc_ast::Constructor),
}

/// Access modifier for class members.
///
/// Represents the visibility level of a class field or method in TypeScript.
/// When no modifier is specified, TypeScript defaults to `public`.
///
/// # Examples
///
/// ```typescript
/// class Example {
///     public name: string;      // Visibility::Public
///     protected id: number;     // Visibility::Protected
///     private secret: string;   // Visibility::Private
///     age: number;              // Visibility::Public (implicit)
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Visibility {
    /// Accessible from anywhere.
    Public,
    /// Accessible from the class and its subclasses.
    Protected,
    /// Accessible only within the class.
    Private,
}
