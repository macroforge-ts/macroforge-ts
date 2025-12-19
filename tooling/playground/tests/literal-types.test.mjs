/**
 * Tests for literal type support in the Default macro.
 *
 * Verifies that string, number, and boolean literal types generate correct
 * default values (the literal itself) rather than invalid function calls.
 *
 * Bug fixed: Before this fix, `__brand: "Duration"` would incorrectly generate
 * `__brand: "Duration"DefaultValue()` instead of `__brand: "Duration"`.
 */

import { test, describe } from "node:test";
import assert from "node:assert/strict";
import path from "node:path";
import { createRequire } from "node:module";
import { repoRoot } from "./test-utils.mjs";

const require = createRequire(import.meta.url);
const swcMacrosPath = path.join(repoRoot, "crates/macroforge_ts/index.js");
const { expandSync } = require(swcMacrosPath);

// ============================================================================
// String Literal Type Tests
// ============================================================================

describe("Default macro with string literal types", () => {
  test("generates correct default for branded type with string literal", () => {
    const code = `
      /** @derive(Default) */
      interface Duration {
        millis: number;
        __brand: "Duration";
      }
    `;
    const result = expandSync(code, "test.ts");

    // Should use the literal string directly
    assert.ok(
      result.code.includes('__brand: "Duration"'),
      'Should generate __brand: "Duration" as the default value',
    );

    // Should NOT generate a function call
    assert.ok(
      !result.code.includes('"Duration"DefaultValue()'),
      'Should NOT generate "Duration"DefaultValue() function call',
    );

    // No errors
    const errors = result.diagnostics.filter((d) => d.level === "error");
    assert.equal(errors.length, 0, "Should have no errors");
  });

  test("generates correct default for single-quoted string literal", () => {
    const code = `
      /** @derive(Default) */
      interface Status {
        value: string;
        kind: 'active';
      }
    `;
    const result = expandSync(code, "test.ts");

    // Should use the literal string directly
    assert.ok(
      result.code.includes("kind: 'active'"),
      "Should generate kind: 'active' as the default value",
    );

    // Should NOT generate a function call
    assert.ok(
      !result.code.includes("'active'DefaultValue()"),
      "Should NOT generate 'active'DefaultValue() function call",
    );
  });

  test("generates correct default for multiple branded interfaces", () => {
    const code = `
      /** @derive(Default) */
      interface Duration {
        millis: number;
        __brand: "Duration";
      }

      /** @derive(Default) */
      interface Utc {
        epochMillis: number;
        __brand: "Utc";
      }

      /** @derive(Default) */
      interface Zoned {
        year: number;
        month: number;
        __brand: "Zoned";
      }
    `;
    const result = expandSync(code, "test.ts");

    // All three branded types should use literals correctly
    assert.ok(
      result.code.includes('__brand: "Duration"'),
      'Should generate __brand: "Duration"',
    );
    assert.ok(
      result.code.includes('__brand: "Utc"'),
      'Should generate __brand: "Utc"',
    );
    assert.ok(
      result.code.includes('__brand: "Zoned"'),
      'Should generate __brand: "Zoned"',
    );

    // None should generate invalid function calls
    assert.ok(
      !result.code.includes('"Duration"DefaultValue()'),
      'Should NOT generate "Duration"DefaultValue()',
    );
    assert.ok(
      !result.code.includes('"Utc"DefaultValue()'),
      'Should NOT generate "Utc"DefaultValue()',
    );
    assert.ok(
      !result.code.includes('"Zoned"DefaultValue()'),
      'Should NOT generate "Zoned"DefaultValue()',
    );
  });
});

// ============================================================================
// Number Literal Type Tests
// ============================================================================

