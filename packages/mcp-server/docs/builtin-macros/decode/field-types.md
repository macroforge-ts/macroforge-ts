## Field Types

A field is decoded by the whole structure of its type, to any depth, and every member is checked on
the way:

| Type                                                                                  | Decoding                                                                                                                                                                                          |
| ------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `string`, `number`, `boolean`, `bigint`, literals and their unions                    | The value, when it is one of the type's                                                                                                                                                           |
| `Date`                                                                                | A `Date` from its ISO string                                                                                                                                                                      |
| Arrays, `Set<T>`                                                                      | From an array, each element decoded by the element type                                                                                                                                           |
| `Map<K,V>`                                                                            | From an object, each value decoded by `V` and each key read back as `K`: a number, bigint, boolean, `Date`, enum or newtype from its text                                                         |
| `Record<K,V>`                                                                         | From an object, each value decoded by `V` and each key checked to be a `K`. Over a finite set of keys, `Record<'a' \| 'b', V>` or an enum, every key has to be there                              |
| Tuples                                                                                | From an array of the tuple's length, each element decoded by its own type                                                                                                                         |
| An object type written in place                                                       | An object of exactly its members, each decoded by its own type                                                                                                                                    |
| `Partial<T>`, `Pick<T,K>`, `Omit<T,K>`                                                | `nameDecodePartialWithContext`                                                                                                                                                                    |
| `Required<T>`                                                                         | As `T`, with the fields `T` leaves optional required too, unless a default fills them                                                                                                             |
| A type deriving `Decode`                                                              | Its `nameDecodeWithContext` function                                                                                                                                                              |
| A configured foreign type                                                             | Its `decode` handler                                                                                                                                                                              |
| A type parameter                                                                      | The decoder handed in for it, or the value as it is, unchecked, without one                                                                                                                       |
| `string \| User`, a primitive beside one type deriving `Decode`                       | The primitive as it is, anything else through the type                                                                                                                                            |
| Any other union with a member that is decoded, `Circle \| Square`, `Date \| number`   | An expansion error: nothing in a field says which member a value is. Name the union with an alias and derive on that                                                                              |
| A union of values that are their own encoding, `string \| string[]`, `Date \| string` | The value as it is, checked to be one of the members                                                                                                                                              |
| `object`                                                                              | The value, when it is an object                                                                                                                                                                   |
| A template literal type, `` `id-$number` ``                                           | The text, when it spells the template: its fixed text in place, a `$number` or `$bigint` hole a number in decimal, a hole of literals one of them. A hole of any other type is an expansion error |
| `keyof T`, `T['field']` over a type the project declares                              | As what the operator yields                                                                                                                                                                       |
| A function, a symbol, `void`, `never`, a mapped, conditional or `typeof` type         | An expansion error: code the field with `decodeWith`, or leave it out with `skip`                                                                                                                 |
| A type named through a namespace import, `models.User`                                | An expansion error: import the type by name                                                                                                                                                       |
| `unknown`, `any`, a type the project does not declare                                 | The value as it is, unchecked                                                                                                                                                                     |

A `null` in an array, a tuple or a `Set`, or as the value of a `Record` or `Map` entry, is read as
`undefined` where the type there takes `undefined` and not `null`: it is what `encode` writes for
one, as JSON has no `undefined`.

A number has to be finite. `NaN`, an infinity, and JSON text that spells a number too large to hold
(`1e999`) are refused: `encode` could not write them back. Only the members an input holds itself
are read, so one it inherits is not there.

A `$Newtype<T>` is decoded as the `T` it brands, whatever kind of type that is, and a union alias
tells a template literal member by its text.

A member that fails is reported under its path: `grid[0][1]`, `byName.key.id`. Members share the
context of the value that holds them, so a value held in several places, or a child pointing back at
its parent through a collection, decodes to one object.
