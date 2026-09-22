import path, { dirname, isAbsolute, join } from 'path';
import { existsSync, readdirSync, statSync, writeFileSync } from 'fs';
import ts from 'typescript';
import { format, resolveConfig } from 'prettier';
import { Document, DocumentManager } from '../../../src/lib/documents/index.ts';
import { FileMap } from '../../../src/lib/documents/fileCollection.ts';
import { LSConfigManager } from '../../../src/ls-config.ts';
import { LSAndTSDocResolver } from '../../../src/plugins/index.ts';
import {
    createGetCanonicalFileName,
    normalizePath,
    pathToUrl,
    urlToPath
} from '../../../src/utils.ts';
import { VERSION } from 'svelte/compiler';
import { findTsConfigPath } from '../../../src/plugins/typescript/utils.ts';
import { Position, Range } from 'vscode-languageserver';

const isSvelte5Plus = Number(VERSION.split('.')[0]) >= 5;

export interface FindSnippetOptions {
    /**
     * Only start looking for the snippet behind the first occurrence of this
     * anchor text. Useful to disambiguate short snippets such as `{` or a
     * function name that is also a prefix of another one.
     */
    after?: string;
    /** 1-based occurrence of the snippet within the searched region. Defaults to 1. */
    occurrence?: number;
}

/**
 * Reads a fixture from disk and returns the LSP {@link Position} of the first
 * character of `snippet`.
 *
 * WHY: test fixtures are formatted by `deno fmt` as part of the repo's verify
 * pipeline, which can change indentation, quote style and trailing commas.
 * Hardcoded line/character coordinates pointing into a fixture silently rot
 * whenever the fixture is reformatted, even though nothing about the behaviour
 * under test changed. Deriving the coordinates from the fixture's actual text
 * at test time keeps the assertions meaningful (they still pin down the exact
 * source construct) while making them immune to reformatting.
 */
export function positionOf(
    filePath: string,
    snippet: string,
    options?: FindSnippetOptions
): Position {
    const content = readFixture(filePath);
    return offsetToPosition(
        content,
        indexOfSnippet(content, filePath, snippet, options)
    );
}

/**
 * Reads a fixture from disk and returns the LSP {@link Range} spanning
 * `snippet`. See {@link positionOf} for why coordinates are derived instead of
 * hardcoded.
 */
export function rangeOf(
    filePath: string,
    snippet: string,
    options?: FindSnippetOptions
): Range {
    const content = readFixture(filePath);
    const start = indexOfSnippet(content, filePath, snippet, options);
    return {
        start: offsetToPosition(content, start),
        end: offsetToPosition(content, start + snippet.length)
    };
}

/**
 * Reads a fixture from disk and returns the LSP {@link Range} that starts at
 * `startSnippet` and ends behind the first `endSnippet` following it. Use this
 * for multi-line constructs (e.g. an object literal spanning `{` to `}`) whose
 * inner text is itself formatting dependent. See {@link positionOf} for why
 * coordinates are derived instead of hardcoded.
 */
export function rangeBetween(
    filePath: string,
    startSnippet: string,
    endSnippet: string,
    options?: FindSnippetOptions
): Range {
    const content = readFixture(filePath);
    const start = indexOfSnippet(content, filePath, startSnippet, options);
    const end = indexOfSnippet(content, filePath, endSnippet, {
        after: content.substring(0, start + startSnippet.length)
    });
    return {
        start: offsetToPosition(content, start),
        end: offsetToPosition(content, end + endSnippet.length)
    };
}

function readFixture(filePath: string): string {
    const content = ts.sys.readFile(filePath);

    if (content == null) {
        throw new Error(`Could not read fixture ${filePath}`);
    }

    return content;
}

