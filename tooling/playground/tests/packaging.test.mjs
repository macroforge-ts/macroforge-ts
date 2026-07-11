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
import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import path from 'node:path';
import { test } from 'node:test';
import { pathToFileURL } from 'node:url';
import { playgroundRoot } from './test-utils.mjs';

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

async function importDist(relPath) {
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
    assert.equal(roundTrip.success, true, 'round-trip deserialize should succeed');
    assert.deepEqual(roundTrip.value, {
        firstName: 'Ada',
        lastName: 'Lovelace'
    });

    const invalid = personName.personNameDeserialize({
        firstName: '',
        lastName: 'x'
    });
    assert.equal(invalid.success, false, 'nonEmpty validation should reject empty');
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
