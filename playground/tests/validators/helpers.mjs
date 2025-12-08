import { test, describe, beforeEach, afterEach, before, after } from "node:test";
import assert from "node:assert/strict";
import { createRequire } from "node:module";
import path from "node:path";
import { fileURLToPath } from "node:url";
import fs from "node:fs";
import vm from "node:vm";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const playgroundRoot = path.resolve(__dirname, "..", "..");
const repoRoot = path.resolve(playgroundRoot, "..");
const vanillaRoot = path.join(playgroundRoot, "vanilla");

const require = createRequire(import.meta.url);
const swcMacrosPath = path.join(repoRoot, "crates/macroforge_ts/index.js");
const { expandSync } = require(swcMacrosPath);

/**
 * Simple Result implementation for testing
 */
class Result {
  constructor(ok, value) {
    this._ok = ok;
    this._value = value;
  }

  static ok(value) {
    return new Result(true, value);
  }

  static err(errors) {
    return new Result(false, errors);
  }

  isOk() {
    return this._ok;
  }

  isErr() {
    return !this._ok;
  }

  unwrap() {
    if (!this._ok) throw new Error("Called unwrap on Err: " + JSON.stringify(this._value));
    return this._value;
  }

  unwrapErr() {
    if (this._ok) throw new Error("Called unwrapErr on Ok");
    return this._value;
  }
}

// Cache for compiled modules
const moduleCache = new Map();

/**
 * Expand macros in a TypeScript file and compile it for runtime testing
 * @param {string} filePath - Absolute path to the TypeScript file
 * @returns {object} Module exports with fromJSON methods
 */
