<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import MacroExample from '$lib/components/ui/MacroExample.svelte';
	import InteractiveMacro from '$lib/components/ui/InteractiveMacro.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Deserialize Macro - Macroforge Documentation</title>
	<meta name="description" content="The Deserialize derive macro generates a fromJSON() method for JSON deserialization with runtime validation." />
</svelte:head>

<h1>Deserialize</h1>

{#if data.version}
	<p class="version-badge">macroforge v{data.version}</p>
{/if}

<p class="lead">
	{data.macro?.description ||
		'The Deserialize macro generates a static fromJSON() method for JSON deserialization with runtime validation.'}
</p>

<h2 id="basic-usage">Basic Usage</h2>

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

<CodeBlock code={`const json = '{"name":"Alice","age":30,"createdAt":"2024-01-15T10:30:00.000Z"}';
const user = User.fromJSON(JSON.parse(json));

console.log(user.name);                    // "Alice"
console.log(user.age);                     // 30
console.log(user.createdAt instanceof Date); // true`} lang="typescript" />

<h2 id="validation">Runtime Validation</h2>

<p>
	Deserialize validates the input data and throws descriptive errors:
</p>

<InteractiveMacro code={`/** @derive(Deserialize) */
class User {
  name: string;
  email: string;
}`} />

<CodeBlock code={`// Missing required field
User.fromJSON({ name: "Alice" });
// Error: User.fromJSON: missing required field "email"

// Wrong type
User.fromJSON("not an object");
// Error: User.fromJSON: expected an object, got string

// Array instead of object
User.fromJSON([1, 2, 3]);
// Error: User.fromJSON: expected an object, got array`} lang="typescript" />

<h2 id="type-handling">Automatic Type Conversion</h2>

<p>
	Deserialize automatically converts JSON types to their TypeScript equivalents:
</p>

<table>
	<thead>
		<tr>
			<th>JSON Type</th>
			<th>TypeScript Type</th>
			<th>Conversion</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td>string/number/boolean</td>
			<td><code>string</code>/<code>number</code>/<code>boolean</code></td>
			<td>Direct assignment</td>
		</tr>
		<tr>
			<td>ISO string</td>
			<td><code>Date</code></td>
			<td><code>new Date(string)</code></td>
		</tr>
		<tr>
			<td>array</td>
			<td><code>T[]</code></td>
			<td>Maps items with auto-detection</td>
		</tr>
		<tr>
			<td>object</td>
			<td><code>Map&lt;K, V&gt;</code></td>
			<td><code>new Map(Object.entries())</code></td>
		</tr>
		<tr>
			<td>array</td>
			<td><code>Set&lt;T&gt;</code></td>
			<td><code>new Set(array)</code></td>
		</tr>
		<tr>
			<td>object</td>
			<td>Nested class</td>
			<td>Calls <code>fromJSON()</code> if available</td>
		</tr>
	</tbody>
</table>

<h2 id="serde-options">Serde Options</h2>

<p>
	Use the <code>@serde</code> decorator to customize deserialization:
</p>

<h3>Renaming Fields</h3>

<MacroExample before={data.examples.rename.before} after={data.examples.rename.after} />

<CodeBlock code={`const user = User.fromJSON({ user_id: "123", full_name: "Alice" });
console.log(user.id);   // "123"
console.log(user.name); // "Alice"`} lang="typescript" />

<h3>Default Values</h3>

<MacroExample before={data.examples.default.before} after={data.examples.default.after} />

<CodeBlock code={`const config = Config.fromJSON({ host: "localhost" });
console.log(config.port);  // "3000"
console.log(config.debug); // false`} lang="typescript" />

<h3>Skipping Fields</h3>

<InteractiveMacro code={`/** @derive(Deserialize) */
class User {
  name: string;
  email: string;

  /** @serde({ skip: true }) */
  cachedData: unknown;

  /** @serde({ skip_deserializing: true }) */
  computedField: string;
}`} />

<Alert type="tip" title="skip vs skip_deserializing">
	Use <code>skip: true</code> to exclude from both serialization and deserialization.
	Use <code>skip_deserializing: true</code> to only skip during deserialization.
</Alert>

<h3>Deny Unknown Fields</h3>

<InteractiveMacro code={`/** @derive(Deserialize) */
/** @serde({ deny_unknown_fields: true }) */
class StrictUser {
  name: string;
  email: string;
}`} />

<CodeBlock code={`// This will throw an error
StrictUser.fromJSON({ name: "Alice", email: "a@b.com", extra: "field" });
// Error: StrictUser.fromJSON: unknown field "extra"`} lang="typescript" />

<h3>Flatten Nested Objects</h3>

<InteractiveMacro code={`/** @derive(Deserialize) */
class Address {
  city: string;
  zip: string;
}

/** @derive(Deserialize) */
class User {
  name: string;

  /** @serde({ flatten: true }) */
  address: Address;
}`} />

<CodeBlock code={`// Flat JSON structure
const user = User.fromJSON({
  name: "Alice",
  city: "NYC",
  zip: "10001"
});
console.log(user.address.city); // "NYC"`} lang="typescript" />

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
		<tr>
			<td><code>deny_unknown_fields</code></td>
			<td><code>boolean</code></td>
			<td>Throw error if JSON has unknown keys</td>
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
			<td><code>skip_deserializing</code></td>
			<td><code>boolean</code></td>
			<td>Exclude from deserialization only</td>
		</tr>
		<tr>
			<td><code>default</code></td>
			<td><code>boolean | string</code></td>
			<td>Use TypeScript default or custom expression if missing</td>
		</tr>
		<tr>
			<td><code>flatten</code></td>
			<td><code>boolean</code></td>
			<td>Merge nested object fields from parent</td>
		</tr>
	</tbody>
</table>

<h2 id="interface-support">Interface Support</h2>

<p>
	Deserialize also works with interfaces. For interfaces, a namespace is generated with <code>is</code> (type guard) and <code>fromJSON</code> functions:
</p>

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

<CodeBlock code={`const json = { status: 200, message: "OK", timestamp: "2024-01-15T10:30:00.000Z" };

// Type guard
if (ApiResponse.is(json)) {
  console.log(json.status); // TypeScript knows this is ApiResponse
}

// Deserialize with validation
const response = ApiResponse.fromJSON(json);
console.log(response.timestamp instanceof Date); // true`} lang="typescript" />

<h2 id="enum-support">Enum Support</h2>

<p>
	Deserialize also works with enums. The <code>fromJSON</code> function validates that the input
	matches one of the enum values:
</p>

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

<CodeBlock code={`const status = Status.fromJSON("active");
console.log(status); // Status.Active

// Invalid values throw an error
try {
  Status.fromJSON("invalid");
} catch (e) {
  console.log(e.message); // "Invalid Status value: invalid"
}`} lang="typescript" />

<p>
	Works with numeric enums too:
</p>

<InteractiveMacro code={`/** @derive(Deserialize) */
enum Priority {
  Low = 1,
  Medium = 2,
  High = 3,
}`} />

<CodeBlock code={`const priority = Priority.fromJSON(3);
console.log(priority); // Priority.High`} lang="typescript" />

<h2 id="type-alias-support">Type Alias Support</h2>

<p>
	Deserialize works with type aliases. For object types, validation and type conversion is applied:
</p>

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

<CodeBlock code={`const json = {
  id: "123",
  name: "Alice",
  createdAt: "2024-01-15T00:00:00.000Z"
};

const profile = UserProfile.fromJSON(json);
console.log(profile.createdAt instanceof Date); // true`} lang="typescript" />

<p>
	For union types, basic validation is applied:
</p>

<InteractiveMacro code={`/** @derive(Deserialize) */
type ApiStatus = "loading" | "success" | "error";`} />

<CodeBlock code={`const status = ApiStatus.fromJSON("success");
console.log(status); // "success"`} lang="typescript" />

<h2 id="combining-with-serialize">Combining with Serialize</h2>

<p>
	Use both Serialize and Deserialize for complete JSON round-trip support:
</p>

<InteractiveMacro code={`/** @derive(Serialize, Deserialize) */
/** @serde({ rename_all: "camelCase" }) */
class UserProfile {
  user_name: string;
  created_at: Date;
  is_active: boolean;
}`} />

<CodeBlock code={`// Create and serialize
const profile = new UserProfile();
profile.user_name = "Alice";
profile.created_at = new Date();
profile.is_active = true;

const json = JSON.stringify(profile);
// {"userName":"Alice","createdAt":"2024-...","isActive":true}

// Deserialize back
const restored = UserProfile.fromJSON(JSON.parse(json));
console.log(restored.user_name);              // "Alice"
console.log(restored.created_at instanceof Date); // true`} lang="typescript" />

<h2 id="error-handling">Error Handling</h2>

<p>
	Handle deserialization errors gracefully:
</p>

<InteractiveMacro code={`/** @derive(Deserialize) */
class User {
  name: string;
  email: string;
}`} />

<CodeBlock code={`function parseUser(json: unknown): User | null {
  try {
    return User.fromJSON(json);
  } catch (error) {
    console.error("Failed to parse user:", error.message);
    return null;
  }
}

const user = parseUser({ name: "Alice" });
// Logs: Failed to parse user: User.fromJSON: missing required field "email"
// Returns: null`} lang="typescript" />

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
