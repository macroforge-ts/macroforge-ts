## Identifier Concatenation: `{| content |}`

When you need to build identifiers dynamically (like `getUser`, `setName`), use the ident block syntax. Everything inside `{| |}` is concatenated without spaces:

Rust

```
let field_name = "User";

let code = ts_template! {
    function {|get@{field_name}|}() {
        return this.@{field_name.to_lowercase()};
    }
};
```

**Generates:**

TypeScript

```
function getUser() {
  return this.user;
}
```

Without ident blocks, `@{}` always adds a space after for readability. Use `{| |}` when you explicitly want concatenation:

Rust

```
let name = "Status";

// With space (default behavior)
ts_template! { namespace @{name} }  // → "namespace Status"

// Without space (ident block)
ts_template! { {|namespace@{name}|} }  // → "namespaceStatus"
```

Multiple interpolations can be combined:

Rust

```
let entity = "user";
let action = "create";

ts_template! { {|@{entity}_@{action}|} }  // → "user_create"
```