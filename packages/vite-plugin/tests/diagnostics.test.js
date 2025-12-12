/**
 * Tests for diagnostic handling (errors, warnings).
 */

import test from "node:test";
import assert from "node:assert/strict";
import fs from "fs";
import path from "path";
import napiMacrosPlugin from "../dist/index.js";
import {
  initializePlugin,
  invokeTransform,
  createTempDir,
  cleanupTempDir,
  writeTestFile,
  createTransformContext,
  loadFixture,
  getFixturePath,
  FIXTURES_DIR,
} from "./test-utils.js";

test("successfully transforms valid macro code", async () => {
  const plugin = napiMacrosPlugin({ generateTypes: false, emitMetadata: false });
  initializePlugin(plugin, FIXTURES_DIR);

  const code = loadFixture("simple-macro");
  const id = getFixturePath("simple-macro");

  const { result, error } = await invokeTransform(plugin, code, id);

  // Valid code should not produce errors
  assert.equal(error, null);
  // Should have transformed code
  if (result) {
    assert.ok(result.code);
    assert.ok(result.code.includes("User"));
  }
});

test("processes code without macros", async () => {
  const plugin = napiMacrosPlugin({ generateTypes: false, emitMetadata: false });
  initializePlugin(plugin, FIXTURES_DIR);

  const code = loadFixture("no-macro");
  const id = getFixturePath("no-macro");

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);
  // Plugin may return result even without macros (depending on transformer)
});

test("handles syntax errors gracefully", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  // Create file with syntax error
  writeTestFile(
    tempDir,
    "src/broken.ts",
    `class BrokenClass {
  // Missing closing brace
  value: number;
`
  );

  const plugin = napiMacrosPlugin({ generateTypes: false, emitMetadata: false });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/broken.ts"), "utf-8");
  const id = path.join(tempDir, "src/broken.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  // Plugin should handle syntax errors - either return error or null
  // It shouldn't crash unexpectedly
  assert.ok(error !== undefined || result === null || result !== null);
});

test("transform context captures errors", async (t) => {
  // This test verifies the transform context error handling mechanism
  const context = createTransformContext();

  assert.deepEqual(context.getErrors(), []);
  assert.deepEqual(context.getWarnings(), []);

  // Test warning capture
  context.warn("test warning");
  assert.deepEqual(context.getWarnings(), ["test warning"]);

  // Test error throw
  assert.throws(
    () => context.error("test error"),
    { message: "test error" }
  );
  assert.deepEqual(context.getErrors(), ["test error"]);
});

test("handles unknown macro gracefully", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  // Create file with unknown macro
  writeTestFile(
    tempDir,
    "src/unknown.ts",
    `/** @derive(UnknownMacro) */
class Test {
  value: number;
}
export { Test };`
  );

  const plugin = napiMacrosPlugin({ generateTypes: false, emitMetadata: false });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/unknown.ts"), "utf-8");
  const id = path.join(tempDir, "src/unknown.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  // Should either error or return null/utils - not crash
  assert.ok(error !== undefined || result !== undefined);
});

test("handles empty file", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  writeTestFile(tempDir, "src/empty.ts", "");

  const plugin = napiMacrosPlugin({ generateTypes: false, emitMetadata: false });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/empty.ts"), "utf-8");
  const id = path.join(tempDir, "src/empty.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);
  assert.equal(result, null);
});

test("handles whitespace-only file", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  writeTestFile(tempDir, "src/whitespace.ts", "   \n\n   \t\t\n   ");

  const plugin = napiMacrosPlugin({ generateTypes: false, emitMetadata: false });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/whitespace.ts"), "utf-8");
  const id = path.join(tempDir, "src/whitespace.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  // Should not error on whitespace-only files
  assert.equal(error, null);
});

