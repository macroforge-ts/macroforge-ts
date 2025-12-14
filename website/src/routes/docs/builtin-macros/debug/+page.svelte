<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import MacroExample from '$lib/components/ui/MacroExample.svelte';
	import InteractiveMacro from '$lib/components/ui/InteractiveMacro.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Debug Macro - Macroforge Documentation</title>
	<meta name="description" content="The Debug derive macro generates a toString() method for human-readable output." />
</svelte:head>

<h1>Debug</h1>

{#if data.version}
	<p class="version-badge">macroforge v{data.version}</p>
{/if}

<p class="lead">
	{data.macro?.description ||
		'The Debug macro generates a toString() method that produces a human-readable string representation of your class.'}
</p>

<h2 id="basic-usage">Basic Usage</h2>

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

<CodeBlock code={`const user = new User("Alice", 30);
console.log(user.toString());
// Output: User { name: Alice, age: 30 }`} lang="typescript" />

<h2 id="field-options">Field Options</h2>

<p>
	Use the <code>@debug</code> field decorator to customize behavior:
</p>

<h3>Renaming Fields</h3>

<MacroExample before={data.examples.rename.before} after={data.examples.rename.after} />

<CodeBlock code={`const user = new User(42, "Alice");
console.log(user.toString());
// Output: User { userId: 42, name: Alice }`} lang="typescript" />

<h3>Skipping Fields</h3>

<p>
	Use <code>skip: true</code> to exclude sensitive fields from the output:
</p>

<MacroExample before={data.examples.skip.before} after={data.examples.skip.after} />

<CodeBlock code={`const user = new User("Alice", "alice@example.com", "secret", "tok_xxx");
console.log(user.toString());
// Output: User { name: Alice, email: alice@example.com }
// Note: password and authToken are not included`} lang="typescript" />

<Alert type="tip" title="Security">
	Always skip sensitive fields like passwords, tokens, and API keys to prevent accidental logging.
</Alert>

<h2 id="combining-options">Combining Options</h2>

<InteractiveMacro code={`/** @derive(Debug) */
class ApiResponse {
  /** @debug({ rename: "statusCode" }) */
  status: number;

  data: unknown;

  /** @debug({ skip: true }) */
  internalMetadata: Record<string, unknown>;
}`} />

<h2 id="all-options">All Options</h2>

<table>
	<thead>
		<tr>
			<th>Option</th>
			<th>Type</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><code>rename</code></td>
			<td><code>string</code></td>
			<td>Display a different name in the output</td>
		</tr>
		<tr>
			<td><code>skip</code></td>
			<td><code>boolean</code></td>
			<td>Exclude this field from the output</td>
		</tr>
	</tbody>
</table>

<h2 id="interface-support">Interface Support</h2>

<p>
	Debug also works with interfaces. For interfaces, a namespace is generated with a <code>toString</code> function:
</p>

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

<CodeBlock code={`const status: Status = { active: true, message: "OK" };
console.log(Status.toString(status));
// Output: Status { active: true, message: OK }`} lang="typescript" />

<h2 id="enum-support">Enum Support</h2>

<p>
	Debug also works with enums. For enums, a namespace is generated with a <code>toString</code> function
	that displays the enum name and variant:
</p>

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

<CodeBlock code={`console.log(Priority.toString(Priority.High));
// Output: Priority.High

console.log(Priority.toString(Priority.Low));
// Output: Priority.Low`} lang="typescript" />

<p>
	Works with both numeric and string enums:
</p>

<InteractiveMacro code={`/** @derive(Debug) */
enum Status {
  Active = "active",
  Inactive = "inactive",
}`} />

<h2 id="type-alias-support">Type Alias Support</h2>

<p>
	Debug works with type aliases. For object types, fields are displayed similar to interfaces:
</p>

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

<CodeBlock code={`const point: Point = { x: 10, y: 20 };
console.log(Point.toString(point));
// Output: Point { x: 10, y: 20 }`} lang="typescript" />

<p>
	For union types, the value is displayed using JSON.stringify:
</p>

<InteractiveMacro code={`/** @derive(Debug) */
type ApiStatus = "loading" | "success" | "error";`} />

<CodeBlock code={`console.log(ApiStatus.toString("success"));
// Output: ApiStatus("success")`} lang="typescript" />

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
