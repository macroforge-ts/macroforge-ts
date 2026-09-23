## Complete Example: JSON Derive Macro

Here's a comparison showing how `ts_template!` simplifies code generation:

### Before (Manual AST Building)

Rust

```
pub fn derive_json_macro(input: TsStream) -> MacroResult {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();

            let mut body_stmts = vec![ts_quote!("const result = {};" as Stmt)];

            for field_name in class.field_names() {
                body_stmts.push(ts_quote!(
                    "result.$field = this.$field;" as Stmt,
                    field = ts_ident!(field_name)
                ));
            }

            body_stmts.push(ts_quote!("return result;" as Stmt));

            let runtime_code = fn_assign!(
                member_expr!(Expr::Ident(ts_ident!(class_name)), "prototype"),
                "toJSON",
                body_stmts
            );

            // ...
        }
    }
}
```

### After (With ts\_template!)

Rust

```
pub fn derive_json_macro(input: TsStream) -> MacroResult {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let fields = class.field_names();

            let runtime_code = ts_template! {
                @{class_name}.prototype.toJSON = function() {
                    const result = {};
                    {#for field in fields}
                        result.@{field} = this.@{field};
                    {/for}
                    return result;
                };
            };

            // ...
        }
    }
}
```

## How It Works

1. **Compile-Time:** The template is parsed during macro expansion
2. **String Building:** Generates Rust code that builds a TypeScript string at runtime
3. **Parsing:** The generated string is parsed with oxc to produce a typed AST.
4. **Result:** Returns a `TsStream` that can be returned directly as macro output

## Return Type

`ts_template!` returns a `TsStream`, which is what a macro function returns as its output. If the
generated source fails to parse, the macro reports an error showing the generated TypeScript:

Text

```
Failed to parse generated TypeScript:
User.prototype.toJSON = function( {
    return {};
}
```

This shows you exactly what was generated, making debugging easy!

## Nesting and Regular TypeScript

You can mix template syntax with regular TypeScript. Braces `{}` are recognized as either:

- **Template tags** if they start with `#`, `$`, `:`, or `/`
- **Regular TypeScript blocks** otherwise

Rust

```
ts_template! {
    const config = {
        {#if use_strict}
            strict: true,
        {:else}
            strict: false,
        {/if}
        timeout: 5000
    };
}
```

## Comparison with Alternatives

| Approach         | Pros                               | Cons                             |
| ---------------- | ---------------------------------- | -------------------------------- |
| `ts_quote!`      | Compile-time validation, type-safe | Can't handle Vec\<Stmt>, verbose |
| `parse_ts_str()` | Maximum flexibility                | Runtime parsing, less readable   |
| `ts_template!`   | Readable, handles loops/conditions | Small runtime parsing overhead   |

## Best Practices

1. Use `ts_template!` for complex code generation with loops/conditions
2. Use `ts_quote!` for simple, static statements
3. Keep templates readable - extract complex logic into variables
4. Don't nest templates too deeply - split into helper functions
