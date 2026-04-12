# Non-derive proc macros (`@attribute(...)`)

## Context

macroforge-ts already has derive macros (`@derive(MacroName)` decorators on classes / interfaces / enums / type aliases). The Rust-side macro receives a `MacroContextIR`, sees the entire decorated declaration, and emits patches via `MacroResult`.

What's missing is the analog of Rust's `#[proc_macro_attribute]`: an attribute-style macro that attaches to **arbitrary items** (primarily function declarations) and rewrites the whole item, not just emits adjacent code. This is the prerequisite for things like:

- Svelte-runes-style reactive function bodies (`@attribute(Reactive) function Component() { let x = state(0); ... }`).
- `#[tokio::main]`-style runtime wrapping (`@attribute(MainAsync) async function main() { ... }`).
- Pin-projection / borrow-rewriting / async desugaring style transforms.
- Anything where the macro needs to TRANSFORM the body of the thing it's attached to, not just generate sibling impls.

The audit of the existing infrastructure showed that **about 80% of what's needed is already in place**, sitting as unused enum variants and as comments describing the intended use:

- `MacroKind::Attribute` exists in `crates/macroforge_ts_syn/src/abi/ir/context.rs:82` but is never constructed.
- `TargetIR::Function` exists at `context.rs:127` as a unit-variant placeholder.
- The dispatcher (`crates/macroforge_ts/src/host/dispatch/dispatcher.rs`) routes by descriptor lookup, not by kind, so attribute macros work through the existing path once registered.
- The external-macro FFI loader (`crates/macroforge_ts/src/host/expand/external_loader.rs`) uses generic JSON-serialized context — works for any macro kind without changes.
- Inline-decorator parsing on classes/interfaces/enums already produces `Vec<DecoratorIR>` for ALL decorators; the `@derive` filter at `derive_targets.rs:80` is the only thing currently restricting which ones get acted on.

The work concentrates in three areas: (1) **discovery** — find `@attribute(...)` decorators on function declarations, (2) **lowering** — build a `FunctionIR` so the Rust macro has structured access, (3) **patch integration** — make sure patches over a function's full span flow through the existing applicator unchanged.

Total scope: ≈700 LOC of Rust + ≈250 LOC of tests across 6 PRs. None of it is architectural rework; every piece slots into an existing extension point.

## Summary of PRs

| PR | What | Rough size |
|---|---|---|
| 1 | `FunctionIR` struct + `lower_functions_oxc()` pass | ~250 LOC |
| 2 | Generalize JSDoc decorator scanner from `@derive`-only to `@derive` + `@attribute` | ~80 LOC |
| 3 | `collect_attribute_targets()` discovery | ~150 LOC |
| 4 | Pipeline integration: dispatch attribute macros via existing infra | ~150 LOC |
| 5 | `#[ts_macro_attribute]` proc macro for ergonomic Rust-side registration | ~120 LOC |
| 6 | End-to-end test bundle + worked example | ~250 LOC |

## PR 1 — `FunctionIR` and lowering

**Symptom.** Functions are completely invisible to the macro pipeline. There's no `FunctionIR` struct, no `lower_functions()` pass, and `TargetIR::Function` (`context.rs:127`) is a unit variant placeholder. Anything attribute macros want to do with a function — read its parameters, walk its body, replace its declaration — has nowhere to start.

**Fix.**

1. **New IR struct.** Add `crates/macroforge_ts_syn/src/abi/ir/function.rs`:

   ```rust
   pub struct FunctionIR {
       pub name: String,
       /// Span of the entire declaration: `function name(args): T { body }`,
       /// inclusive of the leading `function`/`async function`/`export function`
       /// keyword(s) and the closing `}`. Attribute macros that fully
       /// replace the function emit a `Patch::Replace` over this span.
       pub span: SpanIR,
       /// Span of the body alone, between `{` and `}`. Attribute macros
       /// that only rewrite the body (e.g. inserting reactivity hooks)
       /// patch within this span.
       pub body_span: SpanIR,
       /// Span of the signature: everything before the body's opening
       /// brace. Useful for macros that want to replace just the
       /// signature (e.g. add a new parameter, change the return type).
       pub signature_span: SpanIR,
       pub is_async: bool,
       pub is_generator: bool,
       pub is_exported: bool,
       pub type_params: Vec<String>,
       pub params: Vec<FunctionParamIR>,
       pub return_type_src: Option<String>,
       /// Raw source of the body (between but not including the braces).
       /// Convenient for macros that want to walk it as text without
       /// re-parsing OXC.
       pub body_src: String,
       pub decorators: Vec<DecoratorIR>,
   }

   pub struct FunctionParamIR {
       pub name: String,
       pub span: SpanIR,
       pub type_src: Option<String>,
       pub default_src: Option<String>,
       pub is_optional: bool,
       pub is_rest: bool,
       pub decorators: Vec<DecoratorIR>,
   }
   ```

   Mirror the layout of `ClassIR`/`InterfaceIR` (whose layout the audit documented at `class.rs:75–115` and `interface.rs:67–92`) so the macro author's mental model carries straight over from derives to attributes.

