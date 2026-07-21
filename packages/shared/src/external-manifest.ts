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
 * Options accepted by the native engine's `expandSync(code, filepath, options)`.
 *
 * Mirrors the napi `ExpandOptions` object (camelCase over the ABI boundary).
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
     * Build mode for declarative (reverse-monomorphization) macros.
     * `"dev"` expands everything inline for precise diagnostics; `"prod"`
     * lets share/cluster modes emit shared runtime helpers. Engine default
     * when absent: `"dev"`.
     */
    buildMode?: 'dev' | 'prod';
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
    if (externalManifestCache.has(modulePath)) {
        return externalManifestCache.get(modulePath) ?? null;
    }

    try {
        // Use provided require function or create one
        const req = requireFn ?? createRequire(import.meta.url);
        // If the require function supports .resolve(), create a scoped require
        // from the package's own directory. This ensures that native NAPI-RS
        // bindings using relative paths (e.g. require('./macros.darwin-x64.node'))
        // resolve from the package directory, not from the project root.
        // We temporarily set globalThis.require to the scoped version so that
        // Deno's CJS compat layer picks it up when loading the module.
        let pkg: Record<string, unknown>;
        if ('resolve' in req && typeof req.resolve === 'function') {
            const resolvedPath = (req as NodeRequire).resolve(modulePath);
            const scopedReq = createRequire(resolvedPath);
            const prevRequire = (globalThis as Record<string, unknown>).require;
            (globalThis as Record<string, unknown>).require = scopedReq;
            try {
                pkg = scopedReq(resolvedPath) as Record<string, unknown>;
            } finally {
                // Restore previous globalThis.require
                if (prevRequire === undefined) {
                    delete (globalThis as Record<string, unknown>).require;
                } else {
                    (globalThis as Record<string, unknown>).require = prevRequire;
                }
            }
        } else {
            pkg = req(modulePath) as Record<string, unknown>;
        }
        const manifest: MacroManifest = {
            macros: [],
            decorators: [],
            version: '0.0.0'
        };
        let found = false;

        // 1. Check for legacy global manifest
        if (typeof pkg.__macroforgeGetManifest === 'function') {
            const m = pkg.__macroforgeGetManifest() as MacroManifest;
            manifest.macros.push(...(m.macros || []));
            manifest.decorators.push(...(m.decorators || []));
            manifest.version = m.version || manifest.version;
            found = true;
        }

        // 2. Discover and aggregate macro-specific manifests
        // These are generated by #[ts_macro_derive] with unique names
        const keys = Object.keys(pkg);
        // console.log(`[macroforge:shared] Discovering manifests in ${modulePath}, found keys:`, keys);

        for (const key of keys) {
            if (
                key.startsWith('__macroforgeGetManifest_') &&
                typeof pkg[key] === 'function'
            ) {
                const fn = (pkg as Record<string, unknown>)[key] as () => MacroManifest;
                const m = fn();
                manifest.macros.push(...(m.macros || []));
                manifest.decorators.push(...(m.decorators || []));
                manifest.version = m.version || manifest.version;
                found = true;
            }
        }

        if (found) {
            // Deduplicate macros and decorators by name/export
            manifest.macros = Array.from(
                new Map(manifest.macros.map((m) => [m.name, m])).values()
            );
            manifest.decorators = Array.from(
                new Map(manifest.decorators.map((d) => [d.export, d])).values()
            );

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
