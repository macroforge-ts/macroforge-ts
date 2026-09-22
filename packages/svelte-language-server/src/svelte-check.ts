import { isAbsolute } from 'path';
import ts from 'typescript';
import { Diagnostic, Position, Range } from 'vscode-languageserver';
import { WorkspaceFolder } from 'vscode-languageserver-protocol';
import { Document, DocumentManager } from './lib/documents/index.ts';
import { Logger } from './logger.ts';
import { LSConfigManager } from './ls-config.ts';
import {
    CSSPlugin,
    LSAndTSDocResolver,
    PluginHost,
    SveltePlugin,
    TypeScriptPlugin
} from './plugins/index.ts';
import { FileSystemProvider } from './plugins/css/FileSystemProvider.ts';
import { createLanguageServices } from './plugins/css/service.ts';
import type { LanguageServiceContainer } from './plugins/typescript/service.ts';
import { JSOrTSDocumentSnapshot } from './plugins/typescript/DocumentSnapshot.ts';
import { isInGeneratedCode } from './plugins/typescript/features/utils.ts';
import { convertRange, getDiagnosticTag, mapSeverity } from './plugins/typescript/utils.ts';
import { pathToUrl, urlToPath } from './utils.ts';
import { groupBy } from 'lodash-es';

/** A kind of diagnostic svelte-check can report. */
export type SvelteCheckDiagnosticSource = 'js' | 'css' | 'svelte';

/** Options for {@link SvelteCheck}. */
export interface SvelteCheckOptions {
    /** Svelte compiler warning codes to ignore or report as errors. */
    compilerWarnings?: Record<string, 'ignore' | 'error'>;
    /** The kinds of diagnostics to report; all of them when omitted. */
    diagnosticSources?: SvelteCheckDiagnosticSource[];
    /**
     * Path has to be absolute
     */
    tsconfig?: string;
    /** Called after a project reloads its configuration in watch mode. */
    onProjectReload?: () => void;
    /** Keep watching the workspace and re-check changed files. */
    watch?: boolean;
    /**
     * Optional callback invoked when a new snapshot is created.
     * Provides the absolute file path of the snapshot.
     */
    onFileSnapshotCreated?: (filePath: string) => void;
}

/**
 * Small wrapper around PluginHost's Diagnostic Capabilities
 * for svelte-check, without the overhead of the lsp.
 */
export class SvelteCheck {
    private docManager = new DocumentManager(
        (textDocument) => new Document(textDocument.uri, textDocument.text)
    );
    private configManager = new LSConfigManager();
    private pluginHost = new PluginHost(this.docManager);
    private lsAndTSDocResolver?: LSAndTSDocResolver;

    /** Starts checking `workspacePath`, the absolute path of the workspace root. */
    constructor(
        workspacePath: string,
        private options: SvelteCheckOptions = {}
    ) {
        Logger.setLogErrorsOnly(true);
        this.initialize(workspacePath, options);
    }

    /** Registers the plugins the enabled diagnostic sources need. */
    private async initialize(workspacePath: string, options: SvelteCheckOptions): Promise<void> {
        if (options.tsconfig && !isAbsolute(options.tsconfig)) {
            throw new Error(
                'tsconfigPath needs to be absolute, got ' + options.tsconfig
            );
        }

        this.configManager.update({
            svelte: {
                compilerWarnings: options.compilerWarnings
            }
        });
        // No HTMLPlugin, it does not provide diagnostics
        if (shouldRegister('svelte')) {
            this.pluginHost.register(new SveltePlugin(this.configManager));
        }
        if (shouldRegister('css')) {
            const services = createLanguageServices({
                fileSystemProvider: new FileSystemProvider()
            });
            const workspaceFolders: WorkspaceFolder[] = [
                {
                    name: '',
                    uri: pathToUrl(workspacePath)
                }
            ];
            this.pluginHost.register(
                new CSSPlugin(
                    this.docManager,
                    this.configManager,
                    workspaceFolders,
                    services
                )
            );
        }
        if (shouldRegister('js') || options.tsconfig) {
            const workspaceUris = [pathToUrl(workspacePath)];
            this.lsAndTSDocResolver = new LSAndTSDocResolver(
                this.docManager,
                workspaceUris,
                this.configManager,
                {
                    tsconfigPath: options.tsconfig,
                    isSvelteCheck: true,
                    onProjectReloaded: options.onProjectReload,
                    watch: options.watch,
                    onFileSnapshotCreated: options.onFileSnapshotCreated
                }
            );
            this.pluginHost.register(
                new TypeScriptPlugin(
                    this.configManager,
                    this.lsAndTSDocResolver,
                    workspaceUris,
                    this.docManager
                )
            );
        }

        function shouldRegister(source: SvelteCheckDiagnosticSource) {
            return !options.diagnosticSources ||
                options.diagnosticSources.includes(source);
        }
    }

