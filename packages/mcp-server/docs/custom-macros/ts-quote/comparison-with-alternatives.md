## Comparison with Alternatives

| Approach                            | Pros                                    | Cons                             |
| ----------------------------------- | --------------------------------------- | -------------------------------- |
| `ts_quote!`                         | Rust compile-time validation, type-safe | Can't handle Vec\<Stmt>, verbose |
| `parse_expr()`, `parse_statement()` | Maximum flexibility                     | Runtime parsing, less readable   |
| `ts_template!`                      | Readable, handles loops/conditions      | Small runtime parsing overhead   |

## Best Practices

1. Use `ts_template!` for complex code generation with loops/conditions
2. Use `ts_quote!` for simple, static statements
3. Keep templates readable - extract complex logic into variables
4. Don't nest templates too deeply - split into helper functions
