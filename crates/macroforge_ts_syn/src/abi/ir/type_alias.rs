//! Type alias IR types for TypeScript type alias declarations.
//!
//! This module provides the intermediate representation for TypeScript type aliases
//! (`type X = ...`), supporting various forms including unions, intersections,
//! object types, tuples, and simple aliases.
//!
//! ## Supported Type Alias Forms
//!
//! ```typescript
//! // Union type
//! type Status = "active" | "inactive" | "pending";
//!
//! // Intersection type
//! type AdminUser = User & { role: "admin" };
//!
//! // Object type literal
//! type Point = { x: number; y: number };
//!
//! // Tuple type
//! type Pair<T, U> = [T, U];
//!
//! // Simple alias
//! type ID = string;
//! ```
//!
//! ## Working with Type Bodies
//!
//! The [`TypeBody`] enum classifies the type alias body and provides
//! convenient accessor methods:
//!
//! ```rust,no_run
//! use macroforge_ts_syn::{TypeAliasIR, TypeBody};
//!
//! fn process_type_alias(alias: &TypeAliasIR) {
//!     match &alias.body {
//!         TypeBody::Union(members) => {
//!             println!("Union with {} members", members.len());
//!         }
//!         TypeBody::Object { fields } => {
//!             println!("Object with {} fields", fields.len());
//!         }
//!         TypeBody::Alias(target) => {
//!             println!("Alias to {}", target);
//!         }
//!         _ => {}
//!     }
//! }
//! ```

use serde::{Deserialize, Serialize};

use crate::abi::ir::TypeParamIR;
use crate::abi::{DecoratorIR, InterfaceFieldIR, SpanIR};

/// Intermediate representation of a TypeScript type alias declaration.
///
/// Captures the type alias name, type parameters, and the body of the alias.
/// The body is classified into different [`TypeBody`] variants based on
/// the structure of the type.
///
/// # Example
///
/// For the TypeScript type alias:
///
/// ```typescript
/// /** @derive(Encode) */
/// type Result<T, E> = { ok: T } | { err: E };
/// ```
///
/// The `TypeAliasIR` would have:
/// - `name`: `"Result"`
/// - `type_params`: `T` and `E`
/// - `body`: `TypeBody::Union([...])`
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TypeAliasIR {
    /// The type alias name (identifier).
    pub name: String,

    /// Source span covering the entire type alias declaration.
    pub span: SpanIR,

    /// Decorators applied to the type alias (from JSDoc).
    pub decorators: Vec<DecoratorIR>,

    /// Generic type parameters as declared (`T`, `E` for `type Result<T, E>`).
    pub type_params: Vec<TypeParamIR>,

    /// The body/definition of the type alias.
    pub body: TypeBody,
}

impl TypeAliasIR {
    /// Resets every source span, for comparing declarations by content.
    pub(crate) fn clear_spans(&mut self) {
        self.span = SpanIR::default();
        self.decorators
            .iter_mut()
            .for_each(DecoratorIR::clear_spans);
        self.body.clear_spans();
    }
}

