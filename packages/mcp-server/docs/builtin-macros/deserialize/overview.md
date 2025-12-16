# Deserialize

The `Deserialize` macro generates JSON deserialization methods with **cycle and
forward-reference support**, plus comprehensive runtime validation. This enables
safe parsing of complex JSON structures including circular references.

## Generated Output

| Type | Generated Code | Description |
|------|----------------|-------------|
| Class | `classNameDeserialize(input)` + `static deserialize(input)` | Standalone function + static factory method |
| Enum | `enumNameDeserialize(input)`, `enumNameDeserializeWithContext(data)`, `enumNameIs(value)` | Standalone functions |
| Interface | `interfaceNameDeserialize(input)`, etc. | Standalone functions |
| Type Alias | `typeNameDeserialize(input)`, etc. | Standalone functions |

## Return Type

All public deserialization methods return `Result<T, Array<{ field: string; message: string }>>`:

- `Result.ok(value)` - Successfully deserialized value
- `Result.err(errors)` - Array of validation errors with field names and messages