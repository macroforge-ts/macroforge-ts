# Declarative Macros (`$name(...)` Pattern Macros)

## Context

macroforge-ts currently supports macros via Rust `#[ts_macro_derive]`, which requires Rust knowledge and limits macro authorship to Rust developers. This plan adds **declarative macros**: pattern-matching macros defined and invoked entirely in TypeScript, with semantics modeled on Rust's `macro_rules!` but adapted for TS.

The previous version of this plan used a `@macroRules` JSDoc + `_ident`/`_ty` parameter type annotation approach. This revision changes two fundamental things:

1. **Pattern matching operates on OXC AST nodes**, not on raw token trees. This is the key adaptation for TypeScript: the JS grammar is wildly context-sensitive (`<` is comparison or generic, `{` is block or object, `/` is division or regex), and re-tokenizing a synthesized stream and getting back the same parse is genuinely difficult. AST-pattern macros sidestep this by working at a level where parsing has already happened. Each fragment kind is a typed predicate over an OXC AST node category. The macro engine never has to re-tokenize anything.

2. **Definitions use a sentinel-tagged template literal** rather than a JSDoc + function pattern. The form is `const $name = macro\`...\``. This is plain TypeScript that OXC parses as a `const` binding to a tagged template. The macro pass recognizes `<ident> = macro\`...\``, parses the template body with the dedicated macro grammar, registers the macro, and erases the const declaration before V8 sees it. No JSDoc parsing, no function-shape detection, no parameter-name-as-fragment-marker workaround.

The plan also integrates **reverse monomorphization** — the dev/prod two-mode emission strategy — so declarative macros work for the ergonomics-and-performance use cases that motivated reverse-mono in the first place. Macros that benefit from per-call inline expansion in dev get it; the same macros can compile to a shared runtime + per-call data constants in prod, letting V8 specialize via inline caches instead of fragmenting call sites.

## User-Facing Design

### Macro Definition

```ts
import { macro } from "macroforge/rules";

const $vec = macro`
  () => []

  ($($x:Expr),+ $(,)?) => {
    const __v = [];
    $( __v.push($x); )+
    __v
  }
`;
```

Parts:

- `const $vec` — the binding name; convention requires `$`-prefix on macro identifiers
- `macro\`...\`` — the sentinel tag the macro pass recognizes
- Multiple arms separated by blank lines (or by an explicit arm separator — see grammar below)
- Each arm is `pattern => body`
- Patterns use fragment specifiers like `$x:Expr` and repetition like `$( ... ),+`
- Bodies are TypeScript-shaped templates with `$x` substitution and `$( ... )<sep><kind>` repetition

### Macro Invocation

```ts
const empty = $vec();
const xs    = $vec(1, 2, 3);
const ys    = $vec(getValue(), foo + bar, "hello");
```

OXC parses `$vec(...)` as a normal function call (`$` is a valid identifier character). The macro pass recognizes that `$vec` is registered in the project's macro registry and intercepts the call before V8 ever sees it. **No `!` marker is needed.** The `$` prefix is convention, optionally enforced by lint.

This means:

- Existing TS tooling treats `$vec(1, 2, 3)` as an ordinary function call. Autocomplete, hover, and find-references all keep working.
- The expansion is invisible to the call site's source — you read it as a function call, the toolchain expands it.
- Failed macro expansions (no arm matches, fragment kind mismatch) report at the call site with the patterns that were tried.

### Multi-arm dispatch

```ts
const $opt = macro`
  ()                            => undefined
  ($x:Expr)                     => $x
  ($x:Expr, $default:Expr)      => ($x ?? $default)
  ($x:Expr, $default:Expr, ...) => $opt($x, $default)
`;
```

Three arms, three different shapes. The macro engine tries each arm against the call args in order; first match wins. If no arm matches, the diagnostic points at the call site with "no arm of `$opt` matches argument list" and lists the patterns tried. Same UX as Rust's `macro_rules!` "no rules expected this token" error.

### Fragment specifiers

Map onto OXC AST node categories:

