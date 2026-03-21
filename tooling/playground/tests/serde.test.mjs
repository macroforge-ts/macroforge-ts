/**
 * Comprehensive tests for Serialize/Deserialize macros with cycle detection,
 * forward references, and polymorphic types.
 */

import assert from 'node:assert/strict';
import { createRequire } from 'node:module';
import path from 'node:path';
import { describe, test } from 'node:test';
import { repoRoot } from './test-utils.mjs';

const require = createRequire(import.meta.url);
const swcMacrosPath = path.join(repoRoot, 'crates/macroforge_ts/index.js');
const { expandSync } = require(swcMacrosPath);

// ============================================================================
// Serialize Macro Expansion Tests
// ============================================================================

describe('Serialize macro expansion', () => {
    test('generates serialize and serializeWithContext methods for classes', () => {
        const code = `
      /** @derive(Serialize) */
      class User {
        name: string;
        age: number;
      }
    `;
        const result = expandSync(code, 'test.ts');

        // New format uses static methods
        assert.ok(
            result.code.includes(
                'static serialize(value: User, keepMetadata?: boolean)'
            ),
            'Should generate static serialize method'
        );
        assert.ok(
            result.code.includes('static serializeWithContext(value: User, ctx'),
            'Should generate static serializeWithContext method'
        );
        assert.ok(
            result.code.includes('SerializeContext'),
            'Should use SerializeContext'
        );
    });

    test('generates __type and __id in serialization output', () => {
        const code = `
      /** @derive(Serialize) */
      class Point {
        x: number;
        y: number;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('"__type": "Point"'),
            'Should include __type marker'
        );
        assert.ok(
            result.code.includes('__id'),
            'Should include __id for cycle detection'
        );
    });

    test('generates cycle detection with __ref', () => {
        const code = `
      /** @derive(Serialize) */
      class Node {
        value: number;
        next: Node | null;
      }
    `;
        const result = expandSync(code, 'test.ts');

        // New format uses 'value' parameter instead of 'this'
        assert.ok(
            result.code.includes('ctx.getId(value)'),
            'Should check for existing ID'
        );
        assert.ok(result.code.includes('__ref:'), 'Should return __ref for cycles');
        assert.ok(
            result.code.includes('ctx.register(value)'),
            'Should register object'
        );
    });

    test('handles optional fields correctly', () => {
        const code = `
      /** @derive(Serialize) */
      class Config {
        name: string;
        description?: string;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('description') &&
                result.code.includes('!== undefined'),
            'Should check for undefined on optional fields'
        );
    });

    test('generates prefixed functions for interfaces', () => {
        const code = `
      /** @derive(Serialize) */
      interface IPoint {
        x: number;
        y: number;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('function iPointSerialize('),
            'Should generate iPointSerialize function'
        );
        assert.ok(
            result.code.includes('function iPointSerializeWithContext('),
            'Should generate iPointSerializeWithContext function'
        );
    });

    test('interfaces call nested prefixed serializeWithContext functions', () => {
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
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('function userSerializeWithContext('),
            'Should generate userSerializeWithContext'
        );
        assert.ok(
            result.code.includes('metadataSerializeWithContext('),
            'Should call metadataSerializeWithContext for nested type'
        );
        assert.ok(
            !result.code.includes('Metadata.serializeWithContext('),
            'Should not use namespace-style Metadata.serializeWithContext'
        );
    });

    test('handles @serde(rename) decorator', () => {
        const code = `
      /** @derive(Serialize) */
      class ApiResponse {
        /** @serde({ rename: "user_id" }) */
        userId: number;
      }
    `;
        const result = expandSync(code, 'test.ts');

        // Direct property access: result.user_id instead of result["user_id"]
        assert.ok(
            result.code.includes('result.user_id'),
            'Should use renamed key in output'
        );
    });

    test('handles @serde(skip) decorator', () => {
        const code = `
      /** @derive(Serialize) */
      class Credentials {
        username: string;
        /** @serde({ skip: true }) */
        password: string;
      }
    `;
        const result = expandSync(code, 'test.ts');

        // Direct property access: result.username instead of result["username"]
        assert.ok(
            result.code.includes('result.username'),
            'Should include non-skipped fields'
        );
        assert.ok(
            !result.code.includes('result.password'),
            'Should skip fields with skip: true'
        );
    });

    test('handles @serde(flatten) decorator', () => {
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
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('__flattened'),
            'Should flatten nested object'
        );
        assert.ok(
            result.code.includes('Object.assign'),
            'Should merge flattened fields'
        );
    });

    test('handles Date serialization to ISO string', () => {
        const code = `
      /** @derive(Serialize) */
      class Event {
        name: string;
        timestamp: Date;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('toISOString()'),
            'Should serialize Date as ISO string'
        );
    });

    test('handles Array serialization with nested objects', () => {
        const code = `
      /** @derive(Serialize) */
      class Container {
        items: Item[];
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(result.code.includes('.map('), 'Should map over array items');
        assert.ok(
            result.code.includes('SerializeWithContext'),
            'Should call serializeWithContext on nested items'
        );
    });

    test('handles Map serialization', () => {
        const code = `
      /** @derive(Serialize) */
      class Dictionary {
        entries: Map<string, number>;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('Object.fromEntries'),
            'Should convert Map to object'
        );
        assert.ok(
            result.code.includes('.entries()'),
            'Should iterate over Map entries'
        );
    });

    test('handles Set serialization', () => {
        const code = `
      /** @derive(Serialize) */
      class Tags {
        values: Set<string>;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('Array.from'),
            'Should convert Set to array'
        );
    });
});

// ============================================================================
// Deserialize Macro Expansion Tests
// ============================================================================

describe('Deserialize macro expansion', () => {
    test('generates deserialize and deserializeWithContext methods for classes', () => {
        const code = `
      /** @derive(Deserialize) */
      class User {
        name: string;
        age: number;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('static deserialize('),
            'Should generate static deserialize'
        );
        assert.ok(
            result.code.includes('static deserializeWithContext('),
            'Should generate static deserializeWithContext'
        );
        assert.ok(
            result.code.includes('DeserializeContext'),
            'Should use DeserializeContext'
        );
    });

    test('handles __ref for cycle detection', () => {
        const code = `
      /** @derive(Deserialize) */
      class Node {
        value: number;
        next: Node | null;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('__ref'),
            'Should handle __ref for forward references'
        );
        assert.ok(
            result.code.includes('getOrDefer'),
            'Should use getOrDefer for cycle handling'
        );
    });

    test('validates required fields', () => {
        const code = `
      /** @derive(Deserialize) */
      class Required {
        name: string;
        value: number;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('missing required field'),
            'Should check for required fields'
        );
    });

    test('handles optional fields with defaults', () => {
        const code = `
      /** @derive(Deserialize) */
      class Config {
        name: string;
        /** @serde({ default: '"default_value"' }) */
        value?: string;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('default_value'),
            'Should use default value'
        );
    });

    test('handles Date deserialization from ISO string', () => {
        const code = `
      /** @derive(Deserialize) */
      class Event {
        timestamp: Date;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('new Date('),
            'Should parse Date from string'
        );
    });

    test('handles Array deserialization', () => {
        const code = `
      /** @derive(Deserialize) */
      class Container {
        items: Item[];
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('Array.isArray'),
            'Should check for array type'
        );
    });

    test('handles Map deserialization', () => {
        const code = `
      /** @derive(Deserialize) */
      class Dictionary {
        entries: Map<string, number>;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('new Map('),
            'Should construct Map from object'
        );
        assert.ok(
            result.code.includes('Object.entries'),
            'Should use Object.entries'
        );
    });

    test('handles Set deserialization', () => {
        const code = `
      /** @derive(Deserialize) */
      class Tags {
        values: Set<string>;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('new Set('),
            'Should construct Set from array'
        );
    });

    test('rejects classes with custom constructors', () => {
        const code = `
      /** @derive(Deserialize) */
      class Invalid {
        name: string;
        constructor(name: string) {
          this.name = name;
        }
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.diagnostics.length > 0,
            'Should report error for custom constructor'
        );
        assert.ok(
            result.diagnostics.some((d) => d.message.includes('constructor')),
            'Error should mention constructor'
        );
    });

    test('handles @serde(denyUnknownFields)', () => {
        const code = `
      /**
       * @derive(Deserialize)
       * @serde({ denyUnknownFields: true })
       */
      class Strict {
        name: string;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('unknown field'),
            'Should check for unknown fields'
        );
        assert.ok(result.code.includes('knownKeys'), 'Should track known keys');
    });

    test('registers deserialized objects with context', () => {
        const code = `
      /** @derive(Deserialize) */
      class Entity {
        id: string;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('ctx.register('),
            'Should register with context'
        );
    });

    test('supports freeze option in deserialize', () => {
        const code = `
      /** @derive(Deserialize) */
      class Data {
        value: string;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('opts?.freeze'),
            'Should check freeze option'
        );
        assert.ok(result.code.includes('freezeAll'), 'Should call freezeAll');
    });

    test('interfaces call nested prefixed deserializeWithContext functions', () => {
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
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('function userDeserializeWithContext('),
            'Should generate userDeserializeWithContext'
        );
        assert.ok(
            result.code.includes('metadataDeserializeWithContext('),
            'Should call metadataDeserializeWithContextfor nested type'
        );
        assert.ok(
            !result.code.includes('Metadata.deserializeWithContext('),
            'Should not use namespace-style Metadata.deserializeWithContext'
        );
    });
});

describe('External type function imports', () => {
    test('injects imports for nested type functions (prefix style)', () => {
        const code = `
      import { Metadata } from "./metadata.svelte";

      /** @derive(Default, Serialize, Deserialize) */
      interface User {
        metadata: Metadata;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('metadataDefaultValue()'),
            'Should call metadataDefaultValue for nested default values'
        );
        assert.ok(
            !result.code.includes('Metadata.defaultValue()'),
            'Should not use namespace-style Metadata.defaultValue'
        );

        assert.ok(
            result.code.includes(
                'import { metadataSerializeWithContext } from "./metadata.svelte";'
            ),
            'Should import metadataSerializeWithContext from metadata module'
        );
        assert.ok(
            result.code.includes(
                'import { metadataDeserializeWithContext } from "./metadata.svelte";'
            ),
            'Should import metadataDeserializeWithContext from metadata module'
        );
        assert.ok(
            result.code.includes(
                'import { metadataDefaultValue } from "./metadata.svelte";'
            ),
            'Should import metadataDefaultValue from metadata module'
        );
    });
});

