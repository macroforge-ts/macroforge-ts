<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
</script>

<svelte:head>
	<title>Built-in Macros - Macroforge Documentation</title>
	<meta name="description" content="Overview of Macroforge's built-in derive macros: Debug, Clone, Eq, Serialize, and Deserialize." />
</svelte:head>

<h1>Built-in Macros</h1>

<p class="lead">
	Macroforge comes with five built-in derive macros that cover the most common code generation needs.
	All macros work with both classes and interfaces.
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
		<tr>
			<td><code>Serialize</code></td>
			<td><code>toJSON(): Record&lt;string, unknown&gt;</code></td>
			<td>JSON serialization with type handling</td>
		</tr>
		<tr>
			<td><code>Deserialize</code></td>
			<td><code>fromJSON(data: unknown): T</code></td>
			<td>JSON deserialization with validation</td>
		</tr>
	</tbody>
</table>

<h2 id="using-built-in-macros">Using Built-in Macros</h2>

<p>
	Built-in macros don't require imports. Just use them with <code>@derive</code>:
</p>

<CodeBlock code={`/** @derive(Debug, Clone, Eq) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}`} lang="typescript" />

<h2 id="interface-support">Interface Support</h2>

<p>
	All built-in macros work with interfaces. For interfaces, methods are generated as functions
	in a namespace with the same name, using <code>self</code> as the first parameter:
</p>

<CodeBlock code={`/** @derive(Debug, Clone, Eq) */
interface Point {
  x: number;
  y: number;
}

// Generated namespace:
// namespace Point {
//   export function toString(self: Point): string { ... }
//   export function clone(self: Point): Point { ... }
//   export function equals(self: Point, other: Point): boolean { ... }
//   export function hashCode(self: Point): number { ... }
// }

const point: Point = { x: 10, y: 20 };

// Use the namespace functions
console.log(Point.toString(point));     // "Point { x: 10, y: 20 }"
const copy = Point.clone(point);        // { x: 10, y: 20 }
console.log(Point.equals(point, copy)); // true`} lang="typescript" />

<h2 id="combining-macros">Combining Macros</h2>

<p>
	All macros can be used together. They don't conflict and each generates independent methods:
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
	<li><a href="/docs/builtin-macros/clone"><strong>Clone</strong></a> - Shallow copying for all field types</li>
	<li><a href="/docs/builtin-macros/eq"><strong>Eq</strong></a> - Value-based equality comparison</li>
	<li><a href="/docs/builtin-macros/serialize"><strong>Serialize</strong></a> - JSON serialization with serde-style options</li>
	<li><a href="/docs/builtin-macros/deserialize"><strong>Deserialize</strong></a> - JSON deserialization with validation</li>
</ul>
