<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
</script>

<svelte:head>
	<title>PartialOrd Macro - Macroforge Documentation</title>
	<meta name="description" content="The PartialOrd derive macro generates a compareTo() method for partial ordering, returning null for incomparable values." />
</svelte:head>

<h1>PartialOrd</h1>

<p class="lead">
	The <code>PartialOrd</code> macro generates a <code>compareTo()</code> method that implements partial ordering, returning <code>-1</code>, <code>0</code>, <code>1</code>, or <code>null</code> for incomparable values.
</p>

<h2 id="basic-usage">Basic Usage</h2>

<CodeBlock code={`/** @derive(PartialOrd) */
class Temperature {
  celsius: number;

  constructor(celsius: number) {
    this.celsius = celsius;
  }
}

const t1 = new Temperature(20);
const t2 = new Temperature(30);
const t3 = new Temperature(20);

console.log(t1.compareTo(t2)); // -1 (t1 < t2)
console.log(t2.compareTo(t1)); // 1  (t2 > t1)
console.log(t1.compareTo(t3)); // 0  (t1 == t3)

// Returns null for incomparable types
console.log(t1.compareTo("not a Temperature")); // null`} lang="typescript" />

<h2 id="generated-code">Generated Code</h2>

<p>The PartialOrd macro generates a compareTo method with runtime type checking:</p>

<CodeBlock code={`compareTo(other: unknown): number | null {
    if (this === other) return 0;
    if (!(other instanceof Temperature)) return null;
    const typedOther = other as Temperature;
    const cmp0 = (this.celsius < typedOther.celsius ? -1 : this.celsius > typedOther.celsius ? 1 : 0);
    if (cmp0 === null) return null;
    if (cmp0 !== 0) return cmp0;
    return 0;
}`} lang="typescript" />

<h2 id="return-values">Return Values</h2>

<p>
	The <code>compareTo()</code> method returns:
</p>

<ul>
	<li><code>-1</code> → <code>this</code> is less than <code>other</code></li>
	<li><code>0</code> → <code>this</code> equals <code>other</code></li>
	<li><code>1</code> → <code>this</code> is greater than <code>other</code></li>
	<li><code>null</code> → Values are incomparable (e.g., different types)</li>
</ul>

<h2 id="comparison-logic">Comparison Logic</h2>

<p>
	The PartialOrd macro compares fields in declaration order with type checking:
</p>

<ul>
	<li><code>number</code> / <code>bigint</code> → Direct numeric comparison</li>
	<li><code>string</code> → Uses <code>localeCompare()</code></li>
	<li><code>boolean</code> → <code>false &lt; true</code></li>
	<li><code>Date</code> → Compares timestamps; returns <code>null</code> if not both Date instances</li>
	<li><code>Array</code> → Lexicographic comparison; returns <code>null</code> if not both arrays</li>
	<li><code>Object</code> → Calls <code>compareTo()</code> if available</li>
	<li><strong>Type mismatch</strong> → Returns <code>null</code></li>
</ul>

<h2 id="field-options">Field Options</h2>

<h3>@ord(skip)</h3>

<p>
	Use <code>@ord(skip)</code> to exclude a field from ordering comparison:
</p>

<CodeBlock code={`/** @derive(PartialOrd) */
class Item {
  price: number;
  name: string;

  /** @ord(skip) */
  description: string;  // Not used for ordering

  constructor(price: number, name: string, description: string) {
    this.price = price;
    this.name = name;
    this.description = description;
  }
}

const i1 = new Item(10, "Widget", "A useful widget");
const i2 = new Item(10, "Widget", "Different description");

console.log(i1.compareTo(i2)); // 0 (description is skipped)`} lang="typescript" />

<h2 id="handling-null">Handling Null Results</h2>

<p>
	When using PartialOrd, always handle the <code>null</code> case:
</p>

<CodeBlock code={`/** @derive(PartialOrd) */
class Value {
  amount: number;

  constructor(amount: number) {
    this.amount = amount;
  }
}

function safeCompare(a: Value, b: unknown): string {
  const result = a.compareTo(b);
  if (result === null) {
    return "incomparable";
  } else if (result < 0) {
    return "less than";
  } else if (result > 0) {
    return "greater than";
  } else {
    return "equal";
  }
}

const v = new Value(100);
console.log(safeCompare(v, new Value(50)));  // "greater than"
console.log(safeCompare(v, "string"));       // "incomparable"`} lang="typescript" />

