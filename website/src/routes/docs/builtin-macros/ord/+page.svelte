<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
</script>

<svelte:head>
	<title>Ord Macro - Macroforge Documentation</title>
	<meta name="description" content="The Ord derive macro generates a compareTo() method for total ordering comparison between objects." />
</svelte:head>

<h1>Ord</h1>

<p class="lead">
	The <code>Ord</code> macro generates a <code>compareTo()</code> method that implements total ordering, always returning <code>-1</code>, <code>0</code>, or <code>1</code>.
</p>

<h2 id="basic-usage">Basic Usage</h2>

<CodeBlock code={`/** @derive(Ord) */
class Version {
  major: number;
  minor: number;
  patch: number;

  constructor(major: number, minor: number, patch: number) {
    this.major = major;
    this.minor = minor;
    this.patch = patch;
  }
}

const v1 = new Version(1, 0, 0);
const v2 = new Version(1, 2, 0);
const v3 = new Version(1, 2, 0);

console.log(v1.compareTo(v2)); // -1 (v1 < v2)
console.log(v2.compareTo(v1)); // 1  (v2 > v1)
console.log(v2.compareTo(v3)); // 0  (v2 == v3)`} lang="typescript" />

<h2 id="generated-code">Generated Code</h2>

<p>The Ord macro generates a compareTo method using lexicographic field comparison:</p>

<CodeBlock code={`compareTo(other: Version): number {
    if (this === other) return 0;
    const typedOther = other;
    const cmp0 = (this.major < typedOther.major ? -1 : this.major > typedOther.major ? 1 : 0);
    if (cmp0 !== 0) return cmp0;
    const cmp1 = (this.minor < typedOther.minor ? -1 : this.minor > typedOther.minor ? 1 : 0);
    if (cmp1 !== 0) return cmp1;
    const cmp2 = (this.patch < typedOther.patch ? -1 : this.patch > typedOther.patch ? 1 : 0);
    if (cmp2 !== 0) return cmp2;
    return 0;
}`} lang="typescript" />

<h2 id="comparison-logic">Comparison Logic</h2>

<p>
	The Ord macro compares fields in declaration order (lexicographic ordering). For each type:
</p>

<ul>
	<li><code>number</code> / <code>bigint</code> → Direct numeric comparison</li>
	<li><code>string</code> → Uses <code>localeCompare()</code> clamped to -1/0/1</li>
	<li><code>boolean</code> → <code>false &lt; true</code></li>
	<li><code>Date</code> → Compares timestamps via <code>getTime()</code></li>
	<li><code>Array</code> → Lexicographic: compares element-by-element, then length</li>
	<li><code>Map/Set</code> → Size and content comparison</li>
	<li><code>Object</code> → Calls <code>compareTo()</code> if available, otherwise 0</li>
	<li><code>null/undefined</code> → Treated as equal (returns 0)</li>
</ul>

<h2 id="return-values">Return Values</h2>

<p>
	The <code>compareTo()</code> method always returns:
</p>

<ul>
	<li><code>-1</code> → <code>this</code> is less than <code>other</code></li>
	<li><code>0</code> → <code>this</code> equals <code>other</code></li>
	<li><code>1</code> → <code>this</code> is greater than <code>other</code></li>
</ul>

<p>
	Unlike <code>PartialOrd</code>, the <code>Ord</code> macro never returns <code>null</code> - it provides total ordering.
</p>

<h2 id="field-options">Field Options</h2>

<h3>@ord(skip)</h3>

<p>
	Use <code>@ord(skip)</code> to exclude a field from ordering comparison:
</p>

<CodeBlock code={`/** @derive(Ord) */
class Task {
  priority: number;
  name: string;

  /** @ord(skip) */
  createdAt: Date;  // Not used for ordering

  constructor(priority: number, name: string, createdAt: Date) {
    this.priority = priority;
    this.name = name;
    this.createdAt = createdAt;
  }
}

const t1 = new Task(1, "Bug fix", new Date("2024-01-01"));
const t2 = new Task(1, "Bug fix", new Date("2024-12-01"));

console.log(t1.compareTo(t2)); // 0 (createdAt is skipped)`} lang="typescript" />

<h2 id="sorting">Sorting Arrays</h2>

<p>
	The generated <code>compareTo()</code> method works directly with <code>Array.sort()</code>:
</p>

