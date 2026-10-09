# PartialEq

The `PartialEq` macro generates an `equals()` method for field-by-field structural equality
comparison. This is analogous to Rust's `PartialEq` trait, enabling value-based equality semantics
instead of reference equality.

## Generated Output

| Type       | Generated Code                                                     | Description                                          |
| ---------- | ------------------------------------------------------------------ | ---------------------------------------------------- |
| Class      | `classNameEquals(a, b)` + `static equals(a, b)`                    | Standalone function + static wrapper method          |
| Enum       | `enumNameEquals(a: EnumName, b: EnumName): boolean`                | Standalone function using strict equality            |
| Interface  | `interfaceNameEquals(a: InterfaceName, b: InterfaceName): boolean` | Standalone function comparing fields                 |
| Type Alias | `typeNameEquals(a: TypeName, b: TypeName): boolean`                | Standalone function with type-appropriate comparison |

## Comparison Strategy

The generated equality check:

1. **Identity check**: `a === b` returns true immediately
2. **Field comparison**: Compares each non-skipped field

## Type-Specific Comparisons

| Type                                  | Comparison Method                                     |
| ------------------------------------- | ----------------------------------------------------- |
| Primitives and literals               | Strict equality (`===`)                               |
| Arrays                                | Length, then each element by its own type             |
| `Date`, `RegExp`, `URL`, typed arrays | By value                                              |
| `Map`                                 | Size, then each key's value by its own type           |
| `Set`                                 | Size, then each element matched by an equal element   |
| Optional and nullable                 | Equal when both are the same absent value             |
| Types deriving `PartialEq`            | Their `equals` function                               |
| Anything else                         | `structuralEquals` from `@macroforge/core/structural` |

## Field-Level Options

The `@partialEq` decorator supports:

- `skip` - Exclude the field from equality comparison

## Example

```typescript before
/** @derive(PartialEq) */
class User {
    id: number;
    name: string;

    /** @partialEq({ skip: true }) */
    cachedScore: number;
}
```

```typescript after
class User {
    id: number;
    name: string;

    cachedScore: number;

    static equals(a: User, b: User): boolean {
        return userEquals(a, b);
    }
}

export function userEquals(a: User, b: User): boolean {
    if (a === b) return true;
    return a.id === b.id && a.name === b.name;
}
```

## Equality Contract

When implementing `PartialEq`, consider also implementing `Hash`:

- **Reflexivity**: `User.equals(a, a)` is always true
- **Symmetry**: `User.equals(a, b)` implies `User.equals(b, a)`
- **Hash consistency**: Equal objects must have equal hash codes

To maintain the hash contract, skip the same fields in both `PartialEq` and `Hash`, as shown in the
example above using `@partialEq({ skip: true }) @hash({ skip: true })`.
