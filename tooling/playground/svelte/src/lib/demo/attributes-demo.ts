// Mirror of `playground/vanilla/src/attributes-test.ts` for the SvelteKit
// playground. The two fixtures share the same set of annotations so the
// vanilla and svelte e2e suites stay in lockstep — fixing a bug in one
// playground's expansion uncovers it in the other.
//
// `playground/svelte/macroforge.config.ts` sets
// `cfg.features = ['playground']` and `cfg.target = 'web'`.

/** @cfg({ feature: 'playground' }) */
export function keptByPlayground(): string {
    return 'kept-by-feature';
}

/** @cfg({ feature: 'never-defined-anywhere' }) */
export function strippedByMissingFeature(): string {
    return 'should-be-stripped';
}

/** @cfg({ target: 'web' }) */
export function keptByWebTarget(): string {
    return 'kept-by-target';
}

/** @cfg({ target: 'node' }) */
export function strippedByNodeTarget(): string {
    return 'should-be-stripped';
}

/** @deprecated('use renderV2 instead', { since: '0.3.0' }) */
export function renderV1(): string {
    return 'render-v1';
}

/** @mustUse('connection handle must be closed') */
export function openConnection(): { close: () => void } {
    return { close() {} };
}

/** @nonExhaustive */
export type ServerStatus = 'green' | 'yellow' | 'red';

// External code constructing a `ServerStatus` must cast through the brand
// `@nonExhaustive` adds — that's the whole point of the brand.
export const exampleStatus = 'green' as ServerStatus;

// Consume the @mustUse return value so no diagnostic fires at build time.
const handle = openConnection();
handle.close();
