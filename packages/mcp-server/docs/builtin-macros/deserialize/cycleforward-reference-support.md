## Cycle/Forward-Reference Support

Uses deferred patching to handle references:

1. When encountering `{ "__ref": id }`, returns a `PendingRef` marker
2. Continues deserializing other fields
3. After all objects are created, `ctx.applyPatches()` resolves all pending references

References only apply to object-shaped, serializable values. The generator avoids probing for
`__ref` on primitive-like fields (including literal unions and `T | null` where `T` is primitive-like),
and it parses `Date` / `Date | null` from ISO strings without treating them as references.