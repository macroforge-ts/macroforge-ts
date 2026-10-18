/**
 * @module @macroforge/deno-plugin/project
 *
 * Project-wide expansion: walk a directory tree, expand every TypeScript
 * module containing macro annotations, and write the output to a mirror
 * directory. This is the workflow that lets you run a Deno app without
 * Vite: point Deno at the mirror via an `imports` entry in `deno.json`.
 */

import * as path from '@std/path';
import { ensureDir, walk } from '@std/fs';
import { escape } from '@std/regexp';
import { hasMacroAnnotations } from '@macroforge/core';
import { TYPESCRIPT_EXTENSIONS } from '@macroforge/shared';
import { expand, type ExpandOptions, residentRegistries } from './index.ts';

/** Options for expanding a whole project tree into a mirror directory. */
export interface ExpandProjectOptions extends ExpandOptions {
    /** Project root that's walked. Defaults to `Deno.cwd()`. */
    root?: string;
    /** Output directory (relative to `root`). Defaults to `.macroforge/cache`. */
    outDir?: string;
    /** Glob-like extensions to include. Defaults to `['.ts', '.tsx', '.mts', '.cts']`. */
    extensions?: string[];
    /** Subpath substrings to skip. Defaults to common build/cache dirs. */
    exclude?: string[];
    /** When true, copy non-macro files through to the mirror unchanged so the mirror is self-contained. */
    copyPassthrough?: boolean;
    /** Called for each processed file; useful for CLI progress output. */
    onFile?: (event: ExpandFileEvent) => void;
}

/** One file the project expansion visited. */
export interface ExpandFileEvent {
    /** Absolute source path. */
    source: string;
    /** Absolute destination path in the mirror. */
    dest: string;
    /** True when expansion produced different output than the input. */
    expanded: boolean;
    /** True when the file was skipped because no macro annotations were found. */
    skipped: boolean;
    /**
     * True when the engine reported an error for the file, or the file could
     * not be read or expanded at all. Nothing is written for it, and what an
     * earlier expansion wrote is removed.
     */
    failed: boolean;
    /** Diagnostics emitted by the engine (forwarded for logging). */
    diagnostics: Array<{ level: string; message: string }>;
}

const DEFAULT_EXCLUDES = [
    '/node_modules/',
    '/.macroforge/',
    '/target/',
    '/dist/',
    '/build/',
    '/.git/'
];

/**
 * A pattern for an excluded path segment. A segment that ends in `/` names a
 * directory, so it also matches the directory's own path, which has no
 * trailing separator, and the walk skips it whole.
 */
function excludePattern(segment: string): RegExp {
    return segment.endsWith('/')
        ? new RegExp(`${escape(segment.slice(0, -1))}(/|$)`)
        : new RegExp(escape(segment));
}

/** What a file is expanded with: where the project is, and its engine. */
interface FileExpansion {
    root: string;
    outDir: string;
    copyPassthrough: boolean;
    expandOptions: ExpandOptions;
}

/**
 * Removes what an earlier expansion wrote at `dest`, when anything is there:
 * the mirror never holds code its source no longer expands to.
 */
async function removeStale(dest: string): Promise<void> {
    try {
        await Deno.remove(dest);
    } catch (error) {
        if (!(error instanceof Deno.errors.NotFound)) throw error;
    }
}

/**
 * Expands one source file into the mirror and says what came of it. A file
 * whose macros report an error, or that cannot be read or expanded at all,
 * is failed: nothing is written for it, and what was written before is
 * removed.
 */
async function expandFile(source: string, into: FileExpansion): Promise<ExpandFileEvent> {
    const dest = path.join(into.outDir, path.relative(into.root, source));
    const write = async (code: string) => {
        await ensureDir(path.dirname(dest));
        await Deno.writeTextFile(dest, code);
    };
    try {
        const code = await Deno.readTextFile(source);
        if (!hasMacroAnnotations(code, source)) {
            await (into.copyPassthrough ? write(code) : removeStale(dest));
            return { source, dest, expanded: false, skipped: true, failed: false, diagnostics: [] };
        }

        const result = expand(code, source, { ...into.expandOptions, projectRoot: into.root });
        // Code a macro reported an error in is not written, as a compile
        // error writes none.
        const failed = result.diagnostics.some((diag) => diag.level === 'error');
        await (failed ? removeStale(dest) : write(result.code));
        return {
            source,
            dest,
            expanded: result.hasMacros && !failed,
            skipped: false,
            failed,
            diagnostics: result.diagnostics
        };
    } catch (error) {
        return failure(source, dest, error);
    }
}

