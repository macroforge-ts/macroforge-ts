# Rust Setup
 *Create a new Rust crate that will contain your custom macros. This crate compiles to a native Node.js addon.*
 ## Prerequisites
 - Rust toolchain (1.88 or later)
 - Node.js 24 or later
 - NAPI-RS CLI: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">cargo<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62"> install<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62"> macroforge_ts</code>
 ## Create the Project
 ```
#&nbsp;Create&nbsp;a&nbsp;new&nbsp;directory
mkdir&nbsp;my-macros
cd&nbsp;my-macros

#&nbsp;Initialize&nbsp;with&nbsp;NAPI-RS
napi&nbsp;new&nbsp;--platform&nbsp;--name&nbsp;my-macros
``` ## Configure Cargo.toml
 Update your <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#FDAEB7;--shiki-dark-font-style:italic;--shiki-light:#B31D28;--shiki-light-font-style:italic">Cargo.toml</code> with the required dependencies:
 ```
[package]
name&nbsp;=&nbsp;"my-macros"
version&nbsp;=&nbsp;"0.1.0"
edition&nbsp;=&nbsp;"2024"

[lib]
crate-type&nbsp;=&nbsp;["cdylib"]

[dependencies]
macroforge_ts&nbsp;=&nbsp;"0.1"
napi&nbsp;=&nbsp;&#123;&nbsp;version&nbsp;=&nbsp;"3",&nbsp;features&nbsp;=&nbsp;["napi8",&nbsp;"compat-mode"]&nbsp;&#125;
napi-derive&nbsp;=&nbsp;"3"

[build-dependencies]
napi-build&nbsp;=&nbsp;"2"

[profile.release]
lto&nbsp;=&nbsp;true
strip&nbsp;=&nbsp;true
``` ## Create build.rs
 ```
fn&nbsp;main()&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;napi_build::setup();
&#125;
``` ## Create src/lib.rs
 ```
use&nbsp;macroforge_ts::macros::&#123;ts_macro_derive,&nbsp;body&#125;;
use&nbsp;macroforge_ts::ts_syn::&#123;
&nbsp;&nbsp;&nbsp;&nbsp;Data,&nbsp;DeriveInput,&nbsp;MacroforgeError,&nbsp;TsStream,&nbsp;parse_ts_macro_input,
&#125;;

#[ts_macro_derive(
&nbsp;&nbsp;&nbsp;&nbsp;JSON,
&nbsp;&nbsp;&nbsp;&nbsp;description&nbsp;=&nbsp;"Generates&nbsp;toJSON()&nbsp;returning&nbsp;a&nbsp;plain&nbsp;object"
)]
pub&nbsp;fn&nbsp;derive_json(mut&nbsp;input:&nbsp;TsStream)&nbsp;->&nbsp;Result&#x3C;TsStream,&nbsp;MacroforgeError>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;input&nbsp;=&nbsp;parse_ts_macro_input!(input&nbsp;as&nbsp;DeriveInput);

&nbsp;&nbsp;&nbsp;&nbsp;match&nbsp;&#x26;input.data&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Data::Class(class)&nbsp;=>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Ok(body!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;toJSON():&nbsp;Record&#x3C;string,&nbsp;unknown>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;field&nbsp;in&nbsp;class.field_names()&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;@&#123;field&#125;:&nbsp;this.@&#123;field&#125;,
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;)
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;_&nbsp;=>&nbsp;Err(MacroforgeError::new(
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;input.decorator_span(),
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"@derive(JSON)&nbsp;only&nbsp;works&nbsp;on&nbsp;classes",
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;)),
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;
``` ## Create package.json
 ```
&#123;
&nbsp;&nbsp;"name":&nbsp;"@my-org/macros",
&nbsp;&nbsp;"version":&nbsp;"0.1.0",
&nbsp;&nbsp;"main":&nbsp;"index.js",
&nbsp;&nbsp;"types":&nbsp;"index.d.ts",
&nbsp;&nbsp;"napi":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;"name":&nbsp;"my-macros",
&nbsp;&nbsp;&nbsp;&nbsp;"triples":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"defaults":&nbsp;true
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&#125;,
&nbsp;&nbsp;"files":&nbsp;[
&nbsp;&nbsp;&nbsp;&nbsp;"index.js",
&nbsp;&nbsp;&nbsp;&nbsp;"index.d.ts",
&nbsp;&nbsp;&nbsp;&nbsp;"*.node"
&nbsp;&nbsp;],
&nbsp;&nbsp;"scripts":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;"build":&nbsp;"napi&nbsp;build&nbsp;--release",
&nbsp;&nbsp;&nbsp;&nbsp;"prepublishOnly":&nbsp;"napi&nbsp;build&nbsp;--release"
&nbsp;&nbsp;&#125;,
&nbsp;&nbsp;"devDependencies":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;"@napi-rs/cli":&nbsp;"^3.0.0-alpha.0"
&nbsp;&nbsp;&#125;
&#125;
``` ## Build the Package
 ```
#&nbsp;Build&nbsp;the&nbsp;native&nbsp;addon
npm&nbsp;run&nbsp;build

#&nbsp;This&nbsp;creates:
#&nbsp;-&nbsp;index.js&nbsp;(JavaScript&nbsp;bindings)
#&nbsp;-&nbsp;index.d.ts&nbsp;(TypeScript&nbsp;types)
#&nbsp;-&nbsp;*.node&nbsp;(native&nbsp;binary)
```  **Tip For cross-platform builds, use GitHub Actions with the NAPI-RS CI template. ## Next Steps
 - [Learn the #[ts_macro_derive] attribute](../../docs/custom-macros/ts-macro-derive)
 - [Master the template syntax](../../docs/custom-macros/ts-quote)
**