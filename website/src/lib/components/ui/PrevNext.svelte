<script lang="ts">
	import { getPrevNext } from '$lib/config/navigation';
	import { page } from '$app/state';

	const { prev, next } = $derived(getPrevNext(page.url.pathname));
</script>

<nav class="mt-12 pt-6 border-t border-slate-200 dark:border-slate-700">
	<div class="flex justify-between">
		<div>
			{#if prev}
				<a
					href={prev.href}
					class="group flex flex-col text-left hover:no-underline"
				>
					<span class="text-xs text-slate-500 dark:text-slate-400 mb-1">Previous</span>
					<span class="text-sm font-medium text-slate-900 dark:text-white group-hover:text-primary-600 dark:group-hover:text-primary-400 flex items-center gap-1">
						<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
							<path fill-rule="evenodd" d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z" clip-rule="evenodd" />
						</svg>
						{prev.title}
					</span>
				</a>
			{/if}
		</div>
		<div>
			{#if next}
				<a
					href={next.href}
					class="group flex flex-col text-right hover:no-underline"
				>
					<span class="text-xs text-slate-500 dark:text-slate-400 mb-1">Next</span>
					<span class="text-sm font-medium text-slate-900 dark:text-white group-hover:text-primary-600 dark:group-hover:text-primary-400 flex items-center gap-1">
						{next.title}
						<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
							<path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd" />
						</svg>
					</span>
				</a>
			{/if}
		</div>
	</div>
</nav>