function indexOfSnippet(
    content: string,
    filePath: string,
    snippet: string,
    options?: FindSnippetOptions
): number {
    let searchFrom = 0;

    if (options?.after != null) {
        const anchor = content.indexOf(options.after);

        if (anchor === -1) {
            throw new Error(
                `Could not find anchor ${JSON.stringify(options.after)} in ${filePath}`
            );
        }

        searchFrom = anchor + options.after.length;
    }

    const occurrence = options?.occurrence ?? 1;
    let index = -1;

    for (let i = 0; i < occurrence; i++) {
        index = content.indexOf(snippet, searchFrom);

        if (index === -1) {
            throw new Error(
                `Could not find occurrence ${occurrence} of ${
                    JSON.stringify(snippet)
                } in ${filePath}`
            );
        }

        searchFrom = index + Math.max(snippet.length, 1);
    }

    return index;
}

function offsetToPosition(content: string, offset: number): Position {
    let line = 0;
    let lineStart = 0;

    for (let i = 0; i < offset; i++) {
        if (content[i] === '\n') {
            line++;
            lineStart = i + 1;
        }
    }

    return { line, character: offset - lineStart };
}

export function createVirtualTsSystem(currentDirectory: string): ts.System {
    const virtualFs = new FileMap<string>();
    // array behave more similar to the actual fs event than Set
    const watchers = new FileMap<ts.FileWatcherCallback[]>();
    const watchTimeout = new FileMap<Array<ReturnType<typeof setTimeout>>>();
    const getCanonicalFileName = createGetCanonicalFileName(
        ts.sys.useCaseSensitiveFileNames
    );
    const modifiedTime = new FileMap<Date>();

    function toAbsolute(path: string) {
        return isAbsolute(path) ? path : join(currentDirectory, path);
    }

    const virtualSystem: ts.System = {
        ...ts.sys,
        getCurrentDirectory() {
            return currentDirectory;
        },
        writeFile(path, data) {
            const normalizedPath = normalizePath(toAbsolute(path));
            const existsBefore = virtualFs.has(normalizedPath);
            virtualFs.set(normalizedPath, data);
            modifiedTime.set(normalizedPath, new Date());
            triggerWatch(
                normalizedPath,
                existsBefore ? ts.FileWatcherEventKind.Changed : ts.FileWatcherEventKind.Created
            );
        },
        readFile(path) {
            return virtualFs.get(normalizePath(toAbsolute(path)));
        },
        fileExists(path) {
            return virtualFs.has(normalizePath(toAbsolute(path)));
        },
        directoryExists(path) {
            const normalizedPath = getCanonicalFileName(
                normalizePath(toAbsolute(path))
            );
            return Array.from(virtualFs.keys()).some((fileName) =>
                fileName.startsWith(normalizedPath)
            );
        },
        deleteFile(path) {
            const normalizedPath = normalizePath(toAbsolute(path));
            const existsBefore = virtualFs.has(normalizedPath);
            virtualFs.delete(normalizedPath);

            if (existsBefore) {
                triggerWatch(normalizedPath, ts.FileWatcherEventKind.Deleted);
            }
        },
        watchFile(path, callback) {
            const normalizedPath = normalizePath(toAbsolute(path));
            let watchersOfPath = watchers.get(normalizedPath);

            if (!watchersOfPath) {
                watchersOfPath = [];
                watchers.set(normalizedPath, watchersOfPath);
            }

            watchersOfPath.push(callback);

            return {
                close() {
                    const watchersOfPath = watchers.get(normalizedPath);

                    if (watchersOfPath) {
                        watchers.set(
                            normalizedPath,
                            watchersOfPath.filter((watcher) => watcher === callback)
                        );
                    }

                    const timeouts = watchTimeout.get(normalizedPath);

                    if (timeouts != null) {
                        timeouts.forEach((timeout) => clearTimeout(timeout));
                    }
                }
            };
        },
        getModifiedTime(path) {
            return modifiedTime.get(normalizePath(toAbsolute(path)));
        },
        readDirectory(path, _extensions, _exclude, include, _depth) {
            if (include && (include.length != 1 || include[0] !== '**/*')) {
                throw new Error(
                    'include pattern matching not implemented. Mock it if the test needs it. Pattern: ' +
                        include
                );
            }

            const normalizedPath = getCanonicalFileName(
                normalizePath(toAbsolute(path))
            );
            return Array.from(virtualFs.keys()).filter((fileName) =>
                fileName.startsWith(normalizedPath)
            );
        }
    };

    return virtualSystem;

    function triggerWatch(normalizedPath: string, kind: ts.FileWatcherEventKind) {
        // if watcher is not set yet. don't trigger it
        if (!watchers.has(normalizedPath)) {
            return;
        }

        let timeoutsOfPath = watchTimeout.get(normalizedPath);

        if (!timeoutsOfPath) {
            timeoutsOfPath = [];
            watchTimeout.set(normalizedPath, timeoutsOfPath);
        }

        timeoutsOfPath.push(
            setTimeout(
                () =>
                    watchers
                        .get(normalizedPath)
                        ?.forEach((callback) => callback(normalizedPath, kind)),
                0
            )
        );
    }
}

