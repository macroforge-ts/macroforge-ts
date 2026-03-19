/**
 * CLI integration tests for the macroforge binary.
 *
 * Tests the CLI's ability to:
 * - Cache files with macro expansion via `macroforge cache`
 * - Expand single files via `macroforge expand` (for expand-specific features)
 * - Load and respect macroforge.config.ts (foreign types)
 */

import { assert, assertEquals } from 'https://deno.land/std@0.224.0/assert/mod.ts';
import { existsSync } from 'node:fs';
import fs from 'node:fs';
import path from 'node:path';
import { repoRoot, runCli } from './test-utils.mjs';

// Temporary directory for test files
const tmpDir = path.join(
    repoRoot,
    'tooling',
    'playground',
    'tests',
    '.tmp-cli'
);

// Cache directory within the temp project
const cacheDir = path.join(tmpDir, '.macroforge', 'cache');

// Setup helper — creates a minimal project root so `macroforge cache` works
function setupTmpDir() {
    if (existsSync(tmpDir)) {
        fs.rmSync(tmpDir, { recursive: true });
    }
    fs.mkdirSync(tmpDir, { recursive: true });
    fs.writeFileSync(
        path.join(tmpDir, 'package.json'),
        '{ "name": "cli-test" }'
    );
}

// Cleanup helper
function cleanupTmpDir() {
    if (existsSync(tmpDir)) {
        fs.rmSync(tmpDir, { recursive: true });
    }
}

// Helper to read a cached file
function readCacheFile(relPath) {
    const cachePath = path.join(cacheDir, relPath + '.cache');
    return fs.readFileSync(cachePath, 'utf8');
}

// ============================================================================
// Cache-Based Tests
// ============================================================================

