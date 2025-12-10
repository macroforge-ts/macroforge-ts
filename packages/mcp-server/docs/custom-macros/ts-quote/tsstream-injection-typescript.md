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