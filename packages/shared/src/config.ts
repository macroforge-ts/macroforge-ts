/**
 * @module config
 *
 * Utilities for loading Macroforge configuration files.
 */

import * as fs from 'node:fs';
import * as path from 'node:path';

/**
 * Supported config file names in order of precedence.
 */
export const CONFIG_FILES = [
    'macroforge.config.ts',
    'macroforge.config.mts',
    'macroforge.config.js',
    'macroforge.config.mjs',
    'macroforge.config.cjs'
] as const;

/**
 * Result from parsing a config file (as returned by the native `loadConfig`).
 *
 * Note that `loadMacroConfig` forwards only `keepDecorators`,
 * `generateConvenienceConst`, and `hasForeignTypes` into its `MacroConfig`
 * result; the remaining flags are informational for the caller.
 */
export interface ConfigLoadResult {
    /** Value of `keepDecorators` from the config file (false when unset). */
    keepDecorators: boolean;
    /** Value of `generateConvenienceConst` from the config file (true when unset). */
    generateConvenienceConst: boolean;
    /** True when the config registers at least one foreign type handler. */
    hasForeignTypes: boolean;
    /** Number of foreign type handlers registered by the config. */
    foreignTypeCount: number;
    /** True when the config provides a non-empty `cfg` block. */
    hasCfgFlags?: boolean;
    /** True when the config overrides any `deprecated` defaults. */
    hasDeprecatedConfig?: boolean;
    /** True when the config overrides any `mustUse` defaults. */
    hasMustUseConfig?: boolean;
    /** True when the config overrides any `nonExhaustive` defaults. */
    hasNonExhaustiveConfig?: boolean;
}

/**
 * Build flags consumed by the `@cfg` attribute macro.
 *
 * @example
 * ```ts
 * // macroforge.config.ts
 * export default {
 *   cfg: {
 *     features: ['ssr'],
 *     target: 'web',
 *     debugAssertions: true,
 *     custom: { tenant: 'acme' }
 *   }
 * };
 *
 * // Source:
 * /​** @cfg({ feature: 'ssr' }) *​/
 * export function render() { ... }
 * ```
 */
export interface CfgFlags {
    /** Feature flags. `@cfg({ feature: 'ssr' })` matches when `'ssr'` is in this list. */
    features?: string[];
    /** Build target (e.g. `"web"`, `"node"`, `"deno"`). Matched exactly. */
    target?: string;
    /** Whether `@cfg({ debugAssertions: true })` should pass. */
    debugAssertions?: boolean;
    /** Arbitrary string-keyed predicate values matched exactly against annotations. */
    custom?: Record<string, string | number | boolean | null>;
}

/**
 * Behavior knobs for the `@deprecated` attribute macro.
 */
export interface DeprecatedConfig {
    /** Inject a one-shot `console.warn(...)` into the deprecated body. Default: true. */
    runtimeWarn?: boolean;
    /** Treat any use of a deprecated symbol as a macro-expansion error. Default: false. */
    failOnUse?: boolean;
}

/**
 * Behavior knobs for the `@mustUse` attribute macro.
 */
export interface MustUseConfig {
    /**
     * Enforcement strategy. Today only `"lint"` is supported (build-time
     * diagnostic at discarded-call sites).
     */
    mode?: 'lint';
}

/**
 * Behavior knobs for the `@nonExhaustive` attribute macro.
 */
export interface NonExhaustiveConfig {
    /**
     * Brand property name used in the intersection. Keep this stable across a
     * project so downstream consumers can pattern-match on it.
     * @default "__nonExhaustive"
     */
    brand?: string;
}

/**
 * Vite plugin configuration options.
 *
 * @remarks
 * These options control the Vite plugin behavior for type generation and metadata emission.
 */
export interface VitePluginConfig {
    /**
     * Whether to generate `.d.ts` type definition files from expanded code.
     * @default true
     */
    generateTypes?: boolean;

    /**
     * Output directory for generated type definitions, relative to project root.
     * @default ".macroforge/types"
     */
    typesOutputDir?: string;

    /**
     * Whether to emit macro IR metadata as JSON files.
     * @default true
     */
    emitMetadata?: boolean;

    /**
     * Output directory for metadata JSON files, relative to project root.
     * @default ".macroforge/meta"
     */
    metadataOutputDir?: string;

    /**
     * Enable disk-based expansion cache in dev mode (`vite dev`).
     *
     * When enabled, the plugin reads pre-expanded files from `.macroforge/cache/`
     * instead of calling `expandSync()` on every transform, and self-populates
     * the cache on misses. Use `macroforge watch` to keep the cache warm.
     *
     * @default true
     */
    devCache?: boolean;
}

/**
 * Configuration options loaded from `macroforge.config.js` (or .ts/.mjs/.cjs).
 *
 * @remarks
 * This configuration affects how macros are expanded and what artifacts
 * are preserved in the output.
 */
export interface MacroConfig {
    /**
     * Whether to preserve `@derive` decorators in the output code after macro expansion.
     *
     * @remarks
     * When `false` (default), decorators are removed after expansion since they serve
     * only as compile-time directives. When `true`, decorators are kept in the output,
     * which can be useful for debugging or when using runtime reflection.
     */
    keepDecorators: boolean;

