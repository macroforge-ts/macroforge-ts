/**
 * @module @macroforge/deno-plugin
 *
 * Deno-native macro expansion for projects that don't use Vite.
 *
 * Deno has no public loader-hook API equivalent to Node's `--loader` flag,
 * so this package takes the file-on-disk approach: walk the project, expand
 * `.ts` / `.tsx` files, and write the expanded output to a mirror directory
 * (`.macroforge/cache/` by default). Point Deno at the mirror via an
 * `imports` / `scopes` entry in `deno.json`, or via path mapping in your
 * task script.
 *
 * @example Programmatic
 * ```ts
 * import { expandFile } from '@macroforge/deno-plugin';
 *
 * const { code, hasMacros } = await expandFile('./src/forms/User.ts');
 * await Deno.writeTextFile('./build/forms/User.ts', code);
 * ```
 *
 * @example CLI
 * ```sh
 * deno run -A jsr:@macroforge/deno-plugin/cli expand --root . --out .macroforge/cache
 * deno run -A jsr:@macroforge/deno-plugin/cli watch  --root .
 * ```
 *
 * @packageDocumentation
 */

import {
  collectExternalDecoratorModules,
  loadMacroConfig,
} from "@macroforge/shared";
import { createRequire } from "node:module";
import { loadRustTransformer } from "./bootstrap.ts";

export interface ExpandOptions {
  /** Project root used for config discovery and external macro resolution. Defaults to `Deno.cwd()`. */
  projectRoot?: string;
  /** Override `keepDecorators` from `macroforge.config.*`. */
  keepDecorators?: boolean;
  /** Build mode passed to the engine. `dev` runs declarative macros expand-only for diagnostics. */
  buildMode?: "dev" | "prod";
  /** Pre-loaded type registry JSON (usually from `.macroforge/type-registry.json`). */
  typeRegistryJson?: string;
  /** Pre-loaded declarative macro registry JSON. */
  declarativeRegistryJson?: string;
}

export interface ExpandResult {
  /** The post-expansion source. Equal to the input when nothing matched. */
  code: string;
  /** True if derive macros emitted regions or `@buildtime` rewrote the source. */
  hasMacros: boolean;
  /** Diagnostics emitted by the engine. Errors are reported but not thrown. */
  diagnostics: Array<
    { level: string; message: string; start?: number; end?: number }
  >;
  /** Absolute paths the `@buildtime` pre-pass read; useful for watch-mode invalidation. */
  buildtimeDependencies: string[];
  /** Macro IR metadata as a JSON string, when emitted by the engine. */
  metadata?: string;
}

/**
 * Expand a single in-memory TypeScript source string.
 *
 * Looks up `macroforge.config.*` from `projectRoot` (or `Deno.cwd()`) on
 * first call, caches the parsed config, and forwards every other concern
 * to the Rust engine.
 */
export function expand(
  code: string,
  filepath: string,
  options: ExpandOptions = {},
): ExpandResult {
  const projectRoot = options.projectRoot ?? Deno.cwd();
  const transformer = loadRustTransformer(projectRoot);
  const macroConfig = loadMacroConfig(projectRoot, transformer.loadConfig);

  const projectRequire = createRequire(projectRoot + "/");
  const externalDecoratorModules = collectExternalDecoratorModules(
    code,
    projectRequire,
  );

  const result = transformer.expandSync(code, filepath, {
    keepDecorators: options.keepDecorators ?? macroConfig.keepDecorators,
    externalDecoratorModules,
    configPath: macroConfig.configPath,
    typeRegistryJson: options.typeRegistryJson,
    declarativeRegistryJson: options.declarativeRegistryJson,
    buildMode: options.buildMode ?? "prod",
  });

  let outCode = result.code ?? code;

  // The vite-plugin strips macro-only imports here so SSR doesn't load
  // native bindings. Same reasoning applies to Deno output.
  outCode = outCode.replace(/\/\*\*\s*import\s+macro[\s\S]*?\*\/\s*/gi, "");

  // For .svelte.ts modules, strip @derive JSDoc to prevent the Svelte
  // preprocessor from re-expanding macros downstream.
  if (filepath.endsWith(".svelte.ts") || filepath.endsWith(".svelte.js")) {
    outCode = outCode.replace(/\/\*\*\s*@derive\b[^*]*\*\//g, "");
  }

  const hasMacros = (result.sourceMapping?.generatedRegions?.length ?? 0) > 0 ||
    (result.buildtimeDependencies?.length ?? 0) > 0 ||
    outCode !== code;

  return {
    code: outCode,
    hasMacros,
    diagnostics: result.diagnostics ?? [],
    buildtimeDependencies: result.buildtimeDependencies ?? [],
    metadata: result.metadata,
  };
}

/**
 * Read a file from disk and expand it.
 */
export async function expandFile(
  filepath: string,
  options: ExpandOptions = {},
): Promise<ExpandResult> {
  const code = await Deno.readTextFile(filepath);
  return expand(code, filepath, options);
}

export { loadRustTransformer } from "./bootstrap.ts";
