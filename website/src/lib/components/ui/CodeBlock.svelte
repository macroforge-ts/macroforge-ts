<script lang="ts">
	import CopyButton from './CopyButton.svelte';

	interface Props {
		code: string;
		lang?: string;
		filename?: string;
		showLineNumbers?: boolean;
	}

	let { code, lang = 'typescript', filename, showLineNumbers = false }: Props = $props();

	// Simple syntax highlighting using CSS classes
	// For production, you'd integrate Shiki here
	const lines = $derived(code.trim().split('\n'));

	// Get language display name
	const langDisplay: Record<string, string> = {
		typescript: 'TypeScript',
		ts: 'TypeScript',
		javascript: 'JavaScript',
		js: 'JavaScript',
		rust: 'Rust',
		bash: 'Bash',
		shell: 'Shell',
		json: 'JSON',
		toml: 'TOML'
	};
</script>

<div class="relative group my-4 rounded-lg overflow-hidden border border-slate-200 dark:border-slate-700">
	{#if filename}
		<div
			class="flex items-center justify-between px-4 py-2 bg-slate-100 dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700"
		>
			<span class="text-xs font-medium text-slate-600 dark:text-slate-400">{filename}</span>
			<CopyButton text={code} />
		</div>
	{:else}
		<div class="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity z-10">
			<CopyButton text={code} />
		</div>
		{#if lang && langDisplay[lang]}
			<div class="absolute top-2 left-4 text-xs text-slate-500 dark:text-slate-500">
				{langDisplay[lang]}
			</div>
		{/if}
	{/if}
	<pre
		class="overflow-x-auto p-4 bg-slate-900 dark:bg-slate-950 text-sm {filename ? '' : 'pt-8'}"
	><code class="text-slate-100 font-mono">{#each lines as line, i}{#if showLineNumbers}<span class="inline-block w-8 text-right mr-4 text-slate-500 select-none">{i + 1}</span>{/if}<span>{line}</span>{#if i < lines.length - 1}
{/if}{/each}</code></pre>
</div>
