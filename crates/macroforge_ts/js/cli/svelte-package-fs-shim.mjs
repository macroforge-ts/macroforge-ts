/**
 * fs shim for the macroforge svelte-package wrapper.
 *
 * Re-exports the real `node:fs` (the `export *` / `import *` here resolve to the
 * real builtin because the wrapper's resolve hook passes through imports whose parentURL
 * is this shim), overriding `readFileSync` and `copyFileSync`: svelte-package
 * reads the `.ts` modules it compiles and copies the `.tsx`, `.mts` and `.cts`
 * ones, and each of a macro-annotated module returns or writes its expanded
 * source, while its reads of the project manifest leave out `#macroforge/config`.
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

export const copyFileSync = function (source, destination, mode) {
    const expanded = globalThis.__macroforgeExpanded;
    const text = typeof expanded === 'function' ? expanded(source) : undefined;
    if (text === undefined) return real.copyFileSync(source, destination, mode);
    real.writeFileSync(destination, text);
};

// A default import (`import fs from 'node:fs'`) reads the same overrides, and
// everything else from the real module.
export default Object.assign(Object.create(real.default ?? real), {
    readFileSync,
    copyFileSync
});
