<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
</script>

<svelte:head>
	<title>CLI - Macroforge Documentation</title>
	<meta name="description" content="Command-line interface for macro expansion and TypeScript type checking." />
</svelte:head>

<h1>Command Line Interface</h1>

<p class="lead">
	The <code>macroforge</code> CLI provides commands for expanding macros and running type checks with macro support.
</p>

<h2 id="installation">Installation</h2>

<p>The CLI is a Rust binary. You can install it using Cargo:</p>

<CodeBlock code={`cargo install macroforge_ts`} lang="bash" />

<p>Or build from source:</p>

<CodeBlock code={`git clone https://github.com/rymskip/macroforge-ts.git
cd macroforge-ts/crates
cargo build --release --bin macroforge

# The binary is at target/release/macroforge`} lang="bash" />

<h2 id="commands">Commands</h2>

<h3 id="expand">macroforge expand</h3>

<p>Expands macros in a TypeScript file and outputs the transformed code.</p>

<CodeBlock code={`macroforge expand <input> [options]`} lang="bash" />

<h4>Arguments</h4>

<table>
	<thead>
		<tr>
			<th>Argument</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><code>&lt;input&gt;</code></td>
			<td>Path to the TypeScript or TSX file to expand</td>
		</tr>
	</tbody>
</table>

<h4>Options</h4>

<table>
	<thead>
		<tr>
			<th>Option</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><code>--out &lt;path&gt;</code></td>
			<td>Write the expanded JavaScript/TypeScript to a file</td>
		</tr>
		<tr>
			<td><code>--types-out &lt;path&gt;</code></td>
			<td>Write the generated <code>.d.ts</code> declarations to a file</td>
		</tr>
		<tr>
			<td><code>--print</code></td>
			<td>Print output to stdout even when <code>--out</code> is specified</td>
		</tr>
		<tr>
			<td><code>--builtin-only</code></td>
			<td>Use only built-in Rust macros (faster, but no external macro support)</td>
		</tr>
	</tbody>
</table>

<h4>Examples</h4>

<p>Expand a file and print to stdout:</p>

<CodeBlock code={`macroforge expand src/user.ts`} lang="bash" />

<p>Expand and write to a file:</p>

<CodeBlock code={`macroforge expand src/user.ts --out dist/user.js`} lang="bash" />

<p>Expand with both runtime output and type declarations:</p>

<CodeBlock code={`macroforge expand src/user.ts --out dist/user.js --types-out dist/user.d.ts`} lang="bash" />

<p>Use fast built-in macros only (no external macro support):</p>

<CodeBlock code={`macroforge expand src/user.ts --builtin-only`} lang="bash" />

<Alert type="note">
	<span>By default, the CLI uses Node.js for full macro support (including external macros). It must be run from your project's root directory where <code>macroforge</code> and any external macro packages are installed in <code>node_modules</code>.</span>
</Alert>

<h3 id="tsc">macroforge tsc</h3>

<p>
	Runs TypeScript type checking with macro expansion. This wraps <code>tsc --noEmit</code> and
	expands macros before type checking, so your generated methods are properly type-checked.
</p>

<CodeBlock code={`macroforge tsc [options]`} lang="bash" />

<h4>Options</h4>

<table>
	<thead>
		<tr>
			<th>Option</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><code>-p, --project &lt;path&gt;</code></td>
			<td>Path to <code>tsconfig.json</code> (defaults to <code>tsconfig.json</code> in current directory)</td>
		</tr>
	</tbody>
</table>

<h4>Examples</h4>

<p>Type check with default tsconfig.json:</p>

<CodeBlock code={`macroforge tsc`} lang="bash" />

<p>Type check with a specific config:</p>

<CodeBlock code={`macroforge tsc -p tsconfig.build.json`} lang="bash" />

<h2 id="output-format">Output Format</h2>

<h3>Expanded Code</h3>

<p>When expanding a file like this:</p>

<CodeBlock code={`/** @derive(Debug) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}`} lang="typescript" />

<p>The CLI outputs the expanded code with the generated methods:</p>

<CodeBlock code={`class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }

  [Symbol.for("nodejs.util.inspect.custom")](): string {
    return \`User { name: \${this.name}, age: \${this.age} }\`;
  }
}`} lang="typescript" />

<h3>Diagnostics</h3>

<p>Errors and warnings are printed to stderr in a readable format:</p>

<CodeBlock code={`[macroforge] error at src/user.ts:5:1: Unknown derive macro: InvalidMacro
[macroforge] warning at src/user.ts:10:3: Field 'unused' is never used`} lang="text" />

<h2 id="use-cases">Use Cases</h2>

<h3>CI/CD Type Checking</h3>

<p>Use <code>macroforge tsc</code> in your CI pipeline to type-check with macro expansion:</p>

<CodeBlock code={`# package.json
{
  "scripts": {
    "typecheck": "macroforge tsc"
  }
}`} lang="json" />

<h3>Debugging Macro Output</h3>

<p>Use <code>macroforge expand</code> to inspect what code your macros generate:</p>

<CodeBlock code={`macroforge expand src/models/user.ts | less`} lang="bash" />

<h3>Build Pipeline</h3>

<p>Generate expanded files as part of a custom build:</p>

<CodeBlock code={`#!/bin/bash
for file in src/**/*.ts; do
  outfile="dist/$(basename "$file" .ts).js"
  macroforge expand "$file" --out "$outfile"
done`} lang="bash" />

<h2 id="builtin-vs-full">Built-in vs Full Mode</h2>

<p>By default, the CLI uses Node.js for full macro support including external macros. Use <code>--builtin-only</code> for faster expansion when you only need built-in macros:</p>

<table>
	<thead>
		<tr>
			<th>Feature</th>
			<th>Default (Node.js)</th>
			<th><code>--builtin-only</code> (Rust)</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td>Built-in macros</td>
			<td>Yes</td>
			<td>Yes</td>
		</tr>
		<tr>
			<td>External macros</td>
			<td>Yes</td>
			<td>No</td>
		</tr>
		<tr>
			<td>Performance</td>
			<td>Standard</td>
			<td>Faster</td>
		</tr>
		<tr>
			<td>Dependencies</td>
			<td>Requires <code>macroforge</code> in node_modules</td>
			<td>None</td>
		</tr>
	</tbody>
</table>
