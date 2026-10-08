/**
 * Tests for foreign types configuration in macroforge.config.js
 *
 * Foreign types allow global registration of handlers for external types
 * (like Effect's DateTime) so they work automatically in encoding/decoding
 * without needing per-field decorators.
 */

import assert from 'node:assert/strict';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, test } from 'node:test';
import { clearConfigCache, expandSync, loadConfig } from '@macroforge/core';

// ============================================================================
// Foreign Type Configuration Tests
// ============================================================================

describe('Foreign types configuration', () => {
    test('loadConfig parses foreign types from config content', () => {
        clearConfigCache();
        const configContent = `
      export default {
        foreignTypes: {
          "DateTime.DateTime": {
            from: ["effect"],
            encode: (v) => v.toJSON(),
            decode: (raw) => DateTime.fromJSON(raw),
            default: () => DateTime.now()
          }
        }
      }
    `;

        const result = loadConfig(configContent, 'macroforge.config.js');

        assert.equal(result.hasForeignTypes, true, 'Should have foreign types');
        assert.equal(result.foreignTypeCount, 1, 'Should have 1 foreign type');
    });

    test('loadConfig handles multiple foreign types', () => {
        clearConfigCache();
        const configContent = `
      export default {
        foreignTypes: {
          "DateTime.DateTime": {
            from: ["effect"],
            encode: (v) => v.toJSON(),
            decode: (raw) => DateTime.fromJSON(raw)
          },
          "Duration.Duration": {
            from: ["effect"],
            encode: (v) => v.toMillis(),
            decode: (raw) => Duration.millis(raw)
          }
        }
      }
    `;

        const result = loadConfig(configContent, 'macroforge.config.js');

        assert.equal(result.hasForeignTypes, true, 'Should have foreign types');
        assert.equal(result.foreignTypeCount, 2, 'Should have 2 foreign types');
    });

    test('loadConfig handles config without foreign types', () => {
        clearConfigCache();
        const configContent = `
      export default {
        keepDecorators: false
      }
    `;

        const result = loadConfig(configContent, 'macroforge.config.js');

        assert.equal(
            result.hasForeignTypes,
            false,
            'Should not have foreign types'
        );
        assert.equal(result.foreignTypeCount, 0, 'Should have 0 foreign types');
    });

    // The wasm engine reads base configs through Node's `fs`, so `extends`
    // and imported bases resolve here exactly as they do natively.
    test('loadConfig follows extends and imported bases from disk', () => {
        clearConfigCache();
        const dir = mkdtempSync(join(tmpdir(), 'macroforge-config-'));
        try {
            writeFileSync(
                join(dir, 'base.config.ts'),
                `export default {
                    foreignTypes: {
                        "DateTime.DateTime": { from: ["effect"] },
                        "Duration.Duration": { from: ["effect"] },
                    },
                };`
            );
            const extended = loadConfig(
                `export default {
                    extends: "./base.config.ts",
                    foreignTypes: { "Option.Option": { from: ["effect"] } },
                };`,
                join(dir, 'macroforge.config.ts')
            );
            assert.equal(extended.foreignTypeCount, 3, 'base and own foreign types merge');

            const reexported = loadConfig(
                `import base from "./base.config";\nexport default base;`,
                join(dir, 'reexport.config.ts')
            );
            assert.equal(reexported.foreignTypeCount, 2, 'an imported base is the config');

            assert.throws(
                () => loadConfig(`export default makeConfig();`, join(dir, 'called.config.ts')),
                /has 0 arguments/,
                'a config the reader cannot follow is an error'
            );
        } finally {
            rmSync(dir, { recursive: true, force: true });
        }
    });
});

// ============================================================================
// Foreign Type Expansion Tests - Default Macro
// ============================================================================

describe('Foreign types in Default macro', () => {
    const configContent = `
    export default {
      foreignTypes: {
        "DateTime.DateTime": {
          from: ["effect"],
          encode: (v) => v.toJSON(),
          decode: (raw) => DateTime.fromJSON(raw),
          default: () => DateTime.unsafeNow()
        }
      }
    }
  `;
    // Use unique config path to avoid caching issues
    const configPath = '/test/default-macro/macroforge.config.js';

    test('default value uses foreign type default function', () => {
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

        const result = expandSync(code, 'test.ts', { configPath });

        // The default value should call the configured default function as an IIFE
        assert.ok(
            result.code.includes('DateTime.unsafeNow()'),
            `Default should use the foreign type default function. Got: ${result.code}`
        );
    });
});

// ============================================================================
// Foreign Type Expansion Tests - Encode Macro
// ============================================================================

