# `ts_template!` - Svelte-Style TypeScript Code Generation

The `ts_template!` macro provides an intuitive, template-based way to generate TypeScript code in your macros. It uses Svelte-inspired syntax for control flow and interpolation.

## Syntax

### Interpolation: `#{expr}`

Insert Rust expressions into the generated TypeScript:

```rust
let class_name = "User";
let method = "toString";

let code = ts_template! {
    #{class_name}.prototype.#{method} = function() {
        return "User instance";
    };
};
```

**Generates:**

```typescript
User.prototype.toString = function () {
  return "User instance";
};
```

### Conditionals: `{#if}...{:else}...{/if}`

```rust
let needs_validation = true;

let code = ts_template! {
    function save() {
        {#if needs_validation}
            if (!this.isValid()) return false;
        {/if}
        return this.doSave();
    }
};
```

### Iteration: `{#each list as item}`

```rust
let fields = vec!["name", "email", "age"];

let code = ts_template! {
    function toJSON() {
        const result = {};
        {#each fields as field}
            result.#{field} = this.#{field};
        {/each}
        return result;
    }
};
```

**Generates:**

```typescript
function toJSON() {
  const result = {};
  result.name = this.name;
  result.email = this.email;
  result.age = this.age;
  return result;
}
```

## Complete Example: JSON Derive Macro

**Before** (manual AST building):

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

**After** (with `ts_template!`):

```rust
pub fn derive_json_macro(input: TsStream) -> MacroResult {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let fields = class.field_names();

            let runtime_code = ts_template! {
                #{class_name}.prototype.toJSON = function() {
                    const result = {};
                    {#each fields as field}
                        result.#{field} = this.#{field};
                    {/each}
                    return result;
                };
            };

            // ...
        }
    }
}
```

## How It Works

1. **Compile-Time**: The template is parsed during macro expansion
2. **String Building**: Generates Rust code that builds a TypeScript string at runtime
3. **SWC Parsing**: The generated string is parsed with SWC to produce a typed AST
4. **Result**: Returns `Stmt` that can be used in `MacroResult` patches

## Return Type

`ts_template!` returns a `Result<Stmt, TsSynError>` by default. The macro automatically unwraps and provides helpful error messages showing the generated TypeScript code if parsing fails.

## Nesting and Regular TypeScript

You can mix template syntax with regular TypeScript. Braces `{}` are recognized as either:

- **Template tags** if they start with `#`, `:`, or `/`
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

## Advanced: Nested Iterations

```rust
let classes = vec![
    ("User", vec!["name", "email"]),
    ("Post", vec!["title", "content"]),
];

ts_template! {
    {#each classes as (class_name, fields)}
        #{class_name}.prototype.toJSON = function() {
            return {
                {#each fields as field}
                    #{field}: this.#{field},
                {/each}
            };
        };
    {/each}
}
```

## Comparison with Alternatives

| Approach             | Pros                               | Cons                            |
| -------------------- | ---------------------------------- | ------------------------------- |
| **`ts_quote!`**      | Compile-time validation, type-safe | Can't handle Vec<Stmt>, verbose |
| **`parse_ts_str()`** | Maximum flexibility                | Runtime parsing, less readable  |
| **`ts_template!`**   | Readable, handles loops/conditions | Small runtime parsing overhead  |

## Best Practices

1. ✅ Use `ts_template!` for complex code generation with loops/conditions
2. ✅ Use `ts_quote!` for simple, static statements
3. ✅ Keep templates readable - extract complex logic into variables
4. ❌ Don't nest templates too deeply - split into helper functions

## Error Messages

If the generated TypeScript is invalid, you'll see:

```
Failed to parse generated TypeScript:
User.prototype.toJSON = function( {
    return {};
}
```

This shows you exactly what was generated, making debugging easy!
