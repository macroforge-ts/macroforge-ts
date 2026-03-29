<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
	import { resolve } from '$app/paths';
</script>

<svelte:head>
	<title>vtsls-macroforge - Macroforge Documentation</title>
	<meta name="description" content="VTSLS-based TypeScript language server with Macroforge plugin integration for the Zed editor." />
</svelte:head>

<h1>vtsls-macroforge</h1>

<p class="lead">
	A Zed extension that wraps <a href="https://github.com/yioneko/vtsls">VTSLS</a> (VS Code's TypeScript language server) with the <code>@macroforge/typescript-plugin</code> pre-configured for compile-time macros.
</p>

<Alert type="warning" title="Developer Installation Required">
	This extension is not yet in the Zed extension registry. You'll need to install it as a developer extension.
</Alert>

<h2 id="features">Features</h2>

<ul>
	<li>Full TypeScript language features via VTSLS</li>
	<li>Macro expansion at edit time</li>
	<li>Accurate error positions in original source</li>
	<li>Completions for macro-generated methods</li>
	<li>Automatic download of <code>@vtsls/language-server</code> and <code>@macroforge/typescript-plugin</code></li>
	<li>Platform-aware native binary installation</li>
</ul>

<h2 id="how-it-works">How It Works</h2>

<p>The extension:</p>

<ol>
	<li>Downloads <code>@vtsls/language-server</code> and <code>@macroforge/typescript-plugin</code> from npm</li>
	<li>Launches VTSLS as the language server</li>
	<li>Configures VTSLS with the macroforge plugin via <code>globalPlugins</code></li>
</ol>

<h2 id="installation">Installation</h2>

<h3>1. Clone the Repository</h3>

<CodeBlock code={`git clone https://github.com/macroforge-ts/zed-extensions.git
cd macroforge-ts/crates/extensions/vtsls-macroforge`} lang="bash" />

<h3>2. Install as Dev Extension in Zed</h3>

<p>
	In Zed, open the command palette and run <strong>zed: install dev extension</strong>, then select the <code>vtsls-macroforge</code> directory.
</p>

<p>Alternatively, symlink to your Zed extensions directory:</p>

<CodeBlock code={`# macOS
ln -s /path/to/macroforge-ts/crates/extensions/vtsls-macroforge ~/Library/Application\\ Support/Zed/extensions/installed/vtsls-macroforge

# Linux
ln -s /path/to/macroforge-ts/crates/extensions/vtsls-macroforge ~/.config/zed/extensions/installed/vtsls-macroforge`} lang="bash" />

<h3>3. Configure Zed Settings</h3>

<p>Add this to your <code>.zed/settings.json</code>:</p>

<CodeBlock code={`{
  "languages": {
    "TypeScript": {
      "language_servers": ["macroforge-ts", "!vtsls", "!typescript-language-server"]
    }
  }
}`} lang="json" filename=".zed/settings.json" />

<h2 id="supported-languages">Supported Languages</h2>

<table>
	<thead>
		<tr>
			<th>Language</th>
			<th>Supported</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td>TypeScript</td>
			<td>Yes</td>
		</tr>
		<tr>
			<td>TSX</td>
			<td>Yes</td>
		</tr>
		<tr>
			<td>JavaScript</td>
			<td>Yes</td>
		</tr>
	</tbody>
</table>

<h2 id="troubleshooting">Troubleshooting</h2>

<h3>Extension not loading</h3>

<p>
	Make sure you've restarted Zed after installing the extension. Check the Zed logs for any error messages.
</p>

<h3>Macros not expanding</h3>

<p>
	Ensure your project has the <code>macroforge</code> package installed. The extension handles the TypeScript plugin configuration automatically — you don't need to add it to your <code>tsconfig.json</code> when using this extension.
</p>
