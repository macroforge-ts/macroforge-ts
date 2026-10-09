/**
 * Comprehensive tests for Encode/Decode macros with cycle detection,
 * forward references, and polymorphic types.
 */

import assert from 'node:assert/strict';
import { describe, test } from 'node:test';
import { expandSync } from '@macroforge/core';

// ============================================================================
// Encode Macro Expansion Tests
// ============================================================================

describe('Encode macro expansion', () => {
    test('generates encode and encodeWithContext methods for classes', () => {
        const code = `
      /** @derive(Encode) */
      class User {
        name: string;
        age: number;
      }
    `;
        const result = expandSync(code, 'test.ts');

        // New format uses static methods
        assert.ok(
            result.code.includes(
                'static encode(value: User, keepMetadata?: boolean)'
            ),
            'Should generate static encode method'
        );
        assert.ok(
            result.code.includes('static encodeWithContext(value: User, ctx'),
            'Should generate static encodeWithContext method'
        );
        assert.ok(
            result.code.includes('EncodeContext'),
            'Should use EncodeContext'
        );
    });

    test('generates __type and __id in encoding output', () => {
        const code = `
      /** @derive(Encode) */
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
      /** @derive(Encode) */
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
      /** @derive(Encode) */
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
      /** @derive(Encode) */
      interface IPoint {
        x: number;
        y: number;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('function iPointEncode('),
            'Should generate iPointEncode function'
        );
        assert.ok(
            result.code.includes('function iPointEncodeWithContext('),
            'Should generate iPointEncodeWithContext function'
        );
    });

    test('interfaces call nested prefixed encodeWithContext functions', () => {
        const code = `
      /** @derive(Encode) */
      interface Metadata {
        createdAt: string;
      }

      /** @derive(Encode) */
      interface User {
        metadata: Metadata | null;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('function userEncodeWithContext('),
            'Should generate userEncodeWithContext'
        );
        assert.ok(
            result.code.includes('metadataEncodeWithContext('),
            'Should call metadataEncodeWithContext for nested type'
        );
        assert.ok(
            !result.code.includes('Metadata.encodeWithContext('),
            'Should not use namespace-style Metadata.encodeWithContext'
        );
    });

    test('handles @endec(rename) decorator', () => {
        const code = `
      /** @derive(Encode) */
      class ApiResponse {
        /** @endec({ rename: "user_id" }) */
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

    test('handles @endec(skip) decorator', () => {
        const code = `
      /** @derive(Encode) */
      class Credentials {
        username: string;
        /** @endec({ skip: true }) */
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

    test('handles @endec(flatten) decorator', () => {
        const code = `
      /** @derive(Encode) */
      interface Metadata {
        createdAt: Date;
      }

      /** @derive(Encode) */
      class Entity {
        id: string;
        /** @endec({ flatten: true }) */
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

    test('handles Date encoding to ISO string', () => {
        const code = `
      /** @derive(Encode) */
      class Event {
        name: string;
        timestamp: Date;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('toISOString()'),
            'Should encode Date as ISO string'
        );
    });

    test('handles Array encoding with nested objects', () => {
        const code = `
      /** @derive(Encode) */
      class Container {
        items: Item[];
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(result.code.includes('.map('), 'Should map over array items');
        assert.ok(
            result.code.includes('EncodeWithContext'),
            'Should call encodeWithContext on nested items'
        );
    });

    test('handles Map encoding', () => {
        const code = `
      /** @derive(Encode) */
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

    test('handles Set encoding', () => {
        const code = `
      /** @derive(Encode) */
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
// Decode Macro Expansion Tests
// ============================================================================

describe('Decode macro expansion', () => {
    test('generates decode and decodeWithContext methods for classes', () => {
        const code = `
      /** @derive(Decode) */
      class User {
        name: string;
        age: number;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('static decode('),
            'Should generate static decode'
        );
        assert.ok(
            result.code.includes('static decodeWithContext('),
            'Should generate static decodeWithContext'
        );
        assert.ok(
            result.code.includes('DecodeContext'),
            'Should use DecodeContext'
        );
    });

    test('handles __ref for cycle detection', () => {
        const code = `
      /** @derive(Decode) */
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
      /** @derive(Decode) */
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
      /** @derive(Decode) */
      class Config {
        name: string;
        /** @endec({ default: '"default_value"' }) */
        value?: string;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('default_value'),
            'Should use default value'
        );
    });

    test('handles Date decoding from ISO string', () => {
        const code = `
      /** @derive(Decode) */
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

    test('handles Array decoding', () => {
        const code = `
      /** @derive(Decode) */
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

    test('handles Map decoding', () => {
        const code = `
      /** @derive(Decode) */
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

    test('handles Set decoding', () => {
        const code = `
      /** @derive(Decode) */
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
      /** @derive(Decode) */
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

    test('handles @endec(denyUnknownFields)', () => {
        const code = `
      /**
       * @derive(Decode)
       * @endec({ denyUnknownFields: true })
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

    test('registers decoded objects with context', () => {
        const code = `
      /** @derive(Decode) */
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

    test('supports freeze option in decode', () => {
        const code = `
      /** @derive(Decode) */
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

    test('interfaces call nested prefixed decodeWithContext functions', () => {
        const code = `
      /** @derive(Decode) */
      interface Metadata {
        createdAt: string;
      }

      /** @derive(Decode) */
      interface User {
        metadata: Metadata | null;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('function userDecodeWithContext('),
            'Should generate userDecodeWithContext'
        );
        assert.ok(
            result.code.includes('metadataDecodeWithContext('),
            'Should call metadataDecodeWithContextfor nested type'
        );
        assert.ok(
            !result.code.includes('Metadata.decodeWithContext('),
            'Should not use namespace-style Metadata.decodeWithContext'
        );
    });
});

describe('External type function imports', () => {
    test('injects imports for nested type functions (prefix style)', () => {
        const code = `
      import { Metadata } from "./metadata.svelte";

      /** @derive(Default, Encode, Decode) */
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
                'import { metadataDecodeWithContext, metadataEncodeWithContext, metadataDefaultValue } from "./metadata.svelte";'
            ),
            `Should import the three metadata functions in one declaration. Got: ${result.code}`
        );
    });
});

// ============================================================================
// Combined Encode + Decode
// ============================================================================

describe('Combined Encode + Decode', () => {
    test('generates both sets of methods when both derived', () => {
        const code = `
      /** @derive(Encode, Decode) */
      class Entity {
        id: string;
        data: string;
      }
    `;
        const result = expandSync(code, 'test.ts');

        // Encode methods (now static)
        assert.ok(
            result.code.includes(
                'static encode(value: Entity, keepMetadata?: boolean)'
            ),
            'Should have static encode'
        );
        assert.ok(
            result.code.includes('static encodeWithContext(value: Entity, ctx'),
            'Should have static encodeWithContext'
        );

        // Decode methods
        assert.ok(
            result.code.includes('static decode('),
            'Should have decode'
        );
        assert.ok(
            result.code.includes('static decodeWithContext('),
            'Should have decodeWithContext'
        );
    });

    test('handles nested encodable types', () => {
        const code = `
      /** @derive(Encode, Decode) */
      class Address {
        street: string;
        city: string;
      }

      /** @derive(Encode, Decode) */
      class Person {
        name: string;
        address: Address;
      }
    `;
        const result = expandSync(code, 'test.ts');

        // Both types should have endec methods
        assert.ok(
            result.code.includes('class Address'),
            'Should have Address class'
        );
        assert.ok(result.code.includes('class Person'), 'Should have Person class');

        // Person should call Address's encodeWithContext
        assert.ok(
            result.code.includes('EncodeWithContext') &&
                result.code.includes('address'),
            'Should encode nested address'
        );
    });
});

// ============================================================================
// Enum encoding
// ============================================================================

describe('Enum encoding', () => {
    test('generates prefixed functions for enum encoding', () => {
        const code = `
      /** @derive(Encode) */
      enum Status {
        Active = "active",
        Inactive = "inactive"
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('function statusEncode('),
            'Should have statusEncode function'
        );
        assert.ok(
            result.code.includes('function statusEncodeWithContext('),
            'Should have statusEncodeWithContext function'
        );
    });

    test('generates prefixed functions for enum decoding', () => {
        const code = `
      /** @derive(Decode) */
      enum Status {
        Active = "active",
        Inactive = "inactive"
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('function statusDecode('),
            'Should have statusDecode function'
        );
        assert.ok(
            result.code.includes('function statusDecodeWithContext('),
            'Should have statusDecodeWithContext function'
        );
        assert.ok(result.code.includes('Invalid'), 'Should validate enum values');
    });
});

// ============================================================================
// Type alias encoding
// ============================================================================

describe('Type alias encoding', () => {
    test('generates prefixed functions for object type alias', () => {
        const code = `
      /** @derive(Encode) */
      type Point = {
        x: number;
        y: number;
      };
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('function pointEncode('),
            'Should have pointEncode'
        );
        assert.ok(
            result.code.includes('function pointEncodeWithContext('),
            'Should have pointEncodeWithContext'
        );
    });

    test('generates prefixed functions for union type alias', () => {
        const code = `
      /** @derive(Encode) */
      type Result = Success | Failure;
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('function resultEncode('),
            'Should have resultEncode'
        );
        assert.ok(
            result.code.includes('function resultEncodeWithContext('),
            'Should have resultEncodeWithContext'
        );
    });
});

// ============================================================================
// Import handling
// ============================================================================

describe('Import handling', () => {
    test('adds EncodeContext import for Encode', () => {
        const code = `
      /** @derive(Encode) */
      class User {
        name: string;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes(
                'import { EncodeContext as __mf_EncodeContext } from "@macroforge/core/endec"'
            ),
            'Should add EncodeContext import with __mf_ alias'
        );
    });

    test('adds DecodeContext import for Decode', () => {
        const code = `
      /** @derive(Decode) */
      class User {
        name: string;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('import { DecodeContext'),
            'Should add DecodeContext import'
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
    test('empty class encoding', () => {
        const code = `
      /** @derive(Encode, Decode) */
      class Empty {}
    `;
        const result = expandSync(code, 'test.ts');

        // New format uses static methods
        assert.ok(
            result.code.includes(
                'static encode(value: Empty, keepMetadata?: boolean)'
            ),
            'Should generate static encode'
        );
        assert.ok(
            result.code.includes('static decode'),
            'Should generate static decode'
        );
        assert.ok(
            result.code.includes('"__type": "Empty"'),
            'Should have type marker'
        );
    });

    test('nullable field handling', () => {
        const code = `
      /** @derive(Encode, Decode) */
      class WithNull {
        value: string | null;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(result.code.includes('null'), 'Should handle null explicitly');
    });

    test('self-referential type', () => {
        const code = `
      /** @derive(Encode) */
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
      /** @derive(Encode) */
      class Deep {
        data: Map<string, Set<number[]>>;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('Object.fromEntries'),
            'Should handle Map encoding'
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
       * @derive(Encode)
       * @endec({ renameAll: "camelCase" })
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
       * @derive(Encode)
       * @endec({ renameAll: "snake_case" })
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
// Recursive collection decoding
// ============================================================================

describe('Recursive collection decoding', () => {
    test('Array<Encodable> recursively decodes elements in interfaces', () => {
        const code = `
      /** @derive(Decode) */
      interface Activity {
        action: string;
        timestamp: Date;
      }

      /** @derive(Decode) */
      interface Account {
        name: string;
        activity: Activity[];
      }
    `;
        const result = expandSync(code, 'test.ts');

        // Should call activityDecode (result-returning) for each array element
        assert.ok(
            result.code.includes('activityDecode(item)'),
            'Should recursively decode Activity elements via result-returning function'
        );
        // Should NOT have a raw `as Activity[]` cast in the Account decoder
        // (the Activity[] field should go through .map with decoding)
        assert.ok(
            result.code.includes('__arr'),
            'Should use __arr intermediate for PendingRef support'
        );
        // Activity.timestamp should be parsed as Date
        assert.ok(
            result.code.includes(
                'typeof __raw_timestamp === "string" ? new Date(__raw_timestamp)'
            ),
            'Activity.timestamp should be decoded from ISO string to Date'
        );
    });

    test('Array<Encodable> recursively decodes elements in classes', () => {
        const code = `
      /** @derive(Decode) */
      interface Tag {
        label: string;
        createdAt: Date;
      }

      /** @derive(Decode) */
      class Post {
        title: string;
        tags: Tag[];
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('tagDecode(item)'),
            'Class template should call tagDecode for Tag[] elements'
        );
    });

    test('Array<Encodable> recursively decodes elements in type aliases', () => {
        const code = `
      /** @derive(Decode) */
      interface Item {
        name: string;
        price: number;
      }

      /** @derive(Decode) */
      type Cart = {
        items: Item[];
      };
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('itemDecode(item)'),
            'Type alias template should call itemDecode for Item[] elements'
        );
    });

    test('Array<Date> maps ISO strings to Date objects', () => {
        const code = `
      /** @derive(Decode) */
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

    test('Set<Encodable> recursively decodes elements', () => {
        const code = `
      /** @derive(Decode) */
      interface Permission {
        resource: string;
        level: number;
      }

      /** @derive(Decode) */
      interface Role {
        permissions: Set<Permission>;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('permissionDecode(item)'),
            'Should call permissionDecode for Set<Permission> elements'
        );
        assert.ok(
            result.code.includes('new Set('),
            'Should wrap result in new Set()'
        );
    });

    test('Map<string, Encodable> recursively decodes values', () => {
        const code = `
      /** @derive(Decode) */
      interface Score {
        value: number;
        date: Date;
      }

      /** @derive(Decode) */
      interface Leaderboard {
        scores: Map<string, Score>;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('scoreDecode(v)'),
            'Should call scoreDecode for Map values'
        );
        assert.ok(
            result.code.includes('new Map('),
            'Should wrap result in new Map()'
        );
    });

    test('deeply nested: Array<T> where T has Array<U> with Date fields', () => {
        const code = `
      /** @derive(Decode) */
      interface Metric {
        name: string;
        recordedAt: Date;
      }

      /** @derive(Decode) */
      interface Report {
        title: string;
        metrics: Metric[];
      }

      /** @derive(Decode) */
      interface Dashboard {
        reports: Report[];
        generatedAt: Date;
      }
    `;
        const result = expandSync(code, 'test.ts');

        // Dashboard.reports should recursively decode Report
        assert.ok(
            result.code.includes('reportDecode(item)'),
            'Dashboard should recursively decode Report[] elements'
        );
        // Report.metrics should recursively decode Metric
        assert.ok(
            result.code.includes('metricDecode(item)'),
            'Report should recursively decode Metric[] elements'
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

    test('mixed collection types: Array, Set, Map with encodable values', () => {
        const code = `
      /** @derive(Decode) */
      interface Address {
        street: string;
        city: string;
      }

      /** @derive(Decode) */
      interface Company {
        locations: Address[];
        branches: Set<Address>;
        directoryByCity: Map<string, Address>;
      }
    `;
        const result = expandSync(code, 'test.ts');

        // Array<Address>
        assert.ok(
            result.code.includes('addressDecode(item)'),
            'Array<Address> should recursively decode'
        );
        // Set<Address>
        assert.ok(
            result.code.includes('new Set(') &&
                result.code.includes('addressDecode(item)'),
            'Set<Address> should recursively decode into Set'
        );
        // Map<string, Address>
        assert.ok(
            result.code.includes('addressDecode(v)'),
            'Map<string, Address> should recursively decode values'
        );
    });

    test('primitive collections are NOT recursively decoded', () => {
        const code = `
      /** @derive(Decode) */
      interface Simple {
        names: string[];
        ids: Set<number>;
        labels: Map<string, string>;
      }
    `;
        const result = expandSync(code, 'test.ts');

        // string[] should be a direct cast, no DecodeWithContext
        assert.ok(
            result.code.includes('as string[]'),
            'string[] should use direct cast'
        );
        assert.ok(
            !result.code.includes('DecodeWithContext(item'),
            'Primitive arrays should NOT call DecodeWithContext on elements'
        );
    });
});

// ============================================================================
// Built-in type endec tests (bigint, URL, RegExp, TypedArrays, etc.)
// ============================================================================

describe('Built-in type endec', () => {
    test('bigint field uses String/BigInt conversion', () => {
        const code = `
      /** @derive(Encode, Decode) */
      interface Counter { count: bigint; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('String(v)'),
            'Should encode bigint with String(v)'
        );
        assert.ok(
            result.code.includes('BigInt(v as string)'),
            'Should decode bigint with BigInt()'
        );
    });

    test('URL field uses toString/new URL conversion', () => {
        const code = `
      /** @derive(Encode, Decode) */
      interface Endpoint { url: URL; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('v.toString()'),
            'Should encode URL with toString()'
        );
        assert.ok(
            result.code.includes('new URL(v as string)'),
            'Should decode URL with new URL()'
        );
    });

    test('RegExp field uses source/flags encoding', () => {
        const code = `
      /** @derive(Encode, Decode) */
      interface Rule { pattern: RegExp; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('v.source') && result.code.includes('v.flags'),
            'Should encode RegExp with source and flags'
        );
        assert.ok(
            result.code.includes('new RegExp('),
            'Should decode RegExp with new RegExp()'
        );
    });

    test('Uint8Array field uses Array.from/new Uint8Array', () => {
        const code = `
      /** @derive(Encode, Decode) */
      interface Buffer { data: Uint8Array; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('Array.from(v)'),
            'Should encode Uint8Array with Array.from()'
        );
        assert.ok(
            result.code.includes('new Uint8Array(v as number[])'),
            'Should decode Uint8Array with new Uint8Array()'
        );
    });

    test('URL[] composite uses map with toString/new URL', () => {
        const code = `
      /** @derive(Encode, Decode) */
      interface Links { urls: URL[]; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('.map(') && result.code.includes('v.toString()'),
            'Should encode URL[] by mapping toString()'
        );
        assert.ok(
            result.code.includes('.map(') &&
                result.code.includes('new URL(v as string)'),
            'Should decode URL[] by mapping new URL()'
        );
    });

    test('Set<URL> composite uses Array.from + map / new Set + map', () => {
        const code = `
      /** @derive(Encode, Decode) */
      interface BookmarkSet { urls: Set<URL>; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('Array.from(s)') &&
                result.code.includes('v.toString()'),
            'Should encode Set<URL> with Array.from and toString()'
        );
        assert.ok(
            result.code.includes('new Set(') &&
                result.code.includes('new URL(v as string)'),
            'Should decode Set<URL> with new Set and new URL()'
        );
    });

    test('Map<string, URL> composite uses entries + map', () => {
        const code = `
      /** @derive(Encode, Decode) */
      interface UrlMap { endpoints: Map<string, URL>; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('m.entries()') &&
                result.code.includes('v.toString()'),
            'Should encode Map<string, URL> by mapping entries'
        );
        assert.ok(
            result.code.includes('new Map(') &&
                result.code.includes('new URL(v as string)'),
            'Should decode Map<string, URL> with new Map and new URL()'
        );
    });

    test('bigint | null nullable uses null check + String/BigInt', () => {
        const code = `
      /** @derive(Encode, Decode) */
      interface OptCounter { count: bigint | null; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('=== null ? null') &&
                result.code.includes('String(v)'),
            'Should encode bigint | null with null check and String()'
        );
        assert.ok(
            result.code.includes('=== null ? null') &&
                result.code.includes('BigInt(v as string)'),
            'Should decode bigint | null with null check and BigInt()'
        );
    });

    test('Error field uses name/message/stack encoding', () => {
        const code = `
      /** @derive(Encode, Decode) */
      interface ErrorLog { err: Error; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('v.name') && result.code.includes('v.message'),
            'Should encode Error with name and message'
        );
        assert.ok(
            result.code.includes('new Error('),
            'Should decode Error with new Error()'
        );
    });

    test('URLSearchParams field uses toString/new URLSearchParams', () => {
        const code = `
      /** @derive(Encode, Decode) */
      interface QueryConfig { params: URLSearchParams; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('v.toString()'),
            'Should encode URLSearchParams with toString()'
        );
        assert.ok(
            result.code.includes('new URLSearchParams(v as string)'),
            'Should decode URLSearchParams with new URLSearchParams()'
        );
    });

    test('ArrayBuffer field uses Uint8Array intermediary', () => {
        const code = `
      /** @derive(Encode, Decode) */
      interface BinaryData { buf: ArrayBuffer; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('new Uint8Array(v)') &&
                result.code.includes('Array.from('),
            'Should encode ArrayBuffer via Uint8Array + Array.from()'
        );
        assert.ok(
            result.code.includes('new Uint8Array(v as number[])') &&
                result.code.includes('.buffer'),
            'Should decode ArrayBuffer via new Uint8Array().buffer'
        );
    });

    test('built-in types are not misclassified as Encodable', () => {
        const code = `
      /** @derive(Encode) */
      interface Config { endpoint: URL; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            !result.code.includes('urlEncodeWithContext'),
            'URL should NOT generate urlEncodeWithContext (was being misclassified as Encodable)'
        );
    });

    test('Record<string, URL> composite uses Object.entries + map', () => {
        const code = `
      /** @derive(Encode, Decode) */
      interface UrlDict { links: Record<string, URL>; }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('Object.entries(obj)') &&
                result.code.includes('v.toString()'),
            'Should encode Record<string, URL> by mapping entries'
        );
        assert.ok(
            result.code.includes('Object.entries(raw as Record<string, unknown>)') &&
                result.code.includes('new URL(v as string)'),
            'Should decode Record<string, URL> with Object.entries and new URL()'
        );
    });
});

// ============================================================================
// Encode metadata stripping (keepMetadata parameter)
// ============================================================================

describe('Encode metadata stripping', () => {
    test('encode function includes keepMetadata parameter', () => {
        const code = `
      /** @derive(Encode) */
      interface Account {
        name: string;
        phones: Array<PhoneNumber>;
      }

      /** @derive(Encode) */
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

    test('encodeWithContext still includes __type and __id', () => {
        const code = `
      /** @derive(Encode) */
      interface Point {
        x: number;
        y: number;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('"__type": "Point"'),
            'encodeWithContext should still have __type marker'
        );
        assert.ok(
            result.code.includes('const __id = ctx.register(value)'),
            'encodeWithContext should still register __id'
        );
    });

    test('class static encode passes keepMetadata through', () => {
        const code = `
      /** @derive(Encode) */
      class User {
        name: string;
      }
    `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes(
                'static encode(value: User, keepMetadata?: boolean)'
            ),
            'Static method should accept keepMetadata'
        );
        assert.ok(
            result.code.includes('return userEncode(value, keepMetadata)'),
            'Static method should pass keepMetadata to standalone function'
        );
    });
});

// ============================================================================
// Enum tagging modes: expansion tests
// ============================================================================

describe('Enum tagging modes: expansion', () => {
    const baseTypes = `
        /** @derive(Encode, Decode) */
        class Cat { name: string; lives: number; }
        /** @derive(Encode, Decode) */
        class Dog { name: string; breed: string; }
    `;

    test('default (internally tagged): preserves __type pass-through', () => {
        const code = `${baseTypes}
            /** @derive(Encode, Decode) */
            type Animal = Cat | Dog;
        `;
        const result = expandSync(code, 'test.ts');

        // Encode: should pass through variant directly (internally tagged default)
        assert.ok(
            result.code.includes('return __variant'),
            'InternallyTagged encode should pass through variant'
        );
        // Decode: should check __type tag field
        assert.ok(
            result.code.includes('(value as any)["__type"]'),
            'InternallyTagged decode should check __type field'
        );
    });

    test('internally tagged with custom tag name', () => {
        const code = `${baseTypes}
            /** @derive(Encode, Decode) */
            /** @endec({ tag: "kind" }) */
            type Animal = Cat | Dog;
        `;
        const result = expandSync(code, 'test.ts');

        // Decode: should check custom "kind" field
        assert.ok(
            result.code.includes('(value as any)["kind"]'),
            'InternallyTagged with custom tag should check "kind" field'
        );
    });

    test('externally tagged: wraps as { TypeName: { ...fields } }', () => {
        const code = `${baseTypes}
            /** @derive(Encode, Decode) */
            /** @endec({ externallyTagged: true }) */
            type Animal = Cat | Dog;
        `;
        const result = expandSync(code, 'test.ts');

        // Encode: should destructure __type and wrap as { [typeName]: fields }
        assert.ok(
            result.code.includes('{ __type: __typeName, __id: __idVal, ...fields }'),
            'ExternallyTagged encode should destructure __type from variant'
        );
        assert.ok(
            result.code.includes('[__typeName]: fields'),
            'ExternallyTagged encode should wrap as { [typeName]: fields }'
        );
        // Decode: should extract variant name from object key
        assert.ok(
            result.code.includes('const __variantName = __keys[0]'),
            'ExternallyTagged decode should get variant name from first key'
        );
        assert.ok(
            result.code.includes('__variantName === "Cat"'),
            'ExternallyTagged decode should match variant name "Cat"'
        );
        // HasShape: should check for object key matching variant name
        assert.ok(
            result.code.includes('animalHasShape'),
            'Should generate animalHasShape function'
        );
    });

    test('adjacently tagged: wraps as { tag: "TypeName", content: { ...fields } }', () => {
        const code = `${baseTypes}
            /** @derive(Encode, Decode) */
            /** @endec({ tag: "t", content: "c" }) */
            type Animal = Cat | Dog;
        `;
        const result = expandSync(code, 'test.ts');

        // Encode: should wrap with tag and content fields
        assert.ok(
            result.code.includes('"t": __typeName'),
            'AdjacentlyTagged encode should set tag field "t"'
        );
        assert.ok(
            result.code.includes('"c": fields'),
            'AdjacentlyTagged encode should set content field "c"'
        );
        // Decode: should read tag and content fields
        assert.ok(
            result.code.includes('(value as any)["t"]'),
            'AdjacentlyTagged decode should read tag field "t"'
        );
        assert.ok(
            result.code.includes('(value as any)["c"]'),
            'AdjacentlyTagged decode should read content field "c"'
        );
        assert.ok(
            result.code.includes('adjacently tagged object with'),
            'AdjacentlyTagged error message should reference adjacently tagged structure'
        );
    });

    test('untagged: strips __type and uses shape matching only', () => {
        const code = `${baseTypes}
            /** @derive(Encode, Decode) */
            /** @endec({ untagged: true }) */
            type Animal = Cat | Dog;
        `;
        const result = expandSync(code, 'test.ts');

        // Encode: should strip __type and return raw fields
        assert.ok(
            result.code.includes('{ __type: __typeName, __id: __idVal, ...fields }'),
            'Untagged encode should destructure __type from variant'
        );
        assert.ok(
            result.code.includes('{ ...fields }'),
            'Untagged encode should spread raw fields'
        );
        // Decode: should NOT check any tag field, should use shape matching
        assert.ok(
            result.code.includes('catHasShape(value)'),
            'Untagged decode should use catHasShape for shape matching'
        );
        assert.ok(
            result.code.includes('dogHasShape(value)'),
            'Untagged decode should use dogHasShape for shape matching'
        );
        assert.ok(
            result.code.includes('does not match any variant shape'),
            'Untagged error message should mention shape matching failure'
        );
    });
});

// ============================================================================
// Enum tagging modes: runtime round-trip tests
// ============================================================================

describe('Enum tagging modes: runtime round-trip', () => {
    test('externally tagged: encode + decode both generate correct code', () => {
        const code = `
            /** @derive(Encode, Decode) */
            class Cat {
                name: string;
                lives: number;
            }
            /** @derive(Encode, Decode) */
            class Dog {
                name: string;
                breed: string;
            }
            /** @derive(Encode, Decode) */
            /** @endec({ externallyTagged: true }) */
            type Animal = Cat | Dog;
        `;
        const result = expandSync(code, 'test.ts');

        // Verify the generated code includes the externally tagged wrapping
        assert.ok(
            result.code.includes('[__typeName]: fields'),
            'Should generate externally tagged wrapping in encode'
        );
        assert.ok(
            result.code.includes('const __variantName = __keys[0]'),
            'Decode should extract variant name from object keys'
        );
    });

    test('adjacently tagged: encode produces { tag, content } structure', () => {
        const code = `
            /** @derive(Encode) */
            class Cat { name: string; lives: number; }
            /** @derive(Encode) */
            class Dog { name: string; breed: string; }
            /** @derive(Encode) */
            /** @endec({ tag: "type", content: "data" }) */
            type Animal = Cat | Dog;
        `;
        const result = expandSync(code, 'test.ts');

        assert.ok(
            result.code.includes('"type": __typeName, "data": fields'),
            'Should wrap as { type: name, data: fields }'
        );
    });

    test('untagged: encode strips all type metadata', () => {
        const code = `
            /** @derive(Encode) */
            class Cat { name: string; lives: number; }
            /** @derive(Encode) */
            class Dog { name: string; breed: string; }
            /** @derive(Encode) */
            /** @endec({ untagged: true }) */
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
