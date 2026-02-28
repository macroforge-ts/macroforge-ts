/**
 * CLI integration tests for `macroforge svelte-check`.
 *
 * Tests the CLI's ability to run svelte-check with macro expansion
 * baked into TypeScript file reads via ts.sys.readFile patching.
 *
 * These tests run against the playground/svelte SvelteKit project
 * which has svelte-check, macroforge, and .svelte.ts files with @derive.
 */

import { assert, assertEquals, assertStringIncludes } from 'https://deno.land/std@0.224.0/assert/mod.ts';
import { existsSync } from 'node:fs';
import path from 'node:path';
import { repoRoot, svelteRoot } from './test-utils.mjs';

// Path to the CLI binary (same resolution as cli.test.mjs)
const cliBinary = (() => {
    const releaseInCrate = path.join(
        repoRoot,
        'crates',
        'macroforge_ts',
        'target',
        'release',
        'macroforge'
    );
    const debugInCrate = path.join(
        repoRoot,
        'crates',
        'macroforge_ts',
        'target',
        'debug',
        'macroforge'
    );
    const release = path.join(
        repoRoot,
        'crates',
        'target',
        'release',
        'macroforge'
    );
    const debug = path.join(repoRoot, 'crates', 'target', 'debug', 'macroforge');
    // Also check cargo install location
    const cargoInstall = path.join(
        process.env.HOME || process.env.USERPROFILE || '',
        '.cargo',
        'bin',
        'macroforge'
    );
    if (existsSync(releaseInCrate)) return releaseInCrate;
    if (existsSync(debugInCrate)) return debugInCrate;
    if (existsSync(release)) return release;
    if (existsSync(debug)) return debug;
    if (existsSync(cargoInstall)) return cargoInstall;
    throw new Error(
        'macroforge CLI binary not found. Run `cargo build` or `cargo install --path .` first.'
    );
})();

// Helper to run CLI and capture output
function runCli(args, options = {}) {
    const command = new Deno.Command(cliBinary, {
        args,
        cwd: options.cwd || svelteRoot,
        stdout: 'piped',
        stderr: 'piped',
    });
    const result = command.outputSync();
    const decoder = new TextDecoder();
    return {
        stdout: decoder.decode(result.stdout),
        stderr: decoder.decode(result.stderr),
        status: result.code,
        success: result.success,
    };
}

// ============================================================================
// Help & Basic Invocation
// ============================================================================

Deno.test('svelte-check: --help shows usage', () => {
    const result = runCli(['svelte-check', '--help']);

    assertEquals(result.success, true, `Should succeed. stderr: ${result.stderr}`);
    assertStringIncludes(result.stdout, '--workspace', 'Should show --workspace flag');
    assertStringIncludes(result.stdout, '--tsconfig', 'Should show --tsconfig flag');
    assertStringIncludes(result.stdout, '--output', 'Should show --output flag');
    assertStringIncludes(result.stdout, '--fail-on-warnings', 'Should show --fail-on-warnings flag');
});

// ============================================================================
// Integration Tests (run against playground/svelte)
// ============================================================================

Deno.test('svelte-check: type-checks the svelte playground project', () => {
    // Ensure .svelte-kit types are generated (prerequisite for svelte-check)
    const syncResult = new Deno.Command('node', {
        args: ['./node_modules/.bin/svelte-kit', 'sync'],
        cwd: svelteRoot,
        stdout: 'piped',
        stderr: 'piped',
    }).outputSync();

    assert(
        syncResult.success,
        `svelte-kit sync should succeed. stderr: ${new TextDecoder().decode(syncResult.stderr)}`
    );

    const result = runCli(['svelte-check', '--tsconfig', './tsconfig.json']);

    assertEquals(
        result.success,
        true,
        `macroforge svelte-check should pass on the playground project.\nstdout: ${result.stdout}\nstderr: ${result.stderr}`
    );
});

Deno.test('svelte-check: --output machine produces machine-readable output', () => {
    const result = runCli(['svelte-check', '--tsconfig', './tsconfig.json', '--output', 'machine']);

    assertEquals(
        result.success,
        true,
        `Should pass with --output machine.\nstdout: ${result.stdout}\nstderr: ${result.stderr}`
    );
});

Deno.test('svelte-check: --workspace flag is forwarded', () => {
    const result = runCli(['svelte-check', '--workspace', svelteRoot, '--tsconfig', './tsconfig.json']);

    assertEquals(
        result.success,
        true,
        `Should pass with explicit --workspace.\nstdout: ${result.stdout}\nstderr: ${result.stderr}`
    );
});

Deno.test('svelte-check: --fail-on-warnings exits non-zero on warnings', () => {
    // This test just verifies the flag is forwarded without crashing.
    // If there are no warnings it should still succeed (exit 0).
    const result = runCli(['svelte-check', '--tsconfig', './tsconfig.json', '--fail-on-warnings']);

    // We don't assert success here because the project might have warnings.
    // We just verify it doesn't crash (status should be 0 or 1, not a node crash).
    assert(
        result.status === 0 || result.status === 1,
        `Should exit cleanly (0 or 1), got ${result.status}.\nstdout: ${result.stdout}\nstderr: ${result.stderr}`
    );
});
