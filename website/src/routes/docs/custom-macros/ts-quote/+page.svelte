<script lang="ts">
    import CodeBlock from "$lib/components/ui/CodeBlock.svelte";
</script>

<svelte:head>
    <title>Template Syntax - Macroforge Documentation</title>
    <meta
        name="description"
        content="Learn the ts_template syntax for generating TypeScript code in macros."
    />
</svelte:head>

<h1>Template Syntax</h1>

<p class="lead">
    The <code>macroforge_ts_quote</code> crate provides template-based code
    generation for TypeScript. The <code>ts_template!</code> macro uses Svelte + Rust-inspired
    syntax for control flow and interpolation, making it easy to generate complex
    TypeScript code.
</p>

<h2 id="macros">Available Macros</h2>

<table>
    <thead>
        <tr>
            <th>Macro</th>
            <th>Output</th>
            <th>Use Case</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td><code>ts_template!</code></td>
            <td>Any TypeScript code</td>
            <td>General code generation</td>
        </tr>
        <tr>
            <td><code>body!</code></td>
            <td>Class body members</td>
            <td>Methods and properties</td>
        </tr>
    </tbody>
</table>

<h2 id="quick-reference">Quick Reference</h2>

<table>
    <thead>
        <tr>
            <th>Syntax</th>
            <th>Description</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td><code>@&#123;expr&#125;</code></td>
            <td>Interpolate a Rust expression (adds space after)</td>
        </tr>
        <tr>
            <td><code>&#123;| content |&#125;</code></td>
            <td
                >Ident block: concatenates without spaces (e.g., <code
                    >&#123;|get@&#123;name&#125;|&#125;</code
                >
                → <code>getUser</code>)</td
            >
        </tr>
        <tr>
            <td><code>&#123;&gt; "comment" &lt;&#125;</code></td>
            <td
                >Block comment: outputs <code>/* comment */</code> (string preserves
                whitespace)</td
            >
        </tr>
        <tr>
            <td><code>&#123;&gt;&gt; "doc" &lt;&lt;&#125;</code></td>
            <td
                >Doc comment: outputs <code>/** doc */</code> (string preserves whitespace)</td
            >
        </tr>
        <tr>
            <td><code>@@&#123;</code></td>
            <td
                >Escape for literal <code>@&#123;</code> (e.g.,
                <code>"@@&#123;foo&#125;"</code>
                → <code>@&#123;foo&#125;</code>)</td
            >
        </tr>
        <tr>
            <td><code>"text @&#123;expr&#125;"</code></td>
            <td>String interpolation (auto-detected)</td>
        </tr>
        <tr>
            <td><code>"'^template $&#123;js&#125;^'"</code></td>
            <td
                >JS backtick template literal (outputs <code
                    >`template $&#123;js&#125;`</code
                >)</td
            >
        </tr>
        <tr>
            <td><code>&#123;#if cond&#125;...&#123;/if&#125;</code></td>
            <td>Conditional block</td>
        </tr>
        <tr>
            <td
                ><code
                    >&#123;#if cond&#125;...&#123;:else&#125;...&#123;/if&#125;</code
                ></td
            >
            <td>Conditional with else</td>
        </tr>
        <tr>
            <td
                ><code
                    >&#123;#if a&#125;...&#123;:else if
                    b&#125;...&#123;:else&#125;...&#123;/if&#125;</code
                ></td
            >
            <td>Full if/else-if/else chain</td>
        </tr>
        <tr>
            <td
                ><code
                    >&#123;#if let pattern = expr&#125;...&#123;/if&#125;</code
                ></td
            >
            <td>Pattern matching if-let</td>
        </tr>
        <tr>
            <td
                ><code
                    >&#123;#match expr&#125;&#123;:case
                    pattern&#125;...&#123;/match&#125;</code
                ></td
            >
            <td>Match expression with case arms</td>
        </tr>
        <tr>
            <td
                ><code>&#123;#for item in list&#125;...&#123;/for&#125;</code
                ></td
            >
            <td>Iterate over a collection</td>
        </tr>
        <tr>
            <td><code>&#123;#while cond&#125;...&#123;/while&#125;</code></td>
            <td>While loop</td>
        </tr>
        <tr>
            <td
                ><code
                    >&#123;#while let pattern = expr&#125;...&#123;/while&#125;</code
                ></td
            >
            <td>While-let pattern matching loop</td>
        </tr>
        <tr>
            <td><code>&#123;$let name = expr&#125;</code></td>
            <td>Define a local constant</td>
        </tr>
        <tr>
            <td><code>&#123;$let mut name = expr&#125;</code></td>
            <td>Define a mutable local variable</td>
        </tr>
        <tr>
            <td><code>&#123;$do expr&#125;</code></td>
            <td>Execute a side-effectful expression</td>
        </tr>
        <tr>
            <td><code>&#123;$typescript stream&#125;</code></td>
            <td
                >Inject a TsStream, preserving its source and runtime_patches
                (imports)</td
            >
        </tr>
    </tbody>
