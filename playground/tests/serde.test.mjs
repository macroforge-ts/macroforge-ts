/**
 * Comprehensive tests for Serialize/Deserialize macros with cycle detection,
 * forward references, and polymorphic types.
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
// Serialize Macro Expansion Tests
// ============================================================================

describe("Serialize macro expansion", () => {
  test("generates serialize and serializeWithContext methods for classes", () => {
    const code = `
      /** @derive(Serialize) */
      class User {
        name: string;
        age: number;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("serialize():"),
      "Should generate serialize method",
    );
    assert.ok(
      result.code.includes("serializeWithContext("),
      "Should generate serializeWithContext method",
    );
    assert.ok(
      result.code.includes("SerializeContext"),
      "Should use SerializeContext",
    );
  });

  test("generates __type and __id in serialization output", () => {
    const code = `
      /** @derive(Serialize) */
      class Point {
        x: number;
        y: number;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes('__type: "Point"'),
      "Should include __type marker",
    );
    assert.ok(
      result.code.includes("__id"),
      "Should include __id for cycle detection",
    );
  });

  test("generates cycle detection with __ref", () => {
    const code = `
      /** @derive(Serialize) */
      class Node {
        value: number;
        next: Node | null;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("ctx.getId(this)"),
      "Should check for existing ID",
    );
    assert.ok(result.code.includes("__ref:"), "Should return __ref for cycles");
    assert.ok(
      result.code.includes("ctx.register(this)"),
      "Should register object",
    );
  });

  test("handles optional fields correctly", () => {
    const code = `
      /** @derive(Serialize) */
      class Config {
        name: string;
        description?: string;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("description") &&
        result.code.includes("!== undefined"),
      "Should check for undefined on optional fields",
    );
  });

  test("generates prefixed functions for interfaces", () => {
    const code = `
      /** @derive(Serialize) */
      interface IPoint {
        x: number;
        y: number;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("function iPointSerialize("),
      "Should generate iPointSerialize function",
    );
    assert.ok(
      result.code.includes("function iPointSerializeWithContext("),
      "Should generate iPointSerializeWithContext function",
    );
  });

  test("interfaces call nested prefixed serializeWithContext functions", () => {
    const code = `
      /** @derive(Serialize) */
      interface Metadata {
        createdAt: string;
      }

      /** @derive(Serialize) */
      interface User {
        metadata: Metadata | null;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("function userSerializeWithContext("),
      "Should generate userSerializeWithContext",
    );
    assert.ok(
      result.code.includes("metadataSerializeWithContext("),
      "Should call metadataSerializeWithContext for nested type",
    );
    assert.ok(
      !result.code.includes("Metadata.serializeWithContext("),
      "Should not use namespace-style Metadata.serializeWithContext",
    );
  });

  test("handles @serde(rename) decorator", () => {
    const code = `
      /** @derive(Serialize) */
      class ApiResponse {
        /** @serde({ rename: "user_id" }) */
        userId: number;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes('"user_id"'),
      "Should use renamed key in output",
    );
  });

  test("handles @serde(skip) decorator", () => {
    const code = `
      /** @derive(Serialize) */
      class Credentials {
        username: string;
        /** @serde({ skip: true }) */
        password: string;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes('"username"'),
      "Should include non-skipped fields",
    );
    assert.ok(
      !result.code.includes('"password"'),
      "Should skip fields with skip: true",
    );
  });

  test("handles @serde(flatten) decorator", () => {
    const code = `
      /** @derive(Serialize) */
      interface Metadata {
        createdAt: Date;
      }

      /** @derive(Serialize) */
      class Entity {
        id: string;
        /** @serde({ flatten: true }) */
        meta: Metadata;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("__flattened"),
      "Should flatten nested object",
    );
    assert.ok(
      result.code.includes("Object.assign"),
      "Should merge flattened fields",
    );
  });

  test("handles Date serialization to ISO string", () => {
    const code = `
      /** @derive(Serialize) */
      class Event {
        name: string;
        timestamp: Date;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("toISOString()"),
      "Should serialize Date as ISO string",
    );
  });

  test("handles Array serialization with nested objects", () => {
    const code = `
      /** @derive(Serialize) */
      class Container {
        items: Item[];
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(result.code.includes(".map("), "Should map over array items");
    assert.ok(
      result.code.includes("SerializeWithContext"),
      "Should call serializeWithContext on nested items",
    );
  });

  test("handles Map serialization", () => {
    const code = `
      /** @derive(Serialize) */
      class Dictionary {
        entries: Map<string, number>;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("Object.fromEntries"),
      "Should convert Map to object",
    );
    assert.ok(
      result.code.includes(".entries()"),
      "Should iterate over Map entries",
    );
  });

  test("handles Set serialization", () => {
    const code = `
      /** @derive(Serialize) */
      class Tags {
        values: Set<string>;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("Array.from"),
      "Should convert Set to array",
    );
  });
});