export function expandAndCompile(filePath) {
  if (moduleCache.has(filePath)) {
    return moduleCache.get(filePath);
  }

  const sourceCode = fs.readFileSync(filePath, "utf8");
  const result = expandSync(sourceCode, path.basename(filePath));

  if (result.diagnostics && result.diagnostics.length > 0) {
    const errors = result.diagnostics.filter(d => d.severity === "error");
    if (errors.length > 0) {
      throw new Error(`Macro expansion errors:\n${errors.map(e => e.message).join("\n")}`);
    }
  }

  // The expanded code is in result.code
  let expandedCode = result.code;

  // Replace the Result import with our mock
  expandedCode = expandedCode.replace(
    /import\s*\{\s*Result\s*\}\s*from\s*["']macroforge\/result["'];?/g,
    ""
  );

  // Strip TypeScript type annotations to make it valid JavaScript
  // Use a more comprehensive approach - strip all type annotations

  // Remove type annotations from variable declarations: const x: Type = ... or const x: Type[] = ...
  // This handles primitives, array types, and generic types
  expandedCode = expandedCode.replace(/:\s*(?:string|number|boolean|unknown|any|void|null|undefined|bigint|Date|never)(?:\[\])?(?=\s*[=;,)\]}])/g, "");
  expandedCode = expandedCode.replace(/:\s*(?:Record|Result|Map|Set|Array)<[^>]+>(?:\[\])?(?=\s*[=;,)\]}])/g, "");
  expandedCode = expandedCode.replace(/:\s*[A-Z]\w*(?:\[\])?(?=\s*[=;,)\]}])/g, "");

  // Remove type assertions: value as Type
  expandedCode = expandedCode.replace(/\s+as\s+(?:string|number|boolean|unknown|any|void|null|undefined|bigint|Date|never)(?:\[\])?/g, "");
  expandedCode = expandedCode.replace(/\s+as\s+(?:Record|Result|Map|Set|Array)<[^>]+>(?:\[\])?/g, "");
  expandedCode = expandedCode.replace(/\s+as\s+[A-Z]\w*(?:\[\])?/g, "");

  // Remove function return type annotations: function foo(): Type { or ): Type[] {
  expandedCode = expandedCode.replace(/\):\s*(?:string|number|boolean|unknown|any|void|null|undefined|bigint|Date|never)(?:\[\])?\s*\{/g, ") {");
  expandedCode = expandedCode.replace(/\):\s*(?:Record|Result|Map|Set|Array)<[^>]+>(?:\[\])?\s*\{/g, ") {");
  expandedCode = expandedCode.replace(/\):\s*[A-Z]\w*(?:\[\])?\s*\{/g, ") {");

  // Remove generic type parameters from classes: class Foo<T>
  expandedCode = expandedCode.replace(/class\s+(\w+)<[^>]+>/g, "class $1");

  // Remove constructor parameter types: constructor(init: { ... })
  expandedCode = expandedCode.replace(/constructor\s*\(\s*init\s*:\s*\{[^}]+\}\s*\)/g, "constructor(init)");

  // Remove any remaining type-only lines (interfaces)
  expandedCode = expandedCode.replace(/^\s*interface\s+\w+\s*\{[\s\S]*?\}\s*$/gm, "");

  // Create a module context
  const exports = {};
  const moduleContext = {
    Result,
    exports,
    console,
    Object,
    Array,
    String,
    Number,
    Boolean,
    Date,
    Map,
    Set,
    BigInt,
    JSON,
    Error,
    TypeError,
    isNaN,
    parseFloat,
    parseInt,
    URL,
    RegExp,
  };

  // Convert ESM exports to CommonJS-style for vm execution
  // Handle: export class X, export const X, export function X
  expandedCode = expandedCode.replace(/export\s+class\s+(\w+)/g, "class $1");
  expandedCode = expandedCode.replace(/export\s+const\s+(\w+)/g, "const $1");
  expandedCode = expandedCode.replace(/export\s+function\s+(\w+)/g, "function $1");
  expandedCode = expandedCode.replace(/export\s+\{[^}]+\}/g, "");

  // Find all actual class declarations (must be followed by { or extends, not in comments)
  // First, strip comments to avoid false matches
  const codeWithoutComments = expandedCode
    .replace(/\/\*[\s\S]*?\*\//g, '')  // Remove block comments
    .replace(/\/\/.*$/gm, '');          // Remove line comments

  const classMatches = codeWithoutComments.matchAll(/class\s+(\w+)\s*(?:\{|extends)/g);
  const classNames = [...classMatches].map(m => m[1]);

  // Add exports at the end
  const exportStatements = classNames.map(name => `exports.${name} = ${name};`).join("\n");
  expandedCode += "\n" + exportStatements;

  try {
    const script = new vm.Script(expandedCode, { filename: filePath });
    const context = vm.createContext(moduleContext);
    script.runInContext(context);

    moduleCache.set(filePath, exports);
    return exports;
  } catch (error) {
    // Find where the error might be
    const errorMatch = error.message.match(/(\w+) is not defined/);
    if (errorMatch) {
      const missingVar = errorMatch[1];
      const lines = expandedCode.split("\n");
      const lineIndex = lines.findIndex(l => l.includes(missingVar));
      if (lineIndex >= 0) {
        console.error(`Error: ${missingVar} is not defined at line ${lineIndex + 1}`);
        console.error("Context:");
        console.error(lines.slice(Math.max(0, lineIndex - 3), lineIndex + 4).join("\n"));
      }
    }
    console.error("Failed to compile expanded code:", error.message);
    console.error("Expanded code (first 3000 chars):\n", expandedCode.substring(0, 3000));
    throw error;
  }
}

/**
 * Load a validator test module
 * @param {string} moduleName - Name of the module in validators directory (e.g., "string-validator-tests")
 * @returns {object} Module exports
 */
export function loadValidatorModule(moduleName) {
  const filePath = path.join(vanillaRoot, "src/validators", `${moduleName}.ts`);
  return expandAndCompile(filePath);
}

/**
 * Assert that fromJSON returns an error Result with expected message substring
 * @param {object} result - Result from fromJSON
 * @param {string} fieldName - Field name for error context
 * @param {string} messageSubstring - Expected substring in error message
 */
export function assertValidationError(result, fieldName, messageSubstring) {
  assert.ok(
    result.isErr(),
    `Expected validation to fail for "${fieldName}", but it succeeded`
  );
  const errors = result.unwrapErr();
  const hasExpectedError = errors.some((e) => e.includes(messageSubstring));
  assert.ok(
    hasExpectedError,
    `Expected error containing "${messageSubstring}" for field "${fieldName}", got: ${errors.join("; ")}`
  );
}

/**
 * Assert that fromJSON returns a successful Result
 * @param {object} result - Result from fromJSON
 * @param {string} fieldName - Field name for error context
 */
export function assertValidationSuccess(result, fieldName) {
  if (result.isErr()) {
    const errors = result.unwrapErr();
    assert.fail(
      `Expected validation to succeed for "${fieldName}", but got errors: ${errors.join("; ")}`
    );
  }
  assert.ok(result.isOk(), `Expected validation to succeed for "${fieldName}"`);
}

/**
 * Assert that fromJSON returns specific error count
 * @param {object} result - Result from fromJSON
 * @param {number} expectedCount - Expected number of errors
 */
export function assertErrorCount(result, expectedCount) {
  assert.ok(result.isErr(), `Expected validation to fail`);
  const errors = result.unwrapErr();
  assert.equal(
    errors.length,
    expectedCount,
    `Expected ${expectedCount} errors, got ${errors.length}: ${errors.join("; ")}`
  );
}

export { test, describe, before, after, beforeEach, afterEach, assert };
