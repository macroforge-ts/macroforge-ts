## Identifier Concatenation: `&#123;| content |&#125;`

When you need to build identifiers dynamically (like `getUser`, `setName`), use the ident block syntax. Everything inside `&#123;| |&#125;` is concatenated without spaces:

```rust
let field_name = "User";

let code = ts_template! {
    function {|get@{field_name}|}() {
        return this.@{field_name.to_lowercase()};
    }
};
```

**Generates:**

```typescript
function getUser() {
  return this.user;
}
```

Without ident blocks, `@&#123;&#125;` always adds a space after for readability. Use `&#123;| |&#125;` when you explicitly want concatenation:

```rust
let name = "Status";

// With space (default behavior)
ts_template! { namespace @{name} }  // → "namespace Status"

// Without space (ident block)
ts_template! { {|namespace@{name}|} }  // → "namespaceStatus"
```

Multiple interpolations can be combined:

```rust
let entity = "user";
let action = "create";

ts_template! { {|@{entity}_@{action}|} }  // → "user_create"
```