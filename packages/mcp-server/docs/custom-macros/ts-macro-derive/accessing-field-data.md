## Accessing Field Data

### Class Fields (FieldIR)

Rust

```
struct FieldIR {
    pub name: String,               // Field name
    pub key: MemberKey,             // Named, Private (#name) or Unnamed ([KEY], [key: string])
    pub span: SpanIR,               // Field span
    pub ts_type: String,            // Annotation, else what the initializer makes plain, else "any"
    pub optional: bool,             // Whether field has ?
    pub readonly: bool,             // Whether field is readonly
    pub visibility: Visibility,     // Public, Protected, Private
    pub decorators: Vec<DecoratorIR>, // Field decorators
    pub initializer: Option<String>,  // Initializer as written, e.g. "dark"
    pub accessor: bool,             // Declared with the accessor keyword
    pub untyped: bool,              // No annotation and no plain initializer: ts_type is "any"
}
```

### Interface Fields (InterfaceFieldIR)

Rust

```
struct InterfaceFieldIR {
    pub name: String,
    pub key: MemberKey,
    pub span: SpanIR,
    pub ts_type: String,
    pub optional: bool,
    pub readonly: bool,
    pub decorators: Vec<DecoratorIR>,
    // Note: No visibility field (interfaces are always public)
}
```

Note

Every member a type declares is listed. One under a computed key or an index signature has no name
to read it by: its `key` is `MemberKey::Unnamed` and its `name` is the key as written, brackets
included. A `#private` field has `MemberKey::Private` and keeps its `#`. Skip or reject what your
macro cannot reach.

### Enum Variants (EnumVariantIR)

Rust

```
struct EnumVariantIR {
    pub name: String,
    pub span: SpanIR,
    pub value: EnumValue,  // String(String), Number(f64), Auto or Expr(String)
    pub decorators: Vec<DecoratorIR>,
}
```

### Decorator Structure

Rust

```
struct DecoratorIR {
    pub name: String,      // e.g., "endec"
    pub args_src: String,  // Raw args text, e.g., "skip, rename: 'id'"
    pub span: SpanIR,
}
```

Note

To check for decorators, iterate through `field.decorators` and check `decorator.name`. `has_flag`
and `extract_named_string` in `macroforge_ts::builtin::derive::common` read options out of
`args_src`; see [Decorators](../../docs/custom-macros/context-and-ir#decorators).