2. **Promote the placeholder.** Change `TargetIR::Function` in `context.rs:127` from a unit variant to `Function(FunctionIR)`. Add `MacroContextIR::as_function() -> Option<&FunctionIR>` alongside the existing `as_class`/`as_interface`/`as_enum`/`as_type_alias` accessors at `context.rs:279–308`.

3. **Lowering pass.** Add `lower_functions_oxc()` in `crates/macroforge_ts/src/host/expand/lowering/` next to `lower_classes_oxc()` / `lower_interfaces_oxc()` / `lower_enums_oxc()` / `lower_type_aliases_oxc()`. The function walks the OXC program for `Statement::FunctionDeclaration` and `Statement::ExportNamedDeclaration` wrapping a function declaration, builds `FunctionIR`s, populates the spans / params / body source, and adds them to the project's IR registry.

4. **Wire into the expand pipeline.** Call `lower_functions_oxc()` at `expand/mod.rs:558–571` next to the other lowering calls.

**Out of scope for PR 1**: arrow functions (`ArrowFunctionExpression`), function expressions, class methods. These are syntactically distinct and have no natural place for a JSDoc decorator. PR 6 may add them as a follow-up if usage demand surfaces.

**Files.**
- New: `crates/macroforge_ts_syn/src/abi/ir/function.rs`
- Modified: `crates/macroforge_ts_syn/src/abi/ir/context.rs` (`TargetIR::Function`, `as_function()`)
- Modified: `crates/macroforge_ts_syn/src/abi/ir/mod.rs` (export new module)
- New: `crates/macroforge_ts/src/host/expand/lowering/functions.rs`
- Modified: `crates/macroforge_ts/src/host/expand/mod.rs` (call site at ~line 558)

**Tests.**
- Plain `function foo() {}` → name, span, body_span, signature_span, no decorators.
- `export async function bar(x: number, y: string = "z"): Promise<void> { ... }` — verifies async, exported, params with types and defaults, return type.
- Function with `/** @hi */` JSDoc decorator above it — decorator parsed into `DecoratorIR`.
- Generic `function f<T, U>(x: T): U { ... }` — type params populated.
- Span correctness: assert that `span` covers the leading keywords through the closing brace and `body_span` is properly nested.

**Migration risk.** None — pure addition. The existing `TargetIR` consumers are pattern-matched against named variants; adding a payload to `Function` won't break exhaustive matches because every existing match site uses `_` for it (per the audit, the variant is "100% present in the type system but zero runtime implementation exists").

---

## PR 2 — Generalize the JSDoc decorator scanner

**Symptom.** `find_leading_derive_comment()` in `crates/macroforge_ts/src/host/expand/derive_targets.rs:349–388` is hardcoded to recognize only `@derive` (line 379: `if !name.eq_ignore_ascii_case("derive")`). The inline-decorator filter at lines 78–95 has the same hardcoding (line 80). All other decorator names already parse into `DecoratorIR` but are silently ignored at discovery time.

**Fix.** Generalize the scanner to recognize a small known set of macro-attaching decorator names instead of just `derive`.

1. **Introduce a `DecoratorKind` enum** in `derive_targets.rs`:

   ```rust
   #[derive(Debug, Clone, Copy, PartialEq, Eq)]
   enum DecoratorKind {
       /// `@derive(...)` — generates additional impls / patches
       /// alongside the decorated type. Restricted to type-like
       /// declarations (class, interface, enum, type alias).
       Derive,
       /// `@attribute(...)` — rewrites or replaces the decorated
       /// item. Primarily for function declarations but also valid
       /// on classes / interfaces / enums / type aliases.
       Attribute,
   }

   fn classify_decorator_name(name: &str) -> Option<DecoratorKind> {
       match name {
           n if n.eq_ignore_ascii_case("derive") => Some(DecoratorKind::Derive),
           n if n.eq_ignore_ascii_case("attribute") => Some(DecoratorKind::Attribute),
           _ => None,
       }
   }
   ```

