<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
</script>

<svelte:head>
	<title>Clone Macro - Macroforge Documentation</title>
	<meta name="description" content="The Clone derive macro generates a clone() method for creating object copies." />
</svelte:head>

<h1>Clone</h1>

<p class="lead">
	The <code>Clone</code> macro generates a <code>clone()</code> method that creates a copy of the object.
</p>

<h2 id="basic-usage">Basic Usage</h2>

<CodeBlock code={`/** @derive(Clone) */
class Point {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}

const original = new Point(10, 20);
const copy = original.clone();

console.log(copy.x, copy.y); // 10, 20
console.log(original === copy); // false (different instances)`} lang="typescript" />

<h2 id="generated-code">Generated Code</h2>

<CodeBlock code={`clone(): Point {
  return new Point(this.x, this.y);
}`} lang="typescript" />

<h2 id="how-it-works">How It Works</h2>

<p>
	The Clone macro:
</p>

<ol>
	<li>Creates a new instance of the class</li>
	<li>Passes all field values to the constructor</li>
	<li>Returns the new instance</li>
</ol>

<p>
	This creates a <strong>shallow clone</strong> - primitive values are copied, but object references remain the same.
</p>

<h2 id="with-nested-objects">With Nested Objects</h2>

<CodeBlock code={`/** @derive(Clone) */
class User {
  name: string;
  address: { city: string; zip: string };

  constructor(name: string, address: { city: string; zip: string }) {
    this.name = name;
    this.address = address;
  }
}

const original = new User("Alice", { city: "NYC", zip: "10001" });
const copy = original.clone();

// The address object is the same reference
console.log(original.address === copy.address); // true

// Modifying the copy's address affects the original
copy.address.city = "LA";
console.log(original.address.city); // "LA"`} lang="typescript" />

<p>
	For deep cloning of nested objects, you would need to implement custom clone methods or use a deep clone utility.
</p>

<h2 id="combining-with-eq">Combining with Eq</h2>

<p>
	Clone works well with Eq for creating independent copies that compare as equal:
</p>

<CodeBlock code={`/** @derive(Clone, Eq) */
class Point {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}

const original = new Point(10, 20);
const copy = original.clone();

console.log(original === copy);       // false (different instances)
console.log(original.equals(copy));   // true (same values)`} lang="typescript" />

<h2 id="interface-support">Interface Support</h2>

<p>
	Clone also works with interfaces. For interfaces, a namespace is generated with a <code>clone</code> function:
</p>

<CodeBlock code={`/** @derive(Clone) */
interface Point {
  x: number;
  y: number;
}

// Generated:
// export namespace Point {
//   export function clone(self: Point): Point {
//     return { x: self.x, y: self.y };
//   }
// }

const original: Point = { x: 10, y: 20 };
const copy = Point.clone(original);

console.log(copy.x, copy.y);        // 10, 20
console.log(original === copy);     // false (different objects)`} lang="typescript" />

<h2 id="enum-support">Enum Support</h2>

<p>
	Clone also works with enums. For enums, the clone function simply returns the value as-is,
	since enum values are primitives and don't need cloning:
</p>

<CodeBlock code={`/** @derive(Clone) */
enum Status {
  Active = "active",
  Inactive = "inactive",
}

// Generated:
// export namespace Status {
//   export function clone(value: Status): Status {
//     return value;
//   }
// }

const original = Status.Active;
const copy = Status.clone(original);

console.log(copy);               // "active"
console.log(original === copy);  // true (same primitive value)`} lang="typescript" />

<h2 id="type-alias-support">Type Alias Support</h2>

<p>
	Clone works with type aliases. For object types, a shallow copy is created using spread:
</p>

<CodeBlock code={`/** @derive(Clone) */
type Point = {
  x: number;
  y: number;
};

// Generated:
// export namespace Point {
//   export function clone(value: Point): Point {
//     return { ...value };
//   }
// }

const original: Point = { x: 10, y: 20 };
const copy = Point.clone(original);

console.log(copy.x, copy.y);     // 10, 20
console.log(original === copy);  // false (different objects)`} lang="typescript" />

<p>
	For union types, the value is returned as-is (unions of primitives don't need cloning):
</p>

<CodeBlock code={`/** @derive(Clone) */
type ApiStatus = "loading" | "success" | "error";

const status: ApiStatus = "success";
const copy = ApiStatus.clone(status);
console.log(copy); // "success"`} lang="typescript" />
