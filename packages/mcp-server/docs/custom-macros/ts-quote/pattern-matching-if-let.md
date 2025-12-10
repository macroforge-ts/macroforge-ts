## Pattern Matching: `&#123;#if let&#125;`

Use `if let` for pattern matching on `Option`, `Result`, or other Rust enums:

```rust
let maybe_name: Option<&str> = Some("Alice");

let code = ts_template! {
    {#if let Some(name) = maybe_name}
        console.log("Hello, @{name}!");
    {:else}
        console.log("Hello, anonymous!");
    {/if}
};
```

**Generates:**

```typescript
console.log("Hello, Alice!");
```

This is useful when working with optional values from your IR:

```rust
let code = ts_template! {
    {#if let Some(default_val) = field.default_value}
        this.@{field.name} = @{default_val};
    {:else}
        this.@{field.name} = undefined;
    {/if}
};
```