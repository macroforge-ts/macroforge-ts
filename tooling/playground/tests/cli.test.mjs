/**
 * CLI integration tests for the macroforge binary.
 *
 * Tests the CLI's ability to:
 * - Expand single files
 * - Scan directories
 * - Load and respect macroforge.config.ts (foreign types)
 * - Work in both default and --builtin-only modes
 */

import { assert, assertEquals, assertMatch } from 'https://deno.land/std@0.224.0/assert/mod.ts';
import { existsSync } from 'node:fs';
import fs from 'node:fs';
import path from 'node:path';
import { repoRoot } from './test-utils.mjs';

// Path to the CLI binary
const cliBinary = (() => {
    // Try the macroforge_ts crate target directory first (where cargo builds by default)
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
    // Also try workspace-level target directory
    const release = path.join(
        repoRoot,
        'crates',
        'target',
        'release',
        'macroforge'
    );
    const debug = path.join(repoRoot, 'crates', 'target', 'debug', 'macroforge');
    if (existsSync(releaseInCrate)) return releaseInCrate;
    if (existsSync(debugInCrate)) return debugInCrate;
    if (existsSync(release)) return release;
    if (existsSync(debug)) return debug;
    throw new Error(
        'macroforge CLI binary not found. Run `cargo build --release` first.'
    );
})();

// Temporary directory for test files
const tmpDir = path.join(
    repoRoot,
    'tooling',
    'playground',
    'tests',
    '.tmp-cli'
);

// Helper to run CLI and capture output
function runCli(args, options = {}) {
    const command = new Deno.Command(cliBinary, {
        args,
        cwd: options.cwd || tmpDir,
        stdout: 'piped',
        stderr: 'piped'
    });
    const result = command.outputSync();
    const decoder = new TextDecoder();
    return {
        stdout: decoder.decode(result.stdout),
        stderr: decoder.decode(result.stderr),
        status: result.code,
        success: result.success
    };
}

// Setup helper
function setupTmpDir() {
    if (existsSync(tmpDir)) {
        fs.rmSync(tmpDir, { recursive: true });
    }
    fs.mkdirSync(tmpDir, { recursive: true });
}

// Cleanup helper
function cleanupTmpDir() {
    if (existsSync(tmpDir)) {
        fs.rmSync(tmpDir, { recursive: true });
    }
}

// ============================================================================
// Basic CLI Tests
// ============================================================================

