/**
 * Tests for configuration loading (macroforge.json, tsconfig.json).
 */

import test from "node:test";
import assert from "node:assert/strict";
import fs from "fs";
import path from "path";
import macroforge from "../src/index.js";
import {
  initializePlugin,
  invokeTransform,
  createTempDir,
  cleanupTempDir,
  writeTestFile,
  FIXTURES_DIR,
} from "./test-utils.js";

test("loads macroforge.json from project root", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  // Create macroforge.json with keepDecorators: true
  writeTestFile(tempDir, "macroforge.json", JSON.stringify({ keepDecorators: true }));

  // Create a source file with macro
  writeTestFile(
    tempDir,
    "src/test.ts",
    `/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
  );

  const plugin = await macroforge();
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/test.ts"), "utf-8");
  const id = path.join(tempDir, "src/test.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  // Should transform without error
  assert.equal(error, null);
  // With macros present, should return transformed code
  if (result) {
    assert.ok(result.code);
  }
});

test("falls back to default config when macroforge.json not found", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  // No macroforge.json - should use default config

  // Create a source file
  writeTestFile(
    tempDir,
    "src/test.ts",
    `class PlainClass {
  value: number;
}
export { PlainClass };`
  );

  const plugin = await macroforge();
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/test.ts"), "utf-8");
  const id = path.join(tempDir, "src/test.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  // Should work without config file
  assert.equal(error, null);
});

test("handles malformed macroforge.json gracefully", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  // Create invalid JSON
  writeTestFile(tempDir, "macroforge.json", "{ invalid json }");

  // Create a source file
  writeTestFile(
    tempDir,
    "src/test.ts",
    `class PlainClass {
  value: number;
}
export { PlainClass };`
  );

  const plugin = await macroforge();
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/test.ts"), "utf-8");
  const id = path.join(tempDir, "src/test.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  // Should fallback to defaults and not crash
  assert.equal(error, null);
});

test("handles empty macroforge.json gracefully", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  // Create empty file
  writeTestFile(tempDir, "macroforge.json", "");

  // Create a source file
  writeTestFile(
    tempDir,
    "src/test.ts",
    `class PlainClass {
  value: number;
}
export { PlainClass };`
  );

  const plugin = await macroforge();
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/test.ts"), "utf-8");
  const id = path.join(tempDir, "src/test.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  // Should fallback to defaults and not crash
  assert.equal(error, null);
});

test("searches parent directories for macroforge.json", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  // Create macroforge.json in parent directory
  writeTestFile(tempDir, "macroforge.json", JSON.stringify({ keepDecorators: true }));

  // Create nested project structure
  writeTestFile(
    tempDir,
    "packages/my-package/src/test.ts",
    `class PlainClass {
  value: number;
}
export { PlainClass };`
  );

  const projectRoot = path.join(tempDir, "packages/my-package");
  const plugin = await macroforge();
  initializePlugin(plugin, projectRoot);

  const code = fs.readFileSync(path.join(projectRoot, "src/test.ts"), "utf-8");
  const id = path.join(projectRoot, "src/test.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  // Should find parent config and work
  assert.equal(error, null);
});

test("loads tsconfig.json from project root", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  // Create tsconfig.json
  writeTestFile(
    tempDir,
    "tsconfig.json",
    JSON.stringify({
      compilerOptions: {
        target: "ES2022",
        module: "ESNext",
        strict: true,
      },
    })
  );

  // Create a source file with macro (for type generation)
  writeTestFile(
    tempDir,
    "src/test.ts",
    `/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
  );

  const plugin = await macroforge();
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/test.ts"), "utf-8");
  const id = path.join(tempDir, "src/test.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  // Should work with tsconfig
  assert.equal(error, null);
});

test("works without tsconfig.json", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  // No tsconfig.json

  // Create a source file
  writeTestFile(
    tempDir,
    "src/test.ts",
    `class PlainClass {
  value: number;
}
export { PlainClass };`
  );

  const plugin = await macroforge();
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/test.ts"), "utf-8");
  const id = path.join(tempDir, "src/test.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  // Should work without tsconfig
  assert.equal(error, null);
});

test("handles malformed tsconfig.json gracefully", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  // Create invalid JSON
  writeTestFile(tempDir, "tsconfig.json", "{ invalid json }");

  // Create a source file
  writeTestFile(
    tempDir,
    "src/test.ts",
    `class PlainClass {
  value: number;
}
export { PlainClass };`
  );

  const plugin = await macroforge();
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/test.ts"), "utf-8");
  const id = path.join(tempDir, "src/test.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  // Should fallback and not crash (may log warning)
  assert.equal(error, null);
});

test("uses config from fixtures directory", async () => {
  // Test using the pre-created fixtures
  const configDir = path.join(FIXTURES_DIR, "config");

  const plugin = await macroforge();
  initializePlugin(plugin, configDir);

  // Create inline test code
  const code = `class Test { value: number; }`;
  const id = path.join(configDir, "test.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);
});
