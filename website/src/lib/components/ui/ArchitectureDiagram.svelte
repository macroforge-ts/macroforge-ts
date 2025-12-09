<script lang="ts">
	interface Layer {
		title: string;
		items?: string[];
	}

	interface Props {
		layers: Layer[];
	}

	let { layers }: Props = $props();
</script>

<div class="my-8 flex flex-col gap-0">
	{#each layers as layer, i}
		<div
			class="border border-border bg-card p-4 text-center {i === 0 ? 'rounded-t-lg' : ''} {i === layers.length - 1 ? 'rounded-b-lg' : ''} {i > 0 ? 'border-t-0' : ''}"
		>
			<div class="font-semibold text-foreground">{layer.title}</div>
			{#if layer.items && layer.items.length > 0}
				<div class="flex flex-wrap justify-center gap-3 mt-3">
					{#each layer.items as item}
						<div class="px-3 py-1.5 bg-muted rounded text-sm text-muted-foreground font-mono">
							{item}
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/each}
</div>
