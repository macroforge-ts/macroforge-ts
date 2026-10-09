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
// Handler import contract
// ============================================================================

// Generated code imports each foreign-type handler, by its export name, from
// `#macroforge/config` and calls it. The handler bodies stay in the expanded
// config, so no body text reaches the generated module.

function importsHandler(code, name) {
    return new RegExp(
        `import \\{[^}]*\\b${name}\\b[^}]*\\} from ["']#macroforge/config["']`
    ).test(code);
}

function assertCallsHandlers(code, ...names) {
    for (const name of names) {
        assert.ok(importsHandler(code, name), `\`${name}\` is imported. Got: ${code}`);
        assert.ok(code.includes(`${name}(`), `\`${name}\` is called. Got: ${code}`);
    }
}

function assertSkipsHandlers(code, ...names) {
    for (const name of names) {
        assert.ok(!code.includes(name), `\`${name}\` is not used. Got: ${code}`);
    }
}

function assertNoBodies(code, ...fragments) {
    for (const fragment of fragments) {
        assert.ok(
            !code.includes(fragment),
            `The handler body \`${fragment}\` stays in the config. Got: ${code}`
        );
    }
}

function assertNoErrors(result) {
    assert.ok(
        !result.diagnostics?.some((d) => d.level === 'error'),
        `Expansion should succeed. Got: ${JSON.stringify(result.diagnostics)}`
    );
}

