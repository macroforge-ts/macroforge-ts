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

let code = ts_template! &#123;
    @&#123;class_name&#125;.prototype.@&#123;method&#125; = function() &#123;
        return "User instance";
    &#125;;
&#125;;
``` **Generates:**
 ```
User.prototype.toString = function () &#123;
  return "User instance";
&#125;;
``` ## Identifier Concatenation: `{| content |}`
 When you need to build identifiers dynamically (like `getUser`, `setName`), use the ident block syntax. Everything inside `{| |}` is concatenated without spaces:
 ```
let field_name = "User";

let code = ts_template! &#123;
    function &#123;|get@&#123;field_name&#125;|&#125;() &#123;
        return this.@&#123;field_name.to_lowercase()&#125;;
    &#125;
&#125;;
``` **Generates:**
 ```
function getUser() &#123;
  return this.user;
&#125;
``` Without ident blocks, `@{}` always adds a space after for readability. Use `{| |}` when you explicitly want concatenation:
 ```
let name = "Status";

// With space (default behavior)
ts_template! &#123; namespace @&#123;name&#125; &#125;  // → "namespace Status"

// Without space (ident block)
ts_template! &#123; &#123;|namespace@&#123;name&#125;|&#125; &#125;  // → "namespaceStatus"
``` Multiple interpolations can be combined:
 ```
let entity = "user";
let action = "create";

ts_template! &#123; &#123;|@&#123;entity&#125;_@&#123;action&#125;|&#125; &#125;  // → "user_create"
``` ## Comments: `{> "..." <}` and `{>> "..." <<}`
 Since Rust's tokenizer strips whitespace before macros see them, use string literals to preserve exact spacing in comments:
 ### Block Comments
 Use `{> "comment" <}` for block comments:
 ```
let code = ts_template! &#123;
    &#123;> "This is a block comment" &#x3C;&#125;
    const x = 42;
&#125;;
``` **Generates:**
 ```
/* This is a block comment */
const x = 42;
``` ### Doc Comments (JSDoc)
 Use `{>> "doc" <<}` for JSDoc comments:
 ```
let code = ts_template! &#123;
    &#123;>> "@param &#123;string&#125; name - The user's name" &#x3C;&#x3C;&#125;
    &#123;>> "@returns &#123;string&#125; A greeting message" &#x3C;&#x3C;&#125;
    function greet(name: string): string &#123;
        return "Hello, " + name;
    &#125;
&#125;;
``` **Generates:**
 ```
/** @param &#123;string&#125; name - The user's name */
/** @returns &#123;string&#125; A greeting message */
function greet(name: string): string &#123;
    return "Hello, " + name;
&#125;
``` ### Comments with Interpolation
 Use `format!()` or similar to build dynamic comment strings:
 ```
let param_name = "userId";
let param_type = "number";
let comment = format!("@param &#123;&#123;&#123;&#125;&#125;&#125; &#123;&#125; - The user ID", param_type, param_name);

let code = ts_template! &#123;
    &#123;>> @&#123;comment&#125; &#x3C;&#x3C;&#125;
    function getUser(userId: number) &#123;&#125;
&#125;;
``` **Generates:**
 ```
/** @param &#123;number&#125; userId - The user ID */
function getUser(userId: number) &#123;&#125;
``` ## String Interpolation: `"text @{expr}"`
 Interpolation works automatically inside string literals - no `format!()` needed:
 ```
let name = "World";
let count = 42;

let code = ts_template! &#123;
    console.log("Hello @&#123;name&#125;!");
    console.log("Count: @&#123;count&#125;, doubled: @&#123;count * 2&#125;");
&#125;;
``` **Generates:**
 ```
console.log("Hello World!");
console.log("Count: 42, doubled: 84");
``` This also works with method calls and complex expressions:
 ```
let field = "username";

let code = ts_template! &#123;
    throw new Error("Invalid @&#123;field.to_uppercase()&#125;");
