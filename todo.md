# TODO

## 1. Deferred behavior — docs are accurate, the code is incomplete

- [ ] **`@deprecated` `runtimeWarn` is a no-op.** The config flag is parsed and
      defaults to `true`, but no runtime `console.warn` is injected — the branch is an
      empty block. `crates/macroforge_ts/src/host/attributes/deprecated.rs:62-72`.
      Either implement the injection or flip the default to `false`.
- [ ] **Foreign-type import-source mismatches fall through silently.**
      `import_mismatch()` has zero callers and `match_foreign_type` never populates
      `error`, so a name match from the wrong package is ignored with no diagnostic.
      `crates/macroforge_ts/src/builtin/serde/foreign_types.rs:220-244`,
      `type_category.rs:378-384`. Wire it up, or delete the dead constructor.
- [ ] **`NapiAutoBuildConfig.prefer_npx` is never read.** The generated script always
      tries npx then bunx. `crates/macroforge_ts/src/build.rs:50-51`.
- [ ] **`MacroforgeApi` trait is implemented by nothing.** The real facade is the
      inherent-impl `CoreEngine`. Kept for semver. `crates/macroforge_ts/src/api.rs:10`.
- [ ] **`PatchCode::From<Vec<ModuleItem>>` is lossy.** More than one item silently
      becomes the literal text `/* generated code */`, discarding the generated code at
      construction time. `crates/macroforge_ts_syn/src/abi/patch.rs:420-436`.
- [ ] **Buildtime `maxHeap` is advisory.** Boa exposes no memory-limit hook, so
      `SandboxError::OutOfMemory` is unreachable. Config value is carried but unenforced.
- [ ] **Buildtime `filesystem.write` / `network` are reserved.** No write or network API
      is exposed to sandboxed code, so the capabilities have nothing to gate yet.

## 2. `mf docs build-book` is broken — emits an empty book

- [ ] **The book generator produces 0 sections.** It reads four hardcoded paths under
      `docs/` — `README.md`, `getting-started.md`, `configuration.md`, `api/README.md` —
      and **none of them exist**; `docs/` contains only `api/` (JSON) and `BOOK.md`.
      Output is a 6-line stub: title, timestamp, horizontal rule.
      `tooling/scripts/src/cli/commands/docs/build_book.rs:27-48`.
      Decide: point it at the website's `.svx` content, or drop the command.
- [ ] **`verify` swallows the failure.** Step 8 calls `let _ = build_book::run(...)`, so
      the empty result never surfaces. `tooling/scripts/src/cli/commands/verify.rs:836`.
      (The neighbouring doc-extraction calls were changed to propagate; this one wasn't.)
- [ ] **`mf docs all` skips `build-book` and `check-freshness`**, despite the name.
      `tooling/scripts/src/main.rs:66-79`. Either include them or rename the subcommand.

## 3. Known test issues (both pre-existing, both fail at HEAD)

- [ ] **`DiagnosticsProvider › strictEvents` fails on TypeScript union ordering.**
      Expected `'"foo" | "click"'`, actual `'"click" | "foo"'`. Not formatting-related —
      verified identical failure on an unmodified tree. Fixing means either
      order-insensitive message matching (weakens the assertion) or re-baselining the
      snapshot. `packages/svelte-language-server/test/.../fixtures/strictEvents/`.
- [ ] **`DiagnosticsProvider › notices update of imported module` is order-dependent.**
      Fails when the 4-file subset runs, passes when its file runs alone. Test-isolation
      artifact, not a product bug.

## 4. Packaging

- [ ] **deno-plugin publish status is contradictory.** `packages/deno-plugin/package.json`
      sets `"private": true` while its README (and the new website page) reference
      `jsr:@macroforge/deno-plugin`. Publish it or correct the install instructions —
      the website page currently carries a caveat saying the specifier may not resolve.

[] add support for export { default } from '../macroforge.config.ts';
in the config file
