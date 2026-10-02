<script lang="ts">
    import CodeBlock from "$lib/components/ui/CodeBlock.svelte";
    import Alert from "$lib/components/ui/Alert.svelte";
    import { resolve } from "$app/paths";
</script>

<svelte:head>
    <title>Template Syntax - Macroforge Documentation</title>
    <meta
        name="description"
        content="Learn the ts_template syntax for generating TypeScript code in macros, and ts_quote for building AST nodes."
    />
</svelte:head>

<h1>Template Syntax</h1>

<p class="lead">
    The <code>macroforge_ts_quote</code> crate provides two macros for generating TypeScript.
    <code>ts_template!</code> writes TypeScript as text, with Svelte-like control flow and
    Rust interpolation; it is what most macros use. <code>ts_quote!</code> builds a single
    AST node and checks its syntax when your crate compiles.
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
            <td>A <code>TsStream</code> of TypeScript source</td>
            <td>Generating code: methods, functions, declarations</td>
        </tr>
        <tr>
            <td><code>ts_template!(Within &lbrace; … &rbrace;)</code></td>
            <td>The same, placed in the class body</td>
            <td>Methods and properties; see <a href="#positions">Positions</a></td>
        </tr>
        <tr>
            <td><code>ts_quote!</code></td>
            <td>One oxc AST node</td>
            <td>Building or inspecting syntax trees; see <a href="#ts-quote">ts_quote!</a></td>
        </tr>
        <tr>
            <td><code>ts_ident!</code></td>
            <td>An identifier</td>
            <td>Names built from strings; see <a href="#ts-ident">ts_ident!</a></td>
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
            <td>Interpolate a Rust expression</td>
        </tr>
        <tr>
            <td><code>"text @&#123;expr&#125;"</code></td>
            <td>String interpolation (auto-detected)</td>
        </tr>
        <tr>
            <td><code>"'^template $&#123;js&#125;^'"</code></td>
            <td>JS backtick template literal (outputs <code>`template $&#123;js&#125;`</code>)</td>
        </tr>
        <tr>
            <td><code>@@&#123;</code></td>
            <td>Escape for literal <code>@&#123;</code> (e.g., <code>"@@&#123;foo&#125;"</code> → <code>@&#123;foo&#125;</code>)</td>
        </tr>
        <tr>
            <td><code>&#123;&gt; "comment" &lt;&#125;</code></td>
            <td>Line comment: outputs <code>// comment</code></td>
        </tr>
        <tr>
            <td><code>&#123;&gt;&gt; "comment" &lt;&lt;&#125;</code></td>
            <td>Block comment: outputs <code>/* comment */</code></td>
        </tr>
        <tr>
            <td><code>/// text</code> or <code>/** text */</code></td>
            <td>JSDoc comment: outputs <code>/** text */</code></td>
        </tr>
        <tr>
            <td><code>&#123;#if cond&#125;...&#123;/if&#125;</code></td>
            <td>Conditional block</td>
        </tr>
        <tr>
            <td><code>&#123;#if cond&#125;...&#123;:else&#125;...&#123;/if&#125;</code></td>
            <td>Conditional with else</td>
        </tr>
        <tr>
            <td><code>&#123;#if a&#125;...&#123;:else if b&#125;...&#123;:else&#125;...&#123;/if&#125;</code></td>
            <td>Full if/else-if/else chain</td>
        </tr>
        <tr>
            <td><code>&#123;#if let pattern = expr&#125;...&#123;/if&#125;</code></td>
            <td>Pattern matching if-let</td>
        </tr>
        <tr>
            <td><code>&#123;#match expr&#125;&#123;:case pattern&#125;...&#123;/match&#125;</code></td>
            <td>Match expression with case arms</td>
        </tr>
        <tr>
            <td><code>&#123;#for item in list&#125;...&#123;/for&#125;</code></td>
            <td>Iterate over a collection</td>
        </tr>
        <tr>
            <td><code>&#123;#while cond&#125;...&#123;/while&#125;</code></td>
            <td>While loop</td>
        </tr>
        <tr>
            <td><code>&#123;#while let pattern = expr&#125;...&#123;/while&#125;</code></td>
            <td>While-let pattern matching loop</td>
        </tr>
        <tr>
            <td><code>&#123;$let name = expr&#125;</code></td>
            <td>Define a local constant (<code>&#123;%let&#125;</code> is the same)</td>
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
            <td>Inject a <code>TsStream</code>, with its patches, imports and warnings</td>
        </tr>
    </tbody>
</table>

<p>
    <strong>Note:</strong> A single <code>@</code> not followed by <code>&#123;</code> passes
    through unchanged (e.g., <code>email@domain.com</code> works as expected).
</p>

<h2 id="positions">Positions</h2>

<p>
    A template's first word can say where a derive's code goes: <code>Top</code> or
    <code>Bottom</code> of the file, <code>Above</code> or <code>Below</code> the target, or
    <code>Within</code> the class body. Without one, the code goes <code>Below</code>.
</p>

<CodeBlock
    code={`let members = ts_template!(Within {
    toString(): string { return "User"; }
});

let setup = ts_template!(Top {
    const registry = new Map<string, unknown>();
});`}
    lang="rust"
/>

<p>
    A template with a position marks its code with it, so the position holds when the stream is
    injected into another template. See
    <a href={resolve('/docs/custom-macros/output#insert-positions')}>Insert Positions</a>.
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
    code={`User.prototype.toString = function() {
    return "User instance";
};`}
    lang="typescript"
/>

<p>
    <code>@&#123;expr&#125;</code> accepts any value that implements <code>ToTsString</code>:
    strings, numbers, booleans and <code>char</code>, the identifiers <code>ts_ident!</code>
    makes, a <code>TsStream</code>, and references or smart pointers to any of them.
</p>

<h3 id="spacing">Spacing</h3>

<p>
    The output keeps the spacing of the template as written. Tokens written next to each other
    stay joined, and tokens with space between them stay apart, so building identifiers needs no
    special syntax:
</p>

<CodeBlock
    code={`let name = "User";

let code = ts_template! {
    function get@{name}(): @{name} { ... }   // function getUser(): User { ... }
    const @{name.to_lowercase()}_id = 1;     // const user_id = 1;
};`}
    lang="rust"
/>

<p>
    Line breaks and indentation follow the template too, so the generated code is laid out the
    way the template is.
</p>

<h2 id="string-interpolation">
    String Interpolation: <code>"text @&#123;expr&#125;"</code>
</h2>

<p>
    Interpolation works automatically inside string literals - no <code>format!()</code> needed:
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

<p>
    Text inside <code>@&#123;...&#125;</code> that is not a Rust expression is a compile error.
</p>

<h2 id="backtick-templates">
    Backtick Template Literals: <code>"'^...^'"</code>
</h2>

<p>
    For JavaScript template literals (backtick strings), use the <code>'^...^'</code> syntax.
    This outputs actual backticks and passes through <code>${"${}"}</code> for JS interpolation:
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
    You can mix Rust <code>@&#123;&#125;</code> interpolation (evaluated at macro expansion
    time) with JS <code>${"${}"}</code> interpolation (evaluated at runtime):
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

