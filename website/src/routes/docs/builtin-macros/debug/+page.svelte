<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
</script>

<svelte:head>
	<title>Debug Macro - Macroforge Documentation</title>
	<meta name="description" content="The Debug derive macro generates a toString() method for human-readable output." />
</svelte:head>

<h1>Debug</h1>

<p class="lead">
	The <code>Debug</code> macro generates a <code>toString()</code> method that produces a human-readable string representation of your class.
</p>

<h2 id="basic-usage">Basic Usage</h2>

<CodeBlock code={`/** @derive(Debug) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}

const user = new User("Alice", 30);
console.log(user.toString());
// Output: User { name: Alice, age: 30 }`} lang="typescript" />

<h2 id="generated-code">Generated Code</h2>

<CodeBlock code={`toString(): string {
  const parts: string[] = [];
  parts.push("name: " + this.name);
  parts.push("age: " + this.age);
  return "User { " + parts.join(", ") + " }";
}`} lang="typescript" />

<h2 id="field-options">Field Options</h2>

<p>
	Use the <code>@debug</code> field decorator to customize behavior:
</p>

<h3>Renaming Fields</h3>

<CodeBlock code={`/** @derive(Debug) */
class User {
  /** @debug({ rename: "userId" }) */
  id: number;

  name: string;

  constructor(id: number, name: string) {
    this.id = id;
    this.name = name;
  }
}

const user = new User(42, "Alice");
console.log(user.toString());
// Output: User { userId: 42, name: Alice }`} lang="typescript" />

<h3>Skipping Fields</h3>

<p>
	Use <code>skip: true</code> to exclude sensitive fields from the output:
</p>

<CodeBlock code={`/** @derive(Debug) */
class User {
  name: string;
  email: string;

  /** @debug({ skip: true }) */
  password: string;

  /** @debug({ skip: true }) */
  authToken: string;

  constructor(name: string, email: string, password: string, authToken: string) {
    this.name = name;
    this.email = email;
    this.password = password;
    this.authToken = authToken;
  }
}

const user = new User("Alice", "alice@example.com", "secret", "tok_xxx");
console.log(user.toString());
// Output: User { name: Alice, email: alice@example.com }
// Note: password and authToken are not included`} lang="typescript" />

<Alert type="tip" title="Security">
	Always skip sensitive fields like passwords, tokens, and API keys to prevent accidental logging.
</Alert>

<h2 id="combining-options">Combining Options</h2>

<CodeBlock code={`/** @derive(Debug) */
class ApiResponse {
  /** @debug({ rename: "statusCode" }) */
  status: number;

  data: unknown;

  /** @debug({ skip: true }) */
  internalMetadata: Record<string, unknown>;
}`} lang="typescript" />

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

<CodeBlock code={`/** @derive(Debug) */
interface Status {
  active: boolean;
  message: string;
}

// Generated:
// export namespace Status {
//   export function toString(self: Status): string {
//     const parts: string[] = [];
//     parts.push("active: " + self.active);
//     parts.push("message: " + self.message);
//     return "Status { " + parts.join(", ") + " }";
//   }
// }

const status: Status = { active: true, message: "OK" };
console.log(Status.toString(status));
// Output: Status { active: true, message: OK }`} lang="typescript" />
