/**
 * fs shim for the macroforge svelte-package wrapper.
 *
 * Re-exports the real `node:fs` (the `export *` / `import *` here resolve to the
 * real builtin because the resolve hook passes through imports whose parentURL
 * is this shim), overriding only `readFileSync` so that svelte-package's JS-emit
 * reads of macro-annotated `.ts`/`.svelte.ts` modules return expanded source.
 *
 * Expansion is delegated to `globalThis.__macroforgeExpand`, installed by the
 * wrapper entrypoint before svelte-package is imported. Every non-macro read
 * passes through untouched.
 */
export * from 'node:fs';
import * as real from 'node:fs';

export const readFileSync = function (path, options) {
    const content = real.readFileSync(path, options);
    const expand = globalThis.__macroforgeExpand;
    if (typeof content === 'string' && typeof expand === 'function') {
        return expand(path, content);
    }
    return content;
};

export default real.default ?? real;
