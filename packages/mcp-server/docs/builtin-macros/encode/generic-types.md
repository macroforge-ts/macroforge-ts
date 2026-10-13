## Generic Types

A type parameter says nothing of how its values are encoded, so `nameEncodeWithContext` and
`nameEncodePartialWithContext` of a generic type take one optional `Encoder` for each parameter,
after their other parameters. A field written with the type's arguments hands them in:

```typescript
/** @derive(Encode) */
interface Box<T> {
    held: T;
    list: T[];
}

/** @derive(Encode) */
interface Shelf {
    dated: Box<Date>;
}
// shelfEncodeWithContext calls
// boxEncodeWithContext<Date>(value.dated, ctx, (date) => date.toISOString())
```

A union alias hands the encoders on to a member written with type arguments. The top-level
`nameEncode` and a class's static `encode` take the same encoders after `keepMetadata`,
`boxEncode(box, false, (date) => date.toISOString())`. Without an encoder a value of the parameter
is written as it is. A type argument a field leaves to its default, `Box` for `Box<T = Date>`, is
encoded as that default.

## Field-Level Options

The `@endec` decorator supports:

- `skip` / `skipEncoding` - Exclude field from encoding
- `rename = "jsonKey"` - Use different JSON property name
- `flatten` - Merge nested object's fields into parent. The flattened value is written out by its
  fields, with no `__id` of its own
