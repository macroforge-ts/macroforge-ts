User-Defined Declarative Macros ($macroName syntax)

Context

macroforge-ts currently supports macros defined in Rust via #[ts_macro_derive]. This requires Rust
knowledge, limiting macro authorship to Rust developers. This feature enables TypeScript developers
to define declarative macros directly in .ts files using familiar syntax, with Rust macro_rules!
semantics (greedy matching, first-arm-wins, fragment types, repetition).

The Rust expansion engine parses these TS-defined macros at build time and performs pattern matching

- expansion. An npm package (macroforge/rules or similar) provides type markers and compile-time
  helpers.

User-Facing Design

Macro Definition (TypeScript)

// macros.ts

```ts
import { $items } from "macroforge/rules";

/*  @macroRules */
export function $map<T, U>(argStream: '[key: _expr, val: _expr]'): Map<T, U> {
    const _map = new Map<T, U>();
    $items(entries, (key, value) => {
        _map.set(key, value);
    });
    return _map;
}

/*  @macroRules */
export const $registerComponent = (
name: _ident,
kind: _ty
) => {
    return {
        `class ${name} {
            public data: ${kind};
                constructor(val: ${kind}) {
                this.data = val;
            }
        }`
    };
};
```

Macro Invocation (TypeScript)

```ts
const myMap = $map({ a: 1, b: 2, c: 3 });
// Expands to:
// const myMap = (() => {
// const \_map = new Map();
// \_map.set("a", 1); \_map.set("b", 2); \_map.set("c", 3);
// return \_map;
// })();

$registerComponent(Position, { x: number, y: number });
// Expands to:
// class Position {
// public data: { x: number, y: number };
// constructor(val: { x: number, y: number }) { this.data = val; }
// }
```

Fragment Types (mapped from Rust)

┌──────────┬─────────────────┬──────────────────────────────────────────────────────────┐
│ Marker │ Rust equivalent │ Matches │
├──────────┼─────────────────┼──────────────────────────────────────────────────────────┤
│ \_expr │ $x:expr │ Any TS expression │
├──────────┼─────────────────┼──────────────────────────────────────────────────────────┤
│ \_ident │ $x:ident │ Single identifier │
├──────────┼─────────────────┼──────────────────────────────────────────────────────────┤
│ \_ty │ $x:ty │ TypeScript type annotation │
├──────────┼─────────────────┼──────────────────────────────────────────────────────────┤
│ \_literal │ $x:literal │ String/number/boolean literal │
├──────────┼─────────────────┼──────────────────────────────────────────────────────────┤
│ \_stmt │ $x:stmt │ Full statement │
├──────────┼─────────────────┼──────────────────────────────────────────────────────────┤
│ \_block │ $x:block │ { ... } block │
├──────────┼─────────────────┼──────────────────────────────────────────────────────────┤
│ \_item │ $x:item │ Top-level declaration (class, function, interface, etc.) │
├──────────┼─────────────────┼──────────────────────────────────────────────────────────┤
│ \_pat │ $x:pat │ Destructuring pattern │
├──────────┼─────────────────┼──────────────────────────────────────────────────────────┤
│ \_tt │ $x:tt │ Token tree (any balanced group) │
└──────────┴─────────────────┴──────────────────────────────────────────────────────────┘

Parameters typed with these markers tell the engine what AST fragment to capture from invocation
arguments.

Compile-Time Helpers

┌─────────────────────────────────┬────────────────────┬────────────────────────────────────────┐
│ Helper │ Rust equivalent │ Purpose │
├─────────────────────────────────┼────────────────────┼────────────────────────────────────────┤
│ $items(collection, (k, v) => {  │ $($k => $v),*      │ Iterate over AST tokens of an          │
 │ ... })                          │                    │ object/array argument                  │
 ├─────────────────────────────────┼────────────────────┼────────────────────────────────────────┤
 │ $repeat(list, (item) => { ...   │ $($item),\* │ Iterate over a captured repetition │
│ }) │ │ │
├─────────────────────────────────┼────────────────────┼────────────────────────────────────────┤
│ $concat(a, b)                   │ ${a}${b} (paste) │ Concatenate identifiers │
├─────────────────────────────────┼────────────────────┼────────────────────────────────────────┤
│ $stringify(ident)               │ stringify!($ident) │ Convert ident to string literal │
└─────────────────────────────────┴────────────────────┴────────────────────────────────────────┘

