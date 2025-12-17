/**
 * Tests for foreign types configuration in macroforge.config.js
 *
 * Foreign types allow global registration of handlers for external types
 * (like Effect's DateTime) so they work automatically in serialization/deserialization
 * without needing per-field decorators.
 */

import { test, describe } from "node:test";
import assert from "node:assert/strict";
import path from "node:path";
import { createRequire } from "node:module";
import { repoRoot } from "./test-utils.mjs";

const require = createRequire(import.meta.url);
const swcMacrosPath = path.join(repoRoot, "crates/macroforge_ts/index.js");
const { expandSync, loadConfig, clearConfigCache } = require(swcMacrosPath);

// ============================================================================
// Foreign Type Configuration Tests
// ============================================================================

describe("Foreign types configuration", () => {
  test("loadConfig parses foreign types from config content", () => {
    clearConfigCache();
    const configContent = `
      export default {
        foreignTypes: {
          "DateTime.DateTime": {
            from: ["effect"],
            serialize: (v) => v.toJSON(),
            deserialize: (raw) => DateTime.fromJSON(raw),
            default: () => DateTime.now()
          }
        }
      }
    `;

    const result = loadConfig(configContent, "macroforge.config.js");

    assert.equal(result.hasForeignTypes, true, "Should have foreign types");
    assert.equal(result.foreignTypeCount, 1, "Should have 1 foreign type");
  });

  test("loadConfig handles multiple foreign types", () => {
    clearConfigCache();
    const configContent = `
      export default {
        foreignTypes: {
          "DateTime.DateTime": {
            from: ["effect"],
            serialize: (v) => v.toJSON(),
            deserialize: (raw) => DateTime.fromJSON(raw)
          },
          "Duration.Duration": {
            from: ["effect"],
            serialize: (v) => v.toMillis(),
            deserialize: (raw) => Duration.millis(raw)
          }
        }
      }
    `;

    const result = loadConfig(configContent, "macroforge.config.js");

    assert.equal(result.hasForeignTypes, true, "Should have foreign types");
    assert.equal(result.foreignTypeCount, 2, "Should have 2 foreign types");
  });

  test("loadConfig handles config without foreign types", () => {
    clearConfigCache();
    const configContent = `
      export default {
        keepDecorators: false
      }
    `;

    const result = loadConfig(configContent, "macroforge.config.js");

    assert.equal(result.hasForeignTypes, false, "Should not have foreign types");
    assert.equal(result.foreignTypeCount, 0, "Should have 0 foreign types");
  });
});

// ============================================================================
// Foreign Type Expansion Tests - Default Macro
// ============================================================================

describe("Foreign types in Default macro", () => {
  const configContent = `
    export default {
      foreignTypes: {
        "DateTime.DateTime": {
          from: ["effect"],
          serialize: (v) => v.toJSON(),
          deserialize: (raw) => DateTime.fromJSON(raw),
          default: () => DateTime.unsafeNow()
        }
      }
    }
  `;
  // Use unique config path to avoid caching issues
  const configPath = "/test/default-macro/macroforge.config.js";

  test("default value uses foreign type default function", () => {
    // Clear cache and load config
    clearConfigCache();
    loadConfig(configContent, configPath);

    const code = `
      import type { DateTime } from 'effect';

      /** @derive(Default) */
      interface Event {
        name: string;
        startTime: DateTime.DateTime;
      }
    `;

    const result = expandSync(code, "test.ts", { configPath });

    // The default value should call the configured default function as an IIFE
    assert.ok(
      result.code.includes("DateTime.unsafeNow()"),
      `Default should use the foreign type default function. Got: ${result.code}`
    );
  });
});

// ============================================================================
// Foreign Type Expansion Tests - Serialize Macro
// ============================================================================

describe("Foreign types in Serialize macro", () => {
  const configContent = `
    export default {
      foreignTypes: {
        "DateTime.DateTime": {
          from: ["effect"],
          serialize: (v) => DateTime.formatIso(v),
          deserialize: (raw) => DateTime.unsafeFromDate(new Date(raw)),
          default: () => DateTime.unsafeNow()
        }
      }
    }
  `;
  // Use unique config path to avoid caching issues
  const configPath = "/test/serialize-macro/macroforge.config.js";

  test("serialize uses foreign type serialize function", () => {
    clearConfigCache();
    loadConfig(configContent, configPath);

    const code = `
      import type { DateTime } from 'effect';

      /** @derive(Serialize) */
      interface Event {
        name: string;
        startTime: DateTime.DateTime;
      }
    `;

    const result = expandSync(code, "test.ts", { configPath });

    // The serialize should use the configured serialize function
    assert.ok(
      result.code.includes("DateTime.formatIso"),
      `Serialize should use the foreign type serialize function. Got: ${result.code}`
    );
    // Should NOT generate a generic helper call like dateTime.DateTimeSerializeWithContext
    assert.ok(
      !result.code.includes("dateTime.DateTime"),
      `Should not generate generic helper namespace. Got: ${result.code}`
    );
  });
});

