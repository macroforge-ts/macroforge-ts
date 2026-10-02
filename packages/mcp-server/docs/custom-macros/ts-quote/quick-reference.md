## Quick Reference

| Syntax                                     | Description                                                   |
| ------------------------------------------ | ------------------------------------------------------------- |
| `@{expr}`                                  | Interpolate a Rust expression                                 |
| `"text @{expr}"`                           | String interpolation (auto-detected)                          |
| `"'^template ${js}^'"`                     | JS backtick template literal (outputs `` `template ${js}` ``) |
| `@@{`                                      | Escape for literal `@{` (e.g., `"@@{foo}"` → `@{foo}`)        |
| `{> "comment" <}`                          | Line comment: outputs `// comment`                            |
| `{>> "comment" <<}`                        | Block comment: outputs `/* comment */`                        |
| `/// text` or `/** text */`                | JSDoc comment: outputs `/** text */`                          |
| `{#if cond}...{/if}`                       | Conditional block                                             |
| `{#if cond}...{:else}...{/if}`             | Conditional with else                                         |
| `{#if a}...{:else if b}...{:else}...{/if}` | Full if/else-if/else chain                                    |
| `{#if let pattern = expr}...{/if}`         | Pattern matching if-let                                       |
| `{#match expr}{:case pattern}...{/match}`  | Match expression with case arms                               |
| `{#for item in list}...{/for}`             | Iterate over a collection                                     |
| `{#while cond}...{/while}`                 | While loop                                                    |
| `{#while let pattern = expr}...{/while}`   | While-let pattern matching loop                               |
| `{$let name = expr}`                       | Define a local constant (`{%let}` is the same)                |
| `{$let mut name = expr}`                   | Define a mutable local variable                               |
| `{$do expr}`                               | Execute a side-effectful expression                           |
| `{$typescript stream}`                     | Inject a `TsStream`, with its patches, imports and warnings   |

**Note:** A single `@` not followed by `{` passes through unchanged (e.g., `email@domain.com` works
as expected).
