## Type Alias Support

All built-in macros work with type aliases. For object type aliases, field-aware methods are generated in a namespace:

TypeScript

```
/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
type Point = {
  x: number;
  y: number;
};

// Generated namespace:
// namespace Point {
//   export function toString(value: Point): string { ... }
//   export function clone(value: Point): Point { ... }
//   export function equals(a: Point, b: Point): boolean { ... }
//   export function hashCode(value: Point): number { ... }
//   export function toJSON(value: Point): Record<string, unknown> { ... }
//   export function fromJSON(data: unknown): Point { ... }
// }

const point: Point = { x: 10, y: 20 };
console.log(Point.toString(point));     // "Point { x: 10, y: 20 }"
const copy = Point.clone(point);        // { x: 10, y: 20 }
console.log(Point.equals(point, copy)); // true
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
console.log(user.toString());
// "User { name: Alice, age: 30 }"

// Clone
const copy = user.clone();
console.log(copy.name); // "Alice"

// Eq
console.log(user.equals(copy)); // true
```