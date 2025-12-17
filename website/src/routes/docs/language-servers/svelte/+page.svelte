<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
	import { resolve } from '$app/paths';
</script>

<svelte:head>
	<title>Svelte Language Server - Macroforge Documentation</title>
	<meta name="description" content="Svelte language server with macroforge integration for full IDE support." />
</svelte:head>

<h1>Svelte Language Server</h1>

<p class="lead">
	<code>@macroforge/svelte-language-server</code> provides full Svelte IDE support with macroforge integration.
</p>

<Alert type="warning" title="Developer Installation Required">
	This package is not yet published as an official extension. You'll need to build and install it manually.
</Alert>

<h2 id="features">Features</h2>

<ul>
	<li><strong>Svelte syntax diagnostics</strong> - Errors and warnings in .svelte files</li>
	<li><strong>HTML support</strong> - Hover info, autocompletions, Emmet, outline symbols</li>
	<li><strong>CSS/SCSS/LESS</strong> - Diagnostics, hover, completions, formatting, Emmet, color picking</li>
	<li><strong>TypeScript/JavaScript</strong> - Full language features with macroforge macro expansion</li>
	<li><strong>Go-to-definition</strong> - Navigate to macro-generated code</li>
	<li><strong>Code actions</strong> - Quick fixes and refactorings</li>
</ul>

<h2 id="installation">Installation</h2>

<h3>1. Clone the Repository</h3>

<CodeBlock code={`git clone https://github.com/rymskip/macroforge-ts.git
cd macroforge-ts`} lang="bash" />

<h3>2. Build the Language Server</h3>

<CodeBlock code={`# Install dependencies
npm install

# Build the Svelte language server
cd packages/svelte-language-server
npm run build`} lang="bash" />

<h3>3. Configure Your Editor</h3>

<p>
	The language server exposes a <code>svelteserver</code> binary that implements the Language Server Protocol (LSP). Configure your editor to use it:
</p>

<CodeBlock code={`# The binary is located at:
./packages/svelte-language-server/bin/server.js`} lang="bash" />

<h2 id="package-info">Package Info</h2>

<table>
	<tbody>
		<tr>
			<td>Package</td>
			<td><code>@macroforge/svelte-language-server</code></td>
		</tr>
		<tr>
			<td>Version</td>
			<td>0.1.7</td>
		</tr>
		<tr>
			<td>CLI Command</td>
			<td><code>svelteserver</code></td>
		</tr>
		<tr>
			<td>Node Version</td>
			<td>&gt;= 18.0.0</td>
		</tr>
	</tbody>
</table>

<h2 id="how-it-works">How It Works</h2>

<p>
	The Svelte language server extends the standard Svelte language tooling with macroforge integration:
</p>

<ol>
	<li>Parses <code>.svelte</code> files and extracts TypeScript/JavaScript blocks</li>
	<li>Expands macros using the <code>@macroforge/typescript-plugin</code></li>
	<li>Maps diagnostics back to original source positions</li>
	<li>Provides completions for macro-generated methods</li>
</ol>

<h2 id="zed-integration">Using with Zed</h2>

<p>
	For Zed editor, see the <a href={resolve('/docs/language-servers/zed')}>Zed Extensions</a> page for the dedicated <code>svelte-macroforge</code> extension.
</p>