// ============================================================================
// Combined Serialize + Deserialize
// ============================================================================

describe('Combined Serialize + Deserialize', () => {
    test('generates both sets of methods when both derived', () => {
        const code = `
      /** @derive(Serialize, Deserialize) */
      class Entity {
        id: string;
        data: string;
      }
    `;
        const result = expandSync(code, 'test.ts');

        // Serialize methods (now static)
        assert.ok(
            result.code.includes(
                'static serialize(value: Entity, keepMetadata?: boolean)'
            ),
            'Should have static serialize'
        );
        assert.ok(
            result.code.includes('static serializeWithContext(value: Entity, ctx'),
            'Should have static serializeWithContext'
        );

        // Deserialize methods
        assert.ok(
            result.code.includes('static deserialize('),
            'Should have deserialize'
        );
        assert.ok(
            result.code.includes('static deserializeWithContext('),
            'Should have deserializeWithContext'
        );
    });

    test('handles nested serializable types', () => {
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
        const result = expandSync(code, 'test.ts');

        // Both types should have serde methods
        assert.ok(
            result.code.includes('class Address'),
            'Should have Address class'
        );
        assert.ok(result.code.includes('class Person'), 'Should have Person class');

        // Person should call Address's serializeWithContext
        assert.ok(
            result.code.includes('SerializeWithContext') &&
                result.code.includes('address'),
            'Should serialize nested address'
        );
    });
});

