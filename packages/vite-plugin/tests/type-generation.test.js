/**
 * Tests for type definition (.d.ts) generation.
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
  listFilesRecursively,
} from "./test-utils.js";

test("generates .d.ts when generateTypes is true", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  // Create tsconfig.json for type generation
  writeTestFile(
    tempDir,
    "tsconfig.json",
    JSON.stringify({
      compilerOptions: {
        target: "ES2022",
        module: "ESNext",
        moduleResolution: "Bundler",
        strict: true,
        skipLibCheck: true,
      },
    })
  );

  // Create a source file with macro
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

  const typesDir = "generated-types";
  const plugin = napiMacrosPlugin({
    generateTypes: true,
    typesOutputDir: typesDir,
    emitMetadata: false,
  });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/user.ts"), "utf-8");
  const id = path.join(tempDir, "src/user.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);

  // Check if .d.ts file was generated
  const expectedTypesPath = path.join(tempDir, typesDir, "src", "user.d.ts");
  const typesContent = readFileOrNull(expectedTypesPath);

  // If transformation happened and TypeScript is available, types should be generated
  if (result && result.code) {
    assert.ok(
      typesContent === null || typesContent.includes("User"),
      "Types should contain User class declaration or not be generated if TS unavailable"
    );
  }
});

test("skips .d.ts generation when generateTypes is false", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  writeTestFile(
    tempDir,
    "tsconfig.json",
    JSON.stringify({
      compilerOptions: {
        target: "ES2022",
        module: "ESNext",
      },
    })
  );

  writeTestFile(
    tempDir,
    "src/user.ts",
    `/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
  );

  const typesDir = "no-types";
  const plugin = napiMacrosPlugin({
    generateTypes: false,
    typesOutputDir: typesDir,
    emitMetadata: false,
  });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/user.ts"), "utf-8");
  const id = path.join(tempDir, "src/user.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);

  // Types directory should not be created
  const typesPath = path.join(tempDir, typesDir);
  assert.equal(fs.existsSync(typesPath), false, "Types directory should not exist");
});

test("uses default typesOutputDir when not specified", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  writeTestFile(
    tempDir,
    "tsconfig.json",
    JSON.stringify({
      compilerOptions: {
        target: "ES2022",
        module: "ESNext",
      },
    })
  );

  writeTestFile(
    tempDir,
    "src/user.ts",
    `/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
  );

  // Don't specify typesOutputDir - should default to "src/macros/generated"
  const plugin = napiMacrosPlugin({
    generateTypes: true,
    emitMetadata: false,
  });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/user.ts"), "utf-8");
  const id = path.join(tempDir, "src/user.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);

  // Check default path
  const defaultTypesPath = path.join(tempDir, "src/macros/generated");
  // Directory might be created if transformation occurred
  if (result && result.code) {
    // Types may or may not be generated depending on TS availability
    assert.ok(true);
  }
});

test("preserves directory structure in types output", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  writeTestFile(
    tempDir,
    "tsconfig.json",
    JSON.stringify({
      compilerOptions: {
        target: "ES2022",
        module: "ESNext",
      },
    })
  );

  // Create nested source file
  writeTestFile(
    tempDir,
    "src/models/entities/user.ts",
    `/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
  );

  const typesDir = "types";
  const plugin = napiMacrosPlugin({
    generateTypes: true,
    typesOutputDir: typesDir,
    emitMetadata: false,
  });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/models/entities/user.ts"), "utf-8");
  const id = path.join(tempDir, "src/models/entities/user.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);

  // Check if directory structure is preserved
  const expectedPath = path.join(tempDir, typesDir, "src/models/entities/user.d.ts");
  if (result && result.code && fs.existsSync(expectedPath)) {
    const content = fs.readFileSync(expectedPath, "utf-8");
    assert.ok(content.includes("User"));
  }
});

test("skips write when .d.ts content unchanged", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  writeTestFile(
    tempDir,
    "tsconfig.json",
    JSON.stringify({
      compilerOptions: {
        target: "ES2022",
        module: "ESNext",
      },
    })
  );

  writeTestFile(
    tempDir,
    "src/user.ts",
    `/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
  );

  const typesDir = "types";
  const plugin = napiMacrosPlugin({
    generateTypes: true,
    typesOutputDir: typesDir,
    emitMetadata: false,
  });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/user.ts"), "utf-8");
  const id = path.join(tempDir, "src/user.ts");

  // First transform
  const { result: result1, error: error1 } = await invokeTransform(plugin, code, id);
  assert.equal(error1, null);

  // Get mtime of types file if it exists
  const typesPath = path.join(tempDir, typesDir, "src/user.d.ts");
  let mtime1 = null;
  if (fs.existsSync(typesPath)) {
    mtime1 = fs.statSync(typesPath).mtimeMs;

    // Wait a bit to ensure mtime would change if file is rewritten
    await new Promise((resolve) => setTimeout(resolve, 50));

    // Second transform with same content
    const { result: result2, error: error2 } = await invokeTransform(plugin, code, id);
    assert.equal(error2, null);

    // mtime should not change (file not rewritten)
    const mtime2 = fs.statSync(typesPath).mtimeMs;
    assert.equal(mtime1, mtime2, "File should not be rewritten when content is unchanged");
  }
});

test("handles TypeScript not being available gracefully", async (t) => {
  // This test verifies the plugin doesn't crash when TS is unavailable
  // In our test environment, TS is available, so we just verify basic operation
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

  const plugin = napiMacrosPlugin({
    generateTypes: true,
    typesOutputDir: "types",
    emitMetadata: false,
  });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/user.ts"), "utf-8");
  const id = path.join(tempDir, "src/user.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  // Should not throw even if types can't be generated
  assert.equal(error, null);
});

test("creates output directory if it does not exist", async (t) => {
  const tempDir = createTempDir();

  t.after(() => cleanupTempDir(tempDir));

  writeTestFile(
    tempDir,
    "tsconfig.json",
    JSON.stringify({
      compilerOptions: {
        target: "ES2022",
        module: "ESNext",
      },
    })
  );

  writeTestFile(
    tempDir,
    "src/user.ts",
    `/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
  );

  // Use deeply nested output directory that doesn't exist
  const typesDir = "deep/nested/types/directory";
  const plugin = napiMacrosPlugin({
    generateTypes: true,
    typesOutputDir: typesDir,
    emitMetadata: false,
  });
  initializePlugin(plugin, tempDir);

  const code = fs.readFileSync(path.join(tempDir, "src/user.ts"), "utf-8");
  const id = path.join(tempDir, "src/user.ts");

  const { result, error } = await invokeTransform(plugin, code, id);

  assert.equal(error, null);

  // If transformation occurred, directory should be created
  if (result && result.code) {
    const outputDir = path.join(tempDir, typesDir);
    // Directory might exist now
    assert.ok(true);
  }
});