    /**
     * Whether to generate a convenience const for non-class types.
     *
     * @remarks
     * When `true` (default), generates an `export const TypeName = { ... } as const;`
     * that groups all generated functions for a type into a single namespace-like object.
     * For example: `export const User = { clone: userClone, serialize: userSerialize } as const;`
     *
     * When `false`, only the standalone functions are generated without the grouping const.
     */
    generateConvenienceConst?: boolean;

    /**
     * Path to the config file (used to cache and retrieve foreign types).
     */
    configPath?: string;

    /**
     * Whether the config has foreign type handlers defined.
     */
    hasForeignTypes?: boolean;

    /**
     * Build flags for the `@cfg` attribute macro.
     */
    cfg?: CfgFlags;

    /**
     * Behavior knobs for the `@deprecated` attribute macro.
     */
    deprecated?: DeprecatedConfig;

    /**
     * Behavior knobs for the `@mustUse` attribute macro.
     */
    mustUse?: MustUseConfig;

    /**
     * Behavior knobs for the `@nonExhaustive` attribute macro.
     */
    nonExhaustive?: NonExhaustiveConfig;

    /**
     * Vite plugin configuration options.
     *
     * @remarks
     * These options configure the `@macroforge/vite-plugin` behavior.
     */
    vite?: VitePluginConfig;
}

/**
 * Function type for loading config content.
 * This allows plugins to inject their own config loading mechanism.
 */
export type ConfigLoader = (
    content: string,
    filepath: string
) => ConfigLoadResult;

/**
 * Finds a macroforge config file in the directory tree.
 *
 * @param startDir - The directory to start searching from
 * @returns The path to the config file, or null if not found
 *
 * @remarks
 * The search stops when:
 * - A config file is found
 * - A package.json boundary is reached
 * - The filesystem root is reached
 *
 * @example
 * ```typescript
 * const configPath = findConfigFile('/project/src/components');
 * // => '/project/macroforge.config.js' or null
 * ```
 */
export function findConfigFile(startDir: string): string | null {
    let current = startDir;

    while (true) {
        for (const filename of CONFIG_FILES) {
            const candidate = path.join(current, filename);
            if (fs.existsSync(candidate)) {
                return candidate;
            }
        }

        // Stop at package.json boundary
        if (fs.existsSync(path.join(current, 'package.json'))) {
            break;
        }

        const parent = path.dirname(current);
        if (parent === current) break;
        current = parent;
    }

    return null;
}

/**
 * Loads Macroforge configuration from `macroforge.config.js` (or .ts/.mjs/.cjs).
 *
 * @remarks
 * Starting from the given directory, this function walks up the filesystem hierarchy
 * looking for a macroforge config file. The first one found is parsed using the
 * provided loader function (if any), which extracts configuration including
 * foreign type handlers.
 *
 * The returned `MacroConfig` is a subset: only `keepDecorators`,
 * `generateConvenienceConst`, `configPath`, and `hasForeignTypes` are ever
 * populated. The macro-behavior sections (`cfg`, `deprecated`, `mustUse`,
 * `nonExhaustive`) and the `vite` section are NOT surfaced here — the native
 * engine consumes them itself from its cache keyed by `configPath` (pass
 * `configPath` through in `ExpandOptions`), and the Vite plugin re-imports
 * the config file to read its `vite` section.
 *
 * If the loader function throws, the error is swallowed and the fallback
 * (defaults plus `configPath`) is returned.
 *
 * @param startDir - The directory to start searching from (typically the project root)
 * @param loadConfigFn - Optional function to parse the config file content.
 *                       If provided, will be called with (content, filepath).
 *                       If not provided, only the configPath will be set.
 *
 * @returns The loaded configuration subset, or default values if no config file is found
 *
 * @example
 * ```typescript
 * // Basic usage (no parsing, just find the config path)
 * const config = loadMacroConfig('/project/src');
 * // => { keepDecorators: false, configPath: '/project/macroforge.config.js' }
 *
 * // With Rust binary parser
 * const config = loadMacroConfig('/project/src', rustTransformer.loadConfig);
 * // => { keepDecorators: true, configPath: '...', hasForeignTypes: true }
 * ```
 */
export function loadMacroConfig(
    startDir: string,
    loadConfigFn?: ConfigLoader
): MacroConfig {
    const fallback: MacroConfig = {
        keepDecorators: false,
        hasForeignTypes: false
    };

    const configPath = findConfigFile(startDir);
    if (!configPath) {
        return fallback;
    }

    // If a loader function is provided, use it to parse the config
    if (loadConfigFn) {
        try {
            const content = fs.readFileSync(configPath, 'utf8');
            const result = loadConfigFn(content, configPath);
            return {
                keepDecorators: result.keepDecorators,
                generateConvenienceConst: result.generateConvenienceConst,
                configPath,
                hasForeignTypes: result.hasForeignTypes
            };
        } catch {
            // console.error(`[macroforge:shared] loadMacroConfig failed for ${configPath}:`, e);
            // Fall through to fallback
        }
    }

    // Fallback: just mark the path but use defaults
    return {
        ...fallback,
        configPath
    };
}
