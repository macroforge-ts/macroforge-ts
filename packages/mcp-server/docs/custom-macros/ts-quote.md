# Template Syntax
 *The <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">macroforge_ts_quote</code> crate provides template-based code generation for TypeScript. The <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">ts_template!</code> macro uses Rust-inspired syntax for control flow and interpolation, making it easy to generate complex TypeScript code.*
 ## Available Macros
 | Macro | Output | Use Case |
| --- | --- | --- |
| ts_template! | Any TypeScript code | General code generation |
| body! | Class body members | Methods and properties |
 ## Quick Reference
 | Syntax | Description |
| --- | --- |
| @&#123;expr&#125; | Interpolate a Rust expression (adds space after) |
| &#123;|content|&#125; | Ident block: concatenates without spaces (e.g., `{|get@{name}|}` → getUser) |
| &#123;&gt;"comment"&lt;&#125; | Block comment: outputs /* comment */ (string preserves whitespace) |
| &#123;&gt;&gt;"doc"&lt;&lt;&#125; | Doc comment: outputs /** doc */ (string preserves whitespace) |
| @@&#123; | Escape for literal @&#123; (e.g., "@@&#123;foo&#125;" → @&#123;foo&#125;) |
| "text @&#123;expr&#125;" | String interpolation (auto-detected) |
| "'^template $&#123;js&#125;^'" | JS backtick template literal (outputs ``template ${js}``) |
| &#123;#ifcond&#125;...&#123;/if&#125; | Conditional block |
| `{#if cond}...{:else}...{/if}` | Conditional with else |
| {#if a}...{:else if b}...{:else}...{/if} | Full if/else-if/else chain |
| `{#if let pattern = expr}...{/if}` | Pattern matching if-let |
| {#match expr}{:case pattern}...{/match} | Match expression with case arms |
| `{#for item in list}...{/for}` | Iterate over a collection |
| &#123;#whilecond&#125;...&#123;/while&#125; | While loop |
| `{#while let pattern = expr}...{/while}` | While-let pattern matching loop |
| &#123;$let name=expr&#125; | Define a local constant |
| &#123;$let mut name=expr&#125; | Define a mutable local variable |
| &#123;$do expr&#125; | Execute a side-effectful expression |
| &#123;$typescript stream&#125; | Inject a TsStream, preserving its source and runtime_patches (imports) |
 **Note:** A single <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@</code> not followed by <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code> passes through unchanged (e.g., <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">email@domain.com</code> works as expected).
 ## Interpolation: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;expr<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code>
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
``` ## Identifier Concatenation: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">|<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">content<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">|&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code>
 When you need to build identifiers dynamically (like <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">getUser</code>, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">setName</code>), use the ident block syntax. Everything inside <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">|<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">|&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code> is concatenated without spaces:
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
``` Without ident blocks, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code> always adds a space after for readability. Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">|<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">|&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code> when you explicitly want concatenation:
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
``` ## Comments: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">gt;<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"..."<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">lt;<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code> and <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">gt;<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">gt;<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"..."<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">lt;<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">lt;<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code>
 Since Rust's tokenizer strips whitespace before macros see them, use string literals to preserve exact spacing in comments:
 ### Block Comments
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">gt;<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"comment"<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">lt;<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code> for block comments:
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
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">gt;<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">gt;<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"doc"<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">lt;<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">lt;<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code> for JSDoc comments:
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
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">format<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">!<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">()</code> or similar to build dynamic comment strings:
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
``` ## String Interpolation: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"text @&#123;expr&#125;"</code>
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
``` ## Backtick Template Literals: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"'^...^'"</code>
 For JavaScript template literals (backtick strings), use the `'^...^'` syntax. This outputs actual backticks and passes through <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">${<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"${}"<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code> for JS interpolation:
 ```
let tag_name = "div";

let code = ts_template! {
    const html = "'^<@{tag_name}>${content}</@{tag_name}>^'";
};
``` **Generates:**
 ```
const html = `${content}`;
``` You can mix Rust <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code> interpolation (evaluated at macro expansion time) with JS <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">${<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"${}"<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code> interpolation (evaluated at runtime):
 ```
let class_name = "User";

let code = ts_template! {
    "'^Hello ${this.name}, you are a @{class_name}^'"
};
``` **Generates:**
 ```
`Hello $&#123;this.name&#125;, you are a User`
``` ## Conditionals: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;#<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">if&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">...&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">/if&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code>
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
``` ## Pattern Matching: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;#<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">if<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">let<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">&#125;</code>
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">if<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">let</code> for pattern matching on <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">Option</code>, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">Result</code>, or other Rust enums:
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
``` ## Match Expressions: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;#match<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code>
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">match</code> for exhaustive pattern matching:
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
``` ## Iteration: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;#for<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code>
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
``` ## While Loops: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;#<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">while&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code>
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">while</code> for loops that need to continue until a condition is false:
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
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">while<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">let</code> for iterating with pattern matching, similar to <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">if<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">let</code>:
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
``` ## Local Constants: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;$let<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code>
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
 ## Mutable Variables: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;$let mut<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code>
 When you need to modify a variable within the template (e.g., in a `while` loop), use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;$let mut<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code>:
 ```
let code = ts_template! &#123;
    &#123;$let mut count = 0&#125;
    &#123;#for item in items&#125;
        console.log("Item @&#123;count&#125;: @&#123;item&#125;");
        &#123;$do count += 1&#125;
    &#123;/for&#125;
    console.log("Total: @&#123;count&#125;");
&#125;;
``` ## Side Effects: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;$do<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code>
 Execute an expression for its side effects without producing output. This is commonly used with mutable variables:
 ```
let code = ts_template! &#123;
    &#123;$let mut results: Vec&#x3C;String> = Vec::new()&#125;
    &#123;#for field in fields&#125;
        &#123;$do results.push(format!("this.&#123;&#125;", field))&#125;
    &#123;/for&#125;
    return [@&#123;results.join(", ")&#125;];
&#125;;
``` Common uses for <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;$do<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code>:
 - Incrementing counters: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;$do i<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">+=<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">1<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code>
 - Building collections: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;$do vec.<span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">push<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">(item)<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code>
 - Setting flags: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;$do found<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">=<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">true<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code>
 - Any mutating operation
 ## TsStream Injection: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;$typescript<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code>
 Inject another TsStream into your template, preserving both its source code and runtime patches (like imports added via <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">add_import<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">()</code>):
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
 If you need a literal <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code> in your output (not interpolation), use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@@<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code>:
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
 Here's a comparison showing how <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">ts_template!</code> simplifies code generation:
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
 4. **Result:** Returns <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">Stmt</code> that can be used in <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">MacroResult</code> patches
 ## Return Type
 <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">ts_template!</code> returns a <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">Result<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">lt;Stmt, TsSynError<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">gt;</code> by default. The macro automatically unwraps and provides helpful error messages showing the generated TypeScript code if parsing fails:
 ```
Failed to parse generated TypeScript:
User.prototype.toJSON = function( &#123;
    return &#123;&#125;;
&#125;
``` This shows you exactly what was generated, making debugging easy!
 ## Nesting and Regular TypeScript
 You can mix template syntax with regular TypeScript. Braces `{}` are recognized as either:
 - **Template tags** if they start with <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#</code>, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">$</code>, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">:</code>, or <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">/</code>
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
| ts_quote! | Compile-time validation, type-safe | Can't handle Vec<Stmt>, verbose |
| parse_ts_str() | Maximum flexibility | Runtime parsing, less readable |
| ts_template! | Readable, handles loops/conditions | Small runtime parsing overhead |
 ## Best Practices
 1. Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">ts_template!</code> for complex code generation with loops/conditions
 2. Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">ts_quote!</code> for simple, static statements
 3. Keep templates readable - extract complex logic into variables
 4. Don't nest templates too deeply - split into helper functions