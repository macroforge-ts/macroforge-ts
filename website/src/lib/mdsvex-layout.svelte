<script module>
	// Export components for use in .svx files via the layout
	export { default as CodeBlock } from '$lib/components/ui/CodeBlock.svelte';
	export { default as MacroExample } from '$lib/components/ui/MacroExample.svelte';
	export { default as InteractiveMacro } from '$lib/components/ui/InteractiveMacro.svelte';
	export { default as Alert } from '$lib/components/ui/Alert.svelte';
</script>

<script>
	let { children } = $props();
</script>

<div class="mdsvex-content">
	{@render children()}
</div>
