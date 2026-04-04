import pkg from 'macroforge';
const { transformSync: transform_node_oxc } = pkg;
import * as wasm_oxc from '../../../crates/macroforge_ts/pkg/macroforge_ts.js';
import fs from 'node:fs';
import path from 'node:path';
import { performance } from 'node:perf_hooks';

const TEST_FILE = path.join(process.cwd(), 'src/validator-form.ts');
const code = fs.readFileSync(TEST_FILE, 'utf-8');
const ITERATIONS = 1000; // Increased for better accuracy

async function runBenchmark() {
    console.log(
        `\n🚀 Benchmarking Oxc Performance: Native Node vs WASM (${ITERATIONS} iterations)`
    );
    console.log(
        `File: ${path.basename(TEST_FILE)} (${(code.length / 1024).toFixed(2)} KB)\n`
    );

    // Warmup
    transform_node_oxc(code, TEST_FILE);
    wasm_oxc.transform_sync(code, TEST_FILE);

    // Node (Native Oxc) Benchmark
    const startNode = performance.now();
    for (let i = 0; i < ITERATIONS; i++) {
        transform_node_oxc(code, TEST_FILE);
    }
    const endNode = performance.now();
    const nodeTotal = endNode - startNode;
    const nodeAvg = nodeTotal / ITERATIONS;

    // WASM (Oxc) Benchmark
    const startWasmOxc = performance.now();
    for (let i = 0; i < ITERATIONS; i++) {
        wasm_oxc.transform_sync(code, TEST_FILE);
    }
    const endWasmOxc = performance.now();
    const wasmOxcTotal = endWasmOxc - startWasmOxc;
    const wasmOxcAvg = wasmOxcTotal / ITERATIONS;

    console.log(
        `------------------------------------------------------------------`
    );
    console.log(
        `| Target           | Total Time | Avg Time | Operations/sec      |`
    );
    console.log(
        `|------------------|------------|----------|---------------------|`
    );
    console.log(
        `| Node (Native Oxc)| ${nodeTotal.toFixed(2)}ms   | ${nodeAvg.toFixed(3)}ms   | ${
            (1000 / nodeAvg).toFixed(2)
        } ops/s |`
    );
    console.log(
        `| WASM (Oxc)       | ${wasmOxcTotal.toFixed(2)}ms   | ${wasmOxcAvg.toFixed(3)}ms   | ${
            (1000 / wasmOxcAvg).toFixed(2)
        } ops/s |`
    );
    console.log(
        `------------------------------------------------------------------`
    );

    console.log(`\n📊 Comparison:`);
    console.log(
        `✅ Native Oxc is ${(wasmOxcAvg / nodeAvg).toFixed(2)}x faster than WASM Oxc`
    );
}

runBenchmark().catch(console.error);
