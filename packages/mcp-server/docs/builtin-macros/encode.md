# Encode

The `Encode` macro generates JSON encoding methods with **cycle detection** and object identity
tracking. This enables encoding of complex object graphs including circular references.

## Generated Methods

| Type       | Generated Code                                                                  | Description                                 |
| ---------- | ------------------------------------------------------------------------------- | ------------------------------------------- |
| Class      | `classNameEncode(value, keepMetadata?)` + `static encode(value, keepMetadata?)` | Standalone function + static wrapper method |
| Enum       | `enumNameEncode(value)`, `enumNameEncodeWithContext`                            | Standalone functions                        |
| Interface  | `interfaceNameEncode(value)`, etc.                                              | Standalone functions                        |
| Type Alias | `typeNameEncode(value)`, etc.                                                   | Standalone functions                        |

## Cycle Detection Protocol

The generated code handles circular references using `__id` and `__ref` markers:

```json
{
    "__type": "User",
    "__id": 1,
    "name": "Alice",
    "friend": { "__ref": 2 } // Reference to object with __id: 2
}
```

When an object is encoded:

1. Check if it's already been encoded (has an `__id`)
2. If so, return `{ "__ref": existingId }` instead
3. Otherwise, register the object and encode its fields

## Type-Specific Encoding

| Type                                           | Encoding Strategy                                                                                                                  |
| ---------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| Primitives                                     | Direct value                                                                                                                       |
| `Date`                                         | `toISOString()`                                                                                                                    |
| Arrays, readonly or not                        | For primitive-like element types, pass through; for `Date`/`Date                                                                   |
| `Map<K,V>`, `ReadonlyMap<K,V>`                 | For primitive-like values, `Object.fromEntries(map.entries())`; for `Date`/`Date                                                   |
| `Set<T>`, `ReadonlySet<T>`                     | Convert to array; element handling matches `Array<T>`                                                                              |
| `Record<K,V>`                                  | For primitive-like values, pass through; for `Date`/`Date                                                                          |
| `Required<T>`, `Readonly<T>`, `NonNullable<T>` | Encode as the inner type `T`                                                                                                       |
| `Partial<T>`, `Pick<T,K>`, `Omit<T,K>`         | Call `nameEncodePartialWithContext`, which encodes the fields the value has and, for a `Pick` or `Omit`, only those the type keeps |
| Nullable                                       | Include `null` explicitly; non-null values follow the inner type's strategy                                                        |
| Objects                                        | Call the type's `nameEncodeWithContext` function                                                                                   |

Note: which strategy applies is resolved **statically** from the field's declared TypeScript type at
expansion time: there is no runtime feature detection.

## Field-Level Options

The `@endec` decorator supports:

- `skip` / `skipEncoding` - Exclude field from encoding
- `rename = "jsonKey"` - Use different JSON property name
- `flatten` - Merge nested object's fields into parent

## Example

```typescript before
/** @derive(Encode) */
class User {
    id: number;

    /** @endec({ rename: "userName" }) */
    name: string;

    /** @endec({ skipEncoding: true }) */
    password: string;

    /** @endec({ flatten: true }) */
    metadata: UserMetadata;
}
```

```typescript after
import {
    EncodeContext as __mf_EncodeContext,
    encodedText as __mf_encodedText,
    encodedTextWithoutMetadata as __mf_encodedTextWithoutMetadata
} from '@macroforge/core/endec';

class User {
    id: number;

    name: string;

    password: string;

    metadata: UserMetadata;
    /** Encodes a value to a JSON string.
@param value - The value to encode
@returns JSON string representation with cycle detection metadata  */

    static encode(value: User, keepMetadata?: boolean): string {
        return userEncode(value, keepMetadata);
    }
    /** @internal Encodes with an existing context for nested/cyclic object graphs.
@param value - The value to encode
@param ctx - The encoding context  */

    static encodeWithContext(value: User, ctx: __mf_EncodeContext): Record<string, unknown> {
        return userEncodeWithContext(value, ctx);
    }
}

/** Encodes a value to a JSON string.
@param value - The value to encode
@returns JSON string representation with cycle detection metadata */ export function userEncode(
    value: User,
    keepMetadata?: boolean
): string {
    const ctx = __mf_EncodeContext.create();
    const __raw = userEncodeWithContext(value, ctx);
    return keepMetadata
        ? __mf_encodedText(__raw)
        : __mf_encodedTextWithoutMetadata(__raw, '__type');
} /** @internal Encodes with an existing context for nested/cyclic object graphs.
@param value - The value to encode
@param ctx - The encoding context */

export function userEncodeWithContext(
    value: User,
    ctx: EncodeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'User', __id };
    result['id'] = value.id;
    result['userName'] = value.name;
    {
        const __flattened = userMetadataEncodeWithContext(value.metadata, ctx);
        const { __type: _, __id: __, ...rest } = __flattened as any; // tag field name is configurable via `tag` option
        Object.assign(result, rest);
    }
    return result;
}
```

## Required Imports

The generated code automatically imports `EncodeContext` and the helpers that give an encoded
value's JSON text from `@macroforge/core/endec`. `encode` throws a `TypeError` for a value JSON
cannot represent, such as a top-level `undefined`.
