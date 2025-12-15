<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import MacroExample from '$lib/components/ui/MacroExample.svelte';
	import InteractiveMacro from '$lib/components/ui/InteractiveMacro.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>PartialEq Macro - Macroforge Documentation</title>
	<meta name="description" content="The PartialEq derive macro generates an equals() method for value-based equality comparison." />
</svelte:head>

<h1>PartialEq</h1>

{#if data.version}
	<p class="version-badge">macroforge v{data.version}</p>
{/if}

<p class="lead">
	{data.macro?.description ||
		'The PartialEq macro generates an equals() method for value-based equality comparison.'}
</p>

<h2 id="basic-usage">Basic Usage</h2>

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

<CodeBlock code={`const p1 = new Point(10, 20);
const p2 = new Point(10, 20);
const p3 = new Point(5, 5);

console.log(p1.equals(p2)); // true (same values)
console.log(p1.equals(p3)); // false (different values)
console.log(p1 === p2);     // false (different references)`} lang="typescript" />

<h2 id="how-it-works">How It Works</h2>

<p>
	The PartialEq macro performs field-by-field comparison using these strategies:
</p>

<ul>
	<li><strong>Primitives</strong> (string, number, boolean, null, undefined) → Strict equality (<code>===</code>)</li>
	<li><strong>Date</strong> → Compares timestamps via <code>getTime()</code></li>
	<li><strong>Array</strong> → Length check + element-by-element comparison</li>
	<li><strong>Map</strong> → Size check + entry comparison</li>
	<li><strong>Set</strong> → Size check + membership verification</li>
	<li><strong>Objects</strong> → Calls <code>equals()</code> if available, otherwise <code>===</code></li>
</ul>

<h2 id="field-options">Field Options</h2>

<h3>@partialEq(skip)</h3>

<p>
	Use <code>@partialEq(skip)</code> to exclude a field from equality comparison:
</p>

<MacroExample before={data.examples.skip.before} after={data.examples.skip.after} />

<CodeBlock code={`const user1 = new User(1, "Alice", new Date("2024-01-01"));
const user2 = new User(1, "Alice", new Date("2024-12-01"));

console.log(user1.equals(user2)); // true (createdAt is skipped)`} lang="typescript" />

<h2 id="type-safety">Type Safety</h2>

<p>
	The generated <code>equals()</code> method accepts <code>unknown</code> and performs runtime type checking:
</p>

<InteractiveMacro code={`/** @derive(PartialEq) */
class User {
  name: string;
  constructor(name: string) {
    this.name = name;
  }
}`} />

<CodeBlock code={`const user = new User("Alice");

console.log(user.equals(new User("Alice"))); // true
console.log(user.equals("Alice")); // false (not a User instance)`} lang="typescript" />

<h2 id="with-nested-objects">With Nested Objects</h2>

<p>
	For objects with nested fields, PartialEq recursively calls <code>equals()</code> if available:
</p>

<InteractiveMacro code={`/** @derive(PartialEq) */
class Address {
  city: string;
  zip: string;

  constructor(city: string, zip: string) {
    this.city = city;
    this.zip = zip;
  }
}

/** @derive(PartialEq) */
class Person {
  name: string;
  address: Address;

  constructor(name: string, address: Address) {
    this.name = name;
    this.address = address;
  }
}`} />

<CodeBlock code={`const addr1 = new Address("NYC", "10001");
const addr2 = new Address("NYC", "10001");

const p1 = new Person("Alice", addr1);
const p2 = new Person("Alice", addr2);

console.log(p1.equals(p2)); // true (deep equality via Address.equals)`} lang="typescript" />

<h2 id="interface-support">Interface Support</h2>

<p>
	PartialEq works with interfaces. For interfaces, a namespace is generated with an <code>equals</code> function:
</p>

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

<CodeBlock code={`const p1: Point = { x: 10, y: 20 };
const p2: Point = { x: 10, y: 20 };
const p3: Point = { x: 5, y: 5 };

console.log(Point.equals(p1, p2)); // true
console.log(Point.equals(p1, p3)); // false`} lang="typescript" />

<h2 id="enum-support">Enum Support</h2>

<p>
	PartialEq works with enums. For enums, strict equality comparison is used:
</p>

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

<CodeBlock code={`console.log(Status.equals(Status.Active, Status.Active));   // true
console.log(Status.equals(Status.Active, Status.Inactive)); // false`} lang="typescript" />

<h2 id="type-alias-support">Type Alias Support</h2>

<p>
	PartialEq works with type aliases. For object types, field-by-field comparison is used:
</p>

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

<CodeBlock code={`const p1: Point = { x: 10, y: 20 };
const p2: Point = { x: 10, y: 20 };

console.log(Point.equals(p1, p2)); // true`} lang="typescript" />

<p>
	For union types, strict equality is used:
</p>

<InteractiveMacro code={`/** @derive(PartialEq) */
type ApiStatus = "loading" | "success" | "error";`} />

<CodeBlock code={`console.log(ApiStatus.equals("success", "success")); // true
console.log(ApiStatus.equals("success", "error"));   // false`} lang="typescript" />

<h2 id="common-patterns">Common Patterns</h2>

<h3>Finding Items in Arrays</h3>

<InteractiveMacro code={`/** @derive(PartialEq) */
class Product {
  sku: string;
  name: string;

  constructor(sku: string, name: string) {
    this.sku = sku;
    this.name = name;
  }
}`} />

<CodeBlock code={`const products = [
  new Product("A1", "Widget"),
  new Product("B2", "Gadget"),
  new Product("C3", "Gizmo")
];

const target = new Product("B2", "Gadget");
const found = products.find(p => p.equals(target));

console.log(found); // Product { sku: "B2", name: "Gadget" }`} lang="typescript" />

<h3>Use with Hash</h3>

<p>
	When using objects as keys in Map-like structures, combine PartialEq with Hash:
</p>

<InteractiveMacro code={`/** @derive(PartialEq, Hash) */
class Key {
  id: number;
  type: string;

  constructor(id: number, type: string) {
    this.id = id;
    this.type = type;
  }
}`} />

<CodeBlock code={`const k1 = new Key(1, "user");
const k2 = new Key(1, "user");

// Equal objects should have equal hash codes
console.log(k1.equals(k2));                   // true
console.log(k1.hashCode() === k2.hashCode()); // true`} lang="typescript" />

<style>
	.version-badge {
		display: inline-block;
		background: var(--color-primary);
		color: white;
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 500;
		margin-bottom: 1rem;
	}
</style>
