## Backtick Template Literals: `"'^...^'"`

For JavaScript template literals (backtick strings), use the `'^...^'` syntax. This outputs actual backticks and passes through `${"${}"}` for JS interpolation:

```rust
let tag_name = "div";

let code = ts_template! {
    const html = "'^<@{tag_name}>\${content}</@{tag_name}>^'";
};
```

**Generates:**

<CodeBlock code={'const html = `${content}`;'} lang="typescript" />

You can mix Rust `@&#123;&#125;` interpolation (evaluated at macro expansion time) with JS `${"${}"}` interpolation (evaluated at runtime):

```rust
let class_name = "User";

let code = ts_template! {
    "'^Hello \${this.name}, you are a @{class_name}^'"
};
```

**Generates:**

<CodeBlock code={'`Hello ${this.name}, you are a User`'} lang="typescript" />