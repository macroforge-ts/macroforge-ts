## String Interpolation: `"text @{expr}"`

Interpolation works automatically inside string literals - no `format!()` needed:

Rust

```
let name = "World";
let count = 42;

let code = ts_template! {
    console.log("Hello @{name}!");
    console.log("Count: @{count}, doubled: @{count * 2}");
};
```

**Generates:**

TypeScript

```
console.log("Hello World!");
console.log("Count: 42, doubled: 84");
```

This also works with method calls and complex expressions:

Rust

```
let field = "username";

let code = ts_template! {
    throw new Error("Invalid @{field.to_uppercase()}");
};
```