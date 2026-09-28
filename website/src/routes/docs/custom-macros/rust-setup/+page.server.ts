import { getVersion } from '$lib/server/api-docs.ts';

export function load() {
    const version = getVersion('rust', 'macroforge_ts');
    if (!version) {
        throw new Error('api-data has no macroforge_ts version; run `pixi run docs:api`');
    }
    // Cargo's default caret requirement: every release up to the next minor.
    return { requirement: version.split('.').slice(0, 2).join('.') };
}
