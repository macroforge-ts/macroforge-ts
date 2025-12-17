# Template Syntax
 *The <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">macroforge_ts_quote</code> crate provides template-based code generation for TypeScript. The <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">ts_template!</code> macro uses Svelte + Rust-inspired syntax for control flow and interpolation, making it easy to generate complex TypeScript code.*
 ## Available Macros
 | Macro | Output | Use Case |
| --- | --- | --- |
| ts_template! | Any TypeScript code | General code generation |
| body! | Class body members | Methods and properties |
 ## Quick Reference
 | Syntax | Description |
| --- | --- |
| @{expr} | Interpolate a Rust expression (adds space after) |
| {| content |} | Ident block: concatenates without spaces (e.g., `{|get@{name}|}` → getUser) |
| {> "comment" <} | Block comment: outputs /* comment */ (string preserves whitespace) |
| {>> "doc" <<} | Doc comment: outputs /** doc */ (string preserves whitespace) |
| @@{ | Escape for literal @{ (e.g., "@@{foo}" → @{foo}) |
| "text @{expr}" | String interpolation (auto-detected) |
| "'^template ${js}^'" | JS backtick template literal (outputs ``template ${js}``) |
| {#if cond}...{/if} | Conditional block |
| `{#if cond}...{:else}...{/if}` | Conditional with else |
| {#if a}...{:else if b}...{:else}...{/if} | Full if/else-if/else chain |
| `{#if let pattern = expr}...{/if}` | Pattern matching if-let |
| {#match expr}{:case pattern}...{/match} | Match expression with case arms |
| `{#for item in list}...{/for}` | Iterate over a collection |
| {#while cond}...{/while} | While loop |
| `{#while let pattern = expr}...{/while}` | While-let pattern matching loop |
| {$let name = expr} | Define a local constant |
| {$let mut name = expr} | Define a mutable local variable |
| {$do expr} | Execute a side-effectful expression |
| {$typescript stream} | Inject a TsStream, preserving its source and runtime_patches (imports) |
 **Note:** A single <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@</code> not followed by <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{</code> passes through unchanged (e.g., <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">email@domain.com</code> works as expected).
 ## Interpolation: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@{expr}</code>
 Insert Rust expressions into the generated TypeScript:
 ```
let&nbsp;class_name&nbsp;=&nbsp;"User";
let&nbsp;method&nbsp;=&nbsp;"toString";

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;@&#123;class_name&#125;.prototype.@&#123;method&#125;&nbsp;=&nbsp;function()&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;"User&nbsp;instance";
&nbsp;&nbsp;&nbsp;&nbsp;&#125;;
&#125;;
``` **Generates:**
 ```
User.prototype.toString&nbsp;=&nbsp;function&nbsp;()&nbsp;&#123;
&nbsp;&nbsp;return&nbsp;"User&nbsp;instance";
&#125;;
``` ## Identifier Concatenation: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">|<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E"> content <span style="--shiki-dark:#F97583;--shiki-light:#D73A49">|<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code>
 When you need to build identifiers dynamically (like <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">getUser</code>, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">setName</code>), use the ident block syntax. Everything inside <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">|<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"> |<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code> is concatenated without spaces:
 ```
let&nbsp;field_name&nbsp;=&nbsp;"User";

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;function&nbsp;&#123;|get@&#123;field_name&#125;|&#125;()&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;this.@&#123;field_name.to_lowercase()&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;;
``` **Generates:**
 ```
function&nbsp;getUser()&nbsp;&#123;
&nbsp;&nbsp;return&nbsp;this.user;
&#125;
``` Without ident blocks, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@{}</code> always adds a space after for readability. Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">|<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"> |<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code> when you explicitly want concatenation:
 ```
let&nbsp;name&nbsp;=&nbsp;"Status";

//&nbsp;With&nbsp;space&nbsp;(default&nbsp;behavior)
ts_template!&nbsp;&#123;&nbsp;namespace&nbsp;@&#123;name&#125;&nbsp;&#125;&nbsp;&nbsp;//&nbsp;→&nbsp;"namespace&nbsp;Status"

//&nbsp;Without&nbsp;space&nbsp;(ident&nbsp;block)
ts_template!&nbsp;&#123;&nbsp;&#123;|namespace@&#123;name&#125;|&#125;&nbsp;&#125;&nbsp;&nbsp;//&nbsp;→&nbsp;"namespaceStatus"
``` Multiple interpolations can be combined:
 ```
let&nbsp;entity&nbsp;=&nbsp;"user";
let&nbsp;action&nbsp;=&nbsp;"create";

ts_template!&nbsp;&#123;&nbsp;&#123;|@&#123;entity&#125;_@&#123;action&#125;|&#125;&nbsp;&#125;&nbsp;&nbsp;//&nbsp;→&nbsp;"user_create"
``` ## Comments: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">><span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62"> "..."<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"> <<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code> and <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">>><span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62"> "..."<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"> <<<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code>
 Since Rust's tokenizer strips whitespace before macros see them, use string literals to preserve exact spacing in comments:
 ### Block Comments
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">><span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62"> "comment"<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"> <<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code> for block comments:
 ```
let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;>&nbsp;"This&nbsp;is&nbsp;a&nbsp;block&nbsp;comment"&nbsp;&#x3C;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;x&nbsp;=&nbsp;42;
&#125;;
``` **Generates:**
 ```
/*&nbsp;This&nbsp;is&nbsp;a&nbsp;block&nbsp;comment&nbsp;*/
const&nbsp;x&nbsp;=&nbsp;42;
``` ### Doc Comments (JSDoc)
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">>><span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62"> "doc"<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"> <<<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code> for JSDoc comments:
 ```
let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;>>&nbsp;"@param&nbsp;&#123;string&#125;&nbsp;name&nbsp;-&nbsp;The&nbsp;user's&nbsp;name"&nbsp;&#x3C;&#x3C;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;>>&nbsp;"@returns&nbsp;&#123;string&#125;&nbsp;A&nbsp;greeting&nbsp;message"&nbsp;&#x3C;&#x3C;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;function&nbsp;greet(name:&nbsp;string):&nbsp;string&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;"Hello,&nbsp;"&nbsp;+&nbsp;name;
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;;
``` **Generates:**
 ```
/**&nbsp;@param&nbsp;&#123;string&#125;&nbsp;name&nbsp;-&nbsp;The&nbsp;user's&nbsp;name&nbsp;*/
/**&nbsp;@returns&nbsp;&#123;string&#125;&nbsp;A&nbsp;greeting&nbsp;message&nbsp;*/
function&nbsp;greet(name:&nbsp;string):&nbsp;string&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;"Hello,&nbsp;"&nbsp;+&nbsp;name;
&#125;
``` ### Comments with Interpolation
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">format<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">!<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">()</code> or similar to build dynamic comment strings:
 ```
let&nbsp;param_name&nbsp;=&nbsp;"userId";
let&nbsp;param_type&nbsp;=&nbsp;"number";
let&nbsp;comment&nbsp;=&nbsp;format!("@param&nbsp;&#123;&#123;&#123;&#125;&#125;&#125;&nbsp;&#123;&#125;&nbsp;-&nbsp;The&nbsp;user&nbsp;ID",&nbsp;param_type,&nbsp;param_name);

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;>>&nbsp;@&#123;comment&#125;&nbsp;&#x3C;&#x3C;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;function&nbsp;getUser(userId:&nbsp;number)&nbsp;&#123;&#125;
&#125;;
``` **Generates:**
 ```
/**&nbsp;@param&nbsp;&#123;number&#125;&nbsp;userId&nbsp;-&nbsp;The&nbsp;user&nbsp;ID&nbsp;*/
function&nbsp;getUser(userId:&nbsp;number)&nbsp;&#123;&#125;
``` ## String Interpolation: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"text @{expr}"</code>
 Interpolation works automatically inside string literals - no `format!()` needed:
 ```
let&nbsp;name&nbsp;=&nbsp;"World";
let&nbsp;count&nbsp;=&nbsp;42;

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;console.log("Hello&nbsp;@&#123;name&#125;!");
&nbsp;&nbsp;&nbsp;&nbsp;console.log("Count:&nbsp;@&#123;count&#125;,&nbsp;doubled:&nbsp;@&#123;count&nbsp;*&nbsp;2&#125;");
&#125;;
``` **Generates:**
 ```
console.log("Hello&nbsp;World!");
console.log("Count:&nbsp;42,&nbsp;doubled:&nbsp;84");
``` This also works with method calls and complex expressions:
 ```
let&nbsp;field&nbsp;=&nbsp;"username";

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;throw&nbsp;new&nbsp;Error("Invalid&nbsp;@&#123;field.to_uppercase()&#125;");
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
``` You can mix Rust <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@{}</code> interpolation (evaluated at macro expansion time) with JS <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">${<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"${}"<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code> interpolation (evaluated at runtime):
 ```
let class_name = "User";

let code = ts_template! {
    "'^Hello ${this.name}, you are a @{class_name}^'"
};
``` **Generates:**
 ```
`Hello&nbsp;$&#123;this.name&#125;,&nbsp;you&nbsp;are&nbsp;a&nbsp;User`
``` ## Conditionals: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{#<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">if<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">...<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">/if<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code>
 Basic conditional:
 ```
let&nbsp;needs_validation&nbsp;=&nbsp;true;

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;function&nbsp;save()&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;#if&nbsp;needs_validation&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;if&nbsp;(!this.isValid())&nbsp;return&nbsp;false;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;/if&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;this.doSave();
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;;
``` ### If-Else
 ```
let&nbsp;has_default&nbsp;=&nbsp;true;

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#if&nbsp;has_default&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;defaultValue;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;:else&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;throw&nbsp;new&nbsp;Error("No&nbsp;default");
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/if&#125;
&#125;;
``` ### If-Else-If Chains
 ```
let&nbsp;level&nbsp;=&nbsp;2;

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#if&nbsp;level&nbsp;==&nbsp;1&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("Level&nbsp;1");
&nbsp;&nbsp;&nbsp;&nbsp;&#123;:else&nbsp;if&nbsp;level&nbsp;==&nbsp;2&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("Level&nbsp;2");
&nbsp;&nbsp;&nbsp;&nbsp;&#123;:else&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("Other&nbsp;level");
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/if&#125;
&#125;;
``` ## Pattern Matching: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{#<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">if<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"> let<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code>
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">if<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"> let</code> for pattern matching on <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">Option</code>, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">Result</code>, or other Rust enums:
 ```
let&nbsp;maybe_name:&nbsp;Option&#x3C;&#x26;str>&nbsp;=&nbsp;Some("Alice");

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#if&nbsp;let&nbsp;Some(name)&nbsp;=&nbsp;maybe_name&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("Hello,&nbsp;@&#123;name&#125;!");
&nbsp;&nbsp;&nbsp;&nbsp;&#123;:else&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("Hello,&nbsp;anonymous!");
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/if&#125;
&#125;;
``` **Generates:**
 ```
console.log("Hello,&nbsp;Alice!");
``` This is useful when working with optional values from your IR:
 ```
let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#if&nbsp;let&nbsp;Some(default_val)&nbsp;=&nbsp;field.default_value&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;this.@&#123;field.name&#125;&nbsp;=&nbsp;@&#123;default_val&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;:else&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;this.@&#123;field.name&#125;&nbsp;=&nbsp;undefined;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/if&#125;
&#125;;
``` ## Match Expressions: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{#match}</code>
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">match</code> for exhaustive pattern matching:
 ```
enum&nbsp;Visibility&nbsp;&#123;&nbsp;Public,&nbsp;Private,&nbsp;Protected&nbsp;&#125;
let&nbsp;visibility&nbsp;=&nbsp;Visibility::Public;

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#match&nbsp;visibility&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;:case&nbsp;Visibility::Public&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;public
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;:case&nbsp;Visibility::Private&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;private
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;:case&nbsp;Visibility::Protected&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;protected
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/match&#125;
&nbsp;&nbsp;&nbsp;&nbsp;field:&nbsp;string;
&#125;;
``` **Generates:**
 ```
public&nbsp;field:&nbsp;string;
``` ### Match with Value Extraction
 ```
let&nbsp;result:&nbsp;Result&#x3C;i32,&nbsp;&#x26;str>&nbsp;=&nbsp;Ok(42);

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;value&nbsp;=&nbsp;&#123;#match&nbsp;result&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;:case&nbsp;Ok(val)&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;@&#123;val&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;:case&nbsp;Err(msg)&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;throw&nbsp;new&nbsp;Error("@&#123;msg&#125;")
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/match&#125;;
&#125;;
``` ### Match with Wildcard
 ```
let&nbsp;count&nbsp;=&nbsp;5;

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#match&nbsp;count&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;:case&nbsp;0&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("none");
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;:case&nbsp;1&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("one");
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;:case&nbsp;_&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("many");
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/match&#125;
&#125;;
``` ## Iteration: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{#for}</code>
 ```
let&nbsp;fields&nbsp;=&nbsp;vec!["name",&nbsp;"email",&nbsp;"age"];

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;function&nbsp;toJSON()&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;result&nbsp;=&nbsp;&#123;&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;field&nbsp;in&nbsp;fields&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;result.@&#123;field&#125;&nbsp;=&nbsp;this.@&#123;field&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;result;
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;;
``` **Generates:**
 ```
function&nbsp;toJSON()&nbsp;&#123;
&nbsp;&nbsp;const&nbsp;result&nbsp;=&nbsp;&#123;&#125;;
&nbsp;&nbsp;result.name&nbsp;=&nbsp;this.name;
&nbsp;&nbsp;result.email&nbsp;=&nbsp;this.email;
&nbsp;&nbsp;result.age&nbsp;=&nbsp;this.age;
&nbsp;&nbsp;return&nbsp;result;
&#125;
``` ### Tuple Destructuring in Loops
 ```
let&nbsp;items&nbsp;=&nbsp;vec![("user",&nbsp;"User"),&nbsp;("post",&nbsp;"Post")];

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;(key,&nbsp;class_name)&nbsp;in&nbsp;items&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;@&#123;key&#125;&nbsp;=&nbsp;new&nbsp;@&#123;class_name&#125;();
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&#125;;
``` ### Nested Iterations
 ```
let&nbsp;classes&nbsp;=&nbsp;vec![
&nbsp;&nbsp;&nbsp;&nbsp;("User",&nbsp;vec!["name",&nbsp;"email"]),
&nbsp;&nbsp;&nbsp;&nbsp;("Post",&nbsp;vec!["title",&nbsp;"content"]),
];

ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;(class_name,&nbsp;fields)&nbsp;in&nbsp;classes&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;@&#123;class_name&#125;.prototype.toJSON&nbsp;=&nbsp;function()&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;field&nbsp;in&nbsp;fields&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;@&#123;field&#125;:&nbsp;this.@&#123;field&#125;,
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&#125;
``` ## While Loops: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{#<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">while<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code>
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">while</code> for loops that need to continue until a condition is false:
 ```
let&nbsp;items&nbsp;=&nbsp;get_items();
let&nbsp;mut&nbsp;idx&nbsp;=&nbsp;0;

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;$let&nbsp;mut&nbsp;i&nbsp;=&nbsp;0&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#while&nbsp;i&nbsp;&#x3C;&nbsp;items.len()&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("Item&nbsp;@&#123;i&#125;");
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;$do&nbsp;i&nbsp;+=&nbsp;1&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/while&#125;
&#125;;
``` ### While-Let Pattern Matching
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">while<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"> let</code> for iterating with pattern matching, similar to <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">if<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"> let</code>:
 ```
let&nbsp;mut&nbsp;items&nbsp;=&nbsp;vec!["a",&nbsp;"b",&nbsp;"c"].into_iter();

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#while&nbsp;let&nbsp;Some(item)&nbsp;=&nbsp;items.next()&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("@&#123;item&#125;");
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/while&#125;
&#125;;
``` **Generates:**
 ```
console.log("a");
console.log("b");
console.log("c");
``` This is especially useful when working with iterators or consuming optional values:
 ```
let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#while&nbsp;let&nbsp;Some(next_field)&nbsp;=&nbsp;remaining_fields.pop()&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;result.@&#123;next_field.name&#125;&nbsp;=&nbsp;this.@&#123;next_field.name&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/while&#125;
&#125;;
``` ## Local Constants: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{$let}</code>
 Define local variables within the template scope:
 ```
let&nbsp;items&nbsp;=&nbsp;vec![("user",&nbsp;"User"),&nbsp;("post",&nbsp;"Post")];

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;(key,&nbsp;class_name)&nbsp;in&nbsp;items&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;$let&nbsp;upper&nbsp;=&nbsp;class_name.to_uppercase()&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("Processing&nbsp;@&#123;upper&#125;");
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;@&#123;key&#125;&nbsp;=&nbsp;new&nbsp;@&#123;class_name&#125;();
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&#125;;
``` This is useful for computing derived values inside loops without cluttering the Rust code.
 ## Mutable Variables: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{$let mut}</code>
 When you need to modify a variable within the template (e.g., in a `while` loop), use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{$let mut}</code>:
 ```
let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;$let&nbsp;mut&nbsp;count&nbsp;=&nbsp;0&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;item&nbsp;in&nbsp;items&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("Item&nbsp;@&#123;count&#125;:&nbsp;@&#123;item&#125;");
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;$do&nbsp;count&nbsp;+=&nbsp;1&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&nbsp;&nbsp;&nbsp;&nbsp;console.log("Total:&nbsp;@&#123;count&#125;");
&#125;;
``` ## Side Effects: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{$do}</code>
 Execute an expression for its side effects without producing output. This is commonly used with mutable variables:
 ```
let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;$let&nbsp;mut&nbsp;results:&nbsp;Vec&#x3C;String>&nbsp;=&nbsp;Vec::new()&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;field&nbsp;in&nbsp;fields&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;$do&nbsp;results.push(format!("this.&#123;&#125;",&nbsp;field))&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;[@&#123;results.join(",&nbsp;")&#125;];
&#125;;
``` Common uses for <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{$do}</code>:
 - Incrementing counters: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{$do i <span style="--shiki-dark:#F97583;--shiki-light:#D73A49">+=<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5"> 1<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code>
 - Building collections: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{$do vec.<span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">push<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">(item)}</code>
 - Setting flags: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{$do found <span style="--shiki-dark:#F97583;--shiki-light:#D73A49">=<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5"> true<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code>
 - Any mutating operation
 ## TsStream Injection: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{$typescript}</code>
 Inject another TsStream into your template, preserving both its source code and runtime patches (like imports added via <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">add_import<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">()</code>):
 ```
//&nbsp;Create&nbsp;a&nbsp;helper&nbsp;method&nbsp;with&nbsp;its&nbsp;own&nbsp;import
let&nbsp;mut&nbsp;helper&nbsp;=&nbsp;body!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;validateEmail(email:&nbsp;string):&nbsp;boolean&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;Result.ok(true);
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;;
helper.add_import("Result",&nbsp;"macroforge/utils");

//&nbsp;Inject&nbsp;the&nbsp;helper&nbsp;into&nbsp;the&nbsp;main&nbsp;template
let&nbsp;result&nbsp;=&nbsp;body!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;$typescript&nbsp;helper&#125;

&nbsp;&nbsp;&nbsp;&nbsp;process(data:&nbsp;Record&#x3C;string,&nbsp;unknown>):&nbsp;void&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;...
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;;
//&nbsp;result&nbsp;now&nbsp;includes&nbsp;helper's&nbsp;source&nbsp;AND&nbsp;its&nbsp;Result&nbsp;import
``` This is essential for composing multiple macro outputs while preserving imports and patches:
 ```
let&nbsp;extra_methods&nbsp;=&nbsp;if&nbsp;include_validation&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;Some(body!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;validate():&nbsp;boolean&nbsp;&#123;&nbsp;return&nbsp;true;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#125;)
&#125;&nbsp;else&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;None
&#125;;

body!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;mainMethod():&nbsp;void&nbsp;&#123;&#125;

&nbsp;&nbsp;&nbsp;&nbsp;&#123;#if&nbsp;let&nbsp;Some(methods)&nbsp;=&nbsp;extra_methods&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;$typescript&nbsp;methods&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/if&#125;
&#125;
``` ## Escape Syntax
 If you need a literal <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@{</code> in your output (not interpolation), use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@@{</code>:
 ```
ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;This&nbsp;outputs&nbsp;a&nbsp;literal&nbsp;@&#123;foo&#125;
&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;example&nbsp;=&nbsp;"Use&nbsp;@@&#123;foo&#125;&nbsp;for&nbsp;templates";
&#125;
``` **Generates:**
 ```
//&nbsp;This&nbsp;outputs&nbsp;a&nbsp;literal&nbsp;@&#123;foo&#125;
const&nbsp;example&nbsp;=&nbsp;"Use&nbsp;@&#123;foo&#125;&nbsp;for&nbsp;templates";
``` ## Complete Example: JSON Derive Macro
 Here's a comparison showing how <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">ts_template!</code> simplifies code generation:
 ### Before (Manual AST Building)
 ```
pub&nbsp;fn&nbsp;derive_json_macro(input:&nbsp;TsStream)&nbsp;->&nbsp;MacroResult&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;input&nbsp;=&nbsp;parse_ts_macro_input!(input&nbsp;as&nbsp;DeriveInput);

&nbsp;&nbsp;&nbsp;&nbsp;match&nbsp;&#x26;input.data&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Data::Class(class)&nbsp;=>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;class_name&nbsp;=&nbsp;input.name();

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;mut&nbsp;body_stmts&nbsp;=&nbsp;vec![ts_quote!(&nbsp;const&nbsp;result&nbsp;=&nbsp;&#123;&#125;;&nbsp;as&nbsp;Stmt&nbsp;)];

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;for&nbsp;field_name&nbsp;in&nbsp;class.field_names()&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;body_stmts.push(ts_quote!(
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;result.$(ident!("&#123;&#125;",&nbsp;field_name))&nbsp;=&nbsp;this.$(ident!("&#123;&#125;",&nbsp;field_name));
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;as&nbsp;Stmt
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;));
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;body_stmts.push(ts_quote!(&nbsp;return&nbsp;result;&nbsp;as&nbsp;Stmt&nbsp;));

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;runtime_code&nbsp;=&nbsp;fn_assign!(
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;member_expr!(Expr::Ident(ident!(class_name)),&nbsp;"prototype"),
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"toJSON",
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;body_stmts
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;);

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;...
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;
``` ### After (With ts_template!)
 ```
pub&nbsp;fn&nbsp;derive_json_macro(input:&nbsp;TsStream)&nbsp;->&nbsp;MacroResult&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;input&nbsp;=&nbsp;parse_ts_macro_input!(input&nbsp;as&nbsp;DeriveInput);

&nbsp;&nbsp;&nbsp;&nbsp;match&nbsp;&#x26;input.data&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Data::Class(class)&nbsp;=>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;class_name&nbsp;=&nbsp;input.name();
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;fields&nbsp;=&nbsp;class.field_names();

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;runtime_code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;@&#123;class_name&#125;.prototype.toJSON&nbsp;=&nbsp;function()&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;result&nbsp;=&nbsp;&#123;&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;field&nbsp;in&nbsp;fields&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;result.@&#123;field&#125;&nbsp;=&nbsp;this.@&#123;field&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;result;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;;

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;...
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;
``` ## How It Works
 1. **Compile-Time:** The template is parsed during macro expansion
 2. **String Building:** Generates Rust code that builds a TypeScript string at runtime
 3. **SWC Parsing:** The generated string is parsed with SWC to produce a typed AST
 4. **Result:** Returns <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">Stmt</code> that can be used in <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">MacroResult</code> patches
 ## Return Type
 <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">ts_template!</code> returns a <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">Result<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"><<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">Stmt, TsSynError<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">></code> by default. The macro automatically unwraps and provides helpful error messages showing the generated TypeScript code if parsing fails:
 ```
Failed&nbsp;to&nbsp;parse&nbsp;generated&nbsp;TypeScript:
User.prototype.toJSON&nbsp;=&nbsp;function(&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;&#123;&#125;;
&#125;
``` This shows you exactly what was generated, making debugging easy!
 ## Nesting and Regular TypeScript
 You can mix template syntax with regular TypeScript. Braces `{}` are recognized as either:
 - **Template tags** if they start with <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#</code>, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">$</code>, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">:</code>, or <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">/</code>
 - **Regular TypeScript blocks** otherwise
 ```
ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;config&nbsp;=&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;#if&nbsp;use_strict&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;strict:&nbsp;true,
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;:else&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;strict:&nbsp;false,
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;/if&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;timeout:&nbsp;5000
&nbsp;&nbsp;&nbsp;&nbsp;&#125;;
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