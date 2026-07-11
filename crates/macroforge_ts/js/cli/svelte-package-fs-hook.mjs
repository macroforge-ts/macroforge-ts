/**
 * Module-customization resolve hook for the macroforge svelte-package wrapper.
 *
 * @sveltejs/package reads `.ts` source for its JS emit via `import * as fs from
 * 'node:fs'` and `fs.readFileSync`. That ESM namespace binding cannot be
 * monkeypatched in-process, so we redirect every `node:fs` import to a shim
 * module that re-exports real fs with an expanding `readFileSync`. The shim's
 * own `node:fs` import is passed through (keyed on parentURL) to avoid recursion.
 */
const SHIM = new URL('./svelte-package-fs-shim.mjs', import.meta.url).href;

export async function resolve(specifier, context, nextResolve) {
    if ((specifier === 'node:fs' || specifier === 'fs') && context.parentURL !== SHIM) {
        return { url: SHIM, shortCircuit: true };
    }
    return nextResolve(specifier, context);
}
