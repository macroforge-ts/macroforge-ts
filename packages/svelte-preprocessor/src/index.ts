/**
 * @module @macroforge/svelte-preprocessor
 *
 * Svelte preprocessor for expanding Macroforge macros in component script blocks.
 *
 * This module provides integration between Macroforge's macro expansion system and
 * Svelte's preprocessing pipeline. It intercepts `<script>` blocks in `.svelte` files,
 * detects `@derive` decorators, and expands them into generated code before TypeScript
 * compilation occurs.
 *
 * ## How It Works
 *
 * 1. The preprocessor is registered in `svelte.config.js` as part of the preprocess array
 * 2. When Svelte compiles a component, it passes each `<script>` block to this preprocessor
 * 3. The preprocessor checks if the script contains `@derive` decorators
 * 4. If found, it calls the native `macroforge` binding to expand the macros
 * 5. The expanded code replaces the original script content
 *
 * ## Important Notes
 *
 * - Must be placed BEFORE other preprocessors (like `vitePreprocess()`) in the chain
 * - Uses lazy-loading for native bindings to avoid initialization overhead
 * - Fails a script block whose macros fail, as a syntax error in it would
 * - Only processes TypeScript blocks by default (configurable via options)
 *
 * @packageDocumentation
 */

import type { Preprocessor, PreprocessorGroup } from 'svelte/compiler';
import {
    type ExpandOptions,
    lineAndColumn,
    sourceMapOf,
    type SourceMapping
} from '@macroforge/shared';

/**
 * Result returned from the native macro expansion engine.
 *
 * Contains the expanded code along with any type information, metadata,
 * and diagnostics (errors/warnings) generated during expansion.
 *
 * @internal
 */
interface ExpandResult {
    /**
     * The expanded TypeScript/JavaScript code with all macros processed.
     * This replaces the original script content in the Svelte component.
     */
    code: string;

    /**
     * Generated TypeScript type declarations, if any macros produce type output.
     * Currently unused by the preprocessor but available for future enhancements.
     */
    types?: string | null;

    /**
     * Additional metadata from the expansion process.
     * May contain information about which macros were applied, statistics, etc.
     */
    metadata?: string | null;

    /**
     * Array of diagnostic messages generated during macro expansion.
     * These are logged to the console to help developers debug issues.
     */
    diagnostics: Array<{
        /** Severity level: "error", "warning", or "info" */
        level: string;
        /** Human-readable description of the issue */
        message: string;
        /** 0-based UTF-16 offset where the issue starts in the source (optional) */
        start?: number;
        /** 0-based UTF-16 offset where the issue ends in the source (optional) */
        end?: number;
    }>;

    /**
     * Segment-based source mapping between the original and expanded code,
     * emitted by the engine when expansion changed the source. Used to build
     * the v3 source map returned to Svelte.
     */
    sourceMapping?: SourceMapping;
}

/**
 * The engine functions the preprocessor uses, from the macroforge package.
 *
 * This variable implements a lazy-loading pattern:
 * - Initially `null`, indicating the binding hasn't been loaded yet
 * - Set to the engine after first successful load
 * - Remains `null` if loading fails, so the next call retries the import
 *
 * The lazy-loading approach avoids loading native bindings at module import time,
 * which improves startup performance when the preprocessor is registered but no
 * components contain macros.
 *
 * @internal
 */
let engine: Engine | null = null;

/** The engine functions the preprocessor calls. */
interface Engine {
    expandSync: (
        /** TypeScript/JavaScript source code to process */
        code: string,
        /** File path for error reporting and context */
        filepath: string,
        /** Optional expansion configuration */
        options?: ExpandOptions | null
    ) => ExpandResult;
    /** Whether `code`, the source of `filepath`, may contain anything the engine expands. */
    hasMacroAnnotations: (code: string, filepath: string) => boolean;
}

/**
 * Lazily loads and caches the engine.
 *
 * This function implements the initialization logic for native bindings:
 *
 * 1. On first call, dynamically imports the `macroforge` package
 * 2. Extracts and caches `expandSync` and `hasMacroAnnotations`
 * 3. On subsequent calls after a successful load, returns the cached function
 *    immediately
 *
 * The function is async because dynamic imports return promises, even though
 * the underlying `expandSync` function is synchronous. After a successful
 * initialization, the cached sync function is used directly.
 *
 * ## Error Handling
 *
 * If the native bindings fail to load (e.g., missing native module, architecture
 * mismatch), the function throws with the cause. Without the engine no macro
 * is expanded, and a component built that way lacks what its macros generate.
 * A failed load leaves the cache empty, so the next block tries again.
 *
 * @returns The cached engine
 * @internal
 */