// ============================================================================
// Enum serialization
// ============================================================================

describe('Enum serialization', () => {
    test('generates prefixed functions for enum serialization', () => {
        const code = `
      /** @derive(Serialize) */
      enum Status {
        Active = "active",
        Inactive = "inactive"
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('function statusSerialize('),
            'Should have statusSerialize function'
        );
        assert.ok(
            result.code.includes('function statusSerializeWithContext('),
            'Should have statusSerializeWithContext function'
        );
    });

    test('generates prefixed functions for enum deserialization', () => {
        const code = `
      /** @derive(Deserialize) */
      enum Status {
        Active = "active",
        Inactive = "inactive"
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('function statusDeserialize('),
            'Should have statusDeserialize function'
        );
        assert.ok(
            result.code.includes('function statusDeserializeWithContext('),
            'Should have statusDeserializeWithContext function'
        );
        assert.ok(result.code.includes('Invalid'), 'Should validate enum values');
    });
});

// ============================================================================
// Type alias serialization
// ============================================================================

describe('Type alias serialization', () => {
    test('generates prefixed functions for object type alias', () => {
        const code = `
      /** @derive(Serialize) */
      type Point = {
        x: number;
        y: number;
      };
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('function pointSerialize('),
            'Should have pointSerialize'
        );
        assert.ok(
            result.code.includes('function pointSerializeWithContext('),
            'Should have pointSerializeWithContext'
        );
    });

    test('generates prefixed functions for union type alias', () => {
        const code = `
      /** @derive(Serialize) */
      type Result = Success | Failure;
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('function resultSerialize('),
            'Should have resultSerialize'
        );
        assert.ok(
            result.code.includes('function resultSerializeWithContext('),
            'Should have resultSerializeWithContext'
        );
    });
});

// ============================================================================
// Import handling
// ============================================================================

describe('Import handling', () => {
    test('adds SerializeContext import for Serialize', () => {
        const code = `
      /** @derive(Serialize) */
      class User {
        name: string;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes(
                'import { SerializeContext as __mf_SerializeContext } from "macroforge/serde"'
            ),
            'Should add SerializeContext import with __mf_ alias'
        );
    });

    test('adds DeserializeContext import for Deserialize', () => {
        const code = `
      /** @derive(Deserialize) */
      class User {
        name: string;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('import { DeserializeContext'),
            'Should add DeserializeContext import'
        );
        assert.ok(
            result.code.includes('PendingRef'),
            'Should add PendingRef import'
        );
    });
});

// ============================================================================
// Edge cases
// ============================================================================

