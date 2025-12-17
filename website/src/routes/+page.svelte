<script lang="ts">
    import MacroExample from "$lib/components/ui/MacroExample.svelte";
    import CopyButton from "$lib/components/ui/CopyButton.svelte";
    import { siteConfig } from "$lib/config/site";
    import { resolve } from "$app/paths";

    let { data } = $props();

    const packageManagers = [
        { id: "npm", label: "npm", command: "npm install macroforge" },
        { id: "pnpm", label: "pnpm", command: "pnpm add macroforge" },
        { id: "yarn", label: "yarn", command: "yarn add macroforge" },
        { id: "bun", label: "bun", command: "bun add macroforge" },
        { id: "deno", label: "deno", command: "deno add npm:macroforge" },
    ];

    let selectedPm = $state("npm");
    let currentCommand = $derived(
        packageManagers.find((pm) => pm.id === selectedPm)?.command ??
            packageManagers[0].command,
    );

    const features = [
        {
            title: "Zero Runtime Cost",
            description:
                "All code generation happens at build time. Ship only the code you need.",
            icon: "M13 10V3L4 14h7v7l9-11h-7z",
        },
        {
            title: "Type-Safe Generation",
            description:
                "Catch errors during compilation, not at runtime. Full TypeScript integration.",
            icon: "M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z",
        },
        {
            title: "Built-in Macros",
            description:
                "Debug, Clone, Serialize, and more ready to use out of the box.",
            icon: "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z",
        },
        {
            title: "IDE Integration",
            description:
                "Full TypeScript plugin support with accurate error positions.",
            icon: "M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z",
        },
        {
            title: "Source Maps",
            description:
                "Accurate position mapping from expanded code back to source.",
            icon: "M9 20l-5.447-2.724A1 1 0 013 16.382V5.618a1 1 0 011.447-.894L9 7m0 13l6-3m-6 3V7m6 10l4.553 2.276A1 1 0 0021 18.382V7.618a1 1 0 00-.553-.894L15 4m0 13V4m0 0L9 7",
        },
        {
            title: "Extensible",
            description:
                "Write custom macros in TypeScript or Rust to fit your needs.",
            icon: "M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4",
        },
    ];
</script>

<svelte:head>
    <title>{siteConfig.title}</title>
</svelte:head>

<!-- Hero Section -->
<section
    class="relative overflow-hidden bg-gradient-to-b from-secondary to-background pt-16 pb-20 sm:pt-24 sm:pb-28"