export function getRandomVirtualDirPath(testDir: string) {
    return path.join(
        testDir,
        `virtual-path-${Math.floor(Math.random() * 100_000)}`
    );
}

interface VirtualEnvironmentOptions {
    testDir: string;
    filename: string;
    fileContent: string;
}

export function setupVirtualEnvironment({
    testDir,
    fileContent,
    filename
}: VirtualEnvironmentOptions) {
    const docManager = new DocumentManager(
        (textDocument) => new Document(textDocument.uri, textDocument.text)
    );

    const lsConfigManager = new LSConfigManager();

    const virtualSystem = createVirtualTsSystem(testDir);
    const lsAndTsDocResolver = new LSAndTSDocResolver(
        docManager,
        [pathToUrl(testDir)],
        lsConfigManager,
        {
            tsSystem: virtualSystem
        }
    );

    const filePath = join(testDir, filename);
    virtualSystem.writeFile(filePath, fileContent);
    const document = docManager.openClientDocument(
        <any> {
            uri: pathToUrl(filePath),
            text: virtualSystem.readFile(filePath) || ''
        }
    );

    return {
        lsAndTsDocResolver,
        document,
        docManager,
        virtualSystem,
        lsConfigManager
    };
}

export function createSnapshotTester<
    TestOptions extends {
        dir: string;
        workspaceDir: string;
        context: Mocha.Suite;
    }
>(executeTest: (inputFile: string, testOptions: TestOptions) => Promise<void>) {
    return (testOptions: TestOptions) => {
        serviceWarmup(
            testOptions.context,
            testOptions.dir,
            pathToUrl(testOptions.workspaceDir)
        );
        executeTests(testOptions);
    };

    function executeTests(testOptions: TestOptions) {
        const { dir } = testOptions;
        const workspaceUri = pathToUrl(testOptions.workspaceDir);

        const inputFile = join(dir, 'input.svelte');
        const tsconfig = join(dir, 'tsconfig.json');
        const jsconfig = join(dir, 'jsconfig.json');

        if (existsSync(tsconfig) || existsSync(jsconfig)) {
            serviceWarmup(testOptions.context, dir, workspaceUri);
        }

        if (existsSync(inputFile)) {
            const _it = dir.endsWith('.v5') && !isSvelte5Plus
                ? it.skip
                : dir.endsWith('.only')
                ? it.only
                : it;
            _it(
                dir.substring(import.meta.dirname.length),
                () => executeTest(inputFile, testOptions)
            );
        } else {
            const _describe = dir.endsWith('.only') ? describe.only : describe;
            _describe(dir.substring(import.meta.dirname.length), function () {
                const subDirs = readdirSync(dir);

                for (const subDir of subDirs) {
                    const stat = statSync(join(dir, subDir));
                    if (stat.isDirectory()) {
                        executeTests({
                            ...testOptions,
                            context: this,
                            dir: join(dir, subDir)
                        });
                    }
                }
            });
        }
    }
}

