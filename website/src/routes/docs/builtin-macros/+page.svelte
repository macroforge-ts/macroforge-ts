<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import { resolve } from '$app/paths';
</script>

<svelte:head>
	<title>Built-in Macros - Macroforge Documentation</title>
	<meta name="description" content="Overview of Macroforge's built-in derive macros: Debug, Clone, PartialEq, Encode, and Decode." />
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
			<td><code>static toString(value: T): string</code></td>
			<td>Human-readable string representation</td>
		</tr>
		<tr>
			<td><a href={resolve('/docs/builtin-macros/clone')}><code>Clone</code></a></td>
			<td><code>static clone(value: T): T</code></td>
			<td>Creates a deep copy of the object</td>
		</tr>
		<tr>
			<td><a href={resolve('/docs/builtin-macros/default')}><code>Default</code></a></td>
			<td><code>static defaultValue(): T</code></td>
			<td>Creates an instance with default values</td>
		</tr>
		<tr>
			<td><a href={resolve('/docs/builtin-macros/hash')}><code>Hash</code></a></td>
			<td><code>static hashCode(value: T): number</code></td>
			<td>Generates a hash code for the object</td>
		</tr>
		<tr>
			<td><a href={resolve('/docs/builtin-macros/partial-eq')}><code>PartialEq</code></a></td>
			<td><code>static equals(a: T, b: T): boolean</code></td>
			<td>Value equality comparison</td>
		</tr>
		<tr>
			<td><a href={resolve('/docs/builtin-macros/eq')}><code>Eq</code></a></td>
			<td>None</td>
			<td>Marks equality as total; requires <code>PartialEq</code></td>
		</tr>
		<tr>
			<td><a href={resolve('/docs/builtin-macros/ord')}><code>Ord</code></a></td>
			<td><code>static compare(a: T, b: T): number</code></td>
			<td>Total ordering comparison (-1, 0, 1); requires <code>Eq</code> and <code>PartialOrd</code></td>
		</tr>
		<tr>
			<td><a href={resolve('/docs/builtin-macros/partial-ord')}><code>PartialOrd</code></a></td>
			<td><code>static partialCompare(a: T, b: T): number | null</code></td>
			<td>Partial ordering comparison; requires <code>PartialEq</code></td>
		</tr>
		<tr>
			<td><a href={resolve('/docs/builtin-macros/encode')}><code>Encode</code></a></td>
			<td><code>static encode(value: T, keepMetadata?: boolean): string</code></td>
			<td>JSON encoding with type handling</td>
		</tr>
		<tr>
			<td><a href={resolve('/docs/builtin-macros/decode')}><code>Decode</code></a></td>
			<td><code>static decode(input, opts?): &lbrace; success: true; value: T &rbrace; | &lbrace; success: false; errors &rbrace;</code></td>
			<td>JSON decoding with validation</td>
		</tr>
	</tbody>
</table>

<h2 id="type-position-macros">Type-Position Macros</h2>

<p>
	One built-in macro is written where a type goes rather than in <code>@derive</code>:
</p>

<table>
	<thead>
		<tr>
			<th>Macro</th>
			<th>Expands To</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><a href={resolve('/docs/builtin-macros/newtype')}><code>$Newtype&lt;T&gt;</code></a></td>
			<td><code>T &amp; &lbrace; readonly [brand]: true &rbrace;</code></td>
			<td>Nominal brand backed by a per-site <code>unique symbol</code></td>
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
	All built-in macros work with interfaces. Interfaces have no class to attach statics to, so
	they get standalone functions named <code>&lbrace;typeName&rbrace;&lbrace;Operation&rbrace;</code>
	taking the value as the first parameter. When <code>generateConvenienceConst</code> is enabled
	(the default), a grouping <code>const</code> is also emitted so you can call them by short name:
</p>

