# Serialize

The `Serialize` macro generates JSON serialization methods with **cycle detection**
and object identity tracking. This enables serialization of complex object graphs
including circular references.

## Generated Methods

| Type | Generated Code | Description |
|------|----------------|-------------|
| Class | `classNameSerialize(value)` + `static serialize(value)` | Standalone function + static wrapper method |
| Enum | `enumNameSerialize(value)`, `enumNameSerializeWithContext` | Standalone functions |
| Interface | `interfaceNameSerialize(value)`, etc. | Standalone functions |
| Type Alias | `typeNameSerialize(value)`, etc. | Standalone functions |

## Cycle Detection Protocol

The generated code handles circular references using `__id` and `__ref` markers:

```json
{
    "__type": "User",
    "__id": 1,
    "name": "Alice",
    "friend": { "__ref": 2 }  // Reference to object with __id: 2
}
```

When an object is serialized:
1. Check if it's already been serialized (has an `__id`)
2. If so, return `{ "__ref": existingId }` instead
3. Otherwise, register the object and serialize its fields