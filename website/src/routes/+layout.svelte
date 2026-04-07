<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import Header from '$lib/components/layout/Header.svelte';
	import Footer from '$lib/components/layout/Footer.svelte';
	import MobileMenu from '$lib/components/layout/MobileMenu.svelte';
	import favicon from '$lib/assets/favicon.svg';
	import { siteConfig } from '$lib/config/site';

	let { children } = $props();
	let mobileMenuOpen = $state(false);
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link href="https://fonts.googleapis.com/css2?family=Geist:wght@400;500;600;700&family=Geist+Mono:wght@400;500&display=swap" rel="stylesheet" />
	<meta name="description" content={siteConfig.description} />
	<meta property="og:title" content={siteConfig.title} />
	<meta property="og:description" content={siteConfig.description} />
	<meta property="og:type" content="website" />
	<meta name="twitter:card" content="summary_large_image" />
</svelte:head>

<ModeWatcher />

<div class="min-h-screen flex flex-col">
	<Header onMenuClick={() => (mobileMenuOpen = true)} />
	<MobileMenu open={mobileMenuOpen} onClose={() => (mobileMenuOpen = false)} />

	<main class="flex-1">
		{@render children()}
	</main>

	<Footer />
</div>
