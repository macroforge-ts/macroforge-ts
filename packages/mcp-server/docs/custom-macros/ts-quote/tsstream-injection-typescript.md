## TsStream Injection: `{$typescript}`

Inject another TsStream into your template, preserving both its source code and runtime patches
(like imports added via `add_import()`):

Rust

```
// Create a helper method with its own import
let mut helper = ts_template!(Within {
    validateEmail(email: string): boolean {
        return Result.ok(true);
    }
});
helper.add_import("Result", "@macroforge/core/utils");

// Inject the helper into the main template
let result = ts_template!(Within {
    {$typescript helper}

    process(data: Record<string, unknown>): void {
        // ...
    }
});
// result now includes helper's source AND its Result import
```

This is essential for composing multiple macro outputs while preserving imports and patches:

Rust

```
let extra_methods = if include_validation {
    Some(ts_template!(Within {
        validate(): boolean { return true; }
    }))
} else {
    None
};

ts_template!(Within {
    mainMethod(): void {}

    {#if let Some(methods) = extra_methods}
        {$typescript methods}
    {/if}
})
```

## Escape Syntax

If you need a literal `@{` in your output (not interpolation), use `@@{`:

Rust

```
ts_template! {
    // This outputs a literal @{foo}
    const example = "Use @@{foo} for templates";
}
```

**Generates:**

TypeScript

```
// This outputs a literal @{foo}
const example = "Use @{foo} for templates";
```
