# Debug

The `Debug` macro generates a human-readable `toString()` method for TypeScript classes, interfaces,
enums, and type aliases.

## Generated Output

**Classes**: Generates the static method `static toString(value)`, returning a string like
`"ClassName { field1: value1, field2: value2 }"`, and a standalone function
`classNameToString(value)` that runs it.

**Enums**: Generates a standalone function `enumNameToString(value)` that names the member a value
is, `Status.Active`, for an enum of numbers, of strings or of both, and for a `const enum`.

**Interfaces**: Generates a standalone function `ifaceNameToString(value)`.

**Type Aliases**: Generates a standalone function listing the fields of an object type, or rendering
any other value by its type.

Primitives and built-ins render as `String` renders them, types deriving `Debug` through their
`toString`, and anything else (maps, sets, object literals, unions) through `structuralDebug` from
`@macroforge/core/structural`.

Names use **camelCase** conversion (e.g., `User` -> `userToString`).

## Field-Level Options

The `@debug` decorator supports:

- `skip` - Exclude the field from debug output
- `rename = "label"` - Use a custom label instead of the field name

## Example

```typescript before
/** @derive(Debug) */
class User {
    /** @debug({ rename: "id" }) */
    userId: number;

    /** @debug({ skip: true }) */
    password: string;

    email: string;
}
```

```typescript after
class User {
    userId: number;

    password: string;

    email: string;

    static toString(value: User): string {
        const parts: string[] = [];
        parts.push('id: ' + value.userId);
        parts.push('email: ' + value.email);
        return 'User { ' + parts.join(', ') + ' }';
    }
}

export function userToString(value: User): string {
    return User.toString(value);
}
```
