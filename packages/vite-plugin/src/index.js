/**
 * @module @macroforge/vite-plugin
 *
 * Vite plugin for Macroforge compile-time TypeScript macro expansion.
 *
 * This plugin integrates Macroforge's Rust-based macro expander into the Vite build pipeline,
 * enabling compile-time code generation through `@derive` decorators. It processes TypeScript
 * files during the build, expands macros, generates type definitions, and emits metadata.
 *
 * All configuration is loaded from `macroforge.config.js` (or .ts/.mjs/.cjs).
 * Vite-specific options can be set under the `vite` key in the config file.
 *
 * @example
 * ```typescript
 * // vite.config.ts
 * import { defineConfig } from 'vite';
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
 *     generateTypes: true,        // Generate .d.ts files (default: true)
 *     typesOutputDir: ".macroforge/types",  // Types output dir (default: ".macroforge/types")
 *     emitMetadata: true,         // Emit metadata JSON (default: true)
 *     metadataOutputDir: ".macroforge/meta", // Metadata output dir (default: ".macroforge/meta")
 *     devCache: true,             // Disk cache for dev mode (default: true)
 *   },
 * };
 * ```
 *
 * @packageDocumentation
 */

import { createRequire } from "node:module";
import { createHash } from "node:crypto";
import * as fs from "node:fs";
import * as os from "node:os";
import * as path from "node:path";
import { encode as encodeVlq } from "@jridgewell/sourcemap-codec";
import {
  collectExternalDecoratorModules,
  hasMacroAnnotations,
  loadMacroConfig,
} from "@macroforge/shared";

/**
 * Precompute a line-offset table for `source`: `lineStarts[i]` is the
 * byte offset where line `i` (0-indexed) begins. Used to convert byte
 * offsets to (line, column) in O(log n) per lookup instead of scanning
 * the full source each time.
 *
 * @param {string} source
 * @returns {number[]}
 */
function buildLineStarts(source) {
  const starts = [0];
  for (let i = 0; i < source.length; i++) {
    if (source.charCodeAt(i) === 10 /* \n */) {
      starts.push(i + 1);
    }
  }
  return starts;
}

/**
 * Convert a 0-based byte offset to (line, column) using a precomputed
 * line-starts table. Both line and column are 0-based as required by
 * Source Map v3.
 *
 * @param {number} offset
 * @param {number[]} lineStarts
 * @returns {[number, number]}
 */
function offsetToLineColumn(offset, lineStarts) {
  // Binary search for the largest lineStarts[i] that is <= offset.
  let lo = 0;
  let hi = lineStarts.length - 1;
  while (lo < hi) {
    const mid = (lo + hi + 1) >>> 1;
    if (lineStarts[mid] <= offset) {
      lo = mid;
    } else {
      hi = mid - 1;
    }
  }
  return [lo, offset - lineStarts[lo]];
}

/**
 * Convert a macroforge `SourceMappingResult` into a Source Map v3 JSON
 * object suitable for return from Vite's `transform` hook.
 *
 * The engine's `SourceMapping` tracks segments of the form
 * `{ original_start, original_end, expanded_start, expanded_end }` —
 * byte-offset ranges in the original and expanded source. Source Map
 * v3 wants `(generated_line, generated_column, source_index,
 * original_line, original_column)` per-segment tuples, VLQ-encoded.
 *
 * The original offsets are in the pre-expansion source (`originalCode`),
 * the expanded offsets are in the post-expansion source (`expandedCode`).
 * Note that the engine's offsets are 0-based from a patch-applicator
 * standpoint even though `SpanIR` uses 1-based storage internally —
 * `SourceMappingResult` emits 0-based values across the ABI boundary.
 *
 * We only emit one source entry (`sources: [sourcePath]`). If other
 * plugins in the chain produced maps, Vite composes them automatically.
 *
 * @param {{ segments: Array<{ originalStart: number, originalEnd: number, expandedStart: number, expandedEnd: number }> }} mapping
 * @param {string} sourcePath
 * @param {string} originalCode
 * @param {string} expandedCode
 * @returns {{ version: 3, sources: string[], sourcesContent: string[], mappings: string, names: string[] } | null}
 */
