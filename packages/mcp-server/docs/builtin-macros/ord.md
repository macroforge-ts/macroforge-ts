# Ord

*The `Ord` macro generates a `compareTo()` method that implements total ordering, always returning `-1`, `0`, or `1`.*

## Basic Usage

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

```typescript
const v1 = new Version(1, 0, 0);
const v2 = new Version(1, 2, 0);
const v3 = new Version(1, 2, 0);

console.log(v1.compareTo(v2)); // -1 (v1 < v2)
console.log(v2.compareTo(v1)); // 1  (v2 > v1)
console.log(v2.compareTo(v3)); // 0  (v2 == v3)
```

## Comparison Logic

The Ord macro compares fields in declaration order (lexicographic ordering). For each type:

- `number` / `bigint` → Direct numeric comparison

- `string` → Uses `localeCompare()` clamped to -1/0/1

- `boolean` → `false < true`

- `Date` → Compares timestamps via `getTime()`

- `Array` → Lexicographic: compares element-by-element, then length

- `Map/Set` → Size and content comparison

- `Object` → Calls `compareTo()` if available, otherwise 0

- `null/undefined` → Treated as equal (returns 0)

## Return Values

The `compareTo()` method always returns:

- `-1` → `this` is less than `other`

- `0` → `this` equals `other`

- `1` → `this` is greater than `other`

Unlike `PartialOrd`, the `Ord` macro never returns `null` - it provides total ordering.

## Field Options

### @ord(skip)

Use `@ord(skip)` to exclude a field from ordering comparison:

<MacroExample before={data.examples.skip.before} after={data.examples.skip.after} />

```typescript
const t1 = new Task(1, "Bug fix", new Date("2024-01-01"));
const t2 = new Task(1, "Bug fix", new Date("2024-12-01"));

console.log(t1.compareTo(t2)); // 0 (createdAt is skipped)
```

## Sorting Arrays

The generated `compareTo()` method works directly with `Array.sort()`:

<InteractiveMacro code={`/** @derive(Ord) */
class Score {
  points: number;
  name: string;

  constructor(points: number, name: string) {
    this.points = points;
    this.name = name;
  }
}`} />

```typescript
const scores = [
  new Score(100, "Alice"),
  new Score(50, "Bob"),
  new Score(150, "Charlie"),
  new Score(50, "Alice")  // Same points, different name
];

// Sort ascending
scores.sort((a, b) => a.compareTo(b));
// Result: [Bob(50), Alice(50), Alice(100), Charlie(150)]

// Sort descending
scores.sort((a, b) => b.compareTo(a));
// Result: [Charlie(150), Alice(100), Alice(50), Bob(50)]
```

## Interface Support

Ord works with interfaces. For interfaces, a namespace is generated with a `compareTo` function:

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

```typescript
const points: Point[] = [
  { x: 5, y: 10 },
  { x: 1, y: 20 },
  { x: 5, y: 5 }
];

points.sort((a, b) => Point.compareTo(a, b));
// Result: [{ x: 1, y: 20 }, { x: 5, y: 5 }, { x: 5, y: 10 }]
```

## Enum Support

Ord works with enums. For numeric enums, it compares the numeric values; for string enums, it uses string comparison:

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

```typescript
console.log(Priority.compareTo(Priority.Low, Priority.High));      // -1
console.log(Priority.compareTo(Priority.Critical, Priority.Low));  // 1
console.log(Priority.compareTo(Priority.Medium, Priority.Medium)); // 0
```

## Type Alias Support

Ord works with type aliases. For object types, it uses lexicographic field comparison:

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

```typescript
const c1: Coordinate = { x: 10, y: 20 };
const c2: Coordinate = { x: 10, y: 30 };

console.log(Coordinate.compareTo(c1, c2)); // -1 (c1 < c2)
```

## Ord vs PartialOrd

Use `Ord` when all values of a type are comparable. Use `PartialOrd` when some values might be incomparable (e.g., different types at runtime).

<InteractiveMacro code={`// Ord: Total ordering - never returns null
/** @derive(Ord) */
class Version {
  major: number;
  minor: number;
  constructor(major: number, minor: number) {
    this.major = major;
    this.minor = minor;
  }
}`} />

```typescript
const v1 = new Version(1, 0);
const v2 = new Version(2, 0);
console.log(v1.compareTo(v2)); // Always -1, 0, or 1
```