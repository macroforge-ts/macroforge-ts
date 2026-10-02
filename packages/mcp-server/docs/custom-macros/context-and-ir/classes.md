## Classes

Rust

```
pub struct ClassIR {
    pub name: String,
    pub span: SpanIR,
    pub body_span: SpanIR,           // the braces and everything between them
    pub is_abstract: bool,
    pub type_params: Vec<String>,    // e.g. ["T", "K extends string"]
    pub heritage: Vec<String>,       // extends and implements clauses
    pub decorators: Vec<DecoratorIR>,
    pub fields: Vec<FieldIR>,
    pub methods: Vec<MethodSigIR>,
}

pub struct FieldIR {
    pub name: String,
    pub span: SpanIR,
    pub ts_type: String,             // the annotation as written, e.g. "string[]"
    pub optional: bool,              // declared with ?
    pub readonly: bool,
    pub visibility: Visibility,      // Public, Protected or Private
    pub decorators: Vec<DecoratorIR>,
}

pub struct MethodSigIR {
    pub name: String,
    pub span: SpanIR,
    pub type_params_src: String,
    pub params_src: String,          // the parameter list as written
    pub return_type_src: String,
    pub is_static: bool,
    pub is_async: bool,
    pub visibility: Visibility,
    pub decorators: Vec<DecoratorIR>,
    pub body_span: Option<SpanIR>,   // None for an abstract method
    pub body_src: Option<String>,
}
```
