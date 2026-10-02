## Patches

A patch edits the user's source directly: inserts code at a position, or replaces or deletes a span.
Attribute macros work this way, and any macro can add patches to its stream's `runtime_patches`:

Rust

```
use macroforge_ts::ts_syn::{Patch, SpanIR};

// Replace the target with new code
output.runtime_patches.push(Patch::Replace {
    span: ctx.target_span,
    code: rewritten,
    source_macro: Some("traced".to_string()),
});

// Insert before the closing brace of a class body
output.runtime_patches.push(
    macroforge_ts::ts_syn::insert_into_class(class.body_span(), "static version = 1;")
        .with_source_macro("Versioned"),
);

// Delete a span
output.runtime_patches.push(Patch::Delete { span: decorator_span });
```

`source_macro` names the macro in source maps and in errors about the patch. Patches from one
expansion must not overlap, and a patch outside the file is an error.

### Spans

A `SpanIR` marks a range of the file in _positions_: byte offsets plus one, so the first byte of the
file is position 1. Spans from the IR are already in positions, and `source_range()` turns one into
the 0-based byte range of the file it came from.

A macro sees the target's own text, not the whole file: `ctx.target_source` is the file's text from
`ctx.target_span.start` to `ctx.target_span.end`. To read the text of a span inside the target,
count from the target's start:

Rust

```
let offset = ctx.target_span.start;
let field_text = &ctx.target_source
    [(field.span.start - offset) as usize..(field.span.end - offset) as usize];
```

Note

An insertion point is a zero-width span, with `start == end`. Code inserted at position `n` goes
before the byte at offset `n - 1`.
