# Deserialize

The `Deserialize` macro generates JSON deserialization methods with **cycle and
forward-reference support**, plus comprehensive runtime validation. This enables
safe parsing of complex JSON structures including circular references.

## Generated Output

| Type | Generated Code | Description |
|------|----------------|-------------|
| Class | `static deserialize()`, `static deserializeWithContext()` | Static factory methods |
| Enum | `myEnumDeserialize(input)`, `myEnumDeserializeWithContext(data)`, `myEnumIs(value)` | Standalone functions |
| Interface | `myInterfaceDeserialize(input)`, etc. | Standalone functions |
| Type Alias | `myTypeDeserialize(input)`, etc. | Standalone functions |

## Return Type

All public deserialization methods return `Result<T, Array<{ field: string; message: string }>>`:

- `Result.ok(value)` - Successfully deserialized value
- `Result.err(errors)` - Array of validation errors with field names and messages