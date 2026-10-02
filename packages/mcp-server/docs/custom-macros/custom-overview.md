# Custom Macros

Macroforge allows you to create custom macros in Rust: derive macros, attribute macros and call
macros. Your macros see the declaration they are attached to, can look up any type in the project,
and can generate any TypeScript code.

## Overview

Custom macros are written in Rust and compiled to WebAssembly. The process involves:

1. Creating a Rust crate that builds to WebAssembly
2. Defining macro functions with `#[ts_macro_derive]`, `#[ts_macro_attribute]` or `#[ts_macro]`
3. Using `macroforge_ts_quote` to generate TypeScript code
4. Building and publishing as an npm package

## Quick Example

Rust

```
use macroforge_ts::macros::{ts_macro_derive, ts_template};
use macroforge_ts::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

#[ts_macro_derive(
    JSON,
    description = "Generates toJSON() returning a plain object"
)]
pub fn derive_json(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            Ok(ts_template!(Within {
                toJSON(): Record<string, unknown> {
                    return {
                        {#for field in class.field_names()}
                            @{field}: this.@{field},
                        {/for}
                    };
                }
            }))
        }
        _ => Err(MacroforgeError::new(
            input.error_span(),
            "@derive(JSON) only works on classes",
        )),
    }
}
```

## Using Custom Macros

Once your macro package is published, users can import and use it:

TypeScript

```
/** import macro { JSON } from "@my/macros"; */

/** @derive(JSON) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}

const user = new User("Alice", 30);
console.log(user.toJSON()); // { name: "Alice", age: 30 }
```

Note

The `import macro` comment tells Macroforge which package provides the macro.

## Getting Started

Follow these guides to create your own macros:

- [Set up a Rust macro crate](../docs/custom-macros/rust-setup)
- [Learn the #\[ts\_macro\_derive\] attribute](../docs/custom-macros/ts-macro-derive)
- [Learn the template syntax](../docs/custom-macros/ts-quote)

## Reference

- [`#[ts_macro]`](../docs/custom-macros/ts-macro) and
  [`#[ts_macro_attribute]`](../docs/custom-macros/ts-macro-attribute): call and attribute macros
- [Output and Imports](../docs/custom-macros/output): where generated code goes, imports, patches
  and warnings
- [Context and IR](../docs/custom-macros/context-and-ir): everything a macro can read about its
  target
- [Type-Aware Macros](../docs/custom-macros/type-aware): looking up other types and reading the
  project's config
- [Errors and Diagnostics](../docs/custom-macros/diagnostics): failing with errors, reporting
  warnings
- [Testing and Debugging](../docs/custom-macros/testing-and-debugging): unit tests, debug logging,
  registering macros by hand
