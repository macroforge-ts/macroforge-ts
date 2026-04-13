// Exercises the `@buildtime` evaluation pipeline from the browser side.
//
// The Vite plugin runs macroforge before transforming this module, so
// every `@buildtime` declaration below has been replaced with a TS
// literal by the time the browser downloads it. None of the runtime
// stubs in `macroforge/buildtime` should ever fire.
import { buildtime } from "macroforge/buildtime";

// ---------------------------------------------------------------------
// Tier 1 — compile-time constants
// ---------------------------------------------------------------------

/** @buildtime */
const ANSWER = 6 * 7;

/** @buildtime */
const SCHEMA_HASH = buildtime.crypto.sha256("user-schema-v1");

/** @buildtime */
const APP_CONFIG = buildtime.fs.readJson("./buildtime-data.json") as {
  app: string;
  version: string;
  routes: string[];
};

/** @buildtime */
const CONSTANT_OBJECT = {
  thirteen: 13,
  label: "compile-time",
  items: [1, 2, 3],
};

/** @buildtime */
const DERIVED_SUMMARY = `answer=${6 * 7}, hash=${
  buildtime.crypto
    .sha256("user-schema-v1")
    .slice(0, 8)
}`;

// Tier 1 can also use IIFEs for larger computation, staying within the
// tier's serialize-a-value contract. Here we generate a greeting table
// from a list at compile time.
/** @buildtime */
const GREETINGS = ((): Record<string, string> => {
  const names = ["alice", "bob", "cam"];
  const out: Record<string, string> = {};
  for (const n of names) out[n] = "hello, " + n;
  return out;
})();

// ---------------------------------------------------------------------
// Tier 3 — compile-time computed TypeScript type
// ---------------------------------------------------------------------

/** @buildtime */
type UserId = "string";

// ---------------------------------------------------------------------
// Result payload for main.ts + Playwright.
// ---------------------------------------------------------------------

export interface BuildtimeDemoResult {
  answer: number;
  schemaHash: string;
  appName: string;
  appVersion: string;
  routes: string[];
  constantObject: typeof CONSTANT_OBJECT;
  derivedSummary: string;
  greetingAlice: string;
  greetingBob: string;
  greetingKeys: string[];
  // Probe against the `macroforge/buildtime` runtime stub. If the
  // Vite plugin is running, every real `@buildtime` use is spliced
  // into a TS literal — but importing the module and calling any
  // function on it at runtime still throws. That's the contract.
  runtimeStubThrows: boolean;
  userIdTag: UserId;
}

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
    runtimeStubThrows,
    userIdTag: "user-id-placeholder" as UserId,
  };
}
