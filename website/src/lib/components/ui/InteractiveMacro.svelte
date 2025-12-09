<script lang="ts">
    import CodeBlock from "./CodeBlock.svelte";
    import { base } from "$app/paths";
    import { slide } from "svelte/transition";

    interface Props {
        code: string;
        expanded?: string;
    }

    let { code, expanded }: Props = $props();

    let showExpanded = $state(false);
    let fetchedCode = $state<string | null>(null);
    let isLoading = $state(false);
    let error = $state<string | null>(null);

    // Use prop if provided, otherwise use fetched result
    let expandedCode = $derived(expanded ?? fetchedCode);

    async function fetchExpanded() {
        if (expandedCode !== null) return;

        isLoading = true;
        error = null;

        try {
            const response = await fetch(`${base}/api/expand`, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ code }),
            });

            const result = await response.json();

            if (!response.ok) {
                throw new Error(result.error || "Expansion failed");
            }

            fetchedCode = result.code;
        } catch (e) {
            error = e instanceof Error ? e.message : "Expansion failed";
        } finally {
            isLoading = false;
        }
    }

    function toggle() {
        showExpanded = !showExpanded;
        if (showExpanded && expandedCode === null) {
            fetchExpanded();
        }
    }
</script>

<div class="space-y-4">
    <div>
        <div class="flex items-center justify-between mb-3">
            <div class="flex items-center gap-2">
                <div class="w-3 h-3 rounded-full bg-warning"></div>
                <span class="text-sm font-medium text-muted-foreground"
                    >Source</span
                >
            </div>
            <button
                onclick={toggle}
                class="inline-flex items-center gap-2 px-3 py-1.5 text-sm font-medium rounded-md
					bg-primary text-primary-foreground hover:bg-primary/90
					transition-colors"
            >
                {#if showExpanded}
                    <svg
                        class="w-4 h-4"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                    >
                        <path d="M18 15l-6-6-6 6" />
                    </svg>
                    Hide
                {:else}
                    <svg
                        class="w-4 h-4"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                    >
                        <path d="M13 7l5 5-5 5M6 7l5 5-5 5" />
                    </svg>
                    Expand
                {/if}
            </button>
        </div>
        <CodeBlock {code} lang="typescript" />
    </div>

    {#if showExpanded}
        <div transition:slide>
            {#if isLoading}
                <div class="flex items-center gap-2 text-muted-foreground">
                    <svg
                        class="w-4 h-4 animate-spin"
                        viewBox="0 0 24 24"
                        fill="none"
                    >
                        <circle
                            class="opacity-25"
                            cx="12"
                            cy="12"
                            r="10"
                            stroke="currentColor"
                            stroke-width="4"
                        ></circle>
                        <path
                            class="opacity-75"
                            fill="currentColor"
                            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                        ></path>
                    </svg>
                    <span class="text-sm">Expanding...</span>
                </div>
            {:else if error}
                <div
                    class="p-3 rounded-md bg-destructive/10 border border-destructive/20 text-destructive text-sm"
                >
                    {error}
                </div>
            {:else if expandedCode}
                <div>
                    <div class="flex items-center gap-2 mb-3">
                        <div class="w-3 h-3 rounded-full bg-success"></div>
                        <span class="text-sm font-medium text-muted-foreground"
                            >Generated</span
                        >
                    </div>
                    <CodeBlock code={expandedCode} lang="typescript" />
                </div>
            {/if}
        </div>
    {/if}
</div>