const DATE_TIME_CONFIG = `
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

const DATE_TIME_BODIES = ['DateTime.formatIso', 'DateTime.unsafeFromDate', 'DateTime.unsafeNow'];

// ============================================================================
// Foreign Type Expansion Tests
// ============================================================================

describe('Foreign types in Default macro', () => {
    const configPath = '/test/default-macro/macroforge.config.js';

    test('default value calls the imported default handler', () => {
        clearConfigCache();
        loadConfig(DATE_TIME_CONFIG, configPath);

        const result = expandSync(
            `
      import type { DateTime } from 'effect';

      /** @derive(Default) */
      interface Event {
        name: string;
        startTime: DateTime.DateTime;
      }
    `,
            'test.ts',
            { configPath }
        );

        assertNoErrors(result);
        assertCallsHandlers(result.code, '__foreign__dateTimeDateTimeDefault');
        assertSkipsHandlers(
            result.code,
            '__foreign__dateTimeDateTimeEncode',
            '__foreign__dateTimeDateTimeDecode'
        );
        assertNoBodies(result.code, ...DATE_TIME_BODIES);
    });
});

describe('Foreign types in Encode macro', () => {
    const configPath = '/test/encode-macro/macroforge.config.js';

    test('encode calls the imported encode handler', () => {
        clearConfigCache();
        loadConfig(DATE_TIME_CONFIG, configPath);

        const result = expandSync(
            `
      import type { DateTime } from 'effect';

      /** @derive(Encode) */
      interface Event {
        name: string;
        startTime: DateTime.DateTime;
      }
    `,
            'test.ts',
            { configPath }
        );

        assertNoErrors(result);
        assertCallsHandlers(result.code, '__foreign__dateTimeDateTimeEncode');
        assertNoBodies(result.code, ...DATE_TIME_BODIES);
        assert.ok(
            !result.code.includes('dateTime.DateTime'),
            `Should not generate generic helper namespace. Got: ${result.code}`
        );
    });

    test('a near match is reported as a warning while expansion succeeds', () => {
        clearConfigCache();
        loadConfig(DATE_TIME_CONFIG, configPath);

        const result = expandSync(
            `
      import type { DateTime } from 'effect';

      /** @derive(Encode) */
      interface Event {
        startTime: DateTime;
      }
    `,
            'test.ts',
            { configPath }
        );

        assertNoErrors(result);
        assert.ok(
            result.diagnostics.some(
                (d) =>
                    d.level === 'warning' &&
                    d.message.includes("foreign type 'DateTime.DateTime'") &&
                    d.message.includes('{ name: "DateTime", from: "effect" }')
            ),
            `The near match should be reported with the alias that fixes it. Got: ${
                JSON.stringify(result.diagnostics)
            }`
        );
        assertSkipsHandlers(result.code, '#macroforge/config');
    });

    test('an object type alias calls the imported encode handler', () => {
        clearConfigCache();
        loadConfig(DATE_TIME_CONFIG, configPath);

        const result = expandSync(
            `
      import type { DateTime } from 'effect';

      /** @derive(Encode) */
      type Event = {
        name: string;
        startTime: DateTime.DateTime;
      };
    `,
            'test.ts',
            { configPath }
        );

        assertNoErrors(result);
        assertCallsHandlers(result.code, '__foreign__dateTimeDateTimeEncode');
        assertNoBodies(result.code, ...DATE_TIME_BODIES);
    });
});

describe('Foreign types in Decode macro', () => {
    const configPath = '/test/decode-macro/macroforge.config.js';

    test('decode calls the imported decode handler', () => {
        clearConfigCache();
        loadConfig(DATE_TIME_CONFIG, configPath);

        const result = expandSync(
            `
      import type { DateTime } from 'effect';

      /** @derive(Decode) */
      interface Event {
        name: string;
        startTime: DateTime.DateTime;
      }
    `,
            'test.ts',
            { configPath }
        );

        assertNoErrors(result);
        assertCallsHandlers(result.code, '__foreign__dateTimeDateTimeDecode');
        assertNoBodies(result.code, ...DATE_TIME_BODIES);
        assert.ok(
            !result.code.includes('dateTime.DateTime'),
            `Should not generate generic helper namespace. Got: ${result.code}`
        );
    });
});

describe('Foreign types with combined macros', () => {
    const configPath = '/test/combined-macros/macroforge.config.js';

    test('each macro imports its handler once', () => {
        clearConfigCache();
        loadConfig(DATE_TIME_CONFIG, configPath);

        const result = expandSync(
            `
      import type { DateTime } from 'effect';

      /** @derive(Default, Encode, Decode) */
      interface Event {
        name: string;
        startTime: DateTime.DateTime;
        endTime: DateTime.DateTime;
      }
    `,
            'test.ts',
            { configPath }
        );

        assertNoErrors(result);
        const handlers = [
            '__foreign__dateTimeDateTimeDefault',
            '__foreign__dateTimeDateTimeEncode',
            '__foreign__dateTimeDateTimeDecode'
        ];
        assertCallsHandlers(result.code, ...handlers);
        assertNoBodies(result.code, ...DATE_TIME_BODIES);
        for (const name of handlers) {
            const imports = result.code.match(
                new RegExp(`import \\{[^}]*\\b${name}\\b`, 'g')
            );
            assert.equal(imports?.length, 1, `\`${name}\` is imported once. Got: ${result.code}`);
        }
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
    const configPath = '/test/import-matching/macroforge.config.js';

    for (const source of ['effect', '@effect/schema']) {
        test(`matches the foreign type imported from ${source}`, () => {
            clearConfigCache();
            loadConfig(configContent, configPath);

            const result = expandSync(
                `
      import type { DateTime } from '${source}';

      /** @derive(Default) */
      interface Event {
        startTime: DateTime.DateTime;
      }
    `,
                'test.ts',
                { configPath }
            );

            assertNoErrors(result);
            assertCallsHandlers(result.code, '__foreign__dateTimeDateTimeDefault');
        });
    }

    test('ignores type from different library with same name', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const result = expandSync(
            `
      import type { DateTime } from 'some-other-library';

      /** @derive(Encode) */
      interface Event {
        name: string;
        startTime: DateTime.DateTime;
      }
    `,
            'test.ts',
            { configPath }
        );

        // Falls back to generic handling and leaves the type to tsc.
        assertNoErrors(result);
        assertSkipsHandlers(result.code, '__foreign__', '#macroforge/config');
    });

    test('does not match local types with same name as foreign type', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const result = expandSync(
            `
      /** @derive(Default) */
      interface Event {
        startTime: DateTime.DateTime;
      }
    `,
            'test.ts',
            { configPath }
        );

        assertNoErrors(result);
        assertSkipsHandlers(result.code, '__foreign__', '#macroforge/config');
    });
});

