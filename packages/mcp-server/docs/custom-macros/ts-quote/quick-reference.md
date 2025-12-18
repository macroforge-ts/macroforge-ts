## Quick Reference

| Syntax                                                         | Description                                                                   |
| -------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| `@{expr}`                                                      | Interpolate a Rust expression (adds space after)                              |
| `{&#124; content &#124;}`                                      | Ident block: concatenates without spaces (e.g., `{&#124;get@{name}&#124;}` → `getUser`) |
| `{> "comment" <}`                                              | Block comment: outputs `/* comment */` (string preserves whitespace)          |
| `{>> "doc" <<}`                                                | Doc comment: outputs `/** doc */` (string preserves whitespace)               |
| `@@{`                                                          | Escape for literal `@{` (e.g., `"@@{foo}"` → `@{foo}`)                        |
| `"text @{expr}"`                                               | String interpolation (auto-detected)                                          |
| `"'^template ${js}^'"`                                         | JS backtick template literal (outputs `` `template ${js}` ``)                 |
| `{#if cond}...{/if}`                                           | Conditional block                                                             |
| `{#if cond}...{:else}...{/if}`                                 | Conditional with else                                                         |
| `{#if a}...{:else if                     b}...{:else}...{/if}` | Full if/else-if/else chain                                                    |
| `{#if let pattern = expr}...{/if}`                             | Pattern matching if-let                                                       |
| `{#match expr}{:case                     pattern}...{/match}`  | Match expression with case arms                                               |
| `{#for item in list}...{/for}`                                 | Iterate over a collection                                                     |
| `{#while cond}...{/while}`                                     | While loop                                                                    |
| `{#while let pattern = expr}...{/while}`                       | While-let pattern matching loop                                               |
| `{$let name = expr}`                                           | Define a local constant                                                       |
| `{$let mut name = expr}`                                       | Define a mutable local variable                                               |
| `{$do expr}`                                                   | Execute a side-effectful expression                                           |
| `{$typescript stream}`                                         | Inject a TsStream, preserving its source and runtime\_patches (imports)       |

**Note:** A single `@` not followed by `{` passes through unchanged (e.g., `email@domain.com` works as expected).

## Interpolation: `@{expr}`

Insert Rust expressions into the generated TypeScript:

Rust

```
let class_name = "User";
let method = "toString";

let code = ts_template! {
    @{class_name}.prototype.@{method} = function() {
        return "User instance";
    };
};
```

**Generates:**

TypeScript

```
User.prototype.toString = function () {
  return "User instance";
};
```