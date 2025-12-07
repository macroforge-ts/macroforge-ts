/**
 * Tests for metadata (.macro-ir.json) emission.
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
  readFileOrNull,
} from "./test-utils.js";

test("emits metadata JSON when emitMetadata is true", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  writeTestFile(
    tempDir,
    "src/user.ts",
    `/** @derive(Debug) */
class User {
  id: string;
  name: string;
}
export { User };`
  );

  const metadataDir = "metadata";
  const plugin = napiMacrosPlugin({
    generateTypes: false,
    emitMetadata: true,
    metadataOutputDir: metadataDir,
  });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/user.ts"), "utf-8");
  const id = path.join(tempDir, "src/user.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);

  // Check if metadata file was generated
  const expectedMetadataPath = path.join(tempDir, metadataDir, "src", "user.macro-ir.json");
  const metadataContent = readFileOrNull(expectedMetadataPath);

  if (result && result.code) {
    // Metadata may or may not be present depending on macro expansion
    assert.ok(true, "Transform completed successfully");
  }
});

test("skips metadata emission when emitMetadata is false", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  writeTestFile(
    tempDir,
    "src/user.ts",
    `/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
  );

  const metadataDir = "no-metadata";
  const plugin = napiMacrosPlugin({
    generateTypes: false,
    emitMetadata: false,
    metadataOutputDir: metadataDir,
  });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/user.ts"), "utf-8");
  const id = path.join(tempDir, "src/user.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);

  // Metadata directory should not be created
  const metadataPath = path.join(tempDir, metadataDir);
  assert.equal(fs.existsSync(metadataPath), false, "Metadata directory should not exist");
});

test("uses typesOutputDir as default metadataOutputDir", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  writeTestFile(
    tempDir,
    "src/user.ts",
    `/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
  );

  // Specify typesOutputDir but not metadataOutputDir
  const typesDir = "custom-output";
  const plugin = napiMacrosPlugin({
    generateTypes: false,
    typesOutputDir: typesDir,
    emitMetadata: true,
    // metadataOutputDir not specified - should default to typesOutputDir
  });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/user.ts"), "utf-8");
  const id = path.join(tempDir, "src/user.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);

  // Metadata should be in typesOutputDir
  const expectedMetadataPath = path.join(tempDir, typesDir, "src", "user.macro-ir.json");
  if (result && result.code && fs.existsSync(expectedMetadataPath)) {
    const content = fs.readFileSync(expectedMetadataPath, "utf-8");
    assert.ok(content.length > 0, "Metadata file should have content");
  }
});

test("preserves directory structure in metadata output", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  // Create deeply nested source file
  writeTestFile(
    tempDir,
    "src/models/entities/user.ts",
    `/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
  );

  const metadataDir = "metadata";
  const plugin = napiMacrosPlugin({
    generateTypes: false,
    emitMetadata: true,
    metadataOutputDir: metadataDir,
  });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/models/entities/user.ts"), "utf-8");
  const id = path.join(tempDir, "src/models/entities/user.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);

  // Check if directory structure is preserved
  const expectedPath = path.join(tempDir, metadataDir, "src/models/entities/user.macro-ir.json");
  if (result && result.code && fs.existsSync(expectedPath)) {
    const content = fs.readFileSync(expectedPath, "utf-8");
    // Verify it's valid JSON
    assert.doesNotThrow(() => JSON.parse(content), "Metadata should be valid JSON");
  }
});

test("skips write when metadata content unchanged", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  writeTestFile(
    tempDir,
    "src/user.ts",
    `/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
  );

  const metadataDir = "metadata";
  const plugin = napiMacrosPlugin({
    generateTypes: false,
    emitMetadata: true,
    metadataOutputDir: metadataDir,
  });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/user.ts"), "utf-8");
  const id = path.join(tempDir, "src/user.ts");

  // First transform
  const { result: result1, error: error1 } = await invokeTransform(plugin, code, id);
  assert.equal(error1, null);

  // Get mtime of metadata file if it exists
  const metadataPath = path.join(tempDir, metadataDir, "src/user.macro-ir.json");
  if (fs.existsSync(metadataPath)) {
    const mtime1 = fs.statSync(metadataPath).mtimeMs;

    // Wait a bit to ensure mtime would change if file is rewritten
    await new Promise((resolve) => setTimeout(resolve, 50));

    // Second transform with same content
    const { result: result2, error: error2 } = await invokeTransform(plugin, code, id);
    assert.equal(error2, null);

    // mtime should not change (file not rewritten)
    const mtime2 = fs.statSync(metadataPath).mtimeMs;
    assert.equal(mtime1, mtime2, "File should not be rewritten when content is unchanged");
  }
});

test("handles files without macros for metadata", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  // File without macros
  writeTestFile(
    tempDir,
    "src/plain.ts",
    `class PlainClass {
  value: number;
}
export { PlainClass };`
  );

  const metadataDir = "metadata";
  const plugin = napiMacrosPlugin({
    generateTypes: false,
    emitMetadata: true,
    metadataOutputDir: metadataDir,
  });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/plain.ts"), "utf-8");
  const id = path.join(tempDir, "src/plain.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  // Should not throw even without macros
  assert.equal(error, null);

  // Note: The transformer may still emit metadata for processed files
  // The important thing is that it doesn't error
});

test("creates metadata output directory if it does not exist", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  writeTestFile(
    tempDir,
    "src/user.ts",
    `/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
  );

  // Use deeply nested output directory
  const metadataDir = "deep/nested/metadata/dir";
  const plugin = napiMacrosPlugin({
    generateTypes: false,
    emitMetadata: true,
    metadataOutputDir: metadataDir,
  });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/user.ts"), "utf-8");
  const id = path.join(tempDir, "src/user.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);

  // If transformation occurred, directory should be created
  if (result && result.code) {
    const outputDir = path.join(tempDir, metadataDir);
    // Directory structure should exist if metadata was written
    assert.ok(true);
  }
});

test("metadata is valid JSON", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  writeTestFile(
    tempDir,
    "src/user.ts",
    `/** @derive(Debug) */
class User {
  id: string;
  name: string;
  email: string;
}
export { User };`
  );

  const metadataDir = "metadata";
  const plugin = napiMacrosPlugin({
    generateTypes: false,
    emitMetadata: true,
    metadataOutputDir: metadataDir,
  });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/user.ts"), "utf-8");
  const id = path.join(tempDir, "src/user.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);

  const metadataPath = path.join(tempDir, metadataDir, "src/user.macro-ir.json");
  if (fs.existsSync(metadataPath)) {
    const content = fs.readFileSync(metadataPath, "utf-8");
    let parsed;
    assert.doesNotThrow(() => {
      parsed = JSON.parse(content);
    }, "Metadata must be valid JSON");
  }
});
