/**
 * `macroforge tsc` over Decode output for object shapes, generic ones
 * included, which must type-check under the project's strict settings.
 */

import { assertEquals } from '@std/assert';
import { runCli, vanillaRoot, withProjectFiles } from './test-utils.mjs';

Deno.test('tsc: object and generic object decoders type-check', () => {
    withProjectFiles(
        vanillaRoot,
        {
            'tsconfig.decode-objects.json': JSON.stringify({
                extends: './tsconfig.json',
                include: ['src/validators/nested-decode-tests.ts']
            })
        },
        () => {
            const result = runCli(['tsc', '-p', './tsconfig.decode-objects.json'], {
                cwd: vanillaRoot
            });
            assertEquals(
                result.success,
                true,
                `type check failed:\n${result.stdout}\n${result.stderr}`
            );
        }
    );
});
