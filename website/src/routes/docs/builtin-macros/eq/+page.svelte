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

<CodeBlock code={`/** @derive(Eq) */
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

<p>The Eq macro generates two methods:</p>

<CodeBlock code={`equals(other: Point): boolean {
  return this.x === other.x && this.y === other.y;
}

hashCode(): number {
  let hash = 17;
  hash = hash * 31 + (this.x | 0);
  hash = hash * 31 + (this.y | 0);
  return hash;
}`} lang="typescript" />

<h3>hashCode()</h3>

<p>
	The <code>hashCode()</code> method generates a numeric hash based on field values. Objects with equal values will have the same hash code.
</p>

<CodeBlock code={`const p1 = new Point(10, 20);
const p2 = new Point(10, 20);

console.log(p1.hashCode()); // Same hash
console.log(p2.hashCode()); // Same hash
console.log(p1.equals(p2)); // true`} lang="typescript" />

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

<CodeBlock code={`/** @derive(Debug, Clone, Eq) */
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

<h2 id="interface-support">Interface Support</h2>

<p>
	Eq also works with interfaces. For interfaces, a namespace is generated with <code>equals</code> and <code>hashCode</code> functions:
</p>

<CodeBlock code={`/** @derive(Eq) */
interface Point {
  x: number;
  y: number;
}

// Generated:
// export namespace Point {
//   export function equals(self: Point, other: Point): boolean {
//     if (self === other) return true;
//     return self.x === other.x && self.y === other.y;
//   }
//   export function hashCode(self: Point): number {
//     let hash = 0;
//     hash = (hash * 31 + (self.x ? self.x.toString().charCodeAt(0) : 0)) | 0;
//     hash = (hash * 31 + (self.y ? self.y.toString().charCodeAt(0) : 0)) | 0;
//     return hash;
//   }
// }

const p1: Point = { x: 10, y: 20 };
const p2: Point = { x: 10, y: 20 };
const p3: Point = { x: 5, y: 5 };

console.log(Point.equals(p1, p2)); // true
console.log(Point.equals(p1, p3)); // false`} lang="typescript" />

<h2 id="enum-support">Enum Support</h2>

<p>
	Eq also works with enums. For enums, strict equality comparison is used:
</p>

<CodeBlock code={`/** @derive(Eq) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}

// Generated:
// export namespace Status {
//   export function equals(a: Status, b: Status): boolean {
//     return a === b;
//   }
//   export function hashCode(value: Status): number {
//     // String hash for string enums
//     if (typeof value === "string") {
//       let hash = 0;
//       for (let i = 0; i < value.length; i++) {
//         hash = (hash * 31 + value.charCodeAt(i)) | 0;
//       }
//       return hash;
//     }
//     return value | 0;
//   }
// }

console.log(Status.equals(Status.Active, Status.Active));   // true
console.log(Status.equals(Status.Active, Status.Inactive)); // false
console.log(Status.hashCode(Status.Active)); // consistent hash value`} lang="typescript" />

<h2 id="type-alias-support">Type Alias Support</h2>

<p>
	Eq works with type aliases. For object types, field-by-field comparison is used:
</p>

<CodeBlock code={`/** @derive(Eq) */
type Point = {
  x: number;
  y: number;
};

// Generated:
// export namespace Point {
//   export function equals(a: Point, b: Point): boolean {
//     if (a === b) return true;
//     return a.x === b.x && a.y === b.y;
//   }
//   export function hashCode(value: Point): number {
//     let hash = 0;
//     hash = (hash * 31 + (value.x | 0)) | 0;
//     hash = (hash * 31 + (value.y | 0)) | 0;
//     return hash;
//   }
// }

const p1: Point = { x: 10, y: 20 };
const p2: Point = { x: 10, y: 20 };
const p3: Point = { x: 5, y: 5 };

console.log(Point.equals(p1, p2)); // true
console.log(Point.equals(p1, p3)); // false
console.log(Point.hashCode(p1) === Point.hashCode(p2)); // true`} lang="typescript" />

<p>
	For union types, strict equality is used:
</p>

<CodeBlock code={`/** @derive(Eq) */
type ApiStatus = "loading" | "success" | "error";

console.log(ApiStatus.equals("success", "success")); // true
console.log(ApiStatus.equals("success", "error"));   // false`} lang="typescript" />
