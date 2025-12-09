<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import MacroExample from '$lib/components/ui/MacroExample.svelte';
	import InteractiveMacro from '$lib/components/ui/InteractiveMacro.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Serialize Macro - Macroforge Documentation</title>
	<meta name="description" content="The Serialize derive macro generates a toJSON() method for JSON serialization with automatic type handling." />
</svelte:head>

<h1>Serialize</h1>

<p class="lead">
	The <code>Serialize</code> macro generates a <code>toJSON()</code> method that converts your object to a JSON-compatible format with automatic handling of complex types like Date, Map, Set, and nested objects.
</p>

<h2 id="basic-usage">Basic Usage</h2>

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

<CodeBlock code={`const user = new User("Alice", 30);
console.log(JSON.stringify(user));
// {"name":"Alice","age":30,"createdAt":"2024-01-15T10:30:00.000Z"}`} lang="typescript" />

<h2 id="type-handling">Automatic Type Handling</h2>

<p>
	Serialize automatically handles various TypeScript types:
</p>

<table>
	<thead>
		<tr>
			<th>Type</th>
			<th>Serialization</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><code>string</code>, <code>number</code>, <code>boolean</code></td>
			<td>Direct copy</td>
		</tr>
		<tr>
			<td><code>Date</code></td>
			<td><code>.toISOString()</code></td>
		</tr>
		<tr>
			<td><code>T[]</code></td>
			<td>Maps items, calling <code>toJSON()</code> if available</td>
		</tr>
		<tr>
			<td><code>Map&lt;K, V&gt;</code></td>
			<td><code>Object.fromEntries()</code></td>
		</tr>
		<tr>
			<td><code>Set&lt;T&gt;</code></td>
			<td><code>Array.from()</code></td>
		</tr>
		<tr>
			<td>Nested objects</td>
			<td>Calls <code>toJSON()</code> if available</td>
		</tr>
	</tbody>
</table>

<h2 id="serde-options">Serde Options</h2>

<p>
	Use the <code>@serde</code> decorator for fine-grained control over serialization:
</p>

<h3>Renaming Fields</h3>

<MacroExample before={data.examples.rename.before} after={data.examples.rename.after} />

<CodeBlock code={`const user = new User();
user.id = "123";
user.name = "Alice";
console.log(JSON.stringify(user));
// {"user_id":"123","full_name":"Alice"}`} lang="typescript" />

<h3>Skipping Fields</h3>

<MacroExample before={data.examples.skip.before} after={data.examples.skip.after} />

<Alert type="tip" title="skip vs skip_serializing">
	Use <code>skip: true</code> to exclude from both serialization and deserialization.
	Use <code>skip_serializing: true</code> to only skip during serialization.
</Alert>

<h3>Rename All Fields</h3>

<p>
	Apply a naming convention to all fields at the container level:
</p>

<InteractiveMacro code={`/** @derive(Serialize) */
/** @serde({ rename_all: "camelCase" }) */
class ApiResponse {
  user_name: string;
  created_at: Date;
  is_active: boolean;
}`} />

<p>Supported conventions:</p>
<ul>
	<li><code>camelCase</code></li>
	<li><code>snake_case</code></li>
	<li><code>PascalCase</code></li>
	<li><code>SCREAMING_SNAKE_CASE</code></li>
	<li><code>kebab-case</code></li>
</ul>

<h3>Flattening Nested Objects</h3>

<InteractiveMacro code={`/** @derive(Serialize) */
class Address {
  city: string;
  zip: string;
}

/** @derive(Serialize) */
class User {
  name: string;

  /** @serde({ flatten: true }) */
  address: Address;
}`} />

<CodeBlock code={`const user = new User();
user.name = "Alice";
user.address = { city: "NYC", zip: "10001" };
console.log(JSON.stringify(user));
// {"name":"Alice","city":"NYC","zip":"10001"}`} lang="typescript" />

<h2 id="all-options">All Options</h2>

<h3>Container Options (on class/interface)</h3>

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
			<td><code>rename_all</code></td>
			<td><code>string</code></td>
			<td>Apply naming convention to all fields</td>
		</tr>
	</tbody>
</table>

<h3>Field Options (on properties)</h3>

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
			<td>Use a different JSON key</td>
		</tr>
		<tr>
			<td><code>skip</code></td>
			<td><code>boolean</code></td>
			<td>Exclude from serialization and deserialization</td>
		</tr>
		<tr>
			<td><code>skip_serializing</code></td>
			<td><code>boolean</code></td>
			<td>Exclude from serialization only</td>
		</tr>
		<tr>
			<td><code>flatten</code></td>
			<td><code>boolean</code></td>
			<td>Merge nested object fields into parent</td>
		</tr>
	</tbody>
</table>

<h2 id="interface-support">Interface Support</h2>

<p>
	Serialize also works with interfaces. For interfaces, a namespace is generated with a <code>toJSON</code> function:
</p>

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

<CodeBlock code={`const response: ApiResponse = {
  status: 200,
  message: "OK",
  timestamp: new Date()
};

console.log(JSON.stringify(ApiResponse.toJSON(response)));
// {"status":200,"message":"OK","timestamp":"2024-01-15T10:30:00.000Z"}`} lang="typescript" />

<h2 id="enum-support">Enum Support</h2>

<p>
	Serialize also works with enums. The <code>toJSON</code> function returns the underlying
	enum value (string or number):
</p>

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

<CodeBlock code={`console.log(Status.toJSON(Status.Active));  // "active"
console.log(Status.toJSON(Status.Pending)); // "pending"`} lang="typescript" />

<p>
	Works with numeric enums too:
</p>

<InteractiveMacro code={`/** @derive(Serialize) */
enum Priority {
  Low = 1,
  Medium = 2,
  High = 3,
}`} />

<CodeBlock code={`console.log(Priority.toJSON(Priority.High)); // 3`} lang="typescript" />

<h2 id="type-alias-support">Type Alias Support</h2>

<p>
	Serialize works with type aliases. For object types, fields are serialized with full type handling:
</p>

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

<CodeBlock code={`const profile: UserProfile = {
  id: "123",
  name: "Alice",
  createdAt: new Date("2024-01-15")
};

console.log(JSON.stringify(UserProfile.toJSON(profile)));
// {"id":"123","name":"Alice","createdAt":"2024-01-15T00:00:00.000Z"}`} lang="typescript" />

<p>
	For union types, the value is returned directly:
</p>

<InteractiveMacro code={`/** @derive(Serialize) */
type ApiStatus = "loading" | "success" | "error";`} />

<CodeBlock code={`console.log(ApiStatus.toJSON("success")); // "success"`} lang="typescript" />

<h2 id="combining-with-deserialize">Combining with Deserialize</h2>

<p>
	Use both Serialize and Deserialize for complete JSON round-trip support:
</p>

<InteractiveMacro code={`/** @derive(Serialize, Deserialize) */
class User {
  name: string;
  createdAt: Date;
}`} />

<CodeBlock code={`// Serialize
const user = new User();
user.name = "Alice";
user.createdAt = new Date();
const json = JSON.stringify(user);

// Deserialize
const parsed = User.fromJSON(JSON.parse(json));
console.log(parsed.createdAt instanceof Date); // true`} lang="typescript" />
