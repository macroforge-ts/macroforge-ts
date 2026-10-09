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
    'testground',
    'tests',
    '.tmp-cli'
);

// Cache directory within the temp project
const cacheDir = path.join(tmpDir, '.macroforge', 'cache');

// Setup helper: creates a minimal project root so `macroforge cache` works
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

        // Second cache: should produce identical output
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

/** Writes a project whose config declares one foreign type. */
function writeForeignTypeProject(configDir) {
    fs.mkdirSync(configDir, { recursive: true });
    fs.writeFileSync(
        path.join(configDir, 'macroforge.config.ts'),
        `export default {
  foreignTypes: {
    "DateTime.DateTime": {
      from: ["effect"],
      encode: (v: Date) => v.toISOString(),
      decode: (raw: unknown) => new Date(String(raw)),
      default: () => new Date()
    }
  }
}`
    );
    fs.writeFileSync(
        path.join(configDir, 'package.json'),
        '{ "name": "config-test" }'
    );
    fs.writeFileSync(
        path.join(configDir, 'event.ts'),
        `import type { DateTime } from 'effect';

/** @derive(Default, Encode) */
interface Event {
  name: string;
  startTime: DateTime.DateTime;
}`
    );
}

Deno.test('CLI cache: loads config and applies foreign types', () => {
    setupTmpDir();
    const configDir = path.join(tmpDir, 'config-test');
    try {
        writeForeignTypeProject(configDir);

        const init = runCli(['init', configDir]);
        assertEquals(init.success, true, `init should succeed. stderr: ${init.stderr}`);
        const manifest = JSON.parse(
            fs.readFileSync(path.join(configDir, 'package.json'), 'utf8')
        );
        assertEquals(manifest.imports['#macroforge/config'], {
            types: './.macroforge/config/handlers.ts',
            default: './.macroforge/config/handlers.js'
        });

        const result = runCli(['cache', configDir]);
        assertEquals(
            result.success,
            true,
            `CLI should succeed. stderr: ${result.stderr}`
        );

        const expanded = path.join(configDir, '.macroforge', 'config');
        const handlersTs = fs.readFileSync(path.join(expanded, 'handlers.ts'), 'utf8');
        const handlersJs = fs.readFileSync(path.join(expanded, 'handlers.js'), 'utf8');
        for (const name of ['Encode', 'Decode', 'Default']) {
            const exported = `export const __foreign__dateTimeDateTime${name} =`;
            assert(
                handlersTs.includes(exported),
                `handlers.ts should export ${name}. Got: ${handlersTs}`
            );
            assert(
                handlersJs.includes(exported),
                `handlers.js should export ${name}. Got: ${handlersJs}`
            );
        }
        assert(
            !handlersJs.includes(': Date') && !handlersJs.includes(': unknown'),
            `handlers.js should have its types stripped. Got: ${handlersJs}`
        );

        const cachePath = path.join(configDir, '.macroforge', 'cache', 'event.ts.cache');
        assert(existsSync(cachePath), 'Cache file should exist');
        const content = fs.readFileSync(cachePath, 'utf8');
        assert(
            /from ["']#macroforge\/config["']/.test(content),
            `Should import the handlers from #macroforge/config. Got: ${content}`
        );
        assert(
            content.includes('__foreign__dateTimeDateTimeEncode(') &&
                content.includes('__foreign__dateTimeDateTimeDefault('),
            `Should call the imported handlers. Got: ${content}`
        );
        assert(
            !content.includes('toISOString'),
            `Should not copy the handler bodies. Got: ${content}`
        );
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI cache: a foreign-type config without the manifest entry fails', () => {
    setupTmpDir();
    const configDir = path.join(tmpDir, 'config-test');
    try {
        writeForeignTypeProject(configDir);

        const result = runCli(['cache', configDir]);
        assertEquals(result.success, false, 'cache should refuse the project');
        assert(
            result.stderr.includes('macroforge init'),
            `The error should say how to declare the entry. stderr: ${result.stderr}`
        );
        assert(
            !existsSync(path.join(configDir, '.macroforge', 'config')),
            'Nothing should be expanded for a project that cannot import it'
        );
    } finally {
        cleanupTmpDir();
    }
});

// ============================================================================
// Sync
// ============================================================================

Deno.test('CLI sync: writes the registries and expanded config without expanding', () => {
    setupTmpDir();
    const configDir = path.join(tmpDir, 'config-test');
    try {
        writeForeignTypeProject(configDir);
        const init = runCli(['init', configDir]);
        assertEquals(init.success, true, `init should succeed. stderr: ${init.stderr}`);

        const result = runCli(['sync', configDir]);
        assertEquals(result.success, true, `sync should succeed. stderr: ${result.stderr}`);

        const dir = path.join(configDir, '.macroforge');
        assert(existsSync(path.join(dir, 'type-registry.json')), 'the type registry is written');
        assert(
            existsSync(path.join(dir, 'declarative-registry.json')),
            'the declarative registry is written'
        );
        const handlers = fs.readFileSync(path.join(dir, 'config', 'handlers.js'), 'utf8');
        assert(
            handlers.includes('export const __foreign__dateTimeDateTimeEncode ='),
            `the expanded config is written. Got: ${handlers}`
        );
        assert(!existsSync(path.join(dir, 'cache')), 'no file is expanded');
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI sync: a config without macros still gets its expansion', () => {
    setupTmpDir();
    const configDir = path.join(tmpDir, 'config-test');
    try {
        writeForeignTypeProject(configDir);
        fs.rmSync(path.join(configDir, 'event.ts'));
        assertEquals(runCli(['init', configDir]).success, true);

        const result = runCli(['sync', configDir]);
        assertEquals(result.success, true, `sync should succeed. stderr: ${result.stderr}`);
        assert(
            existsSync(path.join(configDir, '.macroforge', 'config', 'handlers.ts')),
            'the expanded config is written'
        );
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI sync: a foreign-type config without the manifest entry fails', () => {
    setupTmpDir();
    const configDir = path.join(tmpDir, 'config-test');
    try {
        writeForeignTypeProject(configDir);
        const result = runCli(['sync', configDir]);
        assertEquals(result.success, false, 'sync should refuse the project');
        assert(
            result.stderr.includes('macroforge init'),
            `The error should say how to declare the entry. stderr: ${result.stderr}`
        );
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI sync: a project with neither macros nor a config is left alone', () => {
    setupTmpDir();
    try {
        fs.writeFileSync(path.join(tmpDir, 'plain.ts'), 'export const a = 1;\n');
        const result = runCli(['sync', tmpDir]);
        assertEquals(result.success, true, `sync should succeed. stderr: ${result.stderr}`);
        assert(!existsSync(path.join(tmpDir, '.macroforge')), 'no .macroforge/ is created');
    } finally {
        cleanupTmpDir();
    }
});

// ============================================================================
// Endec Macro Tests (via cache)
// ============================================================================

Deno.test('CLI cache: caches Encode macro correctly', () => {
    setupTmpDir();
    try {
        fs.writeFileSync(
            path.join(tmpDir, 'encode.ts'),
            `/** @derive(Encode) */
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

        const content = readCacheFile('encode.ts');
        assert(content.includes('encode'), 'Should have encode function');
        assert(
            content.includes('EncodeContext'),
            'Should import EncodeContext'
        );
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI cache: caches Decode macro correctly', () => {
    setupTmpDir();
    try {
        fs.writeFileSync(
            path.join(tmpDir, 'decode.ts'),
            `/** @derive(Decode) */
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

        const content = readCacheFile('decode.ts');
        assert(content.includes('decode'), 'Should have decode function');
        assert(
            content.includes('DecodeContext'),
            'Should import DecodeContext'
        );
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI cache: caches combined Encode + Decode', () => {
    setupTmpDir();
    try {
        fs.writeFileSync(
            path.join(tmpDir, 'endec-combined.ts'),
            `/** @derive(Encode, Decode) */
interface Data {
  value: string;
  count: number;
}`
        );

        const result = runCli(['cache', tmpDir]);

        assertEquals(result.success, true);

        const content = readCacheFile('endec-combined.ts');
        assert(content.includes('encode'), 'Should have encode');
        assert(content.includes('decode'), 'Should have decode');
    } finally {
        cleanupTmpDir();
    }
});

// ============================================================================
// Expand-Specific Tests (features with no cache equivalent)
// ============================================================================

Deno.test("CLI expand: a macro package's debug lines reach the project's debug log", () => {
    const vanillaRoot = path.join(repoRoot, 'tooling', 'testground', 'vanilla');
    const debugLog = path.join(vanillaRoot, '.macroforge', 'debug.log');
    fs.rmSync(debugLog, { force: true });

    const result = runCli(['expand', 'src/user.ts'], { cwd: vanillaRoot });

    assertEquals(result.success, true, result.stderr);
    assert(existsSync(debugLog), 'the expansion should write the debug log');
    const logged = fs.readFileSync(debugLog, 'utf8');
    assert(logged.includes('[JSON] toJSON for User'), `got:\n${logged}`);
});

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

Deno.test('CLI expand: without --out prints and leaves the source tree alone', () => {
    setupTmpDir();
    try {
        const inputFile = path.join(tmpDir, 'stdout-test.ts');
        fs.writeFileSync(
            inputFile,
            `/** @derive(Debug) */
interface Item { id: string; }`
        );

        const result = runCli(['expand', inputFile]);

        assertEquals(result.success, true, `CLI should succeed. stderr: ${result.stderr}`);
        assert(result.stdout.includes('toString'), 'Should print expanded code to stdout');
        assert(
            !existsSync(path.join(tmpDir, 'stdout-test.expanded.ts')),
            'Should not write an .expanded.ts sibling'
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

        // Encoded writes mean the manifest is never a blend of two writers.
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
