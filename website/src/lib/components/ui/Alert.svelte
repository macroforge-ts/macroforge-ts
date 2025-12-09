<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		type?: 'info' | 'warning' | 'tip' | 'note';
		title?: string;
		children: Snippet;
	}

	let { type = 'info', title, children }: Props = $props();

	const styles = {
		info: {
			container: 'bg-info/10 border-info/20',
			icon: 'text-info',
			title: 'text-info-foreground',
			text: 'text-info-foreground/90'
		},
		warning: {
			container: 'bg-warning/10 border-warning/20',
			icon: 'text-warning',
			title: 'text-warning-foreground',
			text: 'text-warning-foreground/90'
		},
		tip: {
			container: 'bg-success/10 border-success/20',
			icon: 'text-success',
			title: 'text-success-foreground',
			text: 'text-success-foreground/90'
		},
		note: {
			container: 'bg-muted border-border',
			icon: 'text-muted-foreground',
			title: 'text-foreground',
			text: 'text-muted-foreground'
		}
	};

	const icons = {
		info: 'M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z',
		warning: 'M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z',
		tip: 'M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z',
		note: 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z'
	};

	const defaultTitles = {
		info: 'Info',
		warning: 'Warning',
		tip: 'Tip',
		note: 'Note'
	};
</script>

<div class="rounded-lg border p-4 my-4 {styles[type].container}">
	<div class="flex gap-3">
	<div class="flex">
	<svg
		xmlns="http://www.w3.org/2000/svg"
		class="h-5 w-5 shrink-0 {styles[type].icon}"
		fill="none"
		viewBox="0 0 24 24"
		stroke="currentColor"
		stroke-width="2"
	>
		<path stroke-linecap="round" stroke-linejoin="round" d={icons[type]} />
	</svg>
	{#if title || defaultTitles[type]}
		<h3 class="text-sm font-medium {styles[type].title}">
			{title ?? defaultTitles[type]}
		</h3>
	{/if}
	</div>

		<div class="-mt-0.5">

			<div class="text-sm {styles[type].text}" class:mt-1={title || defaultTitles[type]}>
				{@render children()}
			</div>
		</div>
	</div>
</div>
