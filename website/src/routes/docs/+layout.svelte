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
	<div class="flex justify-center px-4 sm:px-6 lg:px-8">
		<article class="min-w-0 max-w-3xl w-full py-8">
			<!-- API Stability Warning -->
			<div class="mb-6 p-4 rounded-lg border border-yellow-500/30 bg-yellow-500/10">
				<div class="flex items-start gap-3">
					<svg class="w-5 h-5 text-yellow-500 mt-0.5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
					</svg>
					<div class="text-sm">
						<p class="font-medium text-yellow-500">API Stability Notice</p>
						<p class="text-muted-foreground mt-1">Macroforge is under active development. The API is not yet stable and may change between versions. Some documentation sections may be outdated.</p>
					</div>
				</div>
			</div>

			<div class="prose">
				{@render children()}
			</div>
			<PrevNext />
		</article>
	</div>
</div>
