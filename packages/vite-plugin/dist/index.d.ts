import { Plugin } from 'vite';
export interface NapiMacrosPluginOptions {
    include?: string | RegExp | (string | RegExp)[];
    exclude?: string | RegExp | (string | RegExp)[];
    generateTypes?: boolean;
    typesOutputDir?: string;
    typeGenStrategy?: 'module-augmentation' | 'ambient';
}
declare function napiMacrosPlugin(options?: NapiMacrosPluginOptions): Plugin;
export default napiMacrosPlugin;
//# sourceMappingURL=index.d.ts.map