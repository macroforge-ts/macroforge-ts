/**
 * CLI integration tests for `macroforge tsc`.
 *
 * Tests the CLI's ability to run tsc with macro expansion
 * baked into TypeScript's getSourceFile via compiler host patching.
 *
 * These tests run against the testground/vanilla project
 * which has TypeScript, macroforge, and .ts files with @derive.
 */

import { assertEquals, assertStringIncludes } from '@std/assert';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
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
// Integration Tests (run against testground/vanilla)
// ============================================================================

Deno.test('tsc: type-checks the vanilla testground project', () => {
    const result = runCli(['tsc', '-p', './tsconfig.json'], {
        cwd: vanillaRoot
    });

    // The vanilla testground has .svelte.ts files that produce TS2564 errors
    // with plain tsc (they need Svelte's preprocessor). Verify that any errors
    // are ONLY from .svelte.ts files: all other files should type-check cleanly.
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

    // Same as above: allow .svelte.ts errors only
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

// ============================================================================
// TypeScript without a compiler API
// ============================================================================

Deno.test('tsc: a typescript package without the compiler API is rejected by name', () => {
    const project = mkdtempSync(join(tmpdir(), 'macroforge-tsc-native-'));
    try {
        writeFileSync(join(project, 'package.json'), '{ "name": "native-typescript" }');
        writeFileSync(join(project, 'tsconfig.json'), '{ "include": ["src"] }');
        // The main module of typescript 7: a version and nothing to compile with.
        const typescript = join(project, 'node_modules', 'typescript');
        mkdirSync(typescript, { recursive: true });
        writeFileSync(
            join(typescript, 'package.json'),
            '{ "name": "typescript", "version": "7.0.2", "main": "version.cjs" }'
        );
        writeFileSync(
            join(typescript, 'version.cjs'),
            'exports.version = "7.0.2";\nexports.versionMajorMinor = "7.0";\n'
        );

        const result = runCli(['tsc'], { cwd: project });

        assertEquals(result.status, 1, `should fail. stderr: ${result.stderr}`);
        assertStringIncludes(result.stderr, 'typescript 7.0.2 in this project has no compiler API');
        assertStringIncludes(result.stderr, 'needs typescript 5 or 6');
    } finally {
        rmSync(project, { recursive: true, force: true });
    }
});
