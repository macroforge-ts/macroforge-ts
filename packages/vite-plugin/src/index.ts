/**
 * @module @macroforge/vite-plugin
 *
 * Vite plugin for Macroforge compile-time TypeScript macro expansion.
 *
 * This plugin integrates Macroforge's Rust-based macro expander into the Vite build pipeline,
 * enabling compile-time code generation through `@derive` decorators. It processes TypeScript
 * files during the build, expands macros, generates type definitions, and emits metadata.
 *
 * @example
 * ```typescript
 * // vite.config.ts
 * import { defineConfig } from 'vite';
 * import macroforgePlugin from '@macroforge/vite-plugin';
 *
 * export default defineConfig({
 *   plugins: [
 *     macroforgePlugin({
 *       generateTypes: true,
 *       typesOutputDir: 'src/types/generated',
 *       emitMetadata: true,
 *     }),
 *   ],
 * });
 * ```
 *
 * @packageDocumentation
 */

import { Plugin } from "vite";
import { createRequire } from "module";
import * as fs from "fs";
import * as path from "path";
import type ts from "typescript";
import type { ExpandOptions, ExpandResult, MacroManifest } from "macroforge";

const moduleRequire = createRequire(import.meta.url);
let tsModule: typeof ts | undefined;
try {
  tsModule = moduleRequire("typescript") as typeof ts;
} catch (error) {
  tsModule = undefined;
  console.warn(
    "[@macroforge/vite-plugin] TypeScript not found. Generated .d.ts files will be skipped.",
  );
}

const compilerOptionsCache = new Map<string, ts.CompilerOptions>();
let cachedRequire: NodeJS.Require | undefined;

/**
 * Cache for external macro package manifests.
 * Maps package path to its manifest (or null if failed to load).
 */
const externalManifestCache = new Map<string, MacroManifest | null>();

/**
 * Parses macro import comments from TypeScript code.
 *
 * @remarks
 * Extracts macro names mapped to their source module paths from
 * `/** import macro { ... } from "package" * /` comments.
 *
 * @param text - The TypeScript source code to parse
 * @returns Map of macro names to their module paths
 *
 * @example
 * ```typescript
 * const text = `/** import macro {JSON, FieldController} from "@playground/macro"; * /`;
 * parseMacroImportComments(text);
 * // => Map { "JSON" => "@playground/macro", "FieldController" => "@playground/macro" }
 * ```
 *
 * @internal
 */
function parseMacroImportComments(text: string): Map<string, string> {
  const imports = new Map<string, string>();
  const pattern =
    /\/\*\*\s*import\s+macro\s*\{([^}]+)\}\s*from\s*["']([^"']+)["']/gi;
  let match: RegExpExecArray | null;

  while ((match = pattern.exec(text)) !== null) {
    const names = match[1]
      .split(",")
      .map((n) => n.trim())
      .filter(Boolean);
    const modulePath = match[2];
    for (const name of names) {
      imports.set(name, modulePath);
    }
  }
  return imports;
}

/**
 * Attempts to load the manifest from an external macro package.
 *
 * External macro packages (like `@playground/macro`) export their own
 * `__macroforgeGetManifest()` function that provides macro metadata
 * including descriptions.
 *
 * @param modulePath - The package path (e.g., "@playground/macro")
 * @returns The macro manifest, or null if loading failed
 *
 * @internal
 */
