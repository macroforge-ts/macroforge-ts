# Patterns & Fragments

A pattern is a sequence of metavariables (`$name:Kind`), repetitions, and the literal separators `,` and `;`. Anything else in a pattern is a parse error.

```typescript
const $pick = macroRules`
  ($obj:Ident, $key:Lit) => $obj[$key]
`;
```

## Fragment specifiers

There are 11 fragment kinds. Each has a PascalCase name and a lowercase alias — note that two of the aliases are **not** simply the lowercased name:

| Kind | Alias | Matches |
| --- | --- | --- |
| `Expr` | `expr` | Any expression |
| `Stmt` | `stmt` | Any statement — *see below* |
| `Block` | `block` | An arrow function (`() => { … }`) |
| `Ident` | `ident` | A bare identifier |
| `Type` | **`ty`** | A TypeScript type — *type position only* |
| `Pat` | `pat` | A destructuring pattern — *see below* |
| `Lit` | **`literal`** | A literal value |
| `Path` | `path` | An identifier or dotted member (`a.b.c`) |
| `Item` | `item` | A top-level declaration — *see below* |
| `Decorator` | `decorator` | A decorator annotation — *see below* |
| `Tt` | `tt` | Structural fallback |

## What each kind actually accepts

In **value position** (a normal `$macro(...)` call), JavaScript's grammar only permits expressions as arguments. That constrains things more than the names suggest:

- **`Lit`** accepts string, numeric, boolean, `null`, BigInt, regex, **and template** literals.
- **`Path`** accepts an identifier or a *static* member expression. `a.b.c` matches; `a[b]` and `a?.b` do **not**.
- **`Block`** accepts only an arrow function expression — not a bare `{ … }`, which isn't an expression.
- **`Tt`** is a fallback that matches any expression. It is not a universal wildcard: it cannot match statements, patterns, or items.
- **Spread arguments (`...xs`) never match any fragment kind.**

Four kinds parse fine but produce a hard error if you use them in value position, because nothing they describe can appear as a call argument. Each error explains the mismatch and suggests an alternative:

| Kind | Why it can't match, and what to do instead |
| --- | --- |
| `Stmt` | Statements aren't expressions. Pass `() => { …stmts }` and capture it with `Block`. |
| `Type` | Types only appear in type position. Use `Expr` here, or declare a `kind: "type"` macro. |
| `Pat` | Destructuring patterns only appear in binding position. Capture the whole object with `Expr`. |
| `Item` | Declarations can't be arguments. Capture a function/class *expression* with `Expr`. |
| `Decorator` | Decorators attach to declarations. Capture the call itself with `Expr`. |

In **type position**, the rules invert: only `Type` and `Tt` are accepted, and any other kind errors. See [Type-Position Macros](/docs/declarative-macros/type-macros).

## Literal separators

`,` and `;` may appear as literal pattern elements, but they are **ignored during matching** — they're structural sugar, not something the matcher enforces. A consequence worth knowing: the common `$(,)?` trailing-comma idiom is a no-op. It parses, and it does no harm, but it isn't what makes trailing commas work.

## Matching order

Arms are tried top to bottom, and the first match wins. Put your most specific arms first:

```typescript
const $sum = macroRules`
  () => 0
  ($x:Expr) => $x
  ($x:Expr, $($rest:Expr),+) => ($x + $sum($($rest),+))
`;
```

`()` matches exactly zero arguments. If no arm matches, you get a `NoArmMatched` error listing every pattern that was tried — see the [diagnostics catalogue](/docs/declarative-macros/cross-file).
