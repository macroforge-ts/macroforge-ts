#!/usr/bin/env node
/**
 * Isolated test runner — spawns each *.test.mjs in its own Deno subprocess
 * to prevent shared process state (NAPI module cache, CWD, config cache)
 * from leaking between test files.
 *
 * Usage: deno run -A run-isolated.mjs [filter...]
 *   No args   → runs all *.test.mjs
 *   With args → runs only matching files (e.g. "cli" matches "cli.test.mjs")
 */

import { readdirSync } from 'node:fs';
import { basename, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = resolve(fileURLToPath(import.meta.url), '..');
const filters = process.argv.slice(2);

const testFiles = readdirSync(__dirname)
    .filter((f) => f.endsWith('.test.mjs'))
    .filter(
        (f) => filters.length === 0 || filters.some((flt) => f.includes(flt))
    )
    .sort();

const denoArgs = [
    'test',
    '--allow-all',
    '--no-check',
    '--unstable-detect-cjs',
    '--node-modules-dir=manual',
    '--env-file=../../.env'
];

let passed = 0;
let failed = 0;
const failures = [];

for (const file of testFiles) {
    const label = basename(file, '.test.mjs');
    const start = performance.now();

    const cmd = new Deno.Command('deno', {
        args: [...denoArgs, resolve(__dirname, file)],
        cwd: __dirname,
        stdout: 'piped',
        stderr: 'piped',
        env: { ...Deno.env.toObject(), FORCE_COLOR: '1' }
    });

    const proc = cmd.spawn();
    const result = await proc.output();
    const elapsed = ((performance.now() - start) / 1000).toFixed(1);
    const decoder = new TextDecoder();

    if (result.success) {
        // Extract pass count from deno test output
        const summary = decoder.decode(result.stdout).match(/(\d+) passed/);
        const count = summary ? summary[1] : '?';
        console.log(`  \x1b[32m✓\x1b[0m ${label} (${count} tests, ${elapsed}s)`);
        passed++;
    } else {
        console.log(`  \x1b[31m✗\x1b[0m ${label} (${elapsed}s)`);
        failures.push({
            file,
            stdout: decoder.decode(result.stdout),
            stderr: decoder.decode(result.stderr)
        });
        failed++;
    }
}

console.log(
    `\n${passed + failed} suites: \x1b[32m${passed} passed\x1b[0m${
        failed > 0 ? `, \x1b[31m${failed} failed\x1b[0m` : ''
    }`
);

if (failures.length > 0) {
    console.log('\n\x1b[31mFailures:\x1b[0m\n');
    for (const f of failures) {
        console.log(`── ${f.file} ──`);
        // Show only the FAILURES section and error lines
        const combined = f.stdout + f.stderr;
        const lines = combined.split('\n');
        let inFailures = false;
        for (const line of lines) {
            if (line.includes('FAILURES') || line.includes('FAILED')) {
                inFailures = true;
            }
            if (inFailures) console.log(line);
            if (line.includes('error:') && line.includes('AssertionError')) {
                // Print the assertion error and next few lines
                const idx = lines.indexOf(line);
                for (let i = idx; i < Math.min(idx + 5, lines.length); i++) {
                    if (!inFailures) console.log(lines[i]);
                }
            }
        }
        console.log();
    }
    Deno.exit(1);
}
