<script lang="ts">
	import CodeBlock from './CodeBlock.svelte';
	import { base } from '$app/paths';

	interface Props {
		code: string;
		expanded?: string;
	}

	let { code, expanded: initialExpanded }: Props = $props();

	let expanded = $state(initialExpanded ?? '');
	let isExpanding = $state(false);
	let error = $state('');
	let showExpanded = $state(!!initialExpanded);

	async function expand() {
		if (expanded && showExpanded) {
			showExpanded = false;
			return;
		}

		if (expanded) {
			showExpanded = true;
			return;
		}

		isExpanding = true;
		error = '';

		try {
			const response = await fetch(`${base}/api/expand`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ code })
			});

			const result = await response.json();

			if (!response.ok) {
				throw new Error(result.error || 'Expansion failed');
			}

			expanded = result.code;
			showExpanded = true;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Expansion failed';
		} finally {
			isExpanding = false;
		}
	}
</script>

<div class="space-y-4">
	<div>
		<div class="flex items-center justify-between mb-3">
			<div class="flex items-center gap-2">
				<div class="w-3 h-3 rounded-full bg-warning"></div>
				<span class="text-sm font-medium text-muted-foreground">Source</span>
			</div>
			<button
				onclick={expand}
				disabled={isExpanding}
				class="inline-flex items-center gap-2 px-3 py-1.5 text-sm font-medium rounded-md
					bg-primary text-primary-foreground hover:bg-primary/90
					disabled:opacity-50 disabled:cursor-not-allowed
					transition-colors"
			>
				{#if isExpanding}
					<svg class="w-4 h-4 animate-spin" viewBox="0 0 24 24" fill="none">
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
						<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
					</svg>
					Expanding...
				{:else if showExpanded}
					<svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M18 15l-6-6-6 6" />
					</svg>
					Hide
				{:else}
					<svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M13 7l5 5-5 5M6 7l5 5-5 5" />
					</svg>
					Expand
				{/if}
			</button>
		</div>
		<CodeBlock {code} lang="typescript" />
	</div>

	{#if error}
		<div class="p-3 rounded-md bg-destructive/10 border border-destructive/20 text-destructive text-sm">
			{error}
		</div>
	{/if}

	{#if showExpanded && expanded}
		<div>
			<div class="flex items-center gap-2 mb-3">
				<div class="w-3 h-3 rounded-full bg-success"></div>
				<span class="text-sm font-medium text-muted-foreground">Generated</span>
			</div>
			<CodeBlock code={expanded} lang="typescript" />
		</div>
	{/if}
</div>
