<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
</script>

<svelte:head>
	<title>Integration - Macroforge Documentation</title>
	<meta name="description" content="Integrate Macroforge with your TypeScript IDE and build tools." />
</svelte:head>

<h1>Integration</h1>

<p class="lead">
	Macroforge integrates with your development workflow through IDE plugins and build tool integration.
</p>

<h2 id="overview">Overview</h2>

<table>
	<thead>
		<tr>
			<th>Integration</th>
			<th>Purpose</th>
			<th>Package</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td>TypeScript Plugin</td>
			<td>IDE support (errors, completions)</td>
			<td><code>@macroforge/typescript-plugin</code></td>
		</tr>
		<tr>
			<td>Vite Plugin</td>
			<td>Build-time macro expansion</td>
			<td><code>@macroforge/vite-plugin</code></td>
		</tr>
	</tbody>
</table>

<h2 id="recommended-setup">Recommended Setup</h2>

<p>
	For the best development experience, use both integrations:
</p>

<ol>
	<li><strong>TypeScript Plugin</strong>: Provides real-time feedback in your IDE</li>
	<li><strong>Vite Plugin</strong>: Expands macros during development and production builds</li>
</ol>

<CodeBlock code={`# Install both plugins
npm install -D @macroforge/typescript-plugin @macroforge/vite-plugin`} lang="bash" />

<h2 id="how-they-work-together">How They Work Together</h2>

<CodeBlock code={`┌────────────────────────────────────────────────────────┐
│                   Development Flow                      │
├────────────────────────────────────────────────────────┤
│                                                         │
│  Your Code ──► TypeScript Plugin ──► IDE Feedback       │
│      │                                                  │
│      │                                                  │
│      └──────► Vite Plugin ────────► Dev Server          │
│                     │                                   │
│                     └─────────────► Production Build    │
│                                                         │
└────────────────────────────────────────────────────────┘`} lang="text" />

<h2 id="next-steps">Detailed Guides</h2>

<ul>
	<li><a href="/docs/integration/typescript-plugin">TypeScript Plugin setup</a></li>
	<li><a href="/docs/integration/vite-plugin">Vite Plugin configuration</a></li>
	<li><a href="/docs/integration/configuration">Configuration options</a></li>
</ul>