describe("Default macro with number literal types", () => {
  test("generates correct default for integer literal type", () => {
    const code = `
      /** @derive(Default) */
      interface Config {
        name: string;
        version: 1;
      }
    `;
    const result = expandSync(code, "test.ts");

    // Should use the number literal directly
    assert.ok(
      result.code.includes("version: 1"),
      "Should generate version: 1 as the default value",
    );

    // Should NOT generate a function call
    assert.ok(
      !result.code.includes("1DefaultValue()"),
      "Should NOT generate 1DefaultValue() function call",
    );
  });

  test("generates correct default for decimal literal type", () => {
    const code = `
      /** @derive(Default) */
      interface Measurement {
        value: number;
        precision: 0.01;
      }
    `;
    const result = expandSync(code, "test.ts");

    // Should use the decimal literal directly
    assert.ok(
      result.code.includes("precision: 0.01"),
      "Should generate precision: 0.01 as the default value",
    );
  });

  test("generates correct default for negative number literal type", () => {
    const code = `
      /** @derive(Default) */
      interface Temperature {
        value: number;
        minKelvin: -273;
      }
    `;
    const result = expandSync(code, "test.ts");

    // Should use the negative number literal directly
    assert.ok(
      result.code.includes("minKelvin: -273"),
      "Should generate minKelvin: -273 as the default value",
    );
  });
});

// ============================================================================
// Boolean Literal Type Tests
// ============================================================================

describe("Default macro with boolean literal types", () => {
  test("generates correct default for true literal type", () => {
    const code = `
      /** @derive(Default) */
      interface Feature {
        name: string;
        enabled: true;
      }
    `;
    const result = expandSync(code, "test.ts");

    // Should use true directly
    assert.ok(
      result.code.includes("enabled: true"),
      "Should generate enabled: true as the default value",
    );

    // Should NOT generate a function call
    assert.ok(
      !result.code.includes("trueDefaultValue()"),
      "Should NOT generate trueDefaultValue() function call",
    );
  });

  test("generates correct default for false literal type", () => {
    const code = `
      /** @derive(Default) */
      interface Feature {
        name: string;
        deprecated: false;
      }
    `;
    const result = expandSync(code, "test.ts");

    // Should use false directly
    assert.ok(
      result.code.includes("deprecated: false"),
      "Should generate deprecated: false as the default value",
    );

    // Should NOT generate a function call
    assert.ok(
      !result.code.includes("falseDefaultValue()"),
      "Should NOT generate falseDefaultValue() function call",
    );
  });
});

// ============================================================================
// Mixed Literal Types Tests
// ============================================================================

describe("Default macro with mixed literal types", () => {
  test("generates correct defaults for interface with multiple literal types", () => {
    const code = `
      /** @derive(Default) */
      interface ComplexConfig {
        name: string;
        version: 2;
        status: "active";
        enabled: true;
        __brand: "Config";
      }
    `;
    const result = expandSync(code, "test.ts");

    // All literal types should be handled correctly
    assert.ok(
      result.code.includes("version: 2"),
      "Should generate version: 2",
    );
    assert.ok(
      result.code.includes('status: "active"'),
      'Should generate status: "active"',
    );
    assert.ok(
      result.code.includes("enabled: true"),
      "Should generate enabled: true",
    );
    assert.ok(
      result.code.includes('__brand: "Config"'),
      'Should generate __brand: "Config"',
    );

    // None should generate invalid function calls for literals
    assert.ok(
      !result.code.includes('2DefaultValue()'),
      "Should NOT generate 2DefaultValue()",
    );
    assert.ok(
      !result.code.includes('"active"DefaultValue()'),
      'Should NOT generate "active"DefaultValue()',
    );
    assert.ok(
      !result.code.includes("trueDefaultValue()"),
      "Should NOT generate trueDefaultValue()",
    );
    assert.ok(
      !result.code.includes('"Config"DefaultValue()'),
      'Should NOT generate "Config"DefaultValue()',
    );

    // Standard types should still use their defaults
    assert.ok(
      result.code.includes('name: ""'),
      'Should generate name: "" for string type',
    );
  });

  test("generates correct defaults for class with literal types", () => {
    const code = `
      /** @derive(Default) */
      class BrandedEntity {
        id: string;
        __brand: "Entity";
        version: 1;
      }
    `;
    const result = expandSync(code, "test.ts");

    // Literal types should be handled correctly in classes too
    assert.ok(
      result.code.includes('__brand: "Entity"'),
      'Should generate __brand: "Entity"',
    );
    assert.ok(
      result.code.includes("version: 1"),
      "Should generate version: 1",
    );
  });
});
