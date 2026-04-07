<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		lang?: string;
		html?: string; // Pre-highlighted HTML from preprocessor
		children?: Snippet;
	}

	let { lang, html, children }: Props = $props();
</script>

{#if html}
	{@html html}
{:else if children}
	<code class="inline-code">{@render children()}</code>
{/if}
