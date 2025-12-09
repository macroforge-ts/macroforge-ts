# Rust Setup

*Create a new Rust crate that will contain your custom macros. This crate compiles to a native Node.js addon.*

## Prerequisites

- Rust toolchain (1.70 or later)

- Node.js 18 or later

- NAPI-RS CLI: `npm install -g @napi-rs/cli`

## Create the Project

```bash
# Create a new directory
mkdir my-macros
cd my-macros

# Initialize with NAPI-RS
napi new --platform --name my-macros
```

## Configure Cargo.toml

Update your `Cargo.toml` with the required dependencies:

`Cargo.toml`
```toml
[package]
name = "my-macros"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
macroforge_ts = "0.1"
napi = { version = "3", features = ["napi8", "compat-mode"] }
napi-derive = "3"

[build-dependencies]
napi-build = "2"

[profile.release]
lto = true
strip = true
```

## Create build.rs

`build.rs`
```rust
fn main() {
    napi_build::setup();
}
```

## Create src/lib.rs

`src/lib.rs`
```rust
use macroforge_ts::ts_macro_derive::ts_macro_derive;
use macroforge_ts::ts_quote::body;
use macroforge_ts::ts_syn::{
    Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input,
};

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

## Create package.json

`package.json`
```json
{
  "name": "@my-org/macros",
  "version": "0.1.0",
  "main": "index.js",
  "types": "index.d.ts",
  "napi": {
    "name": "my-macros",
    "triples": {
      "defaults": true
    }
  },
  "files": [
    "index.js",
    "index.d.ts",
    "*.node"
  ],
  "scripts": {
    "build": "napi build --release",
    "prepublishOnly": "napi build --release"
  },
  "devDependencies": {
    "@napi-rs/cli": "^3.0.0-alpha.0"
  }
}
```

## Build the Package

```bash
# Build the native addon
npm run build

# This creates:
# - index.js (JavaScript bindings)
# - index.d.ts (TypeScript types)
# - *.node (native binary)
```

>
> For cross-platform builds, use GitHub Actions with the NAPI-RS CI template.

## Next Steps

- [Learn the ts_macro_derive attribute](/docs/custom-macros/ts-macro-derive)

- [Master the ts_quote template syntax](/docs/custom-macros/ts-quote)