# Rust Setup

Create a new Rust crate that will contain your custom macros. It compiles to WebAssembly, packaged
as an npm package that Macroforge loads.

## Prerequisites

- A Rust toolchain with the WebAssembly target: `rustup target add wasm32-unknown-unknown`
- The Macroforge CLI: `cargo install macroforge_ts`
- `wasm-bindgen-cli` at the version of the `wasm-bindgen` crate your macro crate builds with. After
  adding `macroforge_ts` below, `cargo tree -i wasm-bindgen --depth 0` prints it; install that
  version with `cargo install wasm-bindgen-cli --version <version> --locked`.

## Create the Project

Bash

```
cargo new --lib my-macros
cd my-macros
```

## Configure Cargo.toml

Cargo.toml

```
[package]
name = "my-macros"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
macroforge_ts = "0.5"

[profile.release]
lto = true
strip = true
```

## Create src/lib.rs

src/lib.rs

```
use macroforge_ts::macros::{ts_macro_derive, ts_template};
use macroforge_ts::ts_syn::{
    Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input,
};

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
            input.decorator_span(),
            "@derive(JSON) only works on classes",
        )),
    }
}
```

## Create package.json

`macroforge build` writes the package into `pkg/`, named after the library target: `my_macros` for
this crate.

package.json

```
{
  "name": "@my-org/macros",
  "version": "0.1.0",
  "main": "pkg/my_macros.js",
  "types": "pkg/my_macros.d.ts",
  "files": ["pkg"],
  "scripts": {
    "build": "macroforge build . --out pkg"
  }
}
```

## Build the Package

Bash

```
npm run build

# This creates, in pkg/:
# - my_macros.js       (JavaScript bindings)
# - my_macros.d.ts     (TypeScript types)
# - my_macros_bg.wasm  (the macros)
```

Tip

The WebAssembly module runs on every platform, so one build serves every OS.

## Next Steps

- [Learn the #\[ts\_macro\_derive\] attribute](../../docs/custom-macros/ts-macro-derive)
- [Master the template syntax](../../docs/custom-macros/ts-quote)
