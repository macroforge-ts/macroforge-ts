/**
 * @module @macroforge/shared
 *
 * Shared utilities for Macroforge plugins.
 *
 * This package provides common functionality used by both `@macroforge/vite-plugin`
 * and `@macroforge/typescript-plugin`, ensuring consistent behavior across
 * different build tools.
 *
 * @packageDocumentation
 */

// Re-export all utilities
export { parseMacroImportComments } from "./macro-imports.js";

export {
  getExternalManifest,
  getExternalMacroInfo,
  getExternalDecoratorInfo,
  clearExternalManifestCache,
  type MacroManifest,
  type MacroManifestEntry,
  type DecoratorManifestEntry,
  type RequireFunction,
} from "./external-manifest.js";

export {
  CONFIG_FILES,
  findConfigFile,
  loadMacroConfig,
  type MacroConfig,
  type VitePluginConfig,
  type ConfigLoader,
  type ConfigLoadResult,
} from "./config.js";

// Import for composite functions
import { parseMacroImportComments } from "./macro-imports.js";
import { getExternalManifest, type RequireFunction } from "./external-manifest.js";

/**
 * Collects decorator modules from external macro packages referenced in the code.
 *
 * This function parses macro import comments in the code to find external packages,
 * then loads their manifests and collects all decorator module names.
 *
 * @param code - The TypeScript source code to scan
 * @param requireFn - Optional custom require function for loading external packages
 * @returns Array of decorator module names from external packages
 *
 * @example
 * ```typescript
 * const code = `/** import macro {Gigaform} from "@playground/macro"; *​/
 * /** @derive(Gigaform) *​/
 * class MyForm {}`;
 *
 * const modules = collectExternalDecoratorModules(code);
 * // => ["hiddenController", "fieldController", ...]
 * ```
 */
export function collectExternalDecoratorModules(
  code: string,
  requireFn?: RequireFunction,
): string[] {
  const imports = parseMacroImportComments(code);
  const modulePaths = [...new Set(imports.values())];

  return modulePaths.flatMap((modulePath) => {
    const manifest = getExternalManifest(modulePath, requireFn);
    return manifest?.decorators.map((d) => d.module) ?? [];
  });
}
