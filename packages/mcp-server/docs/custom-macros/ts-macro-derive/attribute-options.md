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