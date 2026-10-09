/**
 * `macroforge tsc` over newtype sources: declaration output for exported
 * brands, and the positions of type and macro errors in a file the brand
 * declarations are inserted into.
 */

import { assertEquals, assertMatch, assertStringIncludes } from '@std/assert';
import { runCli, vanillaRoot, withProjectFiles } from './test-utils.mjs';

function tsconfig(include) {
    return JSON.stringify({
        extends: './tsconfig.json',
        compilerOptions: { declaration: true, emitDeclarationOnly: true, noEmit: false },
        include
    });
}

Deno.test('tsc: exported newtypes produce a valid declaration surface', () => {
    withProjectFiles(
        vanillaRoot,
        {
            'tsconfig.newtype-declarations.json': tsconfig([
                'src/validators/newtype-validator-tests.ts',
                'src/validators/newtype-consumer.ts',
                'src/validators/field-type-tests.ts'
            ])
        },
        () => {
            const result = runCli(['tsc', '-p', './tsconfig.newtype-declarations.json'], {
                cwd: vanillaRoot
            });
            assertEquals(
                result.success,
                true,
                `declaration check failed:\n${result.stdout}\n${result.stderr}`
            );
        }
    );
});

Deno.test('tsc: errors after a newtype report their source line', () => {
    const probe = [
        '/** @derive(Encode, Decode) */',
        'export type Probe = $Newtype<number>;',
        "export const label = 'café 🚀';",
        'export const wrong: string = 42;',
        ''
    ].join('\n');
    withProjectFiles(
        vanillaRoot,
        {
            'src/newtype-position-probe.ts': probe,
            'tsconfig.newtype-position.json': JSON.stringify({
                extends: './tsconfig.json',
                include: ['src/newtype-position-probe.ts']
            })
        },
        () => {
            const result = runCli(['tsc', '-p', './tsconfig.newtype-position.json'], {
                cwd: vanillaRoot
            });
            assertEquals(result.success, false, 'the probe has a deliberate type error');
            assertStringIncludes(
                result.stdout + result.stderr,
                'newtype-position-probe.ts(4,14): error TS2322'
            );
        }
    );
});

Deno.test('tsc: macro errors after a newtype report their source line', () => {
    const probe = [
        "export const label = 'café 🚀';",
        'export type Id = $Newtype<string>;',
        '/** @derive(Decode, Default) */',
        '/** @endec(positive) */',
        'export type Meters = $Newtype<number>;',
        ''
    ].join('\n');
    withProjectFiles(
        vanillaRoot,
        {
            'src/newtype-macro-error-probe.ts': probe,
            'tsconfig.newtype-macro-error.json': JSON.stringify({
                extends: './tsconfig.json',
                include: ['src/newtype-macro-error-probe.ts']
            })
        },
        () => {
            const result = runCli(['tsc', '-p', './tsconfig.newtype-macro-error.json'], {
                cwd: vanillaRoot
            });
            assertEquals(result.success, false, 'Default needs @default on a validated alias');
            assertMatch(
                result.stderr,
                /newtype-macro-error-probe\.ts:3:\d+: .*requires @default/
            );
        }
    );
});
