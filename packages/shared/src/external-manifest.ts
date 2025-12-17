/**
 * @module external-manifest
 *
 * Utilities for loading and caching external macro package manifests.
 */

import { createRequire } from "module";
import type { MacroManifest, MacroManifestEntry, DecoratorManifestEntry } from "macroforge";

/**
 * Function type for requiring modules.
 * Accepts any function that can load a module by path.
 */
export type RequireFunction = (id: string) => any;

/**
 * Cache for external macro package manifests.
 * Maps package path to its manifest (or null if failed to load).
 */
const externalManifestCache = new Map<string, MacroManifest | null>();

/**
 * Clears the external manifest cache.
 * Useful for testing or when packages may have been updated.
 */
export function clearExternalManifestCache(): void {
  externalManifestCache.clear();
}

/**
 * Attempts to load the manifest from an external macro package.
 *
 * External macro packages (like `@playground/macro`) export their own
 * `__macroforgeGetManifest()` function that provides macro metadata
 * including descriptions.
 *
 * @param modulePath - The package path (e.g., "@playground/macro")
 * @param requireFn - Optional custom require function. If not provided, creates one using import.meta.url context.
 * @returns The macro manifest, or null if loading failed
 *
 * @example
 * ```typescript
 * // Basic usage
 * const manifest = getExternalManifest("@playground/macro");
 *
 * // With custom require function (e.g., from vite-plugin)
 * import { createRequire } from "module";
 * const moduleRequire = createRequire(import.meta.url);
 * const manifest = getExternalManifest("@playground/macro", moduleRequire);
 * ```
 */
export function getExternalManifest(
  modulePath: string,
  requireFn?: RequireFunction,
): MacroManifest | null {
  if (externalManifestCache.has(modulePath)) {
    return externalManifestCache.get(modulePath) ?? null;
  }

  try {
    // Use provided require function or create one
    const req = requireFn ?? createRequire(import.meta.url);
    const pkg = req(modulePath);
    if (typeof pkg.__macroforgeGetManifest === "function") {
      const manifest: MacroManifest = pkg.__macroforgeGetManifest();
      externalManifestCache.set(modulePath, manifest);
      return manifest;
    }
  } catch {
    // Package not found or doesn't export manifest
  }

  externalManifestCache.set(modulePath, null);
  return null;
}

/**
 * Looks up macro info from an external package manifest.
 *
 * @param macroName - The macro name to look up
 * @param modulePath - The package path
 * @param requireFn - Optional custom require function
 * @returns The macro manifest entry, or null if not found
 *
 * @example
 * ```typescript
 * const macroInfo = getExternalMacroInfo("Gigaform", "@playground/macro");
 * if (macroInfo) {
 *   console.log(macroInfo.description);
 * }
 * ```
 */
export function getExternalMacroInfo(
  macroName: string,
  modulePath: string,
  requireFn?: RequireFunction,
): MacroManifestEntry | null {
  const manifest = getExternalManifest(modulePath, requireFn);
  if (!manifest) return null;

  return (
    manifest.macros.find(
      (m) => m.name.toLowerCase() === macroName.toLowerCase(),
    ) ?? null
  );
}

/**
 * Looks up decorator info from an external package manifest.
 *
 * @param decoratorName - The decorator name to look up
 * @param modulePath - The package path
 * @param requireFn - Optional custom require function
 * @returns The decorator manifest entry, or null if not found
 *
 * @example
 * ```typescript
 * const decoratorInfo = getExternalDecoratorInfo("hiddenController", "@playground/macro");
 * if (decoratorInfo) {
 *   console.log(decoratorInfo.description);
 * }
 * ```
 */
export function getExternalDecoratorInfo(
  decoratorName: string,
  modulePath: string,
  requireFn?: RequireFunction,
): DecoratorManifestEntry | null {
  const manifest = getExternalManifest(modulePath, requireFn);
  if (!manifest) return null;

  return (
    manifest.decorators.find(
      (d) => d.export.toLowerCase() === decoratorName.toLowerCase(),
    ) ?? null
  );
}

// Re-export types for convenience
export type { MacroManifest, MacroManifestEntry, DecoratorManifestEntry };
