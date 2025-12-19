<script lang="ts">
    import CodeBlock from "./CodeBlock.svelte";
    import { slide } from "svelte/transition";

    interface Props {
        code: string;
        expanded: string;
        codeHtml?: string;
        expandedHtml?: string;
    }

    let { code, expanded, codeHtml, expandedHtml }: Props = $props();

    let showExpanded = $state(false);
</script>

<div class="space-y-4">
    <div>
        <div class="flex items-center justify-between mb-3">
            <div class="flex items-center gap-2">
                <div class="w-3 h-3 rounded-full bg-warning"></div>
                <span class="text-sm font-medium text-muted-foreground">Source</span>
            </div>
            <button
                onclick={() => showExpanded = !showExpanded}
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
        <CodeBlock {code} lang="typescript" html={codeHtml} />
    </div>

    {#if showExpanded}
        <div transition:slide>
            <div class="flex items-center gap-2 mb-3">
                <div class="w-3 h-3 rounded-full bg-success"></div>
                <span class="text-sm font-medium text-muted-foreground">Generated</span>
            </div>
            <CodeBlock code={expanded} lang="typescript" html={expandedHtml} />
        </div>
    {/if}
</div>
