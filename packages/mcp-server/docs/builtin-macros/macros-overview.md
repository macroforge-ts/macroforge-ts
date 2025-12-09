# Built-in Macros

*Macroforge comes with built-in derive macros that cover the most common code generation needs. All macros work with classes, interfaces, enums, and type aliases.*

## Overview

| [`Debug`]({base}/docs/builtin-macros/debug) 
| `toString(): string` 
| Human-readable string representation 

| [`Clone`]({base}/docs/builtin-macros/clone) 
| `clone(): T` 
| Creates a deep copy of the object 

| [`Default`]({base}/docs/builtin-macros/default) 
| `static default(): T` 
| Creates an instance with default values 

| [`Hash`]({base}/docs/builtin-macros/hash) 
| `hashCode(): number` 
| Generates a hash code for the object 

| [`PartialEq`]({base}/docs/builtin-macros/partial-eq) 
| `equals(other: T): boolean` 
| Value equality comparison 

| [`Ord`]({base}/docs/builtin-macros/ord) 
| `compare(other: T): number` 
| Total ordering comparison (-1, 0, 1) 

| [`PartialOrd`]({base}/docs/builtin-macros/partial-ord) 
| `partialCompare(other: T): number | null` 
| Partial ordering comparison 

| [`Serialize`]({base}/docs/builtin-macros/serialize) 
| `toJSON(): Record<string, unknown>` 
| JSON serialization with type handling 

| [`Deserialize`]({base}/docs/builtin-macros/deserialize) 
| `static fromJSON(data: unknown): T` 
| JSON deserialization with validation

## Using Built-in Macros

Built-in macros don't require imports. Just use them with `@derive`:

```typescript
/** @derive(Debug, Clone, PartialEq) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}
```

## Interface Support

All built-in macros work with interfaces. For interfaces, methods are generated as functions in a namespace with the same name, using `self` as the first parameter:

```typescript
/** @derive(Debug, Clone, PartialEq) */
interface Point {
  x: number;
  y: number;
}

// Generated namespace:
// namespace Point {
//   export function toString(self: Point): string { ... }
//   export function clone(self: Point): Point { ... }
//   export function equals(self: Point, other: Point): boolean { ... }
//   export function hashCode(self: Point): number { ... }
// }

const point: Point = { x: 10, y: 20 };

// Use the namespace functions
console.log(Point.toString(point));     // "Point { x: 10, y: 20 }"
const copy = Point.clone(point);        // { x: 10, y: 20 }
console.log(Point.equals(point, copy)); // true
```

## Enum Support

All built-in macros work with enums. For enums, methods are generated as functions in a namespace with the same name:

```typescript
/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}

// Generated namespace:
// namespace Status {
//   export function toString(value: Status): string { ... }
//   export function clone(value: Status): Status { ... }
//   export function equals(a: Status, b: Status): boolean { ... }
//   export function hashCode(value: Status): number { ... }
//   export function toJSON(value: Status): string | number { ... }
//   export function fromJSON(data: unknown): Status { ... }
// }

// Use the namespace functions
console.log(Status.toString(Status.Active));     // "Status.Active"
console.log(Status.equals(Status.Active, Status.Active)); // true
const json = Status.toJSON(Status.Pending);      // "pending"
const parsed = Status.fromJSON("active");        // Status.Active
```

## Type Alias Support

All built-in macros work with type aliases. For object type aliases, field-aware methods are generated in a namespace:

```typescript
/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
type Point = {
  x: number;
  y: number;
};

// Generated namespace:
// namespace Point {
//   export function toString(value: Point): string { ... }
//   export function clone(value: Point): Point { ... }
//   export function equals(a: Point, b: Point): boolean { ... }
//   export function hashCode(value: Point): number { ... }
//   export function toJSON(value: Point): Record<string, unknown> { ... }
//   export function fromJSON(data: unknown): Point { ... }
// }

const point: Point = { x: 10, y: 20 };
console.log(Point.toString(point));     // "Point { x: 10, y: 20 }"
const copy = Point.clone(point);        // { x: 10, y: 20 }
console.log(Point.equals(point, copy)); // true
```

Union type aliases also work, using JSON-based implementations:

```typescript
/** @derive(Debug, PartialEq) */
type ApiStatus = "loading" | "success" | "error";

const status: ApiStatus = "success";
console.log(ApiStatus.toString(status)); // "ApiStatus(\\"success\\")"
console.log(ApiStatus.equals("success", "success")); // true
```

## Combining Macros

All macros can be used together. They don't conflict and each generates independent methods:

```typescript
const user = new User("Alice", 30);

// Debug
console.log(user.toString());
// "User { name: Alice, age: 30 }"

// Clone
const copy = user.clone();
console.log(copy.name); // "Alice"

// Eq
console.log(user.equals(copy)); // true
```

## Detailed Documentation

Each macro has its own options and behaviors:

- [**Debug**]({base}/docs/builtin-macros/debug) - Customizable field renaming and skipping

- [**Clone**]({base}/docs/builtin-macros/clone) - Deep copying for all field types

- [**Default**]({base}/docs/builtin-macros/default) - Default value generation with field attributes

- [**Hash**]({base}/docs/builtin-macros/hash) - Hash code generation for use in maps and sets

- [**PartialEq**]({base}/docs/builtin-macros/partial-eq) - Value-based equality comparison

- [**Ord**]({base}/docs/builtin-macros/ord) - Total ordering for sorting

- [**PartialOrd**]({base}/docs/builtin-macros/partial-ord) - Partial ordering comparison

- [**Serialize**]({base}/docs/builtin-macros/serialize) - JSON serialization with serde-style options

- [**Deserialize**]({base}/docs/builtin-macros/deserialize) - JSON deserialization with validation