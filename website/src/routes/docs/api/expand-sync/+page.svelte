<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
</script>

<svelte:head>
	<title>expandSync() - Macroforge Documentation</title>
	<meta name="description" content="API reference for the expandSync function." />
</svelte:head>

<h1>expandSync()</h1>

<p class="lead">
	Expands macros in TypeScript code synchronously and returns the transformed output.
</p>

<h2 id="signature">Signature</h2>

<CodeBlock code={`function expandSync(
  code: string,
  filepath: string,
  options?: ExpandOptions
): ExpandResult`} lang="typescript" />

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
		<tr>
			<td><code>options</code></td>
			<td><code>ExpandOptions</code></td>
			<td>Optional configuration</td>
		</tr>
	</tbody>
</table>

<h2 id="expand-options">ExpandOptions</h2>

<CodeBlock code={`interface ExpandOptions {
  // Keep @derive decorators in output (default: false)
  keepDecorators?: boolean;
}`} lang="typescript" />

<h2 id="expand-result">ExpandResult</h2>

<CodeBlock code={`interface ExpandResult {
  // Transformed TypeScript code
  code: string;

  // Generated type declarations (.d.ts content)
  types?: string;

  // Macro expansion metadata (JSON string)
  metadata?: string;

  // Warnings and errors from macro expansion
  diagnostics: MacroDiagnostic[];

  // Position mapping data for source maps
  sourceMapping?: SourceMappingResult;
}`} lang="typescript" />

<h2 id="macro-diagnostic">MacroDiagnostic</h2>

<CodeBlock code={`interface MacroDiagnostic {
  message: string;
  severity: "error" | "warning" | "info";
  span: {
    start: number;
    end: number;
  };
}`} lang="typescript" />

<h2 id="example">Example</h2>

<CodeBlock code={`import { expandSync } from "macroforge";

const sourceCode = \`
/** @derive(Debug) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}
\`;

const result = expandSync(sourceCode, "user.ts");

console.log("Transformed code:");
console.log(result.code);

if (result.types) {
  console.log("Type declarations:");
  console.log(result.types);
}

if (result.diagnostics.length > 0) {
  for (const diag of result.diagnostics) {
    console.log(\`[\${diag.severity}] \${diag.message}\`);
  }
}`} lang="typescript" />

<h2 id="error-handling">Error Handling</h2>

<p>
	Syntax errors and macro errors are returned in the <code>diagnostics</code> array, not thrown as exceptions:
</p>

<CodeBlock code={`const result = expandSync(invalidCode, "file.ts");

for (const diag of result.diagnostics) {
  if (diag.severity === "error") {
    console.error(\`Error at \${diag.span.start}: \${diag.message}\`);
  }
}`} lang="typescript" />
