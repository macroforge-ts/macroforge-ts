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