async function ensureEngine(): Promise<Engine> {
    if (engine === null) {
        try {
            // Dynamic import defers loading until first use
            const macroforge = await import('@macroforge/core');
            engine = {
                expandSync: macroforge.expandSync,
                hasMacroAnnotations: macroforge.hasMacroAnnotations
            };
        } catch (error) {
            // Without the engine no macro is expanded, and a component that
            // uses one would be built without what it generates.
            throw new Error(
                '[@macroforge/svelte-preprocessor] Failed to load the macro engine `@macroforge/core`',
                { cause: error }
            );
        }
    }
    return engine;
}

/**
 * Configuration options for the Macroforge Svelte preprocessor.
 *
 * These options control how the preprocessor identifies and transforms
 * script blocks containing `@derive` decorators.
 *
 * @example
 * ```ts
 * // Default behavior - TypeScript only, decorators stripped
 * macroforgePreprocess()
 *
 * // Keep decorators for debugging
 * macroforgePreprocess({ keepDecorators: true })
 *
 * // Process both TypeScript and JavaScript
 * macroforgePreprocess({ processJavaScript: true })
 * ```
 */
export interface MacroforgePreprocessorOptions {
    /**
     * Whether to preserve `@derive` decorators in the expanded output.
     *
     * By default, decorators are stripped after expansion since they've served
     * their purpose. Set to `true` if you need to:
     * - Debug macro expansion by seeing both decorators and generated code
     * - Pass decorators through to another tool in the pipeline
     * - Preserve decorators for documentation generation
     *
     * @default false
     */
    keepDecorators?: boolean;

    /**
     * Whether to process JavaScript script blocks in addition to TypeScript.
     *
     * By default, only `<script lang="ts">` and `<script lang="typescript">`
     * blocks are processed, since Macroforge is primarily designed for TypeScript.
     *
     * Set to `true` to also process:
     * - `<script>` (no lang attribute)
     * - `<script lang="js">`
     * - `<script lang="javascript">`
     * - `<script type="module">`
     *
     * @default false
     */
    processJavaScript?: boolean;
}

/**
 * Creates a Svelte preprocessor that expands Macroforge macros in `<script>` blocks.
 *
 * This is the main entry point for integrating Macroforge with Svelte. The returned
 * preprocessor intercepts script blocks, detects `@derive` decorators, and expands
 * them into generated code using the native Macroforge engine.
 *
 * ## Preprocessor Order
 *
 * **Important:** This preprocessor must be placed BEFORE other preprocessors like
 * `vitePreprocess()` in the preprocess array. Macros must be expanded before
 * TypeScript compilation occurs, or the TypeScript compiler will fail on the
 * decorator syntax.
 *
 * ## Processing Logic
 *
 * The preprocessor performs these steps for each script block:
 *
 * 1. **Language Check** - Verifies the script is TypeScript (or JavaScript if enabled)
 * 2. **Expansion** - Calls the native engine to expand macros
 * 3. **Diagnostics** - Logs any errors or warnings from expansion
 * 4. **Return** - Returns transformed code or `undefined` if unchanged
 *
 * Every matching script block is passed to the engine; there is no
 * pre-scan for `@derive`, so unchanged blocks simply come back identical.
 *
 * ## Error Handling
 *
 * A failure fails the script block, and Svelte reports it for the component:
 * - If the native bindings fail to load, it throws with the cause
 * - If the engine reports a macro error, it throws with every message
 * - If macro expansion throws, it throws with the file and the cause
 * Warnings are logged and the block is expanded.
 *
 * @param options - Configuration options for the preprocessor
 * @returns A Svelte `PreprocessorGroup` with name "macroforge" and a script preprocessor
 *
 * @example Basic usage
 * ```js
 * // svelte.config.js
 * import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
 * import { macroforgePreprocess } from '@macroforge/svelte-preprocessor';
 *
 * export default {
 *   preprocess: [
 *     macroforgePreprocess(),   // Expand macros FIRST
 *     vitePreprocess()          // Then handle TypeScript/CSS
 *   ]
 * };
 * ```
 *
 * @example With options
 * ```js
 * // svelte.config.js
 * import { macroforgePreprocess } from '@macroforge/svelte-preprocessor';
 *
 * export default {
 *   preprocess: [
 *     macroforgePreprocess({
 *       keepDecorators: true,      // Keep @derive in output for debugging
 *       processJavaScript: true    // Also process <script> blocks without lang="ts"
 *     })
 *   ]
 * };
 * ```
 */
