<script lang="ts">
	import CodeBlock from './CodeBlock.svelte';

	interface Props {
		before: string;
		after: string;
		beforeLabel?: string;
		afterLabel?: string;
	}

	let {
		before,
		after,
		beforeLabel = 'Before (Your Code)',
		afterLabel = 'After (Generated)'
	}: Props = $props();
</script>

<div class="space-y-6">
	<div>
		<div class="flex items-center gap-2 mb-3">
			<div class="w-3 h-3 rounded-full bg-warning"></div>
			<span class="text-sm font-medium text-muted-foreground">{beforeLabel}</span>
		</div>
		<CodeBlock code={before} lang="typescript" />
	</div>
	<div>
		<div class="flex items-center gap-2 mb-3">
			<div class="w-3 h-3 rounded-full bg-success"></div>
			<span class="text-sm font-medium text-muted-foreground">{afterLabel}</span>
		</div>
		<CodeBlock code={after} lang="typescript" />
	</div>
</div>
