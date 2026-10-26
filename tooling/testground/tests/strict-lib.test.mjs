/**
 * The testground apps type-check with a stricter standard library standing in
 * for TypeScript's own: one with no `any` in a value position, so expanded
 * code that leans on one (`JSON.parse`, `Object.getPrototypeOf`, a
 * `JSON.stringify` replacer) fails here and nowhere else.
 *
 * Each app's `tsconfig.strict-lib.json` turns on `libReplacement`, and its
 * `package.json` installs every lib under the `@typescript/lib-*` alias
 * TypeScript resolves a replacement by.
 */

import { assert, assertEquals, assertStringIncludes } from '@std/assert';
import { runCli, svelteRoot, vanillaRoot, withProjectFiles } from './test-utils.mjs';

const decoder = new TextDecoder();

/** The files of `root`'s strict-lib project, as its own TypeScript lists them. */
function projectFiles(root) {
    const listed = new Deno.Command('./node_modules/.bin/tsc', {
        args: ['-p', './tsconfig.strict-lib.json', '--listFilesOnly'],
        cwd: root,
        stdout: 'piped',
        stderr: 'piped'
    }).outputSync();
    assert(
        listed.success,
        `tsc --listFilesOnly failed:\n${decoder.decode(listed.stdout)}\n${
            decoder.decode(listed.stderr)
        }`
    );
    return decoder.decode(listed.stdout).split('\n').filter((line) => line !== '');
}

for (const [name, root] of [['vanilla', vanillaRoot], ['svelte', svelteRoot]]) {
    // A lib with no replacement installed loads TypeScript's own copy, with
    // its `any`s and without a warning, and the project would pass unchecked.
    Deno.test(`strict lib: every lib of the ${name} project is a replacement`, () => {
        const files = projectFiles(root);
        const bundled = files.filter((file) => /\/typescript\/lib\/lib\.[^/]*\.d\.ts$/.test(file));
        assertEquals(bundled, [], 'TypeScript loaded its own copy of these libs');
        // TypeScript's own copy is `lib.es5.d.ts`.
        assert(
            files.some((file) => file.endsWith('/es5.d.ts')),
            `no replacement lib among the project's files:\n${files.join('\n')}`
        );
    });
}

Deno.test('strict lib: the vanilla testground type-checks', () => {
    const result = runCli(['tsc', '-p', './tsconfig.strict-lib.json'], { cwd: vanillaRoot });
    assertEquals(
        result.success,
        true,
        `macroforge tsc failed on the stricter lib:\n${result.stdout}\n${result.stderr}`
    );
});

Deno.test('strict lib: the svelte testground type-checks', () => {
    const sync = new Deno.Command('./node_modules/.bin/svelte-kit', {
        args: ['sync'],
        cwd: svelteRoot,
        stdout: 'piped',
        stderr: 'piped'
    }).outputSync();
    assert(sync.success, `svelte-kit sync failed. stderr: ${decoder.decode(sync.stderr)}`);

    const result = runCli(['svelte-check', '--tsconfig', './tsconfig.strict-lib.json'], {
        cwd: svelteRoot
    });
    assertEquals(
        result.success,
        true,
        `macroforge svelte-check failed on the stricter lib:\nstdout: ${result.stdout}\nstderr: ${result.stderr}`
    );
});

// The check above means nothing if the wrapper's own program ignores the
// replacement, so a line only the stock lib accepts has to fail through it.
Deno.test('strict lib: macroforge tsc checks against the replacement lib', () => {
    const probe = [
        '/** @derive(Encode) */',
        'export interface Probe { id: string }',
        "export const parsed: Probe = JSON.parse('{}');",
        ''
    ].join('\n');
    const include = ['src/strict-lib-probe.ts'];
    withProjectFiles(
        vanillaRoot,
        {
            'src/strict-lib-probe.ts': probe,
            'tsconfig.stock-probe.json': JSON.stringify({ extends: './tsconfig.json', include }),
            'tsconfig.strict-lib-probe.json': JSON.stringify({
                extends: './tsconfig.strict-lib.json',
                include
            })
        },
        () => {
            const stock = runCli(['tsc', '-p', './tsconfig.stock-probe.json'], {
                cwd: vanillaRoot
            });
            assertEquals(
                stock.success,
                true,
                `the probe should pass on TypeScript's own lib:\n${stock.stdout}\n${stock.stderr}`
            );

            const replaced = runCli(['tsc', '-p', './tsconfig.strict-lib-probe.json'], {
                cwd: vanillaRoot
            });
            assertEquals(replaced.success, false, 'JSON.parse should no longer return any');
            assertStringIncludes(
                replaced.stdout + replaced.stderr,
                "strict-lib-probe.ts(3,14): error TS2322: Type 'JSONValue' is not assignable to type 'Probe'"
            );
        }
    );
});