Deno.test('CLI: expands a simple file with @derive(Debug)', () => {
    setupTmpDir();
    try {
        const inputFile = path.join(tmpDir, 'simple.ts');
        fs.writeFileSync(
            inputFile,
            `/** @derive(Debug) */
interface User {
  name: string;
  age: number;
}`
        );

        const result = runCli(['expand', inputFile, '--builtin-only']);

        assertEquals(
            result.success,
            true,
            `CLI should succeed. stderr: ${result.stderr}`
        );

        // CLI writes status to stdout, not stderr
        const output = result.stdout + result.stderr;
        assert(
            output.includes('wrote expanded output') || output.includes('expanded'),
            `Should report writing output. stdout: ${result.stdout}, stderr: ${result.stderr}`
        );

        const expandedFile = path.join(tmpDir, 'simple.expanded.ts');
        assert(existsSync(expandedFile), 'Expanded file should exist');

        const expandedContent = fs.readFileSync(expandedFile, 'utf8');
        assert(
            expandedContent.includes('toString'),
            'Should generate toString method'
        );
        assert(
            !expandedContent.includes('@derive'),
            'Should strip @derive decorator'
        );
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI: expands a file with multiple macros', () => {
    setupTmpDir();
    try {
        const inputFile = path.join(tmpDir, 'multi-macro.ts');
        fs.writeFileSync(
            inputFile,
            `/** @derive(Debug, Clone, Default) */
interface Config {
  host: string;
  port: number;
  enabled: boolean;
}`
        );

        const result = runCli(['expand', inputFile, '--builtin-only']);

        assertEquals(
            result.success,
            true,
            `CLI should succeed. stderr: ${result.stderr}`
        );

        const expandedFile = path.join(tmpDir, 'multi-macro.expanded.ts');
        const content = fs.readFileSync(expandedFile, 'utf8');

        assert(content.includes('toString'), 'Should have Debug (toString)');
        assert(content.includes('clone'), 'Should have Clone');
        assert(content.includes('DefaultValue'), 'Should have Default');
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI: exits with code 2 when no macros found', () => {
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

Deno.test('CLI: prints to stdout with --print flag', () => {
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

// ============================================================================
// Scan Mode Tests
// ============================================================================

Deno.test('CLI --scan: scans directory and expands all files with macros', () => {
    setupTmpDir();
    const scanDir = path.join(tmpDir, 'scan-test');
    try {
        fs.mkdirSync(scanDir, { recursive: true });

        // Create some test files
        fs.writeFileSync(
            path.join(scanDir, 'user.ts'),
            `/** @derive(Debug) */
interface User { name: string; }`
        );

        fs.writeFileSync(
            path.join(scanDir, 'config.ts'),
            `/** @derive(Clone) */
interface Config { value: number; }`
        );

        fs.writeFileSync(
            path.join(scanDir, 'plain.ts'),
            `interface Plain { x: number; }`
        );

        // Create a subdirectory with more files
        const subDir = path.join(scanDir, 'models');
        fs.mkdirSync(subDir, { recursive: true });
        fs.writeFileSync(
            path.join(subDir, 'entity.ts'),
            `/** @derive(Default) */
interface Entity { id: string; }`
        );

        const result = runCli(['expand', '--scan', scanDir, '--builtin-only'], {
            cwd: repoRoot
        });

        assertEquals(
            result.success,
            true,
            `Scan should succeed. stderr: ${result.stderr}`
        );
        assert(result.stderr.includes('scanning'), 'Should report scanning');
        assert(result.stderr.includes('scan complete'), 'Should report completion');

        // Check that files with macros were expanded
        assert(
            existsSync(path.join(scanDir, 'user.expanded.ts')),
            'user.ts should be expanded'
        );
        assert(
            existsSync(path.join(scanDir, 'config.expanded.ts')),
            'config.ts should be expanded'
        );
        assert(
            existsSync(path.join(scanDir, 'models', 'entity.expanded.ts')),
            'models/entity.ts should be expanded'
        );

        // plain.ts has no macros, should not have an expanded file
        assert(
            !existsSync(path.join(scanDir, 'plain.expanded.ts')),
            'plain.ts should NOT be expanded (no macros)'
        );
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI --scan: skips .expanded. files', () => {
    setupTmpDir();
    const scanDir = path.join(tmpDir, 'scan-skip-test');
    try {
        fs.mkdirSync(scanDir, { recursive: true });

        fs.writeFileSync(
            path.join(scanDir, 'user.ts'),
            `/** @derive(Debug) */
interface User { name: string; }`
        );

        // First scan
        runCli(['expand', '--scan', scanDir, '--builtin-only'], { cwd: repoRoot });

        // Second scan - should not create double-expanded files
        const result = runCli(['expand', '--scan', scanDir, '--builtin-only'], {
            cwd: repoRoot
        });

        assertEquals(result.success, true);
        assert(
            !existsSync(path.join(scanDir, 'user.expanded.expanded.ts')),
            'Should not create double-expanded files'
        );
    } finally {
        cleanupTmpDir();
    }
});

// ============================================================================
// Foreign Types / Config Tests
// ============================================================================

Deno.test('CLI config: builtin-only mode loads config and applies foreign types', () => {
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
            `{ "name": "config-test" }`
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

        const result = runCli([
            'expand',
            path.join(configDir, 'event.ts'),
            '--builtin-only'
        ], {
            cwd: configDir
        });

        assertEquals(
            result.success,
            true,
            `CLI should succeed. stderr: ${result.stderr}`
        );

        const expandedFile = path.join(configDir, 'event.expanded.ts');
        assert(existsSync(expandedFile), 'Expanded file should exist');

        const content = fs.readFileSync(expandedFile, 'utf8');

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
// Output Path Tests
// ============================================================================

Deno.test('CLI output: handles .svelte.ts extension correctly', () => {
    setupTmpDir();
    try {
        const inputFile = path.join(tmpDir, 'component.svelte.ts');
        fs.writeFileSync(
            inputFile,
            `/** @derive(Debug) */
interface Props { value: string; }`
        );

        const result = runCli(['expand', inputFile, '--builtin-only']);

        assertEquals(result.success, true);

        const expectedOutput = path.join(tmpDir, 'component.expanded.svelte.ts');
        assert(
            existsSync(expectedOutput),
            `Should create component.expanded.svelte.ts, not component.svelte.expanded.ts`
        );
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI output: respects --out flag for custom output path', () => {
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

// ============================================================================
// Serde Macro Tests
// ============================================================================

Deno.test('CLI serde: expands Serialize macro correctly', () => {
    setupTmpDir();
    try {
        const inputFile = path.join(tmpDir, 'serialize.ts');
        fs.writeFileSync(
            inputFile,
            `/** @derive(Serialize) */
interface Message {
  id: string;
  content: string;
  timestamp: number;
}`
        );

        const result = runCli(['expand', inputFile, '--builtin-only']);

        assertEquals(
            result.success,
            true,
            `CLI should succeed. stderr: ${result.stderr}`
        );

        const content = fs.readFileSync(
            path.join(tmpDir, 'serialize.expanded.ts'),
            'utf8'
        );
        assert(content.includes('serialize'), 'Should have serialize function');
        assert(
            content.includes('SerializeContext'),
            'Should import SerializeContext'
        );
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI serde: expands Deserialize macro correctly', () => {
    setupTmpDir();
    try {
        const inputFile = path.join(tmpDir, 'deserialize.ts');
        fs.writeFileSync(
            inputFile,
            `/** @derive(Deserialize) */
interface Request {
  method: string;
  path: string;
}`
        );

        const result = runCli(['expand', inputFile, '--builtin-only']);

        assertEquals(
            result.success,
            true,
            `CLI should succeed. stderr: ${result.stderr}`
        );

        const content = fs.readFileSync(
            path.join(tmpDir, 'deserialize.expanded.ts'),
            'utf8'
        );
        assert(content.includes('deserialize'), 'Should have deserialize function');
        assert(
            content.includes('DeserializeContext'),
            'Should import DeserializeContext'
        );
    } finally {
        cleanupTmpDir();
    }
});

Deno.test('CLI serde: expands combined Serialize + Deserialize', () => {
    setupTmpDir();
    try {
        const inputFile = path.join(tmpDir, 'serde-combined.ts');
        fs.writeFileSync(
            inputFile,
            `/** @derive(Serialize, Deserialize) */
interface Data {
  value: string;
  count: number;
}`
        );

        const result = runCli(['expand', inputFile, '--builtin-only']);

        assertEquals(result.success, true);

        const content = fs.readFileSync(
            path.join(tmpDir, 'serde-combined.expanded.ts'),
            'utf8'
        );
        assert(content.includes('serialize'), 'Should have serialize');
        assert(content.includes('deserialize'), 'Should have deserialize');
    } finally {
        cleanupTmpDir();
    }
});
