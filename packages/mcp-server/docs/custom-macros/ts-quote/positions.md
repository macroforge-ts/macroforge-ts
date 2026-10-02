## Positions

A template's first word can say where a derive's code goes: `Top` or `Bottom` of the file, `Above`
or `Below` the target, or `Within` the class body. Without one, the code goes `Below`.

Rust

```
let members = ts_template!(Within {
    toString(): string { return "User"; }
});

let setup = ts_template!(Top {
    const registry = new Map<string, unknown>();
});
```

A template with a position marks its code with it, so the position holds when the stream is injected
into another template. See [Insert Positions](../../docs/custom-macros/output#insert-positions).
