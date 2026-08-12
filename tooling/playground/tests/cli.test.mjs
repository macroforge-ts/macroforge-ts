/**
 * CLI integration tests for the macroforge binary.
 *
 * Tests the CLI's ability to:
 * - Cache files with macro expansion via `macroforge cache`
 * - Expand single files via `macroforge expand` (for expand-specific features)
 * - Load and respect macroforge.config.ts (foreign types)
 */

import { assert, assertEquals } from '@std/assert';
import { existsSync } from 'node:fs';
import fs from 'node:fs';
import path from 'node:path';
import { cliBinary, repoRoot, runCli } from './test-utils.mjs';

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

        const result = runCli(['cache', tmpDir]);

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

        const result = runCli(['cache', tmpDir]);

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

        const result = runCli(['cache', tmpDir]);

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
        runCli(['cache', tmpDir]);

        const firstContent = readCacheFile('user.ts');

        // Second cache — should produce identical output
        const result = runCli(['cache', tmpDir]);

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

        const result = runCli(['cache', tmpDir]);

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

        const result = runCli(['cache', configDir]);

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

        const result = runCli(['cache', tmpDir]);

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

        const result = runCli(['cache', tmpDir]);

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

        const result = runCli(['cache', tmpDir]);

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

        const result = runCli(['expand', inputFile, '--quiet']);

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

        const result = runCli(['expand', inputFile, '--print']);

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

// ============================================================================
// Project lock
// ============================================================================

/** Spawns the CLI without waiting, returning the child process. */
function spawnCli(args) {
    return new Deno.Command(cliBinary, {
        args,
        cwd: globalThis.process.cwd(),
        stdout: 'piped',
        stderr: 'piped'
    }).spawn();
}

/** Decodes a child's stdout and stderr once it exits. */
async function collectCli(child) {
    const { code, success, stdout, stderr } = await child.output();
    const decoder = new TextDecoder();
    return {
        status: code,
        success,
        stdout: decoder.decode(stdout),
        stderr: decoder.decode(stderr)
    };
}

Deno.test('CLI lock: waits for a lock held by another process', async () => {
    setupTmpDir();
    fs.mkdirSync(path.join(tmpDir, '.macroforge'), { recursive: true });
    const lockPath = path.join(tmpDir, '.macroforge', '.lock');

    // Stand in for a second macroforge process by taking the same advisory
    // lock the CLI takes. This makes contention deterministic instead of
    // depending on two real runs overlapping.
    const holder = Deno.openSync(lockPath, {
        create: true,
        read: true,
        write: true
    });
    holder.lockSync(true);

    let released = false;
    const release = () => {
        if (released) return;
        released = true;
        try {
            holder.unlockSync();
        } finally {
            holder.close();
        }
    };

    try {
        const child = spawnCli(['cache', tmpDir]);
        const pending = collectCli(child);

        // Give the CLI long enough that an unblocked run would have finished
        // this tiny project several times over. It must still be waiting.
        const settled = await Promise.race([
            pending.then(() => 'exited'),
            new Promise((resolve) => setTimeout(() => resolve('waiting'), 3000))
        ]);
        assertEquals(
            settled,
            'waiting',
            'the CLI must block while another process holds the lock'
        );
        assert(
            !existsSync(path.join(cacheDir, 'manifest.json')),
            'the blocked process must not write the cache before acquiring'
        );

        release();

        const result = await pending;
        assertEquals(
            result.success,
            true,
            `CLI should complete once the lock frees. stderr: ${result.stderr}`
        );
        assert(
            result.stderr.includes('waiting for the project lock'),
            `CLI should report that it was blocked. stderr: ${result.stderr}`
        );
        assert(
            existsSync(path.join(cacheDir, 'manifest.json')),
            'the cache should be written after the lock is acquired'
        );
    } finally {
        release();
        cleanupTmpDir();
    }
});

Deno.test('CLI lock: concurrent runs all succeed with an intact manifest', async () => {
    setupTmpDir();
    try {
        for (let i = 0; i < 8; i++) {
            fs.writeFileSync(
                path.join(tmpDir, `concurrent-${i}.ts`),
                `/** @derive(Debug) */\ninterface Concurrent${i} { id: string; }\n`
            );
        }

        const results = await Promise.all(
            [0, 1, 2].map((_) => collectCli(spawnCli(['cache', tmpDir])))
        );

        for (const result of results) {
            assertEquals(
                result.success,
                true,
                `every concurrent run should succeed. stderr: ${result.stderr}`
            );
        }

        // Serialized writes mean the manifest is never a blend of two writers.
        const manifest = JSON.parse(
            fs.readFileSync(path.join(cacheDir, 'manifest.json'), 'utf8')
        );
        for (let i = 0; i < 8; i++) {
            assert(
                manifest.entries[`concurrent-${i}.ts`],
                `manifest should list concurrent-${i}.ts`
            );
        }

        // Same for the registry the Vite plugin reads.
        JSON.parse(
            fs.readFileSync(
                path.join(tmpDir, '.macroforge', 'type-registry.json'),
                'utf8'
            )
        );
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI lock: leaves no temp files in .macroforge', () => {
    setupTmpDir();
    try {
        fs.writeFileSync(
            path.join(tmpDir, 'leftovers.ts'),
            `/** @derive(Debug) */\ninterface Leftovers { id: string; }\n`
        );

        assertEquals(runCli(['cache', tmpDir]).success, true);

        const strays = [];
        const walk = (dir) => {
            for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
                const full = path.join(dir, entry.name);
                if (entry.isDirectory()) walk(full);
                else if (entry.name.endsWith('.tmp')) strays.push(full);
            }
        };
        walk(path.join(tmpDir, '.macroforge'));

        assertEquals(
            strays,
            [],
            'atomic writes should not leave temp files behind'
        );
    } finally {
        cleanupTmpDir();
    }
});