</table>

<p>
    <strong>Note:</strong> A single <code>@</code> not followed by
    <code>&#123;</code>
    passes through unchanged (e.g., <code>email@domain.com</code> works as expected).
</p>

<h2 id="interpolation">Interpolation: <code>@&#123;expr&#125;</code></h2>

<p>Insert Rust expressions into the generated TypeScript:</p>

<CodeBlock
    code={`let class_name = "User";
let method = "toString";

let code = ts_template! {
    @{class_name}.prototype.@{method} = function() {
        return "User instance";
    };
};`}
    lang="rust"
/>

<p><strong>Generates:</strong></p>

<CodeBlock
    code={`User.prototype.toString = function () {
  return "User instance";
};`}
    lang="typescript"
/>

<h2 id="ident-blocks">
    Identifier Concatenation: <code>&#123;| content |&#125;</code>
</h2>

<p>
    When you need to build identifiers dynamically (like <code>getUser</code>,
    <code>setName</code>), use the ident block syntax. Everything inside
    <code>&#123;| |&#125;</code> is concatenated without spaces:
</p>

<CodeBlock
    code={`let field_name = "User";

let code = ts_template! {
    function {|get@{field_name}|}() {
        return this.@{field_name.to_lowercase()};
    }
};`}
    lang="rust"
/>

<p><strong>Generates:</strong></p>

<CodeBlock
    code={`function getUser() {
  return this.user;
}`}
    lang="typescript"
/>

<p>
    Without ident blocks, <code>@&#123;&#125;</code> always adds a space after
    for readability. Use <code>&#123;| |&#125;</code> when you explicitly want concatenation:
</p>

<CodeBlock
    code={`let name = "Status";

// With space (default behavior)
ts_template! { namespace @{name} }  // → "namespace Status"

// Without space (ident block)
ts_template! { {|namespace@{name}|} }  // → "namespaceStatus"`}
    lang="rust"
/>

<p>Multiple interpolations can be combined:</p>

<CodeBlock
    code={`let entity = "user";
let action = "create";

ts_template! { {|@{entity}_@{action}|} }  // → "user_create"`}
    lang="rust"
/>

<h2 id="comments">
    Comments: <code>&#123;&gt; "..." &lt;&#125;</code> and
    <code>&#123;&gt;&gt; "..." &lt;&lt;&#125;</code>
</h2>

<p>
    Since Rust's tokenizer strips whitespace before macros see them, use string
    literals to preserve exact spacing in comments:
</p>

<h3>Block Comments</h3>

<p>Use <code>&#123;&gt; "comment" &lt;&#125;</code> for block comments:</p>

<CodeBlock
    code={`let code = ts_template! {
    {> "This is a block comment" <}
    const x = 42;
};`}
    lang="rust"
