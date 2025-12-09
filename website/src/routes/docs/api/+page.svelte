<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import { base } from '$app/paths';
</script>

<svelte:head>
	<title>API Reference - Macroforge Documentation</title>
	<meta name="description" content="Complete API reference for Macroforge." />
</svelte:head>

<h1>API Reference</h1>

<p class="lead">
	Macroforge provides a programmatic API for expanding macros in TypeScript code.
</p>

<h2 id="overview">Overview</h2>

<CodeBlock code={`import {
  expandSync,
  transformSync,
  checkSyntax,
  parseImportSources,
  NativePlugin,
  PositionMapper
} from "macroforge";`} lang="typescript" />

<h2 id="functions">Core Functions</h2>

<table>
	<thead>
		<tr>
			<th>Function</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><a href="{base}/docs/api/expand-sync"><code>expandSync()</code></a></td>
			<td>Expand macros synchronously</td>
		</tr>
		<tr>
			<td><a href="{base}/docs/api/transform-sync"><code>transformSync()</code></a></td>
			<td>Transform code with additional metadata</td>
		</tr>
		<tr>
			<td><code>checkSyntax()</code></td>
			<td>Validate TypeScript syntax</td>
		</tr>
		<tr>
			<td><code>parseImportSources()</code></td>
			<td>Extract import information</td>
		</tr>
	</tbody>
</table>

<h2 id="classes">Classes</h2>

<table>
	<thead>
		<tr>
			<th>Class</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><a href="{base}/docs/api/native-plugin"><code>NativePlugin</code></a></td>
			<td>Stateful plugin with caching</td>
		</tr>
		<tr>
			<td><a href="{base}/docs/api/position-mapper"><code>PositionMapper</code></a></td>
			<td>Maps positions between original and expanded code</td>
		</tr>
	</tbody>
</table>

<h2 id="quick-example">Quick Example</h2>

<CodeBlock code={`import { expandSync } from "macroforge";

const sourceCode = \`
/** @derive(Debug) */
class User {
  name: string;
  constructor(name: string) {
    this.name = name;
  }
}
\`;

const result = expandSync(sourceCode, "user.ts", {
  keepDecorators: false
});

console.log(result.code);
// Output: class with toString() method generated

if (result.diagnostics.length > 0) {
  console.error("Errors:", result.diagnostics);
}`} lang="typescript" />

<h2 id="detailed-reference">Detailed Reference</h2>

<ul>
	<li><a href="{base}/docs/api/expand-sync"><code>expandSync()</code></a> - Full options and return types</li>
	<li><a href="{base}/docs/api/transform-sync"><code>transformSync()</code></a> - Transform with source maps</li>
	<li><a href="{base}/docs/api/native-plugin"><code>NativePlugin</code></a> - Caching for language servers</li>
	<li><a href="{base}/docs/api/position-mapper"><code>PositionMapper</code></a> - Position mapping utilities</li>
</ul>
