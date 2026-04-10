# Build-Time Evaluation (`@buildtime`)

## Context

macroforge-ts already operates at compile time — the Vite plugin intercepts source files, the Rust core expands `@derive` decorators, and patched TypeScript is emitted before the bundler ever sees it. But the existing system is **declarative**: you say *what* to derive (Debug, Clone, Serialize), not *how* to compute it. The expansion logic lives in Rust procedural macros, not in user-facing TypeScript.

This plan adds **`@buildtime` evaluation**: a way for users to write TypeScript code that runs at compile time, with the result spliced into the module. It's the missing primitive for compile-time codegen, environment-dependent constants, schema-driven generation, and dead-code elimination based on build flags.

The previous version of this document was an analysis exploring three tiers of ambition (constants, code generation, type manipulation). This revision is a **concrete implementation plan** for tiers 1 and 2. Tier 3 (type manipulation) remains deferred — TS's existing conditional types cover most of it, and the parts that aren't covered overlap heavily with what zod and runtime schema generators already do.

The plan also covers **how `@buildtime` integrates with the rest of macroforge**: with declarative macros (Phase 0 of the macro pipeline), with derive macros (Phase 1), and with the source map composition that flows through both.

## What `@buildtime` is

A function or const declaration annotated with `@buildtime` is **evaluated at compile time** in a sandboxed JavaScript context. The result becomes part of the module's source.

### Tier 1 — `@buildtime const` (compile-time constants)

```ts
/** @buildtime */
const API_VERSION = computeVersionFromPackageJson();

/** @buildtime */
const ROUTES = Object.keys(routeConfig).map(k => `/${k}`);

/** @buildtime */
const SCHEMA_HASH = hashSchema(UserSchema);

/** @buildtime */
const BUILT_AT = new Date().toISOString();
```

After expansion, these become literal values in the output:

```ts
const API_VERSION = "2.4.1";
const ROUTES      = ["/users", "/posts", "/settings"];
const SCHEMA_HASH = "a3f8c2d1";
const BUILT_AT    = "2026-04-10T14:32:11.000Z";
```

The expressions on the right-hand side run during compilation. They can call functions, read files, do computation. The framework serializes the result back to a TS literal and replaces the original expression.

Supported result types: `null`, `undefined`, `boolean`, `number`, `string`, `bigint`, plain object literals, arrays of any of the above (recursively). Unsupported: functions, classes, instances of anything with a constructor, anything with circular references, anything with `Symbol`s. Unsupported results produce a build error pointing at the `@buildtime` declaration.

### Tier 2 — `@buildtime function` (compile-time code generation)

```ts
/** @buildtime */
function generateValidators() {
  const schema = loadSchema("./user.schema.json");
  return schema.fields.map(f =>
    `export function validate_${f.name}(v: unknown): v is ${f.type} {
       return typeof v === "${f.jsType}";
     }`
  ).join("\n");
}
```

The function runs at compile time. Its return value (a string of TypeScript source) is **spliced into the module as if the user had written it**. The original function declaration is removed.

If the function returns something other than a string, the framework treats it as a Tier 1 const declaration: serialize the value, splice it as a literal. So `function f() { return 42; }` is equivalent to `const x = 42;`.

The function can take no arguments. It runs once per compilation unit.

### Tier 3 — `@buildtime type` (type manipulation)

**Deferred.** TypeScript's conditional types cover most use cases. The parts that don't fit are well-served by external schema → type generators (zod, kysely-codegen, sql-tag, etc.). Adding a third sublanguage to macroforge isn't worth the complexity. Revisit if a clear use case emerges that the existing options can't handle.

## Execution sandbox

The buildtime evaluator needs a JavaScript execution environment. The choice of backend depends on the delivery vehicle:

