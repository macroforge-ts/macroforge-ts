<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import { base } from '$app/paths';
</script>

<style>
	.flow-diagram {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin: 2rem 0;
	}
	.flow-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		flex-wrap: wrap;
	}
	.flow-box {
		padding: 0.5rem 1rem;
		border-radius: 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
	}
	.flow-source {
		background-color: var(--primary);
		color: var(--primary-foreground);
	}
	.flow-plugin {
		background-color: var(--muted);
		color: var(--muted-foreground);
	}
	.flow-output {
		background-color: var(--card);
		color: var(--card-foreground);
		border: 1px solid var(--border);
	}
	.flow-arrow {
		color: var(--muted-foreground);
	}
	.flow-branch {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		margin-left: 2rem;
		padding-left: 1rem;
		border-left: 2px solid var(--border);
	}
</style>

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

<div class="flow-diagram">
	<div class="flow-row">
		<span class="flow-box flow-source">Your Code</span>
	</div>
	<div class="flow-branch">
		<div class="flow-row">
			<span class="flow-arrow">→</span>
			<span class="flow-box flow-plugin">TypeScript Plugin</span>
			<span class="flow-arrow">→</span>
			<span class="flow-box flow-output">IDE Feedback</span>
		</div>
		<div class="flow-row">
			<span class="flow-arrow">→</span>
			<span class="flow-box flow-plugin">Vite Plugin</span>
			<span class="flow-arrow">→</span>
			<span class="flow-box flow-output">Dev Server</span>
			<span class="flow-arrow">/</span>
			<span class="flow-box flow-output">Production Build</span>
		</div>
	</div>
</div>

<h2 id="next-steps">Detailed Guides</h2>

<ul>
	<li><a href="{base}/docs/integration/typescript-plugin">TypeScript Plugin setup</a></li>
	<li><a href="{base}/docs/integration/vite-plugin">Vite Plugin configuration</a></li>
	<li><a href="{base}/docs/integration/configuration">Configuration options</a></li>
</ul>
