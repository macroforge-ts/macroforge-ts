// Early declarations using var for hoisting
var moduleCache = new Map();
var expandSync;
var loadConfig;
var configPath;

import { test, describe, beforeEach, afterEach, beforeAll, afterAll } from "bun:test";
import { expect } from "bun:test";
import path from "node:path";
import { fileURLToPath } from "node:url";
import fs from "node:fs";
import { createRequire } from "node:module";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const playgroundRoot = path.resolve(__dirname, "..", "..");
const repoRoot = path.resolve(playgroundRoot, "..");
const vanillaRoot = path.join(playgroundRoot, "vanilla");

// Use require for synchronous loading
const require = createRequire(import.meta.url);
const swcMacrosPath = path.join(repoRoot, "crates/macroforge_ts/index.js");

// Initialize after imports
const macroforge = require(swcMacrosPath);
expandSync = macroforge.expandSync;
loadConfig = macroforge.loadConfig;


// Load and cache the config for the vanilla playground
configPath = path.join(vanillaRoot, "macroforge.config.ts");
if (fs.existsSync(configPath)) {
  const configContent = fs.readFileSync(configPath, "utf8");
  loadConfig(configContent, configPath);
}

/**
 * Expand macros in a TypeScript file and compile it using Bun's transpiler
 * @param {string} filePath - Absolute path to the TypeScript file
 * @returns {object} Module exports
 */
export async function expandAndCompile(filePath) {
  if (moduleCache.has(filePath)) {
    return moduleCache.get(filePath);
  }

  const sourceCode = fs.readFileSync(filePath, "utf8");
  const result = expandSync(sourceCode, path.basename(filePath), { configPath });

  if (result.diagnostics && result.diagnostics.length > 0) {
    const errors = result.diagnostics.filter(d => d.severity === "error");
    if (errors.length > 0) {
      throw new Error(`Macro expansion errors:\n${errors.map(e => e.message).join("\n")}`);
    }
  }

  // Write expanded code to a temp file and import it
  const tempDir = path.join(__dirname, ".temp");
  if (!fs.existsSync(tempDir)) {
    fs.mkdirSync(tempDir, { recursive: true });
  }

  const tempFile = path.join(tempDir, path.basename(filePath));
  fs.writeFileSync(tempFile, result.code);

  try {
    // Bun natively handles TypeScript imports
    const mod = await import(tempFile);
    moduleCache.set(filePath, mod);
    return mod;
  } catch (error) {
    console.error("Failed to compile expanded code:", error.message);
    console.error("Expanded code written to:", tempFile);
    throw error;
  }
}

/**
 * Load a validator test module
 * @param {string} moduleName - Name of the module in validators directory (e.g., "string-validator-tests")
 * @returns {object} Module exports
 */
export async function loadValidatorModule(moduleName) {
  const filePath = path.join(vanillaRoot, "src/validators", `${moduleName}.ts`);
  return expandAndCompile(filePath);
}

/**
 * Assert that fromStringifiedJSON returns an error result with expected message substring
 * @param {object} result - Result from fromStringifiedJSON ({ success: boolean, errors?: Array })
 * @param {string} fieldName - Field name for error context
 * @param {string} messageSubstring - Expected substring in error message
 */
export function assertValidationError(result, fieldName, messageSubstring) {
  expect(result.success).toBe(false);
  const errors = result.errors;
  // Errors are structured as {field, message} objects
  const hasExpectedError = errors.some((e) => {
    const msg = typeof e === "string" ? e : e.message;
    return msg.includes(messageSubstring);
  });
  expect(hasExpectedError).toBe(true);
}

/**
 * Assert that fromStringifiedJSON returns a successful result
 * @param {object} result - Result from fromStringifiedJSON ({ success: boolean, value?: T, errors?: Array })
 * @param {string} fieldName - Field name for error context
 */
export function assertValidationSuccess(result, fieldName) {
  if (!result.success) {
    const errors = result.errors;
    // Errors are structured as {field, message} objects
    const errorMsgs = errors.map((e) => (typeof e === "string" ? e : `${e.field}: ${e.message}`));
    throw new Error(
      `Expected validation to succeed for "${fieldName}", but got errors: ${errorMsgs.join("; ")}`
    );
  }
  expect(result.success).toBe(true);
}

/**
 * Assert that fromStringifiedJSON returns specific error count
 * @param {object} result - Result from fromStringifiedJSON ({ success: boolean, errors?: Array })
 * @param {number} expectedCount - Expected number of errors
 */
export function assertErrorCount(result, expectedCount) {
  expect(result.success).toBe(false);
  const errors = result.errors;
  expect(errors.length).toBe(expectedCount);
}

// Re-export with node:test compatible names
export {
  test,
  describe,
  beforeAll as before,
  afterAll as after,
  beforeEach,
  afterEach,
  beforeAll,
  afterAll,
  expect
};
