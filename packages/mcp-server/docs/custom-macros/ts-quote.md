# Template Syntax

*The `macroforge_ts_quote` crate provides template-based code generation for TypeScript. The `ts_template!` macro uses Rust-inspired syntax for control flow and interpolation, making it easy to generate complex TypeScript code.*

## Available Macros

| `ts_template!` 
| Any TypeScript code 
| General code generation 

| `body!` 
| Class body members 
| Methods and properties

## Quick Reference

| `@&#123;expr&#125;` 
| Interpolate a Rust expression (adds space after) 

| `&#123;| content |&#125;` 
| Ident block: concatenates without spaces (e.g., `&#123;|get@&#123;name&#125;|&#125;` → `getUser`) 

| `&#123;> "comment" <&#125;`
| Block comment: outputs `/* comment */` (string preserves whitespace)

| `&#123;>> "doc" <<&#125;`
| Doc comment: outputs `/** doc */` (string preserves whitespace) 

| `@@&#123;` 
| Escape for literal `@&#123;` (e.g., `"@@&#123;foo&#125;"` → `@&#123;foo&#125;`) 

| `"text @&#123;expr&#125;"` 
| String interpolation (auto-detected) 

| `"'^template $&#123;js&#125;^'"` 
| JS backtick template literal (outputs ``template $&#123;js&#125;``) 

| `&#123;#if cond&#125;...&#123;/if&#125;` 
| Conditional block 

| `&#123;#if cond&#125;...&#123;:else&#125;...&#123;/if&#125;` 
| Conditional with else 

| `&#123;#if a&#125;...&#123;:else if b&#125;...&#123;:else&#125;...&#123;/if&#125;` 
| Full if/else-if/else chain 

| `&#123;#if let pattern = expr&#125;...&#123;/if&#125;` 
| Pattern matching if-let 

| `&#123;#match expr&#125;&#123;:case pattern&#125;...&#123;/match&#125;` 
| Match expression with case arms 

| `&#123;#for item in list&#125;...&#123;/for&#125;` 
| Iterate over a collection 

| `&#123;#while cond&#125;...&#123;/while&#125;` 
| While loop 

| `&#123;#while let pattern = expr&#125;...&#123;/while&#125;` 
| While-let pattern matching loop 

| `&#123;$let name = expr&#125;` 
| Define a local constant 

| `&#123;$let mut name = expr&#125;` 
| Define a mutable local variable 

| `&#123;$do expr&#125;` 
| Execute a side-effectful expression 

| `&#123;$typescript stream&#125;` 
| Inject a TsStream, preserving its source and runtime_patches (imports)

**Note:** A single `@` not followed by `&#123;` passes through unchanged (e.g., `email@domain.com` works as expected).

## Interpolation: `@&#123;expr&#125;`

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

## Identifier Concatenation: `&#123;| content |&#125;`

When you need to build identifiers dynamically (like `getUser`, `setName`), use the ident block syntax. Everything inside `&#123;| |&#125;` is concatenated without spaces:

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

Without ident blocks, `@&#123;&#125;` always adds a space after for readability. Use `&#123;| |&#125;` when you explicitly want concatenation:

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

## Comments: `&#123;> ... <&#125;` and `&#123;>> ... <<&#125;`

Since Rust's tokenizer strips comments before macros see them, you can't write JSDoc comments directly. Instead, use the comment syntax to output JavaScript comments:

### Block Comments

Use `&#123;> comment <&#125;` for block comments:

```rust
let code = ts_template! {
    {> This is a block comment <}
    const x = 42;
};
```

**Generates:**

```typescript
/* This is a block comment */
const x = 42;
```

### Doc Comments (JSDoc)

Use `&#123;>> doc <<&#125;` for JSDoc comments:

```rust
let code = ts_template! {
    {>> @param {string} name - The user's name <<}
    {>> @returns {string} A greeting message <<}
    function greet(name: string): string {
        return "Hello, " + name;
    }
};
```

**Generates:**

```typescript
/** @param {string} name - The user's name */
/** @returns {string} A greeting message */
function greet(name: string): string {
    return "Hello, " + name;
}
```

### Comments with Interpolation

Comments support `@&#123;expr&#125;` interpolation for dynamic content:

```rust
let param_name = "userId";
let param_type = "number";

let code = ts_template! {
    {>> @param {@{param_type}} @{param_name} - The user ID <<}
    function getUser(userId: number) {}
};
```

**Generates:**

```typescript
/** @param {number} userId - The user ID */
function getUser(userId: number) {}
```

## String Interpolation: `"text @&#123;expr&#125;"`

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

## Backtick Template Literals: `"'^...^'"`

For JavaScript template literals (backtick strings), use the `'^...^'` syntax. This outputs actual backticks and passes through `${"${}"}` for JS interpolation:

```rust
let tag_name = "div";

let code = ts_template! {
    const html = "'^<@{tag_name}>\${content}</@{tag_name}>^'";
};
```

**Generates:**

<CodeBlock code={'const html = `${content}`;'} lang="typescript" />

You can mix Rust `@&#123;&#125;` interpolation (evaluated at macro expansion time) with JS `${"${}"}` interpolation (evaluated at runtime):

```rust
let class_name = "User";

let code = ts_template! {
    "'^Hello \${this.name}, you are a @{class_name}^'"
};
```

**Generates:**

<CodeBlock code={'`Hello ${this.name}, you are a User`'} lang="typescript" />

## Conditionals: `&#123;#if&#125;...&#123;/if&#125;`

Basic conditional:

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

### If-Else

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

### If-Else-If Chains

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

## Pattern Matching: `&#123;#if let&#125;`

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

## Match Expressions: `&#123;#match&#125;`

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

### Match with Value Extraction

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

### Match with Wildcard

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

## Iteration: `&#123;#for&#125;`

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

### Nested Iterations

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

## While Loops: `&#123;#while&#125;`

Use `while` for loops that need to continue until a condition is false:

```rust
let items = get_items();
let mut idx = 0;

let code = ts_template! {
    {$let mut i = 0}
    {#while i < items.len()}
        console.log("Item @{i}");
        {$do i += 1}
    {/while}
};
```

### While-Let Pattern Matching

Use `while let` for iterating with pattern matching, similar to `if let`:

```rust
let mut items = vec!["a", "b", "c"].into_iter();

let code = ts_template! {
    {#while let Some(item) = items.next()}
        console.log("@{item}");
    {/while}
};
```

**Generates:**

```typescript
console.log("a");
console.log("b");
console.log("c");
```

This is especially useful when working with iterators or consuming optional values:

```rust
let code = ts_template! {
    {#while let Some(next_field) = remaining_fields.pop()}
        result.@{next_field.name} = this.@{next_field.name};
    {/while}
};
```

## Local Constants: `&#123;$let&#125;`

Define local variables within the template scope:

```rust
let items = vec![("user", "User"), ("post", "Post")];

let code = ts_template! {
    {#for (key, class_name) in items}
        {$let upper = class_name.to_uppercase()}
        console.log("Processing @{upper}");
        const @{key} = new @{class_name}();
    {/for}
};
```

This is useful for computing derived values inside loops without cluttering the Rust code.

## Mutable Variables: `&#123;$let mut&#125;`

When you need to modify a variable within the template (e.g., in a `while` loop), use `&#123;$let mut&#125;`:

```rust
let code = ts_template! {
    {$let mut count = 0}
    {#for item in items}
        console.log("Item @{count}: @{item}");
        {$do count += 1}
    {/for}
    console.log("Total: @{count}");
};
```

## Side Effects: `&#123;$do&#125;`

Execute an expression for its side effects without producing output. This is commonly used with mutable variables:

```rust
let code = ts_template! {
    {$let mut results: Vec<String> = Vec::new()}
    {#for field in fields}
        {$do results.push(format!("this.{}", field))}
    {/for}
    return [@{results.join(", ")}];
};
```

Common uses for `&#123;$do&#125;`:

- Incrementing counters: `&#123;$do i += 1&#125;`

- Building collections: `&#123;$do vec.push(item)&#125;`

- Setting flags: `&#123;$do found = true&#125;`

- Any mutating operation

## TsStream Injection: `&#123;$typescript&#125;`

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
    {$typescript helper}

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
        {$typescript methods}
    {/if}
}
```

## Escape Syntax

If you need a literal `@&#123;` in your output (not interpolation), use `@@&#123;`:

```rust
ts_template! {
    // This outputs a literal @{foo}
    const example = "Use @@{foo} for templates";
}
```

**Generates:**

```typescript
// This outputs a literal @{foo}
const example = "Use @{foo} for templates";
```

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