## Union Type Deserialization

Union types are deserialized based on their member types:

### Literal Unions
For unions of literal values (`"A" | "B" | 123`), the value is validated against
the allowed literals directly.

### Primitive Unions
For unions containing primitive types (`string | number`), the deserializer uses
`typeof` checks to validate the value type. No `__type` discriminator is needed.

### Class/Interface Unions
For unions of serializable types (`User | Admin`), the deserializer requires a
`__type` field in the JSON to dispatch to the correct type's `__deserialize` method.

### Generic Type Parameters
For generic unions like `type Result<T> = T | Error`, the generic type parameter `T`
is passed through as-is since its concrete type is only known at the call site.

### Mixed Unions
Mixed unions (e.g., `string | Date | User`) check in order:
1. Literal values
2. Primitives (via `typeof`)
3. Date (via `instanceof` or ISO string parsing)
4. Serializable types (via `__type` dispatch)
5. Generic type parameters (pass-through)