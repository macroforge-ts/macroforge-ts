// Svelte-side demonstration of the `@buildtime` evaluation pipeline.
//
// The macroforge Vite plugin runs on this module during dev + build,
// so every `@buildtime` declaration here is replaced with a TS literal
// before the browser sees it. The `buildtime` import from
// `macroforge/buildtime` is a runtime stub that throws if actually
// called — a useful sanity check for the Playwright harness.
import { buildtime } from 'macroforge/buildtime';

/** @buildtime */
const ANSWER = 6 * 7;

/** @buildtime */
const SCHEMA_HASH = buildtime.crypto.sha256('svelte-schema-v1');

/** @buildtime */
const APP_CONFIG = buildtime.fs.readJson('./buildtime-data.json') as {
    app: string;
    version: string;
    features: string[];
};

/** @buildtime */
const CONSTANT_LIST = [1, 2, 3, 5, 8, 13].map((n) => n * 2);

// A derived summary that does its own compile-time I/O + hashing.
// Each @buildtime block runs in its own sandbox and cannot see
// names bound by sibling @buildtime blocks, so we re-read the file
// and re-hash here rather than reference ANSWER / APP_CONFIG.
/** @buildtime */
const DERIVED_SUMMARY = `app=${
    (buildtime.fs.readJson('./buildtime-data.json') as { version: string })
        .version
}, short=${buildtime.crypto.sha256('svelte-schema-v1').slice(0, 8)}`;

/** @buildtime */
type RouteId = 'string';

export interface SvelteBuildtimeResult {
    answer: number;
    schemaHash: string;
    appName: string;
    appVersion: string;
    features: string[];
    constantList: number[];
    derivedSummary: string;
    runtimeStubThrows: boolean;
    routeIdTag: RouteId;
}

export function collectSvelteBuildtime(): SvelteBuildtimeResult {
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
        features: APP_CONFIG.features,
        constantList: CONSTANT_LIST,
        derivedSummary: DERIVED_SUMMARY,
        runtimeStubThrows,
        routeIdTag: 'svelte-route-placeholder' as RouteId
    };
}
