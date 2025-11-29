import type ts from "typescript/lib/tsserverlibrary";
interface MacroDiagnostic {
    level: string;
    message: string;
    start?: number;
    end?: number;
}
/** Native module source mapping result (uses camelCase from NAPI-RS) */
interface NativeSourceMapping {
    segments: Array<{
        originalStart: number;
        originalEnd: number;
        expandedStart: number;
        expandedEnd: number;
    }>;
    generatedRegions: Array<{
        start: number;
        end: number;
        sourceMacro: string;
    }>;
}
interface ExpandResult {
    code: string;
    types?: string;
    diagnostics: MacroDiagnostic[];
    sourceMapping?: NativeSourceMapping;
}
type ExpandFn = (code: string, fileName: string) => ExpandResult;
declare function init(modules: {
    typescript: typeof ts;
}): {
    create: (info: ts.server.PluginCreateInfo) => ts.LanguageService;
};
type TsMacrosPluginFactory = typeof init & {
    __setExpandSync?: (fn: ExpandFn) => void;
    __resetExpandSync?: () => void;
};
declare const pluginFactory: TsMacrosPluginFactory;
export = pluginFactory;
//# sourceMappingURL=index.d.ts.map