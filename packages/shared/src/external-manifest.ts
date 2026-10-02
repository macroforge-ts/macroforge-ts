/**
 * @module external-manifest
 *
 * Utilities for loading and caching external macro package manifests.
 */

import { createRequire } from 'node:module';

/** One macro exported by an external macro package (from `__macroforgeGetManifest*`). */
export interface MacroManifestEntry {
    /** Macro name as used in `@derive(...)` (matched case-insensitively). */
    name: string;
    /** Macro kind (e.g. `"derive"`). */
    kind: string;
    /** Human-readable description shown in editor tooling. */
    description: string;
    /** Package specifier the macro lives in (e.g. `"@playground/macro"`). */
    package: string;
}

/** One decorator exported by an external macro package. Note: docs live in `docs`, not `description`. */
export interface DecoratorManifestEntry {
    /** Module specifier the decorator is imported from. */
    module: string;
    /** Exported decorator name (matched case-insensitively by lookups). */
    export: string;
    /** Decorator kind (e.g. `"field"`, `"class"`). */
    kind: string;
    /** Human-readable documentation shown in editor tooling. */
    docs: string;
}

/**
 * Options accepted by the engine's `expandSync(code, filepath, options)`.
 *
 * Mirrors the engine's `ExpandOptions` (camelCase over the wasm boundary).
 */
export interface ExpandOptions {
    /**
     * If `true`, preserves `@derive` decorators in the output.
     * If `false` (default), decorators are stripped after expansion.
     */
    keepDecorators?: boolean;
    /**
     * Decorator module names contributed by external macro packages, used
     * during decorator stripping. Built-in modules are always included.
     */
    externalDecoratorModules?: string[];
    /**
     * Path to a config file previously loaded via the native `loadConfig`.
     * The engine reads macro-behavior sections (foreign types, `cfg`,
     * `deprecated`, ...) from its cache for this path.
     */
    configPath?: string;
    /**
     * Pre-built type registry JSON (from `scanProjectSync` or
     * `.macroforge/type-registry.json`) giving macros project-wide type
     * awareness.
     */
    typeRegistryJson?: string;
    /**
     * Pre-built project-wide declarative macro registry JSON. Enables
     * cross-file "import macro" comment resolution (importing `$name`
     * macros from another file); without it, cross-file macro imports
     * emit diagnostics at each unresolved call site.
     */
    declarativeRegistryJson?: string;
    /**
     * A type registry the engine keeps from `setTypeRegistry`, named by the
     * id it returned. Saves sending the registry's JSON on every call; pass
     * this or `typeRegistryJson`, not both.
     */
    typeRegistryId?: number;
    /**
     * A declarative registry the engine keeps from `setDeclarativeRegistry`.
     * Pass this or `declarativeRegistryJson`, not both.
     */
    declarativeRegistryId?: number;
    /**
     * Build mode for declarative (reverse-monomorphization) macros.
     * `"dev"` expands everything inline for precise diagnostics; `"prod"`
     * lets share/cluster modes emit shared runtime helpers. Engine default
     * when absent: `"dev"`.
     */
    buildMode?: 'dev' | 'prod';
    /**
     * Whether the result carries `metadata`, the processed classes as JSON.
     * A caller that never reads it passes `false` to skip serializing them.
     * Engine default when absent: `true`.
     */
    emitMetadata?: boolean;
}

/** Aggregated manifest for an external macro package. */
export interface MacroManifest {
    /** Manifest schema version reported by the package. */
    version: number | string;
    /** Macros exported by the package. */
    macros: MacroManifestEntry[];
    /** Decorators exported by the package. */
    decorators: DecoratorManifestEntry[];
}

/**
 * Function type for requiring modules.
 * Accepts any function that can load a module by path.
 */
