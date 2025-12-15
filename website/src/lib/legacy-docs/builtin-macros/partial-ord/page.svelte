<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import MacroExample from '$lib/components/ui/MacroExample.svelte';
	import InteractiveMacro from '$lib/components/ui/InteractiveMacro.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>PartialOrd Macro - Macroforge Documentation</title>
	<meta name="description" content="The PartialOrd derive macro generates a compareTo() method for partial ordering, returning null for incomparable values." />
</svelte:head>

<h1>PartialOrd</h1>

{#if data.version}
	<p class="version-badge">macroforge v{data.version}</p>
{/if}

<p class="lead">
	{data.macro?.description ||
		'The PartialOrd macro generates a compareTo() method that implements partial ordering.'}
</p>

<h2 id="basic-usage">Basic Usage</h2>

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

<CodeBlock code={`const t1 = new Temperature(20);
const t2 = new Temperature(30);
const t3 = new Temperature(20);

console.log(t1.compareTo(t2)); // -1 (t1 < t2)
console.log(t2.compareTo(t1)); // 1  (t2 > t1)
console.log(t1.compareTo(t3)); // 0  (t1 == t3)

// Returns null for incomparable types
console.log(t1.compareTo("not a Temperature")); // null`} lang="typescript" />

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

<MacroExample before={data.examples.skip.before} after={data.examples.skip.after} />

<CodeBlock code={`const i1 = new Item(10, "Widget", "A useful widget");
const i2 = new Item(10, "Widget", "Different description");

console.log(i1.compareTo(i2)); // 0 (description is skipped)`} lang="typescript" />

<h2 id="handling-null">Handling Null Results</h2>

<p>
	When using PartialOrd, always handle the <code>null</code> case:
</p>

<InteractiveMacro code={`/** @derive(PartialOrd) */
class Value {
  amount: number;

  constructor(amount: number) {
    this.amount = amount;
  }
}`} />

<CodeBlock code={`function safeCompare(a: Value, b: unknown): string {
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

<InteractiveMacro code={`/** @derive(PartialOrd) */
class Score {
  value: number;

  constructor(value: number) {
    this.value = value;
  }
}`} />

<CodeBlock code={`const scores = [
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

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

<CodeBlock code={`const m1: Measurement = { value: 10, unit: "kg" };
const m2: Measurement = { value: 10, unit: "lb" };

console.log(Measurement.compareTo(m1, m2)); // 1 (kg > lb alphabetically)`} lang="typescript" />

<h2 id="enum-support">Enum Support</h2>

<p>
	PartialOrd works with enums:
</p>

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

<CodeBlock code={`console.log(Size.compareTo(Size.Small, Size.Large)); // -1
console.log(Size.compareTo(Size.Large, Size.Small)); // 1`} lang="typescript" />

<h2 id="type-alias-support">Type Alias Support</h2>

<p>
	PartialOrd works with type aliases:
</p>

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

<CodeBlock code={`const i1: Interval = { start: 0, end: 10 };
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

<InteractiveMacro code={`// PartialOrd is safer for public APIs that accept unknown input
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
}`} />

<CodeBlock code={`const safe = new SafeValue(100);
console.log(safe.isGreaterThan(new SafeValue(50)));  // true
console.log(safe.isGreaterThan("invalid"));          // false`} lang="typescript" />

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
