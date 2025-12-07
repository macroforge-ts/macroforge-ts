<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
</script>

<svelte:head>
	<title>How Macros Work - Macroforge Documentation</title>
	<meta name="description" content="Understand how Macroforge compiles TypeScript macros at build time." />
</svelte:head>

<h1>How Macros Work</h1>

<p class="lead">
	Macroforge performs compile-time code generation by parsing your TypeScript, expanding macros, and outputting transformed code. This happens before your code runs, resulting in zero runtime overhead.
</p>

<h2 id="compile-time-expansion">Compile-Time Expansion</h2>

<p>
	Unlike runtime solutions that use reflection or proxies, Macroforge expands macros at compile time:
</p>

<ol>
	<li><strong>Parse</strong>: Your TypeScript code is parsed into an AST using SWC</li>
	<li><strong>Find</strong>: Macroforge finds <code>@derive</code> decorators and their associated items</li>
	<li><strong>Expand</strong>: Each macro generates new code based on the class structure</li>
	<li><strong>Output</strong>: The transformed TypeScript is written out, ready for normal compilation</li>
</ol>

<CodeBlock code={`// Your source code
/** @derive(Debug) */
class User {
  name: string;
}

// After macro expansion
class User {
  name: string;

  toString(): string {
    return \`User { name: \${this.name} }\`;
  }
}`} lang="typescript" />

<h2 id="zero-runtime">Zero Runtime Overhead</h2>

<p>
	Because code generation happens at compile time, there's no:
</p>

<ul>
	<li>Runtime reflection or metadata</li>
	<li>Proxy objects or wrappers</li>
	<li>Additional dependencies in your bundle</li>
	<li>Performance cost at runtime</li>
</ul>

<p>
	The generated code is plain TypeScript that compiles to efficient JavaScript.
</p>

<h2 id="source-mapping">Source Mapping</h2>

<p>
	Macroforge tracks the relationship between your source code and the expanded output. This means:
</p>

<ul>
	<li>Errors in generated code point back to your source</li>
	<li>Debugging works correctly</li>
	<li>IDE features like "go to definition" work as expected</li>
</ul>

<Alert type="info">
	The TypeScript plugin uses source mapping to show errors at the <code>@derive</code> decorator position, not in the generated code.
</Alert>

<h2 id="execution-flow">Execution Flow</h2>

<CodeBlock code={`┌─────────────────────────────────────────────────────────┐
│                    Your Source Code                      │
│              (with @derive decorators)                   │
└───────────────────────┬─────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────┐
│                   SWC Parser                             │
│            (TypeScript → AST)                            │
└───────────────────────┬─────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────┐
│                Macro Expansion Engine                    │
│    - Finds @derive decorators                            │
│    - Runs registered macros                              │
│    - Generates new AST nodes                             │
└───────────────────────┬─────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────┐
│                 Code Generator                           │
│            (AST → TypeScript)                            │
└───────────────────────┬─────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────┐
│                 Expanded TypeScript                      │
│         (ready for normal compilation)                   │
└─────────────────────────────────────────────────────────┘`} lang="text" />

<h2 id="integration-points">Integration Points</h2>

<p>
	Macroforge integrates at two key points:
</p>

<h3>IDE (TypeScript Plugin)</h3>
<p>
	The TypeScript plugin intercepts language server calls to provide:
</p>
<ul>
	<li>Diagnostics that reference your source, not expanded code</li>
	<li>Completions for generated methods</li>
	<li>Hover information showing what macros generate</li>
</ul>

<h3>Build (Vite Plugin)</h3>
<p>
	The Vite plugin runs macro expansion during the build process:
</p>
<ul>
	<li>Transforms files before they reach the TypeScript compiler</li>
	<li>Generates type declaration files (.d.ts)</li>
	<li>Produces metadata for debugging</li>
</ul>

<h2 id="next-steps">Next Steps</h2>

<ul>
	<li><a href="/docs/concepts/derive-system">Learn about the derive system</a></li>
	<li><a href="/docs/concepts/architecture">Explore the architecture</a></li>
</ul>
