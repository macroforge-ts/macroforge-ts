<script lang="ts">
	import { navigation, resolvePath, type NavSection, type NavItem } from '$lib/config/navigation';
	import { page } from '$app/state';

	const getHref = (href: NavItem['href']) => resolvePath(href);
	const isActive = (href: NavItem['href']) => page.url.pathname === getHref(href);
	const isSectionActive = (section: NavSection) => section.items.some((item) => isActive(item.href));
</script>

<nav class="space-y-6" aria-label="Documentation">
	{#each navigation as section}
		<div>
			<h4
				class="text-sm font-semibold text-foreground mb-3"
				class:text-primary={isSectionActive(section)}
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
									? 'bg-primary/10 text-primary font-medium'
									: 'text-muted-foreground hover:text-foreground hover:bg-accent'}"
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