| Fragment | Matches | OXC node |
|---|---|---|
| `$x:Expr` | any expression | `Expression` |
| `$x:Stmt` | any statement | `Statement` |
| `$x:Block` | brace-delimited block | `BlockStatement` |
| `$x:Ident` | identifier | `BindingIdentifier` / `IdentifierReference` |
| `$x:Type` | TS type expression | `TSType` |
| `$x:Pat` | destructuring pattern | `BindingPattern` |
| `$x:Lit` | literal (string, number, regex, template) | `Literal` |
| `$x:Path` | qualified name (`a.b.c`) | `MemberExpression` chain |
| `$x:Item` | top-level declaration (class, function, interface, etc.) | `Statement` (filtered) |
| `$x:Decorator` | decorator expression | `Decorator` |
| `$x:tt` | any balanced token tree | structural fallback for cases the categories don't cover |

Each fragment kind is a typed predicate over an OXC AST node. The macro engine asks "does this argument have the AST shape of a `TSType`?" — OXC has already done the parse work, so the predicate is a single match on the AST node variant.

### Repetition

```
$( <pattern> )<separator><kind>
```

| Kind | Meaning |
|---|---|
| `*` | zero or more |
| `+` | one or more |
| `?` | zero or one |

```ts
const $sum = macro`
  ($($x:Expr),+) => {
    let __acc = 0;
    $( __acc += $x; )+
    return __acc;
  }
`;

$sum(a, b, c);
// expands to (with hygienic renaming):
// (() => {
//   let __acc$1 = 0;
//   __acc$1 += a;
//   __acc$1 += b;
//   __acc$1 += c;
//   return __acc$1;
// })()
```

Repetition variables get bound to **sequences** of AST sub-trees. Inside the body, `$( ... )<sep><kind>` walks the bound sequence, splicing each element in turn. Nested repetition (`$( $( ... )* )*`) and zip-style cross-iteration both fall out naturally once the basic case works.

### Hygiene

Every identifier introduced inside a macro body gets a unique syntax context tag distinguishing it from identifiers at the call site. Implementation: each macro expansion gets a unique counter `n`. When walking the macro body to splice in bindings, every binding *introduction* (not reference) gets renamed `name → name$n`. References inside the body see the renamed version because the rename happens to both declarations and references within the same scope.

Caller-`__v` and macro-introduced-`__v$1` simply never collide. The user can refer to `$x` anywhere in the body (those are spliced from fragments at the call site and keep their original names) and to `__local` anywhere (those are macro-introduced and get the unique tag).

### Expression vs statement context

The `$vec` example expands to a block (`{ const __v = []; ... __v }`), but the call site is in expression position. The macro pass detects context and wraps the expansion in an IIFE when needed:

```ts
const xs = $vec(1, 2, 3);
// expands to:
const xs = (() => {
  const __v$1 = [];
  __v$1.push(1);
  __v$1.push(2);
  __v$1.push(3);
  return __v$1;
})();
```

In statement position the IIFE wrapper is dropped — the block is spliced in directly. This is the same expression-vs-statement context handling V8 does for arrow function parsing.

## Reverse-Monomorphization Integration

This is the meaningful new feature. Macros can declare a **mode** that controls how they emit in dev vs prod builds.

### The four modes

```ts
const $serialize = macro({
  mode: "auto",  // default: dev expands, prod shares

  // Required for `auto` and `expand-only`
  expand: macro`
    ($x:Expr) => {
      const __v = $x;
      ({ ${"$x.fields.map(f => `${f.name}: __v.${f.name}`).join(\", \")"}})
    }
  `,

  // Required for `auto`, `share-only`, and `share-anyway`
  runtime: `
    function __serialize(value, schema) {
      const out = {};
      for (let i = 0; i < schema.length; i++) {
        const k = schema[i];
        out[k] = value[k];
      }
      return out;
    }
  `,

  call: macro`
    ($x:Expr) => __serialize($x, ${"JSON.stringify($x.type.fields.map(f => f.name))"})
  `,
});
```

