#!/usr/bin/env -S deno run -A
// Macroforge expansion benchmark.
//
// Usage:
//   pixi run bench              — benchmark current build
//   pixi run bench:all          — compare all backends in bench-bins/
//   pixi run bench:build        — build all 4 variants into bench-bins/
//   pixi run bench:wasm-bindgen — run wasm-bindgen (used by build pipeline)
//   MF_BENCH_ITERATIONS=50      — control iteration count (default: 20)

import { createRequire } from "node:module";
import * as path from "node:path";
import * as fs from "node:fs";
import { execSync } from "node:child_process";

const __dirname = path.dirname(new URL(import.meta.url).pathname);
const root = path.resolve(__dirname, "../..");
const crateDir = path.join(root, "crates/macroforge_ts");
const binsDir = path.join(root, "tooling/bench-bins");
const iterations = parseInt(Deno.env.get("MF_BENCH_ITERATIONS") || "20", 10);

const args = Deno.args;
const mode = args.includes("--build") ? "build"
  : args.includes("--all") ? "all"
  : args.includes("--wasm-bindgen") ? "wasm-bindgen"
  : "current";

// ============================================================================
// wasm-bindgen subcommand (replaces the old wasm-bindgen-build.mjs)
// ============================================================================

async function wasmBindgen() {
  const wasmInput = args.find((a) => a.endsWith(".wasm"));
  const outDir = args.filter((a) => !a.startsWith("--") && !a.endsWith(".wasm")).pop();

  if (!wasmInput || !outDir) {
    console.error("Usage: bench.mjs --wasm-bindgen <input.wasm> <out-dir>");
    Deno.exit(1);
  }

  const bin = await resolveWasmBindgen();
  await Deno.mkdir(outDir, { recursive: true });

  const result = await new Deno.Command(bin, {
    args: ["--target", "nodejs", "--out-dir", outDir, wasmInput],
    stdout: "inherit",
    stderr: "inherit",
  }).output();

  if (!result.success) Deno.exit(result.code);
}

async function resolveWasmBindgen() {
  const fromEnv = Deno.env.get("WASM_BINDGEN");
  if (fromEnv) return fromEnv;

  const which = await new Deno.Command("which", {
    args: ["wasm-bindgen"],
    stdout: "piped",
    stderr: "null",
  }).output();

  if (which.success) {
    const candidate = new TextDecoder().decode(which.stdout).trim();
    if (candidate) return candidate;
  }

  const home = Deno.env.get("HOME");
  if (!home) throw new Error("HOME is not set and wasm-bindgen was not found in PATH");

  const cacheDirs = [`${home}/Library/Caches/.wasm-pack`, `${home}/.cache/.wasm-pack`];
  const candidates = [];
  for (const dir of cacheDirs) await collectWasmBindgenBinaries(dir, candidates);

  if (candidates.length === 0) {
    throw new Error("wasm-bindgen not found in PATH or wasm-pack cache; set WASM_BINDGEN");
  }

  const stats = await Promise.all(
    candidates.map(async (c) => ({ c, stat: await Deno.stat(c) })),
  );
  stats.sort((a, b) => (b.stat.mtime?.getTime() ?? 0) - (a.stat.mtime?.getTime() ?? 0));
  return stats[0].c;
}

async function collectWasmBindgenBinaries(dir, acc) {
  try { await Deno.stat(dir); } catch { return; }
  for await (const entry of Deno.readDir(dir)) {
    const full = `${dir}/${entry.name}`;
    if (entry.isFile && entry.name === "wasm-bindgen") acc.push(full);
    else if (entry.isDirectory) await collectWasmBindgenBinaries(full, acc);
  }
}

// ============================================================================
// Benchmark
// ============================================================================

const benchFiles = [
  "tooling/playground/vanilla/src/user.ts",
  "tooling/playground/svelte/src/lib/demo/macro-user.ts",
  "tooling/playground/vanilla/src/all-macros-test.ts",
  "tooling/playground/vanilla/src/enum-type-examples.ts",
  "tooling/playground/vanilla/src/form-model.ts",
  "tooling/playground/vanilla/src/validator-form.ts",
];

function getInputs() {
  return benchFiles
    .filter((f) => fs.existsSync(path.join(root, f)))
    .map((f) => ({
      name: path.basename(f),
      code: fs.readFileSync(path.join(root, f), "utf8"),
      filepath: path.resolve(root, f),
    }));
}

function bench(label, modPath) {
  const inputs = getInputs();
  const require = createRequire(import.meta.url);
  delete require.cache[require.resolve(modPath)];

  let m;
  try { m = require(modPath); } catch (e) { console.log(`  ${label}: ${e.message.split("\n")[0]}`); return null; }
  if (!m.expandSync) { console.log(`  ${label}: no expandSync`); return null; }

  for (let w = 0; w < 2; w++) for (const i of inputs) try { m.expandSync(i.code, i.filepath); } catch {}

  const results = [];
  for (const i of inputs) {
    const t = [];
    for (let n = 0; n < iterations; n++) {
      const s = performance.now();
      try { m.expandSync(i.code, i.filepath); } catch {}
      t.push(performance.now() - s);
    }
    t.sort((a, b) => a - b);
    results.push({ file: i.name, median: t[t.length >> 1], min: t[0], p95: t[Math.floor(t.length * 0.95)] });
  }
  return { label, results };
}

