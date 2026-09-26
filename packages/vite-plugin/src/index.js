/* @ts-self-types="./index.d.ts" */
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

import { createRequire } from 'node:module';
import { createHash } from 'node:crypto';
import * as fs from 'node:fs';
import * as os from 'node:os';
import * as path from 'node:path';
import { fileURLToPath } from 'node:url';
import { encode as encodeVlq } from '@jridgewell/sourcemap-codec';
import * as engine from '@macroforge/core';
import { loadMacroConfig } from '@macroforge/shared';

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
            expandedLineStarts
        );
        const [origLine, origCol] = offsetToLineColumn(
            seg.originalStart,
            originalLineStarts
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
        names: []
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
        process.cwd() + '/',
        import.meta.url
    ];
    for (const root of roots) {
        try {
            const req = createRequire(root);
            tsModule = req('typescript');
            return tsModule;
        } catch {
            // continue to next root
        }
    }

    tsModule = undefined;
    console.warn(
        '[@macroforge/vite-plugin] TypeScript not found. Generated .d.ts files will be skipped.'
    );
    return tsModule;
}

/** @type {Map<string, import('typescript').CompilerOptions>} */
const compilerOptionsCache = new Map();

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
            'tsconfig.json'
        );
    } catch {
        configPath = undefined;
    }

    /** @type {import('typescript').CompilerOptions} */
    let options;
    if (configPath) {
        const configFile = tsModule.readConfigFile(
            configPath,
            tsModule.sys.readFile
        );
        if (configFile.error) {
            const formatted = tsModule.formatDiagnosticsWithColorAndContext(
                [configFile.error],
                {
                    getCurrentDirectory: () => projectRoot,
                    getCanonicalFileName: (fileName) => fileName,
                    getNewLine: () => tsModule.sys.newLine
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
        // Vite projects set `noEmit`, under which emit writes nothing at all.
        noEmit: false,
        noEmitOnError: false,
        incremental: false
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
 * @typedef {object} DeclarationEmitter
 * @property {import('typescript').CompilerHost} host
 * @property {Map<string, string>} overrides - In-memory text for the file being emitted
 * @property {import('typescript').Program | undefined} previousProgram
 */

/** @type {Map<string, DeclarationEmitter>} */
const declarationEmitters = new Map();

/**
 * The declaration emitter for `projectRoot`. Every emit shares its compiler
 * host, which keeps each parsed source file until its text changes, and hands
 * the previous program to the next one: a fresh program per module re-parsed
 * the whole import graph and every ambient `@types` package each time.
 * @param {typeof import('typescript')} ts
 * @param {string} projectRoot
 * @param {import('typescript').CompilerOptions} compilerOptions
 * @returns {DeclarationEmitter}
 */
function declarationEmitterFor(ts, projectRoot, compilerOptions) {
    const existing = declarationEmitters.get(projectRoot);
    if (existing) return existing;

    const baseHost = ts.createCompilerHost(compilerOptions, true);
    /** @type {Map<string, string>} */
    const overrides = new Map();
    /** @type {Map<string, { text: string, sourceFile: import('typescript').SourceFile }>} */
    const parsed = new Map();
    const readFile = (/** @type {string} */ fileName) =>
        overrides.get(path.resolve(fileName)) ?? baseHost.readFile(fileName);

    /** @type {DeclarationEmitter} */
    const emitter = {
        host: {
            ...baseHost,
            readFile,
            fileExists: (fileName) =>
                overrides.has(path.resolve(fileName)) || baseHost.fileExists(fileName),
            getSourceFile: (fileName, languageVersion) => {
                const text = readFile(fileName);
                if (text === undefined) return undefined;
                const key = path.resolve(fileName);
                const cached = parsed.get(key);
                if (cached && cached.text === text) return cached.sourceFile;
                const sourceFile = ts.createSourceFile(fileName, text, languageVersion, true);
                parsed.set(key, { text, sourceFile });
                return sourceFile;
            }
        },
        overrides,
        previousProgram: undefined
    };
    declarationEmitters.set(projectRoot, emitter);
    return emitter;
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
    const emitter = declarationEmitterFor(tsModule, projectRoot, compilerOptions);

    // Capture emitted declaration content
    /** @type {string | undefined} */
    let output;
    const writeFile = (
        /** @type {string} */ outputName,
        /** @type {string} */ text
    ) => {
        if (outputName.endsWith('.d.ts')) {
            output = text;
        }
    };

    emitter.overrides.set(normalizedFileName, code);
    /** @type {import('typescript').EmitResult} */
    let emitResult;
    try {
        const program = tsModule.createProgram(
            [normalizedFileName],
            compilerOptions,
            emitter.host,
            emitter.previousProgram
        );
        emitter.previousProgram = program;
        emitResult = program.emit(undefined, writeFile, undefined, true);
    } finally {
        emitter.overrides.delete(normalizedFileName);
    }

    // Log diagnostics if emission was skipped due to errors
    if (emitResult.emitSkipped && emitResult.diagnostics.length > 0) {
        const formatted = tsModule.formatDiagnosticsWithColorAndContext(
            emitResult.diagnostics,
            {
                getCurrentDirectory: () => projectRoot,
                getCanonicalFileName: (fileName) => fileName,
                getNewLine: () => tsModule.sys.newLine
            }
        );
        console.warn(
            `[@macroforge/vite-plugin] Declaration emit failed for ${
                path.relative(
                    projectRoot,
                    fileName
                )
            }\n${formatted}`
        );
        return undefined;
    }

    if (output === undefined) {
        console.warn(
            `[@macroforge/vite-plugin] Declaration emit produced no .d.ts for ${
                path.relative(projectRoot, fileName)
            }`
        );
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
 * @throws {Error} If a config file is found but cannot be imported (e.g. it
 *   has a syntax error); the underlying import error message is included.
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
 *     generateTypes: true,                   // Generate .d.ts files (default: true)
 *     typesOutputDir: ".macroforge/types",   // Types output dir (default: ".macroforge/types")
 *     emitMetadata: true,                    // Emit metadata JSON (default: true)
 *     metadataOutputDir: ".macroforge/meta", // Metadata output dir (default: ".macroforge/meta")
 *     devCache: true,                        // Disk cache for dev mode (default: true)
 *   },
 * };
 * ```
 */
export async function macroforge() {
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

    // Load config upfront (the engine parses foreign types)
    const macroConfig = loadMacroConfig(process.cwd(), engine.loadConfig);

    if (macroConfig.hasForeignTypes) {
        console.log(
            '[@macroforge/vite-plugin] Loaded config with foreign types from:',
            macroConfig.configPath
        );
    }

    // Vite options resolved from config (with defaults)
    /** @type {boolean} */
    let generateTypes = true;
    /** @type {string} */
    let typesOutputDir = '.macroforge/types';
    /** @type {boolean} */
    let emitMetadata = true;
    /** @type {string} */
    let metadataOutputDir = '.macroforge/meta';
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
                `[@macroforge/vite-plugin] Failed to load config from ${macroConfig.configPath}: ${error.message}`
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
    /** @type {{ version: string, configHash: string, externalMacroHash: string, engineHashes: Record<string, string>, builtinOnly?: boolean, entries: Record<string, { sourceHash: string, hasMacros: boolean }> } | null} */
    let cacheManifest = null;
    /** @type {string} */
    let macroforgeVersion = 'unknown';
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

    /** Distinguishes temp files written by this process within one millisecond. */
    let atomicWriteCounter = 0;

    /**
     * Writes a file atomically: into a temporary sibling, then renamed over the
     * destination.
     *
     * Everything this plugin writes has another reader — the macroforge CLI
     * reads back `manifest.json`, `tsc` and editors read the emitted `.d.ts`,
     * and a subsequent dev-server start reads the `.cache` entries. A plain
     * `writeFileSync` leaves those readers a window in which the file is
     * truncated or half-written; renaming over the destination means they see
     * either the previous contents or the complete new ones.
     *
     * The temporary file is a sibling of the destination so the rename stays on
     * one filesystem, and is tagged with the pid so two writers never collide
     * on it.
     *
     * @param {string} filePath - Destination path
     * @param {string} data - File contents
     */
    function writeFileAtomic(filePath, data) {
        const tmpPath = `${filePath}.${process.pid}.${atomicWriteCounter++}.tmp`;
        fs.writeFileSync(tmpPath, data, 'utf-8');
        try {
            fs.renameSync(tmpPath, filePath);
        } catch (error) {
            try {
                fs.unlinkSync(tmpPath);
            } catch {
                // Best effort — the rename failure is what matters.
            }
            throw error;
        }
    }

    // --- Dev cache helpers ---

    /**
     * Computes SHA-256 hash of a string, returned as hex.
     * @param {string} content
     * @returns {string}
     */
    function contentHash(content) {
        return createHash('sha256').update(content).digest('hex');
    }

    /**
     * The engine module this plugin depends on, as a file path.
     * @returns {string}
     */
    function engineEntryPath() {
        return fileURLToPath(import.meta.resolve('@macroforge/core'));
    }

    /**
     * The engine's version, from the `package.json` enclosing its entry.
     * @returns {string}
     */
    function getMacroforgeVersion() {
        let current = path.dirname(engineEntryPath());
        while (current !== path.dirname(current)) {
            const packageJson = path.join(current, 'package.json');
            if (fs.existsSync(packageJson)) {
                const manifest = JSON.parse(fs.readFileSync(packageJson, 'utf-8'));
                if (manifest.name === '@macroforge/core') return manifest.version;
            }
            current = path.dirname(current);
        }
        throw new Error('[@macroforge/vite-plugin] @macroforge/core has no package.json');
    }

    /**
     * Fingerprints the engine build (its wasm module's size and mtime). The
     * version alone misses rebuilds that keep it, which would serve
     * expansions an older engine produced.
     * @returns {string}
     */
    function getEngineHash() {
        const wasmPath = path.join(path.dirname(engineEntryPath()), 'macroforge_ts_bg.wasm');
        return contentHash(fs.readFileSync(wasmPath));
    }

    /**
     * Computes a hash over external macro package binaries (`.node`, `.wasm`)
     * so the cache invalidates when a local macro package is rebuilt.
     * @returns {string}
     */
    function getExternalMacroHash() {
        // Collect path:size:content parts, sort for deterministic ordering
        // (readdir order varies across Node/Deno/Rust), then hash. Content, not
        // mtime: an install copies these files, and copying identical bytes
        // must not throw the cache away.
        const parts = [];

        /**
         * A macro package's artifacts live wherever its build put them:
         * NAPI packages emit `index.js` + `*.node` at the package root, while
         * `macroforge build` (wasm-bindgen) emits `pkg/<name>.js` +
         * `pkg/<name>_bg.wasm` and the root has no `index.js` at all. Probing
         * only the root therefore missed every wasm package, pinning this hash
         * at 'none' so a rebuilt macro never invalidated the cache.
         * @param {string} pkgDir
         */
        const checkPackage = (pkgDir) => {
            // Directories to probe: the package root and, if present, `pkg/`.
            const dirs = [pkgDir];
            const pkgSubdir = path.join(pkgDir, 'pkg');
            try {
                if (fs.statSync(pkgSubdir).isDirectory()) dirs.push(pkgSubdir);
            } catch { /* no pkg/ subdir */ }

            // A package qualifies if any candidate entry script carries the
            // generated `__macroforgeRun` exports.
            const isMacroPackage = dirs.some((dir) => {
                try {
                    return fs.readdirSync(dir).some((entry) => {
                        if (path.extname(entry) !== '.js') return false;
                        try {
                            return fs
                                .readFileSync(path.join(dir, entry), 'utf-8')
                                .includes('__macroforgeRun');
                        } catch {
                            return false;
                        }
                    });
                } catch {
                    return false;
                }
            });
            if (!isMacroPackage) return;

            for (const dir of dirs) {
                try {
                    for (const entry of fs.readdirSync(dir)) {
                        const ext = path.extname(entry);
                        if (ext !== '.node' && ext !== '.wasm' && ext !== '.js') {
                            continue;
                        }
                        const full = path.join(dir, entry);
                        try {
                            const bytes = fs.readFileSync(full);
                            parts.push(`${full}:${bytes.length}:${contentHash(bytes)}`);
                        } catch { /* expected */ }
                    }
                } catch { /* expected */ }
            }
        };

        // Every ancestor's `node_modules`, not just the project's own, because
        // that is where Node finds a package and therefore where a workspace
        // installs one — a package in `apps/web` gets its dependencies from the
        // repository root. Scanning only `<root>/node_modules` finds nothing
        // there and pins this hash at 'none', so a rebuilt macro package never
        // invalidates the cache and dev keeps serving the previous expansion.
        //
        // `dirname` stops changing at the filesystem root, which visits the same
        // set the Rust twin reaches via `Path::parent` returning `None`.
        let dir = path.resolve(projectRoot || process.cwd());
        for (;;) {
            const nodeModules = path.join(dir, 'node_modules');
            try {
                for (const entry of fs.readdirSync(nodeModules)) {
                    const full = path.join(nodeModules, entry);
                    if (!fs.statSync(full).isDirectory()) continue;
                    if (entry.startsWith('@')) {
                        try {
                            for (const sub of fs.readdirSync(full)) {
                                const subFull = path.join(full, sub);
                                if (fs.statSync(subFull).isDirectory()) checkPackage(subFull);
                            }
                        } catch { /* expected */ }
                    } else if (!entry.startsWith('.')) {
                        checkPackage(full);
                    }
                }
            } catch { /* no node_modules at this level */ }

            const parent = path.dirname(dir);
            if (parent === dir) break;
            dir = parent;
        }

        if (parts.length === 0) return 'none';
        parts.sort();
        const hasher = createHash('sha256');
        for (const part of parts) {
            hasher.update(part);
        }
        return hasher.digest('hex');
    }

    /**
     * Computes a hash of the macroforge config file for cache invalidation.
     * @returns {string}
     */
    function getConfigHash() {
        if (macroConfig.configPath) {
            try {
                return contentHash(fs.readFileSync(macroConfig.configPath, 'utf-8'));
            } catch {
                // config file disappeared
            }
        }
        return 'none';
    }

    /**
     * Loads and validates the cache manifest from disk.
     * Returns null if the cache is stale (version or config mismatch).
     * @returns {{ version: string, configHash: string, externalMacroHash: string, engineHashes: Record<string, string>, builtinOnly?: boolean, entries: Record<string, { sourceHash: string, hasMacros: boolean }> } | null}
     */
    function loadCacheManifest() {
        const manifestPath = path.join(cacheDir, 'manifest.json');
        if (!fs.existsSync(manifestPath)) return null;

        try {
            const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf-8'));

            if (manifest.version !== macroforgeVersion) {
                console.log(
                    '[@macroforge/vite-plugin] Cache invalidated: macroforge version changed'
                );
                return null;
            }

            const currentConfigHash = getConfigHash();
            if (manifest.configHash !== currentConfigHash) {
                console.log(
                    '[@macroforge/vite-plugin] Cache invalidated: config changed'
                );
                return null;
            }

            // Reject caches built with --builtin-only since they may lack external macro expansions
            if (manifest.builtinOnly) {
                console.log(
                    '[@macroforge/vite-plugin] Cache invalidated: built with --builtin-only (run without --builtin-only for full expansion)'
                );
                return null;
            }

            // Compared unconditionally. Both writers normally emit a hash, so
            // the truthiness guard this replaces was near-dead — but the CLI
            // deserializes `external_macro_hash` with `#[serde(default)]`, so a
            // manifest missing the field round-trips as `""`. That is falsy,
            // which skipped the check and let a stale cache outlive an
            // arbitrarily broken macro package. An empty hash is missing
            // evidence, not a match.
            const currentExternalHash = getExternalMacroHash();
            if (manifest.externalMacroHash !== currentExternalHash) {
                console.log(
                    '[@macroforge/vite-plugin] Cache invalidated: external macro binary changed'
                );
                return null;
            }

            // Only this writer's key. A manifest last written by the CLI
            // carries `cli` alone, and its entries are as good as ours, so a
            // missing `wasm` is not evidence against them.
            const recordedEngine = manifest.engineHashes?.wasm;
            if (recordedEngine !== undefined && recordedEngine !== getEngineHash()) {
                console.log(
                    '[@macroforge/vite-plugin] Cache invalidated: macroforge engine rebuilt'
                );
                return null;
            }

            return manifest;
        } catch {
            return null;
        }
    }

    /**
     * Snapshot a file's current content hash + mtime for buildtime
     * dependency tracking. Returns `{ path, hash, mtimeMs }` for an
     * existing file or `{ path, missing: true }` for one that doesn't.
     * @param {string} depPath - Absolute path to probe
     */
    function snapshotBuildtimeDep(depPath) {
        try {
            const stat = fs.statSync(depPath);
            const content = fs.readFileSync(depPath);
            return {
                path: depPath,
                hash: createHash('sha256').update(content).digest('hex'),
                mtimeMs: stat.mtimeMs
            };
        } catch {
            return { path: depPath, missing: true };
        }
    }

    /**
     * Two-level validity check for a stored buildtime dependency. Fast
     * path: mtime unchanged → trust the cache. Slow path: mtime changed
     * but content hash matches → the file was re-saved without edits,
     * cache is still valid; bump the stored mtime to avoid re-hashing
     * next time.
     * @param {{ path: string, hash?: string, mtimeMs?: number, missing?: boolean }} snap
     */
    function buildtimeDepStillValid(snap) {
        if (!snap || typeof snap.path !== 'string') return false;
        if (snap.missing) {
            // Was missing when cached — still cache-valid only if still missing.
            try {
                fs.statSync(snap.path);
                return false;
            } catch {
                return true;
            }
        }
        let stat;
        try {
            stat = fs.statSync(snap.path);
        } catch {
            return false; // existed before, now gone → invalidate
        }
        if (
            typeof snap.mtimeMs === 'number' &&
            stat.mtimeMs === snap.mtimeMs
        ) {
            return true;
        }
        if (typeof snap.hash !== 'string') return false;
        let content;
        try {
            content = fs.readFileSync(snap.path);
        } catch {
            return false;
        }
        const currentHash = createHash('sha256').update(content).digest('hex');
        if (currentHash === snap.hash) {
            // Content unchanged — refresh the stored mtime.
            snap.mtimeMs = stat.mtimeMs;
            return true;
        }
        return false;
    }

    /**
     * Resolves the on-disk `.cache` path for a source file's project-relative
     * path. Sources outside the project root (e.g. a shared library pulled in
     * from a sibling directory) yield a relPath with leading `..` segments; a
     * naive `path.join(cacheDir, relPath)` then climbs out of the cache dir and
     * scatters `.cache` files into a sibling tree. Fold any escape into a
     * contained `_external/` subdir so every entry stays under cacheDir. In-root
     * sources (no `..`) keep their existing layout, so their cache is untouched.
     * @param {string} relPath - `path.relative(projectRoot, id)`
     * @returns {string} absolute path to the `.cache` file
     */
    function cachePathFor(relPath) {
        const segments = relPath.split(path.sep);
        const contained = segments[0] === '..'
            ? path.join('_external', ...segments.filter((s) => s !== '..'))
            : relPath;
        return path.join(cacheDir, contained + '.cache');
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

        // Buildtime dep check — if any dep changed, the cached result
        // is stale even though the source itself is unchanged.
        if (Array.isArray(entry.buildtimeDeps) && entry.buildtimeDeps.length) {
            for (const snap of entry.buildtimeDeps) {
                if (!buildtimeDepStillValid(snap)) return null;
            }
            // Successful validity check may have refreshed mtime fields;
            // mark manifest dirty so the update persists.
            cacheManifestDirty = true;
        }

        const cachePath = cachePathFor(relPath);
        try {
            const expandedCode = fs.readFileSync(cachePath, 'utf-8');
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
     * @param {string} expandedCode - Expanded code from the engine
     * @param {boolean} hasMacros - Whether the file actually had macros expanded
     * @param {string[]} [buildtimeDeps] - Absolute paths the @buildtime
     *   pre-pass read during evaluation. Each is snapshotted with its
     *   mtime + content hash so subsequent builds can invalidate the
     *   cache entry when any of them changes.
     */
    function writeCacheEntry(
        id,
        sourceCode,
        expandedCode,
        hasMacros,
        buildtimeDeps
    ) {
        if (!cacheDir) return;

        const relPath = path.relative(projectRoot, id);

        try {
            // Only write .cache files for files that actually have macros
            if (hasMacros) {
                const cachePath = cachePathFor(relPath);
                ensureDir(path.dirname(cachePath));
                writeFileAtomic(cachePath, expandedCode);
            }

            if (!cacheManifest) {
                cacheManifest = {
                    version: macroforgeVersion,
                    configHash: getConfigHash(),
                    externalMacroHash: getExternalMacroHash(),
                    engineHashes: { wasm: getEngineHash() },
                    entries: {}
                };
            }

            const depSnapshots = Array.isArray(buildtimeDeps)
                ? buildtimeDeps.map(snapshotBuildtimeDep)
                : [];
            cacheManifest.entries[relPath] = {
                sourceHash: contentHash(sourceCode),
                hasMacros,
                buildtimeDeps: depSnapshots
            };

            // Debounce manifest writes — don't write 59KB JSON on every file
            cacheManifestDirty = true;
            if (manifestFlushTimer) clearTimeout(manifestFlushTimer);
            manifestFlushTimer = setTimeout(flushCacheManifest, 500);
        } catch (error) {
            console.warn(
                `[@macroforge/vite-plugin] Failed to write cache for ${relPath}:`,
                error.message
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
            // The macroforge CLI writes this same file (`macroforge watch`
            // running beside `vite dev` is the documented pairing), so it has
            // to land as a single rename rather than a progressive overwrite.
            writeFileAtomic(
                path.join(cacheDir, 'manifest.json'),
                JSON.stringify(cacheManifest, null, 2)
            );
            cacheManifestDirty = false;
        } catch (error) {
            console.warn(
                `[@macroforge/vite-plugin] Failed to write cache manifest:`,
                error.message
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
        const targetPath = typesPathFor(id);
        ensureDir(path.dirname(targetPath));

        try {
            const existing = fs.existsSync(targetPath)
                ? fs.readFileSync(targetPath, 'utf-8')
                : null;
            if (existing !== types) {
                writeFileAtomic(targetPath, types);
                console.log(
                    `[@macroforge/vite-plugin] Wrote types for ${relativePath} -> ${
                        path.relative(projectRoot, targetPath)
                    }`
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
     * Where the declarations generated for `id` are written.
     * @param {string} id - The absolute path of the source file
     * @returns {string}
     */
    function typesPathFor(id) {
        const parsed = path.parse(path.relative(projectRoot, id));
        return path.join(projectRoot, typesOutputDir, parsed.dir, `${parsed.name}.d.ts`);
    }

    /**
     * Emits and writes declarations for a file whose macros generated code.
     * @param {string} id - The absolute path of the source file
     * @param {string} expandedCode - The file's macro-expanded source
     */
    function generateTypeDefinitions(id, expandedCode) {
        const emitted = emitDeclarationsFromCode(expandedCode, id, projectRoot);
        if (emitted) {
            writeTypeDefinitions(id, emitted);
        }
    }

    /**
     * Deletes the declarations generated for a file that no longer has macros:
     * tsc derives its types from the source, and a leftover file would shadow
     * them with stale macro members.
     * @param {string} id - The absolute path of the source file
     */
    function removeTypeDefinitions(id) {
        const targetPath = typesPathFor(id);
        try {
            fs.rmSync(targetPath, { force: true });
        } catch (error) {
            console.error(
                `[@macroforge/vite-plugin] Failed to remove stale type definitions ${targetPath}:`,
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
                ? fs.readFileSync(targetPath, 'utf-8')
                : null;
            if (existing !== metadata) {
                writeFileAtomic(targetPath, metadata);
                console.log(
                    `[@macroforge/vite-plugin] Wrote metadata for ${relativePath} -> ${
                        path.relative(projectRoot, targetPath)
                    }`
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
            const details = error.stack && error.stack.includes(error.message)
                ? error.stack
                : `${error.message}\n${error.stack ?? ''}`;
            return `[@macroforge/vite-plugin] Failed to transform ${relative}\n${details}`
                .trim();
        }
        return `[@macroforge/vite-plugin] Failed to transform ${relative}: ${String(error)}`;
    }

    /** @type {import('vite').Plugin} */
    const plugin = {
        name: '@macroforge/vite-plugin',
        enforce: 'pre',

        /**
         * @param {{ root: string, command: string }} config
         */
        configResolved(config) {
            projectRoot = config.root;
            isDevMode = config.command === 'serve';

            if (isDevMode && devCacheEnabled) {
                cacheDir = path.join(projectRoot, '.macroforge', 'cache');
                macroforgeVersion = getMacroforgeVersion();
                cacheManifest = loadCacheManifest();

                if (cacheManifest) {
                    // Keep whatever key the CLI recorded and stamp our own.
                    cacheManifest.engineHashes = {
                        ...cacheManifest.engineHashes,
                        wasm: getEngineHash()
                    };
                    const entryCount = Object.keys(cacheManifest.entries).length;
                    console.log(
                        `[@macroforge/vite-plugin] Dev cache loaded: ${entryCount} entries`
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
                '.macroforge',
                'type-registry.json'
            );
            if (fs.existsSync(localRegistry)) {
                typeRegistryJson = fs.readFileSync(localRegistry, 'utf-8');
                try {
                    const parsed = JSON.parse(typeRegistryJson);
                    const count = Object.keys(parsed.types ?? parsed).length;
                    console.log(
                        `[@macroforge/vite-plugin] Type registry loaded: ${count} types`
                    );
                } catch {
                    // JSON is passed as-is to expandSync, no need to parse here
                }
            } else {
                console.warn(
                    `[@macroforge/vite-plugin] No type registry found at .macroforge/type-registry.json. Run \`macroforge watch\` to generate it.`
                );
            }

            // Load the declarative macro registry alongside the type registry.
            // Produced by the same project scan, so if one exists the other
            // almost certainly does too. Missing file is a no-op: cross-file
            // declarative macro imports simply won't resolve.
            const localDeclarativeRegistry = path.join(
                projectRoot,
                '.macroforge',
                'declarative-registry.json'
            );
            if (fs.existsSync(localDeclarativeRegistry)) {
                declarativeRegistryJson = fs.readFileSync(
                    localDeclarativeRegistry,
                    'utf-8'
                );
                try {
                    const parsed = JSON.parse(declarativeRegistryJson);
                    const fileCount = Object.keys(parsed.by_file ?? {}).length;
                    if (fileCount > 0) {
                        console.log(
                            `[@macroforge/vite-plugin] Declarative macro registry loaded: ${fileCount} file(s)`
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
            if (!source.endsWith('.svelte') || !importer) return null;

            // Let other plugins (SvelteKit, etc.) try to resolve it first
            const resolved = await this.resolve(source, importer, {
                ...options,
                skipSelf: true
            });

            if (resolved && !resolved.external) return resolved;

            // Fall back: try appending .ts
            const resolvedTs = await this.resolve(source + '.ts', importer, {
                ...options,
                skipSelf: true
            });

            return resolvedTs || null;
        },

        /**
         * @param {string} code
         * @param {string} id
         */
        async transform(code, id) {
            // Only transform TypeScript files
            if (!id.endsWith('.ts') && !id.endsWith('.tsx')) {
                return null;
            }

            // Skip node_modules by default
            if (id.includes('node_modules')) {
                return null;
            }

            // Skip already-expanded files
            if (id.includes('.expanded.')) {
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
                            ''
                        );
                        if (id.endsWith('.svelte.ts') || id.endsWith('.svelte.js')) {
                            cachedCode = cachedCode.replace(
                                /\/\*\*\s*@derive\b[^*]*\*\//g,
                                ''
                            );
                        }

                        // Only files with macros are cached, and their generated
                        // members reach tsc through these declarations.
                        if (generateTypes && !fs.existsSync(typesPathFor(id))) {
                            generateTypeDefinitions(id, cachedCode);
                        }

                        return {
                            code: cachedCode,
                            map: null
                        };
                    }
                }

                // Perform macro expansion
                const result = engine.expandSync(code, id, {
                    keepDecorators: macroConfig.keepDecorators,
                    configPath: macroConfig.configPath,
                    typeRegistryJson,
                    declarativeRegistryJson,
                    // Reverse-monomorphization build mode. Dev (serve) runs all
                    // declarative macros as if they were expand-only for precise
                    // diagnostics; prod (build) emits share-mode helpers for
                    // `share-only`, `share-anyway`, and `auto` macros.
                    buildMode: isDevMode ? 'dev' : 'prod'
                });

                // Report diagnostics from macro expansion
                for (const diag of result.diagnostics) {
                    if (diag.level === 'error') {
                        const message = `Macro error at ${id}:${diag.start ?? '?'}-${
                            diag.end ?? '?'
                        }: ${diag.message}`;
                        /** @type {any} */ (this).error(message);
                    } else {
                        console.warn(
                            `[@macroforge/vite-plugin] ${diag.level}: ${diag.message}`
                        );
                    }
                }

                // Watch files that `@buildtime` declarations read. When any of
                // them change, Vite re-invokes `transform` and the buildtime
                // pre-pass re-evaluates with the fresh content.
                if (
                    result.buildtimeDependencies && result.buildtimeDependencies.length
                ) {
                    for (const dep of result.buildtimeDependencies) {
                        /** @type {any} */ (this).addWatchFile(dep);
                    }
                }

                if (result && result.code) {
                    // Check if macros were actually expanded. A file counts as
                    // "has macros" if derive macros emitted generated regions OR
                    // if the buildtime pre-pass rewrote the source (detected by
                    // presence of dependencies or by text difference against the
                    // input — a `@buildtime const X = 1+1` rewrite reads no files
                    // but still changes the output).
                    const hasMacros = result.sourceMapping?.generatedRegions?.length > 0 ||
                        (result.buildtimeDependencies?.length ?? 0) > 0 ||
                        result.code !== code;

                    // --- Dev cache write (self-populating) ---
                    if (isDevMode && devCacheEnabled) {
                        writeCacheEntry(
                            id,
                            code,
                            result.code,
                            hasMacros,
                            result.buildtimeDependencies
                        );
                    }

                    // Remove macro-only imports so SSR output doesn't load native bindings
                    result.code = result.code.replace(
                        /\/\*\*\s*import\s+macro[\s\S]*?\*\/\s*/gi,
                        ''
                    );

                    // For .svelte.ts modules, strip @derive JSDoc comments to prevent
                    // the Svelte preprocessor from re-expanding macros
                    if (id.endsWith('.svelte.ts') || id.endsWith('.svelte.js')) {
                        result.code = result.code.replace(
                            /\/\*\*\s*@derive\b[^*]*\*\//g,
                            ''
                        );
                    }

                    // Declarations only for files whose macros generated code;
                    // tsc derives every other file's types from its source.
                    if (generateTypes) {
                        if (hasMacros) {
                            generateTypeDefinitions(id, result.code);
                        } else {
                            removeTypeDefinitions(id);
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
                        map
                    };
                }
            } catch (error) {
                // Re-throw Vite plugin errors to preserve plugin attribution
                if (error && typeof error === 'object' && 'plugin' in error) {
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
            const file = ctx.file;
            // Config files invalidate the entire cache (matches the
            // `clear_cache` path in the Rust singleton).
            if (
                file.endsWith('macroforge.config.ts') ||
                file.endsWith('macroforge.config.js') ||
                file.endsWith('macroforge.config.mjs') ||
                file.endsWith('tsconfig.json')
            ) {
                engine.clearScanCache();
                return;
            }
            // Source file change — drop the single entry. The next
            // `scanProjectSync` call (either from buildStart or a future
            // HMR refresh) will re-parse it.
            engine.invalidateScanCacheEntry(file);
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
        }
    };

    return plugin;
}

export default macroforge;
