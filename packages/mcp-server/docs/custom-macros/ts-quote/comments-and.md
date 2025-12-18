## Comments: `{> "..." <}` and `{>> "..." <<}`

Since Rust's tokenizer strips whitespace before macros see them, use string literals to preserve exact spacing in comments:

### Block Comments

Use `{> "comment" <}` for block comments:

Rust

```
let code = ts_template! {
    {> "This is a block comment" <}
    const x = 42;
};
```

**Generates:**

TypeScript

```
/* This is a block comment */
const x = 42;
```

### Doc Comments (JSDoc)

Use `{>> "doc" <<}` for JSDoc comments:

Rust

```
let code = ts_template! {
    {>> "@param {string} name - The user's name" <<}
    {>> "@returns {string} A greeting message" <<}
    function greet(name: string): string {
        return "Hello, " + name;
    }
};
```

**Generates:**

TypeScript

```
/** @param {string} name - The user's name */
/** @returns {string} A greeting message */
function greet(name: string): string {
    return "Hello, " + name;
}
```

### Comments with Interpolation

Use `format!()` or similar to build dynamic comment strings:

Rust

```
let param_name = "userId";
let param_type = "number";
let comment = format!("@param {{{}}} {} - The user ID", param_type, param_name);

let code = ts_template! {
    {>> @{comment} <<}
    function getUser(userId: number) {}
};
```

**Generates:**

TypeScript

```
/** @param {number} userId - The user ID */
function getUser(userId: number) {}
```