## While Loops: `&#123;#while&#125;`

Use `while` for loops that need to continue until a condition is false:

```rust
let items = get_items();
let mut idx = 0;

let code = ts_template! {
    {$let mut i = 0}
    {#while i < items.len()}
        console.log("Item @{i}");
        {$do i += 1}
    {/while}
};
```

### While-Let Pattern Matching

Use `while let` for iterating with pattern matching, similar to `if let`:

```rust
let mut items = vec!["a", "b", "c"].into_iter();

let code = ts_template! {
    {#while let Some(item) = items.next()}
        console.log("@{item}");
    {/while}
};
```

**Generates:**

```typescript
console.log("a");
console.log("b");
console.log("c");
```

This is especially useful when working with iterators or consuming optional values:

```rust
let code = ts_template! {
    {#while let Some(next_field) = remaining_fields.pop()}
        result.@{next_field.name} = this.@{next_field.name};
    {/while}
};
```