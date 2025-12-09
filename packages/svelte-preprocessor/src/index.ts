import type { PreprocessorGroup, Preprocessor } from "svelte/compiler";

// Import types from macroforge
interface ExpandOptions {
  keepDecorators?: boolean;
}

interface ExpandResult {
  code: string;
  types?: string | null;
  metadata?: string | null;
  diagnostics: Array<{
    level: string;
    message: string;
    start?: number;
    end?: number;
  }>;
  source_mapping?: unknown;
}

// Dynamic import to handle native binding loading
let expandSync: ((
  code: string,
  filepath: string,
  options?: ExpandOptions | null
) => ExpandResult) | null = null;

async function ensureExpandSync(): Promise<typeof expandSync> {
  if (expandSync === null) {
    try {
      const macroforge = await import("macroforge");
      expandSync = macroforge.expandSync;
    } catch (error) {
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
 * Options for the macroforge Svelte preprocessor
 */
export interface MacroforgePreprocessorOptions {
  /**
   * Whether to keep @derive decorators in the output.
   * Useful for debugging or when you need to see the decorators in generated code.
   * @default false
   */
  keepDecorators?: boolean;

  /**
   * Whether to process JavaScript files (not just TypeScript).
   * By default, only `<script lang="ts">` blocks are processed.
   * @default false
   */
  processJavaScript?: boolean;
}

/**
 * Creates a Svelte preprocessor that expands macroforge macros in `<script>` blocks.
 *
 * This preprocessor should be placed BEFORE other preprocessors like `vitePreprocess()`
 * to ensure macros are expanded before TypeScript compilation.
 *
 * @example
 * ```js
 * // svelte.config.js
 * import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
 * import { macroforgePreprocess } from '@macroforge/svelte-preprocessor';
 *
 * export default {
 *   preprocess: [
 *     macroforgePreprocess(),  // Expand macros FIRST
 *     vitePreprocess()          // Then handle TypeScript/CSS
 *   ]
 * };
 * ```
 */
export function macroforgePreprocess(
  options: MacroforgePreprocessorOptions = {}
): PreprocessorGroup {
  const { keepDecorators = false, processJavaScript = false } = options;

  const scriptPreprocessor: Preprocessor = async ({
    content,
    filename,
    attributes,
  }) => {
    // Check if we should process this script block
    const lang = attributes.lang || attributes.type;
    const isTypeScript = lang === "ts" || lang === "typescript";
    const isJavaScript =
      !lang || lang === "js" || lang === "javascript" || lang === "module";

    if (!isTypeScript && !(processJavaScript && isJavaScript)) {
      return; // Return undefined = no changes
    }

    // Early bailout: no macros to process
    if (!content.includes("@derive")) {
      return;
    }

    // Ensure native bindings are loaded
    const expand = await ensureExpandSync();
    if (!expand) {
      // Native bindings not available - skip silently
      return;
    }

    try {
      const result = expand(content, filename || "component.svelte", {
        keepDecorators,
      });

      // Report any diagnostics
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

      // Only return if code was actually changed
      if (result.code && result.code !== content) {
        return {
          code: result.code,
          // TODO: Add source map support when expandSync provides it
        };
      }
    } catch (error) {
      // Log warning but don't fail - let Svelte continue
      console.warn(
        `[@macroforge/svelte-preprocessor] Failed to expand macros in ${filename}:`,
        error instanceof Error ? error.message : String(error)
      );
    }

    return; // No changes
  };

  return {
    name: "macroforge",
    script: scriptPreprocessor,
  };
}

export default macroforgePreprocess;