/// The body/definition of a type alias, classified by structure.
///
/// This enum categorizes type alias bodies into their structural form,
/// making it easy to handle different type patterns in macro code.
///
/// # Variant Selection
///
/// The variant is determined by the top-level structure of the type:
///
/// | TypeScript | TypeBody Variant |
/// |------------|------------------|
/// | `A \| B \| C` | `Union` |
/// | `A & B & C` | `Intersection` |
/// | `{ x: T }` | `Object` |
/// | `[A, B]` | `Tuple` |
/// | `SomeType` | `Alias` |
/// | `$Newtype<T>` | `Newtype` |
/// | `Partial<T>` | `Other` (complex) |
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TypeBody {
    /// A union type with multiple alternatives.
    ///
    /// Example: `type Status = "active" | "inactive" | "pending"`
    ///
    /// Each union member is represented as a [`TypeMember`] which can
    /// be a literal, type reference, or inline object.
    Union(Vec<TypeMember>),

    /// An intersection type combining multiple types.
    ///
    /// Example: `type Admin = User & { role: "admin" } & Auditable`
    ///
    /// Each intersection member is a [`TypeMember`].
    Intersection(Vec<TypeMember>),

    /// An object type literal.
    ///
    /// Example: `type Point = { x: number; y: number; z?: number }`
    ///
    /// The fields are represented using [`InterfaceFieldIR`] since they
    /// have the same structure as interface properties.
    Object { fields: Vec<InterfaceFieldIR> },

    /// A tuple type with ordered elements.
    ///
    /// Example: `type Pair = [string, number]`
    ///
    /// Elements are stored as type strings.
    Tuple(Vec<String>),

    /// A simple type alias pointing to another type.
    ///
    /// Example: `type ID = string` or `type UserList = Array<User>`
    ///
    /// The string contains the full type reference.
    Alias(String),

    /// Fallback for complex types not covered by other variants.
    ///
    /// Examples:
    /// - Mapped types: `type Readonly<T> = { readonly [K in keyof T]: T[K] }`
    /// - Conditional types: `type Flatten<T> = T extends Array<infer U> ? U : T`
    /// - Template literal types: `type EventName = \`on${string}\``
    ///
    /// The string contains the raw source of the type.
    Other(String),

    /// A nominal brand over another type, as `$Newtype<T>` declares it.
    ///
    /// Example: `type Meters = $Newtype<number>`
    ///
    /// Holds the branded type. The unexpanded macro call and its expansion
    /// (`T & { readonly [__mf_newtype_N]: true }`) both lower to this, so a
    /// derive sees one shape whichever form it reads.
    Newtype(Box<TypeBody>),
}

impl Default for TypeBody {
    fn default() -> Self {
        TypeBody::Other(String::new())
    }
}

impl TypeBody {
    /// Resets every source span, for comparing declarations by content.
    pub(crate) fn clear_spans(&mut self) {
        match self {
            TypeBody::Union(members) | TypeBody::Intersection(members) => {
                members.iter_mut().for_each(TypeMember::clear_spans);
            }
            TypeBody::Object { fields } => {
                fields.iter_mut().for_each(InterfaceFieldIR::clear_spans);
            }
            TypeBody::Newtype(inner) => inner.clear_spans(),
            TypeBody::Tuple(_) | TypeBody::Alias(_) | TypeBody::Other(_) => {}
        }
    }

    /// Returns true if this is a union type
    pub fn is_union(&self) -> bool {
        matches!(self, TypeBody::Union(_))
    }

    /// Returns true if this is an intersection type
    pub fn is_intersection(&self) -> bool {
        matches!(self, TypeBody::Intersection(_))
    }

    /// Returns true if this is an object type
    pub fn is_object(&self) -> bool {
        matches!(self, TypeBody::Object { .. })
    }

    /// Returns true if this is a tuple type
    pub fn is_tuple(&self) -> bool {
        matches!(self, TypeBody::Tuple(_))
    }

    /// Returns true if this is a simple alias
    pub fn is_alias(&self) -> bool {
        matches!(self, TypeBody::Alias(_))
    }

    /// Returns the union members if this is a union type
    pub fn as_union(&self) -> Option<&[TypeMember]> {
        match self {
            TypeBody::Union(members) => Some(members),
            _ => None,
        }
    }

    /// Returns the intersection members if this is an intersection type
    pub fn as_intersection(&self) -> Option<&[TypeMember]> {
        match self {
            TypeBody::Intersection(members) => Some(members),
            _ => None,
        }
    }

    /// Returns the fields if this is an object type
    pub fn as_object(&self) -> Option<&[InterfaceFieldIR]> {
        match self {
            TypeBody::Object { fields } => Some(fields),
            _ => None,
        }
    }

    /// Returns the elements if this is a tuple type
    pub fn as_tuple(&self) -> Option<&[String]> {
        match self {
            TypeBody::Tuple(elements) => Some(elements),
            _ => None,
        }
    }

