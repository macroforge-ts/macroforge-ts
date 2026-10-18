/**
 * What a file's name says about the module it holds, as TypeScript reads it:
 * extensions are matched as written, without folding case. The engine's
 * `source_path` module reads names the same way.
 *
 * @module
 */

/** The extensions of a TypeScript module, a declaration file among them. */
export const TYPESCRIPT_EXTENSIONS: readonly string[] = ['.ts', '.tsx', '.mts', '.cts'];

/** The extensions of a JavaScript module. */
export const JAVASCRIPT_EXTENSIONS: readonly string[] = ['.js', '.jsx', '.mjs', '.cjs'];

/** The endings of a declaration file. */
export const DECLARATION_ENDINGS: readonly string[] = ['.d.ts', '.d.mts', '.d.cts'];

/**
 * The name of the declaration file a TypeScript module's types are emitted
 * to: `user.ts` and `user.tsx` give `user.d.ts`, `user.mts` gives
 * `user.d.mts` and `user.cts` gives `user.d.cts`.
 */
export function declarationFileOf(fileName: string): string {
    if (fileName.endsWith('.mts')) return `${fileName.slice(0, -'.mts'.length)}.d.mts`;
    if (fileName.endsWith('.cts')) return `${fileName.slice(0, -'.cts'.length)}.d.cts`;
    const extension = ['.tsx', '.ts'].find((candidate) => fileName.endsWith(candidate));
    const stem = extension === undefined ? fileName : fileName.slice(0, -extension.length);
    return `${stem}.d.ts`;
}
