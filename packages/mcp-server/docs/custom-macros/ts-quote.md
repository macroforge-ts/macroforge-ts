# Template Syntax (ts_quote)

*The `ts_quote` crate provides template-based code generation for TypeScript. It's similar to Rust's `quote!` macro but outputs TypeScript.*

## Available Macros

| `ts_template!` 
| Any TypeScript code 
| General code generation 

| `body!` 
| Class body members 
| Methods and properties

## Interpolation

Use `@{'{'}expr{'}'}` to interpolate Rust expressions:

```rust
let class_name = "User";
let field_name = "name";

body! {
    {|get@{field_name}|}(): string {
        return this.@{field_name};
    }
}

// Generates:
// getname(): string {
//     return this.name;
// }
```

## Identifier Concatenation

Use `{'{'} | content | {'}'}` to concatenate identifiers without spaces. This is essential for building dynamic identifiers like `getUser`, `setName`, etc.

```rust
let type_name = "User";

body! {
    // With ident block - concatenates without spaces
    function {|get@{type_name}|}() {
        return this.user;
    }
}

// Generates:
// function getUser() { return this.user; }
```

By default, `@{'{'}expr{'}'}` adds a space after for readability. Use ident blocks when you explicitly need concatenation:

```rust
let name = "Status";

// Regular interpolation (space after)
ts_template! { namespace @{name} }
// → "namespace Status"

// Ident block (no space)
ts_template! { {|namespace@{name}|} }
// → "namespaceStatus"
```

## Loops

Iterate with `{'{'}#for{'}'}`:

```rust
let fields = vec!["name", "age", "email"];

body! {
    {#for field in fields}
        {|get@{field}|}() {
            return this.@{field};
        }
    {/for}
}

// Generates getters for each field
```

### Loop with Tuples

```rust
let fields = vec![
    ("name", "string"),
    ("age", "number"),
];

body! {
    {#for (name, type_name) in fields}
        {|get@{name}|}(): @{type_name} {
            return this.@{name};
        }
    {/for}
}
```

## Conditionals

Use `{'{'}#if{'}'}` for conditional code:

```rust
let include_setter = true;
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
}
```

### If-Else

```rust
let is_nullable = true;

body! {
    getValue(): @{if is_nullable { "number | null" } else { "number" }} {
        return this.value;
    }
}
```

## Local Variables

Use `{'{'}%let{'}'}` to define local variables within templates:

```rust
body! {
    {#for field in fields}
        {%let capitalized = capitalize(field.name)}
        {%let return_type = field.ts_type.clone()}

        {|get@{capitalized}|}(): @{return_type} {
            return this.@{field.name};
        }
    {/for}
}
```

## TsStream Injection

Use `{'{'}%typescript stream{'}'}` to inject another TsStream into your template, preserving both its source code and runtime patches (like imports):

```rust
// Create a helper with its own import
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
};
```

This is essential for composing macro outputs while keeping imports intact:

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

## String Literals

String content is output literally, with interpolation inside:

```rust
let class_name = "User";

body! {
    toString(): string {
        return "@{class_name} { " + this.toJSON() + " }";
    }
}
```

## Complete Example

```rust
use macroforge_ts::ts_quote::body;

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
}
```

## Syntax Reference

| `@&#123;expr&#125;` 
| Interpolate Rust expression (adds space after) 

| `&#123;| content |&#125;` 
| Ident block: concatenates without spaces 

| `&#123;#for x in iter&#125;...&#123;/for&#125;` 
| Loop over iterable 

| `&#123;#if cond&#125;...&#123;/if&#125;` 
| Conditional block 

| `&#123;%let name = expr&#125;` 
| Local variable binding 

| `&#123;%typescript stream&#125;` 
| Inject TsStream, preserving source and patches (imports)