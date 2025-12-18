/**
 * Test utilities for vite-plugin tests.
 * Provides mock factories, fixture loading, and plugin invocation helpers.
 */

import fs from "fs";
import path from "path";
import os from "os";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export const TESTS_DIR = __dirname;
export const FIXTURES_DIR = path.join(TESTS_DIR, "fixtures");

/**
 * Create a mock resolved Vite config.
 * @param {string} root - Project root path
 * @returns {object} Mock Vite resolved config
 */
export function createMockResolvedConfig(root = process.cwd()) {
  return {
    root,
    command: "build",
    mode: "production",
  };
}

/**
 * Create a mock transform context that tracks errors and warnings.
 * @returns {object} Mock context with error/warn methods and getters
 */
export function createTransformContext() {
  const errors = [];
  const warnings = [];

  return {
    error(message) {
      errors.push(message);
      throw new Error(message);
    },
    warn(message) {
      warnings.push(message);
    },
    getErrors() {
      return errors;
    },
    getWarnings() {
      return warnings;
    },
  };
}

/**
 * Load a fixture file.
 * @param {string} fixtureName - Name of the fixture directory
 * @param {string} fileName - File name within the fixture directory (default: 'input.ts')
 * @returns {string} File contents
 */
export function loadFixture(fixtureName, fileName = "input.ts") {
  const fixturePath = path.join(FIXTURES_DIR, fixtureName, fileName);
  return fs.readFileSync(fixturePath, "utf-8");
}

/**
 * Get the path to a fixture file.
 * @param {string} fixtureName - Name of the fixture directory
 * @param {string} fileName - File name within the fixture directory
 * @returns {string} Absolute path to the fixture file
 */
export function getFixturePath(fixtureName, fileName = "input.ts") {
  return path.join(FIXTURES_DIR, fixtureName, fileName);
}

/**
 * Create a temporary directory for test output.
 * @param {string} prefix - Prefix for the temp directory name
 * @returns {string} Path to the created temp directory
 */
export function createTempDir(prefix = "vite-plugin-test-") {
  return fs.mkdtempSync(path.join(os.tmpdir(), prefix));
}

/**
 * Clean up a temporary directory.
 * @param {string} dir - Path to the directory to remove
 */
export function cleanupTempDir(dir) {
  if (dir && fs.existsSync(dir)) {
    fs.rmSync(dir, { recursive: true, force: true });
  }
}

/**
 * Initialize a plugin by calling its configResolved hook.
 * @param {object} plugin - The Vite plugin object
 * @param {string} root - Project root path
 */
export function initializePlugin(plugin, root = process.cwd()) {
  const config = createMockResolvedConfig(root);
  if (plugin.configResolved) {
    plugin.configResolved(config);
  }
}

/**
 * Create and initialize a plugin from an async factory.
 * @param {() => Promise<object>} pluginFactory - Async plugin factory function
 * @param {string} root - Project root path
 * @returns {Promise<object>} Initialized plugin
 */
export async function createAndInitializePlugin(pluginFactory, root = process.cwd()) {
  const plugin = await pluginFactory();
  initializePlugin(plugin, root);
  return plugin;
}

/**
 * Invoke the plugin's transform function.
 * @param {object} plugin - The Vite plugin object
 * @param {string} code - Source code to transform
 * @param {string} id - File path/id
 * @param {object} context - Optional custom context (defaults to createTransformContext())
 * @returns {Promise<{result: any, error: Error|null, context: object}>} Transform result and context
 */
export async function invokeTransform(plugin, code, id, context = null) {
  const ctx = context || createTransformContext();

  if (!plugin.transform) {
    throw new Error("Plugin does not have a transform function");
  }

  let result = null;
  let error = null;

  try {
    result = await plugin.transform.call(ctx, code, id);
  } catch (e) {
    error = e;
  }

  return { result, error, context: ctx };
}

/**
 * Write a file to a directory, creating parent dirs as needed.
 * @param {string} baseDir - Base directory
 * @param {string} relativePath - Relative path within base directory
 * @param {string} content - File content
 */
export function writeTestFile(baseDir, relativePath, content) {
  const fullPath = path.join(baseDir, relativePath);
  const dir = path.dirname(fullPath);
  fs.mkdirSync(dir, { recursive: true });
  fs.writeFileSync(fullPath, content, "utf-8");
}

/**
 * Read a file if it exists, otherwise return null.
 * @param {string} filePath - Path to the file
 * @returns {string|null} File contents or null
 */
export function readFileOrNull(filePath) {
  if (fs.existsSync(filePath)) {
    return fs.readFileSync(filePath, "utf-8");
  }
  return null;
}

/**
 * List all files in a directory recursively.
 * @param {string} dir - Directory to list
 * @param {string} prefix - Prefix for relative paths
 * @returns {string[]} Array of relative file paths
 */
export function listFilesRecursively(dir, prefix = "") {
  if (!fs.existsSync(dir)) {
    return [];
  }

  const files = [];
  const entries = fs.readdirSync(dir, { withFileTypes: true });

  for (const entry of entries) {
    const relativePath = path.join(prefix, entry.name);
    if (entry.isDirectory()) {
      files.push(...listFilesRecursively(path.join(dir, entry.name), relativePath));
    } else {
      files.push(relativePath);
    }
  }

  return files;
}

/**
 * Copy a directory recursively.
 * @param {string} src - Source directory
 * @param {string} dest - Destination directory
 */
export function copyDirRecursively(src, dest) {
  fs.mkdirSync(dest, { recursive: true });
  const entries = fs.readdirSync(src, { withFileTypes: true });

  for (const entry of entries) {
    const srcPath = path.join(src, entry.name);
    const destPath = path.join(dest, entry.name);

    if (entry.isDirectory()) {
      copyDirRecursively(srcPath, destPath);
    } else {
      fs.copyFileSync(srcPath, destPath);
    }
  }
}
