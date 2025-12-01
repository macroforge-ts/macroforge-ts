#!/usr/bin/env node
/**
 * Minimal test to reproduce the file-switching crash
 * This bypasses the TS plugin entirely and directly tests the native module
 */

const path = require("path");

console.log("=== Native Module Crash Test ===\n");

// Load the native module
console.log("1. Loading native module...");
let expand;
try {
  const nativeModule = require("@ts-macros/swc-napi");
  expand = nativeModule.expandSync;
  console.log("   ✓ Native module loaded\n");
} catch (e) {
  console.error("   ✗ Failed to load native module:", e.message);
  process.exit(1);
}

// Test file 1
const file1 = {
  path: "test1.ts",
  code: `
import { Derive } from "@macro/derive";

/** @derive(Debug) */
class User {
  name: string;
}
`,
};

// Test file 2
const file2 = {
  path: "test2.ts",
  code: `
import { Derive } from "@macro/derive";

/** @derive(Clone) */
class Product {
  id: number;
}
`,
};

// Test sequence that mimics file switching
console.log("2. Expanding first file (test1.ts with Debug)...");
try {
  const result1 = expand(file1.code, file1.path, { keepDecorators: true });
  console.log(`   ✓ First expansion succeeded: ${result1.code.length} chars`);
  console.log(`     Diagnostics: ${result1.diagnostics.length}`);
} catch (e) {
  console.error("   ✗ First expansion FAILED:", e.message);
  process.exit(1);
}

console.log("\n3. Expanding second file (test2.ts with Clone)...");
try {
  const result2 = expand(file2.code, file2.path, { keepDecorators: true });
  console.log(`   ✓ Second expansion succeeded: ${result2.code.length} chars`);
  console.log(`     Diagnostics: ${result2.diagnostics.length}`);
} catch (e) {
  console.error("   ✗ Second expansion FAILED:", e.message);
  console.error("   Stack:", e.stack);
  process.exit(1);
}

console.log("\n4. Expanding first file again (simulating tab switch back)...");
try {
  const result3 = expand(file1.code, file1.path, { keepDecorators: true });
  console.log(`   ✓ Third expansion succeeded: ${result3.code.length} chars`);
  console.log(`     Diagnostics: ${result3.diagnostics.length}`);
} catch (e) {
  console.error("   ✗ Third expansion FAILED:", e.message);
  console.error("   Stack:", e.stack);
  process.exit(1);
}

console.log("\n5. Rapid switching test (10 iterations)...");
for (let i = 0; i < 10; i++) {
  const file = i % 2 === 0 ? file1 : file2;
  try {
    const result = expand(file.code, file.path, { keepDecorators: true });
    console.log(`   Iteration ${i + 1}: ✓ (${file.path})`);
  } catch (e) {
    console.error(`   Iteration ${i + 1}: ✗ CRASHED on ${file.path}`);
    console.error("   Error:", e.message);
    process.exit(1);
  }
}

console.log("\n=== All tests passed! ===");
console.log("The native module itself is stable.");
console.log("The crash must be in the TypeScript plugin integration.");