| Mode | Dev | Prod |
|---|---|---|
| `auto` (default) | expand | shared runtime if ≤ 4 distinct types call sites, else cluster |
| `expand-only` | expand | expand |
| `share-only` | share | share |
| `share-anyway` | expand | share even past the megamorphism threshold |

`expand-only` is what you want for tiny bodies, const-folding macros, or single-use macros where the call overhead would exceed the body cost.

`share-only` is what you want when the runtime body is too large to inline and the user just wants a clean call site.

`share-anyway` is rare — for cold-path macros where megamorphism is acceptable.

The framework's default is `auto`: dev expansion for type checking and precise diagnostics, prod sharing for V8-friendly emission.

### Megamorphism analysis

The framework counts how many distinct types call into each shared runtime at compile time (the count is statically known because the macro saw every `expand` invocation across the project). Above a configurable threshold (default: 4), the framework warns the macro author and offers two responses:

1. **Partition the runtime.** Generate two or more shared helpers, each handling a subset of types. The framework picks the partition automatically by clustering shapes that are structurally similar.
2. **Force-share anyway.** The macro author can declare `mode: "share-anyway"` to silence the warning.

### Source map composition

Three layers when `mode: "auto"` is active in prod:

```
original .ts source
       │
       ▼
[ macro pass: expand to dev form (even in prod, for type-check) ]
       │  (source map A: original ↔ dev expansion)
       ▼
dev expansion .ts
       │
       ▼
[ macro pass: collapse to prod form ]
       │  (source map B: dev expansion ↔ prod call site)
       ▼
prod .ts (shared runtime + per-call calls)
       │
       ▼
[ OXC: parse + transform + codegen ]
       │  (source map C: prod ↔ emitted JS)
       ▼
emitted .js
```

Stack-trace rewriting walks all three maps in reverse. macroforge already has source map A and source map C wired up (the existing patch applicator handles map A; OXC handles map C). Source map B is the new piece — it's small (each call site maps to one expansion site, no nested rewrites) but has to be threaded through.

In dev builds the prod step is skipped, so map B doesn't exist and stack traces go straight from emitted JS → original source via maps C and A.

## Implementation Plan

### Phase 0: npm Package — `macroforge/rules`

**New files:**

- `crates/macroforge_ts/js/rules/index.mjs` — runtime stubs that throw at runtime ("macros are compile-time only")
- `crates/macroforge_ts/js/rules/index.d.ts` — TypeScript types for the `macro` tag function and the configuration object

The package exports:

- `macro` — the sentinel tag function (throws at runtime, recognized by macroforge at build time)
- Type definitions for `MacroConfig` (the object form: `{ expand, runtime, call, mode, megamorphismThreshold }`)
- Type-level fragment kind markers for IDE autocomplete (not used by the engine, but useful for editor experience)

### Phase 1: Macro Grammar Parser

**New module:** `crates/macroforge_ts_syn/src/declarative/`

```
declarative/
  mod.rs              Public API: parse_macro_def, MacroDef, MacroArm
  grammar.rs          Hand-written recursive-descent parser for the template body
  pattern.rs          Pattern AST (Pattern, FragmentSpec, RepetitionKind)
  body.rs             Body AST (BodyToken, Substitution, Repetition)
  errors.rs           Diagnostic types
```

**Key types:**

