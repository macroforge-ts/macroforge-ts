## AST Nodes: `ts_quote!`

`ts_quote!` parses a TypeScript snippet when your crate compiles, so a syntax error is a compile
error, and builds it as an oxc AST node at run time. `$name` placeholders are filled from the
variables after the snippet:

Rust

```
use macroforge_ts::macros::ts_quote;
use macroforge_ts::ts_syn::oxc::allocator::Allocator;
use macroforge_ts::ts_syn::{expr_to_string, parse_expr};

let arena = Allocator::default();
let rhs = parse_expr(&arena, "1 + 2").expect("a valid expression");

// $name takes an identifier (the default); $rhs an expression
let assignment = ts_quote!("$name = $rhs" as Expr, name = "count", rhs: Expr = rhs);
assert_eq!(expr_to_string(&assignment), "count = 1 + 2");
```

The node lives in an oxc arena: the `arena` variable in scope, or one passed first, as in
`ts_quote!(&other_arena, "a + b" as Expr)`.

| `as`           | Builds                                            |
| -------------- | ------------------------------------------------- |
| `Expr`         | An expression                                     |
| `Stmt`         | A statement                                       |
| `ModuleItem`   | A top-level item, such as a declaration or import |
| `Program`      | A whole program                                   |
| `Pat`          | A binding pattern                                 |
| `AssignTarget` | The left side of an assignment                    |
| `TsType`       | A type                                            |
| `PropOrSpread` | An object property or spread                      |

A variable's type says what its placeholder holds: `Ident` (the default, from a string), `Expr`,
`Pat`, `Str` (a string literal's text), `AssignTarget` or `TsType`.

`parse_expr`, `parse_statement`, `parse_module_item`, `parse_program`, `parse_type`,
`parse_binding_pattern`, `parse_assignment_target` and `parse_prop_or_spread` parse text into the
same nodes, and `expr_to_string`, `stmt_to_string`, `type_to_string`, `binding_pattern_to_string`,
`assignment_target_to_string` and `string_literal_to_string` print them back, for example to
interpolate into a `ts_template!`.
