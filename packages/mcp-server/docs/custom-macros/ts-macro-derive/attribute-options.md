## Attribute Options

### Name (Required)

The first argument is the macro name that users will reference in `@derive()`:

Rust

```
#[ts_macro_derive(JSON)]  // Users write: @derive(JSON)
pub fn derive_json(...)
```

### Description

Provides documentation for the macro:

Rust

```
#[ts_macro_derive(
    JSON,
    description = "Generates toJSON() returning a plain object"
)]
pub fn derive_json(...)
```

### Attributes

Declare which field-level decorators your macro accepts:

Rust

```
#[ts_macro_derive(
    Debug,
    description = "Generates toString()",
    attributes(debug)  // Allows @debug({ ... }) on fields
)]
pub fn derive_debug(...)
```

Note

Declared attributes become available as `@attributeName({ options })` decorators in TypeScript.