// ============================================================================
// Foreign Type Expansion Tests - Deserialize Macro
// ============================================================================

describe("Foreign types in Deserialize macro", () => {
  const configContent = `
    export default {
      foreignTypes: {
        "DateTime.DateTime": {
          from: ["effect"],
          serialize: (v) => DateTime.formatIso(v),
          deserialize: (raw) => DateTime.unsafeFromDate(new Date(raw)),
          default: () => DateTime.unsafeNow()
        }
      }
    }
  `;
  // Use unique config path to avoid caching issues
  const configPath = "/test/deserialize-macro/macroforge.config.js";

  test("deserialize uses foreign type deserialize function", () => {
    clearConfigCache();
    loadConfig(configContent, configPath);

    const code = `
      import type { DateTime } from 'effect';

      /** @derive(Deserialize) */
      interface Event {
        name: string;
        startTime: DateTime.DateTime;
      }
    `;

    const result = expandSync(code, "test.ts", { configPath });

    // The deserialize should use the configured deserialize function
    assert.ok(
      result.code.includes("DateTime.unsafeFromDate"),
      `Deserialize should use the foreign type deserialize function. Got: ${result.code}`
    );
    // Should NOT generate a generic helper call
    assert.ok(
      !result.code.includes("dateTime.DateTime"),
      `Should not generate generic helper namespace. Got: ${result.code}`
    );
  });
});

// ============================================================================
// Foreign Type Expansion Tests - Combined Macros
// ============================================================================

describe("Foreign types with combined macros", () => {
  const configContent = `
    export default {
      foreignTypes: {
        "DateTime.DateTime": {
          from: ["effect"],
          serialize: (v) => DateTime.formatIso(v),
          deserialize: (raw) => DateTime.unsafeFromDate(new Date(raw)),
          default: () => DateTime.unsafeNow()
        }
      }
    }
  `;
  // Use unique config path to avoid caching issues
  const configPath = "/test/combined-macros/macroforge.config.js";

  test("all macros use foreign type functions when combined", () => {
    clearConfigCache();
    loadConfig(configContent, configPath);

    const code = `
      import type { DateTime } from 'effect';

      /** @derive(Default, Serialize, Deserialize) */
      interface Event {
        name: string;
        startTime: DateTime.DateTime;
      }
    `;

    const result = expandSync(code, "test.ts", { configPath });

    // Check all three foreign type functions are used
    assert.ok(
      result.code.includes("DateTime.unsafeNow()"),
      `Default should use foreign type default. Got: ${result.code}`
    );
    assert.ok(
      result.code.includes("DateTime.formatIso"),
      `Serialize should use foreign type serialize. Got: ${result.code}`
    );
    assert.ok(
      result.code.includes("DateTime.unsafeFromDate"),
      `Deserialize should use foreign type deserialize. Got: ${result.code}`
    );
  });
});

// ============================================================================
// Foreign Type Import Matching Tests
// ============================================================================

