<script lang="ts">
	import { toggleMode, mode } from 'mode-watcher';
	import Sun from 'lucide-svelte/icons/sun';
	import Moon from 'lucide-svelte/icons/moon';
</script>

<button
	onclick={toggleMode}
	class="p-2 rounded-lg text-slate-500 hover:text-slate-900 dark:text-slate-400 dark:hover:text-white hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors"
	aria-label={mode.current === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'}
>
	{#if mode.current === 'dark'}
		<Sun class="h-5 w-5" />
	{:else}
		<Moon class="h-5 w-5" />
	{/if}
</button>
