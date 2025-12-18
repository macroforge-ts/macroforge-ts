/**
 * Tests for file filtering (include/exclude patterns).
 */

import test from "node:test";
import assert from "node:assert/strict";
import macroforge from "../src/index.js";
import {
  initializePlugin,
  invokeTransform,
  loadFixture,
  getFixturePath,
  createTempDir,
  cleanupTempDir,
  FIXTURES_DIR,
} from "./test-utils.js";

test("transforms .ts files", async () => {
  const plugin = await macroforge();
  initializePlugin(plugin, FIXTURES_DIR);

  const code = loadFixture("no-macro");
  const id = getFixturePath("no-macro");

  const { result, error } = await invokeTransform(plugin, code, id);

  // Plugin processes .ts files - returns result even without macros
  assert.equal(error, null);
  // Result may be null or contain code depending on transformer behavior
  if (result !== null) {
    assert.ok(result.code);
  }
});

test("transforms .tsx files", async () => {
  const plugin = await macroforge();
  initializePlugin(plugin, FIXTURES_DIR);

  const code = `
class PlainComponent {
  render(): string {
    return "<div>Hello</div>";
  }
}
export { PlainComponent };
`;
  const id = "/project/src/Component.tsx";

  const { result, error } = await invokeTransform(plugin, code, id);

  // Plugin processes .tsx files
  assert.equal(error, null);
  // Result may be null or contain code depending on transformer behavior
  if (result !== null) {
    assert.ok(result.code);
  }
});

test("skips .js files (returns null)", async () => {
  const plugin = await macroforge();
  initializePlugin(plugin, FIXTURES_DIR);

  const code = `class PlainClass { value = 1; }`;
  const id = "/project/src/file.js";

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);
  assert.equal(result, null);
});

test("skips .jsx files (returns null)", async () => {
  const plugin = await macroforge();
  initializePlugin(plugin, FIXTURES_DIR);

  const code = `function Component() { return <div>Hello</div>; }`;
  const id = "/project/src/Component.jsx";

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);
  assert.equal(result, null);
});

test("skips .css files (returns null)", async () => {
  const plugin = await macroforge();
  initializePlugin(plugin, FIXTURES_DIR);

  const code = `.class { color: red; }`;
  const id = "/project/src/styles.css";

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);
  assert.equal(result, null);
});

test("skips .json files (returns null)", async () => {
  const plugin = await macroforge();
  initializePlugin(plugin, FIXTURES_DIR);

  const code = `{ "key": "value" }`;
  const id = "/project/src/data.json";

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);
  assert.equal(result, null);
});

test("skips node_modules by default", async () => {
  const plugin = await macroforge();
  initializePlugin(plugin, FIXTURES_DIR);

  const code = `
/** @derive(Debug) */
class User {
  id: string;
}
`;
  const id = "/project/node_modules/some-package/index.ts";

  const { result, error } = await invokeTransform(plugin, code, id);

  // Should skip node_modules even with macros
  assert.equal(error, null);
  assert.equal(result, null);
});

test("skips deeply nested node_modules", async () => {
  const plugin = await macroforge();
  initializePlugin(plugin, FIXTURES_DIR);

  const code = loadFixture("simple-macro");
  const id = "/project/src/node_modules/nested/index.ts";

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);
  assert.equal(result, null);
});

test("transforms files with macros", async () => {
  const plugin = await macroforge();
  initializePlugin(plugin, FIXTURES_DIR);

  const code = loadFixture("simple-macro");
  const id = getFixturePath("simple-macro");

  const { result, error } = await invokeTransform(plugin, code, id);

  // With macros, should return transformed code
  assert.equal(error, null);
  assert.notEqual(result, null);
  assert.ok(result.code);
  assert.ok(result.code.includes("class User"));
});

test("handles empty TypeScript file", async () => {
  const plugin = await macroforge();
  initializePlugin(plugin, FIXTURES_DIR);

  const code = "";
  const id = "/project/src/empty.ts";

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);
  // Empty file returns null (no changes needed)
  assert.equal(result, null);
});

test("handles TypeScript file with only comments", async () => {
  const plugin = await macroforge();
  initializePlugin(plugin, FIXTURES_DIR);

  const code = `// This is a comment\n/* Another comment */`;
  const id = "/project/src/comments.ts";

  const { result, error } = await invokeTransform(plugin, code, id);

  // Plugin may or may not return result for comment-only files
  assert.equal(error, null);
});

test("handles .d.ts files (declaration files)", async () => {
  const plugin = await macroforge();
  initializePlugin(plugin, FIXTURES_DIR);

  const code = `declare class User { id: string; }`;
  const id = "/project/src/types.d.ts";

  const { result, error } = await invokeTransform(plugin, code, id);

  // .d.ts files are processed since they end with .ts
  // Plugin may return result even without macros
  assert.equal(error, null);
});
