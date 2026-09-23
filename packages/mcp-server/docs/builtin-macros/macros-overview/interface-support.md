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