<CodeBlock code={`/** @derive(Debug, Clone, PartialEq) */
interface Point {
  x: number;
  y: number;
}

// Generated standalone functions:
// export function pointToString(value: Point): string { ... }
// export function pointClone(value: Point): Point { ... }
// export function pointEquals(a: Point, b: Point): boolean { ... }
// export function pointHashCode(value: Point): number { ... }

// Plus a grouping const (generateConvenienceConst, on by default):
// export const Point = {
//   toString: pointToString,
//   clone: pointClone,
//   equals: pointEquals,
//   hashCode: pointHashCode,
// } as const;

const point: Point = { x: 10, y: 20 };

console.log(pointToString(point));      // "Point { x: 10, y: 20 }"
const copy = pointClone(point);         // { x: 10, y: 20 }
console.log(pointEquals(point, copy));  // true

// …or via the grouping const
console.log(Point.toString(point));`} lang="typescript" />

<h2 id="enum-support">Enum Support</h2>

<p>
	All built-in macros work with enums. For enums, methods are generated as functions
	in a namespace with the same name:
</p>

<CodeBlock code={`/** @derive(Debug, Clone, PartialEq, Encode, Decode) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}

// Generated standalone functions:
// export function statusToString(value: Status): string { ... }
// export function statusClone(value: Status): Status { ... }
// export function statusEquals(a: Status, b: Status): boolean { ... }
// export function statusHashCode(value: Status): number { ... }
// export function statusEncode(value: Status): string { ... }
// export function statusDecode(input: unknown): Status { ... }

// Enums use namespace merging for the convenience names:
// namespace Status {
//   export const toString = statusToString;
//   export const encode = statusEncode;
// }

console.log(statusToString(Status.Active));                // "Status.Active"
console.log(statusEquals(Status.Active, Status.Active));   // true
const json = statusEncode(Status.Pending);              // "pending"
// Note: enum decode throws on invalid input rather than
// returning a success/errors union.
const parsed = statusDecode("active");                // Status.Active`} lang="typescript" />

<h2 id="type-alias-support">Type Alias Support</h2>

<p>
	All built-in macros work with type aliases. Object type aliases get field-aware standalone
	functions, plus the optional grouping <code>const</code>:
</p>

<CodeBlock code={`/** @derive(Debug, Clone, PartialEq, Encode, Decode) */
type Point = {
  x: number;
  y: number;
};

// Generated standalone functions:
// export function pointToString(value: Point): string { ... }
// export function pointClone(value: Point): Point { ... }
// export function pointEquals(a: Point, b: Point): boolean { ... }
// export function pointHashCode(value: Point): number { ... }
// export function pointEncode(value: Point, keepMetadata?: boolean): string { ... }
// export function pointDecode(input: unknown, opts?): { success: true; value: Point }
//                                                        | { success: false; errors } { ... }

const point: Point = { x: 10, y: 20 };
console.log(pointToString(point));      // "Point { x: 10, y: 20 }"
const copy = pointClone(point);         // { x: 10, y: 20 }
console.log(pointEquals(point, copy));  // true`} lang="typescript" />

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
console.log(User.toString(user));
// "User { name: Alice, age: 30 }"

// Clone
const copy = User.clone(user);
console.log(copy.name); // "Alice"

// PartialEq
console.log(User.equals(user, copy)); // true`} lang="typescript" />

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
	<li><a href={resolve('/docs/builtin-macros/eq')}><strong>Eq</strong></a> - Marks equality as total, as Rust's <code>Eq</code></li>
	<li><a href={resolve('/docs/builtin-macros/ord')}><strong>Ord</strong></a> - Total ordering for sorting</li>
	<li><a href={resolve('/docs/builtin-macros/partial-ord')}><strong>PartialOrd</strong></a> - Partial ordering comparison</li>
	<li><a href={resolve('/docs/builtin-macros/encode')}><strong>Encode</strong></a> - JSON encoding with endec-style options</li>
	<li><a href={resolve('/docs/builtin-macros/decode')}><strong>Decode</strong></a> - JSON decoding with validation</li>
	<li><a href={resolve('/docs/builtin-macros/newtype')}><strong>$Newtype</strong></a> - Nominal brands, constructed through <code>Decode</code></li>
</ul>
