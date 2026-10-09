# Template Syntax

The `macroforge_ts_quote` crate provides two macros for generating TypeScript. `ts_template!` writes
TypeScript as text, with Svelte-like control flow and Rust interpolation; it is what most macros
use. `ts_quote!` builds a single AST node and checks its syntax when your crate compiles.

## Available Macros

| Macro                                      | Output                             | Use Case                                                         |
| ------------------------------------------ | ---------------------------------- | ---------------------------------------------------------------- |
| `ts_template!`                             | A `TsStream` of TypeScript source  | Generating code: methods, functions, declarations                |
| `ts_template!(Within &lbrace; … &rbrace;)` | The same, placed in the class body | Methods and properties; see [Positions](#positions)              |
| `ts_quote!`                                | One oxc AST node                   | Building or inspecting syntax trees; see [ts\_quote!](#ts-quote) |
| `ts_ident!`                                | An identifier                      | Names built from strings; see [ts\_ident!](#ts-ident)            |

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
| `{$let name = expr}`                       | Define a local constant                                       |
| `{$let mut name = expr}`                   | Define a mutable local variable                               |
| `{$do expr}`                               | Execute a side-effectful expression                           |
| `{$typescript stream}`                     | Inject a `TsStream`, with its patches, imports and warnings   |

**Note:** A single `@` not followed by `{` passes through unchanged (e.g., `email@domain.com` works
as expected).

## Positions

A template's first word can say where a derive's code goes: `Top` or `Bottom` of the file, `Above`
or `Below` the target, or `Within` the class body. Without one, the code goes `Below`.

Rust

```
let members = ts_template!(Within {
    toString(): string { return "User"; }
});

let setup = ts_template!(Top {
    const registry = new Map<string, unknown>();
});
```

A template with a position marks its code with it, so the position holds when the stream is injected
into another template. See [Insert Positions](../../docs/custom-macros/output#insert-positions).

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
User.prototype.toString = function() {
    return "User instance";
};
```

`@{expr}` accepts any value that implements `ToTsString`: strings, numbers, booleans and `char`, the
identifiers `ts_ident!` makes, a `TsStream`, and references or smart pointers to any of them.

### Spacing

The output keeps the spacing of the template as written. Tokens written next to each other stay
joined, and tokens with space between them stay apart, so building identifiers needs no special
syntax:

Rust

```
let name = "User";

let code = ts_template! {
    function get@{name}(): @{name} { ... }   // function getUser(): User { ... }
    const @{name.to_lowercase()}_id = 1;     // const user_id = 1;
};
```

Line breaks and indentation follow the template too, so the generated code is laid out the way the
template is.

## String Interpolation: `"text @{expr}"`

Interpolation works automatically inside string literals - no `format!()` needed:

Rust

```
let name = "World";
let count = 42;

let code = ts_template! {
    console.log("Hello @{name}!");
    console.log("Count: @{count}, doubled: @{count * 2}");
};
```

**Generates:**

TypeScript

```
console.log("Hello World!");
console.log("Count: 42, doubled: 84");
```

This also works with method calls and complex expressions:

Rust

```
let field = "username";

let code = ts_template! {
    throw new Error("Invalid @{field.to_uppercase()}");
};
```

Text inside `@{...}` that is not a Rust expression is a compile error.

## Backtick Template Literals: `"'^...^'"`

For JavaScript template literals (backtick strings), use the `'^...^'` syntax. This outputs actual
backticks and passes through `${"${}"}` for JS interpolation:

Rust

```
let tag_name = "div";

let code = ts_template! {
    const html = "'^<@{tag_name}>${content}</@{tag_name}>^'";
};
```

**Generates:**

TypeScript

```
const html = `<div>${content}</div>`;
```

You can mix Rust `@{}` interpolation (evaluated at macro expansion time) with JS `${"${}"}`
interpolation (evaluated at runtime):

Rust

```
let class_name = "User";

let code = ts_template! {
    "'^Hello ${this.name}, you are a @{class_name}^'"
};
```

**Generates:**

TypeScript

```
`Hello ${this.name}, you are a User`
```

## Comments

Rust's tokenizer drops ordinary comments before a macro sees them, so a template marks the comments
it wants to emit. Write the comment as a string literal; `@{}` is interpolated in it:

Rust

```
let name = "User";

let code = ts_template! {
    {> "Generated for @{name}" <}
    {>> "Do not edit" <<}
    const version = 1;
};
```

**Generates:**

TypeScript

```
// Generated for User
/* Do not edit */
const version = 1;
```

A line comment whose text has several lines gets `//` on each, and a `*/` in a block comment's text
is broken up, so the comment never ends early.

### Doc Comments (JSDoc)

A Rust doc comment inside a template, `///` or `/** ... */`, becomes a JSDoc comment, with `@{}`
interpolated:

Rust

```
let field = "email";

let code = ts_template! {
    /// Returns the @{field} field.
    get@{field}(): string { return this.@{field}; }
};
```

**Generates:**

TypeScript

```
/** Returns the email field. */
getemail(): string { return this.email; }
```

## Conditionals: `{#if}...{/if}`

Basic conditional:

Rust

```
let needs_validation = true;

let code = ts_template! {
    function save() {
        {#if needs_validation}
            if (!this.isValid()) return false;
        {/if}
        return this.doSave();
    }
};
```

### If-Else

Rust

```
let has_default = true;

let code = ts_template! {
    {#if has_default}
        return defaultValue;
    {:else}
        throw new Error("No default");
    {/if}
};
```

### If-Else-If Chains

Rust

```
let level = 2;

let code = ts_template! {
    {#if level == 1}
        console.log("Level 1");
    {:else if level == 2}
        console.log("Level 2");
    {:else}
        console.log("Other level");
    {/if}
};
```

## Pattern Matching: `{#if let}`

Use `if let` for pattern matching on `Option`, `Result`, or other Rust enums:

Rust

```
let maybe_name: Option<&str> = Some("Alice");

let code = ts_template! {
    {#if let Some(name) = maybe_name}
        console.log("Hello, @{name}!");
    {:else}
        console.log("Hello, anonymous!");
    {/if}
};
```

**Generates:**

TypeScript

```
console.log("Hello, Alice!");
```

This is useful when working with optional values from your IR:

Rust

```
let code = ts_template! {
    {#for variant in enum_.variants()}
        {#if let Some(text) = variant.value.as_string()}
            case "@{text}": return "@{variant.name}";
        {/if}
    {/for}
};
```

## Match Expressions: `{#match}`

Use `match` for exhaustive pattern matching:

Rust

```
use macroforge_ts::ts_syn::Visibility;

let code = ts_template! {
    {#match field.visibility}
        {:case Visibility::Public}
            public
        {:case Visibility::Private}
            private
        {:case Visibility::Protected}
            protected
    {/match}
    @{field.name}: string;
};
```

### Match with Value Extraction

Rust

```
let result: Result<i32, &str> = Ok(42);

let code = ts_template! {
    const value = {#match result}
        {:case Ok(val)}
            @{val}
        {:case Err(msg)}
            throw new Error("@{msg}")
    {/match};
};
```

### Match with Wildcard

Rust

```
let count = 5;

let code = ts_template! {
    {#match count}
        {:case 0}
            console.log("none");
        {:case 1}
            console.log("one");
        {:case _}
            console.log("many");
    {/match}
};
```

## Iteration: `{#for}`

Rust

```
let fields = vec!["name", "email", "age"];

let code = ts_template! {
    function toJSON() {
        const result = {};
        {#for field in fields}
            result.@{field} = this.@{field};
        {/for}
        return result;
    }
};
```

**Generates:**

TypeScript

```
function toJSON() {
    const result = {};
    result.name = this.name;
    result.email = this.email;
    result.age = this.age;
    return result;
}
```

### Tuple Destructuring in Loops

Rust

```
let items = vec![("user", "User"), ("post", "Post")];

let code = ts_template! {
    {#for (key, class_name) in items}
        const @{key} = new @{class_name}();
    {/for}
};
```

### Nested Iterations

Rust

```
let classes = vec![
    ("User", vec!["name", "email"]),
    ("Post", vec!["title", "content"]),
];

ts_template! {
    {#for (class_name, fields) in classes}
        @{class_name}.prototype.toJSON = function() {
            return {
                {#for field in fields}
                    @{field}: this.@{field},
                {/for}
            };
        };
    {/for}
}
```

## While Loops: `{#while}`

Use `while` for loops that need to continue until a condition is false:

Rust

```
let items = vec!["a", "b", "c"];

let code = ts_template! {
    {$let mut i = 0}
    {#while i < items.len()}
        console.log("Item @{i}");
        {$do i += 1}
    {/while}
};
```

### While-Let Pattern Matching

Use `while let` for iterating with pattern matching, similar to `if let`:

Rust

```
let mut items = vec!["a", "b", "c"].into_iter();

let code = ts_template! {
    {#while let Some(item) = items.next()}
        console.log("@{item}");
    {/while}
};
```

**Generates:**

TypeScript

```
console.log("a");
console.log("b");
console.log("c");
```

## Local Constants: `{$let}`

Define local variables within the template scope:

Rust

```
let items = vec![("user", "User"), ("post", "Post")];

let code = ts_template! {
    {#for (key, class_name) in items}
        {$let upper = class_name.to_uppercase()}
        console.log("Processing @{upper}");
        const @{key} = new @{class_name}();
    {/for}
};
```

This is useful for computing derived values inside loops without cluttering the Rust code.

## Mutable Variables: `{$let mut}`

When you need to modify a variable within the template (e.g., in a `while` loop), use `{$let mut}`:

Rust

```
let code = ts_template! {
    {$let mut count = 0}
    {#for item in items}
        console.log("Item @{count}: @{item}");
        {$do count += 1}
    {/for}
    console.log("Total: @{count}");
};
```

## Side Effects: `{$do}`

Execute an expression for its side effects without producing output. This is commonly used with
mutable variables:

Rust

```
let code = ts_template! {
    {$let mut results: Vec<String> = Vec::new()}
    {#for field in fields}
        {$do results.push(format!("this.{}", field))}
    {/for}
    return [@{results.join(", ")}];
};
```

Common uses for `{$do}`:

- Incrementing counters: `{$do i += 1}`
- Building collections: `{$do vec.push(item)}`
- Setting flags: `{$do found = true}`
- Any mutating operation

## TsStream Injection: `{$typescript}`

Inject another `TsStream` into your template. Its source joins the output, and everything else it
carries comes along: patches, cross-module suffixes and warnings. Imports requested with
`add_import()` apply to the whole expansion whichever stream asked for them.

Rust

```
// Create a helper method with its own import
let mut helper = ts_template!(Within {
    validateEmail(email: string): boolean {
        return isEmail(email);
    }
});
helper.add_import("isEmail", "my-validation-lib");

// Inject the helper into the main template
let result = ts_template!(Within {
    {$typescript helper}

    process(data: Record<string, unknown>): void {
        // ...
    }
});
```

The injected value must be a `TsStream` you own; injection moves it. This is how optional parts of a
macro's output are composed:

Rust

```
let extra_methods = if include_validation {
    Some(ts_template!(Within {
        validate(): boolean { return true; }
    }))
} else {
    None
};

ts_template!(Within {
    mainMethod(): void {}

    {#if let Some(methods) = extra_methods}
        {$typescript methods}
    {/if}
})
```

## Escape Syntax

If you need a literal `@{` in your output (not interpolation), use `@@{`:

Rust

```
ts_template! {
    const example = "Use @@{foo} for templates";
}
```

**Generates:**

TypeScript

```
const example = "Use @{foo} for templates";
```

## Nesting and Regular TypeScript

You can mix template syntax with regular TypeScript. Braces `{}` are recognized as either:

- **Template tags** if they start with `#`, `:`, `/`, `$`, `%` or `>`
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

## Identifiers: `ts_ident!`

`ts_ident!` makes an identifier from a string or a format string. It is handy for names a macro
passes around before interpolating them:

Rust

```
use macroforge_ts::ts_syn::ts_ident;

let type_name = input.name();
let encode_fn = ts_ident!("{}Encode", type_name.to_lowercase()); // userEncode

let code = ts_template! {
    export function @{encode_fn}(value: @{type_name}): string { ... }
};
```

Note

`DeriveInput`'s `ident` field is a different identifier type, one that records where the name is. To
write a type's name, use `input.name()`.

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

## Complete Example: JSON Derive Macro

Here's a comparison showing how `ts_template!` simplifies code generation:

### Before (Manual String Building)

Rust

```
#[ts_macro_derive(JSON)]
pub fn derive_json_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);
    let Some(class) = input.as_class() else {
        return Err(MacroforgeError::new(input.error_span(), "@derive(JSON) needs a class"));
    };

    let mut body = String::from("const result = {};\\n");
    for field_name in class.field_names() {
        body.push_str(&format!("result.{field_name} = this.{field_name};\\n"));
    }
    body.push_str("return result;");

    Ok(TsStream::from_string(format!(
        "{}.prototype.toJSON = function() {{\\n{body}\\n}};",
        input.name()
    )))
}
```

### After (With ts\_template!)

Rust

```
#[ts_macro_derive(JSON)]
pub fn derive_json_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);
    let Some(class) = input.as_class() else {
        return Err(MacroforgeError::new(input.error_span(), "@derive(JSON) needs a class"));
    };
    let class_name = input.name();

    Ok(ts_template! {
        @{class_name}.prototype.toJSON = function() {
            const result = {};
            {#for field in class.field_names()}
                result.@{field} = this.@{field};
            {/for}
            return result;
        };
    })
}
```

## How It Works

1. **Rust compile time:** the template is turned into Rust code that writes TypeScript text, with
   the control flow as ordinary Rust `if`, `for` and `match`.
2. **Macro run time:** that code runs and builds the text, interpolating your values.
3. **Result:** a `TsStream` that can be returned directly as macro output. The text is not checked
   here, so a syntax error in it shows up when the expanded file is compiled;
   [`macroforge expand`](../../docs/custom-macros/testing-and-debugging#expand-a-file) shows exactly
   what was generated.

## Choosing a Macro

- Use `ts_template!` to generate code, especially with loops and conditions.
- Use `ts_quote!` when you need an AST node: to analyse or transform syntax, or to have a fixed
  snippet checked when your crate compiles.
- Keep templates readable: compute values in Rust before the template.
- Split large outputs into several templates and combine them with `{$typescript}`.
