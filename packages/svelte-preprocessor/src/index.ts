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
 * - Gracefully degrades if native bindings are unavailable
 * - Only processes TypeScript blocks by default (configurable via options)
 *
 * @packageDocumentation
 */

import type { PreprocessorGroup, Preprocessor } from "svelte/compiler";

/**
 * Options passed to the native macro expansion engine.
 *
 * This interface mirrors the options accepted by `macroforge.expandSync()`.
 * It is defined locally to avoid requiring the macroforge package at type-check time,
 * enabling better decoupling and faster IDE performance.
 *
 * @internal
 */
interface ExpandOptions {
  /**
   * Whether to preserve `@derive` decorators in the expanded output.
   *
   * When `true`, decorators remain in the code after expansion, which can be useful
   * for debugging or when downstream tools need to see the original annotations.
   * When `false` (default), decorators are stripped from the output.
   */
  keepDecorators?: boolean;
}

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
    /** Byte offset where the issue starts in the source (optional) */
    start?: number;
    /** Byte offset where the issue ends in the source (optional) */
    end?: number;
  }>;

  /**
   * Source mapping information for mapping expanded code back to original.
   * Reserved for future source map support.
   * @see https://github.com/nicksrandall/sourcemap-codec for mapping format
   */
  source_mapping?: unknown;
}

/**
 * Cached reference to the native `expandSync` function from the macroforge package.
 *
 * This variable implements a lazy-loading pattern:
 * - Initially `null`, indicating the binding hasn't been loaded yet
 * - Set to the actual function after first successful load
 * - Remains `null` if loading fails (graceful degradation)
 *
 * The lazy-loading approach avoids loading native bindings at module import time,
 * which improves startup performance when the preprocessor is registered but no
 * components contain macros.
 *
 * @internal
 */
let expandSync: ((
  /** TypeScript/JavaScript source code to process */
  code: string,
  /** File path for error reporting and context */
  filepath: string,
  /** Optional expansion configuration */
  options?: ExpandOptions | null
) => ExpandResult) | null = null;

/**
 * Lazily loads and caches the native `expandSync` function.
 *
 * This function implements the initialization logic for native bindings:
 *
 * 1. On first call, dynamically imports the `macroforge` package
 * 2. Extracts and caches the `expandSync` function
 * 3. On subsequent calls, returns the cached function immediately
 *
 * The function is async because dynamic imports return promises, even though
 * the underlying `expandSync` function is synchronous. This async wrapper
 * only runs once; after initialization, the cached sync function is used directly.
 *
 * ## Error Handling
 *
 * If the native bindings fail to load (e.g., missing native module, architecture
 * mismatch), the function logs a warning and returns `null`. This allows the
 * preprocessor to gracefully skip macro expansion rather than crashing the build.
 *
 * @returns The cached `expandSync` function, or `null` if loading failed
 * @internal
 */
async function ensureExpandSync(): Promise<typeof expandSync> {
  if (expandSync === null) {
    try {
      // Dynamic import defers loading until first use
      const macroforge = await import("macroforge");
      expandSync = macroforge.expandSync;
    } catch (error) {
      // Log warning but don't throw - allows graceful degradation
      console.warn(
        "[@macroforge/svelte-preprocessor] Failed to load macroforge native bindings:",
        error
      );
      expandSync = null;
    }
  }
  return expandSync;
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
 * 2. **Quick Scan** - Skips blocks without `@derive` (performance optimization)
 * 3. **Expansion** - Calls the native engine to expand macros
 * 4. **Diagnostics** - Logs any errors or warnings from expansion
 * 5. **Return** - Returns transformed code or `undefined` if unchanged
 *
 * ## Error Handling
 *
 * The preprocessor is designed to be resilient:
 * - If native bindings fail to load, it silently skips processing
 * - If macro expansion throws, it logs a warning and continues
 * - Svelte compilation proceeds even if preprocessing fails
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
    filename,
    attributes,
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
    const isTypeScript = lang === "ts" || lang === "typescript";
    const isJavaScript =
      !lang || lang === "js" || lang === "javascript" || lang === "module";

    // Skip non-TypeScript blocks unless processJavaScript is enabled
    if (!isTypeScript && !(processJavaScript && isJavaScript)) {
      return; // Return undefined = no changes, Svelte keeps original content
    }

    /*
     * STEP 2: Quick Scan Optimization
     *
     * Before loading native bindings, do a cheap string check for "@derive".
     * Most components won't have macros, so this saves the cost of loading
     * and calling the native expansion engine in the common case.
     */
    if (!content.includes("@derive")) {
      return;
    }

    /*
     * STEP 3: Load Native Bindings
     *
     * The expansion engine is a native module (Rust compiled to Node addon).
     * We lazy-load it on first use to avoid startup overhead.
     */
    const expand = await ensureExpandSync();
    if (!expand) {
      // Native bindings unavailable (missing module, architecture mismatch, etc.)
      // Silently skip - the component will fail later if macros are actually needed
      return;
    }

    try {
      /*
       * STEP 4: Macro Expansion
       *
       * Call the native engine to parse the TypeScript, find @derive decorators,
       * and generate the expanded code with all macro-derived methods/properties.
       */
      const result = expand(content, filename || "component.svelte", {
        keepDecorators,
      });

      /*
       * STEP 5: Diagnostic Reporting
       *
       * The expansion engine may report errors (invalid macro syntax, unknown macros)
       * or warnings (deprecated patterns, suggestions). Log these to help developers
       * debug issues without failing the build.
       */
      for (const diag of result.diagnostics) {
        if (diag.level === "error") {
          console.error(
            `[@macroforge/svelte-preprocessor] Error in ${filename}: ${diag.message}`
          );
        } else if (diag.level === "warning") {
          console.warn(
            `[@macroforge/svelte-preprocessor] Warning in ${filename}: ${diag.message}`
          );
        }
      }

      /*
       * STEP 6: Return Transformed Code
       *
       * Only return a result if the code was actually modified. Returning undefined
       * tells Svelte to keep the original content, which is more efficient than
       * returning identical code.
       *
       * The return object can include:
       * - code: The transformed source code (required)
       * - map: Source map for debugging (optional, not yet implemented)
       */
      if (result.code && result.code !== content) {
        return {
          code: result.code,
          // TODO: Add source map support when expandSync provides mappings
          // map: result.source_mapping
        };
      }
    } catch (error) {
      /*
       * Error Recovery
       *
       * If expansion throws (parser error, internal bug, etc.), log a warning
       * but don't fail the build. This allows:
       * - Partial builds during development
       * - Graceful degradation if the macro engine has issues
       * - Svelte's own error reporting to kick in for syntax errors
       */
      console.warn(
        `[@macroforge/svelte-preprocessor] Failed to expand macros in ${filename}:`,
        error instanceof Error ? error.message : String(error)
      );
    }

    // No changes - either expansion produced identical output or an error occurred
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
    name: "macroforge",
    script: scriptPreprocessor,
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