2. **Rename and generalize** `find_leading_derive_comment` → `find_leading_macro_decorators`. Return `Vec<DiscoveredDecorator>` instead of `Option<(SpanIR, String)>`:

   ```rust
   struct DiscoveredDecorator {
       kind: DecoratorKind,
       args: String,         // raw text inside `(...)`, parsed downstream
       span: SpanIR,         // the @decorator(...) source range
   }
   ```

3. **Update existing call sites.** The four `collect_from_*` functions at `derive_targets.rs:72–242` filter the new vector for `DecoratorKind::Derive` so derive's existing behavior is preserved exactly. PR 3 will add a parallel filter for `DecoratorKind::Attribute`.

This is intentionally a minimal refactor — about 80 LOC modified. The scanner does NOT need to know what attribute macros mean semantically; it just needs to surface them so PR 3 can act on them.

**Files.**
- `crates/macroforge_ts/src/host/expand/derive_targets.rs` (rename + generalize the scanner; existing collect-from-* functions filter for `Derive` only).

**Tests.**
- A class with `/** @derive(Foo) @attribute(Bar) */` produces both decorators in the scanner output.
- The existing `@derive`-only behavior continues to fire on classes / interfaces / enums / type aliases (regression guard against accidentally breaking the derive path).
- A class with only `@attribute(Bar)` produces no derive targets (the existing pipeline ignores it for now, until PR 3).
- Decorator with no `(...)` (`/** @derive */`) is rejected with the same error message as today.

**Migration risk.** None. The refactor is internal; the function name change is contained to one module.

---

## PR 3 — Discovery of attribute targets

**Symptom.** No code looks for `@attribute(...)` decorators. PR 2 made them visible to the scanner; this PR threads them into a discovery output the dispatch pipeline can consume.

**Fix.** Add `collect_attribute_targets()` in `derive_targets.rs` next to `collect_derive_targets()` (currently at line 41). Mirror the structure: walk the IR registries (now including `FunctionIR` from PR 1), inspect their decorator vectors for `DecoratorKind::Attribute`, build `AttributeTarget` records.

```rust
pub struct AttributeTarget {
    pub macro_names: Vec<(String, String)>, // (macro_name, module_path) pairs
    pub decorator_span: SpanIR,
    pub target_ir: AttributeTargetIR,
}

pub enum AttributeTargetIR {
    Function(FunctionIR),
    // Attribute macros aren't restricted to functions — they're
    // primarily useful there but valid on type-like declarations
    // too. Reuse the same IR types as DeriveTargetIR.
    Class(ClassIR),
    Interface(InterfaceIR),
    Enum(EnumIR),
    TypeAlias(TypeAliasIR),
}

pub fn collect_attribute_targets(ir: &ProjectIR) -> Vec<AttributeTarget> {
    let mut out = Vec::new();
    for func in ir.functions() {
        out.extend(collect_attribute_from_function(func));
    }
    for class in ir.classes() {
        out.extend(collect_attribute_from_class(class));
    }
    // ... interface, enum, type alias
    out
}
```

The dispatcher input — `MacroContextIR` — already supports all five target types via `TargetIR`. The discovery output just needs a parallel collection function so the expand pipeline can iterate attribute targets the same way it iterates derive targets.

**Cross-target validation.**
- A `@derive(Foo)` on a function declaration is a USER ERROR. Emit a `DiagnosticLevel::Error` with help text: `"`@derive` only applies to type-like declarations (class, interface, enum, type alias); use `@attribute` if you want to transform a function declaration"`. The function isn't added to either target list.
- A `@attribute(Foo)` on a type-like declaration is LEGAL (just less common than functions).

**Files.**
- `crates/macroforge_ts/src/host/expand/derive_targets.rs` — add `collect_attribute_targets()` and supporting types/helpers.

