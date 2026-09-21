<script lang="ts">
	import GitLabIcon from '$lib/components/ui/GitLabIcon.svelte';
	import ThemeToggle from '$lib/components/ui/ThemeToggle.svelte';
	import Logo from '$lib/components/ui/Logo.svelte';
	import { siteConfig } from '$lib/config/site';
	import { page } from '$app/state';
	import { resolve } from '$app/paths';

	interface Props {
		onMenuClick?: () => void;
	}

	let { onMenuClick }: Props = $props();

	const isDocsPage = $derived(page.url.pathname.startsWith(resolve('/docs')));
</script>

<header
	class="sticky top-0 z-50 w-full border-b border-border bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60"
>
	<div class="container mx-auto px-4 sm:px-6 lg:px-8">
		<div class="flex h-16 items-center justify-between">
			<!-- Logo and Mobile Menu Button -->
			<div class="flex items-center gap-4">
				{#if isDocsPage}
					<button
						onclick={() => onMenuClick?.()}
						class="lg:hidden p-2 -ml-2 text-muted-foreground hover:text-foreground"
						aria-label="Open navigation menu"
					>
						<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
						</svg>
					</button>
				{/if}

				<a href={resolve('/')}>
					<Logo />
				</a>
			</div>

			<!-- Desktop Navigation -->
			<nav class="hidden md:flex items-center gap-6">
				<a
					href={resolve('/docs/getting-started')}
					class="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors"
					class:text-primary={isDocsPage}
				>
					Documentation
				</a>
				<a
					href={siteConfig.links.gitlab}
					target="_blank"
					rel="noopener noreferrer"
					class="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors"
				>
					GitLab
				</a>
			</nav>

			<!-- Right side actions -->
			<div class="flex items-center gap-2">
				<a
					href={siteConfig.links.gitlab}
					target="_blank"
					rel="noopener noreferrer"
					class="hidden sm:flex p-2 text-muted-foreground hover:text-foreground"
					aria-label="GitLab repository"
				>
					<GitLabIcon />
				</a>
				<ThemeToggle />
			</div>
		</div>
	</div>
</header>
