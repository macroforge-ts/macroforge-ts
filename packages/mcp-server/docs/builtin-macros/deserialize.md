# Deserialize

The `Deserialize` macro generates JSON deserialization methods with **cycle and forward-reference
support**, plus comprehensive runtime validation. This enables safe parsing of complex JSON
structures including circular references.

## Generated Output

For **classes** (`class_handler`), the macro generates static methods plus standalone functions
(`nameDeserialize`, `nameDeserializeWithContext`, `nameIs`) that delegate to them:

- `static deserialize(input: unknown, opts?: DeserializeOptions):
  { success: true; value: T } | { success: false; errors: Array<{ field: string; message: string }> }` -
  Auto-detects JSON string vs object; never throws. `opts.freeze` freezes all deserialized objects.
- `static deserializeWithContext(value, ctx): T | PendingRef` - Internal method used for
  nested/cyclic graphs; throws `DeserializeError` on structural errors
- `static hasShape(obj): boolean` - Checks all required JSON keys are present
- `static is(value): value is T` - Type guard: instanceof check, then `hasShape`, then a full
  `deserialize`
- `static validateField(field, value)` / `static validateFields(partial)` - Run the field validators
  without deserializing
- A synthesized `constructor(props)` that assigns all deserialized fields

**Interfaces** and **type aliases** get the standalone-function forms of the same surface (there is
no class to attach statics to). **Enums** (`enum_handler`) get
`nameDeserialize`/`nameDeserializeWithContext`/`nameIs`; unlike the other shapes, the enum
`deserialize` function **throws** an `Error` on invalid values rather than returning a result union.

Validation (see `validation`) runs during deserialization and reports failures through the `errors`
array of the result union. Field-level options are parsed by `field_processing`; see the parent
serde module for the option and validator reference.