export async function updateSnapshotIfFailedOrEmpty({
    assertion,
    expectedFile,
    rootDir,
    getFileContent
}: {
    assertion: () => void;
    expectedFile: string;
    rootDir: string;
    getFileContent: () => string | Promise<string>;
}) {
    if (existsSync(expectedFile)) {
        try {
            assertion();
        } catch (e) {
            if (process.argv.includes('--auto')) {
                await writeFile(`Updated ${expectedFile} for`);
            } else {
                throw e;
            }
        }
    } else {
        await writeFile(`Created ${expectedFile} for`);
    }

    async function writeFile(msg: string) {
        console.info(msg, dirname(expectedFile).substring(rootDir.length));
        writeFileSync(expectedFile, await getFileContent(), 'utf-8');
    }
}

export async function createJsonSnapshotFormatter(dir: string) {
    const prettierOptions = await resolveConfig(dir);

    return (obj: any) =>
        format(JSON.stringify(obj), {
            ...prettierOptions,
            parser: 'json'
        });
}

export function serviceWarmup(
    suite: Mocha.Suite,
    testDir: string,
    rootUri = pathToUrl(testDir),
    tsconfigPath: string | undefined = undefined
) {
    const defaultTimeout = suite.timeout();

    // allow to set a higher timeout for slow machines from cli flag
    const warmupTimeout = Math.max(defaultTimeout, 5_000);
    suite.timeout(warmupTimeout);
    before(() => warmup(tsconfigPath));

    suite.timeout(defaultTimeout);

    async function warmup(configFilePath: string | undefined = undefined) {
        const start = Date.now();
        console.log('Warming up language service...');

        const docManager = new DocumentManager(
            (textDocument) => new Document(textDocument.uri, textDocument.text)
        );

        const lsAndTsDocResolver = new LSAndTSDocResolver(
            docManager,
            [rootUri],
            new LSConfigManager()
        );

        configFilePath ??= findTsConfigPath(
            join(testDir, 'DoesNotMater.svelte'),
            [rootUri],
            ts.sys.fileExists,
            createGetCanonicalFileName(ts.sys.useCaseSensitiveFileNames)
        );

        const ls = await lsAndTsDocResolver.getTSServiceByConfigPath(
            configFilePath,
            configFilePath ? dirname(configFilePath) : urlToPath(rootUri)!
        );
        ls.getService();

        const projectReferences = ls.getResolvedProjectReferences();

        if (projectReferences.length) {
            await Promise.all(
                projectReferences.map((ref) => warmup(ref.configFilePath))
            );
        }

        console.log(`Service warming up done in ${Date.now() - start}ms`);
    }
}

export function recursiveServiceWarmup(
    suite: Mocha.Suite,
    testDir: string,
    rootUri = pathToUrl(testDir)
) {
    serviceWarmup(suite, testDir, rootUri);
    recursiveServiceWarmupNonRoot(suite, testDir, rootUri);
}

function recursiveServiceWarmupNonRoot(
    suite: Mocha.Suite,
    testDir: string,
    rootUri = pathToUrl(testDir)
) {
    const subDirs = readdirSync(testDir);

    for (const subDirOrFile of subDirs) {
        const stat = statSync(join(testDir, subDirOrFile));

        if (
            stat.isFile() &&
            (subDirOrFile === 'tsconfig.json' || subDirOrFile === 'jsconfig.json')
        ) {
            serviceWarmup(suite, testDir, rootUri);
        }

        if (stat.isDirectory()) {
            recursiveServiceWarmupNonRoot(
                suite,
                join(testDir, subDirOrFile),
                rootUri
            );
        }
    }
}
