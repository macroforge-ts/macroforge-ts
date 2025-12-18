import ts from 'typescript';
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

export interface MacroforgeAugmentationConfig {
    macroNames: Set<string>;
    mixinModule: string;
    mixinTypes: string[];
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
        const result = expandSync(sourceText, fileName);
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
