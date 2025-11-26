import { Plugin } from 'vite';
export interface NapiMacrosPluginOptions {
    include?: string | RegExp | (string | RegExp)[];
    exclude?: string | RegExp | (string | RegExp)[];
    generateTypes?: boolean;
    typesOutputDir?: string;
    emitMetadata?: boolean;
    metadataOutputDir?: string;
}
declare function napiMacrosPlugin(options?: NapiMacrosPluginOptions): Plugin;
export default napiMacrosPlugin;
//# sourceMappingURL=index.d.ts.map