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

<h2 id="field-validation">Field Validation</h2>

<p>
	Use the <code>validate</code> option to add runtime validation to fields. Validation errors are collected and returned as <code>Result.err(string[])</code>.
</p>

<h3>Basic Validation</h3>

<CodeBlock code={`/** @derive(Deserialize) */
class User {
  /** @serde({ validate: ["email"] }) */
  email: string;

  /** @serde({ validate: ["minLength(2)", "maxLength(50)"] }) */
  name: string;

  /** @serde({ validate: ["positive", "int"] }) */
  age: number;
}

const result = User.fromJSON({ email: "invalid", name: "A", age: -5 });
if (result.isErr()) {
  console.log(result.unwrapErr());
  // [
  //   'User.fromJSON: field "email" must be a valid email',
  //   'User.fromJSON: field "name" must have at least 2 characters',
  //   'User.fromJSON: field "age" must be positive',
  // ]
}`} lang="typescript" />

<h3>Custom Error Messages</h3>

<p>
	Use the object form to provide custom error messages:
</p>

<CodeBlock code={`/** @derive(Deserialize) */
class Product {
  /** @serde({ validate: [
    { validate: "nonEmpty", message: "Product name is required" },
    { validate: "maxLength(100)", message: "Name too long (max 100 chars)" }
  ] }) */
  name: string;

  /** @serde({ validate: [
    { validate: "positive", message: "Price must be greater than zero" }
  ] }) */
  price: number;
}`} lang="typescript" />

<h3>Custom Validator Functions</h3>

<p>
	Use <code>custom(functionName)</code> to call your own validation function:
</p>

<CodeBlock code={`function isValidSKU(value: string): boolean {
  return /^[A-Z]{3}-\\d{4}$/.test(value);
}

/** @derive(Deserialize) */
class Product {
  /** @serde({ validate: [
    { validate: "custom(isValidSKU)", message: "Invalid SKU format (expected XXX-0000)" }
  ] }) */
  sku: string;
}`} lang="typescript" />

<h3>Available Validators</h3>

<h4>String Validators</h4>

<table>
	<thead>
		<tr>
			<th>Validator</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><code>email</code></td>
			<td>Must be a valid email address</td>
		</tr>
		<tr>
			<td><code>url</code></td>
			<td>Must be a valid URL</td>
		</tr>
		<tr>
			<td><code>uuid</code></td>
			<td>Must be a valid UUID</td>
		</tr>
		<tr>
			<td><code>nonEmpty</code></td>
			<td>Must not be empty string</td>
		</tr>
		<tr>
			<td><code>trimmed</code></td>
			<td>Must have no leading/trailing whitespace</td>
		</tr>
		<tr>
			<td><code>lowercase</code></td>
			<td>Must be all lowercase</td>
		</tr>
		<tr>
			<td><code>uppercase</code></td>
			<td>Must be all uppercase</td>
		</tr>
		<tr>
			<td><code>capitalized</code></td>
			<td>First character must be uppercase</td>
		</tr>
		<tr>
			<td><code>uncapitalized</code></td>
			<td>First character must be lowercase</td>
		</tr>
		<tr>
			<td><code>minLength(n)</code></td>
			<td>Must have at least n characters</td>
		</tr>
		<tr>
			<td><code>maxLength(n)</code></td>
			<td>Must have at most n characters</td>
		</tr>
		<tr>
			<td><code>length(n)</code></td>
			<td>Must have exactly n characters</td>
		</tr>
		<tr>
			<td><code>length(min, max)</code></td>
			<td>Must have between min and max characters</td>
		</tr>
		<tr>
			<td><code>pattern("regex")</code></td>
			<td>Must match the regular expression</td>
		</tr>
		<tr>
			<td><code>startsWith("prefix")</code></td>
			<td>Must start with the given prefix</td>
		</tr>
		<tr>
			<td><code>endsWith("suffix")</code></td>
			<td>Must end with the given suffix</td>
		</tr>
		<tr>
			<td><code>includes("substring")</code></td>
			<td>Must contain the substring</td>
		</tr>
	</tbody>
</table>

<h4>Number Validators</h4>

<table>
	<thead>
		<tr>
			<th>Validator</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><code>positive</code></td>
			<td>Must be greater than 0</td>
		</tr>
		<tr>
			<td><code>negative</code></td>
			<td>Must be less than 0</td>
		</tr>
		<tr>
			<td><code>nonNegative</code></td>
			<td>Must be 0 or greater</td>
		</tr>
		<tr>
			<td><code>nonPositive</code></td>
			<td>Must be 0 or less</td>
		</tr>
		<tr>
			<td><code>int</code></td>
			<td>Must be an integer</td>
		</tr>
		<tr>
			<td><code>finite</code></td>
			<td>Must be finite (not Infinity)</td>
		</tr>
		<tr>
			<td><code>nonNaN</code></td>
			<td>Must not be NaN</td>
		</tr>
		<tr>
			<td><code>uint8</code></td>
			<td>Must be integer 0-255</td>
		</tr>
		<tr>
			<td><code>greaterThan(n)</code></td>
			<td>Must be greater than n</td>
		</tr>
		<tr>
			<td><code>greaterThanOrEqualTo(n)</code></td>
			<td>Must be greater than or equal to n</td>
		</tr>
		<tr>
			<td><code>lessThan(n)</code></td>
			<td>Must be less than n</td>
		</tr>
		<tr>
			<td><code>lessThanOrEqualTo(n)</code></td>
			<td>Must be less than or equal to n</td>
		</tr>
		<tr>
			<td><code>between(min, max)</code></td>
			<td>Must be between min and max (inclusive)</td>
		</tr>
		<tr>
			<td><code>multipleOf(n)</code></td>
			<td>Must be a multiple of n</td>
		</tr>
	</tbody>
