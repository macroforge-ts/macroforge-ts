## Local Constants: `&#123;$let&#125;`

Define local variables within the template scope:

```rust
let items = vec![("user", "User"), ("post", "Post")];

let code = ts_template! {
    {#for (key, class_name) in items}
        {$let upper = class_name.to_uppercase()}
        console.log("Processing @{upper}");
        const @{key} = new @{class_name}();
    {/for}
};
```

This is useful for computing derived values inside loops without cluttering the Rust code.

<h2 id="mutable-variables">
    Mutable Variables: `&#123;$let mut&#125;`
</h2>

When you need to modify a variable within the template (e.g., in a <code >while</code > loop), use `&#123;$let mut&#125;`:

```rust
let code = ts_template! {
    {$let mut count = 0}
    {#for item in items}
        console.log("Item @{count}: @{item}");
        {$do count += 1}
    {/for}
    console.log("Total: @{count}");
};
```