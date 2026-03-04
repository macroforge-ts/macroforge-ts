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
import * as path from "node:path";
import {
  collectExternalDecoratorModules,
  loadMacroConfig,
} from "@macroforge/shared";

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
   * @type {{ expandSync: Function, loadConfig?: (content: string, filepath: string) => any, scanProjectSync?: Function } | undefined}
   */
  let rustTransformer;

  /**
   * Cached type registry JSON from project scanning.
   * Built during `buildStart` and passed to every `expandSync` call.
   * @type {string | undefined}
   */
  let typeRegistryJson;

  // Load the Rust binary first
  try {
    const projectRequire = createRequire(process.cwd() + "/");
    rustTransformer = projectRequire("macroforge");
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
    try {
      const req = createRequire(process.cwd() + "/");
      const mainPath = req.resolve("macroforge");
      const pkgDir = path.dirname(mainPath);
      const pkgJson = JSON.parse(
        fs.readFileSync(path.join(pkgDir, "package.json"), "utf-8"),
      );
      return pkgJson.version;
    } catch {
      return "unknown";
    }
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
     * Pre-scan the project to build a type registry for compile-time type awareness.
     * The registry is passed to every expandSync call so macros can introspect
     * any type in the project (Zig-style type awareness).
     */
    buildStart() {
      if (!rustTransformer || !rustTransformer.scanProjectSync) {
        return;
      }

      try {
        const scanStart = performance.now();
        const scanResult = rustTransformer.scanProjectSync(projectRoot, {
          exportedOnly: false,
        });
        const scanTime = (performance.now() - scanStart).toFixed(0);

        typeRegistryJson = scanResult.registryJson;

        console.log(
          `[@macroforge/vite-plugin] Type scan: ${scanResult.typesFound} types from ${scanResult.filesScanned} files (${scanTime}ms)`,
        );

        for (const diag of scanResult.diagnostics) {
          if (diag.level === "error") {
            console.error(
              `[@macroforge/vite-plugin] Scan error: ${diag.message}`,
            );
          }
        }
      } catch (error) {
        console.warn(
          `[@macroforge/vite-plugin] Type scan failed, macros will run without type awareness:`,
          error.message || error,
        );
        typeRegistryJson = undefined;
      }
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

      // Quick check: files without @derive can't have macros — skip entirely
      if (!code.includes("@derive")) {
        return null;
      }

      try {
        // --- Dev cache read ---
        if (isDevMode && devCacheEnabled && cacheManifest) {
          const cached = readCacheEntry(id, code);
          if (cached) {
            let cachedCode = cached.code;

            // Apply same post-processing as the normal path
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

            // Generate type definitions from cached expanded code
            if (generateTypes) {
              const emitted = emitDeclarationsFromCode(
                cachedCode,
                id,
                projectRoot,
              );
              if (emitted) {
                writeTypeDefinitions(id, emitted);
              }
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

          return {
            code: result.code,
            map: null,
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
