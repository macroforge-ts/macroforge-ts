import { expandSync, hasMacroAnnotations } from '@macroforge/core';
import { Logger } from '../../logger.ts';

const FILE_EXTENSIONS = ['.ts', '.tsx', '.svelte', '.svelte.ts', '.svelte.tsx'];

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

/**
 * Runs macroforge macro expansion over `sourceText` and returns the expanded
 * code, generated type declarations, and any diagnostics.
 *
 * Contract and caveats:
 * - Returns an empty result (`{ types: null, code: null, diagnostics: [] }`)
 *   when the file extension is not handled, the text has no macro
 *   annotations, or expansion throws. Expansion errors are logged, never
 *   propagated to the caller.
 * - Only `types`, `code`, and `diagnostics` from the native `ExpandResult`
 *   are returned; `sourceMapping`, `metadata`, and `buildtimeDependencies`
 *   are dropped, so callers cannot map positions inside generated code.
 * - Diagnostic offsets refer to `sourceText` as passed in (pre-expansion).
 */
export function augmentWithMacroforge(
    fileName: string,
    sourceText: string
): MacroExpansionResult {
    if (!shouldProcess(fileName) || !hasMacroAnnotations(sourceText, fileName)) {
        return { types: null, code: null, diagnostics: [] };
    }

    try {
        const result = expandSync(sourceText, fileName, { emitMetadata: false });
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
