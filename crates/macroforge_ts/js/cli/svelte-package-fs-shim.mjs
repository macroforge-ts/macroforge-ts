/**
 * fs shim for the macroforge svelte-package wrapper.
 *
 * Re-exports the real `node:fs` (the `export *` / `import *` here resolve to the
 * real builtin because the wrapper's resolve hook passes through imports whose parentURL
 * is this shim), overriding only `readFileSync` so that svelte-package's JS-emit
 * reads of macro-annotated `.ts`/`.svelte.ts` modules return expanded source,
 * and its reads of the project manifest leave out `#macroforge/config`.
 *
 * Both come from hooks the wrapper entrypoint installs before svelte-package
 * is imported: `globalThis.__macroforgeExpanded` and
 * `globalThis.__macroforgeManifest`. Each is consulted before the file is
 * read, so an expanded module's source is never needed. Every other read, and
 * every read for bytes, passes through untouched.
 */
export * from 'node:fs';
import * as real from 'node:fs';

export const readFileSync = function (path, options) {
    const encoding = typeof options === 'string' ? options : options?.encoding;
    if (encoding) {
        for (const hook of [globalThis.__macroforgeExpanded, globalThis.__macroforgeManifest]) {
            if (typeof hook !== 'function') continue;
            const served = hook(path);
            if (served !== undefined) return served;
        }
    }
    return real.readFileSync(path, options);
};

export default real.default ?? real;
