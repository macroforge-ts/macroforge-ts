<script lang="ts">
	import { page } from '$app/state';
	import { base } from '$app/paths';
</script>

<svelte:head>
	<title>Page Not Found - Macroforge</title>
</svelte:head>

<div class="min-h-[60vh] flex items-center justify-center px-4">
	<div class="text-center">
		<p class="text-6xl font-bold text-primary-600 dark:text-primary-400">
			{page.status}
		</p>
		<h1 class="mt-4 text-2xl font-semibold text-slate-900 dark:text-white">
			{page.error?.message || 'Page not found'}
		</h1>
		<p class="mt-2 text-slate-600 dark:text-slate-400">
			The page you're looking for doesn't exist or has been moved.
		</p>
		<div class="mt-6 flex items-center justify-center gap-4">
			<a
				href="{base}/"
				class="inline-flex items-center px-4 py-2 bg-primary-600 hover:bg-primary-700 text-white font-medium rounded-lg transition-colors"
			>
				Go Home
			</a>
			<a
				href="{base}/docs/getting-started"
				class="inline-flex items-center px-4 py-2 border border-slate-300 dark:border-slate-600 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-800 font-medium rounded-lg transition-colors"
			>
				Documentation
			</a>
		</div>
	</div>
</div>
