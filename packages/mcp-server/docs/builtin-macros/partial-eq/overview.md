# PartialEq

The `PartialEq` macro generates an `equals()` method for field-by-field
structural equality comparison. This is analogous to Rust's `PartialEq` trait,
enabling value-based equality semantics instead of reference equality.

## Generated Output

| Type | Generated Code | Description |
|------|----------------|-------------|
| Class | `equals(other: unknown): boolean` | Instance method with instanceof check |
| Enum | `equalsEnumName(a: EnumName, b: EnumName): boolean` | Standalone function using strict equality |
| Interface | `equalsInterfaceName(a: InterfaceName, b: InterfaceName): boolean` | Standalone function comparing fields |
| Type Alias | `equalsTypeName(a: TypeName, b: TypeName): boolean` | Standalone function with type-appropriate comparison |

## Comparison Strategy

The generated equality check:

1. **Identity check**: `this === other` returns true immediately
2. **Type check**: For classes, uses `instanceof`; returns false if wrong type
3. **Field comparison**: Compares each non-skipped field

## Type-Specific Comparisons

| Type | Comparison Method |
|------|-------------------|
| Primitives | Strict equality (`===`) |
| Arrays | Length + element-by-element (recursive) |
| `Date` | `getTime()` comparison |
| `Map` | Size + entry-by-entry comparison |
| `Set` | Size + membership check |
| Objects | Calls `equals()` if available, else `===` |

## Field-Level Options

The `@partialEq` decorator supports:

- `skip` - Exclude the field from equality comparison