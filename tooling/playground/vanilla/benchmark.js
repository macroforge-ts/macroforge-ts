import pkg from 'macroforge';
const { expandSync: expand_node } = pkg;
import * as expand_wasm from '../../../crates/macroforge_ts/pkg-wasm/macroforge_ts.js';
import fs from 'node:fs';
import path from 'node:path';
import { performance } from 'node:perf_hooks';

const TEST_FILE = path.join(process.cwd(), 'src/validator-form.ts');
const code = fs.readFileSync(TEST_FILE, 'utf-8');
const ITERATIONS = 100;

async function runBenchmark() {
    console.log(
        `\n🚀 Benchmarking Macroforge: Node vs WASM (${ITERATIONS} iterations)`
    );
    console.log(
        `File: ${path.basename(TEST_FILE)} (${(code.length / 1024).toFixed(2)} KB)\n`
    );

    // Warmup
    expand_node(code, TEST_FILE, { keep_decorators: false });
    expand_wasm.expand_sync(code, TEST_FILE, { keep_decorators: false });

    // Node Benchmark
    const startNode = performance.now();
    for (let i = 0; i < ITERATIONS; i++) {
        expand_node(code, TEST_FILE, { keep_decorators: false });
    }
    const endNode = performance.now();
    const nodeTotal = endNode - startNode;
    const nodeAvg = nodeTotal / ITERATIONS;

    // WASM Benchmark
    const startWasm = performance.now();
    for (let i = 0; i < ITERATIONS; i++) {
        expand_wasm.expand_sync(code, TEST_FILE, { keep_decorators: false });
    }
    const endWasm = performance.now();
    const wasmTotal = endWasm - startWasm;
    const wasmAvg = wasmTotal / ITERATIONS;

    console.log(`--------------------------------------------------`);
    console.log(`| Target | Total Time | Avg Time | Operations/sec |`);
    console.log(`|--------|------------|----------|----------------|`);
    console.log(
        `| Node   | ${nodeTotal.toFixed(2)}ms   | ${nodeAvg.toFixed(2)}ms   | ${
            (1000 / nodeAvg).toFixed(2)
        } ops/s |`
    );
    console.log(
        `| WASM   | ${wasmTotal.toFixed(2)}ms   | ${wasmAvg.toFixed(2)}ms   | ${
            (1000 / wasmAvg).toFixed(2)
        } ops/s |`
    );
    console.log(`--------------------------------------------------`);

    const diff = (wasmAvg / nodeAvg).toFixed(2);
    if (nodeAvg < wasmAvg) {
        console.log(`\n✅ Node is ${diff}x faster than WASM`);
    } else {
        console.log(
            `\n✅ WASM is ${(nodeAvg / wasmAvg).toFixed(2)}x faster than Node`
        );
    }
}

runBenchmark().catch(console.error);
