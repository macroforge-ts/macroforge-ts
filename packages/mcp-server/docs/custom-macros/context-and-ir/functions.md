## Functions

Attribute macros on functions, and on class methods, receive a `FunctionIR`:

Rust

```
pub struct FunctionIR {
    pub name: String,
    pub span: SpanIR,
    pub body_span: SpanIR,
    pub signature_span: SpanIR,      // from the start to the body's opening brace
    pub is_async: bool,
    pub is_generator: bool,
    pub is_exported: bool,
    pub is_default_export: bool,
    pub type_params: Vec<TypeParamIR>,
    pub params: Vec<FunctionParamIR>,
    pub return_type_src: String,
    pub body_src: String,
    pub decorators: Vec<DecoratorIR>,
}

pub struct FunctionParamIR {
    pub name: String,
    pub span: SpanIR,
    pub type_src: String,
    pub default_src: Option<String>,
    pub is_optional: bool,
    pub is_rest: bool,
    pub decorators: Vec<DecoratorIR>,
}
```
