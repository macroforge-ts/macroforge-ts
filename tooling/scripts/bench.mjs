#!/usr/bin/env -S deno run -A
// Macroforge expansion benchmark.
//
// Usage:
//   pixi run bench                                  — benchmark current build
//   bench.mjs --wasm-bindgen <input.wasm> <node-out-dir> <deno-out-dir>
//                                                   run wasm-bindgen for npm and
//                                                   JSR (the build:wasm task)
//   MF_BENCH_ITERATIONS=50                          — control iteration count (default: 20)

import { pathToFileURL } from 'node:url';
import * as path from 'node:path';
import * as fs from 'node:fs';

const __dirname = path.dirname(new URL(import.meta.url).pathname);
const root = path.resolve(__dirname, '../..');
const crateDir = path.join(root, 'crates/macroforge_ts');
const iterations = parseInt(Deno.env.get('MF_BENCH_ITERATIONS') || '20', 10);

const args = Deno.args;
const mode = args.includes('--wasm-bindgen') ? 'wasm-bindgen' : 'current';

// ============================================================================
// wasm-bindgen subcommand (replaces the old wasm-bindgen-build.mjs)
// ============================================================================

// The npm package sits in node_modules, so its glue reads the wasm off disk
// with `node:fs`. A JSR module runs from `https://jsr.io/`, where only `fetch`
// reaches the sidecar, so each registry gets the glue that suits it.
const WASM_TARGETS = ['experimental-nodejs-module', 'deno'];

async function wasmBindgen() {
    const wasmInput = args.find((arg) => arg.endsWith('.wasm'));
    const outDirs = args.filter((arg) => !arg.startsWith('--') && !arg.endsWith('.wasm'));

    if (!wasmInput || outDirs.length !== WASM_TARGETS.length) {
        console.error('Usage: bench.mjs --wasm-bindgen <input.wasm> <node-out-dir> <deno-out-dir>');
        Deno.exit(1);
    }

    const bin = await resolveWasmBindgen();

    for (const [index, target] of WASM_TARGETS.entries()) {
        const outDir = outDirs[index];
        await Deno.mkdir(outDir, { recursive: true });

        const result = await new Deno.Command(bin, {
            args: ['--target', target, '--out-dir', outDir, wasmInput],
            stdout: 'inherit',
            stderr: 'inherit'
        }).output();

        if (!result.success) Deno.exit(result.code);
    }
}

async function resolveWasmBindgen() {
    const fromEnv = Deno.env.get('WASM_BINDGEN');
    if (fromEnv) return fromEnv;

    const which = await new Deno.Command('which', {
        args: ['wasm-bindgen'],
        stdout: 'piped',
        stderr: 'null'
    }).output();

    if (which.success) {
        const candidate = new TextDecoder().decode(which.stdout).trim();
        if (candidate) return candidate;
    }

    const home = Deno.env.get('HOME');
    if (!home) throw new Error('HOME is not set and wasm-bindgen was not found in PATH');

    const cacheDirs = [`${home}/Library/Caches/.wasm-pack`, `${home}/.cache/.wasm-pack`];
    const candidates = [];
    for (const dir of cacheDirs) await collectWasmBindgenBinaries(dir, candidates);

    if (candidates.length === 0) {
        throw new Error('wasm-bindgen not found in PATH or wasm-pack cache; set WASM_BINDGEN');
    }

    const stats = await Promise.all(
        candidates.map(async (c) => ({ c, stat: await Deno.stat(c) }))
    );
    stats.sort((a, b) => (b.stat.mtime?.getTime() ?? 0) - (a.stat.mtime?.getTime() ?? 0));
    return stats[0].c;
}

async function collectWasmBindgenBinaries(dir, acc) {
    try {
        await Deno.stat(dir);
    } catch {
        return;
    }
    for await (const entry of Deno.readDir(dir)) {
        const full = `${dir}/${entry.name}`;
        if (entry.isFile && entry.name === 'wasm-bindgen') acc.push(full);
        else if (entry.isDirectory) await collectWasmBindgenBinaries(full, acc);
    }
}

// ============================================================================
// Benchmark
// ============================================================================

const benchFiles = [
    'tooling/playground/vanilla/src/user.ts',
    'tooling/playground/svelte/src/lib/demo/macro-user.ts',
    'tooling/playground/vanilla/src/all-macros-test.ts',
    'tooling/playground/vanilla/src/enum-type-examples.ts',
    'tooling/playground/vanilla/src/form-model.ts',
    'tooling/playground/vanilla/src/validator-form.ts'
];

function getInputs() {
    return benchFiles
        .filter((f) => fs.existsSync(path.join(root, f)))
        .map((f) => ({
            name: path.basename(f),
            code: fs.readFileSync(path.join(root, f), 'utf8'),
            filepath: path.resolve(root, f)
        }));
}

async function bench(label, modPath) {
    const inputs = getInputs();

    let m;
    try {
        m = await import(pathToFileURL(modPath).href);
    } catch (e) {
        console.log(`  ${label}: ${e.message.split('\n')[0]}`);
        return null;
    }
    if (!m.expandSync) {
        console.log(`  ${label}: no expandSync`);
        return null;
    }

    for (let w = 0; w < 2; w++) {
        for (const i of inputs) {
            m.expandSync(i.code, i.filepath);
        }
    }

    const results = [];
    for (const i of inputs) {
        const t = [];
        for (let n = 0; n < iterations; n++) {
            const s = performance.now();
            m.expandSync(i.code, i.filepath);
            t.push(performance.now() - s);
        }
        t.sort((a, b) => a - b);
        results.push({
            file: i.name,
            median: t[t.length >> 1],
            min: t[0],
            p95: t[Math.floor(t.length * 0.95)]
        });
    }
    return { label, results };
}

// ============================================================================
// Main
// ============================================================================

if (mode === 'wasm-bindgen') {
    await wasmBindgen();
} else {
    const entry = path.join(crateDir, 'pkg/macroforge_ts.js');
    if (!fs.existsSync(entry)) {
        console.error('No pkg/. Run: pixi run build:rust');
        Deno.exit(1);
    }
    console.log(`\nMacroforge Benchmark — ${iterations} iterations`);
    console.log('='.repeat(60));
    const r = await bench('current', entry);
    if (!r) Deno.exit(1);
    for (const x of r.results) {
        console.log(
            `  ${x.file.padEnd(25)} ${x.median.toFixed(1)}ms median  ${x.min.toFixed(1)}ms min  ${
                x.p95.toFixed(1)
            }ms p95`
        );
    }
}
console.log();
