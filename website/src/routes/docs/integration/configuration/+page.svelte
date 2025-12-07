<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
</script>

<svelte:head>
	<title>Configuration - Macroforge Documentation</title>
	<meta name="description" content="Configure Macroforge with macroforge.json." />
</svelte:head>

<h1>Configuration</h1>

<p class="lead">
	Macroforge can be configured with a <code>macroforge.json</code> file in your project root.
</p>

<h2 id="config-file">Configuration File</h2>

<p>Create a <code>macroforge.json</code> file:</p>

<CodeBlock code={`{
  "allowNativeMacros": true,
  "macroPackages": [],
  "keepDecorators": false,
  "limits": {
    "maxExecutionTimeMs": 5000,
    "maxMemoryBytes": 104857600,
    "maxOutputSize": 10485760,
    "maxDiagnostics": 100
  }
}`} lang="json" filename="macroforge.json" />

<h2 id="options">Options Reference</h2>

<h3>allowNativeMacros</h3>

<table>
	<tbody>
		<tr>
			<td>Type</td>
			<td><code>boolean</code></td>
		</tr>
		<tr>
			<td>Default</td>
			<td><code>true</code></td>
		</tr>
	</tbody>
</table>

<p>
	Enable or disable native (Rust) macro packages. Set to <code>false</code> to only allow built-in macros.
</p>

<h3>macroPackages</h3>

<table>
	<tbody>
		<tr>
			<td>Type</td>
			<td><code>string[]</code></td>
		</tr>
		<tr>
			<td>Default</td>
			<td><code>[]</code></td>
		</tr>
	</tbody>
</table>

<p>
	List of npm packages that provide macros. Macroforge will look for macros in these packages.
</p>

<CodeBlock code={`{
  "macroPackages": [
    "@my-org/custom-macros",
    "community-macros"
  ]
}`} lang="json" />

<h3>keepDecorators</h3>

<table>
	<tbody>
		<tr>
			<td>Type</td>
			<td><code>boolean</code></td>
		</tr>
		<tr>
			<td>Default</td>
			<td><code>false</code></td>
		</tr>
	</tbody>
</table>

<p>
	Keep <code>@derive</code> decorators in the output. Useful for debugging.
</p>

<h3>limits</h3>

<p>
	Configure resource limits for macro expansion:
</p>

<CodeBlock code={`{
  "limits": {
    // Maximum time for a single macro expansion (ms)
    "maxExecutionTimeMs": 5000,

    // Maximum memory usage (bytes)
    "maxMemoryBytes": 104857600,  // 100MB

    // Maximum size of generated code (bytes)
    "maxOutputSize": 10485760,    // 10MB

    // Maximum number of diagnostics per file
    "maxDiagnostics": 100
  }
}`} lang="json" />

<h2 id="macro-runtime-overrides">Macro Runtime Overrides</h2>

<p>
	Override settings for specific macros:
</p>

<CodeBlock code={`{
  "macroRuntimeOverrides": {
    "@my-org/macros": {
      "maxExecutionTimeMs": 10000
    }
  }
}`} lang="json" />

<Alert type="warning">
	Be careful when increasing limits, as this could allow malicious macros to consume excessive resources.
</Alert>

<h2 id="environment-variables">Environment Variables</h2>

<p>
	Some settings can be overridden with environment variables:
</p>

<table>
	<thead>
		<tr>
			<th>Variable</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><code>MACROFORGE_DEBUG</code></td>
			<td>Enable debug logging</td>
		</tr>
		<tr>
			<td><code>MACROFORGE_LOG_FILE</code></td>
			<td>Write logs to a file</td>
		</tr>
	</tbody>
</table>

<CodeBlock code={`MACROFORGE_DEBUG=1 npm run dev`} lang="bash" />
