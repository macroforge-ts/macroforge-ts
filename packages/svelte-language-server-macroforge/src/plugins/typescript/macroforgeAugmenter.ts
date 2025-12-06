import ts from 'typescript';
import { expandSync } from '@macroforge/swc-napi';

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
        return { types: null, diagnostics: [] };
    }

    // Basic check if macro is used to avoid invoking rust for every file
    // This is a heuristic, but expand_sync parses anyway so it's safe
    if (!sourceText.includes('@')) {
        return { types: null, diagnostics: [] };
    }

    try {
        const result = expandSync(sourceText, fileName);
        return {
            types: result.types || null,
            diagnostics: result.diagnostics
        };
    } catch (e) {
        // Handle generic failures (e.g. host initialization failed)
        // We could create a diagnostic here if critical
        return { types: null, diagnostics: [] };
    }
}

function shouldProcess(fileName: string) {
    return FILE_EXTENSIONS.some((ext) => fileName.endsWith(ext));
}
