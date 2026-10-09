import assert from 'node:assert/strict';
import path from 'node:path';
import { describe, test } from 'node:test';
import { repoRoot } from './test-utils.mjs';

// Use dynamic import for TypeScript to work in both Node and Deno
const ts = await import('typescript').then((m) => m.default ?? m);

function createMockLanguageService(tsModule, fileName, fileText) {
    const sourceFile = tsModule.createSourceFile(
        fileName,
        fileText,
        tsModule.ScriptTarget.Latest,
        true
    );

    return {
        getProgram: () => ({
            getSourceFile: (
                requested
            ) => (requested === fileName ? sourceFile : undefined)
        }),
        getSemanticDiagnostics: () => [],
        getSyntacticDiagnostics: () => [],
        getQuickInfoAtPosition: () => undefined,
        getCompletionsAtPosition: () => undefined,
        getDefinitionAtPosition: () => undefined,
        getDefinitionAndBoundSpan: () => undefined,
        getTypeDefinitionAtPosition: () => undefined,
        getReferencesAtPosition: () => undefined,
        findReferences: () => undefined,
        getSignatureHelpItems: () => undefined,
        getRenameInfo: () => ({ canRename: false }),
        findRenameLocations: () => undefined,
        getDocumentHighlights: () => undefined,
        getImplementationAtPosition: () => undefined,
        getCodeFixesAtPosition: () => [],
        getNavigationTree: () => ({
            text: '',
            kind: '',
            spans: [],
            childItems: []
        }),
        getOutliningSpans: () => []
    };
}

function createHost(tsModule, fileName, fileText, cwd) {
    const snapshot = tsModule.ScriptSnapshot.fromString(fileText);

    return {
        getCompilationSettings: () => ({
            strict: true,
            target: tsModule.ScriptTarget.ES2022,
            module: tsModule.ModuleKind.ESNext
        }),
        getScriptFileNames: () => [fileName],
        getScriptVersion: () => '1',
        getScriptSnapshot: (
            requested
        ) => (requested === fileName ? snapshot : undefined),
        getCurrentDirectory: () => cwd,
        getDefaultLibFileName: (opts) => tsModule.getDefaultLibFilePath(opts),
        fileExists: (p) => p === fileName,
        readFile: () => undefined,
        readDirectory: () => []
    };
}

async function initPluginForFile({ fileName, fileText }) {
    const pluginModule = await import('@macroforge/typescript-plugin');
    const tsPluginInit = pluginModule.default;
    const pluginFactory = tsPluginInit({ typescript: ts });

    const cwd = repoRoot;
    const host = createHost(ts, fileName, fileText, cwd);
    const languageService = createMockLanguageService(ts, fileName, fileText);

    const info = {
        project: {
            getCurrentDirectory: () => cwd,
            projectService: { logger: { info: () => {} } }
        },
        languageService,
        languageServiceHost: host,
        config: {}
    };

    pluginFactory.create(info);
    return info.languageService;
}

describe('TypeScript plugin macro hover + attribute diagnostics', () => {
    test('hover over @derive macro name returns QuickInfo docs', async () => {
        const fileText = `
      /** @derive(Encode, Decode) */
      export interface User {
        id: string;
      }
    `;
        const fileName = path.join(
            repoRoot,
            'testground/tests/.tmp-macro-hover.ts'
        );
        const ls = await initPluginForFile({ fileName, fileText });

        const pos = fileText.indexOf('Encode') + 1;
        assert.ok(pos > 0, 'sanity: expected to find Encode');

        const info = ls.getQuickInfoAtPosition(fileName, pos);
        assert.ok(info, 'expected QuickInfo for Encode');
        const docText = (info.documentation ?? []).map((d) => d.text).join('\n');
        assert.ok(
            docText.toLowerCase().includes('encoding methods'),
            `expected Encode documentation, got: ${docText}`
        );
        assert.equal(
            fileText.slice(
                info.textSpan.start,
                info.textSpan.start + info.textSpan.length
            ),
            'Encode'
        );
    });

    test('hover over @endec decorator returns QuickInfo docs', async () => {
        const fileText = `
      /** @derive(Decode) */
      export interface User {
        /** @endec({ validate: ["email"] }) */
        email: string;
      }
    `;
        const fileName = path.join(
            repoRoot,
            'testground/tests/.tmp-decorator-hover.ts'
        );
        const ls = await initPluginForFile({ fileName, fileText });

        const pos = fileText.indexOf('@endec') + 2;
        assert.ok(pos > 1, 'sanity: expected to find @endec');

        const info = ls.getQuickInfoAtPosition(fileName, pos);
        assert.ok(info, 'expected QuickInfo for @endec');
        const docText = (info.documentation ?? []).map((d) => d.text).join('\n');
        assert.ok(
            docText.toLowerCase().includes('configure') &&
                docText.toLowerCase().includes('for this field'),
            `expected endec documentation, got: ${docText}`
        );
        assert.equal(
            fileText.slice(
                info.textSpan.start,
                info.textSpan.start + info.textSpan.length
            ),
            '@endec'
        );
    });

    test('invalid @endec validate attribute surfaces as semantic diagnostic', async () => {
        const fileText = `
      /** @derive(Decode) */
      export interface User {
        /** @endec({ validate: ["doesNotExist"] }) */
        name: string;
      }
    `;
        const fileName = path.join(repoRoot, 'testground/tests/.tmp-endec-diag.ts');
        const ls = await initPluginForFile({ fileName, fileText });

        const diags = ls.getSemanticDiagnostics(fileName);
        const macroDiags = diags.filter((d) => d.source === 'macroforge');
        assert.ok(
            macroDiags.length >= 1,
            'expected at least one macroforge diagnostic'
        );

        const message = String(macroDiags[0].messageText);
        assert.ok(
            message.toLowerCase().includes('unknown validator'),
            `expected unknown validator error, got: ${message}`
        );
        assert.ok(
            typeof macroDiags[0].start === 'number' && macroDiags[0].start > 0,
            'expected diagnostic start to be set'
        );

        const endecStart = fileText.indexOf('@endec');
        assert.ok(endecStart >= 0, 'sanity: expected @endec in file');
        assert.ok(
            macroDiags[0].start >= endecStart - 4 &&
                macroDiags[0].start <= endecStart + 4,
            `expected diagnostic near @endec (got start=${
                macroDiags[0].start
            }, endecStart=${endecStart})`
        );
    });
});