/>

<p><strong>Generates:</strong></p>

<CodeBlock
    code={`/* This is a block comment */
const x = 42;`}
    lang="typescript"
/>

<h3>Doc Comments (JSDoc)</h3>

<p>Use <code>&#123;&gt;&gt; "doc" &lt;&lt;&#125;</code> for JSDoc comments:</p>

<CodeBlock
    code={`let code = ts_template! {
    {>> "@param {string} name - The user's name" <<}
    {>> "@returns {string} A greeting message" <<}
    function greet(name: string): string {
        return "Hello, " + name;
    }
};`}
    lang="rust"
/>

<p><strong>Generates:</strong></p>

<CodeBlock
    code={`/** @param {string} name - The user's name */
/** @returns {string} A greeting message */
function greet(name: string): string {
    return "Hello, " + name;
}`}
    lang="typescript"
/>

<h3>Comments with Interpolation</h3>

<p>Use <code>format!()</code> or similar to build dynamic comment strings:</p>

<CodeBlock
    code={`let param_name = "userId";
let param_type = "number";
let comment = format!("@param {{{}}} {} - The user ID", param_type, param_name);

let code = ts_template! {
    {>> @{comment} <<}
    function getUser(userId: number) {}
};`}
    lang="rust"
/>

<p><strong>Generates:</strong></p>

<CodeBlock
    code={`/** @param {number} userId - The user ID */
function getUser(userId: number) {}`}
    lang="typescript"
/>

<h2 id="string-interpolation">
    String Interpolation: <code>"text @&#123;expr&#125;"</code>
</h2>

<p>
    Interpolation works automatically inside string literals - no <code
        >format!()</code
    > needed:
</p>

<CodeBlock
    code={`let name = "World";
let count = 42;

let code = ts_template! {
    console.log("Hello @{name}!");
    console.log("Count: @{count}, doubled: @{count * 2}");
};`}
    lang="rust"
/>

<p><strong>Generates:</strong></p>

<CodeBlock
    code={`console.log("Hello World!");
console.log("Count: 42, doubled: 84");`}
    lang="typescript"
/>

<p>This also works with method calls and complex expressions:</p>

<CodeBlock
    code={`let field = "username";

let code = ts_template! {
    throw new Error("Invalid @{field.to_uppercase()}");
};`}
    lang="rust"
/>

<h2 id="backtick-templates">
    Backtick Template Literals: <code>"'^...^'"</code>
</h2>

<p>
    For JavaScript template literals (backtick strings), use the <code
        >'^...^'</code
    >
    syntax. This outputs actual backticks and passes through
    <code>${"${}"}</code> for JS interpolation:
</p>

<CodeBlock
    code={`let tag_name = "div";

let code = ts_template! {
    const html = "'^<@{tag_name}>\${content}</@{tag_name}>^'";
};`}
    lang="rust"
/>

<p><strong>Generates:</strong></p>

<CodeBlock code={"const html = `<div>${content}</div>`;"} lang="typescript" />

<p>
    You can mix Rust <code>@&#123;&#125;</code> interpolation (evaluated at
    macro expansion time) with JS <code>${"${}"}</code> interpolation (evaluated at
    runtime):
</p>

<CodeBlock
    code={`let class_name = "User";

let code = ts_template! {
    "'^Hello \${this.name}, you are a @{class_name}^'"
};`}
    lang="rust"
/>

<p><strong>Generates:</strong></p>

<CodeBlock code={"`Hello ${this.name}, you are a User`"} lang="typescript" />

<h2 id="conditionals">
    Conditionals: <code>&#123;#if&#125;...&#123;/if&#125;</code>
</h2>

<p>Basic conditional:</p>

<CodeBlock
    code={`let needs_validation = true;

let code = ts_template! {
    function save() {
        {#if needs_validation}
            if (!this.isValid()) return false;
        {/if}
        return this.doSave();
    }
};`}
    lang="rust"
