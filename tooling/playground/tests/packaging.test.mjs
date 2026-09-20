/**
 * Packaged-library expansion tests.
 *
 * Reproduces and guards the fix for: a library packaged with svelte-package
 * ships its macro-annotated type modules UNEXPANDED, because the packager's
 * plain TS transpile strips the `@derive` JSDoc and the Svelte preprocessor
 * only expands `.svelte` components (never standalone `.svelte.ts` modules).
 *
 * `@playground/library` exposes two build paths:
 *  - `package:unexpanded` runs svelte-package directly (the bug)
 *  - `build` runs `macroforge expand --scan --out <staging>` first, then
 *    svelte-package over the staged sources (the supported chain)
 */

import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import {
    existsSync,
    mkdirSync,
    readdirSync,
    readFileSync,
    rmSync,
    statSync,
    writeFileSync
} from 'node:fs';
import path from 'node:path';
import { test } from 'node:test';
import { pathToFileURL } from 'node:url';
import { cliBinary, playgroundRoot } from './test-utils.mjs';

const libraryRoot = path.join(playgroundRoot, 'library');

/** Ensure the library's node_modules exist (deno-managed, matches siblings). */
function ensureInstalled() {
    if (existsSync(path.join(libraryRoot, 'node_modules'))) return;
    const result = spawnSync('deno', ['install', '--node-modules-dir'], {
        cwd: libraryRoot,
        stdio: 'inherit'
    });
    if (result.status !== 0) {
        throw new Error('`deno install` failed for @playground/library');
    }
}

/** Run a library deno task, returning captured output. */
function runLibraryTask(taskName) {
    return spawnSync('deno', ['task', taskName], {
        cwd: libraryRoot,
        encoding: 'utf8'
    });
}

/** Run the macroforge CLI directly in the library, returning captured output. */
function runMacroforge(args) {
    return spawnSync(cliBinary, args, {
        cwd: libraryRoot,
        encoding: 'utf8'
    });
}

/** Names of scratch directories the packaging swap creates next to `dist`. */
function swapScratchDirs() {
    return readdirSync(libraryRoot).filter(
        (name) =>
            name.includes('.macroforge-staging-') ||
            name.includes('.macroforge-old-')
    );
}

/** Recursively list files under `dir` (relative POSIX paths). */
function listFiles(dir) {
    const out = [];
    const walk = (current) => {
        for (const entry of readdirSync(current, { withFileTypes: true })) {
            const full = path.join(current, entry.name);
            if (entry.isDirectory()) walk(full);
            else out.push(path.relative(dir, full).split(path.sep).join('/'));
        }
    };
    if (existsSync(dir)) walk(dir);
    return out.sort();
}

function importDist(relPath) {
    const abs = path.join(libraryRoot, 'dist', relPath);
    return import(pathToFileURL(abs).href);
}

test('packaging: svelte-package alone ships empty type modules (bug documentation)', () => {
    ensureInstalled();
    const result = runLibraryTask('package:unexpanded');
    assert.equal(
        result.status,
        0,
        `package:unexpanded should succeed.\n${result.stdout}\n${result.stderr}`
    );

    const typeModule = path.join(
        libraryRoot,
        'dist-unexpanded/types/person-name.js'
    );
    assert.ok(existsSync(typeModule), 'unexpanded type module should exist');

    const code = readFileSync(typeModule, 'utf8');
    assert.ok(
        !code.includes('personNameDefaultValue'),
        'without the expand step the generated runtime is dropped (empty module)'
    );
    assert.ok(
        !code.includes('personNameSerialize'),
        'serialize runtime should be absent in the unexpanded module'
    );
});