export type RequireFunction = (id: string) => unknown;

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
 * import { createRequire } from "node:module";
 * const moduleRequire = createRequire(import.meta.url);
 * const manifest = getExternalManifest("@playground/macro", moduleRequire);
 * ```
 */
export function getExternalManifest(
    modulePath: string,
    requireFn?: RequireFunction
): MacroManifest | null {
    const cached = externalManifestCache.get(modulePath);
    if (cached !== undefined) {
        return cached;
    }
    const manifest = loadManifest(modulePath, requireFn ?? createRequire(import.meta.url));
    externalManifestCache.set(modulePath, manifest);
    return manifest;
}

function isRecord(value: unknown): value is Record<string, unknown> {
    return typeof value === 'object' && value !== null;
}

function isMacroManifestEntry(value: unknown): value is MacroManifestEntry {
    return isRecord(value) && typeof value.name === 'string' && typeof value.kind === 'string' &&
        typeof value.description === 'string' && typeof value.package === 'string';
}

function isDecoratorManifestEntry(value: unknown): value is DecoratorManifestEntry {
    return isRecord(value) && typeof value.module === 'string' &&
        typeof value.export === 'string' && typeof value.kind === 'string' &&
        typeof value.docs === 'string';
}

function isMacroManifest(value: unknown): value is MacroManifest {
    return isRecord(value) &&
        (typeof value.version === 'number' || typeof value.version === 'string') &&
        Array.isArray(value.macros) && value.macros.every(isMacroManifestEntry) &&
        Array.isArray(value.decorators) && value.decorators.every(isDecoratorManifestEntry);
}

/**
 * Aggregates what a package's `__macroforgeGetManifest` and per-macro
 * `__macroforgeGetManifest_<name>` exports return, or `null` when it has none.
 */
function loadManifest(modulePath: string, requireFn: RequireFunction): MacroManifest | null {
    let pkg: unknown;
    try {
        pkg = requireFn(modulePath);
    } catch (error) {
        // A package that cannot be loaded contributes no macros; say why once,
        // since the result is cached.
        console.warn(`[macroforge] could not load ${modulePath}: ${String(error)}`);
        return null;
    }
    if (!isRecord(pkg)) {
        return null;
    }

    const manifest: MacroManifest = { version: '0.0.0', macros: [], decorators: [] };
    let found = false;
    for (const [name, value] of Object.entries(pkg)) {
        const isManifestExport = name === '__macroforgeGetManifest' ||
            name.startsWith('__macroforgeGetManifest_');
        if (!isManifestExport || typeof value !== 'function') {
            continue;
        }
        const exported: unknown = value();
        if (!isMacroManifest(exported)) {
            console.warn(`[macroforge] ${modulePath}'s ${name} returned a malformed manifest`);
            continue;
        }
        manifest.macros.push(...exported.macros);
        manifest.decorators.push(...exported.decorators);
        manifest.version = exported.version;
        found = true;
    }
    if (!found) {
        return null;
    }

    manifest.macros = Array.from(
        new Map(manifest.macros.map((entry) => [entry.name, entry])).values()
    );
    manifest.decorators = Array.from(
        new Map(manifest.decorators.map((entry) => [entry.export, entry])).values()
    );
    return manifest;
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
    requireFn?: RequireFunction
): MacroManifestEntry | null {
    const manifest = getExternalManifest(modulePath, requireFn);
    if (!manifest) return null;

    return (
        manifest.macros.find(
            (m) => m.name.toLowerCase() === macroName.toLowerCase()
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
 *   console.log(decoratorInfo.docs);
 * }
 * ```
 */
export function getExternalDecoratorInfo(
    decoratorName: string,
    modulePath: string,
    requireFn?: RequireFunction
): DecoratorManifestEntry | null {
    const manifest = getExternalManifest(modulePath, requireFn);
    if (!manifest) return null;

    return (
        manifest.decorators.find(
            (d) => d.export.toLowerCase() === decoratorName.toLowerCase()
        ) ?? null
    );
}