Deno.test('CLI cache: caches a simple file with @derive(Debug)', () => {
    setupTmpDir();
    try {
        fs.writeFileSync(
            path.join(tmpDir, 'simple.ts'),
            `/** @derive(Debug) */
interface User {
  name: string;
  age: number;
}`
        );

        const result = runCli(['cache', tmpDir, '--builtin-only']);

        assertEquals(
            result.success,
            true,
            `CLI should succeed. stderr: ${result.stderr}`
        );

        const cachePath = path.join(cacheDir, 'simple.ts.cache');
        assert(existsSync(cachePath), 'Cache file should exist');

        const content = readCacheFile('simple.ts');
        assert(
            content.includes('toString'),
            'Should generate toString method'
        );
        assert(
            !content.includes('@derive'),
            'Should strip @derive decorator'
        );
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI cache: caches a file with multiple macros', () => {
    setupTmpDir();
    try {
        fs.writeFileSync(
            path.join(tmpDir, 'multi-macro.ts'),
            `/** @derive(Debug, Clone, Default) */
interface Config {
  host: string;
  port: number;
  enabled: boolean;
}`
        );

        const result = runCli(['cache', tmpDir, '--builtin-only']);

        assertEquals(
            result.success,
            true,
            `CLI should succeed. stderr: ${result.stderr}`
        );

        const content = readCacheFile('multi-macro.ts');

        assert(content.includes('toString'), 'Should have Debug (toString)');
        assert(content.includes('clone'), 'Should have Clone');
        assert(content.includes('DefaultValue'), 'Should have Default');
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI cache: scans directory and caches all files with macros', () => {
    setupTmpDir();
    try {
        fs.writeFileSync(
            path.join(tmpDir, 'user.ts'),
            `/** @derive(Debug) */
interface User { name: string; }`
        );

        fs.writeFileSync(
            path.join(tmpDir, 'config.ts'),
            `/** @derive(Clone) */
interface Config { value: number; }`
        );

        fs.writeFileSync(
            path.join(tmpDir, 'plain.ts'),
            `interface Plain { x: number; }`
        );

        // Create a subdirectory with more files
        const subDir = path.join(tmpDir, 'models');
        fs.mkdirSync(subDir, { recursive: true });
        fs.writeFileSync(
            path.join(subDir, 'entity.ts'),
            `/** @derive(Default) */
interface Entity { id: string; }`
        );

        const result = runCli(['cache', tmpDir, '--builtin-only']);

        assertEquals(
            result.success,
            true,
            `Cache should succeed. stderr: ${result.stderr}`
        );

        // Check that files with macros were cached
        assert(
            existsSync(path.join(cacheDir, 'user.ts.cache')),
            'user.ts should be cached'
        );
        assert(
            existsSync(path.join(cacheDir, 'config.ts.cache')),
            'config.ts should be cached'
        );
        assert(
            existsSync(path.join(cacheDir, 'models', 'entity.ts.cache')),
            'models/entity.ts should be cached'
        );

        // plain.ts has no macros, should not have a cache file
        assert(
            !existsSync(path.join(cacheDir, 'plain.ts.cache')),
            'plain.ts should NOT be cached (no macros)'
        );
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI cache: is idempotent on re-run', () => {
    setupTmpDir();
    try {
        fs.writeFileSync(
            path.join(tmpDir, 'user.ts'),
            `/** @derive(Debug) */
interface User { name: string; }`
        );

        // First cache
        runCli(['cache', tmpDir, '--builtin-only']);

        const firstContent = readCacheFile('user.ts');

        // Second cache — should produce identical output
        const result = runCli(['cache', tmpDir, '--builtin-only']);

        assertEquals(result.success, true);

        const secondContent = readCacheFile('user.ts');
        assertEquals(
            firstContent,
            secondContent,
            'Cache output should be identical on re-run'
        );
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI cache: handles .svelte.ts extension correctly', () => {
    setupTmpDir();
    try {
        fs.writeFileSync(
            path.join(tmpDir, 'component.svelte.ts'),
            `/** @derive(Debug) */
interface Props { value: string; }`
        );

        const result = runCli(['cache', tmpDir, '--builtin-only']);

        assertEquals(result.success, true);

        const cachePath = path.join(cacheDir, 'component.svelte.ts.cache');
        assert(
            existsSync(cachePath),
            'Should create cache file for .svelte.ts'
        );
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI cache: loads config and applies foreign types', () => {
    setupTmpDir();
    const configDir = path.join(tmpDir, 'config-test');
    try {
        fs.mkdirSync(configDir, { recursive: true });

        // Create a macroforge.config.ts with foreign type
        fs.writeFileSync(
            path.join(configDir, 'macroforge.config.ts'),
            `export default {
  foreignTypes: {
    "DateTime.DateTime": {
      from: ["effect"],
      serialize: (v: any) => v.toISOString(),
      deserialize: (raw: unknown) => new Date(raw as string),
      default: () => new Date()
    }
  }
}`
        );

        // Create a package.json to mark project root
        fs.writeFileSync(
            path.join(configDir, 'package.json'),
            '{ "name": "config-test" }'
        );

        // Create a file that uses the foreign type
        fs.writeFileSync(
            path.join(configDir, 'event.ts'),
            `import type { DateTime } from 'effect';

/** @derive(Default, Serialize) */
interface Event {
  name: string;
  startTime: DateTime.DateTime;
}`
        );

        const result = runCli(['cache', configDir, '--builtin-only']);

        assertEquals(
            result.success,
            true,
            `CLI should succeed. stderr: ${result.stderr}`
        );

        const configCacheDir = path.join(configDir, '.macroforge', 'cache');
        const cachePath = path.join(configCacheDir, 'event.ts.cache');
        assert(existsSync(cachePath), 'Cache file should exist');

        const content = fs.readFileSync(cachePath, 'utf8');

        // Check that foreign type handlers were used
        assert(
            content.includes('toISOString') || content.includes('new Date'),
            `Should use foreign type handlers from config. Got: ${content.substring(0, 500)}...`
        );
    } finally {
        cleanupTmpDir();
    }
});

// ============================================================================
// Serde Macro Tests (via cache)
// ============================================================================

Deno.test('CLI cache: caches Serialize macro correctly', () => {
    setupTmpDir();
    try {
        fs.writeFileSync(
            path.join(tmpDir, 'serialize.ts'),
            `/** @derive(Serialize) */
interface Message {
  id: string;
  content: string;
  timestamp: number;
}`
        );

        const result = runCli(['cache', tmpDir, '--builtin-only']);

        assertEquals(
            result.success,
            true,
            `CLI should succeed. stderr: ${result.stderr}`
        );

        const content = readCacheFile('serialize.ts');
        assert(content.includes('serialize'), 'Should have serialize function');
        assert(
            content.includes('SerializeContext'),
            'Should import SerializeContext'
        );
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI cache: caches Deserialize macro correctly', () => {
    setupTmpDir();
    try {
        fs.writeFileSync(
            path.join(tmpDir, 'deserialize.ts'),
            `/** @derive(Deserialize) */
interface Request {
  method: string;
  path: string;
}`
        );

        const result = runCli(['cache', tmpDir, '--builtin-only']);

        assertEquals(
            result.success,
            true,
            `CLI should succeed. stderr: ${result.stderr}`
        );

        const content = readCacheFile('deserialize.ts');
        assert(content.includes('deserialize'), 'Should have deserialize function');
        assert(
            content.includes('DeserializeContext'),
            'Should import DeserializeContext'
        );
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI cache: caches combined Serialize + Deserialize', () => {
    setupTmpDir();
    try {
        fs.writeFileSync(
            path.join(tmpDir, 'serde-combined.ts'),
            `/** @derive(Serialize, Deserialize) */
interface Data {
  value: string;
  count: number;
}`
        );

        const result = runCli(['cache', tmpDir, '--builtin-only']);

        assertEquals(result.success, true);

        const content = readCacheFile('serde-combined.ts');
        assert(content.includes('serialize'), 'Should have serialize');
        assert(content.includes('deserialize'), 'Should have deserialize');
    } finally {
        cleanupTmpDir();
    }
});

// ============================================================================
// Expand-Specific Tests (features with no cache equivalent)
// ============================================================================

Deno.test('CLI expand: exits with code 2 when no macros found', () => {
    setupTmpDir();
    try {
        const inputFile = path.join(tmpDir, 'no-macros.ts');
        fs.writeFileSync(
            inputFile,
            `interface Plain {
  value: string;
}`
        );

        const result = runCli(['expand', inputFile, '--builtin-only', '--quiet']);

        assertEquals(
            result.status,
            2,
            'Should exit with code 2 when no macros found'
        );
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI expand: prints to stdout with --print flag', () => {
    setupTmpDir();
    try {
        const inputFile = path.join(tmpDir, 'print-test.ts');
        fs.writeFileSync(
            inputFile,
            `/** @derive(Debug) */
interface Item { id: string; }`
        );

        const result = runCli(['expand', inputFile, '--builtin-only', '--print']);

        assertEquals(result.success, true);
        assert(
            result.stdout.includes('toString'),
            'Should print expanded code to stdout'
        );
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI expand: respects --out flag for custom output path', () => {
    setupTmpDir();
    try {
        const inputFile = path.join(tmpDir, 'custom-out.ts');
        const outputFile = path.join(tmpDir, 'output', 'custom.generated.ts');
        fs.writeFileSync(
            inputFile,
            `/** @derive(Debug) */
interface Custom { x: number; }`
        );

        const result = runCli([
            'expand',
            inputFile,
            '--builtin-only',
            '--out',
            outputFile
        ]);

        assertEquals(
            result.success,
            true,
            `CLI should succeed. stderr: ${result.stderr}`
        );
        assert(existsSync(outputFile), 'Custom output file should exist');
    } finally {
        cleanupTmpDir();
    }
});