&#125;;
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
`Hello $&#123;this.name&#125;, you are a User`
``` ## Conditionals: `{#if}...{/if}`
 Basic conditional:
 ```
let needs_validation = true;

let code = ts_template! &#123;
    function save() &#123;
        &#123;#if needs_validation&#125;
            if (!this.isValid()) return false;
        &#123;/if&#125;
        return this.doSave();
    &#125;
&#125;;
``` ### If-Else
 ```
let has_default = true;

let code = ts_template! &#123;
    &#123;#if has_default&#125;
        return defaultValue;
    &#123;:else&#125;
        throw new Error("No default");
    &#123;/if&#125;
&#125;;
``` ### If-Else-If Chains
 ```
let level = 2;

let code = ts_template! &#123;
    &#123;#if level == 1&#125;
        console.log("Level 1");
    &#123;:else if level == 2&#125;
        console.log("Level 2");
    &#123;:else&#125;
        console.log("Other level");
    &#123;/if&#125;
&#125;;
``` ## Pattern Matching: `{#if let}`
 Use `if let` for pattern matching on `Option`, `Result`, or other Rust enums:
 ```
let maybe_name: Option&#x3C;&#x26;str> = Some("Alice");

let code = ts_template! &#123;
    &#123;#if let Some(name) = maybe_name&#125;
        console.log("Hello, @&#123;name&#125;!");
    &#123;:else&#125;
        console.log("Hello, anonymous!");
    &#123;/if&#125;
&#125;;
``` **Generates:**
 ```
console.log("Hello, Alice!");
``` This is useful when working with optional values from your IR:
 ```
let code = ts_template! &#123;
    &#123;#if let Some(default_val) = field.default_value&#125;
        this.@&#123;field.name&#125; = @&#123;default_val&#125;;
    &#123;:else&#125;
        this.@&#123;field.name&#125; = undefined;
    &#123;/if&#125;
&#125;;
``` ## Match Expressions: `{#match}`
 Use `match` for exhaustive pattern matching:
 ```
enum Visibility &#123; Public, Private, Protected &#125;
let visibility = Visibility::Public;

let code = ts_template! &#123;
    &#123;#match visibility&#125;
        &#123;:case Visibility::Public&#125;
            public
        &#123;:case Visibility::Private&#125;
            private
        &#123;:case Visibility::Protected&#125;
            protected
    &#123;/match&#125;
    field: string;
&#125;;
``` **Generates:**
 ```
public field: string;
``` ### Match with Value Extraction
 ```
let result: Result&#x3C;i32, &#x26;str> = Ok(42);

let code = ts_template! &#123;
    const value = &#123;#match result&#125;
        &#123;:case Ok(val)&#125;
            @&#123;val&#125;
        &#123;:case Err(msg)&#125;
            throw new Error("@&#123;msg&#125;")
    &#123;/match&#125;;
&#125;;
``` ### Match with Wildcard
 ```
let count = 5;

let code = ts_template! &#123;
    &#123;#match count&#125;
        &#123;:case 0&#125;
            console.log("none");
        &#123;:case 1&#125;
            console.log("one");
        &#123;:case _&#125;
            console.log("many");
    &#123;/match&#125;
&#125;;
``` ## Iteration: `{#for}`
 ```
let fields = vec!["name", "email", "age"];

let code = ts_template! &#123;
    function toJSON() &#123;
        const result = &#123;&#125;;
        &#123;#for field in fields&#125;
            result.@&#123;field&#125; = this.@&#123;field&#125;;
        &#123;/for&#125;
        return result;
    &#125;
&#125;;
``` **Generates:**
 ```
function toJSON() &#123;
  const result = &#123;&#125;;
  result.name = this.name;
  result.email = this.email;
  result.age = this.age;
  return result;
&#125;
``` ### Tuple Destructuring in Loops
 ```
let items = vec![("user", "User"), ("post", "Post")];

let code = ts_template! &#123;
    &#123;#for (key, class_name) in items&#125;
        const @&#123;key&#125; = new @&#123;class_name&#125;();
    &#123;/for&#125;
&#125;;
``` ### Nested Iterations
 ```
let classes = vec![
    ("User", vec!["name", "email"]),
    ("Post", vec!["title", "content"]),
];

ts_template! &#123;
    &#123;#for (class_name, fields) in classes&#125;
        @&#123;class_name&#125;.prototype.toJSON = function() &#123;
            return &#123;
                &#123;#for field in fields&#125;
                    @&#123;field&#125;: this.@&#123;field&#125;,
                &#123;/for&#125;
            &#125;;
        &#125;;
    &#123;/for&#125;
&#125;
``` ## While Loops: `{#while}`
 Use `while` for loops that need to continue until a condition is false:
 ```
let items = get_items();
let mut idx = 0;

let code = ts_template! &#123;
    &#123;$let mut i = 0&#125;
    &#123;#while i &#x3C; items.len()&#125;
        console.log("Item @&#123;i&#125;");
        &#123;$do i += 1&#125;
    &#123;/while&#125;
&#125;;
``` ### While-Let Pattern Matching
 Use `while let` for iterating with pattern matching, similar to `if let`:
 ```
let mut items = vec!["a", "b", "c"].into_iter();

let code = ts_template! &#123;
    &#123;#while let Some(item) = items.next()&#125;
        console.log("@&#123;item&#125;");
    &#123;/while&#125;
&#125;;
``` **Generates:**
 ```
console.log("a");
console.log("b");
console.log("c");
``` This is especially useful when working with iterators or consuming optional values:
 ```
let code = ts_template! &#123;
    &#123;#while let Some(next_field) = remaining_fields.pop()&#125;
        result.@&#123;next_field.name&#125; = this.@&#123;next_field.name&#125;;
    &#123;/while&#125;
&#125;;
``` ## Local Constants: `{$let}`
 Define local variables within the template scope:
 ```
let items = vec![("user", "User"), ("post", "Post")];

let code = ts_template! &#123;
    &#123;#for (key, class_name) in items&#125;
        &#123;$let upper = class_name.to_uppercase()&#125;
        console.log("Processing @&#123;upper&#125;");
        const @&#123;key&#125; = new @&#123;class_name&#125;();
    &#123;/for&#125;
&#125;;
``` This is useful for computing derived values inside loops without cluttering the Rust code.
 ## Mutable Variables: `{$let mut}`
 When you need to modify a variable within the template (e.g., in a `while` loop), use `{$let mut}`:
 ```
let code = ts_template! &#123;
    &#123;$let mut count = 0&#125;
    &#123;#for item in items&#125;
        console.log("Item @&#123;count&#125;: @&#123;item&#125;");
        &#123;$do count += 1&#125;
    &#123;/for&#125;
    console.log("Total: @&#123;count&#125;");
&#125;;
``` ## Side Effects: `{$do}`
 Execute an expression for its side effects without producing output. This is commonly used with mutable variables:
 ```
let code = ts_template! &#123;
    &#123;$let mut results: Vec&#x3C;String> = Vec::new()&#125;
    &#123;#for field in fields&#125;
        &#123;$do results.push(format!("this.&#123;&#125;", field))&#125;
    &#123;/for&#125;
    return [@&#123;results.join(", ")&#125;];
&#125;;
``` Common uses for `{$do}`:
 - Incrementing counters: `{$do i += 1}`
 - Building collections: `{$do vec.push(item)}`
 - Setting flags: `{$do found = true}`
 - Any mutating operation
 ## TsStream Injection: `{$typescript}`
 Inject another TsStream into your template, preserving both its source code and runtime patches (like imports added via `add_import()`):
 ```
// Create a helper method with its own import
let mut helper = body! &#123;
    validateEmail(email: string): boolean &#123;
        return Result.ok(true);
    &#125;
&#125;;
helper.add_import("Result", "macroforge/utils");

// Inject the helper into the main template
let result = body! &#123;
    &#123;$typescript helper&#125;

    process(data: Record&#x3C;string, unknown>): void &#123;
        // ...
    &#125;
&#125;;
// result now includes helper's source AND its Result import
``` This is essential for composing multiple macro outputs while preserving imports and patches:
 ```
let extra_methods = if include_validation &#123;
    Some(body! &#123;
        validate(): boolean &#123; return true; &#125;
    &#125;)
&#125; else &#123;
    None
&#125;;

body! &#123;
    mainMethod(): void &#123;&#125;

    &#123;#if let Some(methods) = extra_methods&#125;
        &#123;$typescript methods&#125;
    &#123;/if&#125;
&#125;
``` ## Escape Syntax
 If you need a literal `@{` in your output (not interpolation), use `@@{`:
 ```
ts_template! &#123;
    // This outputs a literal @&#123;foo&#125;
    const example = "Use @@&#123;foo&#125; for templates";
&#125;
``` **Generates:**
 ```
// This outputs a literal @&#123;foo&#125;
const example = "Use @&#123;foo&#125; for templates";
``` ## Complete Example: JSON Derive Macro
 Here's a comparison showing how `ts_template!` simplifies code generation:
 ### Before (Manual AST Building)
 ```
pub fn derive_json_macro(input: TsStream) -> MacroResult &#123;
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &#x26;input.data &#123;
        Data::Class(class) => &#123;
            let class_name = input.name();

            let mut body_stmts = vec![ts_quote!( const result = &#123;&#125;; as Stmt )];

            for field_name in class.field_names() &#123;
                body_stmts.push(ts_quote!(
                    result.$(ident!("&#123;&#125;", field_name)) = this.$(ident!("&#123;&#125;", field_name));
                    as Stmt
                ));
            &#125;

            body_stmts.push(ts_quote!( return result; as Stmt ));

            let runtime_code = fn_assign!(
                member_expr!(Expr::Ident(ident!(class_name)), "prototype"),
                "toJSON",
                body_stmts
            );

            // ...
        &#125;
    &#125;
&#125;
``` ### After (With ts_template!)
 ```
pub fn derive_json_macro(input: TsStream) -> MacroResult &#123;
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &#x26;input.data &#123;
        Data::Class(class) => &#123;
            let class_name = input.name();
            let fields = class.field_names();

            let runtime_code = ts_template! &#123;
                @&#123;class_name&#125;.prototype.toJSON = function() &#123;
                    const result = &#123;&#125;;
                    &#123;#for field in fields&#125;
                        result.@&#123;field&#125; = this.@&#123;field&#125;;
                    &#123;/for&#125;
                    return result;
                &#125;;
            &#125;;

            // ...
        &#125;
    &#125;
&#125;
``` ## How It Works
 1. **Compile-Time:** The template is parsed during macro expansion
 2. **String Building:** Generates Rust code that builds a TypeScript string at runtime
 3. **SWC Parsing:** The generated string is parsed with SWC to produce a typed AST
 4. **Result:** Returns `Stmt` that can be used in `MacroResult` patches
 ## Return Type
 `ts_template!` returns a `Result<Stmt, TsSynError>` by default. The macro automatically unwraps and provides helpful error messages showing the generated TypeScript code if parsing fails:
 ```
Failed to parse generated TypeScript:
User.prototype.toJSON = function( &#123;
    return &#123;&#125;;
&#125;
``` This shows you exactly what was generated, making debugging easy!
 ## Nesting and Regular TypeScript
 You can mix template syntax with regular TypeScript. Braces `{}` are recognized as either:
 - **Template tags** if they start with `#`, `$`, `:`, or `/`
 - **Regular TypeScript blocks** otherwise
 ```
ts_template! &#123;
    const config = &#123;
        &#123;#if use_strict&#125;
            strict: true,
        &#123;:else&#125;
            strict: false,
        &#123;/if&#125;
        timeout: 5000
    &#125;;
&#125;
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