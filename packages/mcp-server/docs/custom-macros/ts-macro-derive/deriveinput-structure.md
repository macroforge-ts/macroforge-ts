## DeriveInput Structure

Rust

```
struct DeriveInput {
    pub ident: Ident,            // The type name and its span
    pub span: SpanIR,            // Span of the type definition
    pub attrs: Vec<Attribute>,   // Decorators (excluding @derive)
    pub data: Data,              // The parsed type data
    pub context: MacroContextIR, // The macro context

    // Helper methods
    fn name(&self) -> &str;                       // The type name
    fn as_class(&self) -> Option<&DataClass>;
    fn as_interface(&self) -> Option<&DataInterface>;
    fn as_enum(&self) -> Option<&DataEnum>;
    fn as_type_alias(&self) -> Option<&DataTypeAlias>;
    fn decorator_span(&self) -> SpanIR;           // The whole @derive(...)
    fn macro_name_span(&self) -> Option<SpanIR>;  // This macro's name inside it
    fn error_span(&self) -> SpanIR;               // Where to point an error
    fn target_span(&self) -> SpanIR;              // The declaration
    fn body_span(&self) -> Option<SpanIR>;        // The body; None for enums and type aliases
    fn from_context(ctx: MacroContextIR) -> Result<Self, TsSynError>;
}

struct Attribute {
    fn name(&self) -> &str;   // e.g. "endec"
    fn args(&self) -> &str;   // the arguments as written
    fn span(&self) -> SpanIR;
}

enum Data {
    Class(DataClass),
    Interface(DataInterface),
    Enum(DataEnum),
    TypeAlias(DataTypeAlias),
}

impl DataClass {
    fn fields(&self) -> &[FieldIR];
    fn methods(&self) -> &[MethodSigIR];
    fn field_names(&self) -> impl Iterator<Item = &str>;
    fn field(&self, name: &str) -> Option<&FieldIR>;
    fn method(&self, name: &str) -> Option<&MethodSigIR>;
    fn body_span(&self) -> SpanIR;      // For inserting code into class body
    fn type_params(&self) -> &[String]; // Generic type parameters
    fn heritage(&self) -> &[String];    // extends/implements clauses
    fn is_abstract(&self) -> bool;
}

impl DataInterface {
    fn fields(&self) -> &[InterfaceFieldIR];
    fn methods(&self) -> &[InterfaceMethodIR];
    fn field_names(&self) -> impl Iterator<Item = &str>;
    fn field(&self, name: &str) -> Option<&InterfaceFieldIR>;
    fn method(&self, name: &str) -> Option<&InterfaceMethodIR>;
    fn body_span(&self) -> SpanIR;
    fn type_params(&self) -> &[String];
    fn heritage(&self) -> &[String];    // extends clauses
}

impl DataEnum {
    fn variants(&self) -> &[EnumVariantIR];
    fn variant_names(&self) -> impl Iterator<Item = &str>;
    fn variant(&self, name: &str) -> Option<&EnumVariantIR>;
}

impl DataTypeAlias {
    fn body(&self) -> &TypeBody;
    fn type_params(&self) -> &[String];
    fn is_union(&self) -> bool;
    fn is_intersection(&self) -> bool;
    fn is_object(&self) -> bool;
    fn is_tuple(&self) -> bool;
    fn is_alias(&self) -> bool;
    fn as_union(&self) -> Option<&[TypeMember]>;
    fn as_intersection(&self) -> Option<&[TypeMember]>;
    fn as_object(&self) -> Option<&[InterfaceFieldIR]>;
    fn as_tuple(&self) -> Option<&[String]>;
    fn as_alias(&self) -> Option<&str>;
}
```

Each `Data*` wrapper keeps the full IR in its `inner` field; see
[Context and IR](../../docs/custom-macros/context-and-ir) for every type.

Note

Inside a template, write the type's name with `@{input.name()}`. `input.ident` is a `ts_syn::Ident`,
which records where the name is, and templates do not interpolate it; `ts_ident!(...)` makes the
identifier type that they do.
