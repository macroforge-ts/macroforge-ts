## Comments: `&#123;> ... <&#125;` and `&#123;>> ... <<&#125;`

Since Rust's tokenizer strips comments before macros see them, you can't write JSDoc comments directly. Instead, use the comment syntax to output JavaScript comments:

### Block Comments

Use `&#123;> comment <&#125;` for block comments:

```rust
let code = ts_template! {
    {> This is a block comment <}
    const x = 42;
};
```

**Generates:**

```typescript
/* This is a block comment */
const x = 42;
```

### Doc Comments (JSDoc)

Use `&#123;>> doc <<&#125;` for JSDoc comments:

```rust
let code = ts_template! {
    {>> @param {string} name - The user's name <<}
    {>> @returns {string} A greeting message <<}
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

### Comments with Interpolation

Comments support `@&#123;expr&#125;` interpolation for dynamic content:

```rust
let param_name = "userId";
let param_type = "number";

let code = ts_template! {
    {>> @param {@{param_type}} @{param_name} - The user ID <<}
    function getUser(userId: number) {}
};
```

**Generates:**

```typescript
/** @param {number} userId - The user ID */
function getUser(userId: number) {}
```