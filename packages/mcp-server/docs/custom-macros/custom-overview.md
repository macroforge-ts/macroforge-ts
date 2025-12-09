# Custom Macros

*Macroforge allows you to create custom derive macros in Rust. Your macros have full access to the class AST and can generate any TypeScript code.*

## Overview

Custom macros are written in Rust and compiled to native Node.js addons. The process involves:

1. Creating a Rust crate with NAPI bindings

2. Defining macro functions with `#[ts_macro_derive]`

3. Using `macroforge_ts_quote` to generate TypeScript code

4. Building and publishing as an npm package

## Quick Example

```rust
use macroforge_ts::macros::{ts_macro_derive, body};
use macroforge_ts::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

#[ts_macro_derive(
    JSON,
    description = "Generates toJSON() returning a plain object"
)]
pub fn derive_json(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            Ok(body! {
                toJSON(): Record<string, unknown> {
                    return {
                        {#for field in class.field_names()}
                            @{field}: this.@{field},
                        {/for}
                    };
                }
            })
        }
        _ => Err(MacroforgeError::new(
            input.decorator_span(),
            "@derive(JSON) only works on classes",
        )),
    }
}
```

## Using Custom Macros

Once your macro package is published, users can import and use it:

```typescript
/** import macro { JSON } from "@my/macros"; */

/** @derive(JSON) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}

const user = new User("Alice", 30);
console.log(user.toJSON()); // { name: "Alice", age: 30 }
```

>
> The `import macro` comment tells Macroforge which package provides the macro.

## Getting Started

Follow these guides to create your own macros:

- [Set up a Rust macro crate]({base}/docs/custom-macros/rust-setup)

- [Learn the #[ts_macro_derive] attribute]({base}/docs/custom-macros/ts-macro-derive)

- [Learn the template syntax]({base}/docs/custom-macros/ts-quote)