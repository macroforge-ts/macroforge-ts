<script lang="ts">
	import Sidebar from '$lib/components/layout/Sidebar.svelte';
	import PrevNext from '$lib/components/ui/PrevNext.svelte';

	let { children } = $props();
</script>

<div class="container mx-auto px-4 sm:px-6 lg:px-8">
	<div class="lg:grid lg:grid-cols-[256px_1fr] lg:gap-8 py-8">
		<!-- Desktop Sidebar -->
		<aside class="hidden lg:block">
			<div class="sticky top-24">
				<Sidebar />
			</div>
		</aside>

		<!-- Main Content -->
		<article class="min-w-0 max-w-3xl">
			<div class="prose">
				{@render children()}
			</div>
			<PrevNext />
		</article>
	</div>
</div>