/>

<h3>If-Else</h3>

<CodeBlock
    code={`let has_default = true;

let code = ts_template! {
    {#if has_default}
        return defaultValue;
    {:else}
        throw new Error("No default");
    {/if}
};`}
    lang="rust"
/>

<h3>If-Else-If Chains</h3>

<CodeBlock
    code={`let level = 2;

let code = ts_template! {
    {#if level == 1}
        console.log("Level 1");
    {:else if level == 2}
        console.log("Level 2");
    {:else}
        console.log("Other level");
    {/if}
};`}
    lang="rust"
/>

<h2 id="pattern-matching">
    Pattern Matching: <code>&#123;#if let&#125;</code>
</h2>

<p>
    Use <code>if let</code> for pattern matching on <code>Option</code>,
    <code>Result</code>, or other Rust enums:
</p>

<CodeBlock
    code={`let maybe_name: Option<&str> = Some("Alice");

let code = ts_template! {
    {#if let Some(name) = maybe_name}
        console.log("Hello, @{name}!");
    {:else}
        console.log("Hello, anonymous!");
    {/if}
};`}
    lang="rust"
/>

<p><strong>Generates:</strong></p>

<CodeBlock code={`console.log("Hello, Alice!");`} lang="typescript" />

<p>This is useful when working with optional values from your IR:</p>

<CodeBlock
    code={`let code = ts_template! {
    {#if let Some(default_val) = field.default_value}
        this.@{field.name} = @{default_val};
    {:else}
        this.@{field.name} = undefined;
    {/if}
};`}
    lang="rust"
/>

<h2 id="match-expressions">
    Match Expressions: <code>&#123;#match&#125;</code>
</h2>

<p>Use <code>match</code> for exhaustive pattern matching:</p>

<CodeBlock
    code={`enum Visibility { Public, Private, Protected }
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
};`}
    lang="rust"
/>

<p><strong>Generates:</strong></p>

<CodeBlock code={`public field: string;`} lang="typescript" />

<h3>Match with Value Extraction</h3>

<CodeBlock
    code={`let result: Result<i32, &str> = Ok(42);

let code = ts_template! {
    const value = {#match result}
        {:case Ok(val)}
            @{val}
        {:case Err(msg)}
            throw new Error("@{msg}")
    {/match};
};`}
    lang="rust"
/>

<h3>Match with Wildcard</h3>

<CodeBlock
    code={`let count = 5;

let code = ts_template! {
    {#match count}
        {:case 0}
            console.log("none");
        {:case 1}
            console.log("one");
        {:case _}
            console.log("many");
    {/match}
};`}
    lang="rust"
/>

<h2 id="loops">Iteration: <code>&#123;#for&#125;</code></h2>

<CodeBlock
    code={`let fields = vec!["name", "email", "age"];

let code = ts_template! {
    function toJSON() {
        const result = {};
        {#for field in fields}
            result.@{field} = this.@{field};
        {/for}
        return result;
    }
};`}
    lang="rust"
/>

<p><strong>Generates:</strong></p>

<CodeBlock
    code={`function toJSON() {
  const result = {};
  result.name = this.name;
  result.email = this.email;
  result.age = this.age;
  return result;
}`}
    lang="typescript"
/>

<h3>Tuple Destructuring in Loops</h3>

<CodeBlock
    code={`let items = vec![("user", "User"), ("post", "Post")];

let code = ts_template! {
    {#for (key, class_name) in items}
        const @{key} = new @{class_name}();
    {/for}
};`}
    lang="rust"
/>

<h3>Nested Iterations</h3>

<CodeBlock
    code={`let classes = vec![
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
}`}
    lang="rust"
/>

<h2 id="while-loops">While Loops: <code>&#123;#while&#125;</code></h2>

<p>
    Use <code>while</code> for loops that need to continue until a condition is false:
