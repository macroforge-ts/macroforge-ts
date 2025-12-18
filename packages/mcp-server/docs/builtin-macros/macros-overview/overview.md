# Built-in Macros

Macroforge comes with built-in derive macros that cover the most common code generation needs. All macros work with classes, interfaces, enums, and type aliases.

## Overview

| Macro                                               | Generates                                 | Description                             |
| --------------------------------------------------- | ----------------------------------------- | --------------------------------------- |
| [`Debug`](../docs/builtin-macros/debug)             | `toString(): string`                      | Human-readable string representation    |
| [`Clone`](../docs/builtin-macros/clone)             | `clone(): T`                              | Creates a deep copy of the object       |
| [`Default`](../docs/builtin-macros/default)         | `static default(): T`                     | Creates an instance with default values |
| [`Hash`](../docs/builtin-macros/hash)               | `hashCode(): number`                      | Generates a hash code for the object    |
| [`PartialEq`](../docs/builtin-macros/partial-eq)    | `equals(other: T): boolean`               | Value equality comparison               |
| [`Ord`](../docs/builtin-macros/ord)                 | `compare(other: T): number`               | Total ordering comparison (-1, 0, 1)    |
| [`PartialOrd`](../docs/builtin-macros/partial-ord)  | `partialCompare(other: T): number &#124; null` | Partial ordering comparison             |
| [`Serialize`](../docs/builtin-macros/serialize)     | `toJSON(): Record<string, unknown>`       | JSON serialization with type handling   |
| [`Deserialize`](../docs/builtin-macros/deserialize) | `static fromJSON(data: unknown): T`       | JSON deserialization with validation    |

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