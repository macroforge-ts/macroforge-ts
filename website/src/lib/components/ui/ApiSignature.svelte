<script lang="ts">
	import CodeBlock from './CodeBlock.svelte';

	interface ApiParam {
		name: string;
		type: string;
		description: string;
	}

	interface Props {
		/** The function/struct signature from doc comments */
		signature?: string;
		/** Language for syntax highlighting */
		lang?: 'rust' | 'typescript';
		/** Parameters from doc comments */
		params?: ApiParam[];
		/** Return value description */
		returns?: { type: string; description: string };
		/** Show the signature as a code block */
		showSignature?: boolean;
		/** Show parameters as a table */
		showParams?: boolean;
		/** Show return value */
		showReturns?: boolean;
	}

	let {
		signature,
		lang = 'typescript',
		params,
		returns,
		showSignature = true,
		showParams = true,
		showReturns = true
	}: Props = $props();
</script>

{#if showSignature && signature}
	<CodeBlock code={signature} {lang} />
{/if}

{#if showParams && params && params.length > 0}
	<h3>Parameters</h3>
	<table>
		<thead>
			<tr>
				<th>Parameter</th>
				<th>Type</th>
				<th>Description</th>
			</tr>
		</thead>
		<tbody>
			{#each params as param}
				<tr>
					<td><code>{param.name}</code></td>
					<td><code>{param.type || 'any'}</code></td>
					<td>{param.description}</td>
				</tr>
			{/each}
		</tbody>
	</table>
{/if}

{#if showReturns && returns}
	<h3>Returns</h3>
	{#if returns.type}
		<p><code>{returns.type}</code></p>
	{/if}
	<p>{returns.description}</p>
{/if}
