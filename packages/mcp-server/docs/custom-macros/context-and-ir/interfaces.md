## Interfaces

Rust

```
pub struct InterfaceIR {
    pub name: String,
    pub span: SpanIR,
    pub body_span: SpanIR,
    pub type_params: Vec<String>,
    pub heritage: Vec<String>,       // extends clauses
    pub decorators: Vec<DecoratorIR>,
    pub fields: Vec<InterfaceFieldIR>,
    pub methods: Vec<InterfaceMethodIR>,
}

pub struct InterfaceFieldIR {
    pub name: String,
    pub span: SpanIR,
    pub ts_type: String,
    pub optional: bool,
    pub readonly: bool,
    pub decorators: Vec<DecoratorIR>,
}

pub struct InterfaceMethodIR {
    pub name: String,
    pub span: SpanIR,
    pub type_params_src: String,
    pub params_src: String,
    pub return_type_src: String,
    pub optional: bool,
    pub decorators: Vec<DecoratorIR>,
}
```