    /// Returns the branded type if this is a newtype
    pub fn as_newtype(&self) -> Option<&TypeBody> {
        match self {
            TypeBody::Newtype(inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns the aliased type if this is a simple alias
    pub fn as_alias(&self) -> Option<&str> {
        match self {
            TypeBody::Alias(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the primitive keyword this body brands or aliases.
    ///
    /// Matches a bare primitive (`number`) and a primitive intersected with one
    /// or more object literals, which a primitive value can never carry at
    /// runtime: symbol brands (`number & { readonly [B]: true }`), string-keyed
    /// brands (`number & { readonly __brand: "Meters" }`) and `number & {}`.
    pub fn primitive_base(&self) -> Option<&str> {
        match self {
            TypeBody::Newtype(inner) => inner.primitive_base(),
            TypeBody::Alias(name) | TypeBody::Other(name) => typeof_primitive(name),
            TypeBody::Intersection(members) => {
                let mut base = None;
                let mut branded = false;
                for member in members {
                    match &member.kind {
                        TypeMemberKind::Brand(_) | TypeMemberKind::Object { .. } => branded = true,
                        TypeMemberKind::TypeRef(name) if base.is_none() => {
                            base = Some(typeof_primitive(name)?);
                        }
                        _ => return None,
                    }
                }
                base.filter(|_| branded)
            }
            _ => None,
        }
    }
}

/// The name of the built-in nominal brand macro.
pub const NEWTYPE_MACRO: &str = "$Newtype";

/// The prefix of the `unique symbol` each `$Newtype` expansion brands with.
pub const NEWTYPE_BRAND_PREFIX: &str = "__mf_newtype_";

/// Returns `name` trimmed when it is a primitive keyword whose runtime `typeof`
/// has the same name, and so can be checked and branded.
pub fn typeof_primitive(name: &str) -> Option<&str> {
    let trimmed = name.trim();
    matches!(trimmed, "string" | "number" | "boolean" | "bigint").then_some(trimmed)
}

/// Whether `name` is a primitive keyword whose values are never objects: a
/// [`typeof_primitive`], `null` or `undefined`.
pub fn is_primitive_keyword(name: &str) -> bool {
    typeof_primitive(name).is_some() || matches!(name.trim(), "null" | "undefined")
}

/// A member of a union or intersection type with optional decorators.
///
/// Used to represent individual alternatives in a union type or
/// components of an intersection type. Each member can carry
/// decorators from leading JSDoc comments.
///
/// # Example
///
/// For the union type:
///
/// ```typescript
/// type Status =
///     /** @default */
///     | "active"
///     | "inactive"
///     | { custom: string };
/// ```
///
/// This would be represented as three `TypeMember` instances:
/// 1. `TypeMemberKind::Literal("active")` with `@default` decorator
/// 2. `TypeMemberKind::Literal("inactive")` with no decorators
/// 3. `TypeMemberKind::Object { fields: [...] }` with no decorators
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TypeMember {
    /// The kind of type member (literal, reference, or object).
    pub kind: TypeMemberKind,

    /// Decorators from leading JSDoc comments (e.g., `/** @default */`).
    /// Used for member-level metadata in union/intersection types.
    pub decorators: Vec<DecoratorIR>,
}

/// The structural kind of a type member in unions/intersections.
///
/// Classifies each member of a union or intersection type by its form:
/// - Literal values (`"active"`, `42`, `true`)
/// - Type references (`User`, `Array<T>`)
/// - Inline object types (`{ x: number }`)
/// - Intersection types (`{ variant: 'Account' } & Account`)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TypeMemberKind {
    /// A literal type value.
    ///
    /// Examples: `"active"`, `42`, `true`, `null`
    ///
    /// The string contains the literal as it appears in source.
    Literal(String),

    /// A reference to another type.
    ///
    /// Examples: `User`, `Date`, `Array<T>`, `Map<K, V>`
    ///
    /// The string contains the full type reference including generics.
    TypeRef(String),

    /// An inline object type literal.
    ///
    /// Example: `{ role: string; permissions: string[] }`
    ///
    /// The fields are represented using [`InterfaceFieldIR`].
    Object { fields: Vec<InterfaceFieldIR> },

    /// An intersection type nested inside a union or intersection.
    ///
    /// Example: `{ variant: 'Account' } & Account` as a member of a union.
    ///
    /// The members are the individual parts of the intersection.
    Intersection(Vec<TypeMember>),

    /// An object type literal whose members are all computed symbol keys.
    ///
    /// Example: `{ readonly [MetersBrand]: true }`, the brand half of
    /// `number & { readonly [MetersBrand]: true }`.
    ///
    /// Contains the symbol identifiers in declaration order.
    Brand(Vec<String>),
}

impl TypeMember {
    /// Resets every source span, for comparing declarations by content.
    pub(crate) fn clear_spans(&mut self) {
        self.decorators
            .iter_mut()
            .for_each(DecoratorIR::clear_spans);
        match &mut self.kind {
            TypeMemberKind::Object { fields } => {
                fields.iter_mut().for_each(InterfaceFieldIR::clear_spans);
            }
            TypeMemberKind::Intersection(members) => {
                members.iter_mut().for_each(TypeMember::clear_spans);
            }
            TypeMemberKind::Literal(_) | TypeMemberKind::TypeRef(_) | TypeMemberKind::Brand(_) => {}
        }
    }

    /// Create a new TypeMember with no decorators
    pub fn new(kind: TypeMemberKind) -> Self {
        Self {
            kind,
            decorators: Vec::new(),
        }
    }

    /// Create a new TypeMember with decorators
    pub fn with_decorators(kind: TypeMemberKind, decorators: Vec<DecoratorIR>) -> Self {
        Self { kind, decorators }
    }

    /// Returns true if this is a literal type
    pub fn is_literal(&self) -> bool {
        matches!(self.kind, TypeMemberKind::Literal(_))
    }

    /// Returns true if this is a type reference
    pub fn is_type_ref(&self) -> bool {
        matches!(self.kind, TypeMemberKind::TypeRef(_))
    }

    /// Returns true if this is an object type
    pub fn is_object(&self) -> bool {
        matches!(self.kind, TypeMemberKind::Object { .. })
    }

    /// Returns the literal value if this is a Literal variant
    pub fn as_literal(&self) -> Option<&str> {
        match &self.kind {
            TypeMemberKind::Literal(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the type reference if this is a TypeRef variant
    pub fn as_type_ref(&self) -> Option<&str> {
        match &self.kind {
            TypeMemberKind::TypeRef(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the fields if this is an Object variant
    pub fn as_object(&self) -> Option<&[InterfaceFieldIR]> {
        match &self.kind {
            TypeMemberKind::Object { fields } => Some(fields),
            _ => None,
        }
    }

    /// Returns the type name (for TypeRef or Literal)
    pub fn type_name(&self) -> Option<&str> {
        match &self.kind {
            TypeMemberKind::TypeRef(s) => Some(s),
            TypeMemberKind::Literal(s) => Some(s),
            TypeMemberKind::Object { .. }
            | TypeMemberKind::Intersection(_)
            | TypeMemberKind::Brand(_) => None,
        }
    }

    /// Returns true if this is a symbol brand
    pub fn is_brand(&self) -> bool {
        matches!(self.kind, TypeMemberKind::Brand(_))
    }

    /// Returns the intersection members if this is an Intersection variant
    pub fn as_intersection_members(&self) -> Option<&[TypeMember]> {
        match &self.kind {
            TypeMemberKind::Intersection(members) => Some(members),
            _ => None,
        }
    }

    /// Check if this member has a decorator with the given name
    pub fn has_decorator(&self, name: &str) -> bool {
        self.decorators
            .iter()
            .any(|d| d.name.eq_ignore_ascii_case(name))
    }
}
