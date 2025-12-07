<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
</script>

<svelte:head>
	<title>Built-in Macros - Macroforge Documentation</title>
	<meta name="description" content="Overview of Macroforge's built-in derive macros: Debug, Clone, and Eq." />
</svelte:head>

<h1>Built-in Macros</h1>

<p class="lead">
	Macroforge comes with three built-in derive macros that cover the most common code generation needs.
</p>

<h2 id="overview">Overview</h2>

<table>
	<thead>
		<tr>
			<th>Macro</th>
			<th>Generates</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><code>Debug</code></td>
			<td><code>toString(): string</code></td>
			<td>Human-readable string representation</td>
		</tr>
		<tr>
			<td><code>Clone</code></td>
			<td><code>clone(): T</code></td>
			<td>Creates a copy of the object</td>
		</tr>
		<tr>
			<td><code>Eq</code></td>
			<td><code>equals(other: T): boolean</code></td>
			<td>Value equality comparison</td>
		</tr>
	</tbody>
</table>

<h2 id="using-built-in-macros">Using Built-in Macros</h2>

<p>
	Import the macros you need and use them with <code>@derive</code>:
</p>

<CodeBlock code={`import { Debug, Clone, Eq } from "macroforge";

/** @derive(Debug, Clone, Eq) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}`} lang="typescript" />

<h2 id="combining-macros">Combining Macros</h2>

<p>
	All three macros can be used together. They don't conflict and each generates independent methods:
</p>

<CodeBlock code={`const user = new User("Alice", 30);

// Debug
console.log(user.toString());
// "User { name: Alice, age: 30 }"

// Clone
const copy = user.clone();
console.log(copy.name); // "Alice"

// Eq
console.log(user.equals(copy)); // true`} lang="typescript" />

<h2 id="detailed-documentation">Detailed Documentation</h2>

<p>
	Each macro has its own options and behaviors:
</p>

<ul>
	<li><a href="/docs/builtin-macros/debug"><strong>Debug</strong></a> - Customizable field renaming and skipping</li>
	<li><a href="/docs/builtin-macros/clone"><strong>Clone</strong></a> - Deep copying for all field types</li>
	<li><a href="/docs/builtin-macros/eq"><strong>Eq</strong></a> - Value-based equality comparison</li>
</ul>
