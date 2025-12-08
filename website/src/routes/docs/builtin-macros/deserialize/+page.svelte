<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
</script>

<svelte:head>
	<title>Deserialize Macro - Macroforge Documentation</title>
	<meta name="description" content="The Deserialize derive macro generates a fromJSON() method for JSON deserialization with runtime validation." />
</svelte:head>

<h1>Deserialize</h1>

<p class="lead">
	The <code>Deserialize</code> macro generates a static <code>fromJSON()</code> method that parses JSON data into your class with runtime validation and automatic type conversion.
</p>

<h2 id="basic-usage">Basic Usage</h2>

<CodeBlock code={`/** @derive(Deserialize) */
class User {
  name: string;
  age: number;
  createdAt: Date;
}

const json = '{"name":"Alice","age":30,"createdAt":"2024-01-15T10:30:00.000Z"}';
const user = User.fromJSON(JSON.parse(json));

console.log(user.name);                    // "Alice"
console.log(user.age);                     // 30
console.log(user.createdAt instanceof Date); // true`} lang="typescript" />

<h2 id="generated-code">Generated Code</h2>

<CodeBlock code={`static fromJSON(data: unknown): User {
  if (typeof data !== "object" || data === null || Array.isArray(data)) {
    throw new Error("User.fromJSON: expected an object, got " + typeof data);
  }
  const obj = data as Record<string, unknown>;

  if (!("name" in obj)) {
    throw new Error("User.fromJSON: missing required field \\"name\\"");
  }
  if (!("age" in obj)) {
    throw new Error("User.fromJSON: missing required field \\"age\\"");
  }

  const instance = new User();
  instance.name = obj["name"] as string;
  instance.age = obj["age"] as number;
  instance.createdAt = new Date(obj["createdAt"] as string);
  return instance;
}`} lang="typescript" />

<h2 id="validation">Runtime Validation</h2>

<p>
	Deserialize validates the input data and throws descriptive errors:
</p>

<CodeBlock code={`/** @derive(Deserialize) */
class User {
  name: string;
  email: string;
}

// Missing required field
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

<CodeBlock code={`/** @derive(Deserialize) */
class User {
  /** @serde({ rename: "user_id" }) */
  id: string;

  /** @serde({ rename: "full_name" }) */
  name: string;
}

const user = User.fromJSON({ user_id: "123", full_name: "Alice" });
console.log(user.id);   // "123"
console.log(user.name); // "Alice"`} lang="typescript" />

<h3>Default Values</h3>

<CodeBlock code={`/** @derive(Deserialize) */
class Config {
  host: string;

  /** @serde({ default: "3000" }) */
  port: string;

  /** @serde({ default: "false" }) */
  debug: boolean;
}

const config = Config.fromJSON({ host: "localhost" });
console.log(config.port);  // "3000"
console.log(config.debug); // false`} lang="typescript" />

<h3>Skipping Fields</h3>

<CodeBlock code={`/** @derive(Deserialize) */
class User {
  name: string;
  email: string;

  /** @serde({ skip: true }) */
  cachedData: unknown;

  /** @serde({ skip_deserializing: true }) */
  computedField: string;
}`} lang="typescript" />

<Alert type="tip" title="skip vs skip_deserializing">
	Use <code>skip: true</code> to exclude from both serialization and deserialization.
	Use <code>skip_deserializing: true</code> to only skip during deserialization.
</Alert>

<h3>Deny Unknown Fields</h3>

<CodeBlock code={`/** @derive(Deserialize) */
/** @serde({ deny_unknown_fields: true }) */
class StrictUser {
  name: string;
  email: string;
}

// This will throw an error
StrictUser.fromJSON({ name: "Alice", email: "a@b.com", extra: "field" });
// Error: StrictUser.fromJSON: unknown field "extra"`} lang="typescript" />

<h3>Flatten Nested Objects</h3>

<CodeBlock code={`/** @derive(Deserialize) */
class Address {
  city: string;
  zip: string;
}

/** @derive(Deserialize) */
class User {
  name: string;

  /** @serde({ flatten: true }) */
  address: Address;
}

// Flat JSON structure
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
			<td><code>boolean</code></td>
			<td>Use TypeScript default if missing</td>
		</tr>
		<tr>
			<td><code>default: "expr"</code></td>
			<td><code>string</code></td>
			<td>Custom default expression</td>
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

<CodeBlock code={`/** @derive(Deserialize) */
interface ApiResponse {
  status: number;
  message: string;
  timestamp: Date;
}

// Generated:
// export namespace ApiResponse {
//   export function is(data: unknown): data is ApiResponse {
//     if (typeof data !== "object" || data === null) return false;
//     const obj = data as Record<string, unknown>;
//     if (typeof obj["status"] !== "number") return false;
//     if (typeof obj["message"] !== "string") return false;
//     // ... additional checks
//     return true;
//   }
//
//   export function fromJSON(data: unknown): ApiResponse {
//     if (!is(data)) {
//       throw new Error("ApiResponse.fromJSON: validation failed");
//     }
//     return {
//       ...data,
//       timestamp: new Date(data.timestamp)
//     };
//   }
// }

const json = { status: 200, message: "OK", timestamp: "2024-01-15T10:30:00.000Z" };

// Type guard
if (ApiResponse.is(json)) {
  console.log(json.status); // TypeScript knows this is ApiResponse
}

// Deserialize with validation
const response = ApiResponse.fromJSON(json);
console.log(response.timestamp instanceof Date); // true`} lang="typescript" />

<h2 id="combining-with-serialize">Combining with Serialize</h2>

<p>
	Use both Serialize and Deserialize for complete JSON round-trip support:
</p>

<CodeBlock code={`/** @derive(Serialize, Deserialize) */
/** @serde({ rename_all: "camelCase" }) */
class UserProfile {
  user_name: string;
  created_at: Date;
  is_active: boolean;
}

// Create and serialize
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

<CodeBlock code={`/** @derive(Deserialize) */
class User {
  name: string;
  email: string;
}

function parseUser(json: unknown): User | null {
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
