import type { Plugin } from 'vite';

/**
 * Creates a Vite plugin for Macroforge compile-time macro expansion.
 *
 * Configuration is loaded from `macroforge.config.js` (or .ts/.mjs/.cjs).
 * Vite-specific options can be set under the `vite` key in the config file.
 *
 * @throws {Error} If a config file is found but cannot be imported (e.g. it
 *   has a syntax error); the underlying import error message is included.
 *
 * @example
 * ```typescript
 * // vite.config.ts
 * import { macroforge } from '@macroforge/vite-plugin';
 *
 * export default defineConfig({
 *   plugins: [macroforge()],
 * });
 * ```
 *
 * @example
 * ```typescript
 * // macroforge.config.ts
 * export default {
 *   keepDecorators: false,
 *   vite: {
 *     generateTypes: true,                   // Generate .d.ts files (default: true)
 *     typesOutputDir: ".macroforge/types",   // Types output dir (default: ".macroforge/types")
 *     emitMetadata: true,                    // Emit metadata JSON (default: true)
 *     metadataOutputDir: ".macroforge/meta", // Metadata output dir (default: ".macroforge/meta")
 *     devCache: true,                        // Disk cache for dev mode (default: true)
 *   },
 * };
 * ```
 */
export function macroforge(): Promise<Plugin<any>>;

export default macroforge;
