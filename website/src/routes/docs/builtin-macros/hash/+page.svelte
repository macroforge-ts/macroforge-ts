<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
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

<CodeBlock code={`/** @derive(Hash) */
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

console.log(p1.hashCode()); // Same hash
console.log(p2.hashCode()); // Same hash (equal values = equal hash)
console.log(p3.hashCode()); // Different hash`} lang="typescript" />

<h2 id="generated-code">Generated Code</h2>

<p>The Hash macro generates a method that combines field hashes using the FNV-1a style algorithm:</p>

<CodeBlock code={`hashCode(): number {
    let hash = 17;
    hash = (hash * 31 + (Number.isInteger(this.x) ? this.x | 0 : this.x.toString().split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) | 0;
    hash = (hash * 31 + (Number.isInteger(this.y) ? this.y | 0 : this.y.toString().split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) | 0;
    return hash;
}`} lang="typescript" />

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

<CodeBlock code={`/** @derive(Hash) */
class User {
  id: number;
  name: string;

  /** @hash(skip) */
  lastLogin: Date;  // Not included in hash

  constructor(id: number, name: string, lastLogin: Date) {
    this.id = id;
    this.name = name;
    this.lastLogin = lastLogin;
  }
}

const user1 = new User(1, "Alice", new Date("2024-01-01"));
const user2 = new User(1, "Alice", new Date("2024-12-01"));

console.log(user1.hashCode() === user2.hashCode()); // true (lastLogin is skipped)`} lang="typescript" />

<h2 id="with-partial-eq">Use with PartialEq</h2>

<p>
	Hash is often used together with PartialEq. Objects that are equal should have the same hash code:
</p>

<CodeBlock code={`/** @derive(Hash, PartialEq) */
class Product {
  sku: string;
  name: string;

  constructor(sku: string, name: string) {
    this.sku = sku;
    this.name = name;
  }
}

const p1 = new Product("ABC123", "Widget");
const p2 = new Product("ABC123", "Widget");

// Equal objects have equal hash codes
console.log(p1.equals(p2));                       // true
console.log(p1.hashCode() === p2.hashCode());     // true`} lang="typescript" />

<h2 id="interface-support">Interface Support</h2>

<p>
	Hash also works with interfaces. For interfaces, a namespace is generated with a <code>hashCode</code> function:
</p>

<CodeBlock code={`/** @derive(Hash) */
interface Point {
  x: number;
  y: number;
}

// Generated:
// export namespace Point {
//   export function hashCode(self: Point): number {
//     let hash = 17;
//     hash = (hash * 31 + (self.x | 0)) | 0;
//     hash = (hash * 31 + (self.y | 0)) | 0;
//     return hash;
//   }
// }

const p: Point = { x: 10, y: 20 };
console.log(Point.hashCode(p)); // numeric hash value`} lang="typescript" />

<h2 id="enum-support">Enum Support</h2>

<p>
	Hash works with enums. For string enums, it hashes the string value; for numeric enums, it uses the numeric value directly:
</p>

<CodeBlock code={`/** @derive(Hash) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}

// Generated:
// export namespace Status {
//   export function hashCode(value: Status): number {
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

console.log(Status.hashCode(Status.Active));   // consistent hash
console.log(Status.hashCode(Status.Inactive)); // different hash`} lang="typescript" />

<h2 id="type-alias-support">Type Alias Support</h2>

<p>
	Hash works with type aliases. For object types, it hashes each field:
</p>

<CodeBlock code={`/** @derive(Hash) */
type Coordinates = {
  lat: number;
  lng: number;
};

// Generated:
// export namespace Coordinates {
//   export function hashCode(value: Coordinates): number {
//     let hash = 17;
//     hash = (hash * 31 + (value.lat | 0)) | 0;
//     hash = (hash * 31 + (value.lng | 0)) | 0;
//     return hash;
//   }
// }

const loc: Coordinates = { lat: 40.7128, lng: -74.0060 };
console.log(Coordinates.hashCode(loc));`} lang="typescript" />

<p>
	For union types, it uses JSON stringification as a fallback:
</p>

<CodeBlock code={`/** @derive(Hash) */
type Result = "success" | "error" | "pending";

console.log(Result.hashCode("success")); // hash of "success" string
console.log(Result.hashCode("error"));   // hash of "error" string`} lang="typescript" />
