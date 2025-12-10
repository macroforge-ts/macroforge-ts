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