<script lang="ts">
	import { toggleMode, mode } from 'mode-watcher';
	import Sun from 'lucide-svelte/icons/sun';
	import Moon from 'lucide-svelte/icons/moon';
</script>

<button
	onclick={toggleMode}
	class="p-2 rounded-lg text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
	aria-label={mode.current === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'}
>
	{#if mode.current === 'dark'}
		<Sun class="h-5 w-5" />
	{:else}
		<Moon class="h-5 w-5" />
	{/if}
</button>
