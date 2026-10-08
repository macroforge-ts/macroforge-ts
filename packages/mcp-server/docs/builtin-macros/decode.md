# Decode

The `Decode` macro generates JSON decoding methods with **cycle and forward-reference support**,
plus comprehensive runtime validation. This enables safe parsing of complex JSON structures
including circular references.

## Generated Output

For **classes** (`class_handler`), the macro generates static methods plus standalone functions
(`nameDecode`, `nameDecodeWithContext`, `nameIs`) that delegate to them:

- `static decode(input: unknown, opts?: DecodeOptions):
  { success: true; value: T } | { success: false; errors: Array<{ field: string; message: string }> }` -
  Auto-detects JSON string vs object; never throws. `opts.freeze` freezes all decoded objects.
- `static decodeWithContext(value, ctx): T | PendingRef` - Internal method used for nested/cyclic
  graphs; throws `DecodeError` on structural errors
- `static hasShape(obj): boolean` - Checks all required JSON keys are present
- `static is(value): value is T` - Type guard: instanceof check, then `hasShape`, then a full
  `decode`
- `static validateField(field, value)` / `static validateFields(partial)` - Run the field validators
  without decoding
- A synthesized `constructor(props)` that assigns all decoded fields

**Interfaces** and **type aliases** get the standalone-function forms of the same surface (there is
no class to attach statics to). **Enums** (`enum_handler`) get
`nameDecode`/`nameDecodeWithContext`/`nameIs`; unlike the other shapes, the enum `decode` function
**throws** an `Error` on invalid values rather than returning a result union.

Validation (see `validation`) runs during decoding and reports failures through the `errors` array
of the result union. Field-level options are parsed by `field_processing`; see the parent endec
module for the option and validator reference.