>
    <div class="container mx-auto px-4 sm:px-6 lg:px-8">
        <div class="text-center max-w-3xl mx-auto flex flex-col items-center">
            <h1
                class="text-4xl sm:text-5xl lg:text-6xl font-bold tracking-tight text-foreground"
            >
                TypeScript Macros,
                <span class="text-primary">Generated at Build Time</span>
            </h1>
            <p
                class="mt-6 text-lg sm:text-xl text-muted-foreground max-w-2xl mx-auto"
            >
                Push code generation to compile time for faster runtime and
                safer code. Eliminate boilerplate with zero runtime overhead.
            </p>

            <!-- Install Command -->
            <div class="mt-8 flex flex-col items-center">
                <!-- Package Manager Tabs -->
                <div
                    class="flex w-full justify-around gap-1 p-1 bg-muted rounded-t-lg border border-b-0 border-border"
                >
                    {#each packageManagers as pm}
                        <button
                            onclick={() => (selectedPm = pm.id)}
                            class="px-3 py-1.5 text-xs font-medium rounded-md transition-colors {selectedPm ===
                            pm.id
                                ? 'bg-background text-foreground shadow-sm'
                                : 'text-muted-foreground hover:text-foreground'}"
                        >
                            {pm.label}
                        </button>
                    {/each}
                </div>
                <!-- Command -->
                <div
                    class="flex justify-around w-full items-center gap-3 bg-card rounded-b-lg pr-4 pl-2 py-3 text-sm font-mono text-card-foreground border border-border"
                >
                    <div class="flex w-full justify-start">
                        <span class="text-muted-foreground mr-3">$</span>
                        <span>{currentCommand}</span>
                    </div>

                    <CopyButton text={currentCommand} />
                </div>
            </div>

            <!-- CTA Buttons -->
            <div
                class="mt-8 flex flex-col sm:flex-row items-center justify-center gap-4"
            >
                <a
                    href={resolve('/docs/getting-started')}
                    class="inline-flex items-center justify-center px-6 py-3 bg-primary hover:bg-primary/90 text-primary-foreground font-medium rounded-lg transition-colors"
                >
                    Get Started
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-4 w-4 ml-2"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                    >
                        <path
                            fill-rule="evenodd"
                            d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z"
                            clip-rule="evenodd"
                        />
                    </svg>
                </a>
                <a
                    href={siteConfig.links.github}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="inline-flex items-center justify-center px-6 py-3 border border-border text-foreground hover:bg-accent font-medium rounded-lg transition-colors"
                >
                    <svg
                        class="h-5 w-5 mr-2"
                        fill="currentColor"
                        viewBox="0 0 24 24"
                    >
                        <path
                            fill-rule="evenodd"
                            d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
                            clip-rule="evenodd"
                        />
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
            <h2 class="text-3xl font-bold text-foreground">Why Macroforge?</h2>
            <p class="mt-4 text-lg text-muted-foreground">
                Move work from runtime to build time for better performance and
                safety
            </p>
        </div>

        <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-8">
            {#each features as feature}
                <div
                    class="relative p-6 bg-card rounded-xl border border-border"
                >
                    <div
                        class="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center mb-4"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-5 w-5 text-primary"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                            stroke-width="2"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d={feature.icon}
                            />
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
            <h2 class="text-3xl font-bold text-foreground">See It In Action</h2>
            <p class="mt-4 text-lg text-muted-foreground">
                Write less code, get more functionality
            </p>
        </div>

        <div class="max-w-5xl mx-auto">
            <MacroExample
                before={data.heroExample.before}
                after={data.heroExample.after}
                beforeHtml={data.heroExample.beforeHtml}
                afterHtml={data.heroExample.afterHtml}
            />
        </div>

        <div class="text-center mt-10">
            <a
                href={resolve('/docs/getting-started')}
                class="inline-flex items-center text-primary font-medium hover:underline"
            >
                Learn more about derive macros
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-4 w-4 ml-1"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                >
                    <path
                        fill-rule="evenodd"
                        d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z"
                        clip-rule="evenodd"
                    />
                </svg>
            </a>
        </div>
    </div>
</section>

<!-- Build Time vs Runtime Section -->
<section class="py-16 sm:py-24 bg-background">
    <div class="container mx-auto px-4 sm:px-6 lg:px-8">
        <div class="text-center max-w-2xl mx-auto mb-12">
            <h2 class="text-3xl font-bold text-foreground">
                Build Time, Not Runtime
            </h2>
            <p class="mt-4 text-lg text-muted-foreground">
                Generate code once during build, run it everywhere with zero
                overhead
            </p>
        </div>

        <div class="max-w-4xl mx-auto">
            <div class="grid md:grid-cols-2 gap-8">
                <!-- Build Time -->
                <div class="bg-card rounded-xl border border-border p-6">
                    <div class="flex items-center gap-3 mb-4">
                        <div
                            class="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center"
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-5 w-5 text-primary"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                stroke-width="2"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"
                                />
                            </svg>
                        </div>
                        <h3 class="text-lg font-semibold text-foreground">
                            At Build Time
                        </h3>
                    </div>
                    <ul class="space-y-3 text-sm text-muted-foreground">
                        <li class="flex items-start gap-2">
                            <svg
                                class="h-5 w-5 text-primary flex-shrink-0 mt-0.5"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                stroke-width="2"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M5 13l4 4L19 7"
                                />
                            </svg>
                            <span>Parse decorators and type definitions</span>
                        </li>
                        <li class="flex items-start gap-2">
                            <svg
                                class="h-5 w-5 text-primary flex-shrink-0 mt-0.5"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                stroke-width="2"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M5 13l4 4L19 7"
                                />
                            </svg>
                            <span
                                >Generate serializers, validators, and utilities</span
                            >
                        </li>
                        <li class="flex items-start gap-2">
                            <svg
                                class="h-5 w-5 text-primary flex-shrink-0 mt-0.5"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                stroke-width="2"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M5 13l4 4L19 7"
                                />
                            </svg>
                            <span>Validate types and catch errors early</span>
                        </li>
                        <li class="flex items-start gap-2">
                            <svg
                                class="h-5 w-5 text-primary flex-shrink-0 mt-0.5"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                stroke-width="2"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M5 13l4 4L19 7"
                                />
                            </svg>
                            <span>Output optimized, tree-shakeable code</span>
                        </li>
                    </ul>
                </div>

                <!-- Runtime -->
                <div class="bg-card rounded-xl border border-border p-6">
                    <div class="flex items-center gap-3 mb-4">
                        <div
                            class="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center"
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-5 w-5 text-primary"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                stroke-width="2"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M13 10V3L4 14h7v7l9-11h-7z"
                                />
                            </svg>
                        </div>
                        <h3 class="text-lg font-semibold text-foreground">
                            At Runtime
                        </h3>
                    </div>
                    <ul class="space-y-3 text-sm text-muted-foreground">
                        <li class="flex items-start gap-2">
                            <svg
                                class="h-5 w-5 text-primary flex-shrink-0 mt-0.5"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                stroke-width="2"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M5 13l4 4L19 7"
                                />
                            </svg>
                            <span>No reflection or metadata overhead</span>
                        </li>
                        <li class="flex items-start gap-2">
                            <svg
                                class="h-5 w-5 text-primary flex-shrink-0 mt-0.5"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                stroke-width="2"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M5 13l4 4L19 7"
                                />
                            </svg>
                            <span>No decorator libraries to bundle</span>
                        </li>
                        <li class="flex items-start gap-2">
                            <svg
                                class="h-5 w-5 text-primary flex-shrink-0 mt-0.5"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                stroke-width="2"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M5 13l4 4L19 7"
                                />
                            </svg>
                            <span
                                >Plain JavaScript functions, fully inlineable</span
                            >
                        </li>
                        <li class="flex items-start gap-2">
                            <svg
                                class="h-5 w-5 text-primary flex-shrink-0 mt-0.5"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                stroke-width="2"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M5 13l4 4L19 7"
                                />
                            </svg>
                            <span
                                >Errors caught at build time, not in production</span
                            >
                        </li>
                    </ul>
                </div>
            </div>

            <!-- Comparison note -->
            <div class="mt-8 text-center">
                <p class="text-sm text-muted-foreground">
                    Unlike runtime decorator libraries that use reflection,
                    Macroforge generates plain TypeScript that compiles to
                    efficient JavaScript with no hidden costs.
                </p>
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
            href={resolve('/docs/getting-started')}
            class="inline-flex items-center justify-center px-6 py-3 bg-background text-foreground font-medium rounded-lg hover:bg-background/90 transition-colors"
        >
            Read the Documentation
        </a>
    </div>
</section>
