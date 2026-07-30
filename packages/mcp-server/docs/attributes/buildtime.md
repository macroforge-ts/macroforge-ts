# Buildtime Evaluation

`@buildtime` runs a piece of TypeScript **during the build** and replaces it with its result. Reading a schema file, hashing a manifest, or stamping a build time all become compile-time constants rather than runtime work.

```typescript
import { buildtime } from "macroforge/buildtime";

/** @buildtime */
const BUILT_AT = buildtime.time.iso();

/** @buildtime */
const SCHEMA = buildtime.fs.readJson("./schema.json");
```

After expansion the imports and the `buildtime` calls are gone — the output contains only literals:

```typescript
const BUILT_AT = "2026-07-21T10:30:00.000Z";
const SCHEMA = { "version": 1, "fields": [] };
```

Evaluation happens in a sandboxed JavaScript engine (Boa), so build-time code can't reach the network or write files.

## The three tiers

What `@buildtime` does depends on what it's attached to.

### Tier 1 — `const` expression

The expression is evaluated and its result serialized to a TypeScript literal.

```typescript
/** @buildtime */
const VERSION_HASH = buildtime.crypto.sha256(buildtime.fs.readText("./package.json"));
```

### Tier 2 — function

The function is called with no arguments. If it returns a **string**, that string is spliced into the output verbatim as source code — which is how you generate declarations. Any other return value is serialized as `const NAME = <literal>`.

```typescript
/** @buildtime */
function generateGuards() {
  const schema = buildtime.fs.readJson("./schema.json") as { fields: Field[] };
  return schema.fields
    .map((f) => `export function is${f.name}(v: unknown): v is ${f.type} { return typeof v === "${f.type}"; }`)
    .join("\n");
}
```

### Tier 3 — `type` alias

The expression must return a **string containing TypeScript type syntax**. Returning anything else is an error.

```typescript
/** @buildtime */
type SchemaKeys = (() => {
  const schema = buildtime.fs.readJson("./schema.json") as { fields: { name: string }[] };
  return schema.fields.map((f) => JSON.stringify(f.name)).join(" | ");
})();
```

Declarations must be **top-level**. `export` visibility is preserved.

## API

| Namespace | Members |
| --- | --- |
| `buildtime.fs` | `readText(path)`, `readJson(path)`, `exists(path)`, `listDir(path)` |
| `buildtime.crypto` | `sha256(input)`, `sha512(input)` |
| `buildtime.time` | `now()`, `unix()`, `iso()` |
| `buildtime.location` | `file`, `line`, `column` |
| `buildtime.env` | Allow-listed environment variables |
| `buildtime.flags` | `has(name)`, `get(name)` |

The same object is also exported as `$buildtime`. Calling any of these outside a build pass throws.

## Rebuild invalidation

Every file read through `buildtime.fs` is recorded and returned as `buildtimeDependencies` on the expansion result. The Vite plugin uses this to re-expand a module when a file it read at build time changes, so editing `schema.json` re-runs the generator without a manual restart.

## Sandbox configuration

Build-time code runs sandboxed. Configure it with a `buildtime` block:

```typescript
// macroforge.config.ts
export default {
  buildtime: {
    capabilities: {
      timeout: 5000,                              // ms
      maxHeap: 256,                               // MiB (advisory)
      filesystem: { read: ["src/**"], write: [] },
      env: ["NODE_ENV", "CI"],
      network: false
    },
    flags: { RELEASE: "1", CHANNEL: "beta" }
  }
};
```

Capability keys may also be written flat (`buildtime.timeout`) if you prefer a shorter config; the nested form is canonical because it's the path sandbox diagnostics point you at.

| Option | Default | Effect |
| --- | --- | --- |
| `timeout` | `5000` | Wall-clock budget in ms, **enforced** — an infinite loop fails the build rather than hanging it |
| `maxHeap` | `256` | Heap ceiling in MiB. Advisory: Boa exposes no memory-limit hook, so it's carried but not enforced |
| `filesystem.read` | `["**"]` | Globs readable via `buildtime.fs` |
| `filesystem.write` | `[]` | Reserved — no write API is exposed to sandboxed code |
| `env` | `[]` | Names exposed as `buildtime.env.NAME`; anything not listed is `undefined` |
| `network` | `false` | Reserved — no network API is exposed |
| `flags` | `{}` | Values returned by `buildtime.flags` |

A partial block only overrides what it names. Omitting `buildtime` entirely keeps every default.

## Environment variables

`buildtime.env` is deny-by-default: a variable is only readable when you name it in `env`.

```typescript
// macroforge.config.ts
export default { buildtime: { capabilities: { env: ["NODE_ENV"] } } };
```

```typescript
/** @buildtime */
const MODE = buildtime.env.NODE_ENV ?? "development";
```

## Build flags

`flags` gives you a build-time switch that doesn't depend on the process environment:

```typescript
// macroforge.config.ts
export default { buildtime: { flags: { CHANNEL: "beta" } } };
```

```typescript
/** @buildtime */
const CHANNEL = buildtime.flags.get("CHANNEL") ?? "stable";

/** @buildtime */
const IS_BETA = buildtime.flags.has("CHANNEL");
```

`get()` returns `undefined` for an unset flag, and `has()` returns `false`. Flag values are strings; booleans and numbers written in config are stringified.