/** What went wrong, in the words it was thrown with. */
function described(error: unknown): string {
    return error instanceof Error ? error.message : String(error);
}

/**
 * The event of a file that could not be expanded for `error`. Its copy in
 * the mirror is removed, and a copy that cannot be is reported beside it.
 */
async function failure(source: string, dest: string, error: unknown): Promise<ExpandFileEvent> {
    const diagnostics = [{ level: 'error', message: described(error) }];
    try {
        await removeStale(dest);
    } catch (stale) {
        diagnostics.push({
            level: 'error',
            message: `could not remove ${dest}: ${described(stale)}`
        });
    }
    return { source, dest, expanded: false, skipped: false, failed: true, diagnostics };
}

/** Whether a file is at `source` now. */
async function isThere(source: string): Promise<boolean> {
    try {
        return (await Deno.stat(source)).isFile;
    } catch (error) {
        if (error instanceof Deno.errors.NotFound) return false;
        throw error;
    }
}

/** The projects whose missing registry has been said: once each is enough. */
const saidNoRegistry = new Set<string>();

/** One of the files `macroforge cache` writes for the project at `root`. */
async function writtenByCli(root: string, name: string): Promise<string | undefined> {
    try {
        return await Deno.readTextFile(path.join(root, '.macroforge', name));
    } catch (error) {
        if (error instanceof Deno.errors.NotFound) return undefined;
        throw error;
    }
}

/**
 * `options` with the registries of the project at `root`, for a caller that
 * hands none in: the ones `macroforge cache` and `macroforge watch` write
 * under `.macroforge/`. A derive knows a type declared in another file
 * through them alone, so a project without them is expanded with each file
 * on its own, and a warning says so.
 */
async function withProjectRegistries(
    root: string,
    options: ExpandProjectOptions
): Promise<ExpandProjectOptions> {
    if (options.typeRegistryJson !== undefined || options.typeRegistryId !== undefined) {
        return options;
    }
    const typeRegistryJson = await writtenByCli(root, 'type-registry.json');
    if (typeRegistryJson === undefined) {
        if (!saidNoRegistry.has(root)) {
            saidNoRegistry.add(root);
            console.warn(
                `[@macroforge/deno-plugin] No type registry at ${
                    path.join(root, '.macroforge', 'type-registry.json')
                }: a type declared in another file is unknown to the derives, so a class is expanded without the fields it inherits from one. Run \`macroforge cache\` in the project to write it.`
            );
        }
        return options;
    }
    // A registry lists its files by the paths they were scanned at.
    const scannedElsewhere = scannedFiles(typeRegistryJson).find((file) =>
        path.relative(root, file).startsWith('..')
    );
    if (scannedElsewhere !== undefined) {
        console.warn(
            `[@macroforge/deno-plugin] Ignoring .macroforge/type-registry.json: it was scanned at another location (it lists ${scannedElsewhere}). Run \`macroforge cache\` to write it again.`
        );
        return options;
    }
    const declarativeRegistryJson = options.declarativeRegistryJson ??
        (options.declarativeRegistryId === undefined
            ? await writtenByCli(root, 'declarative-registry.json')
            : undefined);
    return { ...options, typeRegistryJson, declarativeRegistryJson };
}

/** The files a type registry lists, each by the path it was scanned at. */
function scannedFiles(typeRegistryJson: string): string[] {
    const entries: unknown = JSON.parse(typeRegistryJson).entries;
    if (typeof entries !== 'object' || entries === null) return [];
    return Object.values(entries).flatMap((entry) =>
        typeof entry === 'object' && entry !== null && 'file_path' in entry &&
            typeof entry.file_path === 'string'
            ? [entry.file_path]
            : []
    );
}

