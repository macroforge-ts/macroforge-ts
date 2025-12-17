<script lang="ts">
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
					href={siteConfig.links.github}
					target="_blank"
					rel="noopener noreferrer"
					class="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors"
				>
					GitHub
				</a>
			</nav>

			<!-- Right side actions -->
			<div class="flex items-center gap-2">
				<a
					href={siteConfig.links.github}
					target="_blank"
					rel="noopener noreferrer"
					class="hidden sm:flex p-2 text-muted-foreground hover:text-foreground"
					aria-label="GitHub repository"
				>
					<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
						<path
							fill-rule="evenodd"
							d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
							clip-rule="evenodd"
						/>
					</svg>
				</a>
				<ThemeToggle />
			</div>
		</div>
	</div>
</header>