describe('Foreign types in Encode macro', () => {
    const configContent = `
    export default {
      foreignTypes: {
        "DateTime.DateTime": {
          from: ["effect"],
          encode: (v) => DateTime.formatIso(v),
          decode: (raw) => DateTime.unsafeFromDate(new Date(raw)),
          default: () => DateTime.unsafeNow()
        }
      }
    }
  `;
    // Use unique config path to avoid caching issues
    const configPath = '/test/encode-macro/macroforge.config.js';

    test('encode uses foreign type encode function', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const code = `
      import type { DateTime } from 'effect';

      /** @derive(Encode) */
      interface Event {
        name: string;
        startTime: DateTime.DateTime;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        // The encode should use the configured encode function
        assert.ok(
            result.code.includes('DateTime.formatIso'),
            `Encode should use the foreign type encode function. Got: ${result.code}`
        );
        // Should NOT generate a generic helper call like dateTime.DateTimeEncodeWithContext
        assert.ok(
            !result.code.includes('dateTime.DateTime'),
            `Should not generate generic helper namespace. Got: ${result.code}`
        );
    });

    test('a near match is reported as a warning while expansion succeeds', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const code = `
      import type { DateTime } from 'effect';

      /** @derive(Encode) */
      interface Event {
        startTime: DateTime;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        assert.ok(
            !result.diagnostics.some((d) => d.level === 'error'),
            `Expansion should succeed. Got: ${JSON.stringify(result.diagnostics)}`
        );
        assert.ok(
            result.diagnostics.some(
                (d) =>
                    d.level === 'warning' && d.message.includes("foreign type 'DateTime.DateTime'")
            ),
            `The near match should be reported. Got: ${JSON.stringify(result.diagnostics)}`
        );
    });

    test('an object type alias uses the foreign type encode function', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const code = `
      import type { DateTime } from 'effect';

      /** @derive(Encode) */
      type Event = {
        name: string;
        startTime: DateTime.DateTime;
      };
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        assert.ok(
            result.code.includes('DateTime.formatIso'),
            `Encode should use the foreign type encode function. Got: ${result.code}`
        );
        assert.ok(
            !result.code.includes('dateTime.DateTime'),
            `Should not generate generic helper namespace. Got: ${result.code}`
        );
    });
});

// ============================================================================
// Foreign Type Expansion Tests - Decode Macro
// ============================================================================

describe('Foreign types in Decode macro', () => {
    const configContent = `
    export default {
      foreignTypes: {
        "DateTime.DateTime": {
          from: ["effect"],
          encode: (v) => DateTime.formatIso(v),
          decode: (raw) => DateTime.unsafeFromDate(new Date(raw)),
          default: () => DateTime.unsafeNow()
        }
      }
    }
  `;
    // Use unique config path to avoid caching issues
    const configPath = '/test/decode-macro/macroforge.config.js';

    test('decode uses foreign type decode function', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const code = `
      import type { DateTime } from 'effect';

      /** @derive(Decode) */
      interface Event {
        name: string;
        startTime: DateTime.DateTime;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        // The decode should use the configured decode function
        assert.ok(
            result.code.includes('DateTime.unsafeFromDate'),
            `Decode should use the foreign type decode function. Got: ${result.code}`
        );
        // Should NOT generate a generic helper call
        assert.ok(
            !result.code.includes('dateTime.DateTime'),
            `Should not generate generic helper namespace. Got: ${result.code}`
        );
    });
});

// ============================================================================
// Foreign Type Expansion Tests - Combined Macros
// ============================================================================