<h2 id="comments">Comments</h2>

<p>
    Rust's tokenizer drops ordinary comments before a macro sees them, so a template marks the
    comments it wants to emit. Write the comment as a string literal; <code>@&#123;&#125;</code>
    is interpolated in it:
</p>

<CodeBlock
    code={`let name = "User";

let code = ts_template! {
    {> "Generated for @{name}" <}
    {>> "Do not edit" <<}
    const version = 1;
};`}
    lang="rust"
/>

<p><strong>Generates:</strong></p>

<CodeBlock
    code={`// Generated for User
/* Do not edit */
const version = 1;`}
    lang="typescript"
/>

<p>
    A line comment whose text has several lines gets <code>//</code> on each, and a
    <code>*/</code> in a block comment's text is broken up, so the comment never ends early.
</p>

<h3 id="doc-comments">Doc Comments (JSDoc)</h3>

<p>
    A Rust doc comment inside a template, <code>///</code> or <code>/** ... */</code>, becomes a
    JSDoc comment, with <code>@&#123;&#125;</code> interpolated:
</p>

<CodeBlock
    code={`let field = "email";

let code = ts_template! {
    /// Returns the @{field} field.
    get@{field}(): string { return this.@{field}; }
};`}
    lang="rust"
/>

<p><strong>Generates:</strong></p>

<CodeBlock
    code={`/** Returns the email field. */
getemail(): string { return this.email; }`}
    lang="typescript"
