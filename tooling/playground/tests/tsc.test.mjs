/**
 * CLI integration tests for `macroforge tsc`.
 *
 * Tests the CLI's ability to run tsc with macro expansion
 * baked into TypeScript's getSourceFile via compiler host patching.
 *
 * These tests run against the playground/vanilla project
 * which has TypeScript, macroforge, and .ts files with @derive.
 */

import {
    assert,
    assertEquals,
    assertStringIncludes
} from 'https://deno.land/std@0.224.0/assert/mod.ts';
import { runCli, vanillaRoot } from './test-utils.mjs';

// ============================================================================
// Help & Basic Invocation
// ============================================================================

Deno.test('tsc: --help shows usage', () => {
    const result = runCli(['tsc', '--help']);

    assertEquals(
        result.success,
        true,
        `Should succeed. stderr: ${result.stderr}`
    );
    assertStringIncludes(
        result.stdout,
        '--project',
        'Should show --project flag'
    );
});

// ============================================================================
// Integration Tests (run against playground/vanilla)
// ============================================================================

Deno.test('tsc: type-checks the vanilla playground project', () => {
    const result = runCli(['tsc', '-p', './tsconfig.json'], {
        cwd: vanillaRoot
    });

    // The vanilla playground has .svelte.ts files that produce TS2564 errors
    // with plain tsc (they need Svelte's preprocessor). Verify that any errors
    // are ONLY from .svelte.ts files — all other files should type-check cleanly.
    if (!result.success) {
        const errorLines = result.stderr
            .split('\n')
            .filter((line) => line.includes(': error TS'));
        const nonSvelteErrors = errorLines.filter(
            (line) => !line.includes('.svelte.ts')
        );
        assertEquals(
            nonSvelteErrors.length,
            0,
            `macroforge tsc should only have errors from .svelte.ts files.\nNon-svelte errors:\n${
                nonSvelteErrors.join('\n')
            }`
        );
    }
});

Deno.test('tsc: type-checks with default tsconfig', () => {
    const result = runCli(['tsc'], { cwd: vanillaRoot });

    // Same as above — allow .svelte.ts errors only
    if (!result.success) {
        const errorLines = result.stderr
            .split('\n')
            .filter((line) => line.includes(': error TS'));
        const nonSvelteErrors = errorLines.filter(
            (line) => !line.includes('.svelte.ts')
        );
        assertEquals(
            nonSvelteErrors.length,
            0,
            `macroforge tsc should only have errors from .svelte.ts files.\nNon-svelte errors:\n${
                nonSvelteErrors.join('\n')
            }`
        );
    }
});
