<script lang="ts">
import { collectAttributesDemo } from '$lib/demo/attributes-demo-consumer';

const results = collectAttributesDemo();

// Expose results on globalThis so Playwright can inspect them after navigation.
(globalThis as Record<string, unknown>).attributesResults = results;
</script>

<svelte:head>
    <title>Macroforge attribute macros — Svelte playground</title>
</svelte:head>

<h1>Attribute macros</h1>
<p>
    Each row reflects an annotation on a declaration in
    <code>$lib/demo/attributes-demo.ts</code>. <code>(stripped)</code> means
    the export was removed by <code>@cfg</code>; a value means the function
    survived.
</p>

<div data-testid="attributes-results">
    <div>
        <strong>@cfg feature kept:</strong>
        <code data-testid="attr-kept-feature">{results.keptByFeature ?? '(stripped)'}</code>
    </div>
    <div>
        <strong>@cfg feature stripped:</strong>
        <code data-testid="attr-stripped-feature">{results.strippedByFeature ?? '(stripped)'}</code>
    </div>
    <div>
        <strong>@cfg target kept:</strong>
        <code data-testid="attr-kept-target">{results.keptByTarget ?? '(stripped)'}</code>
    </div>
    <div>
        <strong>@cfg target stripped:</strong>
        <code data-testid="attr-stripped-target">{results.strippedByTarget ?? '(stripped)'}</code>
    </div>
    <div>
        <strong>@deprecated call result:</strong>
        <code data-testid="attr-deprecated-call">{results.deprecatedCall}</code>
    </div>
    <div>
        <strong>@nonExhaustive value:</strong>
        <code data-testid="attr-non-exhaustive">{results.nonExhaustiveValue}</code>
    </div>
</div>
