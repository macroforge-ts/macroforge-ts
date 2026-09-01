# Defining Macros

## Tag form

The simplest way to define a macro is a tagged template literal:

```typescript
import { macroRules } from 'macroforge/rules';

const $double = macroRules`
  ($x:Expr) => ($x) * 2
`;
```

Three requirements:

- **The import activates the pipeline.** `import { macroRules } from "macroforge/rules"` is the
  signal that turns on declarative macro discovery for the file. Without it, `macroRules` is an
  ordinary identifier. A default import bound to the name `macroRules` also works, but aliasing
  (`import { macroRules as mr }`) is rejected.
- **The name must start with `$`.** Discovery skips any other binding.
- **The body is a mini-language, not TypeScript.** Everything between the backticks is parsed as
  arms, not as a template literal you could interpolate. `${...}` interpolation inside the template
  is a hard error.

Both `const $name = …` and `export const $name = …` are supported. The expander erases the entire
binding — including the `export` keyword — so a library file that only defines macros still compiles
cleanly under `noUnusedLocals`. The `import { macroRules }` statement is stripped too, when
`macroRules` is its only specifier.

Tag-form macros are always `expand-only`, and they cannot declare a type-position macro. Both of
those require the object form.

## Object form

The object form is required whenever you need anything beyond plain inline expansion:

```typescript
const $serialize = macroRules({
    mode: 'auto',
    kind: 'value',
    expand: macroRules`($x:Expr) => __inline_serialize($x)`,
    runtime: 'function __serialize(value) { return JSON.stringify(value); }',
    call: macroRules`($x:Expr) => __serialize($x)`
});
```

| Option                  | Type             | Meaning                                                                                                                                      |
| ----------------------- | ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| `expand`                | `macroRules` tag | The arms used when the macro is expanded inline. **Required.**                                                                               |
| `mode`                  | string           | `"expand-only"`, `"share-only"`, `"share-anyway"`, `"auto"`. camelCase aliases (`expandOnly`, `shareOnly`, `shareAnyway`) are also accepted. |
| `kind`                  | string           | `"value"` (default) or `"type"`. `"type"` declares a [type-position macro](/docs/declarative-macros/type-macros).                            |
| `runtime`               | string           | Source of the shared runtime helper. Required by the sharing modes.                                                                          |
| `call`                  | `macroRules` tag | The arms used at call sites when the macro is shared. Required by the sharing modes.                                                         |
| `runtimeName`           | string           | Template for the emitted helper's name. Must contain `$__cluster__`.                                                                         |
| `megamorphismThreshold` | number           | `0`–`255`. Distinct-shape count above which `auto` gives up and expands inline. `megamorphism_threshold` is also accepted.                   |

`expand` and `call` must be **literal** `` macroRules`…` `` tags — you cannot pass a variable
holding one. `runtime` may be a plain string or an interpolation-free template literal.

### The `mode` default is inferred

There is no single fixed default. For the tag form the mode is always `expand-only`. For the object
form:

- `runtime` **and** `call` both present → defaults to `auto`
- otherwise → defaults to `expand-only`

So the example above would be `auto` even with the explicit `mode` line removed. See
[Reverse Monomorphization](/docs/declarative-macros/sharing) for what each mode does.

## Arms

A macro body is a list of arms, each `(pattern) => body`. Arms are tried **in order**, and the first
one that matches wins:

```typescript
const $log = macroRules`
  () => console.log("")
  ($msg:Lit) => console.log($msg)
  ($msg:Lit, $($rest:Expr),+) => console.log($msg, $($rest),+)
`;
```

Arms are split on **a line whose first non-whitespace character is `(` at delimiter depth zero**.
This is worth knowing because a _body_ line that begins with `(` in column zero will be misread as
the start of a new arm. Indent such a line, or wrap it, to avoid the ambiguity.

`()` matches exactly zero arguments — it is not a wildcard.

## Scoping and shadowing

Macros are scoped lexically. A macro defined inside a function, block, or static block is visible
only within that scope:

```typescript
const $id = macroRules`($x:Expr) => $x`;

function inner() {
    // Shadows the outer $id within this function.
    const $id = macroRules`($x:Expr) => ($x + 1)`;
    return $id(1); // → (1 + 1)
}

const outer = $id(1); // → 1
```

Registration rejects two macros with the same name whose scopes are identical, or that overlap
without one containing the other. Disjoint sibling scopes may reuse a name freely.

One important exception: **shadowing does not apply to macro-to-macro composition.** When a macro
body calls another macro, resolution always picks the most globally-scoped definition of that name,
not the one visible at the call site.