test('packaging: supported build chain emits expanded runtime into dist', async () => {
    ensureInstalled();
    const result = runLibraryTask('build');
    assert.equal(
        result.status,
        0,
        `build should succeed.\n${result.stdout}\n${result.stderr}`
    );

    // 1. Plain .ts type module ships its full generated runtime.
    const personName = await importDist('types/person-name.js');
    assert.deepEqual(
        personName.personNameDefaultValue(),
        { firstName: '', lastName: '' },
        'default value factory should be present and correct'
    );

    const json = personName.personNameSerialize({
        firstName: 'Ada',
        lastName: 'Lovelace'
    });
    const roundTrip = personName.personNameDeserialize(json);
    assert.equal(
        roundTrip.success,
        true,
        'round-trip deserialize should succeed'
    );
    assert.deepEqual(roundTrip.value, {
        firstName: 'Ada',
        lastName: 'Lovelace'
    });

    const invalid = personName.personNameDeserialize({
        firstName: '',
        lastName: 'x'
    });
    assert.equal(
        invalid.success,
        false,
        'nonEmpty validation should reject empty'
    );
    assert.ok(
        invalid.errors.some((e) => e.field.includes('firstName')),
        `expected a firstName validation error, got: ${JSON.stringify(invalid.errors)}`
    );

    // 2. `.svelte.ts` type module also ships expanded (.svelte.ts -> .svelte.js).
    const order = await importDist('types/order.svelte.js');
    assert.equal(
        typeof order.orderDefaultValue,
        'function',
        '.svelte.ts type module should ship its generated runtime'
    );

    // 3. Contrast: the .svelte component was expanded by the preprocessor.
    const cardSource = readFileSync(
        path.join(libraryRoot, 'dist/PersonCard.svelte'),
        'utf8'
    );
    assert.ok(
        cardSource.includes('cardLabelsDefaultValue'),
        'component macros are expanded via the preprocessor inside svelte-package'
    );

    // 4. The generated .d.ts surface carries the runtime declarations.
    const dts = readFileSync(
        path.join(libraryRoot, 'dist/types/person-name.d.ts'),
        'utf8'
    );
    assert.ok(
        dts.includes('personNameDefaultValue'),
        'type declarations should include the generated symbols'
    );

    // 5. No `.expanded.*` debug artifacts leak into the sources or the package.
    for (const dir of ['dist', 'src/lib']) {
        const leaks = listFiles(path.join(libraryRoot, dir)).filter((f) =>
            f.includes('.expanded.')
        );
        assert.deepEqual(leaks, [], `no .expanded.* files should exist in ${dir}`);
    }

    // Sanity: dist is non-empty.
    assert.ok(
        statSync(path.join(libraryRoot, 'dist/types/person-name.js')).size > 0,
        'packaged type module should not be empty'
    );
});

test('packaging: the output directory is replaced, not merged into', () => {
    ensureInstalled();

    // A file from a previous package that the new one does not produce must
    // not survive. `@sveltejs/package` guarantees this by deleting the output
    // directory outright; the swap has to preserve that behavior while never
    // leaving the output half-built.
    const dist = path.join(libraryRoot, 'dist');
    mkdirSync(dist, { recursive: true });
    const stale = path.join(dist, 'stale-from-a-previous-build.js');
    writeFileSync(stale, 'export const gone = true;\n');

    const result = runLibraryTask('build');
    assert.equal(
        result.status,
        0,
        `build should succeed.\n${result.stdout}\n${result.stderr}`
    );

    assert.ok(
        !existsSync(stale),
        'a stale artifact from the previous output should not survive packaging'
    );
    assert.ok(
        existsSync(path.join(dist, 'types/person-name.js')),
        'the freshly packaged output should be in place'
    );
    assert.deepEqual(
        swapScratchDirs(),
        [],
        'the swap should leave no staging or backup directories behind'
    );
});

