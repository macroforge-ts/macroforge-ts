## Accessing Field Data

### Class Fields (FieldIR)

Rust

```
struct FieldIR {
    pub name: String,               // Field name
    pub span: SpanIR,               // Field span
    pub ts_type: String,            // TypeScript type annotation
    pub optional: bool,             // Whether field has ?
    pub readonly: bool,             // Whether field is readonly
    pub visibility: Visibility,     // Public, Protected, Private
    pub decorators: Vec<DecoratorIR>, // Field decorators
}
```

### Interface Fields (InterfaceFieldIR)

Rust

```
struct InterfaceFieldIR {
    pub name: String,
    pub span: SpanIR,
    pub ts_type: String,
    pub optional: bool,
    pub readonly: bool,
    pub decorators: Vec<DecoratorIR>,
    // Note: No visibility field (interfaces are always public)
}
```

### Enum Variants (EnumVariantIR)

Rust

```
struct EnumVariantIR {
    pub name: String,
    pub span: SpanIR,
    pub value: EnumValue,  // Auto, String(String), or Number(f64)
    pub decorators: Vec<DecoratorIR>,
}
```

### Decorator Structure

Rust

```
struct DecoratorIR {
    pub name: String,      // e.g., "serde"
    pub args_src: String,  // Raw args text, e.g., "skip, rename: 'id'"
    pub span: SpanIR,
}
```

Note

To check for decorators, iterate through `field.decorators` and check `decorator.name`. For parsing options, you can write helper functions like the built-in macros do.