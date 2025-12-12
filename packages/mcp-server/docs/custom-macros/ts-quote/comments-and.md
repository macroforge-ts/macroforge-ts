## Comments: `&#123;> "..." <&#125;` and `&#123;>> "..." <<&#125;`

Since Rust's tokenizer strips whitespace before macros see them, use string literals to preserve exact spacing in comments:

### Block Comments

Use `&#123;> "comment" <&#125;` for block comments:

```rust
let code = ts_template! {
    {> "This is a block comment" <}
    const x = 42;
};
```

**Generates:**

```typescript
/* This is a block comment */
const x = 42;
```

### Doc Comments (JSDoc)

Use `&#123;>> "doc" <<&#125;` for JSDoc comments:

```rust
let code = ts_template! {
    {>> "@param {string} name - The user's name" <<}
    {>> "@returns {string} A greeting message" <<}
    function greet(name: string): string {
        return "Hello, " + name;
    }
};
```

**Generates:**

```typescript
/** @param {string} name - The user's name */
/** @returns {string} A greeting message */
function greet(name: string): string {
    return "Hello, " + name;
}
```