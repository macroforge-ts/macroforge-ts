import type ts from 'typescript/lib/tsserverlibrary';
declare function init(modules: {
    typescript: typeof ts;
}): {
    create: (info: ts.server.PluginCreateInfo) => ts.LanguageService;
};
export = init;
//# sourceMappingURL=index.d.ts.map