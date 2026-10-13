## Cycle Detection Protocol

The generated code handles circular references using `__id` and `__ref` markers:

```json
{
    "__type": "User",
    "__id": 1,
    "name": "Alice",
    "friend": { "__ref": 2 } // Reference to object with __id: 2
}
```

When an object is encoded:

1. Check if it's already been encoded (has an `__id`)
2. If so, return `{ "__ref": existingId }` instead
3. Otherwise, register the object and encode its fields

`encode(value)` leaves `__type` and `__id` out of its text unless `keepMetadata` is set. Nothing is
then left for a reference to name, so a value held in several places is written out at each of them,
and a value that holds itself, which only a reference can write, throws a `TypeError`.
