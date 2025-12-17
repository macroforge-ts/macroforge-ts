# Rust Setup
 *Create a new Rust crate that will contain your custom macros. This crate compiles to a native Node.js addon.*
 ## Prerequisites
 - Rust toolchain (1.88 or later)
 - Node.js 24 or later
 - NAPI-RS CLI: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">cargo<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">install<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">macroforge_ts</code>
 ## Create the Project
 ```
# Create a new directory
mkdir my-macros
cd my-macros

# Initialize with NAPI-RS
napi new --platform --name my-macros
``` ## Configure Cargo.toml
 Update your <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#FDAEB7;--shiki-dark-font-style:italic;--shiki-light:#B31D28;--shiki-light-font-style:italic">Cargo.toml</code> with the required dependencies:
 ```
[package]
name = "my-macros"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
macroforge_ts = "0.1"
napi = &#123; version = "3", features = ["napi8", "compat-mode"] &#125;
napi-derive = "3"

[build-dependencies]
napi-build = "2"

[profile.release]
lto = true
strip = true
``` ## Create build.rs
 ```
fn main() &#123;
    napi_build::setup();
&#125;
``` ## Create src/lib.rs
 ```
use macroforge_ts::macros::&#123;ts_macro_derive, body&#125;;
use macroforge_ts::ts_syn::&#123;
    Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input,
&#125;;

#[ts_macro_derive(
    JSON,
    description = "Generates toJSON() returning a plain object"
)]
pub fn derive_json(mut input: TsStream) -> Result&#x3C;TsStream, MacroforgeError> &#123;
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &#x26;input.data &#123;
        Data::Class(class) => &#123;
            Ok(body! &#123;
                toJSON(): Record&#x3C;string, unknown> &#123;
                    return &#123;
                        &#123;#for field in class.field_names()&#125;
                            @&#123;field&#125;: this.@&#123;field&#125;,
                        &#123;/for&#125;
                    &#125;;
                &#125;
            &#125;)
        &#125;
        _ => Err(MacroforgeError::new(
            input.decorator_span(),
            "@derive(JSON) only works on classes",
        )),
    &#125;
&#125;
``` ## Create package.json
 ```
&#123;
  "name": "@my-org/macros",
  "version": "0.1.0",
  "main": "index.js",
  "types": "index.d.ts",
  "napi": &#123;
    "name": "my-macros",
    "triples": &#123;
      "defaults": true
    &#125;
  &#125;,
  "files": [
    "index.js",
    "index.d.ts",
    "*.node"
  ],
  "scripts": &#123;
    "build": "napi build --release",
    "prepublishOnly": "napi build --release"
  &#125;,
  "devDependencies": &#123;
    "@napi-rs/cli": "^3.0.0-alpha.0"
  &#125;
&#125;
``` ## Build the Package
 ```
# Build the native addon
npm run build

# This creates:
# - index.js (JavaScript bindings)
# - index.d.ts (TypeScript types)
# - *.node (native binary)
```  **Tip For cross-platform builds, use GitHub Actions with the NAPI-RS CI template. ## Next Steps
 - [Learn the #[ts_macro_derive] attribute](../../docs/custom-macros/ts-macro-derive)
 - [Master the template syntax](../../docs/custom-macros/ts-quote)
**