    /**
     * Creates/updates given document
     *
     * @param doc Text and Uri of the document
     * @param isNew Whether or not this is the creation of the document
     */
    async upsertDocument(
        doc: { text: string; uri: string },
        isNew: boolean
    ): Promise<void> {
        const filePath = urlToPath(doc.uri) || '';

        if (this.options.tsconfig) {
            const lsContainer = await this.getLSContainer(this.options.tsconfig);
            if (!lsContainer.fileBelongsToProject(filePath, isNew)) {
                return;
            }
        }

        if (
            doc.uri.endsWith('.ts') ||
            doc.uri.endsWith('.js') ||
            doc.uri.endsWith('.tsx') ||
            doc.uri.endsWith('.jsx') ||
            doc.uri.endsWith('.mjs') ||
            doc.uri.endsWith('.cjs') ||
            doc.uri.endsWith('.mts') ||
            doc.uri.endsWith('.cts')
        ) {
            this.pluginHost.updateTsOrJsFile(filePath, [
                {
                    range: Range.create(
                        Position.create(0, 0),
                        Position.create(Number.MAX_VALUE, Number.MAX_VALUE)
                    ),
                    text: doc.text
                }
            ]);
        } else {
            this.docManager.openClientDocument({
                text: doc.text,
                uri: doc.uri
            });
        }
    }

    /**
     * Removes/closes document
     *
     * @param uri Uri of the document
     */
    async removeDocument(uri: string): Promise<void> {
        if (!this.docManager.get(uri)) {
            return;
        }

        this.docManager.closeDocument(uri);
        this.docManager.releaseDocument(uri);
        if (this.options.tsconfig) {
            const lsContainer = await this.getLSContainer(this.options.tsconfig);
            lsContainer.deleteSnapshot(urlToPath(uri) || '');
        }
    }

    /**
     * Gets the diagnostics for all currently open files.
     */
    async getDiagnostics(): Promise<
        Array<{ filePath: string; text: string; diagnostics: Diagnostic[] }>
    > {
        if (this.options.tsconfig) {
            return this.getDiagnosticsForTsconfig(this.options.tsconfig);
        }
        return await Promise.all(
            this.docManager.getAllOpenedByClient().map(async (doc) => {
                const uri = doc[1].uri;
                return await this.getDiagnosticsForFile(uri);
            })
        );
    }