describe('Edge cases', () => {
    test('empty class serialization', () => {
        const code = `
      /** @derive(Serialize, Deserialize) */
      class Empty {}
    `;
        const result = expandSync(code, 'test.ts');

        // New format uses static methods
        assert.ok(
            result.code.includes(
                'static serialize(value: Empty, keepMetadata?: boolean)'
            ),
            'Should generate static serialize'
        );
        assert.ok(
            result.code.includes('static deserialize'),
            'Should generate static deserialize'
        );
        assert.ok(
            result.code.includes('"__type": "Empty"'),
            'Should have type marker'
        );
    });

    test('nullable field handling', () => {
        const code = `
      /** @derive(Serialize, Deserialize) */
      class WithNull {
        value: string | null;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(result.code.includes('null'), 'Should handle null explicitly');
    });

    test('self-referential type', () => {
        const code = `
      /** @derive(Serialize) */
      class TreeNode {
        value: number;
        left: TreeNode | null;
        right: TreeNode | null;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('__ref'),
            'Should handle self-reference with __ref'
        );
        assert.ok(result.code.includes('ctx.getId'), 'Should check for cycles');
    });

    test('deeply nested types', () => {
        const code = `
      /** @derive(Serialize) */
      class Deep {
        data: Map<string, Set<number[]>>;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('Object.fromEntries'),
            'Should handle Map serialization'
        );
        assert.ok(result.code.includes('.entries()'), 'Should iterate Map entries');
    });
});

// ============================================================================
// renameAll container option
// ============================================================================

describe('renameAll container option', () => {
    test('camelCase rename', () => {
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
        const result = expandSync(code, 'test.ts');

        // Direct property access: result.userName instead of result["userName"]
        assert.ok(
            result.code.includes('result.userName'),
            'Should convert to camelCase'
        );
        assert.ok(
            result.code.includes('result.createdAt'),
            'Should convert to camelCase'
        );
    });

    test('snake_case rename', () => {
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
        const result = expandSync(code, 'test.ts');

        // Direct property access: result.user_name instead of result["user_name"]
        assert.ok(
            result.code.includes('result.user_name'),
            'Should convert to snake_case'
        );
        assert.ok(
            result.code.includes('result.created_at'),
            'Should convert to snake_case'
        );
    });
});

// ============================================================================
// Recursive collection deserialization
// ============================================================================

describe('Recursive collection deserialization', () => {
    test('Array<Serializable> recursively deserializes elements in interfaces', () => {
        const code = `
      /** @derive(Deserialize) */
      interface Activity {
        action: string;
        timestamp: Date;
      }

      /** @derive(Deserialize) */
      interface Account {
        name: string;
        activity: Activity[];
      }
    `;
        const result = expandSync(code, 'test.ts');

        // Should call activityDeserialize (result-returning) for each array element
        assert.ok(
            result.code.includes('activityDeserialize(item)'),
            'Should recursively deserialize Activity elements via result-returning function'
        );
        // Should NOT have a raw `as Activity[]` cast in the Account deserializer
        // (the Activity[] field should go through .map with deserialization)
        assert.ok(
            result.code.includes('__arr'),
            'Should use __arr intermediate for PendingRef support'
        );
        // Activity.timestamp should be parsed as Date
        assert.ok(
            result.code.includes(
                'typeof __raw_timestamp === "string" ? new Date(__raw_timestamp)'
            ),
            'Activity.timestamp should be deserialized from ISO string to Date'
        );
    });

    test('Array<Serializable> recursively deserializes elements in classes', () => {
        const code = `
      /** @derive(Deserialize) */
      interface Tag {
        label: string;
        createdAt: Date;
      }

      /** @derive(Deserialize) */
      class Post {
        title: string;
        tags: Tag[];
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('tagDeserialize(item)'),
            'Class template should call tagDeserialize for Tag[] elements'
        );
    });

    test('Array<Serializable> recursively deserializes elements in type aliases', () => {
        const code = `
      /** @derive(Deserialize) */
      interface Item {
        name: string;
        price: number;
      }

      /** @derive(Deserialize) */
      type Cart = {
        items: Item[];
      };
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('itemDeserialize(item)'),
            'Type alias template should call itemDeserialize for Item[] elements'
        );
    });

    test('Array<Date> maps ISO strings to Date objects', () => {
        const code = `
      /** @derive(Deserialize) */
      interface Timeline {
        dates: Date[];
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes(
                'typeof item === "string" ? new Date(item) : item as Date'
            ),
            'Should map Date[] elements from ISO strings'
        );
    });

    test('Set<Serializable> recursively deserializes elements', () => {
        const code = `
      /** @derive(Deserialize) */
      interface Permission {
        resource: string;
        level: number;
      }

      /** @derive(Deserialize) */
      interface Role {
        permissions: Set<Permission>;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('permissionDeserialize(item)'),
            'Should call permissionDeserialize for Set<Permission> elements'
        );
        assert.ok(
            result.code.includes('new Set('),
            'Should wrap result in new Set()'
        );
    });

    test('Map<string, Serializable> recursively deserializes values', () => {
        const code = `
      /** @derive(Deserialize) */
      interface Score {
        value: number;
        date: Date;
      }

      /** @derive(Deserialize) */
      interface Leaderboard {
        scores: Map<string, Score>;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('scoreDeserialize(v)'),
            'Should call scoreDeserialize for Map values'
        );
        assert.ok(
            result.code.includes('new Map('),
            'Should wrap result in new Map()'
        );
    });

    test('deeply nested: Array<T> where T has Array<U> with Date fields', () => {
        const code = `
      /** @derive(Deserialize) */
      interface Metric {
        name: string;
        recordedAt: Date;
      }

      /** @derive(Deserialize) */
      interface Report {
        title: string;
        metrics: Metric[];
      }

      /** @derive(Deserialize) */
      interface Dashboard {
        reports: Report[];
        generatedAt: Date;
      }
    `;
        const result = expandSync(code, 'test.ts');

        // Dashboard.reports should recursively deserialize Report
        assert.ok(
            result.code.includes('reportDeserialize(item)'),
            'Dashboard should recursively deserialize Report[] elements'
        );
        // Report.metrics should recursively deserialize Metric
        assert.ok(
            result.code.includes('metricDeserialize(item)'),
            'Report should recursively deserialize Metric[] elements'
        );
        // Metric.recordedAt should parse Date
        assert.ok(
            result.code.includes(
                'typeof __raw_recordedAt === "string" ? new Date(__raw_recordedAt)'
            ),
            'Metric.recordedAt should be parsed from ISO string'
        );
        // Dashboard.generatedAt should parse Date
        assert.ok(
            result.code.includes(
                'typeof __raw_generatedAt === "string" ? new Date(__raw_generatedAt)'
            ),
            'Dashboard.generatedAt should be parsed from ISO string'
        );
    });

    test('mixed collection types: Array, Set, Map with serializable values', () => {
        const code = `
      /** @derive(Deserialize) */
      interface Address {
        street: string;
        city: string;
      }

      /** @derive(Deserialize) */
      interface Company {
        locations: Address[];
        branches: Set<Address>;
        directoryByCity: Map<string, Address>;
      }
    `;
        const result = expandSync(code, 'test.ts');

        // Array<Address>
        assert.ok(
            result.code.includes('addressDeserialize(item)'),
            'Array<Address> should recursively deserialize'
        );
        // Set<Address>
        assert.ok(
            result.code.includes('new Set(') &&
                result.code.includes('addressDeserialize(item)'),
            'Set<Address> should recursively deserialize into Set'
        );
        // Map<string, Address>
        assert.ok(
            result.code.includes('addressDeserialize(v)'),
            'Map<string, Address> should recursively deserialize values'
        );
    });

    test('primitive collections are NOT recursively deserialized', () => {
        const code = `
      /** @derive(Deserialize) */
      interface Simple {
        names: string[];
        ids: Set<number>;
        labels: Map<string, string>;
      }
    `;
        const result = expandSync(code, 'test.ts');

        // string[] should be a direct cast, no DeserializeWithContext
        assert.ok(
            result.code.includes('as string[]'),
            'string[] should use direct cast'
        );
        assert.ok(
            !result.code.includes('DeserializeWithContext(item'),
            'Primitive arrays should NOT call DeserializeWithContext on elements'
        );
    });
});