/** When the project's type registry was last written, or nothing where it is not there. */
async function registryStamp(root: string): Promise<number | undefined> {
    try {
        return (await Deno.stat(path.join(root, '.macroforge', 'type-registry.json'))).mtime
            ?.getTime();
    } catch (error) {
        if (error instanceof Deno.errors.NotFound) return undefined;
        throw error;
    }
}

/** The project `options` describe, with the paths the walk and the watch leave out. */
function projectLayout(options: ExpandProjectOptions) {
    const root = path.resolve(options.root ?? Deno.cwd());
    const outDir = path.resolve(root, options.outDir ?? '.macroforge/cache');
    // The mirror is itself under `root`: it is never read as a source.
    const skip = [
        new RegExp(`^${escape(outDir)}(${escape(path.SEPARATOR)}|$)`),
        ...[...DEFAULT_EXCLUDES, ...(options.exclude ?? [])].map(excludePattern)
    ];
    return {
        root,
        outDir,
        skip,
        extensions: options.extensions ?? [...TYPESCRIPT_EXTENSIONS],
        copyPassthrough: options.copyPassthrough ?? false
    };
}

/**
 * Expand every macro-bearing file under `root` and write the output to
 * `<root>/<outDir>/`. Returns the per-file events so callers can summarize.
 */
export async function expandProject(
    options: ExpandProjectOptions = {}
): Promise<ExpandFileEvent[]> {
    const { root, outDir, skip, extensions, copyPassthrough } = projectLayout(options);
    const events: ExpandFileEvent[] = [];
    const resident = residentRegistries(await withProjectRegistries(root, options));
    try {
        // Excluded directories are pruned rather than walked and filtered.
        for await (
            const entry of walk(root, { includeDirs: false, exts: extensions, skip })
        ) {
            const event = await expandFile(entry.path, {
                root,
                outDir,
                copyPassthrough,
                expandOptions: resident.options
            });
            events.push(event);
            options.onFile?.(event);
        }
    } finally {
        resident.release();
    }

    return events;
}

/**
 * Watch `root` for changes and re-expand affected files. Returns a function to
 * stop watching. The initial `expandProject` call is awaited before the
 * watcher starts so the mirror is always populated before file events fire.
 *
 * A file that is removed or renamed away takes its copy in the mirror with
 * it. A change that cannot be expanded is reported through `onFile` as a
 * failed event, as the first run reports one.
 */
export async function watchProject(
    options: ExpandProjectOptions = {}
): Promise<() => void> {
    const { root, outDir, skip, extensions, copyPassthrough } = projectLayout(options);

    await expandProject(options);

    let resident = residentRegistries(await withProjectRegistries(root, options));
    let stamp = await registryStamp(root);
    const watcher = Deno.watchFs(root, { recursive: true });

    (async () => {
        for await (const event of watcher) {
            // A save may arrive as a write, or as a file renamed over the old
            // one: what is at the path now says which it was.
            if (!['create', 'modify', 'rename', 'remove'].includes(event.kind)) continue;
            // The registry `macroforge watch` rewrites is read again when it has.
            const written = await registryStamp(root);
            if (written !== stamp) {
                stamp = written;
                resident.release();
                resident = residentRegistries(await withProjectRegistries(root, options));
            }
            for (const changed of event.paths) {
                if (!extensions.some((ext) => changed.endsWith(ext))) continue;
                if (skip.some((pattern) => pattern.test(changed))) continue;

                const dest = path.join(outDir, path.relative(root, changed));
                try {
                    if (await isThere(changed)) {
                        options.onFile?.(
                            await expandFile(changed, {
                                root,
                                outDir,
                                copyPassthrough,
                                expandOptions: resident.options
                            })
                        );
                    } else {
                        await removeStale(dest);
                    }
                } catch (error) {
                    options.onFile?.(await failure(changed, dest, error));
                }
            }
        }
    })();

    return () => {
        watcher.close();
        resident.release();
    };
}
