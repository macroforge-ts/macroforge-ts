import * as assert from 'assert';
import { existsSync, readFileSync } from 'fs';
import { join } from 'path';
import ts from 'typescript';
import { Document, DocumentManager } from '../../../../../src/lib/documents/index.ts';
import { LSConfigManager } from '../../../../../src/ls-config.ts';
import { LSAndTSDocResolver } from '../../../../../src/plugins/index.ts';
import { DiagnosticsProviderImpl } from '../../../../../src/plugins/typescript/features/DiagnosticsProvider.ts';
import { __resetCache } from '../../../../../src/plugins/typescript/service.ts';
import { pathToUrl } from '../../../../../src/utils.ts';
import {
    createJsonSnapshotFormatter,
    createSnapshotTester,
    updateSnapshotIfFailedOrEmpty
} from '../../test-utils.ts';
import { packageLoader } from '../../../../../src/importPackage.ts';

function setup(workspaceDir: string, filePath: string) {
    const docManager = new DocumentManager(
        (textDocument) => new Document(textDocument.uri, textDocument.text)
    );
    const configManager = new LSConfigManager();
    const lsAndTsDocResolver = new LSAndTSDocResolver(
        docManager,
        [pathToUrl(workspaceDir)],
        configManager
    );
    const plugin = new DiagnosticsProviderImpl(lsAndTsDocResolver, configManager);
    const document = docManager.openClientDocument(
        <any> {
            uri: pathToUrl(filePath),
            text: ts.sys.readFile(filePath) || ''
        }
    );
    return { plugin, document, docManager, lsAndTsDocResolver };
}

const {
    version: { major }
} = packageLoader.getPackageInfo('svelte', import.meta.dirname);
const expected = 'expectedv2.json';
const newSvelteMajorExpected = `expected_svelte_${major}.json`;

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
    const { plugin, document } = setup(workspaceDir, inputFile);
    const diagnostics = await plugin.getDiagnostics(document);

    const defaultExpectedFile = join(dir, expected);
    const expectedFileForCurrentSvelteMajor = join(dir, newSvelteMajorExpected);
    const expectedFile = existsSync(expectedFileForCurrentSvelteMajor)
        ? expectedFileForCurrentSvelteMajor
        : defaultExpectedFile;
    const snapshotFormatter = await createJsonSnapshotFormatter(dir);

    await updateSnapshotIfFailedOrEmpty({
        assertion() {
            assert.deepStrictEqual(
                diagnostics,
                JSON.parse(readFileSync(expectedFile, 'utf-8'))
            );
        },
        expectedFile,
        getFileContent() {
            return snapshotFormatter(diagnostics);
        },
        rootDir: import.meta.dirname
    });
}

const executeTests = createSnapshotTester(executeTest);

describe('DiagnosticsProvider', function () {
    executeTests({
        dir: join(import.meta.dirname, 'fixtures'),
        workspaceDir: join(import.meta.dirname, 'fixtures'),
        context: this
    });

    // Hacky, but it works. Needed due to testing both new and old transformation
    after(() => {
        __resetCache();
    });
});
