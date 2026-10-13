# Decode

The `Decode` macro generates JSON decoding methods with **cycle and forward-reference support**,
plus comprehensive runtime validation. This enables safe parsing of complex JSON structures
including circular references.

## Generated Output

For **classes** (`class_handler`), the macro generates static methods plus standalone functions
(`nameDecode`, `nameDecodeWithContext`, `nameIs`) that delegate to them:

- `static decode(input: unknown, opts?: DecodeOptions):
  { success: true; value: T } | { success: false; errors: Array<{ field: string; message: string }> }` -
  Auto-detects JSON string vs object; never throws. `opts.freeze` freezes all decoded objects,
  arrays, tuples and records; a `Map` and a `Set` have no frozen form.
- `static decodeWithContext(value, ctx): T | PendingRef` - Internal method used for nested/cyclic
  graphs; throws `DecodeError` on structural errors
- `static hasShape(obj): boolean` - Checks all required JSON keys are present
- `static is(value): value is T` - Type guard: instanceof check, then `hasShape`, then a full
  `decode`
- `static validateField(field, value)` / `static validateFields(partial)` - Run the field validators
  on input that has not been decoded, typed `unknown`
- A synthesized `constructor(props)` that assigns the fields, when the class has no constructor of
  its own. A subclass's hands them to its superclass's when that derives `Decode`

`decode` takes JSON text or a value already parsed from it. Where the type itself takes a string, as
an alias of `string` or of `Date`, an enum of strings and a union with a string or a string-literal
member do, a string is the JSON text of a value when it spells one the type takes, and the value
itself otherwise. So `decode(encode(value))` reads back what was encoded, and `decode("small")` is
still the member `small`.

Decoding builds an instance without running its constructor, so a class keeps whatever constructor
it has. Its `private`, `protected` and `readonly` fields, its constructor parameter properties and
the fields of its superclasses are decoded like any other.

**Interfaces** and **type aliases** get the standalone-function forms of the same surface (there is
no class to attach statics to), except that only an object-shaped alias has fields to validate, so
no other alias gets `validateField`/`validateFields`. **Enums** (`enum_handler`) get
`nameDecode`/`nameDecodeWithContext`/`nameIs`; unlike the other shapes, the enum `decode` function
**throws** a `DecodeError` on invalid values rather than returning a result union.

A **type alias of a primitive**, plain (`type Port = number`) or branded
(`type Meters = $Newtype<number>`), decodes to that primitive: `decode` rejects any other `typeof`,
`hasShape` checks the `typeof`, and `is` additionally runs the alias's validators. Validators go on
the alias itself:

```typescript
/** @derive(Decode) */
/** @endec(nonNegative, finite) */
type Meters = $Newtype<number>;

metersDecode(-1); // { success: false, errors: [{ field: "_root", ... }] }
metersIs('12'); // false
```

Alias-level validators on any other alias shape are an expansion error; put them on the fields
instead.

A class, an interface and an object-shaped alias also get
`nameDecodePartialWithContext(value, ctx, fields?)`, which a field typed `Partial<T>`, `Pick<T, K>`
or `Omit<T, K>` decodes through. `fields` is the shape: `{ only: [...] }` for a `Pick`,
`{ except: [...] }` for an `Omit`, and left out for a `Partial`. A field the shape lacks is not
read, even when the input has it; one it keeps is required as `T` requires it, except under
`Partial`, where none is. Such a value is not registered for references and may not be one, since a
reference stands for a whole `T`. The keys of a `Pick` or `Omit` have to be string literals.