**Tests.**
- Function with `/** @attribute(Reactive) */` → discovered, macro name extracted, target_ir is `Function(...)`.
- Class with `/** @attribute(Sealed) */` → discovered with `Class(...)` target.
- Function with `/** @derive(Foo) */` (nonsensical) → no derive target produced AND no attribute target produced; one error diagnostic with the cross-kind help text.
- Function with both `/** @derive(Foo) @attribute(Bar) */` → derive errors as above, attribute is still discovered.
- Multiple attribute macros: `/** @attribute(Reactive) @attribute(Memo) */` → two `AttributeTarget` entries from the same decorator block.

**Migration risk.** None — pure addition. Existing code paths see no change.

---

## PR 4 — Pipeline integration: dispatch attribute macros

**Symptom.** Even with discovery + lowering done, the expand pipeline (`crates/macroforge_ts/src/host/expand/mod.rs` around line 597–598 in `expand_source_oxc`) only calls `collect_macro_patches_oxc()` for derive targets. Nothing kicks off attribute-target dispatch.

**Fix.** Extend `collect_macro_patches_inner()` (around `expand/mod.rs:868`) to also iterate `collect_attribute_targets()` and dispatch each one. The dispatch path is identical to derives — same `dispatcher.dispatch(ctx)` call (line 1151), same patch collection, same external-loader FFI fallback (lines 1215–1272). The only differences are at the construction step:

1. **New context constructors.** Add to `crates/macroforge_ts_syn/src/abi/ir/context.rs` parallel to `new_derive_class` (lines 240–265):

   ```rust
   pub fn new_attribute_function(
       macro_name: String,
       module_path: String,
       decorator_span: SpanIR,
       target_span: SpanIR,
       file_name: String,
       function_ir: FunctionIR,
       target_source: String,
   ) -> Self {
       Self {
           // ... same setup as new_derive_class but ...
           macro_kind: MacroKind::Attribute,
           target: TargetIR::Function(function_ir),
           // ... rest unchanged
       }
   }

   pub fn new_attribute_class(...) -> Self { /* macro_kind=Attribute, target=Class(_) */ }
   pub fn new_attribute_interface(...) -> Self { /* ditto */ }
   pub fn new_attribute_enum(...) -> Self { /* ditto */ }
   pub fn new_attribute_type_alias(...) -> Self { /* ditto */ }
   ```

   These are 25-line near-copies of the existing `new_derive_*` constructors. The only meaningful difference is `macro_kind: MacroKind::Attribute`.

2. **Dispatch loop.** Add a section to `collect_macro_patches_inner()` after the existing derive loop (around line 1310) that iterates `collect_attribute_targets()` and calls the dispatcher with the appropriate `new_attribute_*` constructor:

   ```rust
   for target in collect_attribute_targets(ir) {
       for (macro_name, module_path) in &target.macro_names {
           let ctx = match &target.target_ir {
               AttributeTargetIR::Function(f) => MacroContextIR::new_attribute_function(...),
               AttributeTargetIR::Class(c) => MacroContextIR::new_attribute_class(...),
               // ...
           };
           let result = self.dispatcher.dispatch(ctx)
               .or_else(|_| self.external_loader.dispatch(ctx))?;
           collector.add(result);
       }
   }
   ```

3. **Patch validation.** Attribute macros usually emit a `Patch::Replace` over the target's full span. The existing `PatchApplicator::validate_no_overlaps` (with PR 15's linear-sweep version from the previous round) catches any overlap conflicts when an attribute macro emits patches that interfere with each other or with sibling derive patches. No new validation logic is needed — the existing applicator handles it.

4. **Pipeline ordering.** Run derive macros first, then attribute macros. Rationale: a derive on a class might emit code that an attribute on a sibling function references, and attribute macros are more likely to do invasive whole-item rewrites that should see the derive output. Both touch independent spans so the order rarely matters in practice, but pinning it makes the semantics predictable.

**Files.**
- `crates/macroforge_ts_syn/src/abi/ir/context.rs` — five new `new_attribute_*` constructors.
- `crates/macroforge_ts/src/host/expand/mod.rs` — extend `collect_macro_patches_inner()` with the attribute dispatch loop after the existing derive loop (~line 1310).