test('packaging: a failed package leaves the previous output intact', () => {
    ensureInstalled();

    // The packager is pointed at a scratch directory and only the finished
    // tree is renamed into place, so a build that never finishes must not be
    // able to destroy a working `dist`.
    const dist = path.join(libraryRoot, 'dist');
    mkdirSync(dist, { recursive: true });
    const sentinel = path.join(dist, 'previous-build.js');
    writeFileSync(sentinel, 'export const previous = true;\n');

    const result = runMacroforge([
        'svelte-package',
        '--input',
        'src/does-not-exist',
        '--output',
        'dist'
    ]);

    assert.notEqual(result.status, 0, 'packaging a missing input should fail');
    assert.ok(
        existsSync(sentinel),
        'a failed package must not delete the existing output'
    );
    assert.deepEqual(
        swapScratchDirs(),
        [],
        'a failed package should clean up its staging directory'
    );
});

// --- Incremental packaging --------------------------------------------------
//
// These drive the CLI directly rather than through `deno task build`, which
// deletes `dist` before every run and would make incremental behavior
// unobservable.

const distDir = path.join(libraryRoot, 'dist');
const tsconfigPath = path.join(libraryRoot, 'tsconfig.json');

/** Package the library in place, preserving whatever the last run left behind. */
function packageLibrary(extraArgs = []) {
    return runMacroforge([
        'svelte-package',
        '--input',
        'src/lib',
        '--output',
        'dist',
        '--tsconfig',
        tsconfigPath,
        ...extraArgs
    ]);
}

/** Package, asserting success, and return the combined output. */
function packageOk(extraArgs = []) {
    const result = packageLibrary(extraArgs);
    assert.equal(
        result.status,
        0,
        `packaging should succeed.\n${result.stdout}\n${result.stderr}`
    );
    return `${result.stdout}${result.stderr}`;
}

function wasSkipped(output) {
    return output.includes('is up to date');
}

/** dist as `{ relative path: contents }`, for byte-level comparison. */
function snapshotDist() {
    return Object.fromEntries(
        listFiles(distDir).map((rel) => [
            rel,
            readFileSync(path.join(distDir, rel), 'utf8')
        ])
    );
}

function distMtimes() {
    return Object.fromEntries(
        listFiles(distDir).map((rel) => [
            rel,
            statSync(path.join(distDir, rel)).mtimeMs
        ])
    );
}

/** Run `fn` with a source file rewritten, restoring it however `fn` ends. */
function withEditedSource(relPath, edit, fn) {
    const abs = path.join(libraryRoot, relPath);
    const original = readFileSync(abs, 'utf8');
    try {
        writeFileSync(abs, edit(original));
        return fn();
    } finally {
        writeFileSync(abs, original);
    }
}

/** Run `fn` with an extra source file present, removing it however `fn` ends. */
function withNewSource(relPath, contents, fn) {
    const abs = path.join(libraryRoot, relPath);
    try {
        mkdirSync(path.dirname(abs), { recursive: true });
        writeFileSync(abs, contents);
        return fn();
    } finally {
        rmSync(abs, { force: true });
    }
}

test('packaging: an unchanged project is not repackaged', () => {
    ensureInstalled();
    packageOk();

    const before = distMtimes();
    const output = packageOk();

    assert.ok(
        wasSkipped(output),
        `a second run with no changes should skip the packager.\n${output}`
    );
    assert.deepEqual(
        distMtimes(),
        before,
        'a skipped run must not rewrite the output'
    );
});

test('packaging: a formatting-only edit is not a change', () => {
    ensureInstalled();
    packageOk();

    // Trailing whitespace on every line, plus a run of blank lines at the end:
    // the two things normalization is defined to ignore.
    withEditedSource(
        'src/lib/types/person-name.ts',
        (source) => `${source.replace(/\n/g, '   \n')}\n\n\n`,
        () => {
            const output = packageOk();
            assert.ok(
                wasSkipped(output),
                `reformatting alone should not trigger a rebuild.\n${output}`
            );
        }
    );
});