// ============================================================================
// Deserialize Macro Expansion Tests
// ============================================================================

describe("Deserialize macro expansion", () => {
  test("generates deserialize and deserializeWithContext methods for classes", () => {
    const code = `
      /** @derive(Deserialize) */
      class User {
        name: string;
        age: number;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("static deserialize("),
      "Should generate static deserialize",
    );
    assert.ok(
      result.code.includes("static deserializeWithContext("),
      "Should generate static deserializeWithContext",
    );
    assert.ok(
      result.code.includes("DeserializeContext"),
      "Should use DeserializeContext",
    );
  });

  test("handles __ref for cycle detection", () => {
    const code = `
      /** @derive(Deserialize) */
      class Node {
        value: number;
        next: Node | null;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("__ref"),
      "Should handle __ref for forward references",
    );
    assert.ok(
      result.code.includes("getOrDefer"),
      "Should use getOrDefer for cycle handling",
    );
  });

  test("validates required fields", () => {
    const code = `
      /** @derive(Deserialize) */
      class Required {
        name: string;
        value: number;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("missing required field"),
      "Should check for required fields",
    );
  });

  test("handles optional fields with defaults", () => {
    const code = `
      /** @derive(Deserialize) */
      class Config {
        name: string;
        /** @serde({ default: '"default_value"' }) */
        value?: string;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("default_value"),
      "Should use default value",
    );
  });

  test("handles Date deserialization from ISO string", () => {
    const code = `
      /** @derive(Deserialize) */
      class Event {
        timestamp: Date;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("new Date("),
      "Should parse Date from string",
    );
  });

  test("handles Array deserialization", () => {
    const code = `
      /** @derive(Deserialize) */
      class Container {
        items: Item[];
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("Array.isArray"),
      "Should check for array type",
    );
  });

  test("handles Map deserialization", () => {
    const code = `
      /** @derive(Deserialize) */
      class Dictionary {
        entries: Map<string, number>;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("new Map("),
      "Should construct Map from object",
    );
    assert.ok(
      result.code.includes("Object.entries"),
      "Should use Object.entries",
    );
  });

  test("handles Set deserialization", () => {
    const code = `
      /** @derive(Deserialize) */
      class Tags {
        values: Set<string>;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("new Set("),
      "Should construct Set from array",
    );
  });

  test("rejects classes with custom constructors", () => {
    const code = `
      /** @derive(Deserialize) */
      class Invalid {
        name: string;
        constructor(name: string) {
          this.name = name;
        }
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.diagnostics.length > 0,
      "Should report error for custom constructor",
    );
    assert.ok(
      result.diagnostics.some((d) => d.message.includes("constructor")),
      "Error should mention constructor",
    );
  });

  test("handles @serde(denyUnknownFields)", () => {
    const code = `
      /**
       * @derive(Deserialize)
       * @serde({ denyUnknownFields: true })
       */
      class Strict {
        name: string;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("unknown field"),
      "Should check for unknown fields",
    );
    assert.ok(result.code.includes("knownKeys"), "Should track known keys");
  });

  test("registers deserialized objects with context", () => {
    const code = `
      /** @derive(Deserialize) */
      class Entity {
        id: string;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("ctx.register("),
      "Should register with context",
    );
  });

  test("supports freeze option in deserialize", () => {
    const code = `
      /** @derive(Deserialize) */
      class Data {
        value: string;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("opts?.freeze"),
      "Should check freeze option",
    );
    assert.ok(result.code.includes("freezeAll"), "Should call freezeAll");
  });

  test("interfaces call nested prefixed deserializeWithContext functions", () => {
    const code = `
      /** @derive(Deserialize) */
      interface Metadata {
        createdAt: string;
      }

      /** @derive(Deserialize) */
      interface User {
        metadata: Metadata | null;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("function userDeserializeWithContext("),
      "Should generate userDeserializeWithContext",
    );
    assert.ok(
      result.code.includes("metadataDeserializeWithContext("),
      "Should call metadataDeserializeWithContextfor nested type",
    );
    assert.ok(
      !result.code.includes("Metadata.deserializeWithContext("),
      "Should not use namespace-style Metadata.deserializeWithContext",
    );
  });
});

describe("External type function imports", () => {
  test("injects imports for nested type functions (prefix style)", () => {
    const code = `
      import { Metadata } from "./metadata.svelte";

      /** @derive(Default, Serialize, Deserialize) */
      interface User {
        metadata: Metadata;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("metadataDefaultValue()"),
      "Should call metadataDefaultValue for nested default values",
    );
    assert.ok(
      !result.code.includes("Metadata.defaultValue()"),
      "Should not use namespace-style Metadata.defaultValue",
    );

    assert.ok(
      result.code.includes(
        'import { metadataSerializeWithContext } from "./metadata.svelte";',
      ),
      "Should import metadataSerializeWithContext from metadata module",
    );
    assert.ok(
      result.code.includes(
        'import { metadataDeserializeWithContext } from "./metadata.svelte";',
      ),
      "Should import metadataDeserializeWithContext from metadata module",
    );
    assert.ok(
      result.code.includes(
        'import { metadataDefaultValue } from "./metadata.svelte";',
      ),
      "Should import metadataDefaultValue from metadata module",
    );
  });
});

