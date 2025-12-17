<script lang="ts">
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
</script>

<svelte:head>
	<title>Page Not Found - Macroforge</title>
</svelte:head>

<div class="min-h-[60vh] flex items-center justify-center px-4">
	<div class="text-center">
		<p class="text-6xl font-bold text-primary">
			{page.status}
		</p>
		<h1 class="mt-4 text-2xl font-semibold text-foreground">
			{page.error?.message || 'Page not found'}
		</h1>
		<p class="mt-2 text-muted-foreground">
			The page you're looking for doesn't exist or has been moved.
		</p>
		<div class="mt-6 flex items-center justify-center gap-4">
			<a
				href={resolve('/')}
				class="inline-flex items-center px-4 py-2 bg-primary hover:bg-primary/90 text-primary-foreground font-medium rounded-lg transition-colors"
			>
				Go Home
			</a>
			<a
				href={resolve('/docs/getting-started')}
				class="inline-flex items-center px-4 py-2 border border-border text-foreground hover:bg-accent font-medium rounded-lg transition-colors"
			>
				Documentation
			</a>
		</div>
	</div>
</div>
