## TsStream Injection: `{$typescript}`

Inject another `TsStream` into your template. Its source joins the output, and everything else it
carries comes along: patches, cross-module suffixes and warnings. Imports requested with
`add_import()` apply to the whole expansion whichever stream asked for them.

Rust

```
// Create a helper method with its own import
let mut helper = ts_template!(Within {
    validateEmail(email: string): boolean {
        return isEmail(email);
    }
});
helper.add_import("isEmail", "my-validation-lib");

// Inject the helper into the main template
let result = ts_template!(Within {
    {$typescript helper}

    process(data: Record<string, unknown>): void {
        // ...
    }
});
```

The injected value must be a `TsStream` you own; injection moves it. This is how optional parts of a
macro's output are composed:

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
    const example = "Use @@{foo} for templates";
}
```

**Generates:**

TypeScript

```
const example = "Use @{foo} for templates";
```