// ============================================================================
// Built-in type serde tests (bigint, URL, RegExp, TypedArrays, etc.)
// ============================================================================

describe('Built-in type serde', () => {
    test('bigint field uses String/BigInt conversion', () => {
        const code = `
      /** @derive(Serialize, Deserialize) */
      interface Counter { count: bigint; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('String(v)'),
            'Should serialize bigint with String(v)'
        );
        assert.ok(
            result.code.includes('BigInt(v as string)'),
            'Should deserialize bigint with BigInt()'
        );
    });

    test('URL field uses toString/new URL conversion', () => {
        const code = `
      /** @derive(Serialize, Deserialize) */
      interface Endpoint { url: URL; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('v.toString()'),
            'Should serialize URL with toString()'
        );
        assert.ok(
            result.code.includes('new URL(v as string)'),
            'Should deserialize URL with new URL()'
        );
    });

    test('RegExp field uses source/flags serialization', () => {
        const code = `
      /** @derive(Serialize, Deserialize) */
      interface Rule { pattern: RegExp; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('v.source') && result.code.includes('v.flags'),
            'Should serialize RegExp with source and flags'
        );
        assert.ok(
            result.code.includes('new RegExp('),
            'Should deserialize RegExp with new RegExp()'
        );
    });

    test('Uint8Array field uses Array.from/new Uint8Array', () => {
        const code = `
      /** @derive(Serialize, Deserialize) */
      interface Buffer { data: Uint8Array; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('Array.from(v)'),
            'Should serialize Uint8Array with Array.from()'
        );
        assert.ok(
            result.code.includes('new Uint8Array(v as number[])'),
            'Should deserialize Uint8Array with new Uint8Array()'
        );
    });

    test('URL[] composite uses map with toString/new URL', () => {
        const code = `
      /** @derive(Serialize, Deserialize) */
      interface Links { urls: URL[]; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('.map(') && result.code.includes('v.toString()'),
            'Should serialize URL[] by mapping toString()'
        );
        assert.ok(
            result.code.includes('.map(') &&
                result.code.includes('new URL(v as string)'),
            'Should deserialize URL[] by mapping new URL()'
        );
    });

    test('Set<URL> composite uses Array.from + map / new Set + map', () => {
        const code = `
      /** @derive(Serialize, Deserialize) */
      interface BookmarkSet { urls: Set<URL>; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('Array.from(s)') &&
                result.code.includes('v.toString()'),
            'Should serialize Set<URL> with Array.from and toString()'
        );
        assert.ok(
            result.code.includes('new Set(') &&
                result.code.includes('new URL(v as string)'),
            'Should deserialize Set<URL> with new Set and new URL()'
        );
    });

    test('Map<string, URL> composite uses entries + map', () => {
        const code = `
      /** @derive(Serialize, Deserialize) */
      interface UrlMap { endpoints: Map<string, URL>; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('m.entries()') &&
                result.code.includes('v.toString()'),
            'Should serialize Map<string, URL> by mapping entries'
        );
        assert.ok(
            result.code.includes('new Map(') &&
                result.code.includes('new URL(v as string)'),
            'Should deserialize Map<string, URL> with new Map and new URL()'
        );
    });

    test('bigint | null nullable uses null check + String/BigInt', () => {
        const code = `
      /** @derive(Serialize, Deserialize) */
      interface OptCounter { count: bigint | null; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('=== null ? null') &&
                result.code.includes('String(v)'),
            'Should serialize bigint | null with null check and String()'
        );
        assert.ok(
            result.code.includes('=== null ? null') &&
                result.code.includes('BigInt(v as string)'),
            'Should deserialize bigint | null with null check and BigInt()'
        );
    });

    test('Error field uses name/message/stack serialization', () => {
        const code = `
      /** @derive(Serialize, Deserialize) */
      interface ErrorLog { err: Error; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('v.name') && result.code.includes('v.message'),
            'Should serialize Error with name and message'
        );
        assert.ok(
            result.code.includes('new Error('),
            'Should deserialize Error with new Error()'
        );
    });

    test('URLSearchParams field uses toString/new URLSearchParams', () => {
        const code = `
      /** @derive(Serialize, Deserialize) */
      interface QueryConfig { params: URLSearchParams; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('v.toString()'),
            'Should serialize URLSearchParams with toString()'
        );
        assert.ok(
            result.code.includes('new URLSearchParams(v as string)'),
            'Should deserialize URLSearchParams with new URLSearchParams()'
        );
    });

    test('ArrayBuffer field uses Uint8Array intermediary', () => {
        const code = `
      /** @derive(Serialize, Deserialize) */
      interface BinaryData { buf: ArrayBuffer; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('new Uint8Array(v)') &&
                result.code.includes('Array.from('),
            'Should serialize ArrayBuffer via Uint8Array + Array.from()'
        );
        assert.ok(
            result.code.includes('new Uint8Array(v as number[])') &&
                result.code.includes('.buffer'),
            'Should deserialize ArrayBuffer via new Uint8Array().buffer'
        );
    });

    test('built-in types are not misclassified as Serializable', () => {
        const code = `
      /** @derive(Serialize) */
      interface Config { endpoint: URL; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            !result.code.includes('urlSerializeWithContext'),
            'URL should NOT generate urlSerializeWithContext (was being misclassified as Serializable)'
        );
    });

    test('Record<string, URL> composite uses Object.entries + map', () => {
        const code = `
      /** @derive(Serialize, Deserialize) */
      interface UrlDict { links: Record<string, URL>; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('Object.entries(obj)') &&
                result.code.includes('v.toString()'),
            'Should serialize Record<string, URL> by mapping entries'
        );
        assert.ok(
            result.code.includes('Object.entries(raw as Record<string, unknown>)') &&
                result.code.includes('new URL(v as string)'),
            'Should deserialize Record<string, URL> with Object.entries and new URL()'
        );
    });
});

// ============================================================================
// Serialize metadata stripping (keepMetadata parameter)
// ============================================================================

describe('Serialize metadata stripping', () => {
    test('serialize function includes keepMetadata parameter', () => {
        const code = `
      /** @derive(Serialize) */
      interface Account {
        name: string;
        phones: Array<PhoneNumber>;
      }

      /** @derive(Serialize) */
      interface PhoneNumber {
        main: boolean;
        number: string;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('keepMetadata?: boolean'),
            'Should have keepMetadata optional parameter'
        );
        assert.ok(
            result.code.includes('if (keepMetadata) return JSON.stringify(__raw)'),
            'Should bypass stripping when keepMetadata is true'
        );
        assert.ok(
            result.code.includes(
                'key === "__type" || key === "__id" ? undefined : val'
            ),
            'Should have JSON.stringify replacer that strips __type and __id'
        );
    });

    test('serializeWithContext still includes __type and __id', () => {
        const code = `
      /** @derive(Serialize) */
      interface Point {
        x: number;
        y: number;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('"__type": "Point"'),
            'serializeWithContext should still have __type marker'
        );
        assert.ok(
            result.code.includes('const __id = ctx.register(value)'),
            'serializeWithContext should still register __id'
        );
    });

    test('class static serialize passes keepMetadata through', () => {
        const code = `
      /** @derive(Serialize) */
      class User {
        name: string;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes(
                'static serialize(value: User, keepMetadata?: boolean)'
            ),
            'Static method should accept keepMetadata'
        );
        assert.ok(
            result.code.includes('return userSerialize(value, keepMetadata)'),
            'Static method should pass keepMetadata to standalone function'
        );
    });
});

