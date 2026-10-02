## Identifiers: `ts_ident!`

`ts_ident!` makes an identifier from a string or a format string. It is handy for names a macro
passes around before interpolating them:

Rust

```
use macroforge_ts::ts_syn::ts_ident;

let type_name = input.name();
let serialize_fn = ts_ident!("{}Serialize", type_name.to_lowercase()); // userSerialize

let code = ts_template! {
    export function @{serialize_fn}(value: @{type_name}): string { ... }
};
```

Note

`DeriveInput`'s `ident` field is a different identifier type, one that records where the name is. To
write a type's name, use `input.name()`.
