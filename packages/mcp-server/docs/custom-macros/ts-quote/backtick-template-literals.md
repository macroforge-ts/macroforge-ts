## Backtick Template Literals: `"'^...^'"`

For JavaScript template literals (backtick strings), use the `'^...^'` syntax. This outputs actual backticks and passes through `${"${}"}` for JS interpolation:

Rust

```
let tag_name = "div";

let code = ts_template! {
    const html = "'^<@{tag_name}>${content}</@{tag_name}>^'";
};
```

**Generates:**

TypeScript

```
const html = `<div>${content}</div>`;
```

You can mix Rust `@{}` interpolation (evaluated at macro expansion time) with JS `${"${}"}` interpolation (evaluated at runtime):

Rust

```
let class_name = "User";

let code = ts_template! {
    "'^Hello ${this.name}, you are a @{class_name}^'"
};
```

**Generates:**

TypeScript

```
`Hello ${this.name}, you are a User`
```