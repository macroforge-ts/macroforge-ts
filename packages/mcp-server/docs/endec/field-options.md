# Field Options

Field options go in a `@endec(...)` decorator on the field.

```typescript
/** @derive(Encode, Decode) */
class User {
    /** @endec(rename: "user_name") */
    name: string;

    /** @endec(skip) */
    cachedAvatar?: Blob;

    /** @endec(default: "\"unknown\"") */
    status: string;
}
```

## Reference

| Option         | Form                | Effect                                              |
| -------------- | ------------------- | --------------------------------------------------- |
| `skip`         | flag                | Omit from both encoding and decoding                |
| `skipEncoding` | flag                | Omit from output only                               |
| `skipDecoding` | flag                | Ignore in input only                                |
| `rename`       | `rename: "key"`     | Use a different JSON key                            |
| `default`      | flag                | Use the type's default when the key is missing      |
| `default`      | `default: "expr"`   | Use the given expression when the key is missing    |
| `flatten`      | flag                | Inline the nested object's fields into the parent   |
| `encodeWith`   | `encodeWith: "fn"`  | Call your function instead of the generated encoder |
| `decodeWith`   | `decodeWith: "fn"`  | Call your function instead of the generated decoder |
| `format`       | `format: "decimal"` | Encode a number as a string                         |
| `validate`     | `validate: [...]`   | Run [validators](/docs/endec/validators)            |

## `skip` and its variants

`skip` is symmetric. When you need asymmetry: a server-computed field that should be read but never
written, or a secret that should be written but never echoed: use the directional forms.

A skipped-on-decode field needs a value from somewhere, so pair it with `default`.

## `default`

The flag form falls back to the type's natural default. The expression form takes a string
containing the expression to evaluate:

```typescript
/** @endec(default) */
count: number; // → 0 when missing

/** @endec(default: "\"pending\"") */
status: string; // → "pending" when missing
```

## `flatten`

Lifts a nested object's fields into the parent's JSON:

```typescript
class Meta {
    createdAt: string;
}

/** @derive(Encode) */
class Post {
    title: string;
    /** @endec(flatten) */
    meta: Meta;
}
```

```json
{ "title": "Hello", "createdAt": "2026-01-01" }
```

## Custom encoders

`encodeWith` / `decodeWith` hand a single field to your own function: the escape hatch for a shape
the generated code can't express. For a type you use repeatedly across many classes, prefer
[foreign types](/docs/endec/foreign-types), which configure the behavior once globally.

## `format: "decimal"`

Encodes a number as a **string**, preserving precision for values that would lose it as a JSON
number: monetary amounts, large IDs:

```typescript
/** @endec(format: "decimal") */
balance: number;
```

```json
{ "balance": "1234.56" }
```
