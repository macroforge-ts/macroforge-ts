# Encode

The `Encode` macro generates JSON encoding methods with **cycle detection** and object identity
tracking. This enables encoding of complex object graphs including circular references.

## Generated Methods

| Type       | Generated Code                                                                                                                                      | Description                           |
| ---------- | --------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------- |
| Class      | `static encode(value, keepMetadata?)`, `static encodeWithContext`, `static encodePartialWithContext` + `classNameEncode(value, keepMetadata?)` etc. | Static methods + standalone functions |
| Enum       | `enumNameEncode(value)`, `enumNameEncodeWithContext`                                                                                                | Standalone functions                  |
| Interface  | `interfaceNameEncode(value)`, etc.                                                                                                                  | Standalone functions                  |
| Type Alias | `typeNameEncode(value)`, etc.                                                                                                                       | Standalone functions                  |