**Tests.**
- A `@attribute(Identity)` macro registered as a stub that returns `MacroResult::default()` (no patches). Verifies the dispatch path works end-to-end with no rewrite — the function is preserved as-is.
- A `@attribute(Reverse)` macro that returns a `Patch::Replace` over the function's full span with the function body in reverse statement order. Verifies that the patch flows through the applicator and the replaced function lands in the output.
- A `@attribute(Inject)` macro that emits an `Insert` for an extra helper function ABOVE the decorated function and a `Replace` over the body. Verifies that multiple patches from one attribute coexist correctly and source maps stay intact.
- An `@attribute(BadMacro)` that emits overlapping patches → existing `validate_no_overlaps` returns an error.
- Two attribute macros on the same function (`@attribute(A) @attribute(B)`) both run; their patches must not conflict (test asserts the applicator's overlap detection catches it if they do).

**Migration risk.** None for existing derive-only projects — they take exactly the same code path. New attribute behavior is opt-in via `@attribute(...)` decorators.

---

## PR 5 — `#[ts_macro_attribute]` proc macro

**Symptom.** Authoring an attribute macro on the Rust side currently requires manually constructing a `DerivedMacroDescriptor` with `kind: MacroKind::Attribute` and submitting it via `inventory::submit!`. There's no ergonomic attribute proc macro to make this clean — derives have `#[ts_macro_derive]`, attributes need a parallel.

**Fix.** Add `#[ts_macro_attribute]` proc macro to `crates/macroforge_ts_macros/`. It generates the `DerivedMacroDescriptor` with `kind: MacroKind::Attribute`, the constructor function, and the inventory submission, mirroring `#[ts_macro_derive]`'s expansion. Usage:

```rust
use macroforge_ts::prelude::*;

#[ts_macro_attribute(name = "Reactive", package = "myapp/macros")]
pub fn reactive(ctx: MacroContextIR) -> MacroResult {
    let func = ctx.as_function().expect("attribute on a function");

    // Walk func.body_src looking for `let x = state(0);` declarations.
    let bindings = find_state_bindings(&func.body_src);

    // Rewrite each use of those bindings to `.get()` / `.set()`.
    let rewritten_body = rewrite_reactivity(&func.body_src, &bindings);

    // Reconstruct the function with the new body.
    let new_function_source = format!(
        "function {}({}): {} {{\n{}\n}}",
        func.name,
        format_params(&func.params),
        func.return_type_src.as_deref().unwrap_or("void"),
        rewritten_body,
    );

    MacroResult::default()
        .with_runtime_patch(Patch::Replace {
            span: func.span,
            code: PatchCode::Text(new_function_source),
            source_macro: Some("Reactive".into()),
        })
}
```

The proc macro itself is essentially a one-line variation on `#[ts_macro_derive]`. Look at the existing implementation in `crates/macroforge_ts_macros/src/lib.rs`, copy `ts_macro_derive`, change the `MacroKind::Derive` literal to `MacroKind::Attribute`, and add a new attribute name. Total: ~60 LOC.

**Built-in attribute macros to ship with PR 5.**
- `#[ts_macro_attribute(name = "Identity")]` — no-op, used by tests and as a worked example for users.
- `#[ts_macro_attribute(name = "Memo")]` — wraps a function in a memoization cache (real-world example with practical value).

These live in `crates/macroforge_ts/src/builtin/attribute_*.rs` next to the existing `crates/macroforge_ts/src/builtin/derive_*.rs`.

**Files.**
- New: `crates/macroforge_ts_macros/src/lib.rs` — add `ts_macro_attribute` proc macro.
- New: `crates/macroforge_ts/src/builtin/attribute_identity.rs` — no-op example.
- New: `crates/macroforge_ts/src/builtin/attribute_memo.rs` — memoization example.
- Modified: `crates/macroforge_ts/src/builtin/mod.rs` — register the new built-ins.

**Tests.**
- `#[ts_macro_attribute]` proc-macro hygiene tests via `trybuild`: registration generates compilable code; conflicting names error at compile time; missing `name` attribute argument errors.
- `Identity` end-to-end: `@attribute(Identity)` on a function leaves it unchanged.
- `Memo` end-to-end: `@attribute(Memo) function fib(n: number): number { ... }` wraps the function in a cache map and asserts the wrapper is present in the output and the original function is moved into the wrapper.

**Migration risk.** None — pure addition.

---

## PR 6 — End-to-end test bundle + worked example

**Symptom.** Each prior PR has its own targeted tests. PR 6 stitches them together with realistic end-to-end scenarios that exercise the full pipeline: parse → lower → discover → dispatch → patch → reapply.

**New tests (in `crates/macroforge_ts/src/host/declarative/tests.rs` or a new `attribute_macros_tests.rs` integration file):**

1. **Reactive runes worked example.** A `Reactive` attribute macro implemented in Rust that takes a function whose body declares `let x = state(0);` and rewrites every read/write of `x` to `.get()` / `.set()`. Verifies the full Svelte-runes-style use case the original question (in the planning conversation) was about. Checks:
   - The output function declares `state` calls intact.
   - All reads of `x` become `x.get()`.
   - All writes (`x = ...`, `x++`, `x += 1`) become `x.set(...)`.
   - Other identifiers in scope are NOT rewritten.

2. **Memoization worked example.** `@attribute(Memo)` on a recursive function (`fib`). The output should:
   - Wrap the function in a closure with a `Map<string, number>` cache.
   - Make the recursive calls hit the cache.
   - Preserve the function's exported visibility.

3. **Attribute composition.** Two attribute macros on the same function — `@attribute(Logged) @attribute(Memo)` — verify the patches compose correctly without overlap. Pin the order: first attribute runs first; later attributes see the rewritten function as input.

   *(Caveat: PR 4's design runs attribute macros sequentially against the ORIGINAL function source, not the rewritten output. Composition order matters and the test pins it. If the user wants chained-rewriting semantics, that's a follow-up.)*

4. **Attribute on a class.** `@attribute(Sealed) class Foo {}` — sealed marker that generates a `Object.freeze(Foo.prototype)` statement after the class. Verifies attribute macros work on class declarations as well as functions.

5. **Source map attribution.** The patches emitted by attribute macros carry the macro name in `source_macro`. Verifies that the existing `validate_expanded_source` (PR 5 from the previous round) attributes parse errors back to the originating attribute macro.

6. **External attribute macros via FFI.** Build a tiny external `.dylib` containing an attribute macro using the `external_loader` infrastructure. Verifies that the FFI path works for attributes (it should, because the audit confirmed it's kind-agnostic).

**Documentation (new file `docs/attribute-macros.md`):**

- What attribute macros are (analog to Rust's `#[proc_macro_attribute]`).
- When to use them vs. derive macros vs. declarative macros.
- The full `Reactive` worked example with the Rust-side macro source and the TypeScript-side input/output.
- The constraint: attribute macros currently only attach to **declared functions** (not arrow functions, not class methods) and to type-like declarations (class / interface / enum / type alias). Arrow functions and methods are a follow-up.
- The lifecycle: discovery → dispatch → patch → applicator. Where each step lives in the codebase, for users who want to debug.

**Files.**
- New: `crates/macroforge_ts/src/host/declarative/attribute_tests.rs` (or extend the existing `tests.rs`).
- New: `docs/attribute-macros.md`.
- New: `examples/reactive_macro/` — full worked example with Rust source, TS input, expected output, and a README.

**Migration risk.** None — pure additions.

---

## Critical files

| PR | Primary files |
|---|---|
| 1 | `crates/macroforge_ts_syn/src/abi/ir/function.rs` (new), `context.rs`, `crates/macroforge_ts/src/host/expand/lowering/functions.rs` (new), `expand/mod.rs` |
| 2 | `crates/macroforge_ts/src/host/expand/derive_targets.rs` |
| 3 | `crates/macroforge_ts/src/host/expand/derive_targets.rs` (continued) |
| 4 | `crates/macroforge_ts_syn/src/abi/ir/context.rs`, `crates/macroforge_ts/src/host/expand/mod.rs` |
| 5 | `crates/macroforge_ts_macros/src/lib.rs`, `crates/macroforge_ts/src/builtin/attribute_*.rs` (new) |
| 6 | `crates/macroforge_ts/src/host/declarative/attribute_tests.rs` (new), `docs/attribute-macros.md` (new), `examples/reactive_macro/` (new) |

**Reusable infrastructure (do not duplicate):**
- `MacroContextIR` at `context.rs:176–236` — already supports `MacroKind::Attribute` and any `TargetIR` variant. Just need new constructors.
- `DerivedMacroDescriptor` at `derived/descriptors.rs:29–46` — already has a `kind: MacroKind` field. Attribute descriptors use the same struct with `kind: MacroKind::Attribute`.
- `inventory::submit!` registration at `host/macros.rs:88–97` — kind-agnostic; works for attributes unchanged.
- `Dispatcher::dispatch()` at `dispatch/dispatcher.rs` — kind-agnostic routing; the audit confirmed it has no kind-specific logic.
- `ExternalMacroLoader` at `expand/external_loader.rs:27–290` — generic JSON-serialized FFI; works for attribute macros without changes.
- `PatchApplicator` at `host/patch_applicator/applicator.rs` — patches over function spans flow through unchanged. The PR 15 linear-sweep overlap check (from the previous round) handles attribute-vs-attribute and attribute-vs-derive patch conflicts automatically.

## Suggested landing order

PRs can ship independently within constraints. Recommended sequence:

1. **PR 1** (FunctionIR + lowering) — prerequisite for any function-targeting attribute work. Lands first, has no consumers yet, doesn't change behavior for anyone.
2. **PR 2** (decorator scanner generalization) — prerequisite for PR 3. Refactor only, no new behavior.
3. **PR 3** (attribute target discovery) — depends on PRs 1 + 2. Adds the `collect_attribute_targets()` function but no dispatch yet; existing behavior unchanged.
4. **PR 4** (pipeline integration) — depends on PR 3. First PR where attribute macros actually run. After this lands, users can author attribute macros via the existing `DerivedMacroDescriptor` API with `kind: MacroKind::Attribute`.
5. **PR 5** (`#[ts_macro_attribute]` proc macro + built-ins) — depends on PR 4. Ergonomic Rust-side authoring + first real-world built-ins.
6. **PR 6** (test bundle + docs + worked example) — lands last, exercises everything together.

Each PR can ship to main independently. PR 1 is mechanically the largest; PRs 2 and 5 are the smallest.

## Verification per PR

Same sequence as the existing macroforge-ts test pipeline:
1. `cargo test --features oxc -p macroforge_ts_syn` — IR/lowering tests
2. `cargo test --features oxc -p macroforge_ts` — host-level tests including the attribute discovery + dispatch + worked-example tests added by PRs 3, 4, 6
3. `cargo test --features oxc --all` — catch any downstream-crate breakage from `TargetIR::Function`'s variant change in PR 1
4. `pixi run build:rust` — WASM target build (`feedback_build_wasm` in memory)
5. After PR 5: `cargo test -p macroforge_ts_macros` — proc-macro hygiene tests via `trybuild`

**Cross-cutting invariant checks** introduced alongside PRs:
- PR 1: `debug_assert!` in `lower_functions_oxc()` that every produced `FunctionIR.span` strictly contains its `body_span` and `signature_span`, and `body_span` and `signature_span` don't overlap.
- PR 4: `debug_assert!` that every attribute target's emitted patches lie within the target's `span` (or are top-of-file inserts) — catches buggy attribute macros that emit patches outside their attached item's range.

## Semantic invariants and out-of-scope items

Things that LOOK like gaps but are intentional non-goals for this plan:

1. **Arrow functions and function expressions are not attribute targets.** They have no syntactic place for a JSDoc decorator. Could be added later via inline-decorator parsing on `VariableDeclarator` initializers, but it's a separate feature with its own design space (does the decorator attach to the variable or the function expression? what if both have decorators?).

2. **Class methods are not attribute targets in this plan.** They're `MethodSigIR` inside a `ClassIR`. An attribute on a class method would conflict with the existing field-decorator system used by derives. Future work; document the gap.

3. **Attribute macros don't see siblings.** A `@attribute(Foo)` on `function bar()` only sees `bar`. It doesn't see other functions in the file, can't find a `@attribute(Bar)` on a sibling, and can't coordinate cross-item rewrites. If that's needed, it's a different macro kind (file-level transform).

4. **Composition of attribute macros is sequential and non-rewriting.** Two attribute macros on the same function each receive the ORIGINAL function source as input, not each other's output. The applicator catches overlap conflicts. True chained rewriting (where macro B sees macro A's output) is a different feature with its own design.

5. **No `@attribute(...)` on statements.** Attribute macros target items (function declarations and type-like declarations). Statement-level attributes would require their own discovery path and IR — out of scope.

These are listed explicitly so they don't resurface as confusion later. Each is a "semantic constraint, not a deferred feature."
