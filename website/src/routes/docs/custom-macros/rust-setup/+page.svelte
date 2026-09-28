<script lang="ts">
    import CodeBlock from "$lib/components/ui/CodeBlock.svelte";
    import Alert from "$lib/components/ui/Alert.svelte";
    import { resolve } from "$app/paths";

    let { data } = $props();
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
    Create a new Rust crate that will contain your custom macros. It compiles to
    WebAssembly, packaged as an npm package that Macroforge loads.
</p>

<h2 id="prerequisites">Prerequisites</h2>

<ul>
    <li>
        A Rust toolchain with the WebAssembly target:
        <code>rustup target add wasm32-unknown-unknown</code>
    </li>
    <li>
        The Macroforge CLI: <code>cargo install macroforge_ts</code>
    </li>
    <li>
        <code>wasm-bindgen-cli</code> at the version of the <code>wasm-bindgen</code>
        crate your macro crate builds with. After adding <code>macroforge_ts</code>
        below, <code>cargo tree -i wasm-bindgen --depth 0</code> prints it; install
        that version with
        <code>cargo install wasm-bindgen-cli --version &lt;version&gt; --locked</code>.
    </li>
</ul>

<h2 id="create-project">Create the Project</h2>

<CodeBlock
    code={`cargo new --lib my-macros
cd my-macros`}
    lang="bash"
/>

<h2 id="cargo-toml">Configure Cargo.toml</h2>

<CodeBlock
    code={`[package]
name = "my-macros"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
macroforge_ts = "${data.requirement}"

[profile.release]
lto = true
strip = true`}
    lang="toml"
    filename="Cargo.toml"
/>

<h2 id="lib-rs">Create src/lib.rs</h2>

<CodeBlock
    code={`use macroforge_ts::macros::{ts_macro_derive, ts_template};
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
            Ok(ts_template!(Within {
                toJSON(): Record<string, unknown> {
                    return {
                        {#for field in class.field_names()}
                            @{field}: this.@{field},
                        {/for}
                    };
                }
            }))
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

<p>
    <code>macroforge build</code> writes the package into <code>pkg/</code>, named
    after the library target: <code>my_macros</code> for this crate.
</p>

<CodeBlock
    code={`{
  "name": "@my-org/macros",
  "version": "0.1.0",
  "main": "pkg/my_macros.js",
  "types": "pkg/my_macros.d.ts",
  "files": ["pkg"],
  "scripts": {
    "build": "macroforge build . --out pkg"
  }
}`}
    lang="json"
    filename="package.json"
/>

<h2 id="build">Build the Package</h2>

<CodeBlock
    code={`npm run build

# This creates, in pkg/:
# - my_macros.js       (JavaScript bindings)
# - my_macros.d.ts     (TypeScript types)
# - my_macros_bg.wasm  (the macros)`}
    lang="bash"
/>

<Alert type="tip">
    The WebAssembly module runs on every platform, so one build serves every OS.
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