function printTable(all) {
  const files = all[0].results.map((r) => r.file);
  const header = ["File", ...all.map((r) => r.label)];
  const rows = files.map((f, i) => [f, ...all.map((r) => r.results[i].median.toFixed(1))]);
  const w = header.map((h, c) => Math.max(h.length, ...rows.map((r) => r[c].length)));
  const sep = w.map((n) => "-".repeat(n + 2)).join("+");
  console.log(header.map((h, i) => ` ${h.padEnd(w[i])} `).join("|"));
  console.log(sep);
  for (const row of rows) console.log(row.map((c, i) => ` ${c.padEnd(w[i])} `).join("|"));
  console.log(sep);
  const avg = ["AVERAGE", ...all.map((r) => (r.results.reduce((s, x) => s + x.median, 0) / r.results.length).toFixed(1))];
  console.log(avg.map((c, i) => ` ${c.padEnd(w[i])} `).join("|"));
  if (all.length > 1) {
    const base = all[0].results.reduce((s, x) => s + x.median, 0) / all[0].results.length;
    console.log("\nSpeedup vs " + all[0].label + ":");
    for (const r of all.slice(1)) {
      const a = r.results.reduce((s, x) => s + x.median, 0) / r.results.length;
      console.log(`  ${r.label}: ${(base / a).toFixed(2)}x ${base / a > 1 ? "faster" : "slower"}`);
    }
  }
}

// ============================================================================
// Build
// ============================================================================

function build() {
  fs.mkdirSync(binsDir, { recursive: true });
  const run = (cmd, cwd) => { console.log(`  $ ${cmd}`); execSync(cmd, { cwd, stdio: "inherit" }); };

  console.log("\n[1/4] oxc-wasm");
  run("pixi run build:rust", root);
  fs.cpSync(path.join(crateDir, "pkg"), path.join(binsDir, "oxc-wasm"), { recursive: true });

  console.log("\n[2/4] swc-wasm");
  run("cargo build --release --target wasm32-unknown-unknown --no-default-features --features wasm,swc", crateDir);
  fs.mkdirSync(path.join(binsDir, "swc-wasm"), { recursive: true });
  run(`deno run -A ${path.join(root, "tooling/scripts/bench.mjs")} --wasm-bindgen target/wasm32-unknown-unknown/release/macroforge_ts.wasm ${path.join(binsDir, "swc-wasm")}`, crateDir);

  console.log("\n[3/4] oxc-native");
  fs.mkdirSync(path.join(binsDir, "oxc-native"), { recursive: true });
  run(`deno run -A npm:@napi-rs/cli/napi build --platform --release --no-default-features --features node,oxc --output-dir ${path.join(binsDir, "oxc-native")}`, crateDir);

  console.log("\n[4/4] swc-native");
  fs.mkdirSync(path.join(binsDir, "swc-native"), { recursive: true });
  run(`deno run -A npm:@napi-rs/cli/napi build --platform --release --no-default-features --features node,swc --output-dir ${path.join(binsDir, "swc-native")}`, crateDir);

  console.log("\nRestoring default...");
  run("pixi run build:rust", root);
  console.log("Done. Run: pixi run bench:all");
}

// ============================================================================
// Main
// ============================================================================

if (mode === "wasm-bindgen") {
  await wasmBindgen();
} else if (mode === "build") {
  build();
} else if (mode === "all") {
  if (!fs.existsSync(binsDir)) { console.error("No bench-bins/. Run: pixi run bench:build"); Deno.exit(1); }
  const combos = fs.readdirSync(binsDir, { withFileTypes: true }).filter((d) => d.isDirectory()).map((d) => d.name).sort();
  if (!combos.length) { console.error("Empty bench-bins/. Run: pixi run bench:build"); Deno.exit(1); }
  console.log(`\nMacroforge Benchmark — ${iterations} iterations`);
  console.log("=".repeat(60));
  const all = [];
  for (const c of combos) {
    const dir = path.join(binsDir, c);
    const entry = fs.existsSync(path.join(dir, "macroforge_ts.js")) ? path.join(dir, "macroforge_ts.js") : path.join(dir, "index.js");
    console.log(`  ${c}...`);
    const r = bench(c, entry);
    if (r) all.push(r);
  }
  if (!all.length) { console.log("No results."); Deno.exit(1); }
  console.log("\nMedian expansion time (ms)");
  console.log("=".repeat(60));
  printTable(all);
} else {
  const entry = path.join(crateDir, "pkg/macroforge_ts.js");
  if (!fs.existsSync(entry)) { console.error("No pkg/. Run: pixi run build:rust"); Deno.exit(1); }
  console.log(`\nMacroforge Benchmark — ${iterations} iterations`);
  console.log("=".repeat(60));
  const r = bench("current", entry);
  if (!r) Deno.exit(1);
  for (const x of r.results) console.log(`  ${x.file.padEnd(25)} ${x.median.toFixed(1)}ms median  ${x.min.toFixed(1)}ms min  ${x.p95.toFixed(1)}ms p95`);
}
console.log();
