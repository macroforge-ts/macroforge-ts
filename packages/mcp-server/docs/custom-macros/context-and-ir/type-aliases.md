## Type Aliases

Rust

```
pub struct TypeAliasIR {
    pub name: String,
    pub span: SpanIR,
    pub decorators: Vec<DecoratorIR>,
    pub type_params: Vec<TypeParamIR>,
    pub body: TypeBody,
}

pub enum TypeBody {
    Union(Vec<TypeMember>),          // A | B | "c"
    Intersection(Vec<TypeMember>),   // A & B
    Object { fields: Vec<InterfaceFieldIR> }, // { a: string }
    Tuple(Vec<String>),              // [string, number]
    Alias(String),                   // Other<T>
    Other(String),                   // anything else, as source text
}

pub struct TypeMember {
    pub kind: TypeMemberKind,
    pub decorators: Vec<DecoratorIR>, // JSDoc tags on a union member
}

pub enum TypeMemberKind {
    Literal(String),                 // "active", 42
    TypeRef(String),                 // User
    Object { fields: Vec<InterfaceFieldIR> },
    Intersection(Vec<TypeMember>),
}
```

`TypeBody` has `is_union()`, `is_intersection()`, `is_object()`, `is_tuple()` and `is_alias()`, each
with an `as_...()` accessor. `TypeMember` has `is_literal()`, `is_type_ref()`, `is_object()`, their
`as_...()` accessors, `as_intersection_members()`, `type_name()` (the name of a literal or type
reference) and `has_decorator(name)`.
