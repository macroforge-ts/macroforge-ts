<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import { siteConfig } from '$lib/config/site';
	import { base } from '$app/paths';

	const features = [
		{
			title: 'Built-in Macros',
			description: 'Debug, Clone, and Eq macros ready to use out of the box.',
			icon: 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z'
		},
		{
			title: 'Custom Macros in Rust',
			description: 'Write your own derive macros using the full power of Rust.',
			icon: 'M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4'
		},
		{
			title: 'IDE Integration',
			description: 'Full TypeScript plugin support with accurate error positions.',
			icon: 'M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z'
		},
		{
			title: 'Zero Runtime',
			description: 'All code is generated at compile time. No runtime overhead.',
			icon: 'M13 10V3L4 14h7v7l9-11h-7z'
		},
		{
			title: 'Source Maps',
			description: 'Accurate position mapping from expanded code back to source.',
			icon: 'M9 20l-5.447-2.724A1 1 0 013 16.382V5.618a1 1 0 011.447-.894L9 7m0 13l6-3m-6 3V7m6 10l4.553 2.276A1 1 0 0021 18.382V7.618a1 1 0 00-.553-.894L15 4m0 13V4m0 0L9 7'
		},
		{
			title: 'Powered by SWC',
			description: 'Lightning-fast parsing and code generation with SWC.',
			icon: 'M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z'
		}
	];

	const beforeCode = `import { Debug, Clone, Eq } from "macroforge";

/** @derive(Debug, Clone, Eq) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}`;

	const afterCode = `class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }

  toString(): string {
    return \`User { name: \${this.name}, age: \${this.age} }\`;
  }

  clone(): User {
    return new User(this.name, this.age);
  }

  equals(other: User): boolean {
    return this.name === other.name && this.age === other.age;
  }
}`;
</script>

<svelte:head>
	<title>{siteConfig.title}</title>
</svelte:head>

<!-- Hero Section -->
<section class="relative overflow-hidden bg-gradient-to-b from-secondary to-background pt-16 pb-20 sm:pt-24 sm:pb-28">
	<div class="container mx-auto px-4 sm:px-6 lg:px-8">
		<div class="text-center max-w-3xl mx-auto">
			<h1 class="text-4xl sm:text-5xl lg:text-6xl font-bold tracking-tight text-foreground">
				TypeScript Macros,
				<span class="text-primary">Powered by Rust</span>
			</h1>
			<p class="mt-6 text-lg sm:text-xl text-muted-foreground max-w-2xl mx-auto">
				Compile-time code generation with a Rust-like derive system.
				Eliminate boilerplate and generate type-safe code automatically.
			</p>

			<!-- Install Command -->
			<div class="mt-8 flex justify-center">
				<div class="inline-flex items-center gap-3 bg-card rounded-lg px-4 py-3 text-sm font-mono text-card-foreground border border-border">
					<span class="text-muted-foreground">$</span>
					<span>npm install macroforge</span>
					<button
						onclick={() => navigator.clipboard.writeText('npm install macroforge')}
						class="p-1 hover:bg-accent rounded transition-colors"
						aria-label="Copy install command"
					>
						<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-muted-foreground" viewBox="0 0 20 20" fill="currentColor">
							<path d="M8 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z" />
							<path d="M6 3a2 2 0 00-2 2v11a2 2 0 002 2h8a2 2 0 002-2V5a2 2 0 00-2-2 3 3 0 01-3 3H9a3 3 0 01-3-3z" />
						</svg>
					</button>
				</div>
			</div>

			<!-- CTA Buttons -->
			<div class="mt-8 flex flex-col sm:flex-row items-center justify-center gap-4">
				<a
					href="{base}/docs/getting-started"
					class="inline-flex items-center justify-center px-6 py-3 bg-primary hover:bg-primary/90 text-primary-foreground font-medium rounded-lg transition-colors"
				>
					Get Started
					<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 ml-2" viewBox="0 0 20 20" fill="currentColor">
						<path fill-rule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clip-rule="evenodd" />
					</svg>
				</a>
				<a
					href={siteConfig.links.github}
					target="_blank"
					rel="noopener noreferrer"
					class="inline-flex items-center justify-center px-6 py-3 border border-border text-foreground hover:bg-accent font-medium rounded-lg transition-colors"
				>
					<svg class="h-5 w-5 mr-2" fill="currentColor" viewBox="0 0 24 24">
						<path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd" />
					</svg>
					View on GitHub
				</a>
			</div>
		</div>
	</div>
</section>

