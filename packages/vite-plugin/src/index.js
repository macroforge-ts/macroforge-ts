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
 *   },
 * };
 * ```
 *
 * @packageDocumentation
 */

import { createRequire } from "module";
import * as fs from "fs";
import * as path from "path";
import { collectExternalDecoratorModules, loadMacroConfig } from "@macroforge/shared";


const moduleRequire = createRequire(import.meta.url);

/** @type {typeof import('typescript') | undefined} */
let tsModule;
try {
  tsModule = moduleRequire("typescript");
} catch (error) {
  tsModule = undefined;
  console.warn(
    "[@macroforge/vite-plugin] TypeScript not found. Generated .d.ts files will be skipped."
  );
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
    const { createRequire } = await import("module");
    cachedRequire = /** @type {NodeJS.Require} */ (createRequire(process.cwd() + "/"));
    // @ts-ignore - Expose on globalThis so native runtime loaders can use it
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
      "tsconfig.json"
    );
  } catch {
    configPath = undefined;
  }

  /** @type {import('typescript').CompilerOptions} */
  let options;
  if (configPath) {
    const configFile = tsModule.readConfigFile(configPath, tsModule.sys.readFile);
    if (configFile.error) {
      const formatted = tsModule.formatDiagnosticsWithColorAndContext(
        [configFile.error],
        {
          getCurrentDirectory: () => projectRoot,
          getCanonicalFileName: (fileName) => fileName,
          getNewLine: () => tsModule.sys.newLine,
        }
      );
      console.warn(
        `[@macroforge/vite-plugin] Failed to read tsconfig at ${configPath}\n${formatted}`
      );
      options = {};
    } else {
      const parsed = tsModule.parseJsonConfigFileContent(
        configFile.config,
        tsModule.sys,
        path.dirname(configPath)
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
        true
      );
    }
    const text = tsModule.sys.readFile(requestedFileName);
    return text !== undefined
      ? tsModule.createSourceFile(requestedFileName, text, languageVersion, true)
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
  const writeFile = (/** @type {string} */ outputName, /** @type {string} */ text) => {
    if (outputName.endsWith(".d.ts")) {
      output = text;
    }
  };

  const program = tsModule.createProgram(
    [normalizedFileName],
    compilerOptions,
    compilerHost
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
      }
    );
    console.warn(
      `[@macroforge/vite-plugin] Declaration emit failed for ${path.relative(
        projectRoot,
        fileName
      )}\n${formatted}`
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
   * @type {{ expandSync: Function, loadConfig?: (content: string, filepath: string) => any } | undefined}
   */
  let rustTransformer;

  // Load the Rust binary first
  try {
    rustTransformer = moduleRequire("macroforge");
  } catch (error) {
    console.warn(
      "[@macroforge/vite-plugin] Rust binary not found. Please run `npm run build:rust` first."
    );
    console.warn(error);
  }

  // Load config upfront (passing Rust transformer for foreign type parsing)
  const macroConfig = loadMacroConfig(process.cwd(), rustTransformer?.loadConfig);

  if (macroConfig.hasForeignTypes) {
    console.log(
      "[@macroforge/vite-plugin] Loaded config with foreign types from:",
      macroConfig.configPath
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
      }
    } catch (error) {
      throw new Error(
        `[@macroforge/vite-plugin] Failed to load config from ${macroConfig.configPath}: ${error.message}`
      );
    }
  }

  /** @type {string} */
  let projectRoot;

  /**
   * Ensures a directory exists, creating it recursively if necessary.
   * @param {string} dir
   */
  function ensureDir(dir) {
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true });
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
          `[@macroforge/vite-plugin] Wrote types for ${relativePath} -> ${path.relative(projectRoot, targetPath)}`
        );
      }
    } catch (error) {
      console.error(
        `[@macroforge/vite-plugin] Failed to write type definitions for ${id}:`,
        error
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
          `[@macroforge/vite-plugin] Wrote metadata for ${relativePath} -> ${path.relative(projectRoot, targetPath)}`
        );
      }
    } catch (error) {
      console.error(
        `[@macroforge/vite-plugin] Failed to write metadata for ${id}:`,
        error
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
      const details =
        error.stack && error.stack.includes(error.message)
          ? error.stack
          : `${error.message}\n${error.stack ?? ""}`;
      return `[@macroforge/vite-plugin] Failed to transform ${relative}\n${details}`.trim();
    }
    return `[@macroforge/vite-plugin] Failed to transform ${relative}: ${String(error)}`;
  }

  /** @type {import('vite').Plugin} */
  const plugin = {
    name: "@macroforge/vite-plugin",
    enforce: "pre",

    /**
     * @param {{ root: string }} config
     */
    configResolved(config) {
      projectRoot = config.root;
    },

    /**
     * @param {string} code
     * @param {string} id
     */
    async transform(code, id) {
      // Ensure require() is available for native module loading
      await ensureRequire();

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

      try {
        // Collect external decorator modules from macro imports
        const externalDecoratorModules = collectExternalDecoratorModules(
          code,
          moduleRequire
        );

        // Perform macro expansion via the Rust binary
        const result = rustTransformer.expandSync(code, id, {
          keepDecorators: macroConfig.keepDecorators,
          externalDecoratorModules,
          configPath: macroConfig.configPath,
        });

        // Report diagnostics from macro expansion
        for (const diag of result.diagnostics) {
          if (diag.level === "error") {
            const message = `Macro error at ${id}:${diag.start ?? "?"}-${diag.end ?? "?"}: ${diag.message}`;
            /** @type {any} */ (this).error(message);
          } else {
            console.warn(
              `[@macroforge/vite-plugin] ${diag.level}: ${diag.message}`
            );
          }
        }

        if (result && result.code) {
          // Remove macro-only imports so SSR output doesn't load native bindings
          result.code = result.code.replace(
            /\/\*\*\s*import\s+macro[\s\S]*?\*\/\s*/gi,
            ""
          );

          // Generate type definitions if enabled
          if (generateTypes) {
            const emitted = emitDeclarationsFromCode(
              result.code,
              id,
              projectRoot
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
  };

  return plugin;
}

export default macroforge;