// ============================================================================
// Field Decorator Override Tests
// ============================================================================

describe('Field decorators override foreign type config', () => {
    const configPath = '/test/decorator-override/macroforge.config.js';

    test('encodeWith decorator overrides foreign type encode', () => {
        clearConfigCache();
        loadConfig(DATE_TIME_CONFIG, configPath);

        const result = expandSync(
            `
      import type { DateTime } from 'effect';

      /** @derive(Encode) */
      interface Event {
        name: string;
        /** @endec({ encodeWith: (v) => v.toEpochMillis() }) */
        startTime: DateTime.DateTime;
      }
    `,
            'test.ts',
            { configPath }
        );

        assertNoErrors(result);
        assert.ok(
            result.code.includes('toEpochMillis'),
            `Should use explicit encodeWith decorator. Got: ${result.code}`
        );
        assertSkipsHandlers(result.code, '__foreign__', '#macroforge/config');
    });

    test('decodeWith decorator overrides foreign type decode', () => {
        clearConfigCache();
        loadConfig(DATE_TIME_CONFIG, configPath);

        const result = expandSync(
            `
      import type { DateTime } from 'effect';

      /** @derive(Decode) */
      interface Event {
        name: string;
        /** @endec({ decodeWith: (raw) => DateTime.fromEpochMillis(raw) }) */
        startTime: DateTime.DateTime;
      }
    `,
            'test.ts',
            { configPath }
        );

        assertNoErrors(result);
        assert.ok(
            result.code.includes('fromEpochMillis'),
            `Should use explicit decodeWith decorator. Got: ${result.code}`
        );
        assertSkipsHandlers(result.code, '__foreign__', '#macroforge/config');
    });

    test('default decorator overrides foreign type default', () => {
        clearConfigCache();
        loadConfig(DATE_TIME_CONFIG, configPath);

        const result = expandSync(
            `
      import type { DateTime } from 'effect';

      /** @derive(Default) */
      interface Event {
        name: string;
        /** @default(() => DateTime.make(2024, 1, 1)) */
        startTime: DateTime.DateTime;
      }
    `,
            'test.ts',
            { configPath }
        );

        assertNoErrors(result);
        assert.ok(
            result.code.includes('DateTime.make(2024, 1, 1)'),
            `Should use explicit default decorator. Got: ${result.code}`
        );
        assertSkipsHandlers(result.code, '__foreign__', '#macroforge/config');
    });

    test('an override on one field leaves the handler on another', () => {
        clearConfigCache();
        loadConfig(DATE_TIME_CONFIG, configPath);

        const result = expandSync(
            `
      import type { DateTime } from 'effect';

      /** @derive(Encode) */
      interface Event {
        /** @endec({ encodeWith: (v) => v.toEpochMillis() }) */
        startTime: DateTime.DateTime;
        endTime: DateTime.DateTime;
      }
    `,
            'test.ts',
            { configPath }
        );

        assertNoErrors(result);
        assert.ok(result.code.includes('toEpochMillis'), `Got: ${result.code}`);
        assertCallsHandlers(result.code, '__foreign__dateTimeDateTimeEncode');
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

    // Every alias resolves to the one foreign type, so to its export names.
    const matches = [
        [
            'an alias name and source',
            `import type { DateTime } from 'effect/DateTime';`,
            'DateTime',
            'Encode'
        ],
        [
            'a different alias',
            `import type { MyDateTime } from 'my-effect-wrapper';`,
            'MyDateTime',
            'Default'
        ],
        [
            'the primary from alongside aliases',
            `import type { DateTime } from 'effect';`,
            'DateTime.DateTime',
            'Encode'
        ]
    ];
    for (const [label, importLine, typeName, macro] of matches) {
        test(`matches via ${label}`, () => {
            clearConfigCache();
            loadConfig(configContent, configPath);

            const result = expandSync(
                `
      ${importLine}

      /** @derive(${macro}) */
      interface Event {
        name: string;
        startTime: ${typeName};
      }
    `,
                'test.ts',
                { configPath }
            );

            assertNoErrors(result);
            assertCallsHandlers(result.code, `__foreign__dateTimeDateTime${macro}`);
        });
    }

    test('alias does not match when import source differs', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const result = expandSync(
            `
      import type { DateTime } from 'some-other-library';

      /** @derive(Encode) */
      interface Event {
        name: string;
        startTime: DateTime;
      }
    `,
            'test.ts',
            { configPath }
        );

        assertSkipsHandlers(result.code, '__foreign__', '#macroforge/config');
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

        const result = expandSync(
            `
      import type { Option as EffectOption } from 'effect/Option';

      /** @derive(Encode, Decode, Default) */
      interface UserPreferences {
        name: string;
        theme: EffectOption<string>;
      }
    `,
            'test.ts',
            { configPath }
        );

        assertNoErrors(result);
        assertCallsHandlers(
            result.code,
            '__foreign__optionEncode',
            '__foreign__optionDecode',
            '__foreign__optionDefault'
        );
        assertNoBodies(result.code, 'Option.getOrNull', 'Option.none()');
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

        const result = expandSync(
            `
      import type { Option as MaybeValue } from 'effect/Option';
      import type { DateTime as EffectDateTime } from 'effect';

      /** @derive(Encode) */
      interface Event {
        title: string;
        description: MaybeValue<string>;
        startTime: EffectDateTime.DateTime;
      }
    `,
            'test.ts',
            { configPath: multiConfigPath }
        );

        assertNoErrors(result);
        assertCallsHandlers(
            result.code,
            '__foreign__optionEncode',
            '__foreign__dateTimeDateTimeEncode'
        );
    });

    test('local alias does not affect other types with same name', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const result = expandSync(
            `
      import type { Option as EffectOption } from 'effect/Option';

      type Option<T> = T | undefined;

      /** @derive(Encode) */
      interface Container {
        effectValue: EffectOption<string>;
        localValue: Option<number>;
      }
    `,
            'test.ts',
            { configPath }
        );

        assertNoErrors(result);
        const calls = result.code.match(/__foreign__optionEncode\(/g);
        assert.equal(
            calls?.length,
            1,
            `Only the imported Option uses the handler. Got: ${result.code}`
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

        const result = expandSync(
            `
      import type { Deep as AliasedDeep } from 'deep-module';

      /** @derive(Encode, Default) */
      interface Container {
        value: AliasedDeep.A.B.C.D.E.F.G.H.I.Type;
      }
    `,
            'test.ts',
            { configPath: deepConfigPath }
        );

        assertNoErrors(result);
        assertCallsHandlers(
            result.code,
            '__foreign__deepABCDEFGHITypeEncode',
            '__foreign__deepABCDEFGHITypeDefault'
        );
        assertNoBodies(result.code, 'Deep.encode', 'Deep.empty');
    });
});

// ============================================================================
// Handlers carry their own imports
// ============================================================================

// A handler resolves the names it reads in the config module, so the
// generated module never needs a value import of the foreign type's
// namespace, whether the user imported it as a type or a value.
describe('Handlers need no value imports in the generated module', () => {
    const configContent = `
    export default {
      foreignTypes: {
        "DateTime.DateTime": {
          from: ["effect", "effect/DateTime"],
          aliases: [
            { name: "DateTime", from: "effect/DateTime" }
          ],
          encode: (v) => formatIsoString(DateTime.toDate(v)),
          decode: (raw) => DateTime.fromDate(new Date(raw)),
          default: () => DateTime.unsafeNow()
        }
      }
    }
  `;
    const configPath = '/test/type-only-imports/macroforge.config.js';

    for (
        const importLine of [
            `import type { DateTime } from 'effect/DateTime';`,
            `import { DateTime } from 'effect/DateTime';`
        ]
    ) {
        test(`${importLine} gets no namespace import`, () => {
            clearConfigCache();
            loadConfig(configContent, configPath);

            const result = expandSync(
                `
      ${importLine}

      function formatIsoString(date: Date): string {
        return date.toISOString();
      }

      /** @derive(Encode, Decode, Default) */
      interface Event {
        startTime: DateTime;
      }
    `,
                'test.ts',
                { configPath }
            );

            assertNoErrors(result);
            assertCallsHandlers(
                result.code,
                '__foreign__dateTimeDateTimeEncode',
                '__foreign__dateTimeDateTimeDecode',
                '__foreign__dateTimeDateTimeDefault'
            );
            assert.ok(
                !result.code.includes('__mf_DateTime'),
                `No synthetic namespace alias. Got: ${result.code}`
            );
            assert.equal(
                result.code.match(/from ['"]effect\/DateTime['"]/g)?.length,
                1,
                `The user's import is the only one from effect/DateTime. Got: ${result.code}`
            );
            // The module's own formatIsoString is not the one the handler calls.
            assertNoBodies(result.code, 'DateTime.toDate', 'formatIsoString(DateTime');
        });
    }

    test('several foreign types each import their own handlers', () => {
        clearConfigCache();
        const complexConfig = `
      export default {
        foreignTypes: {
          "Effect.Data.DateTime.Zoned": {
            from: ["effect"],
            aliases: [
              { name: "Zoned", from: "effect/DateTime" }
            ],
            encode: (v) => JSON.stringify({
              iso: Effect.Data.DateTime.Zoned.format(v, "iso"),
              zone: Effect.Data.DateTime.Zoned.getZone(v).name
            }),
            decode: (raw) => {
              const parsed = JSON.parse(raw);
              return parsed.iso
                ? Effect.Data.DateTime.Zoned.fromString(parsed.iso)
                : Effect.Data.DateTime.Zoned.unsafeNow();
            },
            default: () => Effect.Data.DateTime.Zoned.unsafeNow()
          },
          "Option": {
            from: ["effect/Option"],
            encode: (v) => Option.isSome(v) ? Option.getOrThrow(v) : null,
            decode: (raw) => raw === null ? Option.none() : Option.some(raw),
            default: () => Option.none()
          },
          "Duration.Duration": {
            from: ["effect"],
            aliases: [
              { name: "Duration", from: "effect/Duration" }
            ],
            encode: (v) => Duration.toMillis(Duration.abs(v)),
            decode: (raw) => Duration.millis(Math.abs(raw)),
            default: () => Duration.zero
          }
        }
      }
    `;
        const complexConfigPath = '/test/complex-namespaces/macroforge.config.js';
        loadConfig(complexConfig, complexConfigPath);

        const result = expandSync(
            `
      import type { Effect } from 'effect';
      import type { Option } from 'effect/Option';
      import type { Duration } from 'effect/Duration';

      /** @derive(Encode, Decode, Default) */
      interface ComplexEvent {
        timestamp: Effect.Data.DateTime.Zoned;
        optionalNote: Option<string>;
        duration: Duration;
      }
    `,
            'test.ts',
            { configPath: complexConfigPath }
        );

        assertNoErrors(result);
        for (const stem of ['effectDataDateTimeZoned', 'option', 'durationDuration']) {
            assertCallsHandlers(
                result.code,
                `__foreign__${stem}Encode`,
                `__foreign__${stem}Decode`,
                `__foreign__${stem}Default`
            );
        }
        assert.ok(
            !/__mf_(Effect|Option|Duration)\b/.test(result.code),
            `No synthetic namespace aliases. Got: ${result.code}`
        );
        assertNoBodies(
            result.code,
            'Zoned.format',
            'Zoned.getZone',
            'JSON.parse(raw)',
            'Option.getOrThrow',
            'Duration.abs',
            'Math.abs'
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

    function assertNoDottedHelpers(code) {
        for (
            const broken of [
                'dateTime.dateTimeDecodeWithContext',
                'bigDecimal.bigDecimalDecodeWithContext'
            ]
        ) {
            assert.ok(!code.includes(broken), `No dotted helper \`${broken}\`. Got: ${code}`);
        }
    }

    test('union with foreign-only types decodes through hasShape and decode handlers', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const result = expandSync(
            `
      import type { DateTime, BigDecimal } from 'effect';

      /** @derive(Decode) */
      type FlexibleValue = DateTime.DateTime | BigDecimal.BigDecimal;
    `,
            'test.ts',
            { configPath }
        );

        assertNoErrors(result);
        assertCallsHandlers(
            result.code,
            '__foreign__dateTimeDateTimeHasShape',
            '__foreign__dateTimeDateTimeDecode',
            '__foreign__bigDecimalBigDecimalHasShape',
            '__foreign__bigDecimalBigDecimalDecode'
        );
        assertNoBodies(
            result.code,
            'typeof v ===',
            'DateTime.unsafeFromDate',
            'BigDecimal.fromString'
        );
        assertNoDottedHelpers(result.code);
        assert.ok(result.code.includes('flexibleValueHasShape'), `Got: ${result.code}`);
        assert.ok(result.code.includes('flexibleValueIs'), `Got: ${result.code}`);
    });

    test('union with mixed regular and foreign types decodes correctly', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const result = expandSync(
            `
      import type { DateTime } from 'effect';

      /** @derive(Decode) */
      type EventPayload = SuccessData | FailureData | DateTime.DateTime;
    `,
            'test.ts',
            { configPath }
        );

        assertNoErrors(result);
        assert.ok(result.code.includes('successDataDecodeWithContext'), `Got: ${result.code}`);
        assert.ok(result.code.includes('failureDataDecodeWithContext'), `Got: ${result.code}`);
        assertCallsHandlers(result.code, '__foreign__dateTimeDateTimeDecode');
        assertSkipsHandlers(result.code, '__foreign__bigDecimal');
        assertNoDottedHelpers(result.code);
    });

    test('union with foreign type and primitives decodes correctly', () => {
        clearConfigCache();
        loadConfig(configContent, configPath);

        const result = expandSync(
            `
      import type { DateTime } from 'effect';

      /** @derive(Decode) */
      type MaybeDate = DateTime.DateTime | string | number;
    `,
            'test.ts',
            { configPath }
        );

        assertNoErrors(result);
        assertCallsHandlers(result.code, '__foreign__dateTimeDateTimeDecode');
        assert.ok(result.code.includes('typeof value === "string"'), `Got: ${result.code}`);
        assert.ok(result.code.includes('typeof value === "number"'), `Got: ${result.code}`);
    });

    test('foreign type without hasShape falls back to generic handling', () => {
        clearConfigCache();
        const noShapeConfigPath = '/test/union-foreign-no-shape/macroforge.config.js';
        loadConfig(DATE_TIME_CONFIG, noShapeConfigPath);

        const result = expandSync(
            `
      import type { DateTime } from 'effect';

      /** @derive(Decode) */
      type Value = DateTime.DateTime | RegularType;
    `,
            'test.ts',
            { configPath: noShapeConfigPath }
        );

        assertNoErrors(result);
        assertCallsHandlers(result.code, '__foreign__dateTimeDateTimeDecode');
        assertSkipsHandlers(result.code, '__foreign__dateTimeDateTimeHasShape');
        assertNoDottedHelpers(result.code);
    });

    test('loadConfig parses hasShape from config', () => {
        clearConfigCache();
        const result = loadConfig(configContent, configPath);

        assert.equal(result.hasForeignTypes, true, 'Should have foreign types');
        assert.equal(result.foreignTypeCount, 2, 'Should have 2 foreign types');
    });
});