<CodeBlock code={`/** @derive(Ord) */
class Score {
  points: number;
  name: string;

  constructor(points: number, name: string) {
    this.points = points;
    this.name = name;
  }
}

const scores = [
  new Score(100, "Alice"),
  new Score(50, "Bob"),
  new Score(150, "Charlie"),
  new Score(50, "Alice")  // Same points, different name
];

// Sort ascending
scores.sort((a, b) => a.compareTo(b));
// Result: [Bob(50), Alice(50), Alice(100), Charlie(150)]

// Sort descending
scores.sort((a, b) => b.compareTo(a));
// Result: [Charlie(150), Alice(100), Alice(50), Bob(50)]`} lang="typescript" />

<h2 id="interface-support">Interface Support</h2>

<p>
	Ord works with interfaces. For interfaces, a namespace is generated with a <code>compareTo</code> function:
</p>

<CodeBlock code={`/** @derive(Ord) */
interface Point {
  x: number;
  y: number;
}

// Generated:
// export namespace Point {
//   export function compareTo(self: Point, other: Point): number {
//     if (self === other) return 0;
//     const cmp0 = (self.x < other.x ? -1 : self.x > other.x ? 1 : 0);
//     if (cmp0 !== 0) return cmp0;
//     const cmp1 = (self.y < other.y ? -1 : self.y > other.y ? 1 : 0);
//     if (cmp1 !== 0) return cmp1;
//     return 0;
//   }
// }

const points: Point[] = [
  { x: 5, y: 10 },
  { x: 1, y: 20 },
  { x: 5, y: 5 }
];

points.sort((a, b) => Point.compareTo(a, b));
// Result: [{ x: 1, y: 20 }, { x: 5, y: 5 }, { x: 5, y: 10 }]`} lang="typescript" />

<h2 id="enum-support">Enum Support</h2>

<p>
	Ord works with enums. For numeric enums, it compares the numeric values; for string enums, it uses string comparison:
</p>

<CodeBlock code={`/** @derive(Ord) */
enum Priority {
  Low = 0,
  Medium = 1,
  High = 2,
  Critical = 3
}

// Generated:
// export namespace Priority {
//   export function compareTo(a: Priority, b: Priority): number {
//     return a < b ? -1 : a > b ? 1 : 0;
//   }
// }

console.log(Priority.compareTo(Priority.Low, Priority.High));      // -1
console.log(Priority.compareTo(Priority.Critical, Priority.Low));  // 1
console.log(Priority.compareTo(Priority.Medium, Priority.Medium)); // 0`} lang="typescript" />

<h2 id="type-alias-support">Type Alias Support</h2>

<p>
	Ord works with type aliases. For object types, it uses lexicographic field comparison:
</p>

<CodeBlock code={`/** @derive(Ord) */
type Coordinate = {
  x: number;
  y: number;
};

// Generated:
// export namespace Coordinate {
//   export function compareTo(a: Coordinate, b: Coordinate): number {
//     if (a === b) return 0;
//     const cmp0 = (a.x < b.x ? -1 : a.x > b.x ? 1 : 0);
//     if (cmp0 !== 0) return cmp0;
//     const cmp1 = (a.y < b.y ? -1 : a.y > b.y ? 1 : 0);
//     if (cmp1 !== 0) return cmp1;
//     return 0;
//   }
// }

const c1: Coordinate = { x: 10, y: 20 };
const c2: Coordinate = { x: 10, y: 30 };

console.log(Coordinate.compareTo(c1, c2)); // -1 (c1 < c2)`} lang="typescript" />

<h2 id="vs-partial-ord">Ord vs PartialOrd</h2>

<p>
	Use <code>Ord</code> when all values of a type are comparable. Use <code>PartialOrd</code> when some values might be incomparable (e.g., different types at runtime).
</p>

<CodeBlock code={`// Ord: Total ordering - never returns null
/** @derive(Ord) */
class Version {
  major: number;
  minor: number;
  constructor(major: number, minor: number) {
    this.major = major;
    this.minor = minor;
  }
}

const v1 = new Version(1, 0);
const v2 = new Version(2, 0);
console.log(v1.compareTo(v2)); // Always -1, 0, or 1

// PartialOrd: Partial ordering - can return null
/** @derive(PartialOrd) */
class Value {
  data: number;
  constructor(data: number) {
    this.data = data;
  }
}

const val = new Value(10);
console.log(val.compareTo("not a Value")); // null (incomparable)`} lang="typescript" />