export function macroforgePreprocess(
    options: MacroforgePreprocessorOptions = {}
): PreprocessorGroup {
    // Destructure options with defaults
    const { keepDecorators = false, processJavaScript = false } = options;

    /**
     * The script preprocessor function that Svelte calls for each `<script>` block.
     *
     * Svelte passes three properties:
     * - `content`: The text content of the script block
     * - `filename`: Path to the .svelte file being processed
     * - `attributes`: Object of attributes from the script tag (e.g., `{ lang: "ts" }`)
     *
     * @returns An object with `code` property if transformed, or `undefined` if no changes
     */
    const scriptPreprocessor: Preprocessor = async ({
        content,
        markup,
        filename,
        attributes
    }) => {
        /*
         * STEP 1: Language Detection
         *
         * Determine if this script block should be processed based on its language.
         * Svelte allows both `lang` and `type` attributes for specifying script language.
         *
         * Examples:
         * - <script lang="ts">        → isTypeScript = true
         * - <script lang="typescript"> → isTypeScript = true
         * - <script>                  → isJavaScript = true (no lang = JavaScript)
         * - <script lang="js">        → isJavaScript = true
         * - <script type="module">    → isJavaScript = true
         */
        const lang = attributes.lang || attributes.type;
        const isTypeScript = lang === 'ts' || lang === 'typescript';
        const isJavaScript = !lang || lang === 'js' || lang === 'javascript' ||
            lang === 'module';

        // Skip non-TypeScript blocks unless processJavaScript is enabled
        if (!isTypeScript && !(processJavaScript && isJavaScript)) {
            return; // Return undefined = no changes, Svelte keeps original content
        }

        /*
         * STEP 2: Load Native Bindings
         *
         * The expansion engine is a native module (Rust compiled to Node addon).
         * We lazy-load it on first use to avoid startup overhead.
         */
        const loaded = await ensureEngine();

        // A JavaScript block is expanded under a JavaScript name, so that a
        // macro that writes TypeScript is refused there instead of leaving
        // types in a script Svelte reads as JavaScript.
        const component = filename || 'component.svelte';
        const expandedAs = isTypeScript ? component : `${component}.js`;

        // A block with nothing to expand never reaches the engine.
        if (!loaded.hasMacroAnnotations(content, expandedAs)) {
            return;
        }

        /*
         * STEP 3: Macro Expansion
         *
         * Call the native engine to parse the TypeScript, find @derive decorators,
         * and generate the expanded code with all macro-derived methods/properties.
         */
        let result;
        try {
            result = loaded.expandSync(content, expandedAs, {
                keepDecorators,
                emitMetadata: false
            });
        } catch (error) {
            throw new Error(
                `[@macroforge/svelte-preprocessor] Failed to expand macros in ${component}: ${
                    error instanceof Error ? error.message : String(error)
                }`,
                { cause: error }
            );
        }

        /*
         * STEP 4: Diagnostic Reporting
         *
         * A macro error fails the block, as a syntax error would: a component
         * built without the code its macros generate is broken where it is
         * used, with nothing to say why. Warnings are logged.
         */
        // Where the script starts in the component, so a message names the
        // place in the file that holds it.
        const scriptStart = typeof markup === 'string' && content.length > 0
            ? markup.indexOf(content)
            : -1;
        const place = (start: number | undefined) => {
            if (start === undefined || scriptStart < 0 || typeof markup !== 'string') return '';
            const { line, column } = lineAndColumn(markup, scriptStart + start);
            return `${component}:${line}:${column}: `;
        };
        const errors = [];
        for (const diag of result.diagnostics) {
            if (diag.level === 'error') {
                errors.push(place(diag.start) + diag.message);
            } else if (diag.level === 'warning') {
                console.warn(
                    `[@macroforge/svelte-preprocessor] Warning in ${component}: ${diag.message}`
                );
            }
        }
        if (errors.length > 0) {
            throw new Error(
                `[@macroforge/svelte-preprocessor] ${errors.length} macro error(s) in ${component}:\n${
                    errors.join('\n')
                }`
            );
        }

        /*
         * STEP 5: Return Transformed Code
         *
         * Only return a result if the code was actually modified. Returning undefined
         * tells Svelte to keep the original content, which is more efficient than
         * returning identical code.
         *
         * The return object can include:
         * - code: The transformed source code (required)
         * - map: v3 source map built from the engine's segment mapping (when available)
         */
        if (result.code && result.code !== content) {
            const map = result.sourceMapping && filename
                ? sourceMapOf(result.sourceMapping, filename, content, result.code)
                : undefined;
            return { code: result.code, map };
        }

        // The expansion left the block as it was.
        return;
    };

    /*
     * Return the PreprocessorGroup object that Svelte expects.
     *
     * - name: Identifier shown in Svelte's debug output and error messages
     * - script: The preprocessor function for <script> blocks
     *
     * Note: We only handle script blocks, not markup or style blocks.
     */
    return {
        name: 'macroforge',
        script: scriptPreprocessor
    };
}

/**
 * Default export for convenient importing.
 *
 * @example
 * ```js
 * import macroforgePreprocess from '@macroforge/svelte-preprocessor';
 * ```
 */
export default macroforgePreprocess;
