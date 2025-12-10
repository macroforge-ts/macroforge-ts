<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import MacroExample from '$lib/components/ui/MacroExample.svelte';
	import InteractiveMacro from '$lib/components/ui/InteractiveMacro.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Default Macro - Macroforge Documentation</title>
	<meta name="description" content="The Default derive macro generates a static defaultValue() factory method for creating instances with default values, similar to Rust's #[derive(Default)]." />
</svelte:head>

<h1>Default</h1>

<p class="lead">
	The <code>Default</code> macro generates a static <code>defaultValue()</code> factory method that creates instances with default field values. It works like Rust's <code>#[derive(Default)]</code> trait.
</p>

<h2 id="basic-usage">Basic Usage</h2>

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

<CodeBlock code={`const config = Config.defaultValue();
console.log(config.host);    // ""
console.log(config.port);    // 0
console.log(config.enabled); // false`} lang="typescript" />

<h2 id="automatic-defaults">Automatic Default Values</h2>

<p>
	Like Rust's <code>Default</code> trait, the macro automatically determines default values for primitive types and common collections:
</p>

<div class="overflow-x-auto">
	<table class="w-full text-sm">
		<thead>
			<tr>
				<th class="text-left p-2">TypeScript Type</th>
				<th class="text-left p-2">Default Value</th>
				<th class="text-left p-2">Rust Equivalent</th>
			</tr>
		</thead>
		<tbody>
			<tr><td class="p-2"><code>string</code></td><td class="p-2"><code>""</code></td><td class="p-2"><code>String::default()</code></td></tr>
			<tr><td class="p-2"><code>number</code></td><td class="p-2"><code>0</code></td><td class="p-2"><code>i32::default()</code></td></tr>
			<tr><td class="p-2"><code>boolean</code></td><td class="p-2"><code>false</code></td><td class="p-2"><code>bool::default()</code></td></tr>
			<tr><td class="p-2"><code>bigint</code></td><td class="p-2"><code>0n</code></td><td class="p-2"><code>i64::default()</code></td></tr>
			<tr><td class="p-2"><code>T[]</code> / <code>Array&lt;T&gt;</code></td><td class="p-2"><code>[]</code></td><td class="p-2"><code>Vec::default()</code></td></tr>
			<tr><td class="p-2"><code>Map&lt;K, V&gt;</code></td><td class="p-2"><code>new Map()</code></td><td class="p-2"><code>HashMap::default()</code></td></tr>
			<tr><td class="p-2"><code>Set&lt;T&gt;</code></td><td class="p-2"><code>new Set()</code></td><td class="p-2"><code>HashSet::default()</code></td></tr>
			<tr><td class="p-2"><code>Date</code></td><td class="p-2"><code>new Date()</code></td><td class="p-2">—</td></tr>
			<tr><td class="p-2"><code>T | null</code> / <code>T | undefined</code></td><td class="p-2"><code>null</code></td><td class="p-2"><code>Option::default()</code></td></tr>
			<tr><td class="p-2">Custom types</td><td class="p-2 text-red-500"><strong>Error</strong></td><td class="p-2 text-red-500"><strong>Error</strong> (needs <code>impl Default</code>)</td></tr>
		</tbody>
	</table>
</div>

<h2 id="nullable-types">Nullable Types (like Rust's Option)</h2>

<p>
	Just like Rust's <code>Option&lt;T&gt;</code> defaults to <code>None</code>, nullable TypeScript types automatically default to <code>null</code>:
</p>

<MacroExample before={data.examples.nullable.before} after={data.examples.nullable.after} />

<CodeBlock code={`const user = User.defaultValue();
console.log(user.name);     // ""
console.log(user.email);    // null (nullable type)
console.log(user.age);      // 0
console.log(user.metadata); // null (nullable type)`} lang="typescript" />

<h2 id="custom-types">Custom Types Require @default</h2>

<p>
	Just like Rust requires <code>impl Default</code> for custom types, Macroforge requires the <code>@default()</code> decorator on fields with non-primitive types:
</p>

<MacroExample before={data.examples.customType.before} after={data.examples.customType.after} />

<p class="text-red-500 text-sm mt-2">
	Without <code>@default</code> on custom type fields, the macro will emit an error:
</p>

<CodeBlock code={`// Error: @derive(Default) cannot determine default for non-primitive fields.
// Add @default(value) to: settings, permissions`} lang="text" />

<h2 id="custom-defaults">Custom Default Values</h2>

<p>
	Use the <code>@default()</code> decorator to specify custom default values for any field:
</p>

<MacroExample before={data.examples.custom.before} after={data.examples.custom.after} />

<CodeBlock code={`const config = ServerConfig.defaultValue();
console.log(config.host);      // "localhost"
console.log(config.port);      // 8080
console.log(config.enabled);   // true
console.log(config.logLevels); // ["info", "error"]`} lang="typescript" />

<h2 id="interface-support">Interface Support</h2>

<p>
	Default also works with interfaces. For interfaces, a namespace is generated with a <code>defaultValue()</code> function:
</p>

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

<CodeBlock code={`const origin = Point.defaultValue();
console.log(origin); // { x: 0, y: 0 }`} lang="typescript" />

<h2 id="enum-support">Enum Support</h2>

<p>
	Default works with enums. For enums, it returns the first variant as the default value:
</p>

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

<CodeBlock code={`const defaultStatus = Status.defaultValue();
console.log(defaultStatus); // "pending"`} lang="typescript" />

<h2 id="type-alias-support">Type Alias Support</h2>

<p>
	Default works with type aliases. For object types, it creates an object with default field values:
</p>

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

<CodeBlock code={`const dims = Dimensions.defaultValue();
console.log(dims); // { width: 0, height: 0 }`} lang="typescript" />

<h2 id="combining-macros">Combining with Other Macros</h2>

<InteractiveMacro code={`/** @derive(Default, Debug, Clone, PartialEq) */
class User {
  /** @default("Anonymous") */
  name: string;

  /** @default(0) */
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}`} />

<CodeBlock code={`const user1 = User.defaultValue();
const user2 = user1.clone();

console.log(user1.toString());    // User { name: "Anonymous", age: 0 }
console.log(user1.equals(user2)); // true`} lang="typescript" />
