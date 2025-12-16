## Type-Specific Serialization

| Type | Serialization Strategy |
|------|------------------------|
| Primitives | Direct value |
| `Date` | `toISOString()` |
| Arrays | For primitive-like element types, pass through; for `Date`/`Date | null`, map to ISO strings; otherwise map and call `SerializeWithContext(ctx)` when available |
| `Map<K,V>` | For primitive-like values, `Object.fromEntries(map.entries())`; for `Date`/`Date | null`, convert to ISO strings; otherwise call `SerializeWithContext(ctx)` per value when available |
| `Set<T>` | Convert to array; element handling matches `Array<T>` |
| Nullable | Include `null` explicitly; for primitive-like and `Date` unions the generator avoids runtime `SerializeWithContext` checks |
| Objects | Call `SerializeWithContext(ctx)` if available (to support user-defined implementations) |

Note: the generator specializes some code paths based on the declared TypeScript type to
avoid runtime feature detection on primitives and literal unions.

## Field-Level Options

The `@serde` decorator supports:

- `skip` / `skipSerializing` - Exclude field from serialization
- `rename = "jsonKey"` - Use different JSON property name
- `flatten` - Merge nested object's fields into parent