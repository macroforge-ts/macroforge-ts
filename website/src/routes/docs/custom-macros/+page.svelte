<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
</script>

<svelte:head>
	<title>Custom Macros - Macroforge Documentation</title>
	<meta name="description" content="Learn how to create custom derive macros for Macroforge using Rust." />
</svelte:head>

<h1>Custom Macros</h1>

<p class="lead">
	Macroforge allows you to create custom derive macros in Rust. Your macros have full access to the class AST and can generate any TypeScript code.
</p>

<h2 id="overview">Overview</h2>

<p>
	Custom macros are written in Rust and compiled to native Node.js addons. The process involves:
</p>

<ol>
	<li>Creating a Rust crate with NAPI bindings</li>
	<li>Defining macro functions with <code>#[ts_macro_derive]</code></li>
	<li>Using <code>ts_quote</code> to generate TypeScript code</li>
	<li>Building and publishing as an npm package</li>
</ol>

<h2 id="quick-example">Quick Example</h2>

<CodeBlock code={`use macroforge_ts::ts_macro_derive::ts_macro_derive;
use macroforge_ts::ts_quote::body;
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
}`} lang="rust" />

<h2 id="using-custom-macros">Using Custom Macros</h2>

<p>
	Once your macro package is published, users can import and use it:
</p>

<CodeBlock code={`/** import macro { JSON } from "@my/macros"; */

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
console.log(user.toJSON()); // { name: "Alice", age: 30 }`} lang="typescript" />

<Alert type="note">
	The <code>import macro</code> comment tells Macroforge which package provides the macro.
</Alert>

<h2 id="next-steps">Getting Started</h2>

<p>
	Follow these guides to create your own macros:
</p>

<ul>
	<li><a href="/docs/custom-macros/rust-setup">Set up a Rust macro crate</a></li>
	<li><a href="/docs/custom-macros/ts-macro-derive">Use the ts_macro_derive attribute</a></li>
	<li><a href="/docs/custom-macros/ts-quote">Learn the ts_quote template syntax</a></li>
</ul>