test('packaging: a changed type re-expands every macro module', () => {
    ensureInstalled();
    packageOk();

    withEditedSource(
        'src/lib/types/person-name.ts',
        (source) => source.replace(/lastName/g, 'familyName'),
        () => {
            const output = packageOk();

            assert.ok(!wasSkipped(output), `a real edit must rebuild.\n${output}`);
            // A renamed field moves the project's type surface, and expansion
            // output depends on it globally — so both macro modules are
            // re-expanded, not just the edited one. The log has to say that,
            // or re-expanding a whole library after a one-field edit reads as
            // a bug.
            assert.match(
                output,
                /Re-expanded all 2 macro module\(s\) — the project's type surface changed/
            );
            assert.match(
                readFileSync(path.join(distDir, 'types/person-name.js'), 'utf8'),
                /familyName/,
                'the packaged runtime should reflect the renamed field'
            );
        }
    );
});

test('packaging: an edit that leaves the type surface alone re-expands nothing', () => {
    ensureInstalled();
    packageOk();

    // `index.ts` only re-exports, so changing it moves no type and carries no
    // macros: the packager still runs, but the expanded tree is reused whole.
    withEditedSource(
        'src/lib/index.ts',
        (source) => `${source}\nexport const VERSION = 1;\n`,
        () => {
            const output = packageOk();

            assert.ok(!wasSkipped(output), `a real edit must rebuild.\n${output}`);
            assert.match(
                output,
                /Reused all 2 expanded module\(s\) — none of the 1 changed file\(s\) carry macros/
            );
            assert.match(
                readFileSync(path.join(distDir, 'index.js'), 'utf8'),
                /VERSION/
            );
        }
    );
});

test('packaging: a deleted output directory is rebuilt', () => {
    ensureInstalled();
    packageOk();

    rmSync(distDir, { recursive: true, force: true });
    const output = packageOk();

    assert.ok(
        !wasSkipped(output),
        `a missing dist must be rebuilt even with unchanged sources.\n${output}`
    );
    assert.ok(existsSync(path.join(distDir, 'types/person-name.js')));
});

test('packaging: --full-rebuild repackages an unchanged project', () => {
    ensureInstalled();
    packageOk();

    const output = packageOk(['--full-rebuild']);

    assert.ok(
        !wasSkipped(output),
        `--full-rebuild must ignore the previous build.\n${output}`
    );
    assert.match(output, /Re-expanded all 2 macro module\(s\) — no previous build to reuse/);
});

test('packaging: an added source is packaged and its removal is propagated', () => {
    ensureInstalled();
    packageOk();

    withNewSource(
        'src/lib/types/temporary.ts',
        '/** @derive(Default) */\nexport interface Temporary {\n    id: string;\n}\n',
        () => {
            packageOk();
            assert.ok(
                existsSync(path.join(distDir, 'types/temporary.js')),
                'a new source should be packaged'
            );
            assert.match(
                readFileSync(path.join(distDir, 'types/temporary.js'), 'utf8'),
                /temporaryDefault/,
                'the new module should ship its generated runtime'
            );
        }
    );

    packageOk();
    assert.ok(
        !existsSync(path.join(distDir, 'types/temporary.js')),
        'removing a source should remove it from the package'
    );
    assert.ok(
        !existsSync(
            path.join(
                libraryRoot,
                '.macroforge/svelte-package/expanded/types/temporary.ts'
            )
        ),
        'removing a source should drop its expanded artifact'
    );
});

test('packaging: a missing expanded artifact is rebuilt, not silently skipped', () => {
    ensureInstalled();
    packageOk();

    // The artifact for an unchanged file is reused by path. If it disappears —
    // an interrupted write, a stray `rm` — nothing about the *source* looks
    // different, so without an explicit check the packager would read the raw
    // module and publish it with its generated runtime missing.
    rmSync(
        path.join(
            libraryRoot,
            '.macroforge/svelte-package/expanded/types/person-name.ts'
        ),
        { force: true }
    );

    withEditedSource(
        'src/lib/index.ts',
        (source) => `${source}\nexport const VERSION = 1;\n`,
        () => {
            packageOk();
            assert.match(
                readFileSync(path.join(distDir, 'types/person-name.js'), 'utf8'),
                /personNameDefaultValue/,
                'the packaged module must still ship its generated runtime'
            );
        }
    );
});

