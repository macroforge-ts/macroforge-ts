<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import MacroExample from '$lib/components/ui/MacroExample.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
	import { base } from '$app/paths';

	let { data } = $props();
</script>

<svelte:head>
	<title>First Macro - Macroforge Documentation</title>
	<meta name="description" content="Create your first TypeScript macro with Macroforge - a step-by-step guide." />
</svelte:head>

<h1>Your First Macro</h1>

<p class="lead">
	Let's create a class that uses Macroforge's derive macros to automatically generate useful methods.
</p>

<h2 id="creating-a-class">Creating a Class with Derive Macros</h2>

<p>
	Start by creating a simple <code>User</code> class. We'll use the <code>@derive</code> decorator to automatically generate methods.
</p>

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

<h2 id="using-generated-methods">Using the Generated Methods</h2>

<CodeBlock code={`const user = new User("Alice", 30, "alice@example.com");

// Debug: toString()
console.log(user.toString());
// Output: User { name: Alice, age: 30, email: alice@example.com }

// Clone: clone()
const copy = user.clone();
console.log(copy.name); // "Alice"

// Eq: equals()
console.log(user.equals(copy)); // true

const different = new User("Bob", 25, "bob@example.com");
console.log(user.equals(different)); // false`} lang="typescript" />

<h2 id="customizing-behavior">Customizing Behavior</h2>

<p>
	You can customize how macros work using field-level decorators. For example, with the Debug macro:
</p>

<MacroExample before={data.examples.customizing.before} after={data.examples.customizing.after} />

<CodeBlock code={`const user = new User(42, "Alice", "secret123");
console.log(user.toString());
// Output: User { userId: 42, name: Alice }
// Note: 'id' is renamed to 'userId', 'password' is skipped`} lang="typescript" />

<Alert type="tip" title="Field-level decorators">
	Field-level decorators let you control exactly how each field is handled by the macro.
</Alert>

<h2 id="next-steps">Next Steps</h2>

<ul>
	<li><a href="{base}/docs/concepts">Learn how macros work under the hood</a></li>
	<li><a href="{base}/docs/builtin-macros/debug">Explore all Debug options</a></li>
	<li><a href="{base}/docs/custom-macros">Create your own custom macros</a></li>
</ul>