describe("Foreign type import matching", () => {
  const configContent = `
    export default {
      foreignTypes: {
        "DateTime.DateTime": {
          from: ["effect", "@effect/schema"],
          serialize: (v) => DateTime.formatIso(v),
          deserialize: (raw) => DateTime.unsafeFromDate(new Date(raw)),
          default: () => DateTime.unsafeNow()
        }
      }
    }
  `;
  // Use unique config path to avoid caching issues
  const configPath = "/test/import-matching/macroforge.config.js";

  test("matches foreign type from effect import", () => {
    clearConfigCache();
    loadConfig(configContent, configPath);

    const code = `
      import type { DateTime } from 'effect';

      /** @derive(Default) */
      interface Event {
        startTime: DateTime.DateTime;
      }
    `;

    const result = expandSync(code, "test.ts", { configPath });

    assert.ok(
      result.code.includes("DateTime.unsafeNow()"),
      `Should match DateTime from effect import. Got: ${result.code}`
    );
  });

  test("matches foreign type from @effect/schema import", () => {
    clearConfigCache();
    loadConfig(configContent, configPath);

    const code = `
      import type { DateTime } from '@effect/schema';

      /** @derive(Default) */
      interface Event {
        startTime: DateTime.DateTime;
      }
    `;

    const result = expandSync(code, "test.ts", { configPath });

    assert.ok(
      result.code.includes("DateTime.unsafeNow()"),
      `Should match DateTime from @effect/schema import. Got: ${result.code}`
    );
  });

  test("ignores type from different library with same name", () => {
    clearConfigCache();
    loadConfig(configContent, configPath);

    const code = `
      import type { DateTime } from 'some-other-library';

      /** @derive(Serialize) */
      interface Event {
        name: string;
        startTime: DateTime.DateTime;
      }
    `;

    const result = expandSync(code, "test.ts", { configPath });

    // Should NOT use the foreign type handler - falls back to generic handling
    // We can't know if some-other-library's DateTime has the right methods
    assert.ok(
      !result.code.includes("DateTime.formatIso"),
      `Should NOT use foreign type serialize for different library. Got: ${result.code}`
    );
    // Should not have any errors - just ignore and let tsc catch issues downstream
    assert.ok(
      !result.diagnostics || !result.diagnostics.some(d => d.level === "error"),
      `Should not error for types from other libraries. Got: ${JSON.stringify(result.diagnostics)}`
    );
  });

  test("does not match local types with same name as foreign type", () => {
    clearConfigCache();
    loadConfig(configContent, configPath);

    // No import - this is a "local" type scenario
    const code = `
      /** @derive(Default) */
      interface Event {
        startTime: DateTime.DateTime;
      }
    `;

    const result = expandSync(code, "test.ts", { configPath });

    // Should NOT use the foreign type handler because we can't verify the import source
    // Falls back to generic handling
    assert.ok(
      !result.code.includes("DateTime.unsafeNow()"),
      `Should NOT match unimported type. Got: ${result.code}`
    );
    // Should not have any errors either
    assert.ok(
      !result.diagnostics || !result.diagnostics.some(d => d.level === "error"),
      `Should not have errors for unimported types. Got: ${JSON.stringify(result.diagnostics)}`
    );
  });
});

// ============================================================================
// Field Decorator Override Tests
// ============================================================================

describe("Field decorators override foreign type config", () => {
  const configContent = `
    export default {
      foreignTypes: {
        "DateTime.DateTime": {
          from: ["effect"],
          serialize: (v) => DateTime.formatIso(v),
          deserialize: (raw) => DateTime.unsafeFromDate(new Date(raw)),
          default: () => DateTime.unsafeNow()
        }
      }
    }
  `;
  const configPath = "/test/decorator-override/macroforge.config.js";

  test("serializeWith decorator overrides foreign type serialize", () => {
    clearConfigCache();
    loadConfig(configContent, configPath);

    const code = `
      import type { DateTime } from 'effect';
      import { serializeWith } from 'macroforge/serde';

      /** @derive(Serialize) */
      interface Event {
        name: string;
        @serializeWith((v) => v.toEpochMillis())
        startTime: DateTime.DateTime;
      }
    `;

    const result = expandSync(code, "test.ts", { configPath });

    // Should use the explicit serializeWith, NOT the foreign type config
    assert.ok(
      result.code.includes("toEpochMillis"),
      `Should use explicit serializeWith decorator. Got: ${result.code}`
    );
    assert.ok(
      !result.code.includes("DateTime.formatIso"),
      `Should NOT use foreign type serialize when serializeWith is specified. Got: ${result.code}`
    );
  });

  test("deserializeWith decorator overrides foreign type deserialize", () => {
    clearConfigCache();
    loadConfig(configContent, configPath);

    const code = `
      import type { DateTime } from 'effect';
      import { deserializeWith } from 'macroforge/serde';

      /** @derive(Deserialize) */
      interface Event {
        name: string;
        @deserializeWith((raw) => DateTime.fromEpochMillis(raw))
        startTime: DateTime.DateTime;
      }
    `;

    const result = expandSync(code, "test.ts", { configPath });

    // Should use the explicit deserializeWith, NOT the foreign type config
    assert.ok(
      result.code.includes("fromEpochMillis"),
      `Should use explicit deserializeWith decorator. Got: ${result.code}`
    );
    assert.ok(
      !result.code.includes("DateTime.unsafeFromDate"),
      `Should NOT use foreign type deserialize when deserializeWith is specified. Got: ${result.code}`
    );
  });

  test("default decorator overrides foreign type default", () => {
    clearConfigCache();
    loadConfig(configContent, configPath);

    const code = `
      import type { DateTime } from 'effect';
      import { default as defaultValue } from 'macroforge/serde';

      /** @derive(Default) */
      interface Event {
        name: string;
        @defaultValue(() => DateTime.make(2024, 1, 1))
        startTime: DateTime.DateTime;
      }
    `;

    const result = expandSync(code, "test.ts", { configPath });

    // Should use the explicit default, NOT the foreign type config
    assert.ok(
      result.code.includes("DateTime.make(2024, 1, 1)"),
      `Should use explicit default decorator. Got: ${result.code}`
    );
    assert.ok(
      !result.code.includes("DateTime.unsafeNow"),
      `Should NOT use foreign type default when default decorator is specified. Got: ${result.code}`
    );
  });
});