    /** Diagnostics for every file the tsconfig at `tsconfigPath` includes. */
    private async getDiagnosticsForTsconfig(
        tsconfigPath: string
    ): Promise<Array<{ filePath: string; text: string; diagnostics: Diagnostic[] }>> {
        const lsContainer = await this.getLSContainer(tsconfigPath);
        const map = (diagnostic: ts.Diagnostic, range?: Range): Diagnostic => {
            const file = diagnostic.file;
            range ??= file
                ? convertRange(
                    { positionAt: file.getLineAndCharacterOfPosition.bind(file) },
                    diagnostic
                )
                : { start: { line: 0, character: 0 }, end: { line: 0, character: 0 } };

            return {
                range: range,
                severity: mapSeverity(diagnostic.category),
                source: diagnostic.source,
                message: ts.flattenDiagnosticMessageText(diagnostic.messageText, '\n'),
                code: diagnostic.code,
                tags: getDiagnosticTag(diagnostic)
            };
        };

        if (
            lsContainer.configErrors.some((error) => error.category === ts.DiagnosticCategory.Error)
        ) {
            return reportConfigError();
        }

        const lang = lsContainer.getService();
        if (
            lsContainer.configErrors.some((error) => error.category === ts.DiagnosticCategory.Error)
        ) {
            return reportConfigError();
        }

        const files = lang.getProgram()?.getSourceFiles() || [];
        const options = lang.getProgram()?.getCompilerOptions() || {};

        const diagnostics = await Promise.all(
            files.map((file) => {
                const uri = pathToUrl(file.fileName);
                const doc = this.docManager.get(uri);
                if (doc) {
                    this.docManager.markAsOpenedInClient(uri);
                    return this.getDiagnosticsForFile(uri);
                } else {
                    // This check is done inside TS mostly, too, but for some diagnostics like suggestions it
                    // doesn't apply to all code paths. That's why we do it here, too.
                    const skipDiagnosticsForFile =
                        (options.skipLibCheck && file.isDeclarationFile) ||
                        (options.skipDefaultLibCheck && file.hasNoDefaultLib) ||
                        lsContainer.isShimFiles(file.fileName) ||
                        // ignore JS files in node_modules
                        /\/node_modules\/.+\.(c|m)?js$/.test(file.fileName);
                    const snapshot = lsContainer.snapshotManager.get(file.fileName) as
                        | JSOrTSDocumentSnapshot
                        | undefined;
                    const isKitFile = snapshot?.kitFile ?? false;
                    const diagnostics: Diagnostic[] = [];
                    if (!skipDiagnosticsForFile) {
                        const diagnosticSources = [
                            'getSyntacticDiagnostics',
                            'getSuggestionDiagnostics',
                            'getSemanticDiagnostics'
                        ] as const;
                        for (const diagnosticSource of diagnosticSources) {
                            for (let diagnostic of lang[diagnosticSource](file.fileName)) {
                                if (!diagnostic.start || !diagnostic.length || !isKitFile) {
                                    diagnostics.push(map(diagnostic));
                                    continue;
                                }

                                let range: Range | undefined = undefined;
                                const inGenerated = isInGeneratedCode(
                                    file.text,
                                    diagnostic.start,
                                    diagnostic.start + diagnostic.length
                                );
                                if (inGenerated && snapshot) {
                                    const pos = snapshot.getOriginalPosition(
                                        snapshot.positionAt(diagnostic.start)
                                    );
                                    range = {
                                        start: pos,
                                        end: {
                                            line: pos.line,
                                            // adjust length so it doesn't spill over to the next line
                                            character: pos.character + 1
                                        }
                                    };
                                    // If not one of the specific error messages then filter out
                                    if (diagnostic.code === 2307) {
                                        diagnostic = {
                                            ...diagnostic,
                                            messageText:
                                                typeof diagnostic.messageText === 'string' &&
                                                    diagnostic.messageText.includes('./$types')
                                                    ? diagnostic.messageText +
                                                        ` (this likely means that SvelteKit's type generation didn't run yet - try running it by executing 'npm run dev' or 'npm run build')`
                                                    : diagnostic.messageText
                                        };
                                    } else if (diagnostic.code === 2694) {
                                        diagnostic = {
                                            ...diagnostic,
                                            messageText:
                                                typeof diagnostic.messageText === 'string' &&
                                                    diagnostic.messageText.includes('/$types')
                                                    ? diagnostic.messageText +
                                                        ` (this likely means that SvelteKit's generated types are out of date - try rerunning it by executing 'npm run dev' or 'npm run build')`
                                                    : diagnostic.messageText
                                        };
                                    } else if (
                                        diagnostic.code !==
                                            2355 /*  A function whose declared type is neither 'void' nor 'any' must return a value */
                                    ) {
                                        continue;
                                    }
                                }

                                diagnostics.push(map(diagnostic, range));
                            }
                        }
                    }

                    return {
                        filePath: file.fileName,
                        text: snapshot?.originalText ?? file.text,
                        diagnostics
                    };
                }
            })
        );

        if (lsContainer.configErrors.length) {
            diagnostics.push(...reportConfigError());
        }

        return diagnostics;

        function reportConfigError() {
            const grouped = groupBy(
                lsContainer.configErrors,
                (error) => error.file?.fileName ?? tsconfigPath
            );

            return Object.entries(grouped).map(([filePath, errors]) => ({
                filePath,
                text: '',
                diagnostics: (errors as ts.Diagnostic[]).map((diagnostic) => map(diagnostic))
            }));
        }
    }

    /** Diagnostics for the open document at `uri`. */
    private async getDiagnosticsForFile(
        uri: string
    ): Promise<{ filePath: string; text: string; diagnostics: Diagnostic[] }> {
        const diagnostics = await this.pluginHost.getDiagnostics({ uri });
        return {
            filePath: urlToPath(uri) || '',
            text: this.docManager.get(uri)?.getText() || '',
            diagnostics
        };
    }

    /** The language service that owns the tsconfig at `tsconfigPath`. */
    private getLSContainer(tsconfigPath: string): Promise<LanguageServiceContainer> {
        if (!this.lsAndTSDocResolver) {
            throw new Error(
                'Cannot run with tsconfig path without LS/TSdoc resolver'
            );
        }
        return this.lsAndTSDocResolver.getTSService(tsconfigPath);
    }

    /**
     * Gets the watch directories based on the tsconfig include patterns.
     * Returns null if no tsconfig is specified.
     */
    async getWatchDirectories(): Promise<
        { path: string; recursive: boolean }[] | null
    > {
        if (!this.options.tsconfig) {
            return null;
        }
        const lsContainer = await this.getLSContainer(this.options.tsconfig);
        const projectConfig = lsContainer.getProjectConfig();

        if (!projectConfig.wildcardDirectories) {
            return null;
        }

        return Object.entries(projectConfig.wildcardDirectories).map((
            [dir, flags]
        ) => ({
            path: dir,
            recursive: !!(flags & ts.WatchDirectoryFlags.Recursive)
        }));
    }
}