function sourceMappingToV3(mapping, sourcePath, originalCode, expandedCode) {
  if (
    !mapping ||
    !Array.isArray(mapping.segments) ||
    mapping.segments.length === 0
  ) {
    return null;
  }

  const originalLineStarts = buildLineStarts(originalCode);
  const expandedLineStarts = buildLineStarts(expandedCode);

  // Per-line buckets: index = generated line, value = array of
  // unencoded 5-tuples sorted by generatedColumn.
  /** @type {Array<Array<[number, number, number, number]>>} */
  const lines = [];

  for (const seg of mapping.segments) {
    const [genLine, genCol] = offsetToLineColumn(
      seg.expandedStart,
      expandedLineStarts,
    );
    const [origLine, origCol] = offsetToLineColumn(
      seg.originalStart,
      originalLineStarts,
    );

    while (lines.length <= genLine) lines.push([]);
    lines[genLine].push([genCol, 0, origLine, origCol]);
  }

  // Sort each line's segments by generated column.
  for (const line of lines) {
    line.sort((a, b) => a[0] - b[0]);
  }

  // Encode with @jridgewell/sourcemap-codec. The encoder takes a
  // nested array of [[genCol, srcIdx, origLine, origCol], ...] per
  // line and returns the VLQ-encoded mappings string.
  const mappings = encodeVlq(lines);

  return {
    version: 3,
    sources: [sourcePath],
    sourcesContent: [originalCode],
    mappings,
    names: [],
  };
}

/** @type {typeof import('typescript') | undefined} */
let tsModule;
let tsModuleResolved = false;

/**
 * Lazily resolves TypeScript, trying the project root first (so the consuming
 * project's copy is found) and falling back to the plugin's own location.
 */
function ensureTypeScript() {
  if (tsModuleResolved) return tsModule;
  tsModuleResolved = true;

  // Try resolving from the project root (cwd) first, then from the plugin
  const roots = [
    process.cwd() + "/",
    import.meta.url,
  ];
  for (const root of roots) {
    try {
      const req = createRequire(root);
      tsModule = req("typescript");
      return tsModule;
    } catch {
      // continue to next root
    }
  }

  tsModule = undefined;
  console.warn(
    "[@macroforge/vite-plugin] TypeScript not found. Generated .d.ts files will be skipped.",
  );
  return tsModule;
}

/** @type {Map<string, import('typescript').CompilerOptions>} */
const compilerOptionsCache = new Map();

/** @type {NodeJS.Require | undefined} */
let cachedRequire;

/**
 * Ensures that `require()` is available in the current execution context.
 * @returns {Promise<NodeRequire>}
 * @internal
 */
async function ensureRequire() {
  if (typeof require !== "undefined") {
    return require;
  }

  if (!cachedRequire) {
    const { createRequire } = await import("node:module");
    cachedRequire =
      /** @type {NodeJS.Require} */ (createRequire(process.cwd() + "/"));
    // @ts-ignore - Expose on globalThis so Deno's CJS compat layer can use it
    globalThis.require = cachedRequire;
  }

  return cachedRequire;
}

/**
 * Retrieves and normalizes TypeScript compiler options for declaration emission.
 * @param {string} projectRoot - The project root directory
 * @returns {import('typescript').CompilerOptions | undefined}
 * @internal
 */
function getCompilerOptions(projectRoot) {
  ensureTypeScript();
  if (!tsModule) {
    return undefined;
  }
  const cached = compilerOptionsCache.get(projectRoot);
  if (cached) {
    return cached;
  }

  /** @type {string | undefined} */
  let configPath;
  try {
    configPath = tsModule.findConfigFile(
      projectRoot,
      tsModule.sys.fileExists,
      "tsconfig.json",
    );
  } catch {
    configPath = undefined;
  }

  /** @type {import('typescript').CompilerOptions} */
  let options;
  if (configPath) {
    const configFile = tsModule.readConfigFile(
      configPath,
      tsModule.sys.readFile,
    );
    if (configFile.error) {
      const formatted = tsModule.formatDiagnosticsWithColorAndContext(
        [configFile.error],
        {
          getCurrentDirectory: () => projectRoot,
          getCanonicalFileName: (fileName) => fileName,
          getNewLine: () => tsModule.sys.newLine,
        },
      );
      console.warn(
        `[@macroforge/vite-plugin] Failed to read tsconfig at ${configPath}\n${formatted}`,
      );
      options = {};
    } else {
      const parsed = tsModule.parseJsonConfigFileContent(
        configFile.config,
        tsModule.sys,
        path.dirname(configPath),
      );
      options = parsed.options;
    }
  } else {
    options = {};
  }

  // Normalize options for declaration-only emission
  /** @type {import('typescript').CompilerOptions} */
  const normalized = {
    ...options,
    declaration: true,
    emitDeclarationOnly: true,
    noEmitOnError: false,
    incremental: false,
  };

  // Remove output path options to allow programmatic control
  delete normalized.outDir;
  delete normalized.outFile;

  // Apply sensible defaults for modern TypeScript projects
  normalized.moduleResolution ??= tsModule.ModuleResolutionKind.Bundler;
  normalized.module ??= tsModule.ModuleKind.ESNext;
  normalized.target ??= tsModule.ScriptTarget.ESNext;
  normalized.strict ??= true;
  normalized.skipLibCheck ??= true;

  compilerOptionsCache.set(projectRoot, normalized);
  return normalized;
}

