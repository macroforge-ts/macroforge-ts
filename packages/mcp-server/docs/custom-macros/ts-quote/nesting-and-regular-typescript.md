## Nesting and Regular TypeScript

You can mix template syntax with regular TypeScript. Braces `{}` are recognized as either:

- **Template tags** if they start with `#`, `:`, `/`, `$`, `%` or `>`
- **Regular TypeScript blocks** otherwise

Rust

```
ts_template! {
    const config = {
        {#if use_strict}
            strict: true,
        {:else}
            strict: false,
        {/if}
        timeout: 5000
    };
}
```
