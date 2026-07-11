#!/usr/bin/env node
/**
 * Packaging driver for @playground/library.
 *
 * Default mode (the supported build step): `macroforge svelte-package` runs
 * @sveltejs/package with macro expansion baked into its file reads, so the
 * published dist ships the generated derive runtime and correct .d.ts in a
 * single pass — no separate expand step, no staging tree.
 *
 * --unexpanded: run svelte-package directly over src/lib. This reproduces the
 * "library ships unexpanded type modules" bug and is kept as documentation
 * for the regression test in tooling/playground/tests/packaging.test.mjs.
 */

import { existsSync, rmSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const libraryRoot = path.resolve(__dirname, '..');
const repoRoot = globalThis.process.env.MACROFORGE_ROOT ||
    path.resolve(libraryRoot, '..', '..', '..');

function findMacroforgeCli() {
    const home = globalThis.process.env.HOME ||
        globalThis.process.env.USERPROFILE || '';
    const candidates = [
        globalThis.process.env.MACROFORGE_CLI,
        path.join(repoRoot, 'crates', 'macroforge_ts', 'target', 'release', 'macroforge'),
        path.join(repoRoot, 'crates', 'macroforge_ts', 'target', 'debug', 'macroforge'),
        path.join(repoRoot, 'crates', 'target', 'release', 'macroforge'),
        path.join(repoRoot, 'crates', 'target', 'debug', 'macroforge'),
        path.join(home, '.cargo', 'bin', 'macroforge')
    ].filter(Boolean);
    for (const candidate of candidates) {
        if (existsSync(candidate)) return candidate;
    }
    throw new Error(
        'macroforge CLI binary not found. Build it with `cargo build --release -p macroforge_ts` (from crates/) or `pixi run install:cli`.'
    );
}

function run(command, args, label) {
    const result = spawnSync(command, args, {
        cwd: libraryRoot,
        stdio: 'inherit'
    });
    if (result.error) throw result.error;
    if (result.status !== 0) {
        throw new Error(`${label} failed with exit code ${result.status}`);
    }
}

if (globalThis.process.argv.includes('--unexpanded')) {
    rmSync(path.join(libraryRoot, 'dist-unexpanded'), {
        recursive: true,
        force: true
    });
    run(
        globalThis.process.execPath,
        [
            path.join(libraryRoot, 'node_modules', '.bin', 'svelte-package'),
            '-i',
            'src/lib',
            '-o',
            'dist-unexpanded',
            '--tsconfig',
            path.join(libraryRoot, 'tsconfig.json')
        ],
        'svelte-package'
    );
} else {
    rmSync(path.join(libraryRoot, 'dist'), { recursive: true, force: true });
    run(
        findMacroforgeCli(),
        [
            'svelte-package',
            '--input',
            'src/lib',
            '--output',
            'dist',
            '--tsconfig',
            path.join(libraryRoot, 'tsconfig.json')
        ],
        'macroforge svelte-package'
    );
}
