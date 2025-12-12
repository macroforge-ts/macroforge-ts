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

<h2 id="typescript-injection">
    TsStream Injection: `&#123;$typescript&#125;`
</h2>

Inject another TsStream into your template, preserving both its source code and runtime patches (like imports added via `add_import()`):

```rust
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