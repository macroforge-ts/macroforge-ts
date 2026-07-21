import ts from 'typescript';
import type { ExpandOptions } from '@macroforge/shared';
import { Logger } from '../../logger';

let expandSync: typeof import('macroforge').expandSync | undefined;
let macroforgeLoadError: Error | undefined;

try {
    expandSync = require('macroforge').expandSync;
    Logger.log('macroforge native module loaded successfully');
} catch (e) {
    macroforgeLoadError = e as Error;
    Logger.error('Failed to load macroforge native module:', e);
}

const DEFAULT_MACRO_NAMES = ['Derive'];
const DEFAULT_MIXIN_TYPES = ['MacroDebug', 'MacroJSON'];
const FILE_EXTENSIONS = ['.ts', '.tsx', '.svelte', '.svelte.ts', '.svelte.tsx'];

/**
 * Configuration for macroforge augmentation of document snapshots.
 *
 * Note: only `expandOptions` (and the presence of the config object itself)
 * is currently read by {@link augmentWithMacroforge}. The remaining fields
 * are populated from settings but are not read anywhere yet.
 */
export interface MacroforgeAugmentationConfig {
    /** Macro names to look for. Currently unread; expansion always runs. */
    macroNames: Set<string>;
    /** Module specifier for mixin type imports. Currently unread. */
    mixinModule: string;
    /** Mixin type names. Currently unread. */
    mixinTypes: string[];
    /** Options forwarded verbatim to `expandSync` (defaults to `{}`). */
    expandOptions?: ExpandOptions;
}

export interface MacroforgeAugmentationSettings {
    macroNames?: string[];
    mixinModule?: string;
    mixinTypes?: string[];
}

export interface MacroDiagnostic {
    level: string;
    message: string;
    start?: number;
    end?: number;
}

export interface MacroExpansionResult {
    types: string | null;
    code: string | null;
    diagnostics: MacroDiagnostic[];
}

export function createMacroforgeAugmentationConfig(
    settings?: MacroforgeAugmentationSettings
): MacroforgeAugmentationConfig {
    return {
        macroNames: new Set(settings?.macroNames ?? DEFAULT_MACRO_NAMES),
        mixinModule: settings?.mixinModule ?? '$lib/macros',
        mixinTypes: settings?.mixinTypes ?? DEFAULT_MIXIN_TYPES
    };
}

/**
 * Runs macroforge macro expansion over `sourceText` and returns the expanded
 * code, generated type declarations, and any diagnostics.
 *
 * Contract and caveats:
 * - Returns an empty result (`{ types: null, code: null, diagnostics: [] }`)
 *   when no config is given, the file extension is not handled, the native
 *   module failed to load, or expansion throws. Expansion errors are logged
 *   and swallowed - they never propagate to the caller.
 * - Files whose text does not contain an `'@'` character are skipped entirely
 *   as a cheap heuristic (any decorator or macro use requires one).
 * - Only `types`, `code`, and `diagnostics` from the native `ExpandResult`
 *   are returned; `sourceMapping`, `metadata`, and `buildtimeDependencies`
 *   are dropped, so callers cannot map positions inside generated code.
 * - `tsModule` is currently unused.
 * - Diagnostic offsets refer to `sourceText` as passed in (pre-expansion).
 */
export function augmentWithMacroforge(
    tsModule: typeof ts,
    fileName: string,
    sourceText: string,
    config?: MacroforgeAugmentationConfig
): MacroExpansionResult {
    if (!config || !shouldProcess(fileName)) {
        return { types: null, code: null, diagnostics: [] };
    }

    // Check if macroforge module loaded successfully
    if (!expandSync) {
        if (macroforgeLoadError) {
            Logger.debug(
                `Skipping macroforge expansion for ${fileName}: native module not loaded (${macroforgeLoadError.message})`
            );
        }
        return { types: null, code: null, diagnostics: [] };
    }

    // Basic check if macro is used to avoid invoking rust for every file
    // This is a heuristic, but expand_sync parses anyway so it's safe
    if (!sourceText.includes('@')) {
        return { types: null, code: null, diagnostics: [] };
    }

    try {
        const result = expandSync(sourceText, fileName, config.expandOptions ?? {});
        return {
            types: result.types || null,
            code: result.code || null,
            diagnostics: result.diagnostics
        };
    } catch (e) {
        Logger.error(`macroforge expansion failed for ${fileName}:`, e);
        return { types: null, code: null, diagnostics: [] };
    }
}

function shouldProcess(fileName: string) {
    return FILE_EXTENSIONS.some((ext) => fileName.endsWith(ext));
}

/**
 * Check if macroforge native module is available
 */
export function isMacroforgeAvailable(): boolean {
    return expandSync !== undefined;
}

/**
 * Get the macroforge load error if any
 */
export function getMacroforgeLoadError(): Error | undefined {
    return macroforgeLoadError;
}
