## Insert Positions

A derive's code goes in one of five places, named by `InsertPos`:

TypeScript

```
// ─── Top ────────────────────────
import { foo } from "./runtime";

// ─── Above ──────────────────────
/** @derive(Debug) */
class User {
    // ─── Within ─────────────────
    name: string;
}
// ─── Below (the default) ────────
export function userToString(value: User): string { ... }

// ─── Bottom ─────────────────────
```

`ts_template!` takes the position as its first word, as in `ts_template!(Within { ... })`; without
one the code goes `Below`. A stream built by hand takes it from `TsStream::with_insert_pos`.

Rust

```
use macroforge_ts::macros::ts_template;
use macroforge_ts::ts_syn::{InsertPos, TsStream};

// Members of the class body
let members = ts_template!(Within {
    toString(): string { return "User"; }
});

// A function after the class
let standalone = ts_template! {
    export function describeUser(): string { return "a user"; }
};

// The same, built from a string
let by_hand = TsStream::with_insert_pos(
    "export const userVersion = 1;".to_string(),
    InsertPos::Below,
);
```

### Combining Streams

`merge` appends one stream to another and keeps the first stream's position; `TsStream::merge_all`
folds a list. Everything else on the streams (imports, patches, suffixes, warnings) is combined too.
Code for two different positions goes in two streams, injected into one template with
`{$typescript}`:

Rust

```
Ok(ts_template! {
    {$typescript standalone}
    {$typescript members}
})
```

A template with an explicit position starts its code with a marker comment, such as
`/* @macroforge:body */` for `Within`, and the host splits a combined stream back up at the markers.
Code before the first marker takes the combined stream's own position, so put a stream without an
explicit position first, as above.

### Other Stream Methods

| Method                          | Use                                                                             |
| ------------------------------- | ------------------------------------------------------------------------------- |
| `TsStream::from_string(source)` | A stream of hand-written code, positioned `Below`                               |
| `source()`                      | The generated code so far                                                       |
| `take_source()`                 | Takes the code out, leaving the rest of the stream to merge elsewhere           |
| `context()`                     | The [macro context](../../docs/custom-macros/context-and-ir) the host passed in |
| `parse_stmt(&allocator)`        | Parses the first statement of the stream with oxc, to inspect generated code    |
