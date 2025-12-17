/**
 * CLI integration tests for the macroforge binary.
 *
 * Tests the CLI's ability to:
 * - Expand single files
 * - Scan directories
 * - Load and respect macroforge.config.ts (foreign types)
 * - Work in both default and --builtin-only modes
 */

import { test, describe, before, after } from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { execSync, spawnSync } from "node:child_process";
import { repoRoot } from "./test-utils.mjs";

// Path to the CLI binary
const cliBinary = (() => {
  const release = path.join(repoRoot, "crates", "target", "release", "macroforge");
  const debug = path.join(repoRoot, "crates", "target", "debug", "macroforge");
  if (fs.existsSync(release)) return release;
  if (fs.existsSync(debug)) return debug;
  throw new Error("macroforge CLI binary not found. Run `cargo build --release` first.");
})();

// Temporary directory for test files
const tmpDir = path.join(repoRoot, "playground", "tests", ".tmp-cli");

// Helper to run CLI and capture output
function runCli(args, options = {}) {
  const result = spawnSync(cliBinary, args, {
    cwd: options.cwd || tmpDir,
    encoding: "utf8",
    timeout: 30000,
    ...options,
  });
  return {
    stdout: result.stdout || "",
    stderr: result.stderr || "",
    status: result.status,
    success: result.status === 0,
  };
}

// Setup and teardown
before(() => {
  if (fs.existsSync(tmpDir)) {
    fs.rmSync(tmpDir, { recursive: true });
  }
  fs.mkdirSync(tmpDir, { recursive: true });
});

after(() => {
  if (fs.existsSync(tmpDir)) {
    fs.rmSync(tmpDir, { recursive: true });
  }
});

// ============================================================================
// Basic CLI Tests
// ============================================================================

describe("CLI basic functionality", () => {
  test("expands a simple file with @derive(Debug)", () => {
    const inputFile = path.join(tmpDir, "simple.ts");
    fs.writeFileSync(
      inputFile,
      `/** @derive(Debug) */
interface User {
  name: string;
  age: number;
}`
    );

    const result = runCli(["expand", inputFile, "--builtin-only"]);

    assert.equal(result.success, true, `CLI should succeed. stderr: ${result.stderr}`);

    // CLI writes status to stdout, not stderr
    const output = result.stdout + result.stderr;
    assert.ok(
      output.includes("wrote expanded output") || output.includes("expanded"),
      `Should report writing output. stdout: ${result.stdout}, stderr: ${result.stderr}`
    );

    const expandedFile = path.join(tmpDir, "simple.expanded.ts");
    assert.ok(fs.existsSync(expandedFile), "Expanded file should exist");

    const expandedContent = fs.readFileSync(expandedFile, "utf8");
    assert.ok(expandedContent.includes("toString"), "Should generate toString method");
    assert.ok(!expandedContent.includes("@derive"), "Should strip @derive decorator");
  });

  test("expands a file with multiple macros", () => {
    const inputFile = path.join(tmpDir, "multi-macro.ts");
    fs.writeFileSync(
      inputFile,
      `/** @derive(Debug, Clone, Default) */
interface Config {
  host: string;
  port: number;
  enabled: boolean;
}`
    );

    const result = runCli(["expand", inputFile, "--builtin-only"]);

    assert.equal(result.success, true, `CLI should succeed. stderr: ${result.stderr}`);

    const expandedFile = path.join(tmpDir, "multi-macro.expanded.ts");
    const content = fs.readFileSync(expandedFile, "utf8");

    assert.ok(content.includes("toString"), "Should have Debug (toString)");
    assert.ok(content.includes("clone"), "Should have Clone");
    assert.ok(content.includes("DefaultValue"), "Should have Default");
  });

  test("exits with code 2 when no macros found", () => {
    const inputFile = path.join(tmpDir, "no-macros.ts");
    fs.writeFileSync(
      inputFile,
      `interface Plain {
  value: string;
}`
    );

    const result = runCli(["expand", inputFile, "--builtin-only", "--quiet"]);

    assert.equal(result.status, 2, "Should exit with code 2 when no macros found");
  });

  test("prints to stdout with --print flag", () => {
    const inputFile = path.join(tmpDir, "print-test.ts");
    fs.writeFileSync(
      inputFile,
      `/** @derive(Debug) */
interface Item { id: string; }`
    );

    const result = runCli(["expand", inputFile, "--builtin-only", "--print"]);

    assert.equal(result.success, true);
    assert.ok(result.stdout.includes("toString"), "Should print expanded code to stdout");
  });
});

