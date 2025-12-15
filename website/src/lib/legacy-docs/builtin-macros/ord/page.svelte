<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import MacroExample from '$lib/components/ui/MacroExample.svelte';
	import InteractiveMacro from '$lib/components/ui/InteractiveMacro.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Ord Macro - Macroforge Documentation</title>
	<meta name="description" content="The Ord derive macro generates a compareTo() method for total ordering comparison between objects." />
</svelte:head>

<h1>Ord</h1>

{#if data.version}
	<p class="version-badge">macroforge v{data.version}</p>
{/if}

<p class="lead">
	{data.macro?.description ||
		'The Ord macro generates a compareTo() method that implements total ordering.'}
</p>

<h2 id="basic-usage">Basic Usage</h2>

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

<CodeBlock code={`const v1 = new Version(1, 0, 0);
const v2 = new Version(1, 2, 0);
const v3 = new Version(1, 2, 0);

console.log(v1.compareTo(v2)); // -1 (v1 < v2)
console.log(v2.compareTo(v1)); // 1  (v2 > v1)
console.log(v2.compareTo(v3)); // 0  (v2 == v3)`} lang="typescript" />

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

<MacroExample before={data.examples.skip.before} after={data.examples.skip.after} />

<CodeBlock code={`const t1 = new Task(1, "Bug fix", new Date("2024-01-01"));
const t2 = new Task(1, "Bug fix", new Date("2024-12-01"));

console.log(t1.compareTo(t2)); // 0 (createdAt is skipped)`} lang="typescript" />

<h2 id="sorting">Sorting Arrays</h2>

<p>
	The generated <code>compareTo()</code> method works directly with <code>Array.sort()</code>:
</p>

<InteractiveMacro code={`/** @derive(Ord) */
class Score {
  points: number;
  name: string;

  constructor(points: number, name: string) {
    this.points = points;
    this.name = name;
  }
}`} />

<CodeBlock code={`const scores = [
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

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

<CodeBlock code={`const points: Point[] = [
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

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

<CodeBlock code={`console.log(Priority.compareTo(Priority.Low, Priority.High));      // -1
console.log(Priority.compareTo(Priority.Critical, Priority.Low));  // 1
console.log(Priority.compareTo(Priority.Medium, Priority.Medium)); // 0`} lang="typescript" />

<h2 id="type-alias-support">Type Alias Support</h2>

<p>
	Ord works with type aliases. For object types, it uses lexicographic field comparison:
</p>

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

<CodeBlock code={`const c1: Coordinate = { x: 10, y: 20 };
const c2: Coordinate = { x: 10, y: 30 };

console.log(Coordinate.compareTo(c1, c2)); // -1 (c1 < c2)`} lang="typescript" />

<h2 id="vs-partial-ord">Ord vs PartialOrd</h2>

<p>
	Use <code>Ord</code> when all values of a type are comparable. Use <code>PartialOrd</code> when some values might be incomparable (e.g., different types at runtime).
</p>

<InteractiveMacro code={`// Ord: Total ordering - never returns null
/** @derive(Ord) */
class Version {
  major: number;
  minor: number;
  constructor(major: number, minor: number) {
    this.major = major;
    this.minor = minor;
  }
}`} />

<CodeBlock code={`const v1 = new Version(1, 0);
const v2 = new Version(2, 0);
console.log(v1.compareTo(v2)); // Always -1, 0, or 1`} lang="typescript" />

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
