<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
</script>

<svelte:head>
	<title>Template Syntax - Macroforge Documentation</title>
	<meta name="description" content="Learn the ts_quote template syntax for generating TypeScript code in macros." />
</svelte:head>

<h1>Template Syntax (ts_quote)</h1>

<p class="lead">
	The <code>ts_quote</code> crate provides template-based code generation for TypeScript. It's similar to Rust's <code>quote!</code> macro but outputs TypeScript.
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

<h2 id="interpolation">Interpolation</h2>

<p>Use <code>@{'{'}expr{'}'}</code> to interpolate Rust expressions:</p>

<CodeBlock code={`let class_name = "User";
let field_name = "name";

body! {
    {|get@{field_name}|}(): string {
        return this.@{field_name};
    }
}

// Generates:
// getname(): string {
//     return this.name;
// }`} lang="rust" />

<h2 id="ident-blocks">Identifier Concatenation</h2>

<p>Use <code>{'{'} | content | {'}'}</code> to concatenate identifiers without spaces. This is essential for building dynamic identifiers like <code>getUser</code>, <code>setName</code>, etc.</p>

<CodeBlock code={`let type_name = "User";

body! {
    // With ident block - concatenates without spaces
    function {|get@{type_name}|}() {
        return this.user;
    }
}

// Generates:
// function getUser() { return this.user; }`} lang="rust" />

<p>By default, <code>@{'{'}expr{'}'}</code> adds a space after for readability. Use ident blocks when you explicitly need concatenation:</p>

<CodeBlock code={`let name = "Status";

// Regular interpolation (space after)
ts_template! { namespace @{name} }
// → "namespace Status"

// Ident block (no space)
ts_template! { {|namespace@{name}|} }
// → "namespaceStatus"`} lang="rust" />

<h2 id="loops">Loops</h2>

<p>Iterate with <code>{'{'}#for{'}'}</code>:</p>

<CodeBlock code={`let fields = vec!["name", "age", "email"];

body! {
    {#for field in fields}
        {|get@{field}|}() {
            return this.@{field};
        }
    {/for}
}

// Generates getters for each field`} lang="rust" />

<h3>Loop with Tuples</h3>

<CodeBlock code={`let fields = vec![
    ("name", "string"),
    ("age", "number"),
];

body! {
    {#for (name, type_name) in fields}
        {|get@{name}|}(): @{type_name} {
            return this.@{name};
        }
    {/for}
}`} lang="rust" />

<h2 id="conditionals">Conditionals</h2>

<p>Use <code>{'{'}#if{'}'}</code> for conditional code:</p>

<CodeBlock code={`let include_setter = true;
let field_name = "value";

body! {
    getValue(): number {
        return this.@{field_name};
    }

    {#if include_setter}
        setValue(v: number): void {
            this.@{field_name} = v;
        }
    {/if}
}`} lang="rust" />

<h3>If-Else</h3>

<CodeBlock code={`let is_nullable = true;

body! {
    getValue(): @{if is_nullable { "number | null" } else { "number" }} {
        return this.value;
    }
}`} lang="rust" />

<h2 id="local-variables">Local Variables</h2>

<p>Use <code>{'{'}%let{'}'}</code> to define local variables within templates:</p>

<CodeBlock code={`body! {
    {#for field in fields}
        {%let capitalized = capitalize(field.name)}
        {%let return_type = field.ts_type.clone()}

        {|get@{capitalized}|}(): @{return_type} {
            return this.@{field.name};
        }
    {/for}
}`} lang="rust" />

<h2 id="typescript-injection">TsStream Injection</h2>

<p>Use <code>{'{'}%typescript stream{'}'}</code> to inject another TsStream into your template, preserving both its source code and runtime patches (like imports):</p>

<CodeBlock code={`// Create a helper with its own import
let mut helper = body! {
    validateEmail(email: string): boolean {
        return Result.ok(true);
    }
};
helper.add_import("Result", "macroforge/result");

// Inject into the main template - imports are preserved!
let result = body! {
    {%typescript helper}

    process(data: Record<string, unknown>): void {
        // ...
    }
};`} lang="rust" />

<p>This is essential for composing macro outputs while keeping imports intact:</p>

<CodeBlock code={`let extra_methods = if include_validation {
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
}`} lang="rust" />

<h2 id="string-literals">String Literals</h2>

<p>String content is output literally, with interpolation inside:</p>

<CodeBlock code={`let class_name = "User";

body! {
    toString(): string {
        return "@{class_name} { " + this.toJSON() + " }";
    }
}`} lang="rust" />

<h2 id="complete-example">Complete Example</h2>

<CodeBlock code={`use macroforge_ts::ts_quote::body;

fn generate_json_macro(class: &ClassData) -> TsStream {
    body! {
        toJSON(): Record<string, unknown> {
            const result: Record<string, unknown> = {};

            {#for field in class.field_names()}
                result.@{field} = this.@{field};
            {/for}

            return result;
        }

        static fromJSON(data: Record<string, unknown>): @{class.name} {
            return new @{class.name}(
                {#for (i, field) in class.fields().iter().enumerate()}
                    data.@{field.name} as @{field.ts_type}
                    @{if i < class.fields().len() - 1 { "," } else { "" }}
                {/for}
            );
        }
    }
}`} lang="rust" />

<h2 id="syntax-reference">Syntax Reference</h2>

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
			<td>Interpolate Rust expression (adds space after)</td>
		</tr>
		<tr>
			<td><code>&#123;| content |&#125;</code></td>
			<td>Ident block: concatenates without spaces</td>
		</tr>
		<tr>
			<td><code>&#123;#for x in iter&#125;...&#123;/for&#125;</code></td>
			<td>Loop over iterable</td>
		</tr>
		<tr>
			<td><code>&#123;#if cond&#125;...&#123;/if&#125;</code></td>
			<td>Conditional block</td>
		</tr>
		<tr>
			<td><code>&#123;%let name = expr&#125;</code></td>
			<td>Local variable binding</td>
		</tr>
		<tr>
			<td><code>&#123;%typescript stream&#125;</code></td>
			<td>Inject TsStream, preserving source and patches (imports)</td>
		</tr>
	</tbody>
</table>
