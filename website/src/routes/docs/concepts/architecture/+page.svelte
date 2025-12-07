<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
</script>

<svelte:head>
	<title>Architecture - Macroforge Documentation</title>
	<meta name="description" content="Understand the architecture of Macroforge - the TypeScript macro engine." />
</svelte:head>

<h1>Architecture</h1>

<p class="lead">
	Macroforge is built as a native Node.js module using Rust and NAPI-RS. It leverages SWC for fast TypeScript parsing and code generation.
</p>

<h2 id="overview">Overview</h2>

<CodeBlock code={`┌─────────────────────────────────────────────────────────┐
│                    Node.js / Vite                        │
├─────────────────────────────────────────────────────────┤
│                   NAPI-RS Bindings                       │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌──────────────┐  ┌───────────────┐   │
│  │   ts_syn    │  │   ts_quote   │  │ts_macro_derive│   │
│  │  (parsing)  │  │ (templating) │  │  (proc-macro) │   │
│  └─────────────┘  └──────────────┘  └───────────────┘   │
├─────────────────────────────────────────────────────────┤
│                    SWC Core                              │
│            (TypeScript parsing & codegen)                │
└─────────────────────────────────────────────────────────┘`} lang="text" />

<h2 id="components">Core Components</h2>

<h3>SWC Core</h3>
<p>
	The foundation layer provides:
</p>
<ul>
	<li>Fast TypeScript/JavaScript parsing</li>
	<li>AST representation</li>
	<li>Code generation (AST → source code)</li>
</ul>

<h3>ts_syn</h3>
<p>
	A Rust crate that provides:
</p>
<ul>
	<li>TypeScript-specific AST types</li>
	<li>Parsing utilities for macro input</li>
	<li>Derive input structures (class fields, decorators, etc.)</li>
</ul>

<h3>ts_quote</h3>
<p>
	Template-based code generation similar to Rust's <code>quote!</code>:
</p>
<ul>
	<li><code>ts_template!</code> - Generate TypeScript code from templates</li>
	<li><code>body!</code> - Generate class body members</li>
	<li>Control flow: <code>{"{#for}"}</code>, <code>{"{#if}"}</code>, <code>{"{%let}"}</code></li>
</ul>

<h3>ts_macro_derive</h3>
<p>
	The procedural macro attribute for defining derive macros:
</p>
<ul>
	<li><code>#[ts_macro_derive(Name)]</code> attribute</li>
	<li>Automatic registration with the macro system</li>
	<li>Error handling and span tracking</li>
</ul>

<h3>NAPI-RS Bindings</h3>
<p>
	Bridges Rust and Node.js:
</p>
<ul>
	<li>Exposes <code>expandSync</code>, <code>transformSync</code>, etc.</li>
	<li>Provides the <code>NativePlugin</code> class for caching</li>
	<li>Handles data marshaling between Rust and JavaScript</li>
</ul>

<h2 id="data-flow">Data Flow</h2>

<CodeBlock code={`1. Source Code (TypeScript with @derive)
   │
   ▼
2. NAPI-RS receives JavaScript string
   │
   ▼
3. SWC parses to AST
   │
   ▼
4. Macro expander finds @derive decorators
   │
   ▼
5. For each macro:
   │  a. Extract class/interface data
   │  b. Run macro function
   │  c. Generate new AST nodes
   │
   ▼
6. Merge generated nodes into AST
   │
   ▼
7. SWC generates source code
   │
   ▼
8. Return to JavaScript with source mapping`} lang="text" />

<h2 id="performance">Performance Characteristics</h2>

<ul>
	<li><strong>Thread-safe</strong>: Each expansion runs in an isolated thread with a 32MB stack</li>
	<li><strong>Caching</strong>: <code>NativePlugin</code> caches results by file version</li>
	<li><strong>Binary search</strong>: Position mapping uses O(log n) lookups</li>
	<li><strong>Zero-copy</strong>: SWC's arena allocator minimizes allocations</li>
</ul>

<h2 id="re-exports">Re-exported Crates</h2>

<p>
	For custom macro development, <code>macroforge_ts</code> re-exports everything you need:
</p>

<CodeBlock code={`// All available via macroforge_ts::*
pub extern crate ts_syn;         // AST types, parsing
pub extern crate ts_quote;       // Code generation templates
pub extern crate ts_macro_derive; // #[ts_macro_derive] attribute
pub extern crate inventory;       // Macro registration
pub extern crate serde_json;      // Serialization
pub extern crate napi;            // Node.js bindings
pub extern crate napi_derive;     // NAPI proc-macros

// SWC modules
pub use ts_syn::swc_core;
pub use ts_syn::swc_common;
pub use ts_syn::swc_ecma_ast;`} lang="rust" />

<h2 id="next-steps">Next Steps</h2>

<ul>
	<li><a href="/docs/custom-macros">Write custom macros</a></li>
	<li><a href="/docs/api">Explore the API reference</a></li>
</ul>
