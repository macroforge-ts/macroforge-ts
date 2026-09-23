# Built-in Macros

Macroforge comes with built-in derive macros that cover the most common code generation needs. All
macros work with classes, interfaces, enums, and type aliases.

## Overview

| Macro                                               | Generates                                                                                                                     | Description                             |
| --------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- | --------------------------------------- |
| [`Debug`](../docs/builtin-macros/debug)             | `static toString(value: T): string`                                                                                           | Human-readable string representation    |
| [`Clone`](../docs/builtin-macros/clone)             | `static clone(value: T): T`                                                                                                   | Creates a deep copy of the object       |
| [`Default`](../docs/builtin-macros/default)         | `static defaultValue(): T`                                                                                                    | Creates an instance with default values |
| [`Hash`](../docs/builtin-macros/hash)               | `static hashCode(value: T): number`                                                                                           | Generates a hash code for the object    |
| [`PartialEq`](../docs/builtin-macros/partial-eq)    | `static equals(a: T, b: T): boolean`                                                                                          | Value equality comparison               |
| [`Ord`](../docs/builtin-macros/ord)                 | `static compareTo(a: T, b: T): number`                                                                                        | Total ordering comparison (-1, 0, 1)    |
| [`PartialOrd`](../docs/builtin-macros/partial-ord)  | `static compareTo(a: T, b: T): number &#124; null`                                                                            | Partial ordering comparison             |
| [`Serialize`](../docs/builtin-macros/serialize)     | `static serialize(value: T, keepMetadata?: boolean): string`                                                                  | JSON serialization with type handling   |
| [`Deserialize`](../docs/builtin-macros/deserialize) | `static deserialize(input, opts?): &lbrace; success: true; value: T &rbrace; &#124; &lbrace; success: false; errors &rbrace;` | JSON deserialization with validation    |

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

## Interface Support

All built-in macros work with interfaces. Interfaces have no class to attach statics to, so they get
standalone functions named `&lbrace;typeName&rbrace;&lbrace;Operation&rbrace;` taking the value as
the first parameter. When `generateConvenienceConst` is enabled (the default), a grouping `const` is
also emitted so you can call them by short name:

TypeScript

```
/** @derive(Debug, Clone, PartialEq) */
interface Point {
  x: number;
  y: number;
}

// Generated standalone functions:
// export function pointToString(value: Point): string { ... }
// export function pointClone(value: Point): Point { ... }
// export function pointEquals(a: Point, b: Point): boolean { ... }
// export function pointHashCode(value: Point): number { ... }

// Plus a grouping const (generateConvenienceConst, on by default):
// export const Point = {
//   toString: pointToString,
//   clone: pointClone,
//   equals: pointEquals,
//   hashCode: pointHashCode,
// } as const;

const point: Point = { x: 10, y: 20 };

console.log(pointToString(point));      // "Point { x: 10, y: 20 }"
const copy = pointClone(point);         // { x: 10, y: 20 }
console.log(pointEquals(point, copy));  // true

// …or via the grouping const
console.log(Point.toString(point));
```

## Enum Support

All built-in macros work with enums. For enums, methods are generated as functions in a namespace
with the same name:

TypeScript

```
/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}

// Generated standalone functions:
// export function statusToString(value: Status): string { ... }
// export function statusClone(value: Status): Status { ... }
// export function statusEquals(a: Status, b: Status): boolean { ... }
// export function statusHashCode(value: Status): number { ... }
// export function statusSerialize(value: Status): string { ... }
// export function statusDeserialize(input: unknown): Status { ... }

// Enums use namespace merging for the convenience names:
// namespace Status {
//   export const toString = statusToString;
//   export const serialize = statusSerialize;
// }

console.log(statusToString(Status.Active));                // "Status.Active"
console.log(statusEquals(Status.Active, Status.Active));   // true
const json = statusSerialize(Status.Pending);              // "pending"
// Note: enum deserialize throws on invalid input rather than
// returning a success/errors union.
const parsed = statusDeserialize("active");                // Status.Active
```

## Type Alias Support

All built-in macros work with type aliases. Object type aliases get field-aware standalone
functions, plus the optional grouping `const`:

TypeScript

```
/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
type Point = {
  x: number;
  y: number;
};

// Generated standalone functions:
// export function pointToString(value: Point): string { ... }
// export function pointClone(value: Point): Point { ... }
// export function pointEquals(a: Point, b: Point): boolean { ... }
// export function pointHashCode(value: Point): number { ... }
// export function pointSerialize(value: Point, keepMetadata?: boolean): string { ... }
// export function pointDeserialize(input: unknown, opts?): { success: true; value: Point }
//                                                        | { success: false; errors } { ... }

const point: Point = { x: 10, y: 20 };
console.log(pointToString(point));      // "Point { x: 10, y: 20 }"
const copy = pointClone(point);         // { x: 10, y: 20 }
console.log(pointEquals(point, copy));  // true
```

Union type aliases also work, using JSON-based implementations:

TypeScript

```
/** @derive(Debug, PartialEq) */
type ApiStatus = "loading" | "success" | "error";

const status: ApiStatus = "success";
console.log(ApiStatus.toString(status)); // "ApiStatus(\\"success\\")"
console.log(ApiStatus.equals("success", "success")); // true
```

## Combining Macros

All macros can be used together. They don't conflict and each generates independent methods:

TypeScript

```
const user = new User("Alice", 30);

// Debug
console.log(User.toString(user));
// "User { name: Alice, age: 30 }"

// Clone
const copy = User.clone(user);
console.log(copy.name); // "Alice"

// PartialEq
console.log(User.equals(user, copy)); // true
```

## Detailed Documentation

Each macro has its own options and behaviors:

- [**Debug**](../docs/builtin-macros/debug) - Customizable field renaming and skipping
- [**Clone**](../docs/builtin-macros/clone) - Deep copying for all field types
- [**Default**](../docs/builtin-macros/default) - Default value generation with field attributes
- [**Hash**](../docs/builtin-macros/hash) - Hash code generation for use in maps and sets
- [**PartialEq**](../docs/builtin-macros/partial-eq) - Value-based equality comparison
- [**Ord**](../docs/builtin-macros/ord) - Total ordering for sorting
- [**PartialOrd**](../docs/builtin-macros/partial-ord) - Partial ordering comparison
- [**Serialize**](../docs/builtin-macros/serialize) - JSON serialization with serde-style options
- [**Deserialize**](../docs/builtin-macros/deserialize) - JSON deserialization with validation
