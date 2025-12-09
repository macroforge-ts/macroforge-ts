<script lang="ts">
	import Sidebar from '$lib/components/layout/Sidebar.svelte';
	import PrevNext from '$lib/components/ui/PrevNext.svelte';

	let { children } = $props();
</script>

<div class="lg:grid lg:grid-cols-[256px_1fr]">
	<!-- Desktop Sidebar -->
	<aside class="hidden lg:block border-r border-border">
		<div class="sticky top-16 h-[calc(100vh-4rem)] overflow-y-auto py-8 px-6">
			<Sidebar />
		</div>
	</aside>

	<!-- Main Content -->
	<div class="container mx-auto px-4 sm:px-6 lg:px-8">
		<article class="min-w-0 max-w-3xl py-8">
			<div class="prose">
				{@render children()}
			</div>
			<PrevNext />
		</article>
	</div>
</div>