/**
 * Generates TypeScript declaration files from in-memory source code.
 * @param {string} code - The macro-expanded TypeScript source code
 * @param {string} fileName - The original file path
 * @param {string} projectRoot - The project root directory
 * @returns {string | undefined}
 * @internal
 */
function emitDeclarationsFromCode(code, fileName, projectRoot) {
  ensureTypeScript();
  if (!tsModule) {
    return undefined;
  }

  const compilerOptions = getCompilerOptions(projectRoot);
  if (!compilerOptions) {
    return undefined;
  }

  const normalizedFileName = path.resolve(fileName);
  const sourceText = code;
  const compilerHost = tsModule.createCompilerHost(compilerOptions, true);

  // Override getSourceFile to serve in-memory code for the target file
  compilerHost.getSourceFile = (requestedFileName, languageVersion) => {
    if (path.resolve(requestedFileName) === normalizedFileName) {
      return tsModule.createSourceFile(
        requestedFileName,
        sourceText,
        languageVersion,
        true,
      );
    }
    const text = tsModule.sys.readFile(requestedFileName);
    return text !== undefined
      ? tsModule.createSourceFile(
        requestedFileName,
        text,
        languageVersion,
        true,
      )
      : undefined;
  };

  // Override readFile to serve in-memory code for the target file
  compilerHost.readFile = (requestedFileName) => {
    return path.resolve(requestedFileName) === normalizedFileName
      ? sourceText
      : tsModule.sys.readFile(requestedFileName);
  };

  // Override fileExists to report the virtual file as existing
  compilerHost.fileExists = (requestedFileName) => {
    return (
      path.resolve(requestedFileName) === normalizedFileName ||
      tsModule.sys.fileExists(requestedFileName)
    );
  };

  // Capture emitted declaration content
  /** @type {string | undefined} */
  let output;
  const writeFile = (
    /** @type {string} */ outputName,
    /** @type {string} */ text,
  ) => {
    if (outputName.endsWith(".d.ts")) {
      output = text;
    }
  };

  const program = tsModule.createProgram(
    [normalizedFileName],
    compilerOptions,
    compilerHost,
  );
  const emitResult = program.emit(undefined, writeFile, undefined, true);

  // Log diagnostics if emission was skipped due to errors
  if (emitResult.emitSkipped && emitResult.diagnostics.length > 0) {
    const formatted = tsModule.formatDiagnosticsWithColorAndContext(
      emitResult.diagnostics,
      {
        getCurrentDirectory: () => projectRoot,
        getCanonicalFileName: (fileName) => fileName,
        getNewLine: () => tsModule.sys.newLine,
      },
    );
    console.warn(
      `[@macroforge/vite-plugin] Declaration emit failed for ${
        path.relative(
          projectRoot,
          fileName,
        )
      }\n${formatted}`,
    );
    return undefined;
  }

  return output;
}

/**
 * Creates a Vite plugin for Macroforge compile-time macro expansion.
 *
 * Configuration is loaded from `macroforge.config.js` (or .ts/.mjs/.cjs).
 * Vite-specific options can be set under the `vite` key in the config file.
 *
 * @return {Promise<import('vite').Plugin>}
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
 *     generateTypes: true,
 *     typesOutputDir: ".macroforge/types",
 *     emitMetadata: true,
 *     metadataOutputDir: ".macroforge/meta",
 *   },
 * };
 * ```
 */