// ============================================================================
// Combined Serialize + Deserialize
// ============================================================================

describe("Combined Serialize + Deserialize", () => {
  test("generates both sets of methods when both derived", () => {
    const code = `
      /** @derive(Serialize, Deserialize) */
      class Entity {
        id: string;
        data: string;
      }
    `;
    const result = expandSync(code, "test.ts");

    // Serialize methods
    assert.ok(
      result.code.includes("serialize():"),
      "Should have serialize",
    );
    assert.ok(
      result.code.includes("serializeWithContext(ctx:"),
      "Should have serializeWithContext",
    );

    // Deserialize methods
    assert.ok(
      result.code.includes("static deserialize("),
      "Should have deserialize",
    );
    assert.ok(
      result.code.includes("static deserializeWithContext("),
      "Should have deserializeWithContext",
    );
  });

  test("handles nested serializable types", () => {
    const code = `
      /** @derive(Serialize, Deserialize) */
      class Address {
        street: string;
        city: string;
      }

      /** @derive(Serialize, Deserialize) */
      class Person {
        name: string;
        address: Address;
      }
    `;
    const result = expandSync(code, "test.ts");

    // Both types should have serde methods
    assert.ok(
      result.code.includes("class Address"),
      "Should have Address class",
    );
    assert.ok(result.code.includes("class Person"), "Should have Person class");

    // Person should call Address's serializeWithContext
    assert.ok(
      result.code.includes("SerializeWithContext") && result.code.includes("address"),
      "Should serialize nested address",
    );
  });
});

// ============================================================================
// Enum serialization
// ============================================================================

describe("Enum serialization", () => {
  test("generates prefixed functions for enum serialization", () => {
    const code = `
      /** @derive(Serialize) */
      enum Status {
        Active = "active",
        Inactive = "inactive"
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("function statusSerialize("),
      "Should have statusSerialize function",
    );
    assert.ok(
      result.code.includes("function statusSerializeWithContext("),
      "Should have statusSerializeWithContext function",
    );
  });

  test("generates prefixed functions for enum deserialization", () => {
    const code = `
      /** @derive(Deserialize) */
      enum Status {
        Active = "active",
        Inactive = "inactive"
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("function statusDeserialize("),
      "Should have statusDeserialize function",
    );
    assert.ok(
      result.code.includes("function statusDeserializeWithContext("),
      "Should have statusDeserializeWithContext function",
    );
    assert.ok(result.code.includes("Invalid"), "Should validate enum values");
  });
});

