/**
 * @module config
 *
 * Utilities for loading Macroforge configuration files.
 */

import * as fs from "fs";
import * as path from "path";

/**
 * Supported config file names in order of precedence.
 */
export const CONFIG_FILES = [
  "macroforge.config.ts",
  "macroforge.config.mts",
  "macroforge.config.js",
  "macroforge.config.mjs",
  "macroforge.config.cjs",
] as const;

/**
 * Return type style for generated macro code.
 *
 * - `'vanilla'` (default): Plain TypeScript discriminated unions
 *   - Deserialize: `{ success: true; value: T } | { success: false; errors: FieldError[] }`
 *   - PartialOrd: `number | null`
 *
 * - `'custom'`: Uses `@rydshift/mirror` types
 *   - Deserialize: `Result<T, Array<FieldError>>`
 *   - PartialOrd: `Option<number>`
 *
 * - `'effect'`: Uses Effect library types
 *   - Deserialize: `Exit<FieldError[], T>`
 *   - PartialOrd: `Option.Option<number>`
 */
export type ReturnTypesMode = "vanilla" | "custom" | "effect";

/**
 * Result from parsing a config file.
 */
export interface ConfigLoadResult {
  keepDecorators: boolean;
  generateConvenienceConst: boolean;
  hasForeignTypes: boolean;
  foreignTypeCount: number;
  returnTypes: ReturnTypesMode;
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
   * Return type style for generated macro code.
   *
   * - `'vanilla'` (default): Plain TypeScript discriminated unions
   * - `'custom'`: Uses `@rydshift/mirror` Result/Option types
   * - `'effect'`: Uses Effect library Exit/Option types
   */
  returnTypes?: ReturnTypesMode;
}

/**
 * Function type for loading config content.
 * This allows plugins to inject their own config loading mechanism.
 */
export type ConfigLoader = (
  content: string,
  filepath: string,
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
    if (fs.existsSync(path.join(current, "package.json"))) {
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
 * @param startDir - The directory to start searching from (typically the project root)
 * @param loadConfigFn - Optional function to parse the config file content.
 *                       If provided, will be called with (content, filepath).
 *                       If not provided, only the configPath will be set.
 *
 * @returns The loaded configuration, or default values if no config file is found
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
  loadConfigFn?: ConfigLoader,
): MacroConfig {
  const fallback: MacroConfig = { keepDecorators: false };

  const configPath = findConfigFile(startDir);
  if (!configPath) {
    return fallback;
  }

  // If a loader function is provided, use it to parse the config
  if (loadConfigFn) {
    try {
      const content = fs.readFileSync(configPath, "utf8");
      const result = loadConfigFn(content, configPath);

      return {
        keepDecorators: result.keepDecorators,
        generateConvenienceConst: result.generateConvenienceConst,
        configPath,
        hasForeignTypes: result.hasForeignTypes,
        returnTypes: result.returnTypes,
      };
    } catch {
      // Fall through to fallback
    }
  }

  // Fallback: just mark the path but use defaults
  return {
    ...fallback,
    configPath,
  };
}
