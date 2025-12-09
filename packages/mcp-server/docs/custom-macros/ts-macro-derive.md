# ts_macro_derive

*The `#[ts_macro_derive]` attribute is a Rust procedural macro that registers your function as a Macroforge derive macro.*

## Basic Syntax

```rust
use macroforge_ts::macros::ts_macro_derive;
use macroforge_ts::ts_syn::{TsStream, MacroforgeError};

#[ts_macro_derive(MacroName)]
pub fn my_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    // Macro implementation
}
```

## Attribute Options

### Name (Required)

The first argument is the macro name that users will reference in `@derive()`:

```rust
#[ts_macro_derive(JSON)]  // Users write: @derive(JSON)
pub fn derive_json(...)
```

### Description

Provides documentation for the macro:

```rust
#[ts_macro_derive(
    JSON,
    description = "Generates toJSON() returning a plain object"
)]
pub fn derive_json(...)
```

### Attributes

Declare which field-level decorators your macro accepts:

```rust
#[ts_macro_derive(
    Debug,
    description = "Generates toString()",
    attributes(debug)  // Allows @debug({ ... }) on fields
)]
pub fn derive_debug(...)
```

>
> Declared attributes become available as `@attributeName(&#123; options &#125;)` decorators in TypeScript.

## Function Signature

```rust
pub fn my_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError>
```

| `input: TsStream` 
| Token stream containing the class/interface AST 

| `Result<TsStream, MacroforgeError>` 
| Returns generated code or an error with source location

## Parsing Input

Use `parse_ts_macro_input!` to convert the token stream:

```rust
use macroforge_ts::ts_syn::{Data, DeriveInput, parse_ts_macro_input};

#[ts_macro_derive(MyMacro)]
pub fn my_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    // Access class data
    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let fields = class.fields();
            // ...
        }
        Data::Interface(interface) => {
            // Handle interfaces
        }
        Data::Enum(_) => {
            // Handle enums (if supported)
        }
    }
}
```

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

## Accessing Field Data

### Class Fields (FieldIR)

```rust
struct FieldIR {
    pub name: String,               // Field name
    pub span: SpanIR,               // Field span
    pub ts_type: String,            // TypeScript type annotation
    pub optional: bool,             // Whether field has ?
    pub readonly: bool,             // Whether field is readonly
    pub visibility: Visibility,     // Public, Protected, Private
    pub decorators: Vec<DecoratorIR>, // Field decorators
}
```

### Interface Fields (InterfaceFieldIR)

```rust
struct InterfaceFieldIR {
    pub name: String,
    pub span: SpanIR,
    pub ts_type: String,
    pub optional: bool,
    pub readonly: bool,
    pub decorators: Vec<DecoratorIR>,
    // Note: No visibility field (interfaces are always public)
}
```

### Enum Variants (EnumVariantIR)

```rust
struct EnumVariantIR {
    pub name: String,
    pub span: SpanIR,
    pub value: EnumValue,  // Auto, String(String), or Number(f64)
    pub decorators: Vec<DecoratorIR>,
}
```

### Decorator Structure

```rust
struct DecoratorIR {
    pub name: String,      // e.g., "serde"
    pub args_src: String,  // Raw args text, e.g., "skip, rename: 'id'"
    pub span: SpanIR,
}
```

>
> To check for decorators, iterate through `field.decorators` and check `decorator.name`. For parsing options, you can write helper functions like the built-in macros do.

## Adding Imports

If your macro generates code that requires imports, use the `add_import` method on `TsStream`:

```rust
// Add an import to be inserted at the top of the file
let mut output = body! {
    validate(): ValidationResult {
        return validateFields(this);
    }
};

// This will add: import { validateFields, ValidationResult } from "my-validation-lib";
output.add_import("validateFields", "my-validation-lib");
output.add_import("ValidationResult", "my-validation-lib");

Ok(output)
```

>
> Imports are automatically deduplicated. If the same import already exists in the file, it won't be added again.

## Returning Errors

Use `MacroforgeError` to report errors with source locations:

```rust
#[ts_macro_derive(ClassOnly)]
pub fn class_only(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(_) => {
            // Generate code...
            Ok(body! { /* ... */ })
        }
        _ => Err(MacroforgeError::new(
            input.decorator_span(),
            "@derive(ClassOnly) can only be used on classes",
        )),
    }
}
```

## Complete Example

```rust
use macroforge_ts::macros::{ts_macro_derive, body};
use macroforge_ts::ts_syn::{
    Data, DeriveInput, FieldIR, MacroforgeError, TsStream, parse_ts_macro_input,
};

// Helper function to check if a field has a decorator
fn has_decorator(field: &FieldIR, name: &str) -> bool {
    field.decorators.iter().any(|d| d.name.eq_ignore_ascii_case(name))
}

#[ts_macro_derive(
    Validate,
    description = "Generates a validate() method",
    attributes(validate)
)]
pub fn derive_validate(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let validations: Vec<_> = class.fields()
                .iter()
                .filter(|f| has_decorator(f, "validate"))
                .collect();

            Ok(body! {
                validate(): string[] {
                    const errors: string[] = [];
                    {#for field in validations}
                        if (!this.@{field.name}) {
                            errors.push("@{field.name} is required");
                        }
                    {/for}
                    return errors;
                }
            })
        }
        _ => Err(MacroforgeError::new(
            input.decorator_span(),
            "@derive(Validate) only works on classes",
        )),
    }
}
```

## Next Steps

- [Learn the template syntax]({base}/docs/custom-macros/ts-quote)