Implementation Plan

Phase 1: npm Package — macroforge/rules

New files:

- crates/macroforge_ts/js/rules/index.mjs — Runtime stubs + type markers
- crates/macroforge_ts/js/rules/index.d.ts — TypeScript types for fragment markers and helpers

This package exports:

- Fragment marker types (\_ident, \_ty, \_expr, etc.) as opaque branded types
- Helper function stubs ($items, $repeat, $concat, $stringify) that throw at runtime (compile-time
  only)
- These provide IDE autocomplete/type-checking for macro authors

Phase 2: Macro Definition Parsing (Rust)

New module: crates/macroforge_ts_syn/src/rules/

rules/
mod.rs — Public API: MacroRulesDef, parse functions
definition.rs — Types: MacroRulesDef, MacroParam, FragmentKind, BodyTemplate
parser.rs — Parse a $-prefixed TS function into MacroRulesDef
scan.rs — Scan source for $name(...) invocations

Key types:

pub struct MacroRulesDef {
pub name: String, // "map", "registerComponent"
pub params: Vec<MacroParam>, // Captured parameters with fragment types
pub body: BodyTemplate, // Parsed function body as expansion template
pub span: SpanIR, // Full span (comment + function)
pub fn_span: SpanIR, // Function declaration span (for stripping)
}

pub struct MacroParam {
pub name: String, // "name", "kind", "entries"
pub fragment: FragmentKind, // Ident, Ty, Expr, etc.
}

pub enum FragmentKind {
Expr, Ident, Ty, Literal, Stmt, Block, Item, Pat, Tt,
// Special: parameter is a regular TS type, not a fragment marker
// (for $map's `entries: Record<T, U>` — matched structurally)
Structural(String),
}

pub struct MacroInvocation {
pub name: String,
pub args_text: String,
pub span: SpanIR,
}

Parser logic (parser.rs):

1.  Find functions prefixed with $ that have /_ @macroRules _/ JSDoc
2.  Parse parameter list — identify fragment types by \_ident, \_ty, etc. type annotations
3.  Parse function body into a BodyTemplate — a tree of template segments with:

- Literal code segments (emit as-is)
- Parameter references (substitute captured fragment)
- $items/$repeat calls (expand as repetition)
- $concat/$stringify calls (identifier manipulation)

4.  Uses SWC to parse the function body AST, then walks it to identify substitution points

Scanner logic (scan.rs):