// ============================================================================
// Scan Mode Tests
// ============================================================================

describe("CLI --scan mode", () => {
  const scanDir = path.join(tmpDir, "scan-test");

  before(() => {
    fs.mkdirSync(scanDir, { recursive: true });

    // Create some test files
    fs.writeFileSync(
      path.join(scanDir, "user.ts"),
      `/** @derive(Debug) */
interface User { name: string; }`
    );

    fs.writeFileSync(
      path.join(scanDir, "config.ts"),
      `/** @derive(Clone) */
interface Config { value: number; }`
    );

    fs.writeFileSync(
      path.join(scanDir, "plain.ts"),
      `interface Plain { x: number; }`
    );

    // Create a subdirectory with more files
    const subDir = path.join(scanDir, "models");
    fs.mkdirSync(subDir, { recursive: true });
    fs.writeFileSync(
      path.join(subDir, "entity.ts"),
      `/** @derive(Default) */
interface Entity { id: string; }`
    );
  });

  test("scans directory and expands all files with macros", () => {
    const result = runCli(["expand", "--scan", scanDir, "--builtin-only"], { cwd: repoRoot });

    assert.equal(result.success, true, `Scan should succeed. stderr: ${result.stderr}`);
    assert.ok(result.stderr.includes("scanning"), "Should report scanning");
    assert.ok(result.stderr.includes("scan complete"), "Should report completion");

    // Check that files with macros were expanded
    assert.ok(
      fs.existsSync(path.join(scanDir, "user.expanded.ts")),
      "user.ts should be expanded"
    );
    assert.ok(
      fs.existsSync(path.join(scanDir, "config.expanded.ts")),
      "config.ts should be expanded"
    );
    assert.ok(
      fs.existsSync(path.join(scanDir, "models", "entity.expanded.ts")),
      "models/entity.ts should be expanded"
    );

    // plain.ts has no macros, should not have an expanded file
    assert.ok(
      !fs.existsSync(path.join(scanDir, "plain.expanded.ts")),
      "plain.ts should NOT be expanded (no macros)"
    );
  });

  test("scan skips .expanded. files", () => {
    // The previous test created .expanded. files
    // Running scan again should not create .expanded.expanded. files
    const result = runCli(["expand", "--scan", scanDir, "--builtin-only"], { cwd: repoRoot });

    assert.equal(result.success, true);
    assert.ok(
      !fs.existsSync(path.join(scanDir, "user.expanded.expanded.ts")),
      "Should not create double-expanded files"
    );
  });
});

// ============================================================================
// Foreign Types / Config Tests
// ============================================================================

describe("CLI config loading (foreign types)", () => {
  const configDir = path.join(tmpDir, "config-test");

  before(() => {
    fs.mkdirSync(configDir, { recursive: true });

    // Create a macroforge.config.ts with foreign type
    fs.writeFileSync(
      path.join(configDir, "macroforge.config.ts"),
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
      path.join(configDir, "package.json"),
      `{ "name": "config-test" }`
    );

    // Create a file that uses the foreign type
    fs.writeFileSync(
      path.join(configDir, "event.ts"),
      `import type { DateTime } from 'effect';

/** @derive(Default, Serialize) */
interface Event {
  name: string;
  startTime: DateTime.DateTime;
}`
    );
  });

  test("builtin-only mode loads config and applies foreign types", () => {
    const result = runCli(
      ["expand", path.join(configDir, "event.ts"), "--builtin-only"],
      { cwd: configDir }
    );

    assert.equal(result.success, true, `CLI should succeed. stderr: ${result.stderr}`);

    const expandedFile = path.join(configDir, "event.expanded.ts");
    assert.ok(fs.existsSync(expandedFile), "Expanded file should exist");

    const content = fs.readFileSync(expandedFile, "utf8");

    // Check that foreign type handlers were used
    assert.ok(
      content.includes("toISOString") || content.includes("new Date"),
      `Should use foreign type handlers from config. Got: ${content.substring(0, 500)}...`
    );
  });
});