describe('Foreign types with combined macros', () => {
    const configContent = `
    export default {
      foreignTypes: {
        "DateTime.DateTime": {
          from: ["effect"],
          encode: (v) => DateTime.formatIso(v),
          decode: (raw) => DateTime.unsafeFromDate(new Date(raw)),
          default: () => DateTime.unsafeNow()
        }
      }
    }
  `;
    // Use unique config path to avoid caching issues
    const configPath = '/test/combined-macros/macroforge.config.js';

    test('all macros use foreign type functions when combined', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const code = `
      import type { DateTime } from 'effect';

      /** @derive(Default, Encode, Decode) */
      interface Event {
        name: string;
        startTime: DateTime.DateTime;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        // Check all three foreign type functions are used
        assert.ok(
            result.code.includes('DateTime.unsafeNow()'),
            `Default should use foreign type default. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('DateTime.formatIso'),
            `Encode should use foreign type encode. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('DateTime.unsafeFromDate'),
            `Decode should use foreign type decode. Got: ${result.code}`
        );
    });
});

// ============================================================================
// Foreign Type Import Matching Tests
// ============================================================================

describe('Foreign type import matching', () => {
    const configContent = `
    export default {
      foreignTypes: {
        "DateTime.DateTime": {
          from: ["effect", "@effect/schema"],
          encode: (v) => DateTime.formatIso(v),
          decode: (raw) => DateTime.unsafeFromDate(new Date(raw)),
          default: () => DateTime.unsafeNow()
        }
      }
    }
  `;
    // Use unique config path to avoid caching issues
    const configPath = '/test/import-matching/macroforge.config.js';

    test('matches foreign type from effect import', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const code = `
      import type { DateTime } from 'effect';

      /** @derive(Default) */
      interface Event {
        startTime: DateTime.DateTime;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        assert.ok(
            result.code.includes('DateTime.unsafeNow()'),
            `Should match DateTime from effect import. Got: ${result.code}`
        );
    });

    test('matches foreign type from @effect/schema import', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const code = `
      import type { DateTime } from '@effect/schema';

      /** @derive(Default) */
      interface Event {
        startTime: DateTime.DateTime;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        assert.ok(
            result.code.includes('DateTime.unsafeNow()'),
            `Should match DateTime from @effect/schema import. Got: ${result.code}`
        );
    });

    test('ignores type from different library with same name', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const code = `
      import type { DateTime } from 'some-other-library';

      /** @derive(Encode) */
      interface Event {
        name: string;
        startTime: DateTime.DateTime;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        // Should NOT use the foreign type handler - falls back to generic handling
        // We can't know if some-other-library's DateTime has the right methods
        assert.ok(
            !result.code.includes('DateTime.formatIso'),
            `Should NOT use foreign type encode for different library. Got: ${result.code}`
        );
        // Should not have any errors - just ignore and let tsc catch issues downstream
        assert.ok(
            !result.diagnostics ||
                !result.diagnostics.some((d) => d.level === 'error'),
            `Should not error for types from other libraries. Got: ${
                JSON.stringify(result.diagnostics)
            }`
        );
    });

    test('does not match local types with same name as foreign type', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        // No import - this is a "local" type scenario
        const code = `
      /** @derive(Default) */
      interface Event {
        startTime: DateTime.DateTime;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        // Should NOT use the foreign type handler because we can't verify the import source
        // Falls back to generic handling
        assert.ok(
            !result.code.includes('DateTime.unsafeNow()'),
            `Should NOT match unimported type. Got: ${result.code}`
        );
        // Should not have any errors either
        assert.ok(
            !result.diagnostics ||
                !result.diagnostics.some((d) => d.level === 'error'),
            `Should not have errors for unimported types. Got: ${
                JSON.stringify(result.diagnostics)
            }`
        );
    });
});

// ============================================================================
// Field Decorator Override Tests
// ============================================================================

describe('Field decorators override foreign type config', () => {
    const configContent = `
    export default {
      foreignTypes: {
        "DateTime.DateTime": {
          from: ["effect"],
          encode: (v) => DateTime.formatIso(v),
          decode: (raw) => DateTime.unsafeFromDate(new Date(raw)),
          default: () => DateTime.unsafeNow()
        }
      }
    }
  `;
    const configPath = '/test/decorator-override/macroforge.config.js';

    test('encodeWith decorator overrides foreign type encode', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const code = `
      import type { DateTime } from 'effect';

      /** @derive(Encode) */
      interface Event {
        name: string;
        /** @endec({ encodeWith: (v) => v.toEpochMillis() }) */
        startTime: DateTime.DateTime;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        // Should use the explicit encodeWith, NOT the foreign type config
        assert.ok(
            result.code.includes('toEpochMillis'),
            `Should use explicit encodeWith decorator. Got: ${result.code}`
        );
        assert.ok(
            !result.code.includes('DateTime.formatIso'),
            `Should NOT use foreign type encode when encodeWith is specified. Got: ${result.code}`
        );
    });

    test('decodeWith decorator overrides foreign type decode', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const code = `
      import type { DateTime } from 'effect';

      /** @derive(Decode) */
      interface Event {
        name: string;
        /** @endec({ decodeWith: (raw) => DateTime.fromEpochMillis(raw) }) */
        startTime: DateTime.DateTime;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        // Should use the explicit decodeWith, NOT the foreign type config
        assert.ok(
            result.code.includes('fromEpochMillis'),
            `Should use explicit decodeWith decorator. Got: ${result.code}`
        );
        assert.ok(
            !result.code.includes('DateTime.unsafeFromDate'),
            `Should NOT use foreign type decode when decodeWith is specified. Got: ${result.code}`
        );
    });

    test('default decorator overrides foreign type default', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const code = `
      import type { DateTime } from 'effect';

      /** @derive(Default) */
      interface Event {
        name: string;
        /** @default(() => DateTime.make(2024, 1, 1)) */
        startTime: DateTime.DateTime;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        // Should use the explicit default, NOT the foreign type config
        assert.ok(
            result.code.includes('DateTime.make(2024, 1, 1)'),
            `Should use explicit default decorator. Got: ${result.code}`
        );
        assert.ok(
            !result.code.includes('DateTime.unsafeNow'),
            `Should NOT use foreign type default when default decorator is specified. Got: ${result.code}`
        );
    });
});

// ============================================================================
// Foreign Type Alias Tests
// ============================================================================