```rust
pub struct MacroDef {
    pub name: String,           // "vec", "sum", "opt"
    pub arms: Vec<MacroArm>,
    pub mode: MacroMode,        // Auto, ExpandOnly, ShareOnly, ShareAnyway
    pub runtime: Option<String>,// Source for the prod-mode shared runtime
    pub call_arm: Option<MacroArm>, // Pattern for prod-mode call site rewriting
    pub megamorphism_threshold: u8,
    pub span: SpanIR,
}

pub struct MacroArm {
    pub pattern: Pattern,
    pub body: Body,
}

pub enum Pattern {
    Empty,                        // ()
    Sequence(Vec<PatternElement>),
}

pub enum PatternElement {
    Literal(String),              // matches literal token
    Fragment {
        name: String,             // "x" in $x:Expr
        kind: FragmentKind,
    },
    Repetition {
        pattern: Box<Pattern>,
        separator: Option<String>,
        kind: RepetitionKind,     // *, +, ?
    },
}

pub enum FragmentKind {
    Expr,
    Stmt,
    Block,
    Ident,
    Type,
    Pat,
    Lit,
    Path,
    Item,
    Decorator,
    Tt,                           // structural fallback
}

pub enum BodyToken {
    Literal(String),              // emit as-is
    Substitution(String),         // $x — splice the bound fragment's source
    Repetition {
        body: Vec<BodyToken>,
        separator: Option<String>,
        kind: RepetitionKind,
    },
    HygieneIntroduction(String),  // identifier introduced by the macro — gets per-expansion tag
}

pub enum MacroMode {
    Auto,
    ExpandOnly,
    ShareOnly,
    ShareAnyway,
}
```

**Parser logic:**

1. Detect the `<ident> = macro\`...\`` pattern in the OXC AST during the macro pre-pass
2. Extract the template literal's quasi (the static text portion)
3. Hand off to `parse_macro_def(text, span) -> Result<MacroDef, MacroError>`
4. The parser splits arms on blank lines (or explicit `;;` separators if we want stricter syntax)
5. For each arm: parse the `pattern => body` form
6. For the pattern: tokenize, then walk the token stream building a `Pattern` tree
7. For the body: same, building a `Vec<BodyToken>`

The parser is small (~600 LOC) because the grammar is small. It does NOT use OXC for the macro template — the template is its own mini-language.

### Phase 2: Macro Definition Discovery

**Module:** `crates/macroforge_ts/src/host/declarative/discovery.rs`

Walks the OXC AST during the macro pre-pass looking for the sentinel pattern. For each match:

1. Extract the template literal text
2. Call `macroforge_ts_syn::declarative::parse_macro_def`
3. Register the resulting `MacroDef` in the per-file (or per-project) macro registry
4. Mark the original `const $name = macro\`...\`` declaration for stripping (it never reaches V8)

```rust
pub struct MacroRegistry {
    pub by_name: HashMap<String, Arc<MacroDef>>,
}

impl MacroRegistry {
    pub fn register(&mut self, def: MacroDef) -> Result<(), CoherenceError>;
    pub fn lookup(&self, name: &str) -> Option<&MacroDef>;
}
```

Coherence: two macros with the same name in the same project is a build error. (Cross-package macros use the existing `import macro` pattern.)

### Phase 3: Pattern Matching Engine

**Module:** `crates/macroforge_ts/src/host/declarative/matcher.rs`

```rust
pub fn match_invocation(
    def: &MacroDef,
    call_args: &[Expression],   // OXC AST nodes for the call's arguments
    src_text: &str,
) -> Result<MatchResult, MatchError>

pub struct MatchResult {
    pub arm_index: usize,
    pub bindings: HashMap<String, Binding>,
}

pub enum Binding {
    Single(BoundFragment),
    Sequence(Vec<BoundFragment>),
}

pub struct BoundFragment {
    pub kind: FragmentKind,
    pub source: String,         // verbatim source slice
    pub span: SpanIR,
}
```

**Algorithm:**

1. For each arm in order, try to match the arm's pattern against `call_args`
2. Walk the pattern alongside the args:
   - Literal pattern element → check the next arg matches the literal source
   - Fragment pattern element → check the next arg's AST shape matches `kind`; bind it
   - Repetition pattern element → match zero/one/more occurrences with separators
3. If the pattern fully consumes the args, return `Ok(MatchResult)`
4. If not, try the next arm
5. If no arm matches, return `Err` with the list of patterns tried

Fragment-kind validation walks the OXC AST node and checks the variant. `FragmentKind::Expr` matches any `Expression`; `FragmentKind::Type` matches any `TSType`; `FragmentKind::Tt` matches anything (the structural fallback).

### Phase 4: Body Expansion + Hygiene