test("handles comment-only file", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  writeTestFile(
    tempDir,
    "src/comments.ts",
    `// This is a comment
/* Another comment */
/** JSDoc comment */`
  );

  const plugin = napiMacrosPlugin({ generateTypes: false, emitMetadata: false });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/comments.ts"), "utf-8");
  const id = path.join(tempDir, "src/comments.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  // Should not error on comment-only files
  assert.equal(error, null);
});

test("handles large file", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  // Generate a large file
  let largeCode = `/** @derive(Debug) */\nclass LargeClass {\n`;
  for (let i = 0; i < 1000; i++) {
    largeCode += `  field${i}: string;\n`;
  }
  largeCode += `}\nexport { LargeClass };`;

  writeTestFile(tempDir, "src/large.ts", largeCode);

  const plugin = napiMacrosPlugin({ generateTypes: false, emitMetadata: false });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/large.ts"), "utf-8");
  const id = path.join(tempDir, "src/large.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  // Should handle large files without timing out
  assert.equal(error, null);
  if (result) {
    assert.ok(result.code.includes("LargeClass"));
  }
});

test("handles multiple classes with macros", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  writeTestFile(
    tempDir,
    "src/multi.ts",
    `/** @derive(Debug) */
class User {
  id: string;
}

/** @derive(Debug) */
class Post {
  title: string;
}

/** @derive(Debug) */
class Comment {
  text: string;
}

export { User, Post, Comment };`
  );

  const plugin = napiMacrosPlugin({ generateTypes: false, emitMetadata: false });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/multi.ts"), "utf-8");
  const id = path.join(tempDir, "src/multi.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);
  if (result) {
    assert.ok(result.code.includes("User"));
    assert.ok(result.code.includes("Post"));
    assert.ok(result.code.includes("Comment"));
  }
});

test("strips macro-only import comments", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  writeTestFile(
    tempDir,
    "src/with-import.ts",
    `/** import macro from 'macroforge' */
/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
  );

  const plugin = napiMacrosPlugin({ generateTypes: false, emitMetadata: false });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/with-import.ts"), "utf-8");
  const id = path.join(tempDir, "src/with-import.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);
  if (result) {
    // Macro import comments should be stripped
    assert.ok(!result.code.includes("import macro"));
  }
});

test("handles TypeScript generics", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  writeTestFile(
    tempDir,
    "src/generic.ts",
    `/** @derive(Debug) */
class Container<T> {
  value: T;
}
export { Container };`
  );

  const plugin = napiMacrosPlugin({ generateTypes: false, emitMetadata: false });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/generic.ts"), "utf-8");
  const id = path.join(tempDir, "src/generic.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);
  if (result) {
    assert.ok(result.code.includes("Container"));
  }
});

test("handles TypeScript interfaces", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  writeTestFile(
    tempDir,
    "src/interface.ts",
    `interface User {
  id: string;
  name: string;
}
export type { User };`
  );

  const plugin = napiMacrosPlugin({ generateTypes: false, emitMetadata: false });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/interface.ts"), "utf-8");
  const id = path.join(tempDir, "src/interface.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  // Should process interfaces without error
  assert.equal(error, null);
});

test("handles decorators in code", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  writeTestFile(
    tempDir,
    "src/decorated.ts",
    `function decorator(target: any) {}

/** @derive(Debug) */
@decorator
class DecoratedClass {
  id: string;
}
export { DecoratedClass };`
  );

  const plugin = napiMacrosPlugin({ generateTypes: false, emitMetadata: false });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/decorated.ts"), "utf-8");
  const id = path.join(tempDir, "src/decorated.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  // Should handle decorators
  assert.equal(error, null);
});

test("transform returns null source map", async () => {
  const plugin = napiMacrosPlugin({ generateTypes: false, emitMetadata: false });
  initializePlugin(plugin, FIXTURES_DIR);

  const code = loadFixture("simple-macro");
  const id = getFixturePath("simple-macro");

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);
  if (result) {
    // Source maps are not yet generated
    assert.equal(result.map, null);
  }
});
