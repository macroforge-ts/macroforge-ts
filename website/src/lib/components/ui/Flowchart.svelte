<script lang="ts">
	interface Step {
		title: string;
		description?: string;
	}

	interface Props {
		steps: Step[];
	}

	let { steps }: Props = $props();
</script>

<div class="my-8 flex flex-col items-center gap-0">
	{#each steps as step, i}
		<!-- Box -->
		<div
			class="w-full max-w-md border border-border rounded-lg bg-card p-4 text-center shadow-sm"
		>
			<div class="font-semibold text-foreground">{step.title}</div>
			{#if step.description}
				<div class="text-sm text-muted-foreground mt-1">{step.description}</div>
			{/if}
		</div>

		<!-- Arrow (except after last item) -->
		{#if i < steps.length - 1}
			<div class="flex flex-col items-center text-muted-foreground py-1">
				<div class="w-px h-4 bg-border"></div>
				<svg class="w-4 h-4" viewBox="0 0 16 16" fill="currentColor">
					<path d="M8 12L3 7h10L8 12z" />
				</svg>
			</div>
		{/if}
	{/each}
</div>
