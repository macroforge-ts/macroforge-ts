## Side Effects: `{$do}`

Execute an expression for its side effects without producing output. This is commonly used with mutable variables:

Rust

```
let code = ts_template! {
    {$let mut results: Vec<String> = Vec::new()}
    {#for field in fields}
        {$do results.push(format!("this.{}", field))}
    {/for}
    return [@{results.join(", ")}];
};
```

Common uses for `{$do}`:

*   Incrementing counters: `{$do i += 1}`
*   Building collections: `{$do vec.push(item)}`
*   Setting flags: `{$do found = true}`
*   Any mutating operation