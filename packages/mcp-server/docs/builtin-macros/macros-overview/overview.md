# Built-in Macros

Macroforge comes with built-in derive macros that cover the most common code generation needs. All
macros work with classes, interfaces, enums, and type aliases.

## Overview

| Macro                                              | Generates                                                                                                                | Description                                                          |
| -------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------- |
| [`Debug`](../docs/builtin-macros/debug)            | `static toString(value: T): string`                                                                                      | Human-readable string representation                                 |
| [`Clone`](../docs/builtin-macros/clone)            | `static clone(value: T): T`                                                                                              | Creates a deep copy of the object                                    |
| [`Default`](../docs/builtin-macros/default)        | `static defaultValue(): T`                                                                                               | Creates an instance with default values                              |
| [`Hash`](../docs/builtin-macros/hash)              | `static hashCode(value: T): number`                                                                                      | Generates a hash code for the object                                 |
| [`PartialEq`](../docs/builtin-macros/partial-eq)   | `static equals(a: T, b: T): boolean`                                                                                     | Value equality comparison                                            |
| [`Eq`](../docs/builtin-macros/eq)                  | None                                                                                                                     | Marks equality as total; requires `PartialEq`                        |
| [`Ord`](../docs/builtin-macros/ord)                | `static compare(a: T, b: T): number`                                                                                     | Total ordering comparison (-1, 0, 1); requires `Eq` and `PartialOrd` |
| [`PartialOrd`](../docs/builtin-macros/partial-ord) | `static partialCompare(a: T, b: T): number &#124; null`                                                                  | Partial ordering comparison; requires `PartialEq`                    |
| [`Encode`](../docs/builtin-macros/encode)          | `static encode(value: T, keepMetadata?: boolean): string`                                                                | JSON encoding with type handling                                     |
| [`Decode`](../docs/builtin-macros/decode)          | `static decode(input, opts?): &lbrace; success: true; value: T &rbrace; &#124; &lbrace; success: false; errors &rbrace;` | JSON decoding with validation                                        |