| Delivery vehicle | Backend |
|---|---|
| macroforge-ts via Vite plugin (NAPI) | Embedded V8 via [`v8`](https://crates.io/crates/v8) crate |
| macroforge-ts compiled to WASM | QuickJS via [`rquickjs`](https://crates.io/crates/rquickjs) |
| macroforge-ts ported into loopshot | loopshot's existing `JsRuntime` (deno_core) — spin up a child isolate per `@buildtime` evaluation |

To make this swappable, the buildtime evaluator is a **trait** with backend-specific implementations:

```rust
// crates/macroforge_ts/src/host/buildtime/sandbox.rs

pub trait BuildtimeSandbox: Send + Sync {
    fn name(&self) -> &'static str;

    /// Evaluate a TypeScript module in the sandbox and return the value of
    /// `__macroforgeResult` (a global the framework arranges).
    fn evaluate(
        &self,
        source: &str,
        path: &Path,
        opts: &SandboxOptions,
    ) -> Result<SandboxValue, SandboxError>;
}

pub struct SandboxOptions {
    /// Filesystem read paths the sandbox is allowed to access.
    pub fs_read: Vec<PathPattern>,
    /// Filesystem write paths (usually empty).
    pub fs_write: Vec<PathPattern>,
    /// Whether the sandbox can make network requests.
    pub network: bool,
    /// Maximum execution time before the evaluator kills the script.
    pub timeout: Duration,
    /// Maximum heap allocation.
    pub max_heap: usize,
}

pub enum SandboxValue {
    Null,
    Bool(bool),
    Number(f64),
    BigInt(i64),
    String(String),
    Array(Vec<SandboxValue>),
    Object(BTreeMap<String, SandboxValue>),
    /// For Tier 2: the function returned a string of TS source.
    SourceCode(String),
}

#[derive(Debug, thiserror::Error)]
pub enum SandboxError {
    #[error("script timed out after {duration:?}")]
    Timeout { duration: Duration },
    #[error("script exceeded heap limit of {limit} bytes")]
    OutOfMemory { limit: usize },
    #[error("script tried to read disallowed path {path}")]
    UnauthorizedRead { path: PathBuf },
    #[error("script returned an unsupported value: {kind}")]
    UnserializableResult { kind: String },
    #[error("script threw: {message}")]
    Threw { message: String, stack: String },
}
```

Three concrete implementations under `crates/macroforge_ts/src/host/buildtime/backends/`:

- `quickjs.rs` — `BuildtimeSandbox` impl using `rquickjs`. Default for the WASM build of macroforge.
- `v8.rs` — impl using the `v8` crate directly. Default for the NAPI build.
- `deno_core.rs` — impl using `deno_core::JsRuntime`. Used when macroforge is consumed by loopshot.

Each backend creates a fresh sandbox per evaluation (no state leaks across `@buildtime` declarations) and enforces the capability options.

## Sandbox capabilities

By default, the sandbox is **stricter than Node.js**:

- **No filesystem access** unless explicitly granted via config
- **No network access** unless explicitly granted via config
- **No environment variables** unless explicitly granted via config
- **No spawning subprocesses**
- **No native modules**
- **5-second default timeout** per evaluation (configurable)
- **256 MB default heap limit** (configurable)

The user enables capabilities in `macroforge.config.js`:

```js
// macroforge.config.js
export default {
  buildtime: {
    capabilities: {
      filesystem: {
        read: ["**/*.json", "**/*.schema.yaml", "src/**"],
        write: []
      },
      network: false,
      env: ["NODE_ENV", "API_BASE"],
      timeout: 10_000,
      maxHeap: 512 * 1024 * 1024,
    }
  }
};
```

The framework loads the config, builds a `SandboxOptions`, and passes it to every `evaluate` call. Each backend enforces the capabilities through its own primitives:

- QuickJS backend: implements a custom module loader that rejects unauthorized reads, intercepts `fetch` to gate network access
- V8 backend: same approach via the `v8` API's read/write callbacks
- deno_core backend: uses loopshot's existing capability system (which itself wraps WASI Preview 2)

## API access

Inside a `@buildtime` block, the user has access to a small standard library:

```ts
import { buildtime } from "macroforge/buildtime";

/** @buildtime */
const data = (() => {
  // File system (gated by config)
  const schema = buildtime.fs.readJson("./schema.json");
  const text   = buildtime.fs.readText("./template.txt");

  // Crypto (always allowed — pure)
  const hash   = buildtime.crypto.sha256(text);

  // Time
  const now    = buildtime.time.now();      // ISO string
  const stamp  = buildtime.time.unix();     // unix seconds

  // Build environment
  const env    = buildtime.env.NODE_ENV;    // gated by config
  const flag   = buildtime.flags.has("--release");

  // Source location
  const file   = buildtime.location.file;   // current source file
  const line   = buildtime.location.line;   // line of the @buildtime declaration

  return { hash, now, env, flag, file };
})();
```

The `macroforge/buildtime` package exposes these as proper TypeScript types so authors get autocomplete inside the sandbox. At runtime (in the sandbox), the imports resolve to native functions wired to the backend.

What's deliberately NOT in the API:

- Generic `fetch()` (would be a footgun for non-deterministic builds)
- `child_process` (no shell access)
- `os` (host info isn't reproducible across CI environments)
- Generic `process.env` (use `buildtime.env` with a config-declared allowlist instead)

The principle: **builds should be reproducible.** If a `@buildtime` declaration produces different output on different machines, the user gets debugging hell. Restricting the API surface to deterministic primitives prevents the obvious foot-shooting.

Network access is opt-in but discouraged with a config-time warning ("network access in @buildtime makes builds non-reproducible — consider committing the result"). Users who genuinely need it (e.g., generating types from a remote OpenAPI spec) can enable it; the warning makes it clear what they're trading.

## Dependency tracking

If a `@buildtime` declaration reads `schema.json`, the build system needs to know to re-evaluate when `schema.json` changes. Without this, incremental builds produce stale output.

The sandbox tracks every `buildtime.fs.readJson` / `readText` call and records the absolute path. After evaluation, the framework returns the file list alongside the result:

```rust
pub struct EvalResult {
    pub value: SandboxValue,
    pub dependencies: Vec<PathBuf>,   // files read during evaluation
    pub diagnostics: Vec<Diagnostic>,
}
```

The Vite plugin (and loopshot's dev server, when macroforge is ported) consume this list and feed it to the watcher:

```js
// In the Vite plugin's transform hook
const result = await rustTransformer.expandSync(code, id, opts);
for (const dep of result.buildtimeDependencies) {
  this.addWatchFile(dep);
}
```

When any tracked file changes, the framework re-runs the affected `@buildtime` declarations and patches the source again. The cache (which is keyed by source content + buildtime input hashes) automatically invalidates correctly.

For loopshot's dev server (Phase 6), the same mechanism feeds the file watcher. Same cache invalidation logic.

## Error reporting

When a `@buildtime` declaration throws, the error needs to map back to the original source line. The framework wraps the user's expression in a try/catch inside the sandbox:

```js
// What the sandbox actually executes
try {
  globalThis.__macroforgeResult = (() => {
    /* user's expression */
  })();
} catch (e) {
  globalThis.__macroforgeError = {
    message: e.message,
    stack: e.stack,
  };
}
```

If `__macroforgeError` is set after execution, the framework converts it to a diagnostic with the byte range of the original `@buildtime` declaration:

```
error: build-time evaluation failed
  --> src/config.ts:14:1
   |
14 | /** @buildtime */
15 | const API_VERSION = computeVersionFromPackageJson();
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = caused by: ENOENT: no such file or directory, open './package.json'
       at computeVersionFromPackageJson (src/config.ts:8:23)
       at <buildtime>:1:1
```

Stack frames inside the sandbox use a synthetic `<buildtime>` filename. Frames pointing back at the user's declared functions get rewritten to the real source paths via the existing macroforge source map plumbing.

## Interaction with macros

The `@buildtime` pre-pass runs **before** declarative macros and derive macros. This means:

1. **`@buildtime` evaluation happens first.** All buildtime declarations are evaluated and replaced with their results.
2. **Declarative macro expansion runs next.** Macros can reference buildtime constants (which by now are plain TS literals).
3. **Derive macro expansion runs last.** Derives can reference both buildtime constants and macro-generated code.

```
Source
  │
  ▼
[ @buildtime pre-pass ]    ← evaluates `@buildtime` declarations, replaces with results
  │
  ▼
[ Declarative macros ]     ← macros can now see buildtime constants as literals
  │
  ▼
[ Derive macros ]          ← derives can see both
  │
  ▼
Patched source
  │
  ▼
[ Codegen → JS ]
```

This ordering is deliberate. The reverse (macros first, buildtime second) creates a problem: a macro could expand to code that contains `@buildtime` declarations, and the buildtime pass would have to run again, possibly multiple times until fixpoint. Bounded but messy. Buildtime-first is simpler and covers all the use cases I can think of.

For declarative macros that want to **invoke buildtime code at expansion time** (e.g., a macro that takes a type and returns the schema as a constant), the macro definition can call `buildtime.evaluate(...)` directly via a special API. This is rare — most use cases are covered by `@buildtime const` declarations the user writes themselves — but the door is open.

## Interaction with reverse-monomorphization

`@buildtime` is the natural way to produce the **schema constants** that reverse-mono call sites pass to shared runtimes. Concretely:

```ts
const $serialize = macro({
  mode: "auto",
  expand: macro`($x:Expr) => /* per-call inline */`,
  runtime: `function __serialize(value, schema) { /* shared body */ }`,
  call: macro`($x:Expr) => __serialize($x, ${"buildtime.schemaOf($x.type)"})`,
});
```

The `${...}` interpolation inside the `call` template is itself a `@buildtime` evaluation: at expansion time, the framework runs `buildtime.schemaOf($x.type)` for each call site, gets back a JSON-serializable schema, and splices it as a literal array.

This composition is the unified design. Macroforge becomes:

- **Declarative macros** — pattern matching + body expansion
- **Reverse-mono modes** — dev expansion + prod sharing
- **`@buildtime` evaluation** — compile-time computation
- **Derive macros** — auto-generate impls from type info

All four mechanisms compose. A macro can use buildtime to compute its constants. Reverse-mono picks the right form per build mode. Derives can use buildtime to read schemas. The user only ever sees the surface API; the framework wires everything underneath.

## Implementation Plan

### Phase 1: Sandbox trait and QuickJS backend

**New module:** `crates/macroforge_ts/src/host/buildtime/`

```
buildtime/
  mod.rs                Public API: evaluate, BuildtimeSandbox trait
  sandbox.rs            Trait definition + SandboxOptions + SandboxValue + SandboxError
  capabilities.rs       Capability validation + path-pattern matching
  serialize.rs          Convert SandboxValue ↔ TS literal source
  api.rs                Native function bindings exposed to the sandbox (buildtime.fs.*, etc.)
  backends/
    mod.rs              Backend selection (cfg-gated)
    quickjs.rs          QuickJS implementation (default for wasm32 target)
```

**Key types** as shown in the sandbox section above.

**Cargo features:**

```toml
[features]
default = ["buildtime-quickjs"]
buildtime-quickjs    = ["dep:rquickjs"]
buildtime-v8         = ["dep:v8"]
buildtime-deno-core  = ["dep:deno_core"]   # for loopshot integration
```

The `buildtime-deno-core` feature is what loopshot turns on after the macroforge port lands. It uses loopshot's existing `JsRuntime` instead of spinning up a separate engine.

### Phase 2: Discovery and rewriting

**New module:** `crates/macroforge_ts/src/host/buildtime/discovery.rs`

Walks the OXC AST during the macroforge pre-pass looking for `@buildtime` JSDoc annotations. For each match:

1. Identify whether the next declaration is a `const` (Tier 1) or `function` (Tier 2)
2. Extract the declaration's source range
3. Build a small synthetic module wrapping the declaration's right-hand side (for const) or body (for function)
4. Hand it to the sandbox via `BuildtimeSandbox::evaluate`
5. Receive a `SandboxValue` (or a `SourceCode` for Tier 2 functions returning strings)
6. Serialize the result via `serialize::value_to_ts_source`
7. Produce a `Patch::Replace` for the declaration

The synthetic module looks like:

```js
import { buildtime } from "macroforge/buildtime";
globalThis.__macroforgeResult = (() => {
  /* user's expression goes here */
})();
```

After execution, the framework reads `__macroforgeResult` from the sandbox's globalThis and serializes it.

### Phase 3: TypeScript types package

**New files:**

- `crates/macroforge_ts/js/buildtime/index.d.ts`
- `crates/macroforge_ts/js/buildtime/index.mjs` (runtime stubs that throw — buildtime imports must be evaluated by macroforge, not at runtime)

Type definitions:

```ts
// crates/macroforge_ts/js/buildtime/index.d.ts

export const buildtime: {
  fs: {
    readText(path: string): string;
    readJson(path: string): unknown;
    exists(path: string): boolean;
    listDir(path: string): string[];
  };
  crypto: {
    sha256(input: string | Uint8Array): string;
    sha512(input: string | Uint8Array): string;
  };
  time: {
    now(): string;       // ISO 8601
    unix(): number;      // seconds
    iso(): string;       // alias for now()
  };
  env: Record<string, string | undefined>;
  flags: {
    has(flag: string): boolean;
    get(flag: string): string | undefined;
  };
  location: {
    file: string;
    line: number;
    column: number;
  };
};
```

The runtime `index.mjs` exports stubs that throw `"@buildtime APIs are only available inside @buildtime declarations evaluated by macroforge"`. This catches users who try to use them at runtime.

### Phase 4: Capability enforcement

**Module:** `crates/macroforge_ts/src/host/buildtime/capabilities.rs`

The sandbox-side glue that enforces config-declared capabilities:

```rust
pub struct CapabilitySet {
    fs_read: Vec<PathPattern>,
    fs_write: Vec<PathPattern>,
    network: bool,
    env_allow: Vec<String>,
    timeout: Duration,
    max_heap: usize,
}

impl CapabilitySet {
    pub fn check_read(&self, path: &Path) -> Result<(), CapabilityError>;
    pub fn check_write(&self, path: &Path) -> Result<(), CapabilityError>;
    pub fn check_env(&self, key: &str) -> Result<(), CapabilityError>;
    pub fn check_network(&self, url: &str) -> Result<(), CapabilityError>;
}
```

Each backend invokes these checks before performing the underlying operation. A failed check returns a `SandboxError::Unauthorized*` that bubbles up through the framework as a build-time diagnostic.

### Phase 5: Serializer (SandboxValue → TS literal)

**Module:** `crates/macroforge_ts/src/host/buildtime/serialize.rs`

```rust
pub fn value_to_ts_source(value: &SandboxValue) -> Result<String, SerializeError>
```

Walks the `SandboxValue` and emits valid TS literal syntax:

| Value | Output |
|---|---|
| `Null` | `null` |
| `Bool(true)` | `true` |
| `Number(3.14)` | `3.14` |
| `Number(NaN)` | error: `NaN cannot be serialized` |
| `BigInt(42)` | `42n` |
| `String("hi")` | `"hi"` (with proper escaping) |
| `Array([1, 2, 3])` | `[1, 2, 3]` |
| `Object({a: 1, b: 2})` | `{ a: 1, b: 2 }` |
| `SourceCode(text)` | `text` (verbatim — Tier 2 already gave us source) |

Strings get proper JSON escaping. Object keys that aren't valid TS identifiers get quoted. Numbers that aren't representable as decimal literals (NaN, Infinity) error out.

For Tier 2 (`SourceCode`), the framework just splices the returned text. The user is responsible for emitting valid TS — if it doesn't parse, the next OXC pass catches it and reports the error against the `@buildtime function` declaration.

### Phase 6: Pipeline integration

**Modified files:**

- `crates/macroforge_ts/src/lib.rs` — add buildtime detection to early bailout
- `crates/macroforge_ts/src/host/expand.rs` — add buildtime pre-pass before declarative macros

**Early bailout change:**

```rust
let has_derive    = code.contains("@derive");
let has_macro_def = code.contains("= macro`");
let has_macro_call = MACRO_CALL_RE.is_match(code);
let has_buildtime = code.contains("@buildtime");
if !has_derive && !has_macro_def && !has_macro_call && !has_buildtime {
    return Ok(ExpandResult::unchanged(code));
}
```

**Pipeline order in `expand_inner`:**

```
1. Parse to OXC AST
2. Lower to IR
3. NEW: @buildtime pre-pass
   3a. Discovery: find @buildtime declarations
   3b. For each declaration: build synthetic module, evaluate via sandbox
   3c. Serialize result, produce Patch::Replace
   3d. Track dependencies for incremental rebuild
4. Declarative macro pre-pass (if implemented; see plans/macro-rules.md)
5. EXISTING: Derive macro expansion
6. Apply patches → new source string
7. Codegen back to TS source
```

The `ExpandResult` returned from `expand_sync` gains a new field:

```rust
pub struct ExpandResult {
    pub code: String,
    pub types: Option<String>,
    pub metadata: Option<String>,
    pub diagnostics: Vec<MacroDiagnostic>,
    pub source_mapping: Option<SourceMappingResult>,
    pub buildtime_dependencies: Vec<PathBuf>,   // NEW
}
```

Vite plugin code:

```js
const result = await rustTransformer.expandSync(code, id, opts);
for (const dep of result.buildtimeDependencies) {
  this.addWatchFile(dep);
}
return { code: result.code, map: result.map };
```

### Phase 7: Cache integration

The existing macroforge cache (`.macroforge/cache/`) is keyed by source content hash. Buildtime evaluation breaks this assumption: the same source can produce different output depending on filesystem state.

**Solution**: extend the cache key to include the hash of every buildtime dependency.

```rust
pub struct CacheKey {
    pub source_hash: [u8; 32],
    pub buildtime_dep_hashes: BTreeMap<PathBuf, [u8; 32]>,  // NEW
    pub config_hash: [u8; 32],
    pub macroforge_version: String,
}
```

When checking the cache, the framework hashes every dependency and compares. If any dependency changed, the cache entry is invalidated. The first build is slower (compute dep hashes); subsequent builds are unchanged.

The dep hashes are stored alongside the cached entry so subsequent loads don't have to re-read the source files until they change on disk.

### Phase 8: Diagnostics + source maps

Buildtime errors get the same treatment as macro errors:

```rust
fn buildtime_error_to_diagnostic(err: SandboxError, span: SpanIR) -> MacroDiagnostic {
    MacroDiagnostic {
        severity: Severity::Error,
        message: format!("build-time evaluation failed: {}", err),
        span,
        category: DiagCategory::Buildtime,
        notes: vec![format!("caused by: {}", err.cause())],
    }
}
```

Stack frames inside the sandbox use the synthetic `<buildtime>` filename. Frames that reference user-declared functions get rewritten via the existing macroforge source map plumbing — the framework knows which TS file the buildtime block came from and can map line numbers back.

For the dev experience, users see:

```
error: build-time evaluation failed
  --> src/config.ts:14:1
   |
14 | /** @buildtime */
15 | const API_VERSION = computeVersionFromPackageJson();
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = caused by: ENOENT: no such file or directory, open './package.json'
       at computeVersionFromPackageJson (src/config.ts:8:23)
       at <buildtime>:1:1
```

## Key Files to Modify

| File | Change |
|---|---|
| `crates/macroforge_ts/src/lib.rs` | Update early bailout to check for `@buildtime` |
| `crates/macroforge_ts/src/host/expand.rs` | Insert `@buildtime` pre-pass before macro expansion |
| `crates/macroforge_ts/src/host/buildtime/` (NEW) | Sandbox trait, discovery, serializer, capabilities, backends |
| `crates/macroforge_ts/src/api_types.rs` | Add `buildtime_dependencies` field to `ExpandResult` |
| `crates/macroforge_ts/Cargo.toml` | Add `rquickjs` (default), optional `v8` and `deno_core` features |
| `crates/macroforge_ts/js/buildtime/` (NEW) | npm package: TS types + runtime stubs |
| `packages/vite-plugin/src/index.js` | Forward `buildtimeDependencies` to `addWatchFile` |
| `packages/shared/src/config.ts` | Parse `buildtime.capabilities` block from `macroforge.config.js` |

## Verification

### Unit tests

In `crates/macroforge_ts/src/host/buildtime/tests.rs`:

- **Sandbox basics**: evaluate `1 + 1`, get `Number(2)`. Evaluate `[1, 2, 3]`, get `Array([Number, Number, Number])`. Evaluate `({a: 1, b: "hi"})`, get `Object`.
- **Tier 1 const**: declare `/** @buildtime */ const X = 42;`, run the pre-pass, check the source has `const X = 42;` (no change since the value was already a literal).
- **Tier 1 with computation**: declare `/** @buildtime */ const X = 6 * 7;`, run the pre-pass, check the source becomes `const X = 42;`.
- **Tier 1 with object**: declare `/** @buildtime */ const X = { a: 1, b: [2, 3] };`, check serialization round-trip.
- **Tier 2 function**: declare a function returning a string of TS source, run the pre-pass, check the function declaration is replaced with the returned source.
- **Tier 2 with non-string return**: function returns a number, framework treats it as Tier 1 (serialize the value).
- **Capability rejection**: try to read a path not in the allowlist, expect `SandboxError::UnauthorizedRead`.
- **Timeout**: write `while (true) {}`, expect `SandboxError::Timeout`.
- **Out of memory**: allocate a huge array, expect `SandboxError::OutOfMemory`.
- **Unserializable result**: return a function, expect `SandboxError::UnserializableResult`.
- **Throw**: throw an Error, expect `SandboxError::Threw` with the message.
- **Dependency tracking**: read three files via `buildtime.fs.readJson`, check the result's dependency list contains all three.
- **Serializer round-trip**: every `SandboxValue` variant → TS source → re-parse → equal.

### Integration tests

In `crates/macroforge_ts/tests/buildtime/`:

- **Full pipeline**: a fixture file with a `@buildtime const` reading a sibling JSON file. Expect the built output to contain the JSON's content as a literal.
- **Dependency invalidation**: build the fixture, modify the JSON, build again, check the result is updated.
- **Cache hit**: build the fixture twice without changes, check the second build hits the cache.
- **Interaction with derives**: a `@buildtime` declaration produces a constant; a `@derive(Debug)` class references the constant. Both passes complete correctly.
- **Interaction with declarative macros** (when those land): a `@buildtime` produces a schema; a declarative macro reads the schema in its expansion. Verify the ordering.
- **Diagnostic quality**: a buildtime declaration that throws — check the diagnostic has the right span and includes the JS stack.

### Vite plugin tests

In `packages/vite-plugin/tests/`:

- **Watch mode**: start `vite dev`, modify a file referenced by `@buildtime`, expect the consumer module to reload with updated content.

## Order of execution

1. **Phase 1**: sandbox trait + QuickJS backend (~800 LOC). The largest single piece because QuickJS integration has the most boilerplate.
2. **Phase 2**: discovery + rewriting (~300 LOC).
3. **Phase 3**: TS types package (~100 LOC).
4. **Phase 4**: capability enforcement (~250 LOC).
5. **Phase 5**: serializer (~200 LOC).
6. **Phase 6**: pipeline integration (~150 LOC).
7. **Phase 7**: cache integration (~150 LOC).
8. **Phase 8**: diagnostics + source maps (~200 LOC).
9. **Phase 9** (later): V8 backend for the NAPI build path.
10. **Phase 10** (later): deno_core backend for loopshot integration.

Phases 1-8 ship a working `@buildtime` for the WASM build. The QuickJS backend is the right starting point because it works in WASM, native, and embedded contexts without needing a separate engine.

Phase 9 (V8) is a performance upgrade for users who want faster buildtime evaluation in NAPI builds. Phase 10 (deno_core) is the loopshot integration — uses loopshot's existing `JsRuntime` so there's no separate JS engine in the loopshot binary.

## Caveats worth flagging

1. **Reproducibility is a discipline, not a guarantee.** The framework restricts the API to deterministic primitives by default and warns on network access, but a user with `network: true` can write code that produces different output on different machines or different days. There's no way to enforce determinism without crippling the feature; the docs need to call this out clearly.
2. **QuickJS is slower than V8.** For tight buildtime loops with heavy computation, QuickJS will be the bottleneck. Plan: ship QuickJS as the default for portability, add V8 as an opt-in feature flag for performance-sensitive builds.
3. **Sandbox start-up cost.** Each `@buildtime` declaration spawns a fresh sandbox to prevent state leaks. This is ~10-50 ms per declaration with QuickJS. For files with many declarations, this adds up. Possible optimization: pool sandboxes within a single build pass and reset them between evaluations (QuickJS supports this via `JS_FreeContext`/`JS_NewContext`).
4. **Async in @buildtime.** A `@buildtime` declaration is a function or const, not an async function. The framework executes it synchronously and blocks waiting for the result. Async/await inside the body works as long as the top-level expression resolves before the timeout. There's no way to declare `@buildtime async function f() { ... }` and have the framework `await` it from outside — the sandbox runs the body to completion synchronously.
5. **Cross-file references inside @buildtime.** A `@buildtime` declaration in `a.ts` cannot reference functions defined in `b.ts` directly — each declaration runs in an isolated sandbox with no access to other modules. To share code, users must put helpers in `.buildtime.ts` files that the sandbox loads via the (gated) filesystem API. This is a deliberate restriction to keep evaluation cheap and predictable.
6. **No source maps for buildtime stack frames.** Inside the sandbox, errors point at the synthetic `<buildtime>` filename. Mapping these back to user source requires recording the original byte offsets when constructing the synthetic module. Doable but adds complexity. v1 ships without source maps inside the sandbox; users see line numbers in `<buildtime>` and have to correlate manually. v2 adds source map support.
7. **Cache invalidation on macroforge upgrade.** When the macroforge version changes, the cache key includes it, so all entries invalidate. This is correct but means the first build after a macroforge upgrade is slow. Mitigation: a `--keep-cache-across-versions` flag for users who want to take the risk.

## Relationship to other macroforge features

| Feature | Interaction with `@buildtime` |
|---|---|
| Derive macros | `@buildtime` runs first; derives can reference buildtime constants as if they were hand-written literals |
| Declarative macros (`plans/macro-rules.md`) | Same ordering: buildtime first, then declarative macros, then derives |
| Reverse-monomorphization | `@buildtime` is the natural way to produce schema constants for reverse-mono `call()` templates |
| External macros (.node FFI) | Buildtime sandbox is isolated from external macros — they can't read each other's state |
| WASM macro plugins | Same |
| Source maps | Buildtime spans get the same treatment as macro spans; the existing patch applicator handles them |

## Relationship to loopshot

Same engine ships in two delivery vehicles:

- **macroforge-ts upstream**: Vite plugin uses the QuickJS or V8 backend, output goes to disk
- **loopshot**: macroforge port uses the deno_core backend (loopshot's existing `JsRuntime`), no separate JS engine in the loopshot binary

Both call the same `expand_sync` API; the engine doesn't know or care which backend is consuming it. `@buildtime` lands **once** in macroforge-ts upstream and both delivery paths inherit it. The deno_core backend just needs Phase 10 wiring (small — uses loopshot's existing isolate management).

For loopshot, this means `@buildtime` gives users compile-time JS execution that's fully integrated with loopshot's runtime — no separate sandbox process, no FFI overhead, full access to loopshot's source map and error reporting infrastructure. It's the missing piece for things like "read a config file at startup, generate a typed router from it, ship the typed router as part of the bundle."
