# ts_macro_derive

*The `#[ts_macro_derive]` attribute is a Rust procedural macro that registers your function as a Macroforge derive macro.*

## Basic Syntax

```rust
use macroforge_ts::ts_macro_derive::ts_macro_derive;
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
use macroforge_ts::ts_syn::{DeriveInput, Data, parse_ts_macro_input};

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
    // Get the item name (class/interface name)
    fn name(&self) -> &str;

    // Get the span of the @derive decorator
    fn decorator_span(&self) -> Span;

    // The item data (class, interface, or enum)
    data: Data,
}

enum Data {
    Class(ClassData),
    Interface(InterfaceData),
    Enum(EnumData),
}

struct ClassData {
    fn fields(&self) -> &[FieldData];
    fn field_names(&self) -> impl Iterator<Item = &str>;
}
```

## Accessing Field Data

```rust
struct FieldData {
    pub name: String,           // Field name
    pub ts_type: String,        // TypeScript type annotation
    pub decorators: Vec<Decorator>, // Field decorators

    // Check if field has a specific decorator
    fn has_decorator(&self, name: &str) -> bool;

    // Get decorator options as JSON
    fn get_decorator_options(&self, name: &str) -> Option<serde_json::Value>;
}
```

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
use macroforge_ts::ts_macro_derive::ts_macro_derive;
use macroforge_ts::ts_quote::body;
use macroforge_ts::ts_syn::{
    Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input,
};

#[ts_macro_derive(
    Validate,
    description = "Generates a validate() method",
    attributes(validate)
)]
pub fn derive_validate(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let validations: Vec<_> = class.fields()
                .iter()
                .filter(|f| f.has_decorator("validate"))
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

- [Learn the ts_quote template syntax](/docs/custom-macros/ts-quote)