<!-- Features Section -->
<section class="py-16 sm:py-24 bg-background">
	<div class="container mx-auto px-4 sm:px-6 lg:px-8">
		<div class="text-center max-w-2xl mx-auto mb-12">
			<h2 class="text-3xl font-bold text-foreground">
				Why Macroforge?
			</h2>
			<p class="mt-4 text-lg text-muted-foreground">
				Bring the power of Rust-style derive macros to TypeScript
			</p>
		</div>

		<div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-8">
			{#each features as feature}
				<div class="relative p-6 bg-card rounded-xl border border-border">
					<div class="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center mb-4">
						<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
							<path stroke-linecap="round" stroke-linejoin="round" d={feature.icon} />
						</svg>
					</div>
					<h3 class="text-lg font-semibold text-foreground mb-2">
						{feature.title}
					</h3>
					<p class="text-muted-foreground">
						{feature.description}
					</p>
				</div>
			{/each}
		</div>
	</div>
</section>

<!-- Code Example Section -->
<section class="py-16 sm:py-24 bg-muted">
	<div class="container mx-auto px-4 sm:px-6 lg:px-8">
		<div class="text-center max-w-2xl mx-auto mb-12">
			<h2 class="text-3xl font-bold text-foreground">
				See It In Action
			</h2>
			<p class="mt-4 text-lg text-muted-foreground">
				Write less code, get more functionality
			</p>
		</div>

		<div class="grid lg:grid-cols-2 gap-8 max-w-5xl mx-auto">
			<div>
				<div class="flex items-center gap-2 mb-3">
					<div class="w-3 h-3 rounded-full bg-warning"></div>
					<span class="text-sm font-medium text-muted-foreground">Before (Your Code)</span>
				</div>
				<CodeBlock code={beforeCode} lang="typescript" />
			</div>
			<div>
				<div class="flex items-center gap-2 mb-3">
					<div class="w-3 h-3 rounded-full bg-success"></div>
					<span class="text-sm font-medium text-muted-foreground">After (Generated)</span>
				</div>
				<CodeBlock code={afterCode} lang="typescript" />
			</div>
		</div>

		<div class="text-center mt-10">
			<a
				href="{base}/docs/getting-started"
				class="inline-flex items-center text-primary font-medium hover:underline"
			>
				Learn more about derive macros
				<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 ml-1" viewBox="0 0 20 20" fill="currentColor">
					<path fill-rule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clip-rule="evenodd" />
				</svg>
			</a>
		</div>
	</div>
</section>

<!-- Architecture Section -->
<section class="py-16 sm:py-24 bg-background">
	<div class="container mx-auto px-4 sm:px-6 lg:px-8">
		<div class="text-center max-w-2xl mx-auto mb-12">
			<h2 class="text-3xl font-bold text-foreground">
				Built on Solid Foundations
			</h2>
			<p class="mt-4 text-lg text-muted-foreground">
				Native performance with familiar tooling
			</p>
		</div>

		<div class="max-w-3xl mx-auto">
			<!-- Architecture Diagram -->
			<div class="bg-card rounded-xl border border-border p-8">
				<div class="space-y-4">
					<!-- Layer 1 -->
					<div class="text-center p-4 bg-muted rounded-lg border border-border">
						<span class="text-sm font-medium text-muted-foreground">Node.js / Vite / TypeScript Language Server</span>
					</div>

					<!-- Arrow -->
					<div class="flex justify-center">
						<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 text-muted-foreground" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3" />
						</svg>
					</div>

					<!-- Layer 2 -->
					<div class="text-center p-4 bg-primary/10 rounded-lg border border-primary/20">
						<span class="text-sm font-medium text-primary">NAPI-RS Bindings</span>
					</div>

					<!-- Arrow -->
					<div class="flex justify-center">
						<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 text-muted-foreground" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3" />
						</svg>
					</div>

					<!-- Layer 3 -->
					<div class="grid grid-cols-3 gap-4">
						<div class="text-center p-4 bg-warning/10 rounded-lg border border-warning/20">
							<span class="text-xs font-medium text-warning-foreground">ts_syn</span>
							<p class="text-xs text-warning-foreground/80 mt-1">Parsing</p>
						</div>
						<div class="text-center p-4 bg-warning/10 rounded-lg border border-warning/20">
							<span class="text-xs font-medium text-warning-foreground">ts_quote</span>
							<p class="text-xs text-warning-foreground/80 mt-1">Templating</p>
						</div>
						<div class="text-center p-4 bg-warning/10 rounded-lg border border-warning/20">
							<span class="text-xs font-medium text-warning-foreground">ts_macro_derive</span>
							<p class="text-xs text-warning-foreground/80 mt-1">Proc Macro</p>
						</div>
					</div>

					<!-- Arrow -->
					<div class="flex justify-center">
						<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 text-muted-foreground" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3" />
						</svg>
					</div>

					<!-- Layer 4 -->
					<div class="text-center p-4 bg-foreground rounded-lg">
						<span class="text-sm font-medium text-background">SWC Core</span>
						<p class="text-xs text-background/70 mt-1">TypeScript parsing & code generation</p>
					</div>
				</div>
			</div>
		</div>
	</div>
</section>

<!-- CTA Section -->
<section class="py-16 sm:py-24 bg-primary">
	<div class="container mx-auto px-4 sm:px-6 lg:px-8 text-center">
		<h2 class="text-3xl font-bold text-primary-foreground mb-4">
			Ready to Get Started?
		</h2>
		<p class="text-lg text-primary-foreground mb-8 max-w-2xl mx-auto">
			Install Macroforge and start generating code in minutes.
		</p>
		<a
			href="{base}/docs/getting-started"
			class="inline-flex items-center justify-center px-6 py-3 bg-background text-foreground font-medium rounded-lg hover:bg-background/90 transition-colors"
		>
			Read the Documentation
		</a>
	</div>
</section>
