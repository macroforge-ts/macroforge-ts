use serde::{Deserialize, Serialize};

use crate::abi::{DecoratorIR, InterfaceFieldIR, SpanIR};

/// Represents a TypeScript type alias declaration
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq)]
pub struct TypeAliasIR {
    pub name: String,
    pub span: SpanIR,
    pub decorators: Vec<DecoratorIR>,
    pub type_params: Vec<String>,
    pub body: TypeBody,
}

/// The body/definition of a type alias
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq)]
pub enum TypeBody {
    /// Union type: `type Status = "active" | "inactive"`
    Union(Vec<TypeMember>),

    /// Intersection type: `type Admin = User & { role: string }`
    Intersection(Vec<TypeMember>),

    /// Object/type literal: `type Point = { x: number; y: number }`
    Object { fields: Vec<InterfaceFieldIR> },

    /// Tuple type: `type Pair = [string, number]`
    Tuple(Vec<String>),

    /// Simple type alias: `type ID = string`
    Alias(String),

    /// Fallback for complex types (mapped, conditional, etc.)
    Other(String),
}

impl Default for TypeBody {
    fn default() -> Self {
        TypeBody::Other(String::new())
    }
}

impl TypeBody {
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

    /// Returns the aliased type if this is a simple alias
    pub fn as_alias(&self) -> Option<&str> {
        match self {
            TypeBody::Alias(s) => Some(s),
            _ => None,
        }
    }
}

/// A member of a union or intersection type
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq)]
pub enum TypeMember {
    /// A literal type: `"active"`, `42`, `true`
    Literal(String),

    /// A type reference: `User`, `Date`, `Array<T>`
    TypeRef(String),

    /// An inline object type: `{ role: string }`
    Object { fields: Vec<InterfaceFieldIR> },
}

impl TypeMember {
    /// Returns true if this is a literal type
    pub fn is_literal(&self) -> bool {
        matches!(self, TypeMember::Literal(_))
    }

    /// Returns true if this is a type reference
    pub fn is_type_ref(&self) -> bool {
        matches!(self, TypeMember::TypeRef(_))
    }

    /// Returns true if this is an object type
    pub fn is_object(&self) -> bool {
        matches!(self, TypeMember::Object { .. })
    }

    /// Returns the literal value if this is a Literal variant
    pub fn as_literal(&self) -> Option<&str> {
        match self {
            TypeMember::Literal(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the type reference if this is a TypeRef variant
    pub fn as_type_ref(&self) -> Option<&str> {
        match self {
            TypeMember::TypeRef(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the fields if this is an Object variant
    pub fn as_object(&self) -> Option<&[InterfaceFieldIR]> {
        match self {
            TypeMember::Object { fields } => Some(fields),
            _ => None,
        }
    }
}
