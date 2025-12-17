<script lang="ts">
    import CodeBlock from "$lib/components/ui/CodeBlock.svelte";
    import Alert from "$lib/components/ui/Alert.svelte";
    import { resolve } from "$app/paths";
</script>

<svelte:head>
    <title>Rust Setup - Macroforge Documentation</title>
    <meta
        name="description"
        content="Set up a Rust crate for creating custom Macroforge macros."
    />
</svelte:head>

<h1>Rust Setup</h1>

<p class="lead">
    Create a new Rust crate that will contain your custom macros. This crate
    compiles to a native Node.js addon.
</p>

<h2 id="prerequisites">Prerequisites</h2>

<ul>
    <li>Rust toolchain (1.88 or later)</li>
    <li>Node.js 24 or later</li>
    <li>
        NAPI-RS CLI: <code>cargo install macroforge_ts</code>
    </li>
</ul>

<h2 id="create-project">Create the Project</h2>

<CodeBlock
    code={`# Create a new directory
mkdir my-macros
cd my-macros

# Initialize with NAPI-RS
napi new --platform --name my-macros`}
    lang="bash"
/>

<h2 id="cargo-toml">Configure Cargo.toml</h2>

<p>
    Update your <code>Cargo.toml</code> with the required dependencies:
</p>

<CodeBlock
    code={`[package]
name = "my-macros"
version = "0.1.0"
edition = "2024"

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
strip = true`}
    lang="toml"
    filename="Cargo.toml"
/>

<h2 id="build-rs">Create build.rs</h2>

<CodeBlock
    code={`fn main() {
    napi_build::setup();
}`}
    lang="rust"
    filename="build.rs"
/>

<h2 id="lib-rs">Create src/lib.rs</h2>

<CodeBlock
    code={`use macroforge_ts::macros::{ts_macro_derive, body};
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
}`}
    lang="rust"
    filename="src/lib.rs"
/>

<h2 id="package-json">Create package.json</h2>

<CodeBlock
    code={`{
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
}`}
    lang="json"
    filename="package.json"
/>

<h2 id="build">Build the Package</h2>

<CodeBlock
    code={`# Build the native addon
npm run build

# This creates:
# - index.js (JavaScript bindings)
# - index.d.ts (TypeScript types)
# - *.node (native binary)`}
    lang="bash"
/>

<Alert type="tip">
    For cross-platform builds, use GitHub Actions with the NAPI-RS CI template.
</Alert>

<h2 id="next-steps">Next Steps</h2>

<ul>
    <li>
        <a href={resolve('/docs/custom-macros/ts-macro-derive')}
            >Learn the #[ts_macro_derive] attribute</a
        >
    </li>
    <li>
        <a href={resolve('/docs/custom-macros/ts-quote')}
            >Master the template syntax</a
        >
    </li>
</ul>
