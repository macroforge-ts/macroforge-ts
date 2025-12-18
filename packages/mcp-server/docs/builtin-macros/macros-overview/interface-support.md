## Interface Support

All built-in macros work with interfaces. For interfaces, methods are generated as functions in a namespace with the same name, using `self` as the first parameter:

TypeScript

```
/** @derive(Debug, Clone, PartialEq) */
interface Point {
  x: number;
  y: number;
}

// Generated namespace:
// namespace Point {
//   export function toString(self: Point): string { ... }
//   export function clone(self: Point): Point { ... }
//   export function equals(self: Point, other: Point): boolean { ... }
//   export function hashCode(self: Point): number { ... }
// }

const point: Point = { x: 10, y: 20 };

// Use the namespace functions
console.log(Point.toString(point));     // "Point { x: 10, y: 20 }"
const copy = Point.clone(point);        // { x: 10, y: 20 }
console.log(Point.equals(point, copy)); // true
```