describe('Foreign type aliases', () => {
    const configContent = `
    export default {
      foreignTypes: {
        "DateTime.DateTime": {
          from: ["effect"],
          aliases: [
            { name: "DateTime", from: "effect/DateTime" },
            { name: "MyDateTime", from: "my-effect-wrapper" }
          ],
          encode: (v) => DateTime.formatIso(v),
          decode: (raw) => DateTime.unsafeFromDate(new Date(raw)),
          default: () => DateTime.unsafeNow()
        }
      }
    }
  `;
    const configPath = '/test/aliases/macroforge.config.js';

    test('loadConfig parses aliases from config', () => {
        clearConfigCache();
        const result = loadConfig(configContent, configPath);

        assert.equal(result.hasForeignTypes, true, 'Should have foreign types');
        assert.equal(result.foreignTypeCount, 1, 'Should have 1 foreign type');
    });

    test('matches foreign type via alias name and source', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const code = `
      import type { DateTime } from 'effect/DateTime';

      /** @derive(Encode) */
      interface Event {
        name: string;
        startTime: DateTime;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        // Should use the foreign type encode function via alias match
        assert.ok(
            result.code.includes('DateTime.formatIso'),
            `Should match via alias and use foreign type encode. Got: ${result.code}`
        );
    });

    test('matches foreign type via different alias', () => {
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

        const result = expandSync(code, 'test.ts', { configPath });

        // Should use the foreign type default function via MyDateTime alias
        assert.ok(
            result.code.includes('DateTime.unsafeNow()'),
            `Should match MyDateTime alias and use foreign type default. Got: ${result.code}`
        );
    });

    test('alias does not match when import source differs', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const code = `
      import type { DateTime } from 'some-other-library';

      /** @derive(Encode) */
      interface Event {
        name: string;
        startTime: DateTime;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        // Should NOT use foreign type - import source doesn't match main or aliases
        assert.ok(
            !result.code.includes('DateTime.formatIso'),
            `Should NOT match when import source doesn't match any alias. Got: ${result.code}`
        );
    });

    test('primary from still works alongside aliases', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const code = `
      import type { DateTime } from 'effect';

      /** @derive(Encode) */
      interface Event {
        name: string;
        startTime: DateTime.DateTime;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        // Should still work with the primary from: ["effect"]
        assert.ok(
            result.code.includes('DateTime.formatIso'),
            `Primary from should still work. Got: ${result.code}`
        );
    });
});

// ============================================================================
// Local Import Alias Tracking Tests
// ============================================================================

describe('Local import alias tracking', () => {
    const configContent = `
    export default {
      foreignTypes: {
        "Option": {
          from: ["effect/Option"],
          encode: (v) => Option.getOrNull(v),
          decode: (raw) => raw === null ? Option.none() : Option.some(raw),
          default: () => Option.none()
        }
      }
    }
  `;
    const configPath = '/test/local-alias/macroforge.config.js';

    test('tracks local import alias (import { Option as EffectOption })', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const code = `
      import type { Option as EffectOption } from 'effect/Option';

      /** @derive(Encode, Decode, Default) */
      interface UserPreferences {
        name: string;
        theme: EffectOption<string>;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        // Should recognize EffectOption as the foreign type Option from effect/Option
        assert.ok(
            result.code.includes('Option.getOrNull'),
            `Should use foreign type encode for aliased import. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('Option.none()'),
            `Should use foreign type default/decode for aliased import. Got: ${result.code}`
        );
    });

    test('tracks multiple local aliases in same file', () => {
        clearConfigCache();
        const multiAliasConfig = `
      export default {
        foreignTypes: {
          "Option": {
            from: ["effect/Option"],
            encode: (v) => Option.getOrNull(v),
            decode: (raw) => raw === null ? Option.none() : Option.some(raw),
            default: () => Option.none()
          },
          "DateTime.DateTime": {
            from: ["effect"],
            encode: (v) => DateTime.formatIso(v),
            decode: (raw) => DateTime.unsafeFromDate(new Date(raw)),
            default: () => DateTime.unsafeNow()
          }
        }
      }
    `;
        const multiConfigPath = '/test/multi-local-alias/macroforge.config.js';
        loadConfig(multiAliasConfig, multiConfigPath);

        const code = `
      import type { Option as MaybeValue } from 'effect/Option';
      import type { DateTime as EffectDateTime } from 'effect';

      /** @derive(Encode) */
      interface Event {
        title: string;
        description: MaybeValue<string>;
        startTime: EffectDateTime.DateTime;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath: multiConfigPath });

        // Both aliased types should be recognized
        assert.ok(
            result.code.includes('Option.getOrNull'),
            `Should use foreign type encode for MaybeValue alias. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('DateTime.formatIso'),
            `Should use foreign type encode for EffectDateTime alias. Got: ${result.code}`
        );
    });

    test('local alias does not affect other types with same name', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        // Import Option with alias, but also have a local Option type
        const code = `
      import type { Option as EffectOption } from 'effect/Option';

      // Local type with same base name
      type Option<T> = T | undefined;

      /** @derive(Encode) */
      interface Container {
        effectValue: EffectOption<string>;
        localValue: Option<number>;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        // EffectOption should use foreign type handler
        // Local Option should NOT (no import matches)
        assert.ok(
            result.code.includes('Option.getOrNull'),
            `Should use foreign type for EffectOption. Got: ${result.code}`
        );
    });

    test('handles deeply nested namespace (10 levels) with alias', () => {
        clearConfigCache();
        const deepConfig = `
      export default {
        foreignTypes: {
          "Deep.A.B.C.D.E.F.G.H.I.Type": {
            from: ["deep-module"],
            encode: (v) => Deep.encode(v),
            decode: (raw) => Deep.decode(raw),
            default: () => Deep.empty()
          }
        }
      }
    `;
        const deepConfigPath = '/test/deep-namespace/macroforge.config.js';
        loadConfig(deepConfig, deepConfigPath);

        const code = `
      import type { Deep as AliasedDeep } from 'deep-module';

      /** @derive(Encode, Default) */
      interface Container {
        value: AliasedDeep.A.B.C.D.E.F.G.H.I.Type;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath: deepConfigPath });

        // Should resolve AliasedDeep -> Deep and match the foreign type
        assert.ok(
            result.code.includes('Deep.encode'),
            `Should use foreign type encode for deeply nested aliased namespace. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('Deep.empty()'),
            `Should use foreign type default for deeply nested aliased namespace. Got: ${result.code}`
        );
    });
});

// ============================================================================
// Type-only import namespace generation tests
// ============================================================================

describe('Type-only import namespace generation', () => {
    const configContent = `
    export default {
      foreignTypes: {
        "DateTime.DateTime": {
          from: ["effect", "effect/DateTime"],
          aliases: [
            { name: "DateTime", from: "effect/DateTime" }
          ],
          encode: (v) => DateTime.formatIso(v),
          decode: (raw) => DateTime.unsafeFromDate(new Date(raw)),
          default: () => DateTime.unsafeNow()
        }
      }
    }
  `;
    const configPath = '/test/type-only-imports/macroforge.config.js';

    test('generates value import when type-only import is used', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        // Use 'import type' which is a type-only import
        const code = `
      import type { DateTime } from 'effect/DateTime';

      /** @derive(Encode, Default) */
      interface Event {
        startTime: DateTime;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        // Should generate a value import for DateTime with __mf_ prefix
        assert.ok(
            result.code.includes('import { DateTime as __mf_DateTime }'),
            `Should generate value import with alias. Got: ${result.code}`
        );

        // The generated code should use the aliased namespace
        assert.ok(
            result.code.includes('__mf_DateTime.formatIso'),
            `Should use aliased namespace in encode. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('__mf_DateTime.unsafeNow'),
            `Should use aliased namespace in default. Got: ${result.code}`
        );
    });

    test('does not generate import when value import already exists', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        // Use regular import (not type-only)
        const code = `
      import { DateTime } from 'effect/DateTime';

      /** @derive(Encode) */
      interface Event {
        startTime: DateTime;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        // Should NOT generate a duplicate import
        assert.ok(
            !result.code.includes('__mf_DateTime'),
            `Should not generate aliased import when value import exists. Got: ${result.code}`
        );

        // Should use the original namespace directly
        assert.ok(
            result.code.includes('DateTime.formatIso'),
            `Should use original namespace. Got: ${result.code}`
        );
    });

    test('handles mixed type-only and value imports', () => {
        clearConfigCache();
        const multiConfig = `
      export default {
        foreignTypes: {
          "DateTime.DateTime": {
            from: ["effect/DateTime"],
            aliases: [
              { name: "DateTime", from: "effect/DateTime" }
            ],
            encode: (v) => DateTime.formatIso(v),
            default: () => DateTime.unsafeNow()
          },
          "Option": {
            from: ["effect/Option"],
            encode: (v) => Option.getOrNull(v),
            default: () => Option.none()
          }
        }
      }
    `;
        const multiConfigPath = '/test/mixed-imports/macroforge.config.js';
        loadConfig(multiConfig, multiConfigPath);

        // DateTime is type-only, Option is value import
        const code = `
      import type { DateTime } from 'effect/DateTime';
      import { Option } from 'effect/Option';

      /** @derive(Encode, Default) */
      interface Event {
        startTime: DateTime;
        description: Option<string>;
      }
    `;

        const result = expandSync(code, 'test.ts', { configPath: multiConfigPath });

        // Should generate import for DateTime (type-only)
        assert.ok(
            result.code.includes('import { DateTime as __mf_DateTime }'),
            `Should generate import for type-only DateTime. Got: ${result.code}`
        );

        // Should NOT generate import for Option (already value import)
        assert.ok(
            !result.code.includes('__mf_Option'),
            `Should not generate import for Option (already value import). Got: ${result.code}`
        );

        // Should use aliased DateTime
        assert.ok(
            result.code.includes('__mf_DateTime.formatIso'),
            `Should use aliased DateTime. Got: ${result.code}`
        );

        // Should use original Option
        assert.ok(
            result.code.includes('Option.getOrNull'),
            `Should use original Option. Got: ${result.code}`
        );
    });

    test('handles namespaced type with standalone function in encode and namespaced function in decode', () => {
        clearConfigCache();
        const mixedExprConfig = `
      export default {
        foreignTypes: {
          "DateTime.DateTime": {
            from: ["effect/DateTime"],
            aliases: [
              { name: "DateTime", from: "effect/DateTime" }
            ],
            // encode uses DateTime namespace AND a standalone function (formatIsoString)
            encode: (v) => formatIsoString(DateTime.toDate(v)),
            // decode uses DateTime namespace with a method call
            decode: (raw) => DateTime.fromDate(new Date(raw)),
            default: () => DateTime.unsafeNow()
          }
        }
      }
    `;
        const mixedExprConfigPath = '/test/mixed-expr/macroforge.config.js';
        loadConfig(mixedExprConfig, mixedExprConfigPath);

        // Type-only import - needs generated value import
        const code = `
      import type { DateTime } from 'effect/DateTime';

      // Standalone function that doesn't need namespace import
      function formatIsoString(date: Date): string {
        return date.toISOString();
      }

      /** @derive(Encode, Decode, Default) */
      interface Event {
        startTime: DateTime;
      }
    `;

        const result = expandSync(code, 'test.ts', {
            configPath: mixedExprConfigPath
        });

        // Should generate import for DateTime (type-only)
        assert.ok(
            result.code.includes('import { DateTime as __mf_DateTime }'),
            `Should generate import for type-only DateTime. Got: ${result.code}`
        );

        // Encode: should use aliased DateTime namespace but keep standalone function as-is
        assert.ok(
            result.code.includes('__mf_DateTime.toDate'),
            `Encode should use aliased DateTime.toDate. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('formatIsoString('),
            `Encode should keep standalone formatIsoString function. Got: ${result.code}`
        );
        // Should NOT alias the standalone function
        assert.ok(
            !result.code.includes('__mf_formatIsoString'),
            `Should NOT alias standalone function. Got: ${result.code}`
        );

        // Decode: should use aliased DateTime namespace
        assert.ok(
            result.code.includes('__mf_DateTime.fromDate'),
            `Decode should use aliased DateTime.fromDate. Got: ${result.code}`
        );

        // Default: should use aliased DateTime namespace
        assert.ok(
            result.code.includes('__mf_DateTime.unsafeNow'),
            `Default should use aliased DateTime.unsafeNow. Got: ${result.code}`
        );
    });

    test('complex case: multiple namespaces, standalone functions, nested calls, and chained expressions', () => {
        clearConfigCache();
        const complexConfig = `
      export default {
        foreignTypes: {
          // Deep namespace: Effect.Data.DateTime.Zoned
          "Effect.Data.DateTime.Zoned": {
            from: ["effect"],
            aliases: [
              { name: "Zoned", from: "effect/DateTime" }
            ],
            // Complex encode: multiple namespace calls + standalone functions + chaining
            encode: (v) => JSON.stringify({
              iso: Effect.Data.DateTime.Zoned.format(v, "iso"),
              epoch: Effect.Data.DateTime.Zoned.toEpochMillis(v),
              zone: Effect.Data.DateTime.Zoned.getZone(v).name
            }),
            // Complex decode: nested namespace calls + standalone + ternary
            decode: (raw) => {
              const parsed = JSON.parse(raw);
              return parsed.iso
                ? Effect.Data.DateTime.Zoned.fromString(parsed.iso)
                : Effect.Data.DateTime.Zoned.unsafeNow();
            },
            default: () => Effect.Data.DateTime.Zoned.unsafeNow()
          },
          // Another namespace at different depth
          "Option": {
            from: ["effect/Option"],
            encode: (v) => Option.isSome(v) ? Option.getOrThrow(v) : null,
            decode: (raw) => raw === null ? Option.none() : Option.some(raw),
            default: () => Option.none()
          },
          // Third namespace with method chaining in expressions
          "Duration.Duration": {
            from: ["effect"],
            aliases: [
              { name: "Duration", from: "effect/Duration" }
            ],
            // Chained namespace calls
            encode: (v) => Duration.toMillis(Duration.abs(v)),
            decode: (raw) => Duration.millis(Math.abs(raw)),
            default: () => Duration.zero
          }
        }
      }
    `;
        const complexConfigPath = '/test/complex-namespaces/macroforge.config.js';
        loadConfig(complexConfig, complexConfigPath);

        // All type-only imports to force namespace import generation
        const code = `
      import type { Effect } from 'effect';
      import type { Option } from 'effect/Option';
      import type { Duration } from 'effect/Duration';

      // Local standalone functions (should NOT be aliased)
      function validateTimestamp(ts: number): boolean {
        return ts > 0 && ts < Date.now();
      }

      function formatForDisplay(value: unknown): string {
        return String(value);
      }

      const processData = (x: unknown) => x;

      /** @derive(Encode, Decode, Default) */
      interface ComplexEvent {
        // Uses deep namespace Effect.Data.DateTime.Zoned
        timestamp: Effect.Data.DateTime.Zoned;
        // Uses Option namespace
        optionalNote: Option<string>;
        // Uses Duration namespace
        duration: Duration;
      }
    `;

        const result = expandSync(code, 'test.ts', {
            configPath: complexConfigPath
        });

        // === Import generation checks ===

        // Should generate import for Effect (deep namespace root)
        assert.ok(
            result.code.includes('import { Effect as __mf_Effect }'),
            `Should generate import for Effect namespace. Got: ${result.code}`
        );

        // Should generate import for Option
        assert.ok(
            result.code.includes('import { Option as __mf_Option }'),
            `Should generate import for Option namespace. Got: ${result.code}`
        );

        // Should generate import for Duration
        assert.ok(
            result.code.includes('import { Duration as __mf_Duration }'),
            `Should generate import for Duration namespace. Got: ${result.code}`
        );

        // === Deep namespace rewriting checks ===

        // Effect.Data.DateTime.Zoned should become __mf_Effect.Data.DateTime.Zoned
        assert.ok(
            result.code.includes('__mf_Effect.Data.DateTime.Zoned.format'),
            `Should rewrite deep namespace in encode. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('__mf_Effect.Data.DateTime.Zoned.toEpochMillis'),
            `Should rewrite deep namespace toEpochMillis. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('__mf_Effect.Data.DateTime.Zoned.getZone'),
            `Should rewrite deep namespace getZone. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('__mf_Effect.Data.DateTime.Zoned.fromString'),
            `Should rewrite deep namespace in decode. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('__mf_Effect.Data.DateTime.Zoned.unsafeNow'),
            `Should rewrite deep namespace in default/decode fallback. Got: ${result.code}`
        );

        // === Option namespace rewriting checks ===

        assert.ok(
            result.code.includes('__mf_Option.isSome'),
            `Should rewrite Option.isSome. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('__mf_Option.getOrThrow'),
            `Should rewrite Option.getOrThrow. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('__mf_Option.none'),
            `Should rewrite Option.none. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('__mf_Option.some'),
            `Should rewrite Option.some. Got: ${result.code}`
        );

        // === Duration namespace rewriting checks ===

        assert.ok(
            result.code.includes('__mf_Duration.toMillis'),
            `Should rewrite Duration.toMillis. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('__mf_Duration.abs'),
            `Should rewrite Duration.abs. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('__mf_Duration.millis'),
            `Should rewrite Duration.millis. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('__mf_Duration.zero'),
            `Should rewrite Duration.zero. Got: ${result.code}`
        );

        // === Standalone functions should NOT be aliased ===

        // JSON.stringify and JSON.parse are globals, should not be prefixed
        assert.ok(
            result.code.includes('JSON.stringify') &&
                !result.code.includes('__mf_JSON'),
            `Should NOT alias JSON global. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('JSON.parse') && !result.code.includes('__mf_JSON'),
            `Should NOT alias JSON.parse global. Got: ${result.code}`
        );

        // Math.abs is a global, should not be prefixed
        assert.ok(
            result.code.includes('Math.abs') && !result.code.includes('__mf_Math'),
            `Should NOT alias Math global. Got: ${result.code}`
        );

        // Local functions should remain unchanged (they don't use dot notation in calls)
        // validateTimestamp, formatForDisplay, processData should not appear with __mf_ prefix
        assert.ok(
            !result.code.includes('__mf_validateTimestamp'),
            `Should NOT alias local function validateTimestamp. Got: ${result.code}`
        );
        assert.ok(
            !result.code.includes('__mf_formatForDisplay'),
            `Should NOT alias local function formatForDisplay. Got: ${result.code}`
        );
        assert.ok(
            !result.code.includes('__mf_processData'),
            `Should NOT alias local function processData. Got: ${result.code}`
        );

        // === Original namespaces should NOT appear (all should be rewritten) ===

        // Check that unrewritten namespace patterns don't exist (except in type annotations)
        // The encode/decode code should not have bare Effect., Option., Duration.
        const codeWithoutTypes = result.code.replace(
            /:\s*\w+(\.\w+)*(<[^>]+>)?/g,
            ''
        ); // Remove type annotations

        // This is a rough check - the expressions should use __mf_ prefixed versions
        const hasUnrewrittenEffect = /[^_]Effect\.Data\.DateTime\.Zoned\./.test(
            codeWithoutTypes
        );
        assert.ok(
            !hasUnrewrittenEffect,
            `Should not have unrewritten Effect.Data.DateTime.Zoned in code. Got: ${result.code}`
        );
    });
});

// ============================================================================
// Foreign Type in Union Type Alias: Decode
// ============================================================================

describe('Foreign types in union type alias decoding', () => {
    const configContent = `
    export default {
      foreignTypes: {
        "DateTime.DateTime": {
          from: ["effect"],
          encode: (v) => DateTime.formatIso(v),
          decode: (raw) => DateTime.unsafeFromDate(new Date(raw)),
          default: () => DateTime.unsafeNow(),
          hasShape: (v) => typeof v === "string"
        },
        "BigDecimal.BigDecimal": {
          from: ["effect"],
          encode: (v) => BigDecimal.format(v),
          decode: (raw) => BigDecimal.fromString(String(raw)),
          default: () => BigDecimal.unsafeFromNumber(0),
          hasShape: (v) => typeof v === "string" || typeof v === "number"
        }
      }
    }
  `;
    const configPath = '/test/union-foreign-types/macroforge.config.js';

    test('union with foreign-only types uses hasShape for decoding', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const code = `
      import type { DateTime, BigDecimal } from 'effect';

      /** @derive(Decode) */
      type FlexibleValue = DateTime.DateTime | BigDecimal.BigDecimal;
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        // Should use the configured hasShape expressions (not generic object checks)
        assert.ok(
            result.code.includes('typeof v === "string"') ||
                result.code.includes("typeof v === 'string'"),
            `Should use hasShape expression from config for DateTime. Got: ${result.code}`
        );

        // Should use the configured decode expressions
        assert.ok(
            result.code.includes('DateTime.unsafeFromDate') ||
                result.code.includes('__mf_DateTime.unsafeFromDate'),
            `Should use foreign type decode for DateTime. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('BigDecimal.fromString') ||
                result.code.includes('__mf_BigDecimal.fromString'),
            `Should use foreign type decode for BigDecimal. Got: ${result.code}`
        );

        // Should NOT generate broken camelCase helper calls like
        // dateTime.dateTimeDecodeWithContext or bigDecimal.bigDecimalDecodeWithContext
        assert.ok(
            !result.code.includes('dateTime.dateTimeDecodeWithContext'),
            `Should NOT generate broken dotted decode function. Got: ${result.code}`
        );
        assert.ok(
            !result.code.includes('bigDecimal.bigDecimalDecodeWithContext'),
            `Should NOT generate broken dotted decode function. Got: ${result.code}`
        );
    });

    test('union with mixed regular and foreign types decodes correctly', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const code = `
      import type { DateTime } from 'effect';

      /** @derive(Decode) */
      type EventPayload = SuccessData | FailureData | DateTime.DateTime;
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        // Should have regular decode calls for SuccessData and FailureData
        assert.ok(
            result.code.includes('successDataDecodeWithContext'),
            `Should generate regular decode for SuccessData. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('failureDataDecodeWithContext'),
            `Should generate regular decode for FailureData. Got: ${result.code}`
        );

        // Should use foreign type decode for DateTime.DateTime
        assert.ok(
            result.code.includes('DateTime.unsafeFromDate') ||
                result.code.includes('__mf_DateTime.unsafeFromDate'),
            `Should use foreign type decode for DateTime in mixed union. Got: ${result.code}`
        );

        // Should NOT generate broken dateTime.dateTimeDecodeWithContext
        assert.ok(
            !result.code.includes('dateTime.dateTimeDecodeWithContext'),
            `Should NOT generate broken dotted identifier. Got: ${result.code}`
        );
    });

    test('union with foreign type and primitives decodes correctly', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const code = `
      import type { DateTime } from 'effect';

      /** @derive(Decode) */
      type MaybeDate = DateTime.DateTime | string | number;
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        // Should have hasShape check for the foreign type in the mixed branch
        assert.ok(
            result.code.includes('DateTime.unsafeFromDate') ||
                result.code.includes('__mf_DateTime.unsafeFromDate'),
            `Should use foreign type decode for DateTime in mixed union. Got: ${result.code}`
        );

        // Should have primitive checks
        assert.ok(
            result.code.includes('typeof value === "string"'),
            `Should have primitive string check. Got: ${result.code}`
        );
        assert.ok(
            result.code.includes('typeof value === "number"'),
            `Should have primitive number check. Got: ${result.code}`
        );
    });

    test('union hasShape function uses foreign type hasShape config', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const code = `
      import type { DateTime, BigDecimal } from 'effect';

      /** @derive(Decode) */
      type FlexibleValue = DateTime.DateTime | BigDecimal.BigDecimal;
    `;

        const result = expandSync(code, 'test.ts', { configPath });

        // Should generate a hasShape function for the union
        assert.ok(
            result.code.includes('flexibleValueHasShape'),
            `Should generate hasShape function for union. Got: ${result.code}`
        );

        // Should generate an is function for the union
        assert.ok(
            result.code.includes('flexibleValueIs'),
            `Should generate is function for union. Got: ${result.code}`
        );
    });

    test('foreign type without hasShape falls back to generic handling', () => {
        clearConfigCache();
        const noShapeConfig = `
      export default {
        foreignTypes: {
          "DateTime.DateTime": {
            from: ["effect"],
            encode: (v) => DateTime.formatIso(v),
            decode: (raw) => DateTime.unsafeFromDate(new Date(raw)),
            default: () => DateTime.unsafeNow()
          }
        }
      }
    `;
        const noShapeConfigPath = '/test/union-foreign-no-shape/macroforge.config.js';
        loadConfig(noShapeConfig, noShapeConfigPath);

        const code = `
      import type { DateTime } from 'effect';

      /** @derive(Decode) */
      type Value = DateTime.DateTime | RegularType;
    `;

        const result = expandSync(code, 'test.ts', {
            configPath: noShapeConfigPath
        });

        // Should still use foreign decode for __type-based dispatch
        assert.ok(
            result.code.includes('DateTime.unsafeFromDate') ||
                result.code.includes('__mf_DateTime.unsafeFromDate'),
            `Should still use foreign type decode for __type dispatch. Got: ${result.code}`
        );

        // Should NOT generate broken dateTime.dateTimeDecodeWithContext
        assert.ok(
            !result.code.includes('dateTime.dateTimeDecodeWithContext'),
            `Should NOT generate broken dotted identifier even without hasShape. Got: ${result.code}`
        );
    });

    test('loadConfig parses hasShape from config', () => {
        clearConfigCache();
        const result = loadConfig(configContent, configPath);

        assert.equal(result.hasForeignTypes, true, 'Should have foreign types');
        assert.equal(
            result.foreignTypeCount,
            2,
            'Should have 2 foreign types'
        );
    });
});