export async function macroforge() {
  /**
   * Reference to the loaded Macroforge Rust binary module.
   * @type {{ expandSync: Function, loadConfig?: (content: string, filepath: string) => any, scanProjectSync?: Function, invalidateScanCacheEntry?: (path: string) => boolean, clearScanCache?: () => void } | undefined}
   */
  let rustTransformer;

  /**
   * Cached type registry JSON from project scanning.
   * Built during `buildStart` and passed to every `expandSync` call.
   * @type {string | undefined}
   */
  let typeRegistryJson;

  /**
   * Cached declarative macro registry JSON from project scanning.
   * Enables cross-file `/** import macro { $name } from "./file" *\/`
   * resolution. Loaded from `.macroforge/declarative-registry.json` which
   * is written by `macroforge watch` / `ensureTypeRegistryCache`.
   * @type {string | undefined}
   */
  let declarativeRegistryJson;

  // Load the Rust binary first
  try {
    const projectRequire = createRequire(process.cwd() + "/");
    rustTransformer = projectRequire("macroforge");

    // Register external macro callbacks for WASM builds.
    // The WASM build cannot spawn Node subprocesses to resolve external macros,
    // so we provide JS-side resolve/run callbacks. No-op for NAPI builds.
    if (rustTransformer.setupExternalMacros) {
      const req = createRequire(process.cwd() + "/package.json");

      const resolveDecoratorNames = function (packagePath) {
        const pkg = req(packagePath);
        const names = [];
        if (pkg.__macroforgeGetManifest) {
          names.push(
            ...(pkg.__macroforgeGetManifest().decorators || []).map(
              (d) => d.export,
            ),
          );
        }
        for (const key of Object.keys(pkg)) {
          if (
            key.startsWith("__macroforgeGetManifest_") &&
            typeof pkg[key] === "function"
          ) {
            names.push(
              ...(pkg[key]().decorators || []).map((d) => d.export),
            );
          }
        }
        if (names.length > 0) return [...new Set(names)];
        return [];
      };

      const runMacro = function (ctxJson) {
        const ctx = JSON.parse(ctxJson);
        const fnName = `__macroforgeRun${ctx.macro_name}`;
        const pkg = req(ctx.module_path);
        const fn_ = pkg?.[fnName] || pkg?.default?.[fnName];
        if (typeof fn_ === "function") return fn_(ctxJson);
        throw new Error(`Macro ${fnName} not found in ${ctx.module_path}`);
      };

      rustTransformer.setupExternalMacros(resolveDecoratorNames, runMacro);
    }
  } catch (error) {
    console.warn(
      "[@macroforge/vite-plugin] Rust binary not found. Please run `npm run build:rust` first.",
    );
    console.warn(error);
  }

  // Load config upfront (passing Rust transformer for foreign type parsing)
  const macroConfig = loadMacroConfig(
    process.cwd(),
    rustTransformer?.loadConfig,
  );

  if (macroConfig.hasForeignTypes) {
    console.log(
      "[@macroforge/vite-plugin] Loaded config with foreign types from:",
      macroConfig.configPath,
    );
  }

  // Vite options resolved from config (with defaults)
  /** @type {boolean} */
  let generateTypes = true;
  /** @type {string} */
  let typesOutputDir = ".macroforge/types";
  /** @type {boolean} */
  let emitMetadata = true;
  /** @type {string} */
  let metadataOutputDir = ".macroforge/meta";
  /** @type {boolean} */
  let devCacheEnabled = true;

  // Load vite-specific options from the config file
  if (macroConfig.configPath) {
    try {
      const configModule = await import(macroConfig.configPath);
      const userConfig = configModule.default || configModule;
      const viteConfig = userConfig.vite;

      if (viteConfig) {
        if (viteConfig.generateTypes !== undefined) {
          generateTypes = viteConfig.generateTypes;
        }
        if (viteConfig.typesOutputDir !== undefined) {
          typesOutputDir = viteConfig.typesOutputDir;
        }
        if (viteConfig.emitMetadata !== undefined) {
          emitMetadata = viteConfig.emitMetadata;
        }
        if (viteConfig.metadataOutputDir !== undefined) {
          metadataOutputDir = viteConfig.metadataOutputDir;
        }
        if (viteConfig.devCache !== undefined) {
          devCacheEnabled = viteConfig.devCache;
        }
      }
    } catch (error) {
      throw new Error(
        `[@macroforge/vite-plugin] Failed to load config from ${macroConfig.configPath}: ${error.message}`,
      );
    }
  }

  /** @type {string} */
  let projectRoot;

  // --- Dev cache state ---
  /** @type {boolean} */
  let isDevMode = false;
  /** @type {string | undefined} */
  let cacheDir;
  /** @type {{ version: string, configHash: string, entries: Record<string, { sourceHash: string, hasMacros: boolean }> } | null} */
  let cacheManifest = null;
  /** @type {string} */
  let macroforgeVersion = "unknown";
  /** @type {boolean} */
  let cacheManifestDirty = false;
  /** @type {ReturnType<typeof setTimeout> | undefined} */
  let manifestFlushTimer;

  /**
   * Ensures a directory exists, creating it recursively if necessary.
   * @param {string} dir
   */
  function ensureDir(dir) {
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true });
    }
  }

  // --- Dev cache helpers ---

  /**
   * Computes SHA-256 hash of a string, returned as hex.
   * @param {string} content
   * @returns {string}
   */
  function contentHash(content) {
    return createHash("sha256").update(content).digest("hex");
  }

  /**
   * Reads the installed macroforge NAPI package version.
   * Resolves the module's main entry point, then reads package.json
   * from the same directory (avoids exports-map restrictions).
   * @returns {string}
   */
  function getMacroforgeVersion() {
    const req = createRequire(process.cwd() + "/");
    try {
      return JSON.parse(
        fs.readFileSync(req.resolve("macroforge/package.json"), "utf-8"),
      ).version;
    } catch { /* exports map may block ./package.json */ }
    try {
      return JSON.parse(
        fs.readFileSync(
          path.join(process.cwd(), "node_modules", "macroforge", "package.json"),
          "utf-8",
        ),
      ).version;
    } catch { /* not in local node_modules */ }
    return "unknown";
  }

  /**
   * Computes a hash over external macro package binaries (`.node`, `.wasm`)
   * so the cache invalidates when a local macro package is rebuilt.
   * @returns {string}
   */
  function getExternalMacroHash() {
    const nodeModules = path.join(projectRoot || process.cwd(), "node_modules");
    if (!fs.existsSync(nodeModules)) return "none";

    // Collect path:size:mtime_seconds parts, sort for deterministic ordering
    // (readdir order varies across Node/Deno/Rust), then hash.
    const parts = [];

    const checkPackage = (pkgDir) => {
      const indexJs = path.join(pkgDir, "index.js");
      try {
        const content = fs.readFileSync(indexJs, "utf-8");
        if (!content.includes("__macroforgeRun")) return;
      } catch {
        return;
      }
      try {
        for (const entry of fs.readdirSync(pkgDir)) {
          const ext = path.extname(entry);
          if (ext === ".node" || ext === ".wasm" || entry === "index.js") {
            const full = path.join(pkgDir, entry);
            try {
              const stat = fs.statSync(full);
              parts.push(
                `${full}:${stat.size}:${Math.floor(stat.mtimeMs / 1000)}`,
              );
            } catch { /* expected */ }
          }
        }
      } catch { /* expected */ }
    };

    try {
      for (const entry of fs.readdirSync(nodeModules)) {
        const full = path.join(nodeModules, entry);
        if (!fs.statSync(full).isDirectory()) continue;
        if (entry.startsWith("@")) {
          try {
            for (const sub of fs.readdirSync(full)) {
              const subFull = path.join(full, sub);
              if (fs.statSync(subFull).isDirectory()) checkPackage(subFull);
            }
          } catch { /* expected */ }
        } else if (!entry.startsWith(".")) {
          checkPackage(full);
        }
      }
    } catch { /* expected */ }

    if (parts.length === 0) return "none";
    parts.sort();
    const hasher = createHash("sha256");
    for (const part of parts) {
      hasher.update(part);
    }
    return hasher.digest("hex");
  }

  /**
   * Computes a hash of the macroforge config file for cache invalidation.
   * @returns {string}
   */
  function getConfigHash() {
    if (macroConfig.configPath) {
      try {
        return contentHash(fs.readFileSync(macroConfig.configPath, "utf-8"));
      } catch {
        // config file disappeared
      }
    }
    return "none";
  }

  /**
   * Loads and validates the cache manifest from disk.
   * Returns null if the cache is stale (version or config mismatch).
   * @returns {{ version: string, configHash: string, entries: Record<string, { sourceHash: string, hasMacros: boolean }> } | null}
   */
  function loadCacheManifest() {
    const manifestPath = path.join(cacheDir, "manifest.json");
    if (!fs.existsSync(manifestPath)) return null;

    try {
      const manifest = JSON.parse(fs.readFileSync(manifestPath, "utf-8"));

      if (manifest.version !== macroforgeVersion) {
        console.log(
          "[@macroforge/vite-plugin] Cache invalidated: macroforge version changed",
        );
        return null;
      }

      const currentConfigHash = getConfigHash();
      if (manifest.configHash !== currentConfigHash) {
        console.log(
          "[@macroforge/vite-plugin] Cache invalidated: config changed",
        );
        return null;
      }

      // Reject caches built with --builtin-only since they may lack external macro expansions
      if (manifest.builtinOnly) {
        console.log(
          "[@macroforge/vite-plugin] Cache invalidated: built with --builtin-only (run without --builtin-only for full expansion)",
        );
        return null;
      }

      const currentExternalHash = getExternalMacroHash();
      if (
        manifest.externalMacroHash &&
        manifest.externalMacroHash !== currentExternalHash
      ) {
        console.log(
          "[@macroforge/vite-plugin] Cache invalidated: external macro binary changed",
        );
        return null;
      }

      return manifest;
    } catch {
      return null;
    }
  }

  /**
   * Reads a cached expansion result for a source file.
   * @param {string} id - Absolute file path
   * @param {string} code - Current source code content
   * @returns {{ code: string } | null}
   */
  function readCacheEntry(id, code) {
    if (!cacheManifest || !cacheDir) return null;

    const relPath = path.relative(projectRoot, id);
    const entry = cacheManifest.entries[relPath];
    if (!entry || !entry.hasMacros) return null;

    const currentHash = contentHash(code);
    if (entry.sourceHash !== currentHash) return null;

    const cachePath = path.join(cacheDir, relPath + ".cache");
    try {
      const expandedCode = fs.readFileSync(cachePath, "utf-8");
      return { code: expandedCode };
    } catch {
      return null;
    }
  }

  /**
   * Writes a cache entry after macro expansion.
   * Only caches files that actually had macros expanded.
   * @param {string} id - Absolute file path
   * @param {string} sourceCode - Original source code
   * @param {string} expandedCode - Expanded code from rustTransformer
   * @param {boolean} hasMacros - Whether the file actually had macros expanded
   */
  function writeCacheEntry(id, sourceCode, expandedCode, hasMacros) {
    if (!cacheDir) return;

    const relPath = path.relative(projectRoot, id);

    try {
      // Only write .cache files for files that actually have macros
      if (hasMacros) {
        const cachePath = path.join(cacheDir, relPath + ".cache");
        ensureDir(path.dirname(cachePath));
        fs.writeFileSync(cachePath, expandedCode, "utf-8");
      }

      if (!cacheManifest) {
        cacheManifest = {
          version: macroforgeVersion,
          configHash: getConfigHash(),
          externalMacroHash: getExternalMacroHash(),
          entries: {},
        };
      }

      cacheManifest.entries[relPath] = {
        sourceHash: contentHash(sourceCode),
        hasMacros,
      };

      // Debounce manifest writes — don't write 59KB JSON on every file
      cacheManifestDirty = true;
      if (manifestFlushTimer) clearTimeout(manifestFlushTimer);
      manifestFlushTimer = setTimeout(flushCacheManifest, 500);
    } catch (error) {
      console.warn(
        `[@macroforge/vite-plugin] Failed to write cache for ${relPath}:`,
        error.message,
      );
    }
  }

  /**
   * Flushes the dirty cache manifest to disk.
   */
  function flushCacheManifest() {
    if (!cacheManifestDirty || !cacheManifest || !cacheDir) return;
    try {
      ensureDir(cacheDir);
      fs.writeFileSync(
        path.join(cacheDir, "manifest.json"),
        JSON.stringify(cacheManifest, null, 2),
        "utf-8",
      );
      cacheManifestDirty = false;
    } catch (error) {
      console.warn(
        `[@macroforge/vite-plugin] Failed to write cache manifest:`,
        error.message,
      );
    }
  }

  /**
   * Writes generated TypeScript declaration files to the configured output directory.
   * @param {string} id - The absolute path of the source file
   * @param {string} types - The generated declaration file content
   */
  function writeTypeDefinitions(id, types) {
    const relativePath = path.relative(projectRoot, id);
    const parsed = path.parse(relativePath);
    const outputBase = path.join(projectRoot, typesOutputDir, parsed.dir);
    ensureDir(outputBase);
    const targetPath = path.join(outputBase, `${parsed.name}.d.ts`);

    try {
      const existing = fs.existsSync(targetPath)
        ? fs.readFileSync(targetPath, "utf-8")
        : null;
      if (existing !== types) {
        fs.writeFileSync(targetPath, types, "utf-8");
        console.log(
          `[@macroforge/vite-plugin] Wrote types for ${relativePath} -> ${
            path.relative(projectRoot, targetPath)
          }`,
        );
      }
    } catch (error) {
      console.error(
        `[@macroforge/vite-plugin] Failed to write type definitions for ${id}:`,
        error,
      );
    }
  }

  /**
   * Writes macro intermediate representation (IR) metadata to JSON files.
   * @param {string} id - The absolute path of the source file
   * @param {string} metadata - The macro IR metadata as a JSON string
   */
  function writeMetadata(id, metadata) {
    const relativePath = path.relative(projectRoot, id);
    const parsed = path.parse(relativePath);
    const outputBase = path.join(projectRoot, metadataOutputDir, parsed.dir);
    ensureDir(outputBase);
    const targetPath = path.join(outputBase, `${parsed.name}.macro-ir.json`);

    try {
      const existing = fs.existsSync(targetPath)
        ? fs.readFileSync(targetPath, "utf-8")
        : null;
      if (existing !== metadata) {
        fs.writeFileSync(targetPath, metadata, "utf-8");
        console.log(
          `[@macroforge/vite-plugin] Wrote metadata for ${relativePath} -> ${
            path.relative(projectRoot, targetPath)
          }`,
        );
      }
    } catch (error) {
      console.error(
        `[@macroforge/vite-plugin] Failed to write metadata for ${id}:`,
        error,
      );
    }
  }

  /**
   * Formats transformation errors into user-friendly messages.
   * @param {unknown} error
   * @param {string} id
   * @returns {string}
   */
  function formatTransformError(error, id) {
    const relative = projectRoot ? path.relative(projectRoot, id) || id : id;
    if (error instanceof Error) {
      const details = error.stack && error.stack.includes(error.message)
        ? error.stack
        : `${error.message}\n${error.stack ?? ""}`;
      return `[@macroforge/vite-plugin] Failed to transform ${relative}\n${details}`
        .trim();
    }
    return `[@macroforge/vite-plugin] Failed to transform ${relative}: ${
      String(error)
    }`;
  }

  /** @type {import('vite').Plugin} */
  const plugin = {
    name: "@macroforge/vite-plugin",
    enforce: "pre",

    /**
     * @param {{ root: string, command: string }} config
     */
    configResolved(config) {
      projectRoot = config.root;
      isDevMode = config.command === "serve";

      if (isDevMode && devCacheEnabled) {
        cacheDir = path.join(projectRoot, ".macroforge", "cache");
        macroforgeVersion = getMacroforgeVersion();
        cacheManifest = loadCacheManifest();

        if (cacheManifest) {
          const entryCount = Object.keys(cacheManifest.entries).length;
          console.log(
            `[@macroforge/vite-plugin] Dev cache loaded: ${entryCount} entries`,
          );
        }
      }
    },

    /**
     * Load the type registry from the CLI cache for compile-time type awareness.
     * The registry is passed to every expandSync call so macros can introspect
     * any type in the project.
     */
    buildStart() {
      const localRegistry = path.join(
        projectRoot,
        ".macroforge",
        "type-registry.json",
      );
      if (fs.existsSync(localRegistry)) {
        typeRegistryJson = fs.readFileSync(localRegistry, "utf-8");
        try {
          const parsed = JSON.parse(typeRegistryJson);
          const count = Object.keys(parsed.types ?? parsed).length;
          console.log(
            `[@macroforge/vite-plugin] Type registry loaded: ${count} types`,
          );
        } catch {
          // JSON is passed as-is to expandSync, no need to parse here
        }
      } else {
        console.warn(
          `[@macroforge/vite-plugin] No type registry found at .macroforge/type-registry.json. Run \`macroforge watch\` to generate it.`,
        );
      }

      // Load the declarative macro registry alongside the type registry.
      // Produced by the same project scan, so if one exists the other
      // almost certainly does too. Missing file is a no-op: cross-file
      // declarative macro imports simply won't resolve.
      const localDeclarativeRegistry = path.join(
        projectRoot,
        ".macroforge",
        "declarative-registry.json",
      );
      if (fs.existsSync(localDeclarativeRegistry)) {
        declarativeRegistryJson = fs.readFileSync(
          localDeclarativeRegistry,
          "utf-8",
        );
        try {
          const parsed = JSON.parse(declarativeRegistryJson);
          const fileCount = Object.keys(parsed.by_file ?? {}).length;
          if (fileCount > 0) {
            console.log(
              `[@macroforge/vite-plugin] Declarative macro registry loaded: ${fileCount} file(s)`,
            );
          }
        } catch {
          // JSON is passed as-is to expandSync.
        }
      }
    },

    /**
     * Resolve `.svelte` imports to `.svelte.ts` when the `.svelte` file
     * does not exist. Macroforge type files use the `.svelte.ts` extension
     * (Svelte 5 runes modules) but are imported with just `.svelte`.
     *
     * @param {string} source
     * @param {string | undefined} importer
     * @param {object} options
     */
    async resolveId(source, importer, options) {
      if (!source.endsWith(".svelte") || !importer) return null;

      // Let other plugins (SvelteKit, etc.) try to resolve it first
      const resolved = await this.resolve(source, importer, {
        ...options,
        skipSelf: true,
      });

      if (resolved && !resolved.external) return resolved;

      // Fall back: try appending .ts
      const resolvedTs = await this.resolve(source + ".ts", importer, {
        ...options,
        skipSelf: true,
      });

      return resolvedTs || null;
    },

    /**
     * @param {string} code
     * @param {string} id
     */
    async transform(code, id) {
      // Only transform TypeScript files
      if (!id.endsWith(".ts") && !id.endsWith(".tsx")) {
        return null;
      }

      // Skip node_modules by default
      if (id.includes("node_modules")) {
        return null;
      }

      // Skip already-expanded files
      if (id.includes(".expanded.")) {
        return null;
      }

      // Check if Rust transformer is available
      if (!rustTransformer || !rustTransformer.expandSync) {
        return null;
      }

      // Quick check: skip files without a real @derive directive
      if (!hasMacroAnnotations(code)) {
        return null;
      }

      try {
        // --- Dev cache read ---
        if (isDevMode && devCacheEnabled && cacheManifest) {
          const cached = readCacheEntry(id, code);
          if (cached) {
            let cachedCode = cached.code;

            cachedCode = cachedCode.replace(
              /\/\*\*\s*import\s+macro[\s\S]*?\*\/\s*/gi,
              "",
            );
            if (id.endsWith(".svelte.ts") || id.endsWith(".svelte.js")) {
              cachedCode = cachedCode.replace(
                /\/\*\*\s*@derive\b[^*]*\*\//g,
                "",
              );
            }

            return {
              code: cachedCode,
              map: null,
            };
          }
        }

        // Ensure require() is available for native module loading
        // Use the project's CWD-based require for resolving external macro packages
        const projectRequire = await ensureRequire();

        // Collect external decorator modules from macro imports
        // Use projectRequire to resolve packages from the project's CWD, not the plugin's location
        const externalDecoratorModules = collectExternalDecoratorModules(
          code,
          projectRequire,
        );

        // Perform macro expansion via the Rust binary
        const result = rustTransformer.expandSync(code, id, {
          keepDecorators: macroConfig.keepDecorators,
          externalDecoratorModules,
          configPath: macroConfig.configPath,
          typeRegistryJson,
          declarativeRegistryJson,
          // Reverse-monomorphization build mode. Dev (serve) runs all
          // declarative macros as if they were expand-only for precise
          // diagnostics; prod (build) emits share-mode helpers for
          // `share-only`, `share-anyway`, and `auto` macros.
          buildMode: isDevMode ? "dev" : "prod",
        });

        // Report diagnostics from macro expansion
        for (const diag of result.diagnostics) {
          if (diag.level === "error") {
            const message = `Macro error at ${id}:${diag.start ?? "?"}-${
              diag.end ?? "?"
            }: ${diag.message}`;
            /** @type {any} */ (this).error(message);
          } else {
            console.warn(
              `[@macroforge/vite-plugin] ${diag.level}: ${diag.message}`,
            );
          }
        }

        if (result && result.code) {
          // Check if macros were actually expanded
          const hasMacros = result.sourceMapping?.generatedRegions?.length > 0;

          // --- Dev cache write (self-populating) ---
          if (isDevMode && devCacheEnabled) {
            writeCacheEntry(id, code, result.code, hasMacros);
          }

          // Remove macro-only imports so SSR output doesn't load native bindings
          result.code = result.code.replace(
            /\/\*\*\s*import\s+macro[\s\S]*?\*\/\s*/gi,
            "",
          );

          // For .svelte.ts modules, strip @derive JSDoc comments to prevent
          // the Svelte preprocessor from re-expanding macros
          if (id.endsWith(".svelte.ts") || id.endsWith(".svelte.js")) {
            result.code = result.code.replace(
              /\/\*\*\s*@derive\b[^*]*\*\//g,
              "",
            );
          }

          // Generate type definitions if enabled
          if (generateTypes) {
            const emitted = emitDeclarationsFromCode(
              result.code,
              id,
              projectRoot,
            );
            if (emitted) {
              writeTypeDefinitions(id, emitted);
            }
          }

          // Write macro IR metadata if enabled
          if (emitMetadata && result.metadata) {
            writeMetadata(id, result.metadata);
          }

          // Convert macroforge's internal SourceMapping to a v3 map
          // that Vite / the downstream plugin chain understands.
          // Engine already produced the segments during patch
          // application; we only encode them here.
          const map = result.sourceMapping
            ? sourceMappingToV3(result.sourceMapping, id, code, result.code)
            : null;

          return {
            code: result.code,
            map,
          };
        }
      } catch (error) {
        // Re-throw Vite plugin errors to preserve plugin attribution
        if (error && typeof error === "object" && "plugin" in error) {
          throw error;
        }
        // Format and report other errors
        const message = formatTransformError(error, id);
        /** @type {any} */ (this).error(message);
      }

      return null;
    },

    /**
     * Phase 17 — HMR invalidation for the native scan cache.
     *
     * Vite fires `handleHotUpdate` on every file that changed on
     * disk. We forward the path to the Rust scanner's singleton
     * cache so the next call that reads `typeRegistryJson` /
     * `declarativeRegistryJson` sees fresh IR. For configuration
     * files we clear the whole cache since any cached entry may now
     * be stale.
     *
     * The hook returns `undefined` so Vite uses its default module-
     * graph invalidation logic — we're only piggy-backing on the
     * notification, not trying to control what reloads.
     *
     * @param {{ file: string, modules: any[] }} ctx
     */
    handleHotUpdate(ctx) {
      if (!rustTransformer) return;
      const file = ctx.file;
      // Config files invalidate the entire cache (matches the
      // `clear_cache` path in the Rust singleton).
      if (
        file.endsWith("macroforge.config.ts") ||
        file.endsWith("macroforge.config.js") ||
        file.endsWith("macroforge.config.mjs") ||
        file.endsWith("tsconfig.json")
      ) {
        if (typeof rustTransformer.clearScanCache === "function") {
          rustTransformer.clearScanCache();
        }
        return;
      }
      // Source file change — drop the single entry. The next
      // `scanProjectSync` call (either from buildStart or a future
      // HMR refresh) will re-parse it.
      if (typeof rustTransformer.invalidateScanCacheEntry === "function") {
        rustTransformer.invalidateScanCacheEntry(file);
      }
    },

    /**
     * Flush the cache manifest on server close.
     */
    buildEnd() {
      if (manifestFlushTimer) {
        clearTimeout(manifestFlushTimer);
        manifestFlushTimer = undefined;
      }
      flushCacheManifest();
    },
  };

  return plugin;
}

export default macroforge;
