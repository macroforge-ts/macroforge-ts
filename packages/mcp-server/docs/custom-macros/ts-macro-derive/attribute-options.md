## Attribute Options

### Name (Required)

The first argument is the macro name that users will reference in `@derive()`:

Rust

```
#[ts_macro_derive(JSON)]  // Users write: @derive(JSON)
pub fn derive_json(...)
```

### Description

Provides documentation for the macro, shown by editors and in the macro manifest:

Rust

```
#[ts_macro_derive(
    JSON,
    description = "Generates toJSON() returning a plain object"
)]
pub fn derive_json(...)
```

### Attributes

Declare which field-level decorators your macro accepts. Each can carry its own description, shown
when a user hovers the decorator:

Rust

```
#[ts_macro_derive(
    Debug,
    description = "Generates toString()",
    attributes(
        debug,                                       // allows @debug(...) on fields
        (redact, "Hides the field's value in toString()"),
    )
)]
pub fn derive_debug(...)
```

Note

Declared attributes become available as `@attributeName({ options })` decorators in TypeScript.

### Kind

`kind` is `"derive"` by default. [`#[ts_macro]`](../../docs/custom-macros/ts-macro) and
[`#[ts_macro_attribute]`](../../docs/custom-macros/ts-macro-attribute) set it to `"call"` and
`"attribute"` for you, so writing it out is rarely needed.
