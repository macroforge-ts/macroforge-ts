<script lang="ts">
	interface Branch {
		plugin: string;
		pluginDesc?: string;
		outputs: { label: string; desc?: string }[];
	}

	interface Props {
		source: string;
		sourceDesc?: string;
		branches: Branch[];
	}

	let { source, sourceDesc, branches }: Props = $props();
</script>

<div class="my-8 w-full max-w-2xl mx-auto">
	<!-- Source box at top -->
	<div class="flex justify-center">
		<div class="border-2 border-primary bg-primary/10 rounded-lg px-6 py-3 text-center">
			<div class="font-semibold text-primary">{source}</div>
			{#if sourceDesc}
				<div class="text-sm text-muted-foreground mt-1">{sourceDesc}</div>
			{/if}
		</div>
	</div>

	<!-- Connector line down from source -->
	<div class="flex justify-center">
		<div class="w-px h-6 bg-border"></div>
	</div>

	<!-- Horizontal line spanning branches -->
	<div class="relative flex justify-center">
		<div
			class="absolute top-0 h-px bg-border"
			style="width: {branches.length > 1 ? '50%' : '0'}; left: 25%;"
		></div>
	</div>

	<!-- Branches -->
	<div class="grid gap-6" style="grid-template-columns: repeat({branches.length}, 1fr);">
		{#each branches as branch, i}
			<div class="flex flex-col items-center">
				<!-- Vertical connector from horizontal line -->
				<div class="w-px h-6 bg-border"></div>

				<!-- Arrow down -->
				<svg class="w-4 h-4 text-muted-foreground -mt-1" viewBox="0 0 16 16" fill="currentColor">
					<path d="M8 12L4 7h8L8 12z" />
				</svg>

				<!-- Plugin box -->
				<div class="border border-border bg-card rounded-lg px-4 py-3 text-center w-full mt-1">
					<div class="font-medium text-foreground">{branch.plugin}</div>
					{#if branch.pluginDesc}
						<div class="text-xs text-muted-foreground mt-1">{branch.pluginDesc}</div>
					{/if}
				</div>

				<!-- Connector to outputs -->
				<div class="w-px h-4 bg-border"></div>
				<svg class="w-4 h-4 text-muted-foreground -mt-1" viewBox="0 0 16 16" fill="currentColor">
					<path d="M8 12L4 7h8L8 12z" />
				</svg>

				<!-- Output boxes -->
				<div class="flex flex-wrap justify-center gap-2 mt-1">
					{#each branch.outputs as output}
						<div
							class="border border-success/30 bg-success/10 rounded-md px-3 py-2 text-center"
						>
							<div class="text-sm font-medium text-success-foreground">{output.label}</div>
							{#if output.desc}
								<div class="text-xs text-muted-foreground mt-0.5">{output.desc}</div>
							{/if}
						</div>
					{/each}
				</div>
			</div>
		{/each}
	</div>
</div>
