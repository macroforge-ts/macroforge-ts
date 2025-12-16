# Deserialize

The `Deserialize` macro generates JSON deserialization methods with **cycle and
forward-reference support**, plus comprehensive runtime validation. This enables
safe parsing of complex JSON structures including circular references.

## Generated Output

| Type | Generated Code | Description |
|------|----------------|-------------|
| Class | `static deserialize()`, `static __deserialize()` | Static factory methods |
| Enum | `myEnumDeserialize(input)`, `myEnum__deserialize(data)`, `myEnumIs(value)` | Standalone functions |
| Interface | `myInterfaceDeserialize(input)`, etc. | Standalone functions |
| Type Alias | `myTypeDeserialize(input)`, etc. | Standalone functions |

## Configuration

The `functionNamingStyle` option in `macroforge.json` controls naming:
- `"prefix"` (default): Prefixes with type name (e.g., `myTypeDeserialize`)
- `"suffix"`: Suffixes with type name (e.g., `deserializeMyType`)
- `"generic"`: Uses TypeScript generics (e.g., `deserialize<T extends MyType>`)
- `"namespace"`: Namespace wrapping (e.g., `MyType.deserialize`)

## Return Type

All public deserialization methods return `Result<T, Array<{ field: string; message: string }>>`:

- `Result.ok(value)` - Successfully deserialized value
- `Result.err(errors)` - Array of validation errors with field names and messages