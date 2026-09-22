import * as assert from 'assert';
import { readFileSync, writeFileSync } from 'fs';
import { join } from 'path';
import ts from 'typescript';
import { Document, DocumentManager } from '../../../../../src/lib/documents/index.ts';
import { LSConfigManager, type TsInlayHintsConfig } from '../../../../../src/ls-config.ts';
import { LSAndTSDocResolver } from '../../../../../src/plugins/index.ts';
import { InlayHintProviderImpl } from '../../../../../src/plugins/typescript/features/InlayHintProvider.ts';
import { pathToUrl } from '../../../../../src/utils.ts';
import {
    createJsonSnapshotFormatter,
    createSnapshotTester,
    updateSnapshotIfFailedOrEmpty
} from '../../test-utils.ts';
import { InlayHint } from 'vscode-languageserver-types';

function setup(workspaceDir: string, filePath: string) {
    const docManager = new DocumentManager(
        (textDocument) => new Document(textDocument.uri, textDocument.text)
    );
    const configManager = new LSConfigManager();
    const allEnable: TsInlayHintsConfig = {
        enumMemberValues: { enabled: true },
        functionLikeReturnTypes: { enabled: true },
        parameterNames: { enabled: 'all', suppressWhenArgumentMatchesName: false },
        parameterTypes: { enabled: true },
        propertyDeclarationTypes: { enabled: true },
        variableTypes: { enabled: true, suppressWhenTypeMatchesName: false }
    };
    configManager.updateTsJsUserPreferences({
        typescript: {
            inlayHints: allEnable
        },
        javascript: {
            inlayHints: allEnable
        }
    });
    const lsAndTsDocResolver = new LSAndTSDocResolver(
        docManager,
        [pathToUrl(workspaceDir)],
        configManager
    );
    const plugin = new InlayHintProviderImpl(lsAndTsDocResolver);
    const document = docManager.openClientDocument(
        <any> {
            uri: pathToUrl(filePath),
            text: ts.sys.readFile(filePath) || ''
        }
    );
    return { plugin, document, docManager, lsAndTsDocResolver };
}

async function executeTest(
    inputFile: string,
    {
        workspaceDir,
        dir
    }: {
        workspaceDir: string;
        dir: string;
    }
) {
    const expected = 'expectedv2.json';
    const { plugin, document } = setup(workspaceDir, inputFile);
    const workspaceUri = pathToUrl(workspaceDir);
    const inlayHints = sanitizeUri(
        await plugin.getInlayHints(document, {
            start: { line: 0, character: 0 },
            end: document.positionAt(document.getTextLength())
        })
    );

    const expectedFile = join(dir, expected);
    if (process.argv.includes('--debug')) {
        writeFileSync(join(dir, 'debug.svelte'), appendInlayHintAsComment());
    }

    const snapshotFormatter = await createJsonSnapshotFormatter(dir);

    await updateSnapshotIfFailedOrEmpty({
        assertion() {
            assert.deepStrictEqual(
                JSON.parse(JSON.stringify(inlayHints)),
                JSON.parse(readFileSync(expectedFile, 'utf-8'))
            );
        },
        expectedFile,
        getFileContent() {
            return snapshotFormatter(inlayHints);
        },
        rootDir: import.meta.dirname
    });

    function sanitizeUri(inlayHints: InlayHint[] | null) {
        if (!inlayHints) {
            return;
        }

        for (const inlayHint of inlayHints) {
            if (!Array.isArray(inlayHint.label)) {
                continue;
            }

            for (const label of inlayHint.label) {
                if (label.location) {
                    label.location.uri = label.location.uri.replace(
                        workspaceUri,
                        '<workspaceUri>'
                    );

                    const indexOfNodeModules = label.location.uri.lastIndexOf(
                        'node_modules'
                    );
                    if (indexOfNodeModules !== -1) {
                        label.location.uri = '<node_modules>' +
                            label.location.uri.slice(
                                indexOfNodeModules + 'node_modules'.length
                            );
                    }
                }
            }
        }

        return inlayHints;
    }

    function appendInlayHintAsComment() {
        if (!inlayHints) {
            return document.getText();
        }

        const offsetMap = new Map<number, string[]>();
        for (const inlayHint of inlayHints) {
            const offset = document.offsetAt(inlayHint.position);
            const text = Array.isArray(inlayHint.label)
                ? inlayHint.label.map((l) => l.value).join('')
                : inlayHint.label;

            const comment = `/*${inlayHint.paddingLeft ? ' ' : ''}${text}${
                inlayHint.paddingRight ? ' ' : ''
            }*/`;
            offsetMap.set(offset, (offsetMap.get(offset) ?? []).concat(comment));
        }

        const offsets = Array.from(offsetMap.keys()).sort((a, b) => a - b);
        const parts: string[] = [];

        for (let index = 0; index < offsets.length; index++) {
            const offset = offsets[index];
            parts.push(
                document.getText().slice(offsets[index - 1], offset),
                ...(offsetMap.get(offset) ?? [])
            );
        }

        parts.push(document.getText().slice(offsets[offsets.length - 1]));

        return parts.join('');
    }
}

const executeTests = createSnapshotTester(executeTest);

describe('InlayHintProvider', function () {
    executeTests({
        dir: join(import.meta.dirname, 'fixtures'),
        workspaceDir: join(import.meta.dirname, 'fixtures'),
        context: this
    });
});
