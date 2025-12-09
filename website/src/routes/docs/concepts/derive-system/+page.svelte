<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
	import { base } from '$app/paths';
</script>

<svelte:head>
	<title>The Derive System - Macroforge Documentation</title>
	<meta name="description" content="Learn how the @derive decorator system works in Macroforge." />
</svelte:head>

<h1>The Derive System</h1>

<p class="lead">
	The derive system is inspired by Rust's derive macros. It allows you to automatically implement common patterns by annotating your classes with <code>@derive</code>.
</p>

<h2 id="syntax">Syntax Reference</h2>

<p>
	Macroforge uses JSDoc comments for all macro annotations. This ensures compatibility with standard TypeScript tooling.
</p>

<h3 id="derive-statement">The @derive Statement</h3>

<p>
	The <code>@derive</code> decorator triggers macro expansion on a class or interface:
</p>

<CodeBlock code={`/** @derive(MacroName) */
class MyClass { }

/** @derive(Debug, Clone, PartialEq) */
class AnotherClass { }`} lang="typescript" />

<p>Syntax rules:</p>

<ul>
	<li>Must be inside a JSDoc comment (<code>/** */</code>)</li>
	<li>Must appear immediately before the class/interface declaration</li>
	<li>Multiple macros can be comma-separated: <code>@derive(A, B, C)</code></li>
	<li>Multiple <code>@derive</code> statements can be stacked</li>
</ul>

<CodeBlock code={`// Single derive with multiple macros
/** @derive(Debug, Clone) */
class User { }

// Multiple derive statements (equivalent)
/** @derive(Debug) */
/** @derive(Clone) */
class User { }`} lang="typescript" />

<h3 id="import-macro">The import macro Statement</h3>

<p>
	To use macros from external packages, you must declare them with <code>import macro</code>:
</p>

<CodeBlock code={`/** import macro { MacroName } from "package-name"; */`} lang="typescript" />

<p>Syntax rules:</p>

<ul>
	<li>Must be inside a JSDoc comment (<code>/** */</code>)</li>
	<li>Can appear anywhere in the file (typically at the top)</li>
	<li>Multiple macros can be imported: <code>import macro &#123; A, B &#125; from "pkg";</code></li>
	<li>Multiple import statements can be used for different packages</li>
</ul>

<CodeBlock code={`/** import macro { JSON, Validate } from "@my/macros"; */
/** import macro { Builder } from "@other/macros"; */

/** @derive(JSON, Validate, Builder) */
class User {
  name: string;
  email: string;
}`} lang="typescript" />

<Alert type="note">
	Built-in macros (Debug, Clone, Default, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize) do not require an import statement.
</Alert>

<h3 id="field-attributes">Field Attributes</h3>

<p>
	Macros can define field-level attributes to customize behavior per field:
</p>

<CodeBlock code={`/** @attributeName(options) */`} lang="typescript" />

<p>
	The attribute name and available options depend on the macro. Common patterns:
</p>

<CodeBlock code={`/** @derive(Debug, Serialize) */
class User {
  /** @debug({ rename: "userId" }) */
  /** @serde({ rename: "user_id" }) */
  id: number;

  name: string;

  /** @debug({ skip: true }) */
  /** @serde({ skip: true }) */
  password: string;

  /** @serde({ flatten: true }) */
  metadata: Record<string, unknown>;
}`} lang="typescript" />

<p>Syntax rules:</p>

<ul>
	<li>Must be inside a JSDoc comment immediately before the field</li>
	<li>Options use object literal syntax: <code>@attr(&#123; key: value &#125;)</code></li>
	<li>Boolean options: <code>@attr(&#123; skip: true &#125;)</code></li>
	<li>String options: <code>@attr(&#123; rename: "newName" &#125;)</code></li>
	<li>Multiple attributes can be on separate lines or combined</li>
</ul>

<p>Common field attributes by macro:</p>

<table>
	<thead>
		<tr>
			<th>Macro</th>
			<th>Attribute</th>
			<th>Options</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td>Debug</td>
			<td><code>@debug</code></td>
			<td><code>skip</code>, <code>rename</code></td>
		</tr>
		<tr>
			<td>Clone</td>
			<td><code>@clone</code></td>
			<td><code>skip</code>, <code>clone_with</code></td>
		</tr>
		<tr>
			<td>Serialize/Deserialize</td>
			<td><code>@serde</code></td>
			<td><code>skip</code>, <code>rename</code>, <code>flatten</code>, <code>default</code></td>
		</tr>
		<tr>
			<td>Hash</td>
			<td><code>@hash</code></td>
			<td><code>skip</code></td>
		</tr>
		<tr>
			<td>PartialEq/Ord</td>
			<td><code>@eq</code>, <code>@ord</code></td>
			<td><code>skip</code></td>
		</tr>
	</tbody>
</table>

<h2 id="how-it-works">How It Works</h2>

<ol>
	<li><strong>Declaration</strong>: You write <code>@derive(MacroName)</code> before a class</li>
	<li><strong>Discovery</strong>: Macroforge finds all derive decorators in your code</li>
	<li><strong>Expansion</strong>: Each named macro receives the class AST and generates code</li>
	<li><strong>Injection</strong>: Generated methods/properties are added to the class</li>
</ol>

<h2 id="what-can-be-derived">What Can Be Derived</h2>

<p>
	The derive system works on:
</p>

<ul>
	<li><strong>Classes</strong>: The primary target for derive macros</li>
	<li><strong>Interfaces</strong>: Some macros can generate companion functions</li>
</ul>

<Alert type="warning">
	Enums are not currently supported by the derive system.
</Alert>

<h2 id="built-in-vs-custom">Built-in vs Custom Macros</h2>

<p>
	Macroforge comes with built-in macros that work out of the box. You can also create custom macros in Rust and use them via the <code>import macro</code> statement.
</p>

<table>
	<thead>
		<tr>
			<th>Type</th>
			<th>Import Required</th>
			<th>Examples</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td>Built-in</td>
			<td>No</td>
			<td>Debug, Clone, Default, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize</td>
		</tr>
		<tr>
			<td>Custom</td>
			<td>Yes</td>
			<td>Any macro from an external package</td>
		</tr>
	</tbody>
</table>

<h2 id="next-steps">Next Steps</h2>

<ul>
	<li><a href="{base}/docs/builtin-macros">Explore built-in macros</a></li>
	<li><a href="{base}/docs/custom-macros">Create custom macros</a></li>
</ul>
