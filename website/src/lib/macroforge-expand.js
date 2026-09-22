/** @type {typeof import('@macroforge/core').expandSync | null} */
let expandSync = null;

/**
 * Expands the macros in a documentation example at build time and drops its
 * `@macroforge/core` import line, which only adds noise to the rendered output.
 * @param {string} code - Example source
 * @param {string} filepath - Virtual file name the example expands as
 * @returns {Promise<string>} - Expanded source
 */
export async function expandForDisplay(code, filepath) {
    if (!expandSync) {
        expandSync = (await import('@macroforge/core')).expandSync;
    }
    const result = expandSync(code, filepath, {});
    return result.code.replace(
        /^import\s+\{[^}]+\}\s+from\s+['"]@macroforge\/core['"];\s*\n?/m,
        ''
    );
}
