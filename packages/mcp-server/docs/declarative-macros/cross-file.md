# Cross-File Macros & Diagnostics

## Importing macros

A macro defined in one file can be used in another via an `import macro` JSDoc comment:

```typescript
/** import macro { $vec } from "./macros" */

const xs = $vec(1, 2, 3);
```

Specifiers resolve in this order: the bare path, then `.ts`, `.tsx`, `index.ts`, `index.tsx`.

## The project registry requirement

Cross-file resolution is **not automatic**. The expander only resolves imported macros when a project-wide declarative registry has been installed, which means passing `declarativeRegistryJson` in `ExpandOptions`:

```typescript
import { scanProjectSync, expandSync } from "macroforge";

const scan = scanProjectSync({ root: "./src" });

const result = expandSync(source, "src/app.ts", {
  declarativeRegistryJson: scan.declarativeRegistryJson,
});
```

The bundled integrations (Vite plugin, CLI, preprocessor) do this for you. If you call `expandSync` directly and omit it, relative `import macro` specifiers emit an error diagnostic per import and resolve nothing.

Bare **package** specifiers are skipped silently rather than erroring — they fall through to the proc-macro loader.

## Proc-macro fallback

`$`-prefixed names are a reserved namespace. When a `$name(...)` call's callee is not a declarative macro, it's dispatched as a function-like **procedural** macro, with the module resolved from an `import macro` comment or from a plain `import { $foo } from "pkg"` statement. The same applies to `$Name<...>` in type position.

Practical consequence: don't name ordinary functions with a leading `$` in a file where macros are active.

## Diagnostics

### Discovery and parse errors (fatal)

| Message | Cause |
| --- | --- |
| `${…}` interpolation in template | The macro body is a mini-language, not a JS template literal |
| Spread in object form | `macroRules({ ...opts })` isn't supported |
| Non-static key | Object-form keys must be literal identifiers/strings |
| Unknown option key | Lists the accepted keys |
| Bad `mode` string | Must be one of the four modes or their camelCase aliases |
| Bad `kind` | Must be `"value"` or `"type"` |
| `megamorphismThreshold` not a number / out of range | Must be a numeric literal in 0–255 |
| `runtimeName` missing `$__cluster__` | The template needs the placeholder |
| Missing `expand` | Required in the object form |
| Sharing mode without `runtime` + `call` | Both are required to share |
| `kind: "type"` with a sharing mode, `runtime`, or `call` | Types have no runtime representation |
| Macro definition has no arms | Empty body |
| Unbalanced delimiters | Check parens/braces in the body |
| Arm does not start with `(` | Arms begin with a parenthesized pattern |
| Expected `=>` after pattern | Missing arrow |
| Expected `:<Kind>` / unknown fragment kind | Lists the known kinds |
| Expected `*`, `+`, or `?` after repetition | A repetition needs a count suffix |
| Unterminated repetition / macro call | Missing closing delimiter |
| Duplicate macro name | Two definitions with identical or overlapping scopes |

### Match errors

| Message | Cause |
| --- | --- |
| `NoArmMatched` | No arm matched. Lists every pattern tried, and notes that backtracking was attempted. |
| `UnsupportedFragmentKind` | A fragment kind that can't appear in this position — includes a per-kind explanation and suggested alternative. |

### Expansion errors

`UnboundName`, `WrongBindingShape`, `InconsistentSequenceLength`, `UnanchoredRepetition`, `RecursionLimit` (depth 256), `UnknownMacroCall`, `MalformedMacroCallArgs`, `NestedMatchFailure`.

### Type-position errors

- *"macro `$X` is value-only; cannot use it in type position"*
- *"no arm of type-position macro matched"*

### Analyzer diagnostics

| Level | Message |
| --- | --- |
| Warning | Cluster recommendation — call sites partitioned into N clusters |
| Warning | ForceExpand — too many distinct shapes; suggests `share-anyway` to override |
| Warning | A call site matched no cluster; fell back to the single helper |
| Info | Running without a type registry; structural clustering disabled |
| Info | Analyzer telemetry, when enabled |

### Post-expansion validation

In **dev builds only**, the expander re-parses its own output and reports *"macro `$foo` produced invalid TypeScript: …"*, attributing the failure to the originating macro. This catches template bugs that would otherwise surface as a confusing syntax error in generated code.
