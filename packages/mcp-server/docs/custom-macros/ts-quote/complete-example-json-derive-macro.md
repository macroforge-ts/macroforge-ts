## Complete Example: JSON Derive Macro

Here's a comparison showing how `ts_template!` simplifies code generation:

### Before (Manual AST Building)

```rust
pub fn derive_json_macro(input: TsStream) -> MacroResult {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();

            let mut body_stmts = vec![ts_quote!( const result = {}; as Stmt )];

            for field_name in class.field_names() {
                body_stmts.push(ts_quote!(
                    result.$(ident!("{}", field_name)) = this.$(ident!("{}", field_name));
                    as Stmt
                ));
            }

            body_stmts.push(ts_quote!( return result; as Stmt ));

            let runtime_code = fn_assign!(
                member_expr!(Expr::Ident(ident!(class_name)), "prototype"),
                "toJSON",
                body_stmts
            );

            // ...
        }
    }
}
```

### After (With ts_template!)

```rust
pub fn derive_json_macro(input: TsStream) -> MacroResult {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let fields = class.field_names();

            let runtime_code = ts_template! {
                @{class_name}.prototype.toJSON = function() {
                    const result = {};
                    {#for field in fields}
                        result.@{field} = this.@{field};
                    {/for}
                    return result;
                };
            };

            // ...
        }
    }
}
```

## How It Works

1. **Compile-Time:** The template is parsed during macro expansion

2. **String Building:** Generates Rust code that builds a TypeScript string at runtime

3. **SWC Parsing:** The generated string is parsed with SWC to produce a typed AST

4. **Result:** Returns `Stmt` that can be used in `MacroResult` patches

## Return Type

`ts_template!` returns a `Result<Stmt, TsSynError>` by default. The macro automatically unwraps and provides helpful error messages showing the generated TypeScript code if parsing fails:

```text
Failed to parse generated TypeScript:
User.prototype.toJSON = function( {
    return {};
}
```

This shows you exactly what was generated, making debugging easy!

## Nesting and Regular TypeScript

You can mix template syntax with regular TypeScript. Braces `&#123;&#125;` are recognized as either:

- **Template tags** if they start with `#`, `$`, `:`, or `/`

- **Regular TypeScript blocks** otherwise

```rust
ts_template! {
    const config = {
        {#if use_strict}
            strict: true,
        {:else}
            strict: false,
        {/if}
        timeout: 5000
    };
}
```

## Comparison with Alternatives

| `ts_quote!` 
| Compile-time validation, type-safe 
| Can't handle Vec<Stmt>, verbose 

| `parse_ts_str()` 
| Maximum flexibility 
| Runtime parsing, less readable 

| `ts_template!` 
| Readable, handles loops/conditions 
| Small runtime parsing overhead

## Best Practices

1. Use `ts_template!` for complex code generation with loops/conditions

2. Use `ts_quote!` for simple, static statements

3. Keep templates readable - extract complex logic into variables

4. Don't nest templates too deeply - split into helper functions