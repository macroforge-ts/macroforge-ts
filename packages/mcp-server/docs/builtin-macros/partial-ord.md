# PartialOrd

*The `PartialOrd` macro generates a `compareTo()` method that implements partial ordering, returning `-1`, `0`, `1`, or `null` for incomparable values.*

## Basic Usage

```typescript
/** @derive(PartialOrd) */
class Temperature {
  celsius: number;

  constructor(celsius: number) {
    this.celsius = celsius;
  }
}

const t1 = new Temperature(20);
const t2 = new Temperature(30);
const t3 = new Temperature(20);

console.log(t1.compareTo(t2)); // -1 (t1 < t2)
console.log(t2.compareTo(t1)); // 1  (t2 > t1)
console.log(t1.compareTo(t3)); // 0  (t1 == t3)

// Returns null for incomparable types
console.log(t1.compareTo("not a Temperature")); // null
```

## Generated Code

The PartialOrd macro generates a compareTo method with runtime type checking:

```typescript
compareTo(other: unknown): number | null {
    if (this === other) return 0;
    if (!(other instanceof Temperature)) return null;
    const typedOther = other as Temperature;
    const cmp0 = (this.celsius < typedOther.celsius ? -1 : this.celsius > typedOther.celsius ? 1 : 0);
    if (cmp0 === null) return null;
    if (cmp0 !== 0) return cmp0;
    return 0;
}
```

## Return Values

The `compareTo()` method returns:

- `-1` → `this` is less than `other`

- `0` → `this` equals `other`

- `1` → `this` is greater than `other`

- `null` → Values are incomparable (e.g., different types)

## Comparison Logic

The PartialOrd macro compares fields in declaration order with type checking:

- `number` / `bigint` → Direct numeric comparison

- `string` → Uses `localeCompare()`

- `boolean` → `false < true`

- `Date` → Compares timestamps; returns `null` if not both Date instances

- `Array` → Lexicographic comparison; returns `null` if not both arrays

- `Object` → Calls `compareTo()` if available

- **Type mismatch** → Returns `null`

## Field Options

### @ord(skip)

Use `@ord(skip)` to exclude a field from ordering comparison:

```typescript
/** @derive(PartialOrd) */
class Item {
  price: number;
  name: string;

  /** @ord(skip) */
  description: string;  // Not used for ordering

  constructor(price: number, name: string, description: string) {
    this.price = price;
    this.name = name;
    this.description = description;
  }
}

const i1 = new Item(10, "Widget", "A useful widget");
const i2 = new Item(10, "Widget", "Different description");

console.log(i1.compareTo(i2)); // 0 (description is skipped)
```

## Handling Null Results

When using PartialOrd, always handle the `null` case:

```typescript
/** @derive(PartialOrd) */
class Value {
  amount: number;

  constructor(amount: number) {
    this.amount = amount;
  }
}

function safeCompare(a: Value, b: unknown): string {
  const result = a.compareTo(b);
  if (result === null) {
    return "incomparable";
  } else if (result < 0) {
    return "less than";
  } else if (result > 0) {
    return "greater than";
  } else {
    return "equal";
  }
}

const v = new Value(100);
console.log(safeCompare(v, new Value(50)));  // "greater than"
console.log(safeCompare(v, "string"));       // "incomparable"
```

## Sorting with PartialOrd

When sorting, handle `null` values appropriately:

```typescript
/** @derive(PartialOrd) */
class Score {
  value: number;

  constructor(value: number) {
    this.value = value;
  }
}

const scores = [
  new Score(100),
  new Score(50),
  new Score(75)
];

// Safe sort that handles null (treats null as equal)
scores.sort((a, b) => a.compareTo(b) ?? 0);
// Result: [Score(50), Score(75), Score(100)]
```

## Interface Support

PartialOrd works with interfaces. For interfaces, a namespace is generated with a `compareTo` function:

```typescript
/** @derive(PartialOrd) */
interface Measurement {
  value: number;
  unit: string;
}

// Generated:
// export namespace Measurement {
//   export function compareTo(self: Measurement, other: Measurement): number | null {
//     if (self === other) return 0;
//     const cmp0 = (self.value < other.value ? -1 : self.value > other.value ? 1 : 0);
//     if (cmp0 !== 0) return cmp0;
//     const cmp1 = self.unit.localeCompare(other.unit);
//     if (cmp1 !== 0) return cmp1 < 0 ? -1 : 1;
//     return 0;
//   }
// }

const m1: Measurement = { value: 10, unit: "kg" };
const m2: Measurement = { value: 10, unit: "lb" };

console.log(Measurement.compareTo(m1, m2)); // 1 (kg > lb alphabetically)
```

## Enum Support

PartialOrd works with enums:

```typescript
/** @derive(PartialOrd) */
enum Size {
  Small = 1,
  Medium = 2,
  Large = 3
}

// Generated:
// export namespace Size {
//   export function compareTo(a: Size, b: Size): number | null {
//     return a < b ? -1 : a > b ? 1 : 0;
//   }
// }

console.log(Size.compareTo(Size.Small, Size.Large)); // -1
console.log(Size.compareTo(Size.Large, Size.Small)); // 1
```

## Type Alias Support

PartialOrd works with type aliases:

```typescript
/** @derive(PartialOrd) */
type Interval = {
  start: number;
  end: number;
};

// Generated:
// export namespace Interval {
//   export function compareTo(a: Interval, b: Interval): number | null {
//     if (a === b) return 0;
//     const cmp0 = (a.start < b.start ? -1 : a.start > b.start ? 1 : 0);
//     if (cmp0 !== 0) return cmp0;
//     const cmp1 = (a.end < b.end ? -1 : a.end > b.end ? 1 : 0);
//     if (cmp1 !== 0) return cmp1;
//     return 0;
//   }
// }

const i1: Interval = { start: 0, end: 10 };
const i2: Interval = { start: 0, end: 20 };

console.log(Interval.compareTo(i1, i2)); // -1
```

## PartialOrd vs Ord

Choose between `Ord` and `PartialOrd` based on your use case:

- **Ord** → Use when all values are always comparable (never returns null)

- **PartialOrd** → Use when comparing with `unknown` types or when some values might be incomparable

```typescript
// PartialOrd is safer for public APIs that accept unknown input
/** @derive(PartialOrd) */
class SafeValue {
  data: number;
  constructor(data: number) {
    this.data = data;
  }

  // Can safely compare with any value
  isGreaterThan(other: unknown): boolean {
    const result = this.compareTo(other);
    return result !== null && result > 0;
  }
}

const safe = new SafeValue(100);
console.log(safe.isGreaterThan(new SafeValue(50)));  // true
console.log(safe.isGreaterThan("invalid"));          // false
```