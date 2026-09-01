# Field Options

Field options go in a `@serde(...)` decorator on the field.

```typescript
/** @derive(Serialize, Deserialize) */
class User {
    /** @serde(rename: "user_name") */
    name: string;

    /** @serde(skip) */
    cachedAvatar?: Blob;

    /** @serde(default: "\"unknown\"") */
    status: string;
}
```

## Reference

| Option              | Form                    | Effect                                                   |
| ------------------- | ----------------------- | -------------------------------------------------------- |
| `skip`              | flag                    | Omit from both serialization and deserialization         |
| `skipSerializing`   | flag                    | Omit from output only                                    |
| `skipDeserializing` | flag                    | Ignore in input only                                     |
| `rename`            | `rename: "key"`         | Use a different JSON key                                 |
| `default`           | flag                    | Use the type's default when the key is missing           |
| `default`           | `default: "expr"`       | Use the given expression when the key is missing         |
| `flatten`           | flag                    | Inline the nested object's fields into the parent        |
| `serializeWith`     | `serializeWith: "fn"`   | Call your function instead of the generated serializer   |
| `deserializeWith`   | `deserializeWith: "fn"` | Call your function instead of the generated deserializer |
| `format`            | `format: "decimal"`     | Serialize a number as a string                           |
| `validate`          | `validate: [...]`       | Run [validators](/docs/serde/validators)                 |

## `skip` and its variants

`skip` is symmetric. When you need asymmetry — a server-computed field that should be read but never
written, or a secret that should be written but never echoed — use the directional forms.

A skipped-on-deserialize field needs a value from somewhere, so pair it with `default`.

## `default`

The flag form falls back to the type's natural default. The expression form takes a string
containing the expression to evaluate:

```typescript
/** @serde(default) */
count: number; // → 0 when missing

/** @serde(default: "\"pending\"") */
status: string; // → "pending" when missing
```

## `flatten`

Lifts a nested object's fields into the parent's JSON:

```typescript
class Meta {
    createdAt: string;
}

/** @derive(Serialize) */
class Post {
    title: string;
    /** @serde(flatten) */
    meta: Meta;
}
```

```json
{ "title": "Hello", "createdAt": "2026-01-01" }
```

## Custom serializers

`serializeWith` / `deserializeWith` hand a single field to your own function — the escape hatch for
a shape the generated code can't express. For a type you use repeatedly across many classes, prefer
[foreign types](/docs/serde/foreign-types), which configure the behavior once globally.

## `format: "decimal"`

Serializes a number as a **string**, preserving precision for values that would lose it as a JSON
number — monetary amounts, large IDs:

```typescript
/** @serde(format: "decimal") */
balance: number;
```

```json
{ "balance": "1234.56" }
```
