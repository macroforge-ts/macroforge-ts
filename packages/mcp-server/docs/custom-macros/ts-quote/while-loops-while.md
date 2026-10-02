## While Loops: `{#while}`

Use `while` for loops that need to continue until a condition is false:

Rust

```
let items = vec!["a", "b", "c"];

let code = ts_template! {
    {$let mut i = 0}
    {#while i < items.len()}
        console.log("Item @{i}");
        {$do i += 1}
    {/while}
};
```

### While-Let Pattern Matching

Use `while let` for iterating with pattern matching, similar to `if let`:

Rust

```
let mut items = vec!["a", "b", "c"].into_iter();

let code = ts_template! {
    {#while let Some(item) = items.next()}
        console.log("@{item}");
    {/while}
};
```

**Generates:**

TypeScript

```
console.log("a");
console.log("b");
console.log("c");
```
