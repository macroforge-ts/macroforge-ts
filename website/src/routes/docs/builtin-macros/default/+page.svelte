<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
</script>

<svelte:head>
	<title>Default Macro - Macroforge Documentation</title>
	<meta name="description" content="The Default derive macro generates a static default() factory method for creating instances with default values." />
</svelte:head>

<h1>Default</h1>

<p class="lead">
	The <code>Default</code> macro generates a static <code>default()</code> factory method that creates instances with default field values.
</p>

<h2 id="basic-usage">Basic Usage</h2>

<CodeBlock code={`/** @derive(Default) */
class Config {
  host: string;
  port: number;
  enabled: boolean;

  constructor(host: string, port: number, enabled: boolean) {
    this.host = host;
    this.port = port;
    this.enabled = enabled;
  }
}

const config = Config.default();
console.log(config.host);    // ""
console.log(config.port);    // 0
console.log(config.enabled); // false`} lang="typescript" />

<h2 id="generated-code">Generated Code</h2>

<p>The Default macro generates a static factory method:</p>

<CodeBlock code={`static default(): Config {
    const instance = new Config();
    instance.host = "";
    instance.port = 0;
    instance.enabled = false;
    return instance;
}`} lang="typescript" />

<h2 id="automatic-defaults">Automatic Default Values</h2>

<p>
	The Default macro automatically determines default values based on field types:
</p>

<ul>
	<li><code>string</code> → <code>""</code> (empty string)</li>
	<li><code>number</code> → <code>0</code></li>
	<li><code>boolean</code> → <code>false</code></li>
	<li><code>bigint</code> → <code>0n</code></li>
	<li><code>Array&lt;T&gt;</code> or <code>T[]</code> → <code>[]</code> (empty array)</li>
	<li><code>Map&lt;K, V&gt;</code> → <code>new Map()</code></li>
	<li><code>Set&lt;T&gt;</code> → <code>new Set()</code></li>
	<li><code>Date</code> → <code>new Date()</code></li>
	<li>Custom types → <code>null as any</code></li>
</ul>

<h2 id="custom-defaults">Custom Default Values</h2>

<p>
	Use the <code>@defaultValue()</code> decorator to specify custom default values for fields:
</p>

<CodeBlock code={`/** @derive(Default) */
class ServerConfig {
  /** @defaultValue("localhost") */
  host: string;

  /** @defaultValue(8080) */
  port: number;

  /** @defaultValue(true) */
  enabled: boolean;

  /** @defaultValue(["info", "error"]) */
  logLevels: string[];

  constructor(host: string, port: number, enabled: boolean, logLevels: string[]) {
    this.host = host;
    this.port = port;
    this.enabled = enabled;
    this.logLevels = logLevels;
  }
}

const config = ServerConfig.default();
console.log(config.host);      // "localhost"
console.log(config.port);      // 8080
console.log(config.enabled);   // true
console.log(config.logLevels); // ["info", "error"]`} lang="typescript" />

<h2 id="interface-support">Interface Support</h2>

<p>
	Default also works with interfaces. For interfaces, a namespace is generated with a <code>default_()</code> function (note the underscore to avoid conflicts with the reserved word):
</p>

<CodeBlock code={`/** @derive(Default) */
interface Point {
  x: number;
  y: number;
}

// Generated:
// export namespace Point {
//   export function default_(): Point {
//     return {
//       x: 0,
//       y: 0
//     } as Point;
//   }
// }

const origin = Point.default_();
console.log(origin); // { x: 0, y: 0 }`} lang="typescript" />

<h2 id="enum-support">Enum Support</h2>

<p>
	Default works with enums. For enums, it returns the first variant as the default value:
</p>

<CodeBlock code={`/** @derive(Default) */
enum Status {
  Pending = "pending",
  Active = "active",
  Completed = "completed",
}

// Generated:
// export namespace Status {
//   export function default_(): Status {
//     return Status.Pending;
//   }
// }

const defaultStatus = Status.default_();
console.log(defaultStatus); // "pending"`} lang="typescript" />

<h2 id="type-alias-support">Type Alias Support</h2>

<p>
	Default works with type aliases. For object types, it creates an object with default field values:
</p>

<CodeBlock code={`/** @derive(Default) */
type Dimensions = {
  width: number;
  height: number;
};

// Generated:
// export namespace Dimensions {
//   export function default_(): Dimensions {
//     return {
//       width: 0,
//       height: 0
//     } as Dimensions;
//   }
// }

const dims = Dimensions.default_();
console.log(dims); // { width: 0, height: 0 }`} lang="typescript" />

<h2 id="combining-macros">Combining with Other Macros</h2>

<CodeBlock code={`/** @derive(Default, Debug, Clone, PartialEq) */
class User {
  /** @defaultValue("Anonymous") */
  name: string;

  /** @defaultValue(0) */
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}

const user1 = User.default();
const user2 = user1.clone();

console.log(user1.toString());    // User { name: "Anonymous", age: 0 }
console.log(user1.equals(user2)); // true`} lang="typescript" />
