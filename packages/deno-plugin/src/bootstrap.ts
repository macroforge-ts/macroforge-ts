/**
 * @module @macroforge/deno-plugin/bootstrap
 *
 * Lazy initialization of the macroforge Rust binary (NAPI or WASM) for Deno.
 *
 * The WASM build cannot spawn subprocesses or touch the filesystem, so the
 * host must register `setupBuildtimeFs` and `setupExternalMacros` callbacks
 * before `expandSync` is called. This mirrors what `@macroforge/vite-plugin`
 * does in Node, but goes through Deno's `node:` compat for `fs` / `module`.
 */

import { createRequire } from "node:module";
import * as fs from "node:fs";

interface RustTransformer {
  expandSync: (code: string, filepath: string, options: unknown) => {
    code: string;
    diagnostics: Array<
      { level: string; message: string; start?: number; end?: number }
    >;
    sourceMapping?: {
      segments: Array<{
        originalStart: number;
        originalEnd: number;
        expandedStart: number;
        expandedEnd: number;
      }>;
      generatedRegions?: unknown[];
    };
    metadata?: string;
    buildtimeDependencies?: string[];
  };
  loadConfig?: (content: string, filepath: string) => {
    keepDecorators: boolean;
    generateConvenienceConst: boolean;
    hasForeignTypes: boolean;
    foreignTypeCount: number;
  };
  setupBuildtimeFs?: (
    readText: (path: string) => string,
    exists: (path: string) => boolean,
    listDir: (path: string) => string[],
  ) => void;
  setupExternalMacros?: (
    resolve: (packagePath: string) => string[],
    run: (ctxJson: string) => string,
  ) => void;
  invalidateScanCacheEntry?: (path: string) => boolean;
  clearScanCache?: () => void;
}

let cached: RustTransformer | undefined;

/**
 * Resolve and initialize the macroforge engine. The first call loads the
 * binary from the project's `node_modules` (resolved via the project CWD)
 * and registers the host-side callbacks the WASM build needs. Subsequent
 * calls return the same instance.
 */
export function loadRustTransformer(
  projectRoot: string = Deno.cwd(),
): RustTransformer {
  if (cached) return cached;

  const projectRequire = createRequire(projectRoot + "/");
  const transformer = projectRequire("macroforge") as RustTransformer;

  // WASM build: provide a filesystem bridge for `@buildtime` evaluation.
  // NAPI build: setupBuildtimeFs is undefined and this is a no-op.
  if (transformer.setupBuildtimeFs) {
    transformer.setupBuildtimeFs(
      (path) => fs.readFileSync(path, "utf-8"),
      (path) => fs.existsSync(path),
      (path) => {
        try {
          return fs.readdirSync(path);
        } catch {
          return [];
        }
      },
    );
  }

  // WASM build: provide JS-side resolution for external macro packages.
  // The vite-plugin walks `pkg.__macroforgeGetManifest()` plus any
  // `__macroforgeGetManifest_*` helpers; we follow the same shape.
  if (transformer.setupExternalMacros) {
    const pkgRequire = createRequire(projectRoot + "/package.json");

    const resolveDecoratorNames = (packagePath: string): string[] => {
      const pkg = pkgRequire(packagePath) as Record<string, unknown>;
      const names: string[] = [];
      const getManifest = pkg.__macroforgeGetManifest as
        | (() => { decorators?: Array<{ export: string }> })
        | undefined;
      if (getManifest) {
        names.push(...(getManifest().decorators ?? []).map((d) => d.export));
      }
      for (const key of Object.keys(pkg)) {
        if (
          key.startsWith("__macroforgeGetManifest_") &&
          typeof pkg[key] === "function"
        ) {
          const fn = pkg[key] as () => {
            decorators?: Array<{ export: string }>;
          };
          names.push(...(fn().decorators ?? []).map((d) => d.export));
        }
      }
      return [...new Set(names)];
    };

    const runMacro = (ctxJson: string): string => {
      const ctx = JSON.parse(ctxJson) as {
        macro_name: string;
        module_path: string;
      };
      const fnName = `__macroforgeRun${ctx.macro_name}`;
      const pkg = pkgRequire(ctx.module_path) as Record<string, unknown> & {
        default?: Record<string, unknown>;
      };
      const fn_ = pkg?.[fnName] ?? pkg?.default?.[fnName];
      if (typeof fn_ === "function") {
        return (fn_ as (s: string) => string)(ctxJson);
      }
      throw new Error(`Macro ${fnName} not found in ${ctx.module_path}`);
    };

    transformer.setupExternalMacros(resolveDecoratorNames, runMacro);
  }

  cached = transformer;
  return transformer;
}
