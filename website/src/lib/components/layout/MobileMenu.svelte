<script lang="ts">
	import Sidebar from './Sidebar.svelte';

	interface Props {
		open: boolean;
		onClose: () => void;
	}

	let { open, onClose }: Props = $props();

	// Close on escape key
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape' && open) {
			onClose();
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
	<!-- Backdrop -->
	<button
		class="fixed inset-0 z-40 bg-slate-900/50 backdrop-blur-sm lg:hidden cursor-default"
		onclick={onClose}
		aria-label="Close navigation menu"
	></button>

	<!-- Sidebar panel -->
	<div
		class="fixed inset-y-0 left-0 z-50 w-72 bg-white dark:bg-surface-950 border-r border-slate-200 dark:border-slate-800 overflow-y-auto lg:hidden"
	>
		<div class="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-800">
			<span class="font-semibold text-slate-900 dark:text-white">Navigation</span>
			<button
				onclick={onClose}
				class="p-2 text-slate-500 hover:text-slate-900 dark:text-slate-400 dark:hover:text-white"
				aria-label="Close navigation menu"
			>
				<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
					<path
						fill-rule="evenodd"
						d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
						clip-rule="evenodd"
					/>
				</svg>
			</button>
		</div>
		<div class="p-4">
			<Sidebar />
		</div>
	</div>
{/if}