</p>

<CodeBlock
    code={`let items = get_items();
let mut idx = 0;

let code = ts_template! {
    {$let mut i = 0}
    {#while i < items.len()}
        console.log("Item @{i}");
        {$do i += 1}
    {/while}
};`}
    lang="rust"
/>

<h3>While-Let Pattern Matching</h3>

<p>
    Use <code>while let</code> for iterating with pattern matching, similar to
    <code>if let</code>:
</p>

<CodeBlock
    code={`let mut items = vec!["a", "b", "c"].into_iter();

let code = ts_template! {
    {#while let Some(item) = items.next()}
        console.log("@{item}");
    {/while}
};`}
    lang="rust"
/>

<p><strong>Generates:</strong></p>

<CodeBlock
    code={`console.log("a");
console.log("b");
console.log("c");`}
    lang="typescript"
/>

<p>
    This is especially useful when working with iterators or consuming optional
    values:
</p>

<CodeBlock
    code={`let code = ts_template! {
    {#while let Some(next_field) = remaining_fields.pop()}
        result.@{next_field.name} = this.@{next_field.name};
    {/while}
};`}
    lang="rust"
/>

<h2 id="local-variables">Local Constants: <code>&#123;$let&#125;</code></h2>

<p>Define local variables within the template scope:</p>

<CodeBlock
    code={`let items = vec![("user", "User"), ("post", "Post")];

let code = ts_template! {
    {#for (key, class_name) in items}
        {$let upper = class_name.to_uppercase()}
        console.log("Processing @{upper}");
        const @{key} = new @{class_name}();
    {/for}
};`}
    lang="rust"
/>

<p>
    This is useful for computing derived values inside loops without cluttering
    the Rust code.
</p>

<h2 id="mutable-variables">
    Mutable Variables: <code>&#123;$let mut&#125;</code>
</h2>

<p>
    When you need to modify a variable within the template (e.g., in a <code
        >while</code
    >
    loop), use <code>&#123;$let mut&#125;</code>:
</p>

<CodeBlock
    code={`let code = ts_template! {
    {$let mut count = 0}
    {#for item in items}
        console.log("Item @{count}: @{item}");
        {$do count += 1}
    {/for}
    console.log("Total: @{count}");
};`}
    lang="rust"
/>

<h2 id="side-effects">Side Effects: <code>&#123;$do&#125;</code></h2>

<p>
    Execute an expression for its side effects without producing output. This is
    commonly used with mutable variables:
</p>

<CodeBlock
    code={`let code = ts_template! {
    {$let mut results: Vec<String> = Vec::new()}
    {#for field in fields}
        {$do results.push(format!("this.{}", field))}
    {/for}
    return [@{results.join(", ")}];
};`}
    lang="rust"
/>

<p>Common uses for <code>&#123;$do&#125;</code>:</p>

<ul>
    <li>Incrementing counters: <code>&#123;$do i += 1&#125;</code></li>
    <li>Building collections: <code>&#123;$do vec.push(item)&#125;</code></li>
    <li>Setting flags: <code>&#123;$do found = true&#125;</code></li>
    <li>Any mutating operation</li>
</ul>

<h2 id="typescript-injection">
    TsStream Injection: <code>&#123;$typescript&#125;</code>
</h2>

<p>
    Inject another TsStream into your template, preserving both its source code
    and runtime patches (like imports added via <code>add_import()</code>):
</p>

<CodeBlock
    code={`// Create a helper method with its own import
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
// result now includes helper's source AND its Result import`}
    lang="rust"
/>

<p>
    This is essential for composing multiple macro outputs while preserving
    imports and patches:
</p>

<CodeBlock
    code={`let extra_methods = if include_validation {
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
}`}
    lang="rust"
/>

<h2 id="escape-syntax">Escape Syntax</h2>

