## Enums

Rust

```
pub struct EnumIR {
    pub name: String,
    pub span: SpanIR,
    pub body_span: SpanIR,
    pub decorators: Vec<DecoratorIR>,
    pub variants: Vec<EnumVariantIR>,
    pub is_const: bool,              // declared const enum
}

pub struct EnumVariantIR {
    pub name: String,
    pub span: SpanIR,
    pub value: EnumValue,
    pub decorators: Vec<DecoratorIR>,
}

pub enum EnumValue {
    String(String),   // Active = "active"
    Number(f64),      // Low = 1
    Auto,             // no initializer: one more than the previous member
    Expr(String),     // Mask = A | B, kept as source text
}
```

`EnumValue` has a test and an accessor for each kind: `is_string()`/`as_string()`,
`is_number()`/`as_number()`, `is_expr()`/`as_expr()` and `is_auto()`.