// ============================================================================
// Output Path Tests
// ============================================================================

describe("CLI output path handling", () => {
  test("handles .svelte.ts extension correctly", () => {
    const inputFile = path.join(tmpDir, "component.svelte.ts");
    fs.writeFileSync(
      inputFile,
      `/** @derive(Debug) */
interface Props { value: string; }`
    );

    const result = runCli(["expand", inputFile, "--builtin-only"]);

    assert.equal(result.success, true);

    const expectedOutput = path.join(tmpDir, "component.expanded.svelte.ts");
    assert.ok(
      fs.existsSync(expectedOutput),
      `Should create component.expanded.svelte.ts, not component.svelte.expanded.ts`
    );
  });

  test("respects --out flag for custom output path", () => {
    const inputFile = path.join(tmpDir, "custom-out.ts");
    const outputFile = path.join(tmpDir, "output", "custom.generated.ts");
    fs.writeFileSync(
      inputFile,
      `/** @derive(Debug) */
interface Custom { x: number; }`
    );

    const result = runCli(["expand", inputFile, "--builtin-only", "--out", outputFile]);

    assert.equal(result.success, true, `CLI should succeed. stderr: ${result.stderr}`);
    assert.ok(fs.existsSync(outputFile), "Custom output file should exist");
  });
});

// ============================================================================
// Serde Macro Tests
// ============================================================================

describe("CLI serde macro expansion", () => {
  test("expands Serialize macro correctly", () => {
    const inputFile = path.join(tmpDir, "serialize.ts");
    fs.writeFileSync(
      inputFile,
      `/** @derive(Serialize) */
interface Message {
  id: string;
  content: string;
  timestamp: number;
}`
    );

    const result = runCli(["expand", inputFile, "--builtin-only"]);

    assert.equal(result.success, true, `CLI should succeed. stderr: ${result.stderr}`);

    const content = fs.readFileSync(path.join(tmpDir, "serialize.expanded.ts"), "utf8");
    assert.ok(content.includes("serialize"), "Should have serialize function");
    assert.ok(content.includes("SerializeContext"), "Should import SerializeContext");
  });

  test("expands Deserialize macro correctly", () => {
    const inputFile = path.join(tmpDir, "deserialize.ts");
    fs.writeFileSync(
      inputFile,
      `/** @derive(Deserialize) */
interface Request {
  method: string;
  path: string;
}`
    );

    const result = runCli(["expand", inputFile, "--builtin-only"]);

    assert.equal(result.success, true, `CLI should succeed. stderr: ${result.stderr}`);

    const content = fs.readFileSync(path.join(tmpDir, "deserialize.expanded.ts"), "utf8");
    assert.ok(content.includes("deserialize"), "Should have deserialize function");
    assert.ok(content.includes("DeserializeContext"), "Should import DeserializeContext");
  });

  test("expands combined Serialize + Deserialize", () => {
    const inputFile = path.join(tmpDir, "serde-combined.ts");
    fs.writeFileSync(
      inputFile,
      `/** @derive(Serialize, Deserialize) */
interface Data {
  value: string;
  count: number;
}`
    );

    const result = runCli(["expand", inputFile, "--builtin-only"]);

    assert.equal(result.success, true);

    const content = fs.readFileSync(path.join(tmpDir, "serde-combined.expanded.ts"), "utf8");
    assert.ok(content.includes("serialize"), "Should have serialize");
    assert.ok(content.includes("deserialize"), "Should have deserialize");
  });
});