<p>
    If you need a literal <code>@&#123;</code> in your output (not
    interpolation), use <code>@@&#123;</code>:
</p>

<CodeBlock
    code={`ts_template! {
    // This outputs a literal @{foo}
    const example = "Use @@{foo} for templates";
}`}
    lang="rust"
/>

<p><strong>Generates:</strong></p>

<CodeBlock
    code={`// This outputs a literal @{foo}
const example = "Use @{foo} for templates";`}
    lang="typescript"
/>

<h2 id="complete-example">Complete Example: JSON Derive Macro</h2>

<p>
    Here's a comparison showing how <code>ts_template!</code> simplifies code generation:
</p>

<h3>Before (Manual AST Building)</h3>

<CodeBlock
    code={`pub fn derive_json_macro(input: TsStream) -> MacroResult {
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
}`}
    lang="rust"
/>

<h3>After (With ts_template!)</h3>

<CodeBlock
    code={`pub fn derive_json_macro(input: TsStream) -> MacroResult {
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
}`}
    lang="rust"
/>

<h2 id="how-it-works">How It Works</h2>

<ol>
    <li>
        <strong>Compile-Time:</strong> The template is parsed during macro expansion
    </li>
    <li>
        <strong>String Building:</strong> Generates Rust code that builds a TypeScript
        string at runtime
    </li>
    <li>
        <strong>SWC Parsing:</strong> The generated string is parsed with SWC to produce
        a typed AST
    </li>
    <li>
        <strong>Result:</strong> Returns <code>Stmt</code> that can be used in
        <code>MacroResult</code> patches
    </li>
</ol>

<h2 id="return-type">Return Type</h2>

<p>
    <code>ts_template!</code> returns a
    <code>Result&lt;Stmt, TsSynError&gt;</code> by default. The macro automatically
    unwraps and provides helpful error messages showing the generated TypeScript code
    if parsing fails:
</p>

<CodeBlock
    code={`Failed to parse generated TypeScript:
User.prototype.toJSON = function( {
    return {};
}`}
    lang="text"
/>

<p>This shows you exactly what was generated, making debugging easy!</p>

<h2 id="nesting">Nesting and Regular TypeScript</h2>

<p>
    You can mix template syntax with regular TypeScript. Braces <code
        >&#123;&#125;</code
    > are recognized as either:
</p>

<ul>
    <li>
        <strong>Template tags</strong> if they start with <code>#</code>,
        <code>$</code>, <code>:</code>, or <code>/</code>
    </li>
    <li><strong>Regular TypeScript blocks</strong> otherwise</li>
</ul>

<CodeBlock
    code={`ts_template! {
    const config = {
        {#if use_strict}
            strict: true,
        {:else}
            strict: false,
        {/if}
        timeout: 5000
    };
}`}
    lang="rust"
/>

<h2 id="comparison">Comparison with Alternatives</h2>

<table>
    <thead>
        <tr>
            <th>Approach</th>
            <th>Pros</th>
            <th>Cons</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td><code>ts_quote!</code></td>
            <td>Compile-time validation, type-safe</td>
            <td>Can't handle Vec&lt;Stmt&gt;, verbose</td>
        </tr>
        <tr>
            <td><code>parse_ts_str()</code></td>
            <td>Maximum flexibility</td>
            <td>Runtime parsing, less readable</td>
        </tr>
        <tr>
            <td><code>ts_template!</code></td>
            <td>Readable, handles loops/conditions</td>
            <td>Small runtime parsing overhead</td>
        </tr>
    </tbody>
</table>

<h2 id="best-practices">Best Practices</h2>

<ol>
    <li>
        Use <code>ts_template!</code> for complex code generation with loops/conditions
    </li>
    <li>Use <code>ts_quote!</code> for simple, static statements</li>
    <li>Keep templates readable - extract complex logic into variables</li>
    <li>Don't nest templates too deeply - split into helper functions</li>
</ol>