<h2 id="sorting-with-null">Sorting with PartialOrd</h2>

<p>
	When sorting, handle <code>null</code> values appropriately:
</p>

<CodeBlock code={`/** @derive(PartialOrd) */
class Score {
  value: number;

  constructor(value: number) {
    this.value = value;
  }
}

const scores = [
  new Score(100),
  new Score(50),
  new Score(75)
];

// Safe sort that handles null (treats null as equal)
scores.sort((a, b) => a.compareTo(b) ?? 0);
// Result: [Score(50), Score(75), Score(100)]`} lang="typescript" />

<h2 id="interface-support">Interface Support</h2>

<p>
	PartialOrd works with interfaces. For interfaces, a namespace is generated with a <code>compareTo</code> function:
</p>

<CodeBlock code={`/** @derive(PartialOrd) */
interface Measurement {
  value: number;
  unit: string;
}

// Generated:
// export namespace Measurement {
//   export function compareTo(self: Measurement, other: Measurement): number | null {
//     if (self === other) return 0;
//     const cmp0 = (self.value < other.value ? -1 : self.value > other.value ? 1 : 0);
//     if (cmp0 !== 0) return cmp0;
//     const cmp1 = self.unit.localeCompare(other.unit);
//     if (cmp1 !== 0) return cmp1 < 0 ? -1 : 1;
//     return 0;
//   }
// }

const m1: Measurement = { value: 10, unit: "kg" };
const m2: Measurement = { value: 10, unit: "lb" };

console.log(Measurement.compareTo(m1, m2)); // 1 (kg > lb alphabetically)`} lang="typescript" />

<h2 id="enum-support">Enum Support</h2>

<p>
	PartialOrd works with enums:
</p>

<CodeBlock code={`/** @derive(PartialOrd) */
enum Size {
  Small = 1,
  Medium = 2,
  Large = 3
}

// Generated:
// export namespace Size {
//   export function compareTo(a: Size, b: Size): number | null {
//     return a < b ? -1 : a > b ? 1 : 0;
//   }
// }

console.log(Size.compareTo(Size.Small, Size.Large)); // -1
console.log(Size.compareTo(Size.Large, Size.Small)); // 1`} lang="typescript" />

<h2 id="type-alias-support">Type Alias Support</h2>

<p>
	PartialOrd works with type aliases:
</p>

<CodeBlock code={`/** @derive(PartialOrd) */
type Interval = {
  start: number;
  end: number;
};

// Generated:
// export namespace Interval {
//   export function compareTo(a: Interval, b: Interval): number | null {
//     if (a === b) return 0;
//     const cmp0 = (a.start < b.start ? -1 : a.start > b.start ? 1 : 0);
//     if (cmp0 !== 0) return cmp0;
//     const cmp1 = (a.end < b.end ? -1 : a.end > b.end ? 1 : 0);
//     if (cmp1 !== 0) return cmp1;
//     return 0;
//   }
// }

const i1: Interval = { start: 0, end: 10 };
const i2: Interval = { start: 0, end: 20 };

console.log(Interval.compareTo(i1, i2)); // -1`} lang="typescript" />

<h2 id="vs-ord">PartialOrd vs Ord</h2>

<p>
	Choose between <code>Ord</code> and <code>PartialOrd</code> based on your use case:
</p>

<ul>
	<li><strong>Ord</strong> → Use when all values are always comparable (never returns null)</li>
	<li><strong>PartialOrd</strong> → Use when comparing with <code>unknown</code> types or when some values might be incomparable</li>
</ul>

<CodeBlock code={`// PartialOrd is safer for public APIs that accept unknown input
/** @derive(PartialOrd) */
class SafeValue {
  data: number;
  constructor(data: number) {
    this.data = data;
  }

  // Can safely compare with any value
  isGreaterThan(other: unknown): boolean {
    const result = this.compareTo(other);
    return result !== null && result > 0;
  }
}

const safe = new SafeValue(100);
console.log(safe.isGreaterThan(new SafeValue(50)));  // true
console.log(safe.isGreaterThan("invalid"));          // false`} lang="typescript" />
