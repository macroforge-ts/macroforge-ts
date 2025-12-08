<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
</script>

<svelte:head>
	<title>PartialEq Macro - Macroforge Documentation</title>
	<meta name="description" content="The PartialEq derive macro generates an equals() method for value-based equality comparison." />
</svelte:head>

<h1>PartialEq</h1>

<p class="lead">
	The <code>PartialEq</code> macro generates an <code>equals()</code> method for value-based equality comparison between objects.
</p>

<h2 id="basic-usage">Basic Usage</h2>

<CodeBlock code={`/** @derive(PartialEq) */
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

console.log(p1.equals(p2)); // true (same values)
console.log(p1.equals(p3)); // false (different values)
console.log(p1 === p2);     // false (different references)`} lang="typescript" />

<h2 id="generated-code">Generated Code</h2>

<p>The PartialEq macro generates an equals method that compares each field:</p>

<CodeBlock code={`equals(other: unknown): boolean {
    if (this === other) return true;
    if (!(other instanceof Point)) return false;
    const typedOther = other as Point;
    return this.x === typedOther.x && this.y === typedOther.y;
}`} lang="typescript" />

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

<CodeBlock code={`/** @derive(PartialEq) */
class User {
  id: number;
  name: string;

  /** @partialEq(skip) */
  createdAt: Date;  // Not compared

  constructor(id: number, name: string, createdAt: Date) {
    this.id = id;
    this.name = name;
    this.createdAt = createdAt;
  }
}

const user1 = new User(1, "Alice", new Date("2024-01-01"));
const user2 = new User(1, "Alice", new Date("2024-12-01"));

console.log(user1.equals(user2)); // true (createdAt is skipped)`} lang="typescript" />

<h2 id="type-safety">Type Safety</h2>

<p>
	The generated <code>equals()</code> method accepts <code>unknown</code> and performs runtime type checking:
</p>

<CodeBlock code={`/** @derive(PartialEq) */
class User {
  name: string;
  constructor(name: string) {
    this.name = name;
  }
}

/** @derive(PartialEq) */
class Admin {
  name: string;
  constructor(name: string) {
    this.name = name;
  }
}

const user = new User("Alice");
const admin = new Admin("Alice");

console.log(user.equals(admin)); // false (different types)
console.log(user.equals("Alice")); // false (not a User instance)`} lang="typescript" />

<h2 id="with-nested-objects">With Nested Objects</h2>

<p>
	For objects with nested fields, PartialEq recursively calls <code>equals()</code> if available:
</p>

<CodeBlock code={`/** @derive(PartialEq) */
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
}

const addr1 = new Address("NYC", "10001");
const addr2 = new Address("NYC", "10001");

const p1 = new Person("Alice", addr1);
const p2 = new Person("Alice", addr2);

console.log(p1.equals(p2)); // true (deep equality via Address.equals)`} lang="typescript" />

<h2 id="with-arrays">With Arrays</h2>

<CodeBlock code={`/** @derive(PartialEq) */
class Team {
  name: string;
  members: string[];

  constructor(name: string, members: string[]) {
    this.name = name;
    this.members = members;
  }
}

const t1 = new Team("Alpha", ["Alice", "Bob"]);
const t2 = new Team("Alpha", ["Alice", "Bob"]);
const t3 = new Team("Alpha", ["Alice", "Charlie"]);

console.log(t1.equals(t2)); // true
console.log(t1.equals(t3)); // false`} lang="typescript" />

<h2 id="interface-support">Interface Support</h2>

<p>
	PartialEq works with interfaces. For interfaces, a namespace is generated with an <code>equals</code> function:
</p>

<CodeBlock code={`/** @derive(PartialEq) */
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
// }

const p1: Point = { x: 10, y: 20 };
const p2: Point = { x: 10, y: 20 };
const p3: Point = { x: 5, y: 5 };

console.log(Point.equals(p1, p2)); // true
console.log(Point.equals(p1, p3)); // false`} lang="typescript" />

<h2 id="enum-support">Enum Support</h2>

<p>
	PartialEq works with enums. For enums, strict equality comparison is used:
</p>

<CodeBlock code={`/** @derive(PartialEq) */
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
// }

console.log(Status.equals(Status.Active, Status.Active));   // true
console.log(Status.equals(Status.Active, Status.Inactive)); // false`} lang="typescript" />

<h2 id="type-alias-support">Type Alias Support</h2>

<p>
	PartialEq works with type aliases. For object types, field-by-field comparison is used:
</p>

<CodeBlock code={`/** @derive(PartialEq) */
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
// }

const p1: Point = { x: 10, y: 20 };
const p2: Point = { x: 10, y: 20 };

console.log(Point.equals(p1, p2)); // true`} lang="typescript" />

<p>
	For union types, strict equality is used:
</p>

<CodeBlock code={`/** @derive(PartialEq) */
type ApiStatus = "loading" | "success" | "error";

console.log(ApiStatus.equals("success", "success")); // true
console.log(ApiStatus.equals("success", "error"));   // false`} lang="typescript" />

<h2 id="common-patterns">Common Patterns</h2>

<h3>Finding Items in Arrays</h3>

<CodeBlock code={`/** @derive(PartialEq) */
class Product {
  sku: string;
  name: string;

  constructor(sku: string, name: string) {
    this.sku = sku;
    this.name = name;
  }
}

const products = [
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

<CodeBlock code={`/** @derive(PartialEq, Hash) */
class Key {
  id: number;
  type: string;

  constructor(id: number, type: string) {
    this.id = id;
    this.type = type;
  }
}

const k1 = new Key(1, "user");
const k2 = new Key(1, "user");

// Equal objects should have equal hash codes
console.log(k1.equals(k2));                   // true
console.log(k1.hashCode() === k2.hashCode()); // true`} lang="typescript" />