**Module:** `crates/macroforge_ts/src/host/declarative/expander.rs`

```rust
pub fn expand_body(
    body: &Body,
    bindings: &MatchBindings,
    expansion_id: u32,           // for hygiene renaming
) -> Result<String, ExpandError>
```

**Algorithm:**

1. Walk the `Vec<BodyToken>`
2. For each token:
   - `Literal` → append to output buffer
   - `Substitution($x)` → look up `x` in bindings, append the bound fragment's source
   - `Repetition` → walk the bound sequence, recursively expand the inner body for each element, joining with the separator
   - `HygieneIntroduction(name)` → rename to `name$<expansion_id>` and append
3. Return the assembled source string

**Hygiene scope**: a binding introduction inside a `{ ... }` block gets renamed; references to the same name inside the same block also get renamed. The expander tracks introductions in a side scope map and rewrites references on emit.

### Phase 5: Call-Site Rewriting

**Module:** `crates/macroforge_ts/src/host/declarative/rewriter.rs`

The macro pass walks the OXC AST a second time looking for call sites of registered macros:

```rust
pub fn rewrite_call(
    registry: &MacroRegistry,
    call: &CallExpression,
    src_text: &str,
) -> Result<Option<Patch>, RewriteError>
```

For each `CallExpression` whose callee is an `IdentifierReference` matching a registered macro name:

1. Look up the `MacroDef`
2. Run `match_invocation` against the call's arguments
3. Determine the build mode (dev or prod) from the macroforge `ExpandOptions`
4. Pick the right emission strategy:
   - **Dev**, mode `auto` or `expand-only`: expand the matched arm
   - **Dev**, mode `share-only` or `share-anyway`: still call `expand` for type-check purposes (the type-checker sees the dev form), but the emitted code is the call form
   - **Prod**, mode `auto`: pick share if call sites ≤ threshold, else cluster, else expand
   - **Prod**, mode `expand-only`: expand
   - **Prod**, mode `share-only` or `share-anyway`: emit the call form
5. Wrap in IIFE if the expansion is a block in expression position
6. Produce a `Patch::Replace` for the call site

If `share` is chosen, also produce a `Patch::Insert` for the shared runtime (deduplicated per macro per file via the registry).

### Phase 6: Megamorphism Analysis

**Module:** `crates/macroforge_ts/src/host/declarative/megamorph.rs`

Runs after all call sites have been collected but before patches are applied:

```rust
pub fn analyze(
    registry: &MacroRegistry,
    call_sites: &[ResolvedCallSite],
) -> MegamorphReport

pub struct MegamorphReport {
    pub per_macro: HashMap<String, MacroPolymorphism>,
}

pub struct MacroPolymorphism {
    pub distinct_types: usize,
    pub recommendation: Recommendation,
}

pub enum Recommendation {
    Share,                          // ≤ threshold; just emit shared runtime
    Cluster(Vec<TypeCluster>),      // > threshold; partition by shape similarity
    ForceExpand,                    // user requested or threshold exceeded with no clustering
}
```

The analyzer:

1. For each `auto`-mode macro, count how many distinct types flow into the shared runtime across all call sites
2. If above threshold, attempt clustering: group types by structural similarity (same field names? same primitive vs object distribution?)
3. Emit a build-time warning with the recommendation
4. The rewriter consults the report when deciding whether to emit one shared runtime or several

### Phase 7: Pipeline Integration

**Modified files:**

- `crates/macroforge_ts/src/lib.rs` — early bailout
- `crates/macroforge_ts/src/host/expand.rs` — insert the declarative macro pass

**Early bailout change:**

