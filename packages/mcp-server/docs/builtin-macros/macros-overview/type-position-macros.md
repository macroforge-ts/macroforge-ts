## Type-Position Macros

One built-in macro is written where a type goes rather than in `@derive`:

| Macro                                           | Expands To                                     | Description                                        |
| ----------------------------------------------- | ---------------------------------------------- | -------------------------------------------------- |
| [`$Newtype<T>`](../docs/builtin-macros/newtype) | `T & &lbrace; readonly [brand]: true &rbrace;` | Nominal brand backed by a per-site `unique symbol` |

## Using Built-in Macros

Built-in macros don't require imports. Just use them with `@derive`:

TypeScript

```
/** @derive(Debug, Clone, PartialEq) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}
```
