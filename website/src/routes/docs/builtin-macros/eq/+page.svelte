<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
</script>

<svelte:head>
	<title>Eq Macro - Macroforge Documentation</title>
	<meta name="description" content="The Eq derive macro generates equals() for value-based equality comparison." />
</svelte:head>

<h1>Eq</h1>

<p class="lead">
	The <code>Eq</code> macro generates an <code>equals()</code> method for value-based equality comparison.
</p>

<h2 id="basic-usage">Basic Usage</h2>

<CodeBlock code={`import { Eq } from "macroforge";

/** @derive(Eq) */
class Point {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}

const p1 = new Point(10, 20);
const p2 = new Point(10, 20);
const p3 = new Point(5, 5);

console.log(p1.equals(p2)); // true
console.log(p1.equals(p3)); // false
console.log(p1 === p2);     // false (reference comparison)`} lang="typescript" />

<h2 id="generated-code">Generated Code</h2>

<CodeBlock code={`equals(other: Point): boolean {
  return this.x === other.x && this.y === other.y;
}`} lang="typescript" />

<h2 id="how-it-works">How It Works</h2>

<p>
	The Eq macro compares each field using strict equality (<code>===</code>). Two objects are equal if and only if all their fields have the same values.
</p>

<h2 id="with-different-types">Type Safety</h2>

<p>
	The generated <code>equals()</code> method is type-safe - it only accepts instances of the same class:
</p>

<CodeBlock code={`/** @derive(Eq) */
class User {
  name: string;
  constructor(name: string) {
    this.name = name;
  }
}

/** @derive(Eq) */
class Admin {
  name: string;
  constructor(name: string) {
    this.name = name;
  }
}

const user = new User("Alice");
const admin = new Admin("Alice");

// TypeScript error: Argument of type 'Admin' is not assignable to parameter of type 'User'
// user.equals(admin);`} lang="typescript" />

<h2 id="with-nested-objects">With Nested Objects</h2>

<p>
	For objects with nested object fields, Eq uses reference comparison:
</p>

<CodeBlock code={`/** @derive(Eq) */
class User {
  name: string;
  address: { city: string };

  constructor(name: string, address: { city: string }) {
    this.name = name;
    this.address = address;
  }
}

const address = { city: "NYC" };
const u1 = new User("Alice", address);
const u2 = new User("Alice", address);
const u3 = new User("Alice", { city: "NYC" }); // Different object

console.log(u1.equals(u2)); // true (same address reference)
console.log(u1.equals(u3)); // false (different address reference)`} lang="typescript" />

<h2 id="common-patterns">Common Patterns</h2>

<h3>Use with Sets and Maps</h3>

<CodeBlock code={`/** @derive(Eq) */
class Point {
  x: number;
  y: number;
  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}

// Find a matching point in an array
const points = [new Point(0, 0), new Point(10, 20), new Point(5, 5)];
const target = new Point(10, 20);

const found = points.find(p => p.equals(target));
console.log(found); // Point { x: 10, y: 20 }`} lang="typescript" />

<h3>Use with Debug and Clone</h3>

<CodeBlock code={`import { Debug, Clone, Eq } from "macroforge";

/** @derive(Debug, Clone, Eq) */
class Vector {
  x: number;
  y: number;
  z: number;

  constructor(x: number, y: number, z: number) {
    this.x = x;
    this.y = y;
    this.z = z;
  }
}

const v1 = new Vector(1, 2, 3);
const v2 = v1.clone();

console.log(v1.toString());   // Vector { x: 1, y: 2, z: 3 }
console.log(v1.equals(v2));   // true`} lang="typescript" />
