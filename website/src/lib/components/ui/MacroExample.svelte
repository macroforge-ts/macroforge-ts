<script lang="ts">
	import CopyButton from './CopyButton.svelte';

	interface Props {
		before: string;
		after: string;
		beforeHtml?: string;
		afterHtml?: string;
		beforeLabel?: string;
		afterLabel?: string;
	}

	let {
		before,
		after,
		beforeHtml,
		afterHtml,
		beforeLabel = 'Before (Your Code)',
		afterLabel = 'After (Generated)'
	}: Props = $props();
</script>

<div class="space-y-6">
	<!-- Before block -->
	<div>
		<div class="flex items-center justify-between gap-2 px-4 py-2 bg-muted rounded-t-lg border border-b-0 border-border">
			<div class="flex items-center gap-2">
				<div class="w-3 h-3 rounded-full bg-warning"></div>
				<span class="text-sm font-medium text-muted-foreground">{beforeLabel}</span>
			</div>
			<CopyButton text={before} />
		</div>
		<div class="relative group rounded-b-lg overflow-hidden border border-border">
			{#if beforeHtml}
				<div class="shiki-wrapper overflow-x-auto text-sm">
					{@html beforeHtml}
				</div>
			{:else}
				<pre class="overflow-x-auto p-4 bg-card text-sm"><code class="text-card-foreground font-mono">{before.trim()}</code></pre>
			{/if}
		</div>
	</div>
	<!-- After block -->
	<div>
		<div class="flex items-center justify-between gap-2 px-4 py-2 bg-muted rounded-t-lg border border-b-0 border-border">
			<div class="flex items-center gap-2">
				<div class="w-3 h-3 rounded-full bg-success"></div>
				<span class="text-sm font-medium text-muted-foreground">{afterLabel}</span>
			</div>
			<CopyButton text={after} />
		</div>
		<div class="relative group rounded-b-lg overflow-hidden border border-border">
			{#if afterHtml}
				<div class="shiki-wrapper overflow-x-auto text-sm">
					{@html afterHtml}
				</div>
			{:else}
				<pre class="overflow-x-auto p-4 bg-card text-sm"><code class="text-card-foreground font-mono">{after.trim()}</code></pre>
			{/if}
		</div>
	</div>
</div>