// ============================================================================
// Type alias serialization
// ============================================================================

describe("Type alias serialization", () => {
  test("generates prefixed functions for object type alias", () => {
    const code = `
      /** @derive(Serialize) */
      type Point = {
        x: number;
        y: number;
      };
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("function pointSerialize("),
      "Should have pointSerialize",
    );
    assert.ok(
      result.code.includes("function pointSerializeWithContext("),
      "Should have pointSerializeWithContext",
    );
  });

  test("generates prefixed functions for union type alias", () => {
    const code = `
      /** @derive(Serialize) */
      type Result = Success | Failure;
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("function resultSerialize("),
      "Should have resultSerialize",
    );
    assert.ok(
      result.code.includes("function resultSerializeWithContext("),
      "Should have resultSerializeWithContext",
    );
  });
});

// ============================================================================
// Import handling
// ============================================================================

describe("Import handling", () => {
  test("adds SerializeContext import for Serialize", () => {
    const code = `
      /** @derive(Serialize) */
      class User {
        name: string;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes(
        'import { SerializeContext } from "macroforge/serde"',
      ),
      "Should add SerializeContext import",
    );
  });

  test("adds DeserializeContext import for Deserialize", () => {
    const code = `
      /** @derive(Deserialize) */
      class User {
        name: string;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("import { DeserializeContext"),
      "Should add DeserializeContext import",
    );
    assert.ok(
      result.code.includes("PendingRef"),
      "Should add PendingRef import",
    );
  });
});

// ============================================================================
// Edge cases
// ============================================================================

describe("Edge cases", () => {
  test("empty class serialization", () => {
    const code = `
      /** @derive(Serialize, Deserialize) */
      class Empty {}
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("serialize()"),
      "Should generate serialize",
    );
    assert.ok(
      result.code.includes("deserialize"),
      "Should generate deserialize",
    );
    assert.ok(
      result.code.includes('__type: "Empty"'),
      "Should have type marker",
    );
  });

  test("nullable field handling", () => {
    const code = `
      /** @derive(Serialize, Deserialize) */
      class WithNull {
        value: string | null;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(result.code.includes("null"), "Should handle null explicitly");
  });

  test("self-referential type", () => {
    const code = `
      /** @derive(Serialize) */
      class TreeNode {
        value: number;
        left: TreeNode | null;
        right: TreeNode | null;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("__ref"),
      "Should handle self-reference with __ref",
    );
    assert.ok(result.code.includes("ctx.getId"), "Should check for cycles");
  });

  test("deeply nested types", () => {
    const code = `
      /** @derive(Serialize) */
      class Deep {
        data: Map<string, Set<number[]>>;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes("Object.fromEntries"),
      "Should handle Map serialization",
    );
    assert.ok(result.code.includes(".entries()"), "Should iterate Map entries");
  });
});

// ============================================================================
// renameAll container option
// ============================================================================

describe("renameAll container option", () => {
  test("camelCase rename", () => {
    const code = `
      /**
       * @derive(Serialize)
       * @serde({ renameAll: "camelCase" })
       */
      class User {
        user_name: string;
        created_at: Date;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes('"userName"'),
      "Should convert to camelCase",
    );
    assert.ok(
      result.code.includes('"createdAt"'),
      "Should convert to camelCase",
    );
  });

  test("snake_case rename", () => {
    const code = `
      /**
       * @derive(Serialize)
       * @serde({ renameAll: "snake_case" })
       */
      class User {
        userName: string;
        createdAt: Date;
      }
    `;
    const result = expandSync(code, "test.ts");

    assert.ok(
      result.code.includes('"user_name"'),
      "Should convert to snake_case",
    );
    assert.ok(
      result.code.includes('"created_at"'),
      "Should convert to snake_case",
    );
  });
});
