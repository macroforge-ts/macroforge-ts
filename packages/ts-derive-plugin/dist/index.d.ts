import type ts from "typescript/lib/tsserverlibrary";
import { expandSync as expandSyncImpl } from "@ts-macros/swc-napi";
type ExpandFn = typeof expandSyncImpl;
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