<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import { resolve } from '$app/paths';
</script>

<svelte:head>
	<title>Built-in Macros - Macroforge Documentation</title>
	<meta name="description" content="Overview of Macroforge's built-in derive macros: Debug, Clone, Eq, Serialize, and Deserialize." />
</svelte:head>

<h1>Built-in Macros</h1>

<p class="lead">
	Macroforge comes with built-in derive macros that cover the most common code generation needs.
	All macros work with classes, interfaces, enums, and type aliases.
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
			<td><a href={resolve('/docs/builtin-macros/debug')}><code>Debug</code></a></td>
			<td><code>toString(): string</code></td>
			<td>Human-readable string representation</td>
		</tr>
		<tr>
			<td><a href={resolve('/docs/builtin-macros/clone')}><code>Clone</code></a></td>
			<td><code>clone(): T</code></td>
			<td>Creates a deep copy of the object</td>
		</tr>
		<tr>
			<td><a href={resolve('/docs/builtin-macros/default')}><code>Default</code></a></td>
			<td><code>static default(): T</code></td>
			<td>Creates an instance with default values</td>
		</tr>
		<tr>
			<td><a href={resolve('/docs/builtin-macros/hash')}><code>Hash</code></a></td>
			<td><code>hashCode(): number</code></td>
			<td>Generates a hash code for the object</td>
		</tr>
		<tr>
			<td><a href={resolve('/docs/builtin-macros/partial-eq')}><code>PartialEq</code></a></td>
			<td><code>equals(other: T): boolean</code></td>
			<td>Value equality comparison</td>
		</tr>
		<tr>
			<td><a href={resolve('/docs/builtin-macros/ord')}><code>Ord</code></a></td>
			<td><code>compare(other: T): number</code></td>
			<td>Total ordering comparison (-1, 0, 1)</td>
		</tr>
		<tr>
			<td><a href={resolve('/docs/builtin-macros/partial-ord')}><code>PartialOrd</code></a></td>
			<td><code>partialCompare(other: T): number | null</code></td>
			<td>Partial ordering comparison</td>
		</tr>
		<tr>
			<td><a href={resolve('/docs/builtin-macros/serialize')}><code>Serialize</code></a></td>
			<td><code>toJSON(): Record&lt;string, unknown&gt;</code></td>
			<td>JSON serialization with type handling</td>
		</tr>
		<tr>
			<td><a href={resolve('/docs/builtin-macros/deserialize')}><code>Deserialize</code></a></td>
			<td><code>static fromJSON(data: unknown): T</code></td>
			<td>JSON deserialization with validation</td>
		</tr>
	</tbody>
</table>

<h2 id="using-built-in-macros">Using Built-in Macros</h2>

<p>
	Built-in macros don't require imports. Just use them with <code>@derive</code>:
</p>

<CodeBlock code={`/** @derive(Debug, Clone, PartialEq) */
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

<CodeBlock code={`/** @derive(Debug, Clone, PartialEq) */
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

<h2 id="enum-support">Enum Support</h2>

<p>
	All built-in macros work with enums. For enums, methods are generated as functions
	in a namespace with the same name:
</p>

<CodeBlock code={`/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}

// Generated namespace:
// namespace Status {
//   export function toString(value: Status): string { ... }
//   export function clone(value: Status): Status { ... }
//   export function equals(a: Status, b: Status): boolean { ... }
//   export function hashCode(value: Status): number { ... }
//   export function toJSON(value: Status): string | number { ... }
//   export function fromJSON(data: unknown): Status { ... }
// }

// Use the namespace functions
console.log(Status.toString(Status.Active));     // "Status.Active"
console.log(Status.equals(Status.Active, Status.Active)); // true
const json = Status.toJSON(Status.Pending);      // "pending"
const parsed = Status.fromJSON("active");        // Status.Active`} lang="typescript" />

<h2 id="type-alias-support">Type Alias Support</h2>

<p>
	All built-in macros work with type aliases. For object type aliases, field-aware methods
	are generated in a namespace:
</p>

<CodeBlock code={`/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
type Point = {
  x: number;
  y: number;
};

// Generated namespace:
// namespace Point {
//   export function toString(value: Point): string { ... }
//   export function clone(value: Point): Point { ... }
//   export function equals(a: Point, b: Point): boolean { ... }
//   export function hashCode(value: Point): number { ... }
//   export function toJSON(value: Point): Record<string, unknown> { ... }
//   export function fromJSON(data: unknown): Point { ... }
// }

const point: Point = { x: 10, y: 20 };
console.log(Point.toString(point));     // "Point { x: 10, y: 20 }"
const copy = Point.clone(point);        // { x: 10, y: 20 }
console.log(Point.equals(point, copy)); // true`} lang="typescript" />

<p>
	Union type aliases also work, using JSON-based implementations:
</p>

<CodeBlock code={`/** @derive(Debug, PartialEq) */
type ApiStatus = "loading" | "success" | "error";

const status: ApiStatus = "success";
console.log(ApiStatus.toString(status)); // "ApiStatus(\\"success\\")"
console.log(ApiStatus.equals("success", "success")); // true`} lang="typescript" />

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
	<li><a href={resolve('/docs/builtin-macros/debug')}><strong>Debug</strong></a> - Customizable field renaming and skipping</li>
	<li><a href={resolve('/docs/builtin-macros/clone')}><strong>Clone</strong></a> - Deep copying for all field types</li>
	<li><a href={resolve('/docs/builtin-macros/default')}><strong>Default</strong></a> - Default value generation with field attributes</li>
	<li><a href={resolve('/docs/builtin-macros/hash')}><strong>Hash</strong></a> - Hash code generation for use in maps and sets</li>
	<li><a href={resolve('/docs/builtin-macros/partial-eq')}><strong>PartialEq</strong></a> - Value-based equality comparison</li>
	<li><a href={resolve('/docs/builtin-macros/ord')}><strong>Ord</strong></a> - Total ordering for sorting</li>
	<li><a href={resolve('/docs/builtin-macros/partial-ord')}><strong>PartialOrd</strong></a> - Partial ordering comparison</li>
	<li><a href={resolve('/docs/builtin-macros/serialize')}><strong>Serialize</strong></a> - JSON serialization with serde-style options</li>
	<li><a href={resolve('/docs/builtin-macros/deserialize')}><strong>Deserialize</strong></a> - JSON deserialization with validation</li>
</ul>
