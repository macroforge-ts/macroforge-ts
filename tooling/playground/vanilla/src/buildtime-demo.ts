// Exercises the `@buildtime` evaluation pipeline from the browser side.
//
// The Vite plugin runs macroforge before transforming this module, so
// every `@buildtime` declaration below has been replaced with a TS
// literal by the time the browser downloads it. None of the runtime
// stubs in `@macroforge/core/buildtime` should ever fire.
import { buildtime } from '@macroforge/core/buildtime';
import type { BuildtimeDemoResult } from './playground-globals.ts';

// ---------------------------------------------------------------------
// Tier 1 — compile-time constants
// ---------------------------------------------------------------------

/** @buildtime */
const ANSWER = 6 * 7;

/** @buildtime */
const SCHEMA_HASH = buildtime.crypto.sha256('user-schema-v1');

/** @buildtime */
const APP_CONFIG = buildtime.fs.readJson('./buildtime-data.json');

/** @buildtime */
const CONSTANT_OBJECT = {
    thirteen: 13,
    label: 'compile-time',
    items: [1, 2, 3]
};

/** @buildtime */
const DERIVED_SUMMARY = `answer=${6 * 7}, hash=${
    buildtime.crypto
        .sha256('user-schema-v1')
        .slice(0, 8)
}`;

// Tier 1 can also use IIFEs for larger computation, staying within the
// tier's serialize-a-value contract. Here we generate a greeting table
// from a list at compile time.
/** @buildtime */
const GREETINGS = ((): Record<string, string> => {
    const names = ['alice', 'bob', 'cam'];
    const out: Record<string, string> = {};
    for (const n of names) out[n] = 'hello, ' + n;
    return out;
})();

// ---------------------------------------------------------------------
// Tier 3 — compile-time computed TypeScript type
// ---------------------------------------------------------------------

/** @buildtime */
export type UserId = 'string';

export function collectBuildtimeDemo(): BuildtimeDemoResult {
    let runtimeStubThrows = false;
    try {
        buildtime.time.unix();
    } catch {
        runtimeStubThrows = true;
    }

    return {
        answer: ANSWER,
        schemaHash: SCHEMA_HASH,
        appName: APP_CONFIG.app,
        appVersion: APP_CONFIG.version,
        routes: APP_CONFIG.routes,
        constantObject: CONSTANT_OBJECT,
        derivedSummary: DERIVED_SUMMARY,
        greetingAlice: GREETINGS.alice,
        greetingBob: GREETINGS.bob,
        greetingKeys: Object.keys(GREETINGS).sort(),
        runtimeStubThrows
    };
}
