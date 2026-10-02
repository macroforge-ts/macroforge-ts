## Match Expressions: `{#match}`

Use `match` for exhaustive pattern matching:

Rust

```
use macroforge_ts::ts_syn::Visibility;

let code = ts_template! {
    {#match field.visibility}
        {:case Visibility::Public}
            public
        {:case Visibility::Private}
            private
        {:case Visibility::Protected}
            protected
    {/match}
    @{field.name}: string;
};
```

### Match with Value Extraction

Rust

```
let result: Result<i32, &str> = Ok(42);

let code = ts_template! {
    const value = {#match result}
        {:case Ok(val)}
            @{val}
        {:case Err(msg)}
            throw new Error("@{msg}")
    {/match};
};
```

### Match with Wildcard

Rust

```
let count = 5;

let code = ts_template! {
    {#match count}
        {:case 0}
            console.log("none");
        {:case 1}
            console.log("one");
        {:case _}
            console.log("many");
    {/match}
};
```
