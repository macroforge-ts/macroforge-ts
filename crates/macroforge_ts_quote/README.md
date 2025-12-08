# macroforge_ts_quote

> **Warning:** This is a work in progress and probably won't work for you. Use at your own risk!

Quote macro for generating TypeScript code at compile time.

Part of the [macroforge](https://github.com/rymskip/macroforge-ts) project.

## `ts_template!` - Rust-Style TypeScript Code Generation

The `ts_template!` macro provides an intuitive, template-based way to generate TypeScript code in your macros. It uses Rust-inspired syntax for control flow and interpolation.

## Quick Reference

| Syntax | Description |
|--------|-------------|
| `@{expr}` | Interpolate a Rust expression (always adds space after) |
| `{| content |}` | Ident block: concatenates content without spaces (e.g., `{|get@{name}|}` → `getUser`) |
| `@@{` | Escape for literal `@{` (e.g., `"@@{foo}"` → `@{foo}`) |
| `"text @{expr}"` | String interpolation (auto-detected) |
| `"'^template ${js}^'"` | JS backtick template literal (outputs `` `template ${js}` ``) |
| `{#if cond}...{/if}` | Conditional block |
| `{#if cond}...{:else}...{/if}` | Conditional with else |
| `{#if a}...{:else if b}...{:else}...{/if}` | Full if/else-if/else chain |
| `{#if let pattern = expr}...{/if}` | Pattern matching if-let |
| `{#match expr}{:case pattern}...{/match}` | Match expression with case arms |
| `{#for item in list}...{/for}` | Iterate over a collection |
| `{%let name = expr}` | Define a local constant |
| `{%typescript stream}` | Inject a TsStream, preserving its source and runtime_patches (imports) |

> **Note:** A single `@` not followed by `{` passes through unchanged (e.g., `email@domain.com` works as expected).

## Syntax

### Interpolation: `@{expr}`

Insert Rust expressions into the generated TypeScript:

```rust
let class_name = "User";
let method = "toString";

let code = ts_template! {
    @{class_name}.prototype.@{method} = function() {
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

### Identifier Concatenation: `{| content |}`

When you need to build identifiers dynamically (like `getUser`, `setName`), use the ident block syntax. Everything inside `{| |}` is concatenated without spaces:

```rust
let field_name = "User";

let code = ts_template! {
    function {|get@{field_name}|}() {
        return this.@{field_name.to_lowercase()};
    }
};
```

**Generates:**

```typescript
function getUser() {
  return this.user;
}
```

Without ident blocks, `@{}` always adds a space after for readability. Use `{| |}` when you explicitly want concatenation:

```rust
let name = "Status";

// With space (default behavior)
ts_template! { namespace @{name} }  // → "namespace Status"

// Without space (ident block)
ts_template! { {|namespace@{name}|} }  // → "namespaceStatus"
```

Multiple interpolations can be combined:

```rust
let entity = "user";
let action = "create";

ts_template! { {|@{entity}_@{action}|} }  // → "user_create"
```

### String Interpolation: `"text @{expr}"`

Interpolation works automatically inside string literals - no `format!()` needed:

```rust
let name = "World";
let count = 42;

let code = ts_template! {
    console.log("Hello @{name}!");
    console.log("Count: @{count}, doubled: @{count * 2}");
};
```

**Generates:**

```typescript
console.log("Hello World!");
console.log("Count: 42, doubled: 84");
```

This also works with method calls and complex expressions:

```rust
let field = "username";

let code = ts_template! {
    throw new Error("Invalid @{field.to_uppercase()}");
};
```

### Backtick Template Literals: `"'^...^'"`

For JavaScript template literals (backtick strings), use the `'^...^'` syntax. This outputs actual backticks and passes through `${}` for JS interpolation:

```rust
let tag_name = "div";

let code = ts_template! {
    const html = "'^<@{tag_name}>${content}</@{tag_name}>^'";
};
```

**Generates:**

```typescript
const html = `<div>${content}</div>`;
```

You can mix Rust `@{}` interpolation (evaluated at macro expansion time) with JS `${}` interpolation (evaluated at runtime):

```rust
let class_name = "User";

let code = ts_template! {
    "'^Hello ${this.name}, you are a @{class_name}^'"
};
```

**Generates:**

```typescript
`Hello ${this.name}, you are a User`
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

With else:

```rust
let has_default = true;

let code = ts_template! {
    {#if has_default}
        return defaultValue;
    {:else}
        throw new Error("No default");
    {/if}
};
```

With else-if:

```rust
let level = 2;

let code = ts_template! {
    {#if level == 1}
        console.log("Level 1");
    {:else if level == 2}
        console.log("Level 2");
    {:else}
        console.log("Other level");
    {/if}
};
```

### Pattern Matching: `{#if let pattern = expr}...{/if}`

Use `if let` for pattern matching on `Option`, `Result`, or other Rust enums:

```rust
let maybe_name: Option<&str> = Some("Alice");

let code = ts_template! {
    {#if let Some(name) = maybe_name}
        console.log("Hello, @{name}!");
    {:else}
        console.log("Hello, anonymous!");
    {/if}
};
```

**Generates:**

```typescript
console.log("Hello, Alice!");
```

This is useful when working with optional values from your IR:

```rust
let code = ts_template! {
    {#if let Some(default_val) = field.default_value}
        this.@{field.name} = @{default_val};
    {:else}
        this.@{field.name} = undefined;
    {/if}
};
```

### Match Expressions: `{#match expr}{:case pattern}...{/match}`

Use `match` for exhaustive pattern matching:

```rust
enum Visibility { Public, Private, Protected }
let visibility = Visibility::Public;

let code = ts_template! {
    {#match visibility}
        {:case Visibility::Public}
            public
        {:case Visibility::Private}
            private
        {:case Visibility::Protected}
            protected
    {/match}
    field: string;
};
```

**Generates:**

```typescript
public field: string;
```

Match with value extraction:

```rust
let result: Result<i32, &str> = Ok(42);

let code = ts_template! {
    const value = {#match result}
        {:case Ok(val)}
            @{val}
        {:case Err(msg)}
            throw new Error("@{msg}")
    {/match};
};
```

Match with wildcard:

```rust
let count = 5;

let code = ts_template! {
    {#match count}
        {:case 0}
            console.log("none");
        {:case 1}
            console.log("one");
        {:case _}
            console.log("many");
    {/match}
};
```

### Iteration: `{#for item in list}...{/for}`

```rust
let fields = vec!["name", "email", "age"];

let code = ts_template! {
    function toJSON() {
        const result = {};
        {#for field in fields}
            result.@{field} = this.@{field};
        {/for}
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

### Tuple Destructuring in Loops

```rust
let items = vec![("user", "User"), ("post", "Post")];

let code = ts_template! {
    {#for (key, class_name) in items}
        const @{key} = new @{class_name}();
    {/for}
};
```

### Local Constants: `{%let name = expr}`

Define local variables within the template scope:

```rust
let items = vec![("user", "User"), ("post", "Post")];

let code = ts_template! {
    {#for (key, class_name) in items}
        {%let upper = class_name.to_uppercase()}
        console.log("Processing @{upper}");
        const @{key} = new @{class_name}();
    {/for}
};
```

This is useful for computing derived values inside loops without cluttering the Rust code.

### TsStream Injection: `{%typescript stream}`

Inject another TsStream into your template, preserving both its source code and runtime patches (like imports added via `add_import()`):

```rust
// Create a helper method with its own import
let mut helper = body! {
    validateEmail(email: string): boolean {
        return Result.ok(true);
    }
};
helper.add_import("Result", "macroforge/result");

// Inject the helper into the main template
let result = body! {
    {%typescript helper}

    process(data: Record<string, unknown>): void {
        // ...
    }
};
// result now includes helper's source AND its Result import
```

This is essential for composing multiple macro outputs while preserving imports and patches:

```rust
let extra_methods = if include_validation {
    Some(body! {
        validate(): boolean { return true; }
    })
} else {
    None
};

body! {
    mainMethod(): void {}

    {#if let Some(methods) = extra_methods}
        {%typescript methods}
    {/if}
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

1. **Compile-Time**: The template is parsed during macro expansion
2. **String Building**: Generates Rust code that builds a TypeScript string at runtime
3. **SWC Parsing**: The generated string is parsed with SWC to produce a typed AST
4. **Result**: Returns `Stmt` that can be used in `MacroResult` patches

## Return Type

`ts_template!` returns a `Result<Stmt, TsSynError>` by default. The macro automatically unwraps and provides helpful error messages showing the generated TypeScript code if parsing fails.

## Nesting and Regular TypeScript

You can mix template syntax with regular TypeScript. Braces `{}` are recognized as either:

- **Template tags** if they start with `#`, `%`, `:`, or `/`
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
    {#for (class_name, fields) in classes}
        @{class_name}.prototype.toJSON = function() {
            return {
                {#for field in fields}
                    @{field}: this.@{field},
                {/for}
            };
        };
    {/for}
}
```

## Comparison with Alternatives

| Approach             | Pros                               | Cons                            |
| -------------------- | ---------------------------------- | ------------------------------- |
| **`ts_quote!`**      | Compile-time validation, type-safe | Can't handle Vec<Stmt>, verbose |
| **`parse_ts_str()`** | Maximum flexibility                | Runtime parsing, less readable  |
| **`ts_template!`**   | Readable, handles loops/conditions | Small runtime parsing overhead  |

## Best Practices

1. Use `ts_template!` for complex code generation with loops/conditions
2. Use `ts_quote!` for simple, static statements
3. Keep templates readable - extract complex logic into variables
4. Don't nest templates too deeply - split into helper functions

## Error Messages

If the generated TypeScript is invalid, you'll see:

```
Failed to parse generated TypeScript:
User.prototype.toJSON = function( {
    return {};
}
```

This shows you exactly what was generated, making debugging easy!
