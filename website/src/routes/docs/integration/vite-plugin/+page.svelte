<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
</script>

<svelte:head>
	<title>Vite Plugin - Macroforge Documentation</title>
	<meta name="description" content="Set up the Macroforge Vite plugin for build-time macro expansion." />
</svelte:head>

<h1>Vite Plugin</h1>

<p class="lead">
	The Vite plugin provides build-time macro expansion, transforming your code during development and production builds.
</p>

<h2 id="installation">Installation</h2>

<CodeBlock code={`npm install -D @macroforge/vite-plugin`} lang="bash" />

<h2 id="configuration">Configuration</h2>

<p>Add the plugin to your <code>vite.config.ts</code>:</p>

<CodeBlock code={`import macroforge from "@macroforge/vite-plugin";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    macroforge()
  ]
});`} lang="typescript" filename="vite.config.ts" />

<h2 id="options">Options</h2>

<CodeBlock code={`macroforge({
  // Generate .d.ts files for expanded code
  generateTypes: true,

  // Output directory for generated types
  typesOutputDir: ".macroforge/types",

  // Emit metadata files for debugging
  emitMetadata: false,

  // Keep @derive decorators in output (for debugging)
  keepDecorators: false,

  // File patterns to process
  include: ["**/*.ts", "**/*.tsx"],
  exclude: ["node_modules/**"]
})`} lang="typescript" />

<h3>Option Reference</h3>

<table>
	<thead>
		<tr>
			<th>Option</th>
			<th>Type</th>
			<th>Default</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><code>generateTypes</code></td>
			<td><code>boolean</code></td>
			<td><code>true</code></td>
			<td>Generate .d.ts files</td>
		</tr>
		<tr>
			<td><code>typesOutputDir</code></td>
			<td><code>string</code></td>
			<td><code>.macroforge/types</code></td>
			<td>Where to write type files</td>
		</tr>
		<tr>
			<td><code>emitMetadata</code></td>
			<td><code>boolean</code></td>
			<td><code>false</code></td>
			<td>Emit macro metadata files</td>
		</tr>
		<tr>
			<td><code>keepDecorators</code></td>
			<td><code>boolean</code></td>
			<td><code>false</code></td>
			<td>Keep decorators in output</td>
		</tr>
	</tbody>
</table>

<h2 id="with-frameworks">Framework Integration</h2>

<h3>React (Vite)</h3>

<CodeBlock code={`import macroforge from "@macroforge/vite-plugin";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    macroforge(),  // Before React plugin
    react()
  ]
});`} lang="typescript" filename="vite.config.ts" />

<h3>SvelteKit</h3>

<CodeBlock code={`import macroforge from "@macroforge/vite-plugin";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    macroforge(),  // Before SvelteKit
    sveltekit()
  ]
});`} lang="typescript" filename="vite.config.ts" />

<Alert type="note">
	Always place the Macroforge plugin before other framework plugins to ensure macros are expanded first.
</Alert>

<h2 id="dev-server">Development Server</h2>

<p>
	During development, the plugin:
</p>

<ul>
	<li>Watches for file changes</li>
	<li>Expands macros on save</li>
	<li>Provides HMR support for expanded code</li>
</ul>

<h2 id="production-build">Production Build</h2>

<p>
	During production builds, the plugin:
</p>

<ul>
	<li>Expands all macros in the source files</li>
	<li>Generates type declaration files</li>
	<li>Strips <code>@derive</code> decorators from output</li>
</ul>
