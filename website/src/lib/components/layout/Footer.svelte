<script lang="ts">
	import { siteConfig } from '$lib/config/site';
</script>

<footer class="border-t border-slate-200 dark:border-slate-800 bg-slate-50 dark:bg-surface-900">
	<div class="container mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<div class="flex flex-col md:flex-row items-center justify-between gap-4">
			<div class="flex items-center gap-2 text-sm text-slate-500 dark:text-slate-400">
				<span>&copy; {new Date().getFullYear()} {siteConfig.name}.</span>
				<span>Built with SvelteKit.</span>
			</div>

			<div class="flex items-center gap-6">
				<a
					href={siteConfig.links.github}
					target="_blank"
					rel="noopener noreferrer"
					class="text-sm text-slate-500 hover:text-slate-900 dark:text-slate-400 dark:hover:text-white transition-colors"
				>
					GitHub
				</a>
				<a
					href={siteConfig.links.npm}
					target="_blank"
					rel="noopener noreferrer"
					class="text-sm text-slate-500 hover:text-slate-900 dark:text-slate-400 dark:hover:text-white transition-colors"
				>
					npm
				</a>
				<a
					href="{siteConfig.links.github}/blob/main/LICENSE"
					target="_blank"
					rel="noopener noreferrer"
					class="text-sm text-slate-500 hover:text-slate-900 dark:text-slate-400 dark:hover:text-white transition-colors"
				>
					MIT License
				</a>
			</div>
		</div>
	</div>
</footer>