function getExternalManifest(modulePath: string): MacroManifest | null {
  if (externalManifestCache.has(modulePath)) {
    return externalManifestCache.get(modulePath) ?? null;
  }

  try {
    const pkg = moduleRequire(modulePath);
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
 * Collects decorator modules from external macro packages referenced in the code.
 *
 * @param code - The TypeScript source code to scan
 * @returns Array of decorator module names from external packages
 *
 * @internal
 */
function collectExternalDecoratorModules(code: string): string[] {
  const imports = parseMacroImportComments(code);
  const modulePaths = [...new Set(imports.values())];

  return modulePaths.flatMap((modulePath) => {
    const manifest = getExternalManifest(modulePath);
    return manifest?.decorators.map((d) => d.module) ?? [];
  });
}

/**
 * Ensures that `require()` is available in the current execution context.
 *
 * @remarks
 * This function handles the ESM/CommonJS interoperability problem. In pure ESM environments,
 * `require` is not defined, but some native modules (like the Macroforge Rust binary) may
 * depend on it being available. This function:
 *
 * 1. Returns the existing `require` if already available (CommonJS context)
 * 2. Creates a synthetic `require` using Node's `createRequire` API (ESM context)
 * 3. Exposes the created `require` on `globalThis` for native runtime loaders
 * 4. Caches the result to avoid redundant creation
 *
 * @returns A Promise resolving to a Node.js `require` function
 *
 * @internal
 */
async function ensureRequire(): Promise<NodeRequire> {
  if (typeof require !== "undefined") {
    return require;
  }

  if (!cachedRequire) {
    const { createRequire } = await import("module");
    cachedRequire = createRequire(
      process.cwd() + "/",
    ) as unknown as NodeJS.Require;
    // Expose on globalThis so native runtime loaders can use it
    (globalThis as any).require = cachedRequire;
  }

  return cachedRequire;
}

/**
 * Configuration options for the Macroforge Vite plugin.
 *
 * @public
 * @example
 * ```typescript
 * const options: NapiMacrosPluginOptions = {
 *   include: ['src/**\/*.ts'],
 *   exclude: ['**\/*.test.ts'],
 *   generateTypes: true,
 *   typesOutputDir: 'src/types/generated',
 *   emitMetadata: true,
 *   metadataOutputDir: 'src/macros/metadata',
 * };
 * ```
 */
export interface NapiMacrosPluginOptions {
  /**
   * Glob patterns, regular expressions, or arrays of either to specify which files
   * should be processed by the macro expander.
   *
   * @remarks
   * If not specified, all `.ts` and `.tsx` files (excluding `node_modules`) are processed.
   *
   * @example
   * ```typescript
   * include: ['src/**\/*.ts', /components\/.*\.tsx$/]
   * ```
   */
  include?: string | RegExp | (string | RegExp)[];

  /**
   * Glob patterns, regular expressions, or arrays of either to specify which files
   * should be excluded from macro processing.
   *
   * @remarks
   * Files in `node_modules` are always excluded by default.
   *
   * @example
   * ```typescript
   * exclude: ['**\/*.test.ts', '**\/*.spec.ts']
   * ```
   */
  exclude?: string | RegExp | (string | RegExp)[];

  /**
   * Whether to generate TypeScript declaration files (`.d.ts`) for transformed code.
   *
   * @remarks
   * When enabled, the plugin uses the TypeScript compiler to emit declaration files
   * based on the macro-expanded code. This ensures type definitions accurately reflect
   * the generated code.
   *
   * @default true
   */
  generateTypes?: boolean;

  /**
   * Output directory for generated TypeScript declaration files.
   *
   * @remarks
   * Path is relative to the project root. The directory structure of the source files
   * is preserved within this output directory.
   *
   * @default "src/macros/generated"
   *
   * @example
   * ```typescript
   * // Source: src/models/User.ts
   * // Output: src/types/generated/models/User.d.ts
   * typesOutputDir: 'src/types/generated'
   * ```
   */
  typesOutputDir?: string;

  /**
   * Whether to emit macro intermediate representation (IR) metadata as JSON files.
   *
   * @remarks
   * The metadata contains information about which macros were applied, their configurations,
   * and the transformation results. This can be useful for debugging, tooling integration,
   * or build analysis.
   *
   * @default true
   */
  emitMetadata?: boolean;

  /**
   * Output directory for macro IR metadata JSON files.
   *
   * @remarks
   * Path is relative to the project root. If not specified, defaults to the same
   * directory as `typesOutputDir`. Metadata files are named with a `.macro-ir.json` suffix.
   *
   * @default Same as `typesOutputDir`
   *
   * @example
   * ```typescript
   * // Source: src/models/User.ts
   * // Output: src/macros/metadata/models/User.macro-ir.json
   * metadataOutputDir: 'src/macros/metadata'
   * ```
   */
  metadataOutputDir?: string;
}

/**
 * Internal configuration loaded from `macroforge.json`.
 *
 * @remarks
 * This configuration controls macro expansion behavior at the project level.
 * The config file is searched for starting from the project root and traversing
 * up the directory tree until found or the filesystem root is reached.
 *
 * @internal
 */
interface MacroConfig {
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
}

/**
 * Loads Macroforge configuration from `macroforge.json`.
 *
 * @remarks
 * Searches for `macroforge.json` starting from `projectRoot` and traversing up the
 * directory tree until found or the filesystem root is reached. This allows monorepo
 * setups where the config may be at the workspace root rather than the package root.
 *
 * If the config file is found but cannot be parsed (invalid JSON), returns the
 * default configuration rather than throwing an error.
 *
 * @param projectRoot - The directory to start searching from (usually Vite's resolved root)
 *
 * @returns The loaded configuration, or a default config if no file is found
 *
 * @example
 * ```typescript
 * // macroforge.json
 * {
 *   "keepDecorators": true
 * }
 * ```
 *
 * @internal
 */
function loadMacroConfig(projectRoot: string): MacroConfig {
  let current = projectRoot;
  const fallback: MacroConfig = { keepDecorators: false };

  while (true) {
    const candidate = path.join(current, "macroforge.json");
    if (fs.existsSync(candidate)) {
      try {
        const raw = fs.readFileSync(candidate, "utf8");
        const parsed = JSON.parse(raw);
        return { keepDecorators: Boolean(parsed.keepDecorators) };
      } catch {
        return fallback;
      }
    }

    const parent = path.dirname(current);
    if (parent === current) break;
    current = parent;
  }

  return fallback;
}

/**
 * Retrieves and normalizes TypeScript compiler options for declaration emission.
 *
 * @remarks
 * This function reads the project's `tsconfig.json` and adjusts the compiler options
 * specifically for generating declaration files from macro-expanded code. The function:
 *
 * 1. Locates `tsconfig.json` using TypeScript's built-in config file discovery
 * 2. Parses and validates the configuration
 * 3. Normalizes options for declaration-only emission
 * 4. Caches results per project root for performance
 *
 * **Forced Options:**
 * - `declaration: true` - Enable declaration file output
 * - `emitDeclarationOnly: true` - Only emit `.d.ts` files, not JavaScript
 * - `noEmitOnError: false` - Continue emission even with type errors
 * - `incremental: false` - Disable incremental compilation
 *
 * **Default Options** (applied if not specified in tsconfig):
 * - `moduleResolution: Bundler` - Modern bundler-style resolution
 * - `module: ESNext` - ES module output
 * - `target: ESNext` - Latest ECMAScript target
 * - `strict: true` - Enable strict type checking
 * - `skipLibCheck: true` - Skip type checking of declaration files
 *
 * **Removed Options:**
 * - `outDir` and `outFile` - Removed to allow programmatic output control
 *
 * @param projectRoot - The project root directory to search for tsconfig.json
 *
 * @returns Normalized compiler options, or `undefined` if TypeScript is not available
 *
 * @internal
 */
function getCompilerOptions(
  projectRoot: string,
): ts.CompilerOptions | undefined {
  if (!tsModule) {
    return undefined;
  }
  const cached = compilerOptionsCache.get(projectRoot);
  if (cached) {
    return cached;
  }

  let configPath: string | undefined;
  try {
    configPath = tsModule.findConfigFile(
      projectRoot,
      tsModule.sys.fileExists,
      "tsconfig.json",
    );
  } catch {
    configPath = undefined;
  }

  let options: ts.CompilerOptions;
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
  const normalized: ts.CompilerOptions = {
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
 * Generates TypeScript declaration files (`.d.ts`) from in-memory source code.
 *
 * @remarks
 * This function creates a virtual TypeScript compilation environment to emit declaration
 * files from macro-expanded code. It sets up a custom compiler host that serves the
 * transformed source from memory while delegating other file operations to the filesystem.
 *
 * **Virtual Compiler Host:**
 * The custom compiler host intercepts file operations for the target file:
 * - `getSourceFile`: Returns the in-memory code for the target file, filesystem for others
 * - `readFile`: Returns the in-memory code for the target file, filesystem for others
 * - `fileExists`: Reports the target file as existing even though it's virtual
 *
 * This approach allows generating declarations for transformed code without writing
 * intermediate files to disk.
 *
 * **Error Handling:**
 * If declaration emission fails (e.g., due to type errors in the transformed code),
 * diagnostics are formatted and logged as warnings, and `undefined` is returned.
 *
 * @param code - The macro-expanded TypeScript source code
 * @param fileName - The original file path (used for module resolution and output naming)
 * @param projectRoot - The project root directory (used for diagnostic formatting)
 *
 * @returns The generated declaration file content, or `undefined` if emission failed
 *         or TypeScript is not available
 *
 * @internal
 */
function emitDeclarationsFromCode(
  code: string,
  fileName: string,
  projectRoot: string,
): string | undefined {
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
      return tsModule!.createSourceFile(
        requestedFileName,
        sourceText,
        languageVersion,
        true,
      );
    }
    const text = tsModule!.sys.readFile(requestedFileName);
    return text !== undefined
      ? tsModule!.createSourceFile(
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
      : tsModule!.sys.readFile(requestedFileName);
  };

  // Override fileExists to report the virtual file as existing
  compilerHost.fileExists = (requestedFileName) => {
    return (
      path.resolve(requestedFileName) === normalizedFileName ||
      tsModule!.sys.fileExists(requestedFileName)
    );
  };

  // Capture emitted declaration content
  let output: string | undefined;
  const writeFile = (outputName: string, text: string) => {
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
      `[@macroforge/vite-plugin] Declaration emit failed for ${path.relative(
        projectRoot,
        fileName,
      )}\n${formatted}`,
    );
    return undefined;
  }

  return output;
}

/**
 * Creates a Vite plugin for Macroforge compile-time macro expansion.
 *
 * @remarks
 * This is the main entry point for integrating Macroforge into a Vite build pipeline.
 * The plugin:
 *
 * 1. **Runs early** (`enforce: "pre"`) to transform code before other plugins
 * 2. **Processes TypeScript files** (`.ts` and `.tsx`) excluding `node_modules`
 * 3. **Expands macros** using the Macroforge Rust binary via `expandSync()`
 * 4. **Generates type definitions** for transformed code (optional, default: enabled)
 * 5. **Emits metadata** about macro transformations (optional, default: enabled)
 *
 * **Plugin Lifecycle:**
 * - `configResolved`: Initializes project root, loads config, and attempts to load Rust binary
 * - `transform`: Processes each TypeScript file through the macro expander
 *
 * **Error Handling:**
 * - If the Rust binary is not available, files pass through unchanged
 * - Macro expansion errors are reported via Vite's `this.error()` mechanism
 * - TypeScript emission errors are logged as warnings
 *
 * @param options - Plugin configuration options
 *
 * @returns A Vite plugin instance
 *
 * @public
 *
 * @example
 * ```typescript
 * // Basic usage
 * import macroforgePlugin from '@macroforge/vite-plugin';
 *
 * export default defineConfig({
 *   plugins: [macroforgePlugin()],
 * });
 * ```
 *
 * @example
 * ```typescript
 * // With custom options
 * import macroforgePlugin from '@macroforge/vite-plugin';
 *
 * export default defineConfig({
 *   plugins: [
 *     macroforgePlugin({
 *       generateTypes: true,
 *       typesOutputDir: 'src/types/generated',
 *       emitMetadata: false,
 *     }),
 *   ],
 * });
 * ```
 */
function napiMacrosPlugin(options: NapiMacrosPluginOptions = {}): Plugin {
  /**
   * Reference to the loaded Macroforge Rust binary module.
   * Contains the `expandSync` function for synchronous macro expansion.
   * Will be `undefined` if the binary failed to load.
   */
  let rustTransformer:
    | {
        expandSync: (
          code: string,
          filepath: string,
          options?: ExpandOptions,
        ) => ExpandResult;
      }
    | undefined;

  /** The resolved Vite project root directory */
  let projectRoot: string;

  /** Loaded configuration from macroforge.json */
  let macroConfig: MacroConfig = { keepDecorators: false };

  // Resolve options with defaults
  const generateTypes = options.generateTypes !== false; // Default to true
  const typesOutputDir = options.typesOutputDir || "src/macros/generated";
  const emitMetadata = options.emitMetadata !== false;
  const metadataOutputDir = options.metadataOutputDir || typesOutputDir;

  /**
   * Ensures a directory exists, creating it recursively if necessary.
   *
   * @param dir - The directory path to ensure exists
   */
  function ensureDir(dir: string) {
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true });
    }
  }

  /**
   * Writes generated TypeScript declaration files to the configured output directory.
   *
   * @remarks
   * Preserves the source file's directory structure within the output directory.
   * Implements change detection to avoid unnecessary file writes - only writes
   * if the content differs from the existing file (or the file doesn't exist).
   *
   * Output path formula:
   * `{projectRoot}/{typesOutputDir}/{relative/path/to/source}/{filename}.d.ts`
   *
   * @param id - The absolute path of the source file
   * @param types - The generated declaration file content
   */
  function writeTypeDefinitions(id: string, types: string) {
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
          `[@macroforge/vite-plugin] Wrote types for ${relativePath} -> ${path.relative(projectRoot, targetPath)}`,
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
   *
   * @remarks
   * Preserves the source file's directory structure within the output directory.
   * Implements change detection to avoid unnecessary file writes - only writes
   * if the content differs from the existing file (or the file doesn't exist).
   *
   * Output path formula:
   * `{projectRoot}/{metadataOutputDir}/{relative/path/to/source}/{filename}.macro-ir.json`
   *
   * The metadata contains information about which macros were applied and their
   * transformation results, useful for debugging and tooling integration.
   *
   * @param id - The absolute path of the source file
   * @param metadata - The macro IR metadata as a JSON string
   */
  function writeMetadata(id: string, metadata: string) {
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
          `[@macroforge/vite-plugin] Wrote metadata for ${relativePath} -> ${path.relative(projectRoot, targetPath)}`,
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
   *
   * @remarks
   * Handles both Error instances and unknown error types. For Error instances,
   * includes the full stack trace if available. Paths are made relative to the
   * project root for readability.
   *
   * @param error - The caught error (can be any type)
   * @param id - The absolute path of the file that failed to transform
   *
   * @returns A formatted error message string with plugin prefix
   */
  function formatTransformError(error: unknown, id: string): string {
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

  return {
    /**
     * The unique identifier for this plugin.
     * Used by Vite for plugin ordering and error reporting.
     */
    name: "@macroforge/vite-plugin",

    /**
     * Run this plugin before other plugins.
     *
     * @remarks
     * Macro expansion must happen early in the build pipeline so that
     * subsequent plugins (like TypeScript compilation) see the expanded code.
     */
    enforce: "pre",

    /**
     * Hook called when Vite config has been resolved.
     *
     * @remarks
     * Performs plugin initialization:
     * 1. Stores the resolved project root directory
     * 2. Loads the Macroforge configuration from `macroforge.json`
     * 3. Attempts to load the Rust macro expander binary
     *
     * If the Rust binary fails to load, a warning is logged but the plugin
     * continues to function (files will pass through unchanged).
     *
     * @param config - The resolved Vite configuration
     */
    configResolved(config) {
      projectRoot = config.root;
      macroConfig = loadMacroConfig(projectRoot);

      // Load the Rust binary
      try {
        rustTransformer = moduleRequire("macroforge");
      } catch (error) {
        console.warn(
          "[@macroforge/vite-plugin] Rust binary not found. Please run `npm run build:rust` first.",
        );
        console.warn(error);
      }
    },

    /**
     * Transform hook for processing TypeScript files through the macro expander.
     *
     * @remarks
     * This is the core of the plugin. For each TypeScript file (`.ts` or `.tsx`):
     *
     * 1. **Filtering**: Skips non-TypeScript files and `node_modules`
     * 2. **Expansion**: Calls the Rust binary's `expandSync()` function
     * 3. **Diagnostics**: Reports errors via `this.error()`, logs warnings
     * 4. **Post-processing**: Removes macro-only imports to prevent SSR issues
     * 5. **Type Generation**: Optionally generates `.d.ts` files
     * 6. **Metadata Emission**: Optionally writes macro IR JSON files
     *
     * **Return Value:**
     * - Returns `null` if the file should not be transformed (not TS, in node_modules, etc.)
     * - Returns `{ code, map }` with the transformed code (source maps not yet supported)
     *
     * **Error Handling:**
     * - Macro expansion errors are reported via Vite's error mechanism
     * - Vite plugin errors are re-thrown to preserve plugin attribution
     * - Other errors are formatted and reported
     *
     * @param code - The source code to transform
     * @param id - The absolute file path
     *
     * @returns Transformed code and source map, or null if no transformation needed
     */
    async transform(this, code: string, id: string) {
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

      // Check if Rust transformer is available
      if (!rustTransformer || !rustTransformer.expandSync) {
        // Return unchanged if transformer not available
        return null;
      }

      try {
        // Collect external decorator modules from macro imports
        const externalDecoratorModules = collectExternalDecoratorModules(code);

        // Perform macro expansion via the Rust binary
        const result: ExpandResult = rustTransformer.expandSync(code, id, {
          keepDecorators: macroConfig.keepDecorators,
          externalDecoratorModules,
        });

        // Report diagnostics from macro expansion
        for (const diag of result.diagnostics) {
          if (diag.level === "error") {
            const message = `Macro error at ${id}:${diag.start ?? "?"}-${diag.end ?? "?"}: ${diag.message}`;
            this.error(message);
          } else {
            // Log warnings and info messages
            console.warn(
              `[@macroforge/vite-plugin] ${diag.level}: ${diag.message}`,
            );
          }
        }

        if (result && result.code) {
          // TODO: Needs complete overhaul and dynamic attribute removal NO HARDCODING
          // Decorator removal is currently handled by the Rust binary based on keepDecorators config
          // if (!macroConfig.keepDecorators) {
          //   result.code = result.code
          //     .replace(/\/\*\*\s*@derive[\s\S]*?\*\/\s*/gi, "")
          //     .replace(/\/\*\*\s*@debug[\s\S]*?\*\/\s*/gi, "");
          // }

          // Remove macro-only imports so SSR output doesn't load native bindings
          // These imports are only needed at compile-time for type checking
          result.code = result.code.replace(
            /\/\*\*\s*import\s+macro[\s\S]*?\*\/\s*/gi,
            "",
          );

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
            map: null, // expandSync does not generate source maps yet
          };
        }
      } catch (error) {
        // Re-throw Vite plugin errors to preserve plugin attribution
        if (error && typeof error === "object" && "plugin" in error) {
          throw error;
        }
        // Format and report other errors
        const message = formatTransformError(error, id);
        this.error(message);
      }

      return null;
    },
  };
}

export default napiMacrosPlugin;