test('packaging: incremental output matches a full rebuild byte for byte', () => {
    ensureInstalled();

    // Arrive at the same sources by two different routes: a sequence of
    // incremental builds, and one build from nothing. Anything the incremental
    // path reuses that it should not would show up here.
    packageOk();
    withEditedSource(
        'src/lib/types/order.svelte.ts',
        (source) => source.replace('quantity', 'count'),
        () => packageOk()
    );
    packageOk();
    const incremental = snapshotDist();

    packageOk(['--full-rebuild']);

    assert.deepEqual(
        incremental,
        snapshotDist(),
        'incremental packaging must produce exactly what a full rebuild does'
    );
});

test('packaging: an unexpandable source fails the build and spares dist', () => {
    ensureInstalled();
    packageOk();

    const sentinel = snapshotDist();

    withNewSource(
        'src/lib/types/unparseable.ts',
        '/** @derive(Default) */\nexport interface Broken {\n    nested: { a: string\n}\n',
        () => {
            const result = packageLibrary();

            assert.notEqual(
                result.status,
                0,
                'a source that cannot be expanded must fail the build'
            );
            assert.match(
                `${result.stdout}${result.stderr}`,
                /types\/unparseable\.ts/,
                'the failure should name the file it could not expand'
            );
            // Silently packaging the unexpanded source is the defect this
            // command exists to prevent, so the previous package must survive
            // untouched rather than being replaced by a broken one.
            assert.deepEqual(
                snapshotDist(),
                sentinel,
                'a failed expansion must leave the previous package in place'
            );
            assert.deepEqual(
                swapScratchDirs(),
                [],
                'a failed expansion should leave no scratch directories'
            );
        }
    );
});

/**
 * Run `fn` with a macro package installed under the library's `node_modules`.
 *
 * `rebuild` rewrites its entry script, which is what the CLI's external-macro
 * hash tracks. The marker export is the only thing that makes a package count
 * as one, so the contents are otherwise irrelevant: nothing loads this, it just
 * has to move the hash the way a real `napi build` would.
 */
function withMacroPackage(fn) {
    const dir = path.join(libraryRoot, 'node_modules', 'macroforge-test-macros');
    const entry = path.join(dir, 'index.js');
    const rebuild = (marker) =>
        writeFileSync(entry, `exports.__macroforgeRun = () => {};\n// ${marker}\n`);
    try {
        mkdirSync(dir, { recursive: true });
        rebuild('first build');
        return fn(rebuild);
    } finally {
        rmSync(dir, { recursive: true, force: true });
    }
}

test('packaging: a rebuilt macro binary re-expands rather than replaying the tree', () => {
    ensureInstalled();

    // The expanded tree persists under `.macroforge/`, and nothing about a
    // rebuilt macro package touches a source file, so comparing sources finds
    // nothing changed and the run republishes the previous binary's output.
    // Whatever that output was is then recorded as this binary's work, which is
    // how an expansion that would now fail outright gets shipped from cache
    // with no error in sight.
    const artifact = path.join(
        libraryRoot,
        '.macroforge/svelte-package/expanded/types/person-name.ts'
    );

    withMacroPackage((rebuild) => {
        packageOk();

        // Stands in for an expansion the rebuilt macro produces differently.
        // Marking the cached artifact is the only way to tell a reused one from
        // a freshly produced one by looking at the package.
        writeFileSync(
            artifact,
            `${readFileSync(artifact, 'utf8')}\nexport const REPLAYED_FROM_CACHE = true;\n`
        );

        rebuild('second build');
        const output = packageOk();

        assert.match(
            output,
            /Re-expanded all 2 macro module\(s\) — the external macro binary changed/,
            `a rebuilt macro binary must re-expand every module.\n${output}`
        );
        assert.ok(
            !readFileSync(
                path.join(distDir, 'types/person-name.js'),
                'utf8'
            ).includes('REPLAYED_FROM_CACHE'),
            'the package must not ship an expansion the current macro binary never produced'
        );
    });
});
