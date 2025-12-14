# Template Syntax
 *The `macroforge_ts_quote` crate provides template-based code generation for TypeScript. The `ts_template!` macro uses Rust-inspired syntax for control flow and interpolation, making it easy to generate complex TypeScript code.*
 ## Available Macros
 | Macro | Output | Use Case |
| --- | --- | --- |
| `ts_template!` | Any TypeScript code | General code generation |
| `body!` | Class body members | Methods and properties |
 ## Quick Reference
 | Syntax | Description |
| --- | --- |
| `@{expr}` | Interpolate a Rust expression (adds space after) |
| `{| content |}` | Ident block: concatenates without spaces (e.g., `{|get@{name}|}` → `getUser`) |
| `{> "comment" <}` | Block comment: outputs `/* comment */` (string preserves whitespace) |
| `{>> "doc" <<}` | Doc comment: outputs `/** doc */` (string preserves whitespace) |
| `@@{` | Escape for literal `@{` (e.g., `"@@{foo}"` → `@{foo}`) |
| `"text @{expr}"` | String interpolation (auto-detected) |
| `"'^template ${js}^'"` | JS backtick template literal (outputs ``template ${js}``) |
| `{#if cond}...{/if}` | Conditional block |
| `{#if cond}...{:else}...{/if}` | Conditional with else |
| {#if a}...{:else if b}...{:else}...{/if} | Full if/else-if/else chain |
| `{#if let pattern = expr}...{/if}` | Pattern matching if-let |
| {#match expr}{:case pattern}...{/match} | Match expression with case arms |
| `{#for item in list}...{/for}` | Iterate over a collection |
| `{#while cond}...{/while}` | While loop |
| `{#while let pattern = expr}...{/while}` | While-let pattern matching loop |
| `{$let name = expr}` | Define a local constant |
| `{$let mut name = expr}` | Define a mutable local variable |
| `{$do expr}` | Execute a side-effectful expression |
| `{$typescript stream}` | Inject a TsStream, preserving its source and runtime_patches (imports) |
 **Note:** A single `@` not followed by `{` passes through unchanged (e.g., `email@domain.com` works as expected).
 ## Interpolation: `@{expr}`
 Insert Rust expressions into the generated TypeScript:
 ```
let class_name = "User";
let method = "toString";

let code = ts_template! {
    @{class_name}.prototype.@{method} = function() {
        return "User instance";
    };
};
``` **Generates:**
 ```
User.prototype.toString = function () {
  return "User instance";
};
``` ## Identifier Concatenation: `{| content |}`
 When you need to build identifiers dynamically (like `getUser`, `setName`), use the ident block syntax. Everything inside `{| |}` is concatenated without spaces:
 ```
let field_name = "User";

let code = ts_template! {
    function {|get@{field_name}|}() {
        return this.@{field_name.to_lowercase()};
    }
};
``` **Generates:**
 ```
function getUser() {
  return this.user;
}
``` Without ident blocks, `@{}` always adds a space after for readability. Use `{| |}` when you explicitly want concatenation:
 ```
let name = "Status";

// With space (default behavior)
ts_template! { namespace @{name} }  // → "namespace Status"

// Without space (ident block)
ts_template! { {|namespace@{name}|} }  // → "namespaceStatus"
``` Multiple interpolations can be combined:
 ```
let entity = "user";
let action = "create";

ts_template! { {|@{entity}_@{action}|} }  // → "user_create"
``` ## Comments: `{> "..." <}` and `{>> "..." <<}`
 Since Rust's tokenizer strips whitespace before macros see them, use string literals to preserve exact spacing in comments:
 ### Block Comments
 Use `{> "comment" <}` for block comments:
 ```
let code = ts_template! {
    {> "This is a block comment" <}
    const x = 42;
};
``` **Generates:**
 ```
/* This is a block comment */
const x = 42;
``` ### Doc Comments (JSDoc)
 Use `{>> "doc" <<}` for JSDoc comments:
 ```
let code = ts_template! {
    {>> "@param {string} name - The user's name" <<}
    {>> "@returns {string} A greeting message" <<}
    function greet(name: string): string {
        return "Hello, " + name;
    }
};
``` **Generates:**
 ```
/** @param {string} name - The user's name */
/** @returns {string} A greeting message */
function greet(name: string): string {
    return "Hello, " + name;
}
``` ### Comments with Interpolation
 Use `format!()` or similar to build dynamic comment strings:
 ```
let param_name = "userId";
let param_type = "number";
let comment = format!("@param {{{}}} {} - The user ID", param_type, param_name);

let code = ts_template! {
    {>> @{comment} <<}
    function getUser(userId: number) {}
};
``` **Generates:**
 ```
/** @param {number} userId - The user ID */
function getUser(userId: number) {}
``` ## String Interpolation: `"text @{expr}"`
 Interpolation works automatically inside string literals - no `format!()` needed:
 ```
let name = "World";
let count = 42;

let code = ts_template! {
    console.log("Hello @{name}!");
    console.log("Count: @{count}, doubled: @{count * 2}");
};
``` **Generates:**
 ```
console.log("Hello World!");
console.log("Count: 42, doubled: 84");
``` This also works with method calls and complex expressions:
 ```
let field = "username";

let code = ts_template! {
    throw new Error("Invalid @{field.to_uppercase()}");
};
``` ## Backtick Template Literals: `"'^...^'"`
 For JavaScript template literals (backtick strings), use the `'^...^'` syntax. This outputs actual backticks and passes through `$${}` for JS interpolation:
 ```
let tag_name = "div";

let code = ts_template! {
    const html = "'^<@{tag_name}>${content}</@{tag_name}>^'";
};
``` **Generates:**
 ```
const html = `${content}`;
``` You can mix Rust `@{}` interpolation (evaluated at macro expansion time) with JS `$${}` interpolation (evaluated at runtime):
 ```
let class_name = "User";

let code = ts_template! {
    "'^Hello ${this.name}, you are a @{class_name}^'"
};
``` **Generates:**
 ```
`Hello ${this.name}, you are a User`
``` ## Conditionals: `{#if}...{/if}`
 Basic conditional:
 ```
let needs_validation = true;

let code = ts_template! {
    function save() {
        {#if needs_validation}
            if (!this.isValid()) return false;
        {/if}
        return this.doSave();
    }
};
``` ### If-Else
 ```
let has_default = true;

let code = ts_template! {
    {#if has_default}
        return defaultValue;
    {:else}
        throw new Error("No default");
    {/if}
};
``` ### If-Else-If Chains
 ```
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
``` ## Pattern Matching: `{#if let}`
 Use `if let` for pattern matching on `Option`, `Result`, or other Rust enums:
 ```
let maybe_name: Option<&str> = Some("Alice");

let code = ts_template! {
    {#if let Some(name) = maybe_name}
        console.log("Hello, @{name}!");
    {:else}
        console.log("Hello, anonymous!");
    {/if}
};
``` **Generates:**
 ```
console.log("Hello, Alice!");
``` This is useful when working with optional values from your IR:
 ```
let code = ts_template! {
    {#if let Some(default_val) = field.default_value}
        this.@{field.name} = @{default_val};
    {:else}
        this.@{field.name} = undefined;
    {/if}
};
``` ## Match Expressions: `{#match}`
 Use `match` for exhaustive pattern matching:
 ```
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
``` **Generates:**
 ```
public field: string;
``` ### Match with Value Extraction
 ```
let result: Result<i32, &str> = Ok(42);

let code = ts_template! {
    const value = {#match result}
        {:case Ok(val)}
            @{val}
        {:case Err(msg)}
            throw new Error("@{msg}")
    {/match};
};
``` ### Match with Wildcard
 ```
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
``` ## Iteration: `{#for}`
 ```
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
``` **Generates:**
 ```
function toJSON() {
  const result = {};
  result.name = this.name;
  result.email = this.email;
  result.age = this.age;
  return result;
}
``` ### Tuple Destructuring in Loops
 ```
let items = vec![("user", "User"), ("post", "Post")];

let code = ts_template! {
    {#for (key, class_name) in items}
        const @{key} = new @{class_name}();
    {/for}
};
``` ### Nested Iterations
 ```
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
``` ## While Loops: `{#while}`
 Use `while` for loops that need to continue until a condition is false:
 ```
let items = get_items();
let mut idx = 0;

let code = ts_template! {
    {$let mut i = 0}
    {#while i < items.len()}
        console.log("Item @{i}");
        {$do i += 1}
    {/while}
};
``` ### While-Let Pattern Matching
 Use `while let` for iterating with pattern matching, similar to `if let`:
 ```
let mut items = vec!["a", "b", "c"].into_iter();

let code = ts_template! {
    {#while let Some(item) = items.next()}
        console.log("@{item}");
    {/while}
};
``` **Generates:**
 ```
console.log("a");
console.log("b");
console.log("c");
``` This is especially useful when working with iterators or consuming optional values:
 ```
let code = ts_template! {
    {#while let Some(next_field) = remaining_fields.pop()}
        result.@{next_field.name} = this.@{next_field.name};
    {/while}
};
``` ## Local Constants: `{$let}`
 Define local variables within the template scope:
 ```
let items = vec![("user", "User"), ("post", "Post")];

let code = ts_template! {
    {#for (key, class_name) in items}
        {$let upper = class_name.to_uppercase()}
        console.log("Processing @{upper}");
        const @{key} = new @{class_name}();
    {/for}
};
``` This is useful for computing derived values inside loops without cluttering the Rust code.
 ## Mutable Variables: `{$let mut}`
 When you need to modify a variable within the template (e.g., in a `while` loop), use `{$let mut}`:
 ```
let code = ts_template! {
    {$let mut count = 0}
    {#for item in items}
        console.log("Item @{count}: @{item}");
        {$do count += 1}
    {/for}
    console.log("Total: @{count}");
};
``` ## Side Effects: `{$do}`
 Execute an expression for its side effects without producing output. This is commonly used with mutable variables:
 ```
let code = ts_template! {
    {$let mut results: Vec<String> = Vec::new()}
    {#for field in fields}
        {$do results.push(format!("this.{}", field))}
    {/for}
    return [@{results.join(", ")}];
};
``` Common uses for `{$do}`:
 - Incrementing counters: `{$do i += 1}`
 - Building collections: `{$do vec.push(item)}`
 - Setting flags: `{$do found = true}`
 - Any mutating operation
 ## TsStream Injection: `{$typescript}`
 Inject another TsStream into your template, preserving both its source code and runtime patches (like imports added via `add_import()`):
 ```
// Create a helper method with its own import
let mut helper = body! {
    validateEmail(email: string): boolean {
        return Result.ok(true);
    }
};
helper.add_import("Result", "macroforge/utils");

// Inject the helper into the main template
let result = body! {
    {$typescript helper}

    process(data: Record<string, unknown>): void {
        // ...
    }
};
// result now includes helper's source AND its Result import
``` This is essential for composing multiple macro outputs while preserving imports and patches:
 ```
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
``` ## Escape Syntax
 If you need a literal `@{` in your output (not interpolation), use `@@{`:
 ```
ts_template! {
    // This outputs a literal @{foo}
    const example = "Use @@{foo} for templates";
}
``` **Generates:**
 ```
// This outputs a literal @{foo}
const example = "Use @{foo} for templates";
``` ## Complete Example: JSON Derive Macro
 Here's a comparison showing how `ts_template!` simplifies code generation:
 ### Before (Manual AST Building)
 ```
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
``` ### After (With ts_template!)
 ```
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
``` ## How It Works
 1. **Compile-Time:** The template is parsed during macro expansion
 2. **String Building:** Generates Rust code that builds a TypeScript string at runtime
 3. **SWC Parsing:** The generated string is parsed with SWC to produce a typed AST
 4. **Result:** Returns `Stmt` that can be used in `MacroResult` patches
 ## Return Type
 `ts_template!` returns a `Result<Stmt, TsSynError>` by default. The macro automatically unwraps and provides helpful error messages showing the generated TypeScript code if parsing fails:
 ```
Failed to parse generated TypeScript:
User.prototype.toJSON = function( {
    return {};
}
``` This shows you exactly what was generated, making debugging easy!
 ## Nesting and Regular TypeScript
 You can mix template syntax with regular TypeScript. Braces `{}` are recognized as either:
 - **Template tags** if they start with `#`, `$`, `:`, or `/`
 - **Regular TypeScript blocks** otherwise
 ```
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
``` ## Comparison with Alternatives
 | Approach | Pros | Cons |
| --- | --- | --- |
| `ts_quote!` | Compile-time validation, type-safe | Can't handle Vec<Stmt>, verbose |
| `parse_ts_str()` | Maximum flexibility | Runtime parsing, less readable |
| `ts_template!` | Readable, handles loops/conditions | Small runtime parsing overhead |
 ## Best Practices
 1. Use `ts_template!` for complex code generation with loops/conditions
 2. Use `ts_quote!` for simple, static statements
 3. Keep templates readable - extract complex logic into variables
 4. Don't nest templates too deeply - split into helper functions