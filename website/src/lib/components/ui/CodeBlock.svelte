<script lang="ts">
    import CopyButton from "./CopyButton.svelte";
    import { slide } from "svelte/transition";

    interface Props {
        code: string;
        lang?: string;
        html?: string; // Pre-highlighted HTML from Shiki
        filename?: string;
        showLineNumbers?: boolean;
    }

    let {
        code,
        lang = "typescript",
        html,
        filename,
        showLineNumbers = false,
    }: Props = $props();

    // Only compute lines if not using pre-highlighted HTML
    const lines = $derived(html ? [] : code.trim().split("\n"));

    // Get language display name
    const langDisplay: Record<string, string> = {
        typescript: "TypeScript",
        ts: "TypeScript",
        javascript: "JavaScript",
        js: "JavaScript",
        rust: "Rust",
        bash: "Bash",
        shell: "Shell",
        json: "JSON",
        toml: "TOML",
        svelte: "Svelte",
        html: "HTML",
        css: "CSS",
        markdown: "Markdown",
        md: "Markdown",
        text: "Text",
    };
</script>

<div
    class="relative group my-4 rounded-lg overflow-hidden border border-border"
>
    {#if filename}
        <div
            class="flex items-center justify-between px-4 py-2 bg-muted border-b border-border"
        >
            <span class="text-xs font-medium text-muted-foreground"
                >{filename}</span
            >
            <CopyButton text={code} />
        </div>
    {:else}
        <div
            class="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity z-10"
        >
            <CopyButton text={code} />
        </div>
        {#if lang && langDisplay[lang]}
            <div class="absolute top-2 left-4 text-xs text-muted-foreground">
                {langDisplay[lang]}
            </div>
        {/if}
    {/if}
    {#if html}
        <!-- Render pre-highlighted Shiki HTML -->
        <div class="shiki-wrapper overflow-x-auto text-sm {filename ? '' : 'pt-8'}">
            {@html html}
        </div>
    {:else}
        <!-- Fallback to plain text rendering -->
        <pre
            class="overflow-x-auto p-4 bg-card text-sm {filename
                ? ''
                : 'pt-8'}"><code class="text-card-foreground font-mono"
                >{#each lines as line, i}{#if showLineNumbers}<span
                            class="inline-block w-8 text-right mr-4 text-muted-foreground select-none"
                            >{i + 1}</span
                        >{/if}<span>{line}</span
                    >{#if i < lines.length - 1}{"\n"}{/if}{/each}</code
            ></pre>
    {/if}
</div>
