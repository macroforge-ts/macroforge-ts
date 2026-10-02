/**
 * fs shim for the macroforge svelte-package wrapper.
 *
 * Re-exports the real `node:fs` (the `export *` / `import *` here resolve to the
 * real builtin because the wrapper's resolve hook passes through imports whose parentURL
 * is this shim), overriding only `readFileSync` so that svelte-package's JS-emit
 * reads of macro-annotated `.ts`/`.svelte.ts` modules return expanded source.
 *
 * Expanded copies come from `globalThis.__macroforgeExpanded`, installed by
 * the wrapper entrypoint before svelte-package is imported, and are looked up
 * before the source is read: an expanded module's source is never needed.
 * Every other read, and every read for bytes, passes through untouched.
 */
export * from 'node:fs';
import * as real from 'node:fs';

export const readFileSync = function (path, options) {
    const encoding = typeof options === 'string' ? options : options?.encoding;
    const expanded = globalThis.__macroforgeExpanded;
    if (encoding && typeof expanded === 'function') {
        const source = expanded(path);
        if (source !== undefined) return source;
    }
    return real.readFileSync(path, options);
};

export default real.default ?? real;
