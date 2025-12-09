<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import MacroExample from '$lib/components/ui/MacroExample.svelte';
	import InteractiveMacro from '$lib/components/ui/InteractiveMacro.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Hash Macro - Macroforge Documentation</title>
	<meta name="description" content="The Hash derive macro generates a hashCode() method for computing numeric hash values from object fields." />
</svelte:head>

<h1>Hash</h1>

<p class="lead">
	The <code>Hash</code> macro generates a <code>hashCode()</code> method that computes a numeric hash value based on field values.
</p>

<h2 id="basic-usage">Basic Usage</h2>

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

<CodeBlock code={`const p1 = new Point(10, 20);
const p2 = new Point(10, 20);
const p3 = new Point(5, 5);

console.log(p1.hashCode()); // Same hash
console.log(p2.hashCode()); // Same hash (equal values = equal hash)
console.log(p3.hashCode()); // Different hash`} lang="typescript" />

<h2 id="hash-algorithm">Hash Algorithm</h2>

<p>
	The generated hash function uses the following algorithm for different types:
</p>

<ul>
	<li><code>number</code> → Integers use bitwise OR (<code>| 0</code>), floats are stringified and hashed</li>
	<li><code>string</code> → Character-by-character hash: <code>(h * 31 + charCode) | 0</code></li>
	<li><code>boolean</code> → <code>1231</code> for true, <code>1237</code> for false (Java convention)</li>
	<li><code>bigint</code> → Converted to string and hashed character-by-character</li>
	<li><code>Date</code> → Uses <code>getTime() | 0</code> for timestamp hash</li>
	<li><code>Array</code> → Combines element hashes with <code>h * 31 + elementHash</code></li>
	<li><code>Map/Set</code> → Combines all entry hashes</li>
	<li><code>Object</code> → Calls <code>hashCode()</code> if available, otherwise JSON stringifies and hashes</li>
	<li><code>null</code> → Returns 0</li>
	<li><code>undefined</code> → Returns 1</li>
</ul>

<h2 id="field-options">Field Options</h2>

<h3>@hash(skip)</h3>

<p>
	Use <code>@hash(skip)</code> to exclude a field from hash computation:
</p>

<MacroExample before={data.examples.skip.before} after={data.examples.skip.after} />

<CodeBlock code={`const user1 = new User(1, "Alice", new Date("2024-01-01"));
const user2 = new User(1, "Alice", new Date("2024-12-01"));

console.log(user1.hashCode() === user2.hashCode()); // true (lastLogin is skipped)`} lang="typescript" />

<h2 id="with-partial-eq">Use with PartialEq</h2>

<p>
	Hash is often used together with PartialEq. Objects that are equal should have the same hash code:
</p>

<InteractiveMacro code={`/** @derive(Hash, PartialEq) */
class Product {
  sku: string;
  name: string;

  constructor(sku: string, name: string) {
    this.sku = sku;
    this.name = name;
  }
}`} />

<CodeBlock code={`const p1 = new Product("ABC123", "Widget");
const p2 = new Product("ABC123", "Widget");

// Equal objects have equal hash codes
console.log(p1.equals(p2));                       // true
console.log(p1.hashCode() === p2.hashCode());     // true`} lang="typescript" />

<h2 id="interface-support">Interface Support</h2>

<p>
	Hash also works with interfaces. For interfaces, a namespace is generated with a <code>hashCode</code> function:
</p>

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

<CodeBlock code={`const p: Point = { x: 10, y: 20 };
console.log(Point.hashCode(p)); // numeric hash value`} lang="typescript" />

<h2 id="enum-support">Enum Support</h2>

<p>
	Hash works with enums. For string enums, it hashes the string value; for numeric enums, it uses the numeric value directly:
</p>

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

<CodeBlock code={`console.log(Status.hashCode(Status.Active));   // consistent hash
console.log(Status.hashCode(Status.Inactive)); // different hash`} lang="typescript" />

<h2 id="type-alias-support">Type Alias Support</h2>

<p>
	Hash works with type aliases. For object types, it hashes each field:
</p>

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

<CodeBlock code={`const loc: Coordinates = { lat: 40.7128, lng: -74.0060 };
console.log(Coordinates.hashCode(loc));`} lang="typescript" />

<p>
	For union types, it uses JSON stringification as a fallback:
</p>

<InteractiveMacro code={`/** @derive(Hash) */
type Result = "success" | "error" | "pending";`} />

<CodeBlock code={`console.log(Result.hashCode("success")); // hash of "success" string
console.log(Result.hashCode("error"));   // hash of "error" string`} lang="typescript" />
