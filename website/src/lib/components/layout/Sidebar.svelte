<script lang="ts">
	import { navigation, type NavSection } from '$lib/config/navigation';
	import { page } from '$app/state';
	import { base } from '$app/paths';

	const getHref = (href: string) => `${base}${href}`;
	const isActive = (href: string) => page.url.pathname === getHref(href);
	const isSectionActive = (section: NavSection) => section.items.some((item) => isActive(item.href));
</script>

<nav class="space-y-6" aria-label="Documentation">
	{#each navigation as section}
		<div>
			<h4
				class="text-sm font-semibold text-slate-900 dark:text-white mb-3"
				class:text-primary-600={isSectionActive(section)}
				class:dark:text-primary-400={isSectionActive(section)}
			>
				{section.title}
			</h4>
			<ul class="space-y-1">
				{#each section.items as item}
					<li>
						<a
							href={getHref(item.href)}
							class="block py-1.5 px-3 text-sm rounded-md transition-colors
								{isActive(item.href)
									? 'bg-primary-50 dark:bg-primary-950 text-primary-600 dark:text-primary-400 font-medium'
									: 'text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white hover:bg-slate-100 dark:hover:bg-slate-800'}"
							aria-current={isActive(item.href) ? 'page' : undefined}
						>
							{item.title}
						</a>
					</li>
				{/each}
			</ul>
		</div>
	{/each}
</nav>
