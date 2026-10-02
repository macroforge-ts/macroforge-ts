## Interpolation: `@{expr}`

Insert Rust expressions into the generated TypeScript:

Rust

```
let class_name = "User";
let method = "toString";

let code = ts_template! {
    @{class_name}.prototype.@{method} = function() {
        return "User instance";
    };
};
```

**Generates:**

TypeScript

```
User.prototype.toString = function() {
    return "User instance";
};
```

`@{expr}` accepts any value that implements `ToTsString`: strings, numbers, booleans and `char`, the
identifiers `ts_ident!` makes, a `TsStream`, and references or smart pointers to any of them.

### Spacing

The output keeps the spacing of the template as written. Tokens written next to each other stay
joined, and tokens with space between them stay apart, so building identifiers needs no special
syntax:

Rust

```
let name = "User";

let code = ts_template! {
    function get@{name}(): @{name} { ... }   // function getUser(): User { ... }
    const @{name.to_lowercase()}_id = 1;     // const user_id = 1;
};
```

Line breaks and indentation follow the template too, so the generated code is laid out the way the
template is.