```rust
// Before:
if !code.contains("@derive") {
    return Ok(ExpandResult::unchanged(code));
}

// After:
let has_derive = code.contains("@derive");
let has_macro_def = code.contains("= macro`");
let has_macro_call = MACRO_CALL_RE.is_match(code);  // /\$[A-Za-z_][\w]*\(/
if !has_derive && !has_macro_def && !has_macro_call {
    return Ok(ExpandResult::unchanged(code));
}
```

The regex avoids false positives from template literals (`${...}`) and Svelte runes (`$state(`) by requiring an identifier after the `$`.

**Pipeline order in `expand_inner`:**

```
1. Parse to OXC AST
2. Lower to IR
3. NEW: Declarative macro pre-pass
   3a. Discovery: find `const $name = macro\`...\`` definitions, register
   3b. Coherence check: error on duplicate names
   3c. Rewrite: walk for call sites of registered macros, expand them
   3d. Strip: remove the original macro definition declarations
   3e. Megamorphism analysis (prod mode only)
4. EXISTING: Derive macro expansion (operates on the result of step 3)
5. Apply patches → new source string
6. Codegen back to TS source
```

The declarative macro pass runs before derive expansion, so a declarative macro can generate code with `@derive` annotations that get processed in the next pass.

### Phase 8: Cross-File Macro Imports

Extend the existing `import macro { ... } from "..."` syntax:

```ts
/** import macro { $vec, $sum } from "./macros"; */
const xs = $vec(1, 2, 3);
const total = $sum(1, 2, 3);
```

**Modified:** `crates/macroforge_ts_syn/src/import_registry.rs`

- When collecting macro imports, detect `$`-prefixed names
- Load and parse the target file for `const $name = macro\`...\`` definitions
- Cache parsed `MacroDef` in the expander's registry under the importing file's scope
- File-local macros (definition + invocation in same file) work without imports

## Key Files to Modify

| File | Change |
|---|---|
| `crates/macroforge_ts_syn/src/lib.rs` | Add `pub mod declarative;` |
| `crates/macroforge_ts_syn/src/declarative/` (NEW) | Macro grammar parser, pattern + body AST |
| `crates/macroforge_ts/src/host/declarative/` (NEW) | Discovery, matcher, expander, rewriter, megamorph analyzer |
| `crates/macroforge_ts/src/host/expand.rs` | Add declarative pre-pass, integrate with existing derive expansion |
| `crates/macroforge_ts/src/lib.rs` | Update early bailout |
| `crates/macroforge_ts_syn/src/import_registry.rs` | Handle `$`-prefixed macro imports |
| `crates/macroforge_ts/js/rules/` (NEW) | npm package: runtime stubs + TS types |
| `crates/macroforge_ts/Cargo.toml` | Add `regex` dep if not already present |

## Verification

### Unit tests

In `crates/macroforge_ts_syn/src/declarative/tests.rs`:

- Parse a macro with one arm, no fragments → check the resulting `MacroDef`
- Parse a macro with multiple arms → check arm order
- Parse fragment specifiers (each kind) → check `Pattern::Fragment` nodes
- Parse repetition (`$( ... )*`, `$( ... ),+`, `$( ... )?`) → check `Pattern::Repetition` nodes
- Parse nested repetition → check correct nesting
- Parse the four mode declarations → check `MacroMode` value
- Parse erroneous templates → check diagnostic quality

In `crates/macroforge_ts/src/host/declarative/tests.rs`:

- Match a single-arm macro against a valid call → check bindings
- Match against an invalid call → check the right error
- Match repetition against zero, one, many args → check sequence binding
- Try multiple arms in order → check first-match-wins
- Expand a body with simple substitution → check output
- Expand a body with repetition → check unrolled output
- Expand with hygiene → check identifiers get the per-expansion tag
- Expand in expression position → check IIFE wrapping
- Expand in statement position → check no IIFE wrapping
- Reverse-mono mode dispatch: dev vs prod for each mode → check correct emission
- Megamorphism analyzer: 1, 4, 5, 16, 50 distinct types → check recommendations

### Integration tests

In `crates/macroforge_ts/tests/`:

- Define `$vec`, `$sum`, `$opt` in a single file, invoke them, check expansion against snapshots
- Define a macro in `macros.ts`, import via `/** import macro */` in `consumer.ts`, check expansion
- Test interaction with `@derive` (declarative macro generates code with `@derive` decorators that get processed in the next pass)
- Test reverse-mono dev vs prod emission with a fixture that has 3 call sites of an `auto` macro → dev produces three inline expansions, prod produces one shared runtime + three calls
- Test megamorphism warning with a fixture that has 7 distinct types calling one macro → expect partition recommendation

### Vite plugin

The existing Vite plugin calls `expandSync()` which goes through the pipeline. **No plugin changes needed** since the declarative pass runs transparently before the existing derive pass.

For reverse-mono mode, the Vite plugin needs to pass the build mode (`dev` or `prod`) into `ExpandOptions`. Add a `build_mode: BuildMode` field to `ExpandOptions` and have the plugin set it from Vite's `config.command` (`serve` → dev, `build` → prod).

## Order of execution

1. **Phase 0**: npm package skeleton (~50 LOC)
2. **Phase 1**: macro grammar parser (~600 LOC, biggest single piece)
3. **Phase 2**: definition discovery (~200 LOC)
4. **Phase 3**: pattern matcher (~500 LOC)
5. **Phase 4**: body expander + hygiene (~400 LOC)
6. **Phase 5**: call-site rewriter (~300 LOC) — minimum viable system; declarative macros work in `expand-only` mode at this point
7. **Phase 6**: megamorphism analyzer (~300 LOC)
8. **Phase 7**: pipeline integration (~150 LOC of glue)
9. **Phase 8**: cross-file imports (~200 LOC)
10. **Phase 9** (later): full reverse-mono integration (`auto` mode shared runtime emission, requires the bundler to hoist the runtime per file/per bundle)
11. **Phase 10** (later): differential testing harness — diff dev expansion vs prod sharing for the same macro to catch behavior divergence

Phases 1-8 ship a working declarative macro system. Phases 9-10 add the reverse-mono polish.

## Caveats worth flagging

1. **Sentinel syntax is verbose.** `const $name = macro\`...\`` is more characters than Rust's `macro_rules! name { ... }`. The price of fitting inside vanilla TS parser tolerances. A future LSP could render the sentinel form as a cleaner shape in the editor.
2. **Type-position macros are harder.** TS types and TS values use different parser entry points; a macro that expands to a `TSType` (`type Foo = $RecordOf(...)`) needs a separate dispatch path. **Punt to a follow-up** — value/expression/statement position covers ~95% of real macro use.
3. **Recursive expansion needs a depth limit.** Set it to something generous (256?) and emit a diagnostic at the limit.
4. **Compile-time evaluation order** matters when one macro definition references another. Topological sort the macro registry; cycles are an error.
5. **`macro` as a tag name might collide** with someone's existing local variable. Mitigated by requiring an explicit `import { macro } from "macroforge/rules"` — the import is the signal.
6. **Megamorphism is a footgun if ignored.** Without the analyzer, an unwary macro author can ship a "shared runtime" that's actually slower than the inline form. The analyzer is critical, not optional. Default-on warnings.
7. **Source map B adds complexity to debugging.** Three layers of source maps means more places things can break. The error reporter must walk all three. Add a `macroforge trace --map-stack` mode that shows every layer of the map composition for a given line number, for debugging.
8. **Tree-shaking the runtime.** If the bundler ships the shared runtime even when nothing uses it, the prod form is bigger than the dev form. Standard ESM tree-shakers handle this as long as the runtime is emitted as a top-level `export function`. The framework needs an integration test that imports a macro without calling it and verifies the runtime is gone from the output.

## Relationship to loopshot

The same engine ships in two delivery vehicles:

- **macroforge-ts upstream**: invoked via Vite plugin or NAPI bindings, output goes to disk as JS files
- **loopshot**: invoked via the macroforge port, output is held in memory and fed into V8

Both call the same `expand_sync` API; the engine doesn't know or care which delivery vehicle is consuming it. Declarative macros land **once** in macroforge-ts and both delivery paths inherit them. There is no runtime/build-time fork of the engine — the engine is compile-time, period, regardless of when in the toolchain invocation the expansion happens.
