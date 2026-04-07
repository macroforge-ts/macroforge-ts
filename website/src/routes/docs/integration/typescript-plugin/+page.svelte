<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
</script>

<svelte:head>
	<title>TypeScript Plugin - Macroforge Documentation</title>
	<meta name="description" content="Set up the Macroforge TypeScript plugin for IDE support." />
</svelte:head>

<h1>TypeScript Plugin</h1>

<p class="lead">
	The TypeScript plugin provides IDE integration for Macroforge, including error reporting, completions, and type checking for generated code.
</p>

<h2 id="installation">Installation</h2>

<CodeBlock code={`npm install -D @macroforge/typescript-plugin`} lang="bash" />

<h2 id="configuration">Configuration</h2>

<p>Add the plugin to your <code>tsconfig.json</code>:</p>

<CodeBlock code={`{
  "compilerOptions": {
    "plugins": [
      {
        "name": "@macroforge/typescript-plugin"
      }
    ]
  }
}`} lang="json" filename="tsconfig.json" />

<h2 id="vs-code-setup">VS Code Setup</h2>

<p>
	VS Code uses its own TypeScript version by default. To use the workspace version (which includes plugins):
</p>

<ol>
	<li>Open the Command Palette (<code>Cmd/Ctrl + Shift + P</code>)</li>
	<li>Search for "TypeScript: Select TypeScript Version"</li>
	<li>Choose "Use Workspace Version"</li>
</ol>

<Alert type="tip">
	Add this setting to your <code>.vscode/settings.json</code> to make it permanent:
</Alert>

<CodeBlock code={`{
  "typescript.tsdk": "node_modules/typescript/lib"
}`} lang="json" filename=".vscode/settings.json" />

<h2 id="features">Features</h2>

<h3>Error Reporting</h3>

<p>
	Errors in macro-generated code are reported at the <code>@derive</code> decorator position:
</p>

<CodeBlock code={`/** @derive(Debug) */  // <- Errors appear here
class User {
  name: string;
}`} lang="typescript" />

<h3>Completions</h3>

<p>
	The plugin provides completions for generated methods:
</p>

<CodeBlock code={`const user = new User("Alice");
user.to  // Suggests: toString(), toJSON(), etc.`} lang="typescript" />

<h3>Type Information</h3>

<p>
	Hover over generated methods to see their types:
</p>

<CodeBlock code={`// Hover over 'clone' shows:
// (method) User.clone(): User
const copy = user.clone();`} lang="typescript" />

<h2 id="troubleshooting">Troubleshooting</h2>

<h3>Plugin Not Loading</h3>

<ol>
	<li>Ensure you're using the workspace TypeScript version</li>
	<li>Restart the TypeScript server (Command Palette → "TypeScript: Restart TS Server")</li>
	<li>Check that the plugin is listed in <code>tsconfig.json</code></li>
</ol>

<h3>Errors Not Showing</h3>

<p>
	If errors from macros aren't appearing:
</p>

<ol>
	<li>Make sure the Vite plugin is also installed (for source file watching)</li>
	<li>Check that your file is saved (plugins process on save)</li>
</ol>