</table>

<h4>Array Validators</h4>

<table>
	<thead>
		<tr>
			<th>Validator</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><code>minItems(n)</code></td>
			<td>Must have at least n items</td>
		</tr>
		<tr>
			<td><code>maxItems(n)</code></td>
			<td>Must have at most n items</td>
		</tr>
		<tr>
			<td><code>itemsCount(n)</code></td>
			<td>Must have exactly n items</td>
		</tr>
	</tbody>
</table>

<h4>Date Validators</h4>

<table>
	<thead>
		<tr>
			<th>Validator</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><code>validDate</code></td>
			<td>Must be a valid date (not Invalid Date)</td>
		</tr>
		<tr>
			<td><code>greaterThanDate("ISO")</code></td>
			<td>Must be after the given date</td>
		</tr>
		<tr>
			<td><code>greaterThanOrEqualToDate("ISO")</code></td>
			<td>Must be on or after the given date</td>
		</tr>
		<tr>
			<td><code>lessThanDate("ISO")</code></td>
			<td>Must be before the given date</td>
		</tr>
		<tr>
			<td><code>lessThanOrEqualToDate("ISO")</code></td>
			<td>Must be on or before the given date</td>
		</tr>
		<tr>
			<td><code>betweenDate("ISO1", "ISO2")</code></td>
			<td>Must be between the two dates</td>
		</tr>
	</tbody>
</table>

<h4>BigInt Validators</h4>

<table>
	<thead>
		<tr>
			<th>Validator</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><code>positiveBigInt</code></td>
			<td>Must be greater than 0n</td>
		</tr>
		<tr>
			<td><code>negativeBigInt</code></td>
			<td>Must be less than 0n</td>
		</tr>
		<tr>
			<td><code>nonNegativeBigInt</code></td>
			<td>Must be 0n or greater</td>
		</tr>
		<tr>
			<td><code>nonPositiveBigInt</code></td>
			<td>Must be 0n or less</td>
		</tr>
		<tr>
			<td><code>greaterThanBigInt(n)</code></td>
			<td>Must be greater than BigInt(n)</td>
		</tr>
		<tr>
			<td><code>lessThanBigInt(n)</code></td>
			<td>Must be less than BigInt(n)</td>
		</tr>
		<tr>
			<td><code>betweenBigInt(min, max)</code></td>
			<td>Must be between BigInt(min) and BigInt(max)</td>
		</tr>
	</tbody>
</table>

<h4>Custom Validators</h4>

<table>
	<thead>
		<tr>
			<th>Validator</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><code>custom(fnName)</code></td>
			<td>Calls fnName(value), fails if it returns false</td>
		</tr>
	</tbody>
</table>

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
		<tr>
			<td><code>validate</code></td>
			<td><code>string[] | object[]</code></td>
			<td>Array of validators to run during deserialization</td>
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

<h2 id="enum-support">Enum Support</h2>

<p>
	Deserialize also works with enums. The <code>fromJSON</code> function validates that the input
	matches one of the enum values:
</p>

<CodeBlock code={`/** @derive(Deserialize) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}

// Generated:
// export namespace Status {
//   export function fromJSON(data: unknown): Status {
//     for (const key of Object.keys(Status)) {
//       if (Status[key as keyof typeof Status] === data) {
//         return data as Status;
//       }
//     }
//     throw new Error(\`Invalid Status value: \${data}\`);
//   }
// }

const status = Status.fromJSON("active");
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

<CodeBlock code={`/** @derive(Deserialize) */
enum Priority {
  Low = 1,
  Medium = 2,
  High = 3,
}

const priority = Priority.fromJSON(3);
console.log(priority); // Priority.High`} lang="typescript" />

<h2 id="type-alias-support">Type Alias Support</h2>

<p>
	Deserialize works with type aliases. For object types, validation and type conversion is applied:
</p>

<CodeBlock code={`/** @derive(Deserialize) */
type UserProfile = {
  id: string;
  name: string;
  createdAt: Date;
};

// Generated:
// export namespace UserProfile {
//   export function fromJSON(data: unknown): UserProfile {
//     if (typeof data !== "object" || data === null) {
//       throw new Error("UserProfile.fromJSON: expected object");
//     }
//     const obj = data as Record<string, unknown>;
//     return {
//       id: obj["id"] as string,
//       name: obj["name"] as string,
//       createdAt: new Date(obj["createdAt"] as string),
//     };
//   }
// }

const json = {
  id: "123",
  name: "Alice",
  createdAt: "2024-01-15T00:00:00.000Z"
};

const profile = UserProfile.fromJSON(json);
console.log(profile.createdAt instanceof Date); // true`} lang="typescript" />

<p>
	For union types, basic validation is applied:
</p>

<CodeBlock code={`/** @derive(Deserialize) */
type ApiStatus = "loading" | "success" | "error";

const status = ApiStatus.fromJSON("success");
console.log(status); // "success"`} lang="typescript" />

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
