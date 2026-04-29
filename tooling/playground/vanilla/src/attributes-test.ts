// End-to-end fixture for the four Rust-inspired attribute macros:
// `@cfg`, `@deprecated`, `@mustUse`, `@nonExhaustive`.
//
// `macroforge.config.ts` sets `cfg.features = ['playground']` and
// `cfg.target = 'web'`. With those flags:
//
// - `@cfg({ feature: 'playground' })`  ⇒  declaration kept
// - `@cfg({ feature: 'never' })`       ⇒  declaration stripped (export becomes undefined)
// - `@cfg({ target: 'web' })`          ⇒  kept
// - `@cfg({ target: 'node' })`         ⇒  stripped
//
// `@deprecated` rewrites the JSDoc to a tsc-readable form. The `.expanded.ts`
// snapshot proves the rewrite; runtime can only confirm the function still
// exists since JSDoc isn't reflected.
//
// `@mustUse` only flags discarded call sites. We consume the return below so
// no diagnostic fires — the playground stays buildable.
//
// `@nonExhaustive` brands a type alias's RHS at type-level only. The runtime
// artifact is just the type alias declaration; the brand intersection lives
// in the snapshot.

/** @cfg({ feature: 'playground' }) */
export function keptByPlayground(): string {
  return "kept-by-feature";
}

/** @cfg({ feature: 'never-defined-anywhere' }) */
export function strippedByMissingFeature(): string {
  return "should-be-stripped";
}

/** @cfg({ target: 'web' }) */
export function keptByWebTarget(): string {
  return "kept-by-target";
}

/** @cfg({ target: 'node' }) */
export function strippedByNodeTarget(): string {
  return "should-be-stripped";
}

/** @deprecated('use renderV2 instead', { since: '0.3.0' }) */
export function renderV1(): string {
  return "render-v1";
}

/** @mustUse('connection handle must be closed') */
export function openConnection(): { close: () => void } {
  return { close() {} };
}

/** @nonExhaustive */
export type ServerStatus = "green" | "yellow" | "red";

// Used to verify the type still typechecks after the brand intersection
// added by `@nonExhaustive`. External code constructing a `ServerStatus`
// must cast through the brand — that's the whole point of the brand.
export const exampleStatus = "green" as ServerStatus;

// Consume the @mustUse return value so no diagnostic fires at build time.
const handle = openConnection();
handle.close();