/>

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
    Use <code>if let</code> for pattern matching on <code>Option</code>, <code>Result</code>, or
    other Rust enums:
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
    {#for variant in enum_.variants()}
        {#if let Some(text) = variant.value.as_string()}
            case "@{text}": return "@{variant.name}";
        {/if}
    {/for}
};`}
    lang="rust"
/>

<h2 id="match-expressions">
    Match Expressions: <code>&#123;#match&#125;</code>
</h2>

<p>Use <code>match</code> for exhaustive pattern matching:</p>

<CodeBlock
    code={`use macroforge_ts::ts_syn::Visibility;

let code = ts_template! {
    {#match field.visibility}
        {:case Visibility::Public}
            public
        {:case Visibility::Private}
            private
        {:case Visibility::Protected}
            protected
    {/match}
    @{field.name}: string;
};`}
    lang="rust"
/>

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

<p>Use <code>while</code> for loops that need to continue until a condition is false:</p>

<CodeBlock
    code={`let items = vec!["a", "b", "c"];

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
    Use <code>while let</code> for iterating with pattern matching, similar to <code>if let</code>:
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
    This is useful for computing derived values inside loops without cluttering the Rust code.
</p>

<h2 id="mutable-variables">
    Mutable Variables: <code>&#123;$let mut&#125;</code>
</h2>

<p>
    When you need to modify a variable within the template (e.g., in a <code>while</code> loop),
    use <code>&#123;$let mut&#125;</code>:
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
    Execute an expression for its side effects without producing output. This is commonly used
    with mutable variables:
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
    Inject another <code>TsStream</code> into your template. Its source joins the output, and
    everything else it carries comes along: patches, cross-module suffixes and warnings.
    Imports requested with <code>add_import()</code> apply to the whole expansion whichever
    stream asked for them.
</p>

<CodeBlock
    code={`// Create a helper method with its own import
let mut helper = ts_template!(Within {
    validateEmail(email: string): boolean {
        return isEmail(email);
    }
});
helper.add_import("isEmail", "my-validation-lib");

// Inject the helper into the main template
let result = ts_template!(Within {
    {$typescript helper}

    process(data: Record<string, unknown>): void {
        // ...
    }
});`}
    lang="rust"
/>

<p>
    The injected value must be a <code>TsStream</code> you own; injection moves it. This is how
    optional parts of a macro's output are composed:
</p>

<CodeBlock
    code={`let extra_methods = if include_validation {
    Some(ts_template!(Within {
        validate(): boolean { return true; }
    }))
} else {
    None
};

ts_template!(Within {
    mainMethod(): void {}

    {#if let Some(methods) = extra_methods}
        {$typescript methods}
    {/if}
})`}
    lang="rust"
/>

<h2 id="escape-syntax">Escape Syntax</h2>

<p>
    If you need a literal <code>@&#123;</code> in your output (not interpolation), use
    <code>@@&#123;</code>:
</p>

<CodeBlock
    code={`ts_template! {
    const example = "Use @@{foo} for templates";
}`}
    lang="rust"
/>

<p><strong>Generates:</strong></p>

<CodeBlock code={`const example = "Use @{foo} for templates";`} lang="typescript" />

<h2 id="nesting">Nesting and Regular TypeScript</h2>

<p>
    You can mix template syntax with regular TypeScript. Braces <code>&#123;&#125;</code> are
    recognized as either:
</p>

<ul>
    <li>
        <strong>Template tags</strong> if they start with <code>#</code>, <code>:</code>,
        <code>/</code>, <code>$</code>, <code>%</code> or <code>&gt;</code>
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

<h2 id="ts-ident">Identifiers: <code>ts_ident!</code></h2>

<p>
    <code>ts_ident!</code> makes an identifier from a string or a format string. It is handy for
    names a macro passes around before interpolating them:
</p>

<CodeBlock
    code={`use macroforge_ts::ts_syn::ts_ident;

let type_name = input.name();
let serialize_fn = ts_ident!("{}Serialize", type_name.to_lowercase()); // userSerialize

let code = ts_template! {
    export function @{serialize_fn}(value: @{type_name}): string { ... }
};`}
    lang="rust"
/>

<Alert type="note">
    <span>
        <code>DeriveInput</code>'s <code>ident</code> field is a different identifier type, one
        that records where the name is. To write a type's name, use <code>input.name()</code>.
    </span>
</Alert>

<h2 id="ts-quote">AST Nodes: <code>ts_quote!</code></h2>

<p>
    <code>ts_quote!</code> parses a TypeScript snippet when your crate compiles, so a syntax
    error is a compile error, and builds it as an oxc AST node at run time. <code>$name</code>
    placeholders are filled from the variables after the snippet:
</p>

<CodeBlock
    code={`use macroforge_ts::macros::ts_quote;
use macroforge_ts::ts_syn::oxc::allocator::Allocator;
use macroforge_ts::ts_syn::{expr_to_string, parse_expr};

let arena = Allocator::default();
let rhs = parse_expr(&arena, "1 + 2").expect("a valid expression");

// $name takes an identifier (the default); $rhs an expression
let assignment = ts_quote!("$name = $rhs" as Expr, name = "count", rhs: Expr = rhs);
assert_eq!(expr_to_string(&assignment), "count = 1 + 2");`}
    lang="rust"
/>

<p>
    The node lives in an oxc arena: the <code>arena</code> variable in scope, or one passed first,
    as in <code>ts_quote!(&amp;other_arena, "a + b" as Expr)</code>.
</p>

<table>
    <thead>
        <tr>
            <th><code>as</code></th>
            <th>Builds</th>
        </tr>
    </thead>
    <tbody>
        <tr><td><code>Expr</code></td><td>An expression</td></tr>
        <tr><td><code>Stmt</code></td><td>A statement</td></tr>
        <tr><td><code>ModuleItem</code></td><td>A top-level item, such as a declaration or import</td></tr>
        <tr><td><code>Program</code></td><td>A whole program</td></tr>
        <tr><td><code>Pat</code></td><td>A binding pattern</td></tr>
        <tr><td><code>AssignTarget</code></td><td>The left side of an assignment</td></tr>
        <tr><td><code>TsType</code></td><td>A type</td></tr>
        <tr><td><code>PropOrSpread</code></td><td>An object property or spread</td></tr>
    </tbody>
</table>

<p>
    A variable's type says what its placeholder holds: <code>Ident</code> (the default, from a
    string), <code>Expr</code>, <code>Pat</code>, <code>Str</code> (a string literal's text),
    <code>AssignTarget</code> or <code>TsType</code>.
</p>

<p>
    <code>parse_expr</code>, <code>parse_statement</code>, <code>parse_module_item</code>,
    <code>parse_program</code>, <code>parse_type</code>, <code>parse_binding_pattern</code>,
    <code>parse_assignment_target</code> and <code>parse_prop_or_spread</code> parse text into
    the same nodes, and <code>expr_to_string</code>, <code>stmt_to_string</code>,
    <code>type_to_string</code>, <code>binding_pattern_to_string</code>,
    <code>assignment_target_to_string</code> and <code>string_literal_to_string</code> print them
    back, for example to interpolate into a <code>ts_template!</code>.
</p>

<h2 id="complete-example">Complete Example: JSON Derive Macro</h2>

<p>
    Here's a comparison showing how <code>ts_template!</code> simplifies code generation:
</p>

<h3>Before (Manual String Building)</h3>

<CodeBlock
    code={`#[ts_macro_derive(JSON)]
pub fn derive_json_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);
    let Some(class) = input.as_class() else {
        return Err(MacroforgeError::new(input.error_span(), "@derive(JSON) needs a class"));
    };

    let mut body = String::from("const result = {};\\n");
    for field_name in class.field_names() {
        body.push_str(&format!("result.{field_name} = this.{field_name};\\n"));
    }
    body.push_str("return result;");

    Ok(TsStream::from_string(format!(
        "{}.prototype.toJSON = function() {{\\n{body}\\n}};",
        input.name()
    )))
}`}
    lang="rust"
/>

<h3>After (With ts_template!)</h3>

<CodeBlock
    code={`#[ts_macro_derive(JSON)]
pub fn derive_json_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);
    let Some(class) = input.as_class() else {
        return Err(MacroforgeError::new(input.error_span(), "@derive(JSON) needs a class"));
    };
    let class_name = input.name();

    Ok(ts_template! {
        @{class_name}.prototype.toJSON = function() {
            const result = {};
            {#for field in class.field_names()}
                result.@{field} = this.@{field};
            {/for}
            return result;
        };
    })
}`}
    lang="rust"
/>

<h2 id="how-it-works">How It Works</h2>

<ol>
    <li>
        <strong>Rust compile time:</strong> the template is turned into Rust code that writes
        TypeScript text, with the control flow as ordinary Rust <code>if</code>,
        <code>for</code> and <code>match</code>.
    </li>
    <li>
        <strong>Macro run time:</strong> that code runs and builds the text, interpolating your
        values.
    </li>
    <li>
        <strong>Result:</strong> a <code>TsStream</code> that can be returned directly as macro
        output. The text is not checked here, so a syntax error in it shows up when the expanded
        file is compiled; <a href={resolve('/docs/custom-macros/testing-and-debugging#expand-a-file')}><code>macroforge expand</code></a>
        shows exactly what was generated.
    </li>
</ol>

<h2 id="choosing">Choosing a Macro</h2>

<ul>
    <li>Use <code>ts_template!</code> to generate code, especially with loops and conditions.</li>
    <li>
        Use <code>ts_quote!</code> when you need an AST node: to analyse or transform syntax, or
        to have a fixed snippet checked when your crate compiles.
    </li>
    <li>Keep templates readable: compute values in Rust before the template.</li>
    <li>
        Split large outputs into several templates and combine them with
        <code>&#123;$typescript&#125;</code>.
    </li>
</ul>