// ============================================================================
// Enum tagging modes — expansion tests
// ============================================================================

describe('Enum tagging modes — expansion', () => {
    const baseTypes = `
        /** @derive(Serialize, Deserialize) */
        class Cat { name: string; lives: number; }
        /** @derive(Serialize, Deserialize) */
        class Dog { name: string; breed: string; }
    `;

    test('default (internally tagged) — preserves __type pass-through', () => {
        const code = `${baseTypes}
            /** @derive(Serialize, Deserialize) */
            type Animal = Cat | Dog;
        `;
        const result = expandSync(code, 'test.ts');

        // Serialize: should pass through variant directly (internally tagged default)
        assert.ok(
            result.code.includes('return __variant'),
            'InternallyTagged serialize should pass through variant'
        );
        // Deserialize: should check __type tag field
        assert.ok(
            result.code.includes('(value as any)["__type"]'),
            'InternallyTagged deserialize should check __type field'
        );
    });

    test('internally tagged with custom tag name', () => {
        const code = `${baseTypes}
            /** @derive(Serialize, Deserialize) */
            /** @serde({ tag: "kind" }) */
            type Animal = Cat | Dog;
        `;
        const result = expandSync(code, 'test.ts');

        // Deserialize: should check custom "kind" field
        assert.ok(
            result.code.includes('(value as any)["kind"]'),
            'InternallyTagged with custom tag should check "kind" field'
        );
    });

    test('externally tagged — wraps as { TypeName: { ...fields } }', () => {
        const code = `${baseTypes}
            /** @derive(Serialize, Deserialize) */
            /** @serde({ externallyTagged: true }) */
            type Animal = Cat | Dog;
        `;
        const result = expandSync(code, 'test.ts');

        // Serialize: should destructure __type and wrap as { [typeName]: fields }
        assert.ok(
            result.code.includes('{ __type: __typeName, __id: __idVal, ...fields }'),
            'ExternallyTagged serialize should destructure __type from variant'
        );
        assert.ok(
            result.code.includes('[__typeName]: fields'),
            'ExternallyTagged serialize should wrap as { [typeName]: fields }'
        );
        // Deserialize: should extract variant name from object key
        assert.ok(
            result.code.includes('const __variantName = __keys[0]'),
            'ExternallyTagged deserialize should get variant name from first key'
        );
        assert.ok(
            result.code.includes('__variantName === "Cat"'),
            'ExternallyTagged deserialize should match variant name "Cat"'
        );
        // HasShape: should check for object key matching variant name
        assert.ok(
            result.code.includes('animalHasShape'),
            'Should generate animalHasShape function'
        );
    });

    test('adjacently tagged — wraps as { tag: "TypeName", content: { ...fields } }', () => {
        const code = `${baseTypes}
            /** @derive(Serialize, Deserialize) */
            /** @serde({ tag: "t", content: "c" }) */
            type Animal = Cat | Dog;
        `;
        const result = expandSync(code, 'test.ts');

        // Serialize: should wrap with tag and content fields
        assert.ok(
            result.code.includes('"t": __typeName'),
            'AdjacentlyTagged serialize should set tag field "t"'
        );
        assert.ok(
            result.code.includes('"c": fields'),
            'AdjacentlyTagged serialize should set content field "c"'
        );
        // Deserialize: should read tag and content fields
        assert.ok(
            result.code.includes('(value as any)["t"]'),
            'AdjacentlyTagged deserialize should read tag field "t"'
        );
        assert.ok(
            result.code.includes('(value as any)["c"]'),
            'AdjacentlyTagged deserialize should read content field "c"'
        );
        assert.ok(
            result.code.includes('adjacently tagged object with'),
            'AdjacentlyTagged error message should reference adjacently tagged structure'
        );
    });

    test('untagged — strips __type and uses shape matching only', () => {
        const code = `${baseTypes}
            /** @derive(Serialize, Deserialize) */
            /** @serde({ untagged: true }) */
            type Animal = Cat | Dog;
        `;
        const result = expandSync(code, 'test.ts');

        // Serialize: should strip __type and return raw fields
        assert.ok(
            result.code.includes('{ __type: __typeName, __id: __idVal, ...fields }'),
            'Untagged serialize should destructure __type from variant'
        );
        assert.ok(
            result.code.includes('{ ...fields }'),
            'Untagged serialize should spread raw fields'
        );
        // Deserialize: should NOT check any tag field, should use shape matching
        assert.ok(
            result.code.includes('catHasShape(value)'),
            'Untagged deserialize should use catHasShape for shape matching'
        );
        assert.ok(
            result.code.includes('dogHasShape(value)'),
            'Untagged deserialize should use dogHasShape for shape matching'
        );
        assert.ok(
            result.code.includes('does not match any variant shape'),
            'Untagged error message should mention shape matching failure'
        );
    });
});