// ============================================================================
// Foreign Type Alias Tests
// ============================================================================

describe("Foreign type aliases", () => {
  const configContent = `
    export default {
      foreignTypes: {
        "DateTime.DateTime": {
          from: ["effect"],
          aliases: [
            { name: "DateTime", from: "effect/DateTime" },
            { name: "MyDateTime", from: "my-effect-wrapper" }
          ],
          serialize: (v) => DateTime.formatIso(v),
          deserialize: (raw) => DateTime.unsafeFromDate(new Date(raw)),
          default: () => DateTime.unsafeNow()
        }
      }
    }
  `;
  const configPath = "/test/aliases/macroforge.config.js";

  test("loadConfig parses aliases from config", () => {
    clearConfigCache();
    const result = loadConfig(configContent, configPath);

    assert.equal(result.hasForeignTypes, true, "Should have foreign types");
    assert.equal(result.foreignTypeCount, 1, "Should have 1 foreign type");
  });

  test("matches foreign type via alias name and source", () => {
    clearConfigCache();
    loadConfig(configContent, configPath);

    const code = `
      import type { DateTime } from 'effect/DateTime';

      /** @derive(Serialize) */
      interface Event {
        name: string;
        startTime: DateTime;
      }
    `;

    const result = expandSync(code, "test.ts", { configPath });

    // Should use the foreign type serialize function via alias match
    assert.ok(
      result.code.includes("DateTime.formatIso"),
      `Should match via alias and use foreign type serialize. Got: ${result.code}`
    );
  });

  test("matches foreign type via different alias", () => {
    clearConfigCache();
    loadConfig(configContent, configPath);

    const code = `
      import type { MyDateTime } from 'my-effect-wrapper';

      /** @derive(Default) */
      interface Event {
        name: string;
        startTime: MyDateTime;
      }
    `;

    const result = expandSync(code, "test.ts", { configPath });

    // Should use the foreign type default function via MyDateTime alias
    assert.ok(
      result.code.includes("DateTime.unsafeNow()"),
      `Should match MyDateTime alias and use foreign type default. Got: ${result.code}`
    );
  });

  test("alias does not match when import source differs", () => {
    clearConfigCache();
    loadConfig(configContent, configPath);

    const code = `
      import type { DateTime } from 'some-other-library';

      /** @derive(Serialize) */
      interface Event {
        name: string;
        startTime: DateTime;
      }
    `;

    const result = expandSync(code, "test.ts", { configPath });

    // Should NOT use foreign type - import source doesn't match main or aliases
    assert.ok(
      !result.code.includes("DateTime.formatIso"),
      `Should NOT match when import source doesn't match any alias. Got: ${result.code}`
    );
  });

  test("primary from still works alongside aliases", () => {
    clearConfigCache();
    loadConfig(configContent, configPath);

    const code = `
      import type { DateTime } from 'effect';

      /** @derive(Serialize) */
      interface Event {
        name: string;
        startTime: DateTime.DateTime;
      }
    `;

    const result = expandSync(code, "test.ts", { configPath });

    // Should still work with the primary from: ["effect"]
    assert.ok(
      result.code.includes("DateTime.formatIso"),
      `Primary from should still work. Got: ${result.code}`
    );
  });
});