1.  Regex scan for $[a-zA-Z]\w\*\( in source text
2.  Find matching close paren (balanced paren tracking)
3.  Return Vec<MacroInvocation> with spans

Phase 3: Pattern Matching Engine

New module: crates/macroforge_ts_syn/src/rules/matcher.rs

At invocation, match each argument against the corresponding parameter's fragment type:

pub fn match_invocation(
def: &MacroRulesDef,
invocation: &MacroInvocation,
) -> Result<MatchBindings, MacroRulesError>

- Split invocation args on commas at balanced-paren depth 0
- For each arg + param pair, validate the arg matches the fragment kind:
    - \_ident: SWC parse as Ident — must be a single identifier
    - \_expr: SWC parse as Expr — any valid expression
    - \_ty: SWC parse as TsType — any valid type
    - \_literal: SWC parse as Lit — must be a literal
    - Structural(type): For object/array arguments to $items — parse and decompose
- Greedy matching: SWC parser consumes as much as it can for each fragment
- Return MatchBindings mapping parameter names to captured AST nodes + source text

pub struct MatchBindings {
pub captures: HashMap<String, CapturedFragment>,
}

pub enum CapturedFragment {
Single { source: String, ast: CapturedAst },
Items { entries: Vec<(String, String)> }, // For structural object decomposition
}

Phase 4: Expansion Engine

New module: crates/macroforge_ts_syn/src/rules/expand.rs

pub fn expand_macro(
def: &MacroRulesDef,
bindings: &MatchBindings,
) -> Result<String, MacroRulesError>

Walks the BodyTemplate and:

1.  Parameter references → substitute with captured source text
2.  $items calls → unroll the loop body for each entry in the structural capture
3.  $repeat calls → unroll for each element in a list capture
4.  $concat(a, b) → concatenate the source text of two ident captures
5.  $stringify(x) → emit a string literal of the ident's text
6.  Everything else → emit as-is

Produces a Patch::Replace for each invocation site and a Patch::Delete for each macro definition.

Phase 5: Integration with Expansion Pipeline

Modified files:

- crates/macroforge_ts/src/lib.rs (~line 1687) — Update early bailout
- crates/macroforge_ts/src/host/expand.rs — Add declarative macro pre-pass

Early bailout change:
// Before:
if !code.contains("@derive") {
return Ok(ExpandResult::unchanged(code));
}

// After:
let has_derive = code.contains("@derive");
let has_macro_call = code.contains("$") && MACRO_CALL_RE.is_match(code);
if !has_derive && !has_macro_call {
return Ok(ExpandResult::unchanged(code));
}

The regex MACRO_CALL_RE matches $[A-Za-z]\w\*\( to avoid false positives from template literals
${...} and Svelte runes $state(.

Expansion pipeline change (expand.rs):

Before the existing derive processing, add a new phase:

Phase 0 (NEW): Declarative macro expansion

1. Scan for @macroRules definitions → Vec<MacroRulesDef>
2. Scan for $name(...) invocations → Vec<MacroInvocation>
3. For each invocation:
   a. Find matching definition
   b. Match arguments against parameters
   c. Expand body template with bindings
   d. Produce Patch::Replace for invocation
4. Produce Patch::Delete for each definition
5. Apply patches → new source string

Phase 1 (EXISTING): Derive macro expansion
(operates on the output of Phase 0)

This ordering means declarative macros can generate code with @derive decorators that get processed
in Phase 1.

Phase 6: Cross-File Macro Imports

Extend the existing /\*_ import macro { ... } from "..."; _/ syntax:

/\*_ import macro { $map, $registerComponent } from "./macros"; _/
const m = $map({ x: 1 });

Modified: crates/macroforge_ts_syn/src/import_registry.rs

- When collecting macro imports, detect $-prefixed names
- Load and parse the target file for @macroRules definitions
- Cache parsed MacroRulesDef in the expander's registry

File-local macros (definition + invocation in same file) work without imports.

Key Files to Modify

┌─────────────────────────────────────────────────┬────────────────────────────────┐
│ File │ Change │
├─────────────────────────────────────────────────┼────────────────────────────────┤
│ crates/macroforge_ts_syn/src/lib.rs │ Add pub mod rules; │
├─────────────────────────────────────────────────┼────────────────────────────────┤
│ crates/macroforge_ts_syn/src/rules/ (NEW) │ Entire rules module │
├─────────────────────────────────────────────────┼────────────────────────────────┤
│ crates/macroforge_ts/src/lib.rs │ Update early bailout │
├─────────────────────────────────────────────────┼────────────────────────────────┤
│ crates/macroforge_ts/src/host/expand.rs │ Add Phase 0 pre-pass │
├─────────────────────────────────────────────────┼────────────────────────────────┤
│ crates/macroforge_ts/js/rules/ (NEW) │ npm package for TS types/stubs │
├─────────────────────────────────────────────────┼────────────────────────────────┤
│ crates/macroforge_ts_syn/src/import_registry.rs │ Handle $-prefixed imports │
├─────────────────────────────────────────────────┼────────────────────────────────┤
│ crates/macroforge_ts_syn/Cargo.toml │ Add regex dep if not present │
└─────────────────────────────────────────────────┴────────────────────────────────┘

Verification

1.  Unit tests in crates/macroforge_ts_syn/src/rules/ for:

- Parsing @macroRules functions into MacroRulesDef
- Scanning $name(...) invocations
- Fragment matching (each fragment type)
- Template expansion with substitutions and repetitions

2.  Integration test in tooling/playground/tests/:

- Define $map and $registerComponent macros in a test file
- Invoke them and verify expanded output
- Test cross-file imports
- Test interaction with @derive (macro generates code with decorators)

3.  Vite plugin — existing plugin calls expandSync() which goes through the pipeline; no plugin
    changes needed since Phase 0 runs transparently before Phase 1
    ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌
