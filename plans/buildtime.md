Analysis: A buildtime Feature for Macroforge

What Zig's comptime Does

Zig's comptime allows arbitrary code execution at compile time
— any expression, function, or block can be evaluated by the
compiler itself, before the binary is produced. This powers
generics, type-level programming, and constant folding all
through one unified mechanism.

Where Macroforge Stands Today

Macroforge already operates at build time — the Vite plugin
intercepts source files, the Rust core expands @derive
decorators, and patched TypeScript is emitted before the
bundler ever sees it. But the current system is declarative:
you say what to derive (Debug, Clone, Serialize), not how to
compute it. The expansion logic lives in Rust procedural
macros, not in user-facing TypeScript.

The closest thing to user-defined build-time evaluation today
is foreignTypes in macroforge.config.js, where you provide
handler functions for custom types. But those are narrow — they
only influence serde codegen.

---

What a buildtime Feature Could Look Like

There are several tiers of ambition:

Tier 1: Compile-Time Constants (buildtime const)

The simplest version — evaluate TypeScript expressions at build
time and inline the results.

/\*_ @buildtime _/
const API_VERSION = computeVersionFromPackageJson();

/\*_ @buildtime _/
const ROUTES = Object.keys(routeConfig).map(k => `/${k}`);

/\*_ @buildtime _/
const SCHEMA_HASH = hashSchema(UserSchema);

After expansion, these become literal values in the output:

const API_VERSION = "2.4.1";
const ROUTES = ["/users", "/posts", "/settings"];
const SCHEMA_HASH = "a3f8c2d1";

Feasibility: High. Macroforge already has a Vite plugin that
transforms source before bundling. The main challenge is
providing a JavaScript evaluation context (V8/quickjs sandbox)
that can resolve imports and execute TypeScript. Since the Rust
core already calls into Node.js for external macro loading,
this is architecturally possible.

Tier 2: Build-Time Code Generation (buildtime fn)

Functions that run at build time and emit TypeScript code into
the module.

/\*_ @buildtime _/
function generateValidators() {
const schema = loadSchema("./user.schema.json");
return schema.fields.map(f =>
`export function validate_${f.name}(v: unknown): v is
  ${f.type} {
              return typeof v === "${f.jsType}";
          }`
).join("\n");
}

This is more powerful than Tier 1 — it's metaprogramming. The
function runs at build time and its return value is spliced
into the module as source code.

Feasibility: Medium-high. The Rust expander already handles
code splicing via its patch system. The challenge is designing
a clean API for what the function can return (raw strings? AST
fragments? template literals?) and managing the sandboxed
execution context.

Tier 3: Build-Time Type Manipulation (buildtime type)

The most ambitious tier — manipulating types at build time,
similar to how Zig's comptime can construct types.

/\*_ @buildtime _/
type StrictUser = makeAllFieldsRequired<User>;

/\*_ @buildtime _/
type APIResponse<T> = wrapWithEnvelope<T, { pagination: true
}>;

Feasibility: Low-medium. TypeScript's type system is already
Turing-complete through conditional types and mapped types. A
buildtime type system would need to offer something those don't
— likely runtime information feeding into types (e.g., reading
a JSON schema file and producing a type from it). This
overlaps heavily with what tools like zod and code generators
do.

---

How It Fits Macroforge's Architecture

Macroforge's pipeline is well-suited for this:

Source → [Parse (SWC)] → [Lower to IR] → [Expand Macros] →
[Apply Patches] → Output
↑
NEW: buildtime evaluation

The expansion phase currently dispatches to Rust macro
implementations. A buildtime feature would add a parallel
dispatch path that evaluates user-written TypeScript/JavaScript
in a sandboxed context and produces patches the same way
macros do.

Key architectural fits:

- Source mapping already handles generated code regions —
  buildtime output would get the same treatment
- TypeScript plugin could show buildtime-evaluated values on
  hover
- Vite plugin already runs enforce: "pre" — buildtime fits
  naturally here
- Metadata emission (.macroforge/meta/) could include buildtime
  evaluation results for debugging

---

Use Cases

Use Case: Environment-dependent code
Example: Inline config from .env at build time
Current Alternative: import.meta.env (Vite)
────────────────────────────────────────
Use Case: Schema-driven codegen
Example: Generate validators from JSON Schema
Current Alternative: External codegen scripts
────────────────────────────────────────
Use Case: Compile-time assertions
Example: Fail the build if invariants aren't met
Current Alternative: None in TS natively
────────────────────────────────────────
Use Case: Precomputed lookup tables
Example: Embed hash maps, regex tables, etc.
Current Alternative: Runtime initialization
────────────────────────────────────────
Use Case: Build metadata
Example: Embed git hash, build date, version
Current Alternative: Vite define config
────────────────────────────────────────
Use Case: Route manifests
Example: Generate type-safe route maps from filesystem
Current Alternative: Framework-specific
────────────────────────────────────────
Use Case: Feature flags
Example: Dead-code eliminate unused features
Current Alternative: Bundler tree-shaking
────────────────────────────────────────
Use Case: Compile-time validation
Example: Verify decorator arguments, config shapes
Current Alternative: Runtime errors

The most compelling use cases are where buildtime eliminates
runtime cost or catches errors earlier.

---

Challenges and Tradeoffs

1. Execution sandbox
   Buildtime code needs access to the filesystem, node_modules,
   and possibly network — but it also needs to be deterministic
   and safe. You'd need to decide: does it run in a full Node.js
   context, a restricted V8 isolate, or something in between?

2. Dependency tracking
   If buildtime code reads schema.json, the build system needs to
   know to re-run when that file changes. Macroforge would need to
   track file dependencies from buildtime evaluation and feed
   them back to Vite's watcher.

3. Error reporting
   When a buildtime function fails, the error needs to map back to
   the original source location. Macroforge's source mapping
   system handles this for macros but would need extension for
   arbitrary evaluated code.

4. Interaction with @derive
   Should buildtime code be able to introspect macro-expanded
   output? Run before or after macro expansion? This ordering
   question is critical:

- Before: buildtime code sees raw @derive decorators (simpler,
  but can't reference generated methods)
- After: buildtime code sees expanded output (more powerful,
  but creates dependency cycles if buildtime output affects macro
  input)
- Both phases: most flexible, most complex

5. IDE experience
   The TypeScript plugin would need to evaluate buildtime code in
   the language server to provide accurate types and completions.
   This adds latency and complexity. Alternatively, it could use
   the .macroforge/types/ declarations, but those are only fresh
   after a build.

6. Scope creep into "another language"
   Zig's comptime works because Zig has a single unified language.
   TypeScript already has a complex type system. Adding buildtime
   evaluation risks creating a third sublanguage (runtime TS,
   type-level TS, buildtime TS) that users must reason about
   separately.

---

The key differentiator from Vite's define or import.meta.env
would be type safety and integration with Macroforge's macro
system — buildtime values could inform macro expansion (e.g.,
conditionally derive traits based on build config).