// ============================================================================
// Enum tagging modes — runtime round-trip tests
// ============================================================================

describe('Enum tagging modes — runtime round-trip', () => {
    /**
     * Helper: expand code, evaluate the generated module, and return its exports.
     * We concatenate the macroforge/serde runtime stubs so the expanded code can
     * actually execute without a real import resolver.
     */
    function evalExpanded(code) {
        const result = expandSync(code, 'test.ts');
        // Strip TypeScript type annotations so we can eval as JS
        // Also strip import statements (we'll inject stubs)
        let js = result.code
            .replace(/import\s+\{[^}]*\}\s+from\s+["'][^"']+["'];?/g, '')
            .replace(/export\s+/g, '')
            // strip type annotations from parameters/return types
            .replace(/:\s*(?:string|number|boolean|void|any|unknown|null|undefined)(?:\s*\|[^,){}=]*)?/g, '')
            .replace(/:\s*(?:Array|Record|Map|Set)<[^>]*>/g, '')
            .replace(/\?\s*:/g, '?') // optional params
            // strip 'as any', 'as Type', etc.
            .replace(/\bas\s+(?:any|[A-Z]\w*(?:\s*\|\s*\w+)*)/g, '')
            // strip interface/type declarations
            .replace(/(?:interface|type)\s+\w+[^{]*\{[^}]*\}/g, '')
            .replace(/type\s+\w+\s*=[^;]+;/g, '');

        // Inject minimal SerializeContext stub
        const stubs = `
            class SerializeContext {
                static create() { return new SerializeContext(); }
                _nextId = 1;
                _seen = new Map();
                getId(obj) { return this._seen.get(obj) ?? null; }
                register(obj) {
                    const id = this._nextId++;
                    this._seen.set(obj, id);
                    return id;
                }
            }
            const __mf_SerializeContext = SerializeContext;
        `;

        const module = { exports: {} };
        const fn = new Function('module', 'exports', stubs + js + `
            module.exports = { ${
                (js.match(/function\s+(\w+)\s*[<(]/g) || [])
                    .map(m => m.match(/function\s+(\w+)/)[1])
                    .join(', ')
            } };
        `);
        fn(module, module.exports);
        return module.exports;
    }

    test('externally tagged: serialize + deserialize both generate correct code', () => {
        const code = `
            /** @derive(Serialize, Deserialize) */
            class Cat {
                name: string;
                lives: number;
            }
            /** @derive(Serialize, Deserialize) */
            class Dog {
                name: string;
                breed: string;
            }
            /** @derive(Serialize, Deserialize) */
            /** @serde({ externallyTagged: true }) */
            type Animal = Cat | Dog;
        `;
        const result = expandSync(code, 'test.ts');

        // Verify the generated code includes the externally tagged wrapping
        assert.ok(
            result.code.includes('[__typeName]: fields'),
            'Should generate externally tagged wrapping in serialize'
        );
        assert.ok(
            result.code.includes('const __variantName = __keys[0]'),
            'Deserialize should extract variant name from object keys'
        );
    });

    test('adjacently tagged: serialize produces { tag, content } structure', () => {
        const code = `
            /** @derive(Serialize) */
            class Cat { name: string; lives: number; }
            /** @derive(Serialize) */
            class Dog { name: string; breed: string; }
            /** @derive(Serialize) */
            /** @serde({ tag: "type", content: "data" }) */
            type Animal = Cat | Dog;
        `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('"type": __typeName, "data": fields'),
            'Should wrap as { type: name, data: fields }'
        );
    });

    test('untagged: serialize strips all type metadata', () => {
        const code = `
            /** @derive(Serialize) */
            class Cat { name: string; lives: number; }
            /** @derive(Serialize) */
            class Dog { name: string; breed: string; }
            /** @derive(Serialize) */
            /** @serde({ untagged: true }) */
            type Animal = Cat | Dog;
        `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('{ ...fields }'),
            'Should spread raw fields without tag'
        );
        assert.ok(
            !result.code.includes('return __variant') ||
            result.code.includes('{ ...fields }'),
            'Should not pass through variant directly in untagged mode'
        );
    });
});
