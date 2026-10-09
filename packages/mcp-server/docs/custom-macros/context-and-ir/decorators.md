## Decorators

Rust

```
pub struct DecoratorIR {
    pub name: String,      // e.g. "endec"
    pub args_src: String,  // the arguments as written, e.g. "skip, rename: \\"id\\""
    pub span: SpanIR,
}
```

A macro reads its options from `args_src`. The helpers in `macroforge_ts::builtin::derive::common`
parse the common shapes:

Rust

```
use macroforge_ts::builtin::derive::common::{extract_named_string, has_flag};

for decorator in &field.decorators {
    if decorator.name.eq_ignore_ascii_case("validate") {
        let required = has_flag(&decorator.args_src, "required");
        // rename: "id", rename = "id" or rename("id"); escapes are decoded
        let rename = extract_named_string(&decorator.args_src, "rename");
    }
}
```

## Spans

Every IR node carries a `SpanIR`. Spans count positions, which are byte offsets plus one; see
[Spans](../../docs/custom-macros/output#spans) for reading the text a span covers.

## Next Steps

- [Type-Aware Macros](../../docs/custom-macros/type-aware)
- [Output and Imports](../../docs/custom-macros/output)
