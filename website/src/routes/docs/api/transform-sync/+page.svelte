<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
</script>

<svelte:head>
	<title>transformSync() - Macroforge Documentation</title>
	<meta name="description" content="API reference for the transformSync function." />
</svelte:head>

<h1>transformSync()</h1>

<p class="lead">
	A lower-level transform function that returns additional metadata alongside the transformed code.
</p>

<h2 id="signature">Signature</h2>

<CodeBlock code={`function transformSync(
  code: string,
  filepath: string
): TransformResult`} lang="typescript" />

<h2 id="parameters">Parameters</h2>

<table>
	<thead>
		<tr>
			<th>Parameter</th>
			<th>Type</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><code>code</code></td>
			<td><code>string</code></td>
			<td>TypeScript source code to transform</td>
		</tr>
		<tr>
			<td><code>filepath</code></td>
			<td><code>string</code></td>
			<td>File path (used for error reporting)</td>
		</tr>
	</tbody>
</table>

<h2 id="transform-result">TransformResult</h2>

<CodeBlock code={`interface TransformResult {
  // Transformed TypeScript code
  code: string;

  // Source map (JSON string, not yet implemented)
  map?: string;

  // Generated type declarations
  types?: string;

  // Macro expansion metadata
  metadata?: string;
}`} lang="typescript" />

<h2 id="comparison">Comparison with expandSync()</h2>

<table>
	<thead>
		<tr>
			<th>Feature</th>
			<th><code>expandSync</code></th>
			<th><code>transformSync</code></th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td>Options</td>
			<td>Yes</td>
			<td>No</td>
		</tr>
		<tr>
			<td>Diagnostics</td>
			<td>Yes</td>
			<td>No</td>
		</tr>
		<tr>
			<td>Source Mapping</td>
			<td>Yes</td>
			<td>Limited</td>
		</tr>
		<tr>
			<td>Use Case</td>
			<td>General purpose</td>
			<td>Build tools</td>
		</tr>
	</tbody>
</table>

<h2 id="example">Example</h2>

<CodeBlock code={`import { transformSync } from "macroforge";

const sourceCode = \`
/** @derive(Debug) */
class User {
  name: string;
}
\`;

const result = transformSync(sourceCode, "user.ts");

console.log(result.code);

if (result.types) {
  // Write to .d.ts file
  fs.writeFileSync("user.d.ts", result.types);
}

if (result.metadata) {
  // Parse and use metadata
  const meta = JSON.parse(result.metadata);
  console.log("Macros expanded:", meta);
}`} lang="typescript" />

<h2 id="when-to-use">When to Use</h2>

<p>
	Use <code>transformSync</code> when:
</p>

<ul>
	<li>Building custom integrations</li>
	<li>You need raw output without diagnostics</li>
	<li>You're implementing a build tool plugin</li>
</ul>

<p>
	Use <code>expandSync</code> for most other use cases, as it provides better error handling.
</p>
