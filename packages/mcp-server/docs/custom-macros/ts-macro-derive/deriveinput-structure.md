## DeriveInput Structure

```rust
struct DeriveInput {
    pub ident: Ident,           // The type name
    pub span: SpanIR,           // Span of the type definition
    pub attrs: Vec<Attribute>,  // Decorators (excluding @derive)
    pub data: Data,             // The parsed type data
    pub context: MacroContextIR, // Macro context with spans

    // Helper methods
    fn name(&self) -> &str;              // Get the type name
    fn decorator_span(&self) -> SpanIR;  // Span of @derive decorator
    fn as_class(&self) -> Option<&DataClass>;
    fn as_interface(&self) -> Option<&DataInterface>;
    fn as_enum(&self) -> Option<&DataEnum>;
}

enum Data {
    Class(DataClass),
    Interface(DataInterface),
    Enum(DataEnum),
    TypeAlias(DataTypeAlias),
}

impl DataClass {
    fn fields(&self) -> &[FieldIR];
    fn methods(&self) -> &[MethodSigIR];
    fn field_names(&self) -> impl Iterator<Item = &str>;
    fn field(&self, name: &str) -> Option<&FieldIR>;
    fn body_span(&self) -> SpanIR;      // For inserting code into class body
    fn type_params(&self) -> &[String]; // Generic type parameters
    fn heritage(&self) -> &[String];    // extends/implements clauses
    fn is_abstract(&self) -> bool;
}

impl DataInterface {
    fn fields(&self) -> &[InterfaceFieldIR];
    fn methods(&self) -> &[InterfaceMethodIR];
    fn field_names(&self) -> impl Iterator<Item = &str>;
    fn field(&self, name: &str) -> Option<&InterfaceFieldIR>;
    fn body_span(&self) -> SpanIR;
    fn type_params(&self) -> &[String];
    fn heritage(&self) -> &[String];    // extends clauses
}

impl DataEnum {
    fn variants(&self) -> &[EnumVariantIR];
    fn variant_names(&self) -> impl Iterator<Item = &str>;
    fn variant(&self, name: &str) -> Option<&EnumVariantIR>;
}

impl DataTypeAlias {
    fn body(&self) -> &TypeBody;
    fn type_params(&self) -> &[String];
    fn is_union(&self) -> bool;
    fn is_object(&self) -> bool;
    fn as_union(&self) -> Option<&[TypeMember]>;
    fn as_object(&self) -> Option<&[InterfaceFieldIR]>;
}
```