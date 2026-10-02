## Comments

Rust's tokenizer drops ordinary comments before a macro sees them, so a template marks the comments
it wants to emit. Write the comment as a string literal; `@{}` is interpolated in it:

Rust

```
let name = "User";

let code = ts_template! {
    {> "Generated for @{name}" <}
    {>> "Do not edit" <<}
    const version = 1;
};
```

**Generates:**

TypeScript

```
// Generated for User
/* Do not edit */
const version = 1;
```

A line comment whose text has several lines gets `//` on each, and a `*/` in a block comment's text
is broken up, so the comment never ends early.

### Doc Comments (JSDoc)

A Rust doc comment inside a template, `///` or `/** ... */`, becomes a JSDoc comment, with `@{}`
interpolated:

Rust

```
let field = "email";

let code = ts_template! {
    /// Returns the @{field} field.
    get@{field}(): string { return this.@{field}; }
};
```

**Generates:**

TypeScript

```
/** Returns the email field. */
getemail(): string { return this.email; }
```
