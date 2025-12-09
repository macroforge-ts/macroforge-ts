# Hash

*The `Hash` macro generates a `hashCode()` method that computes a numeric hash value based on field values.*

## Basic Usage

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

```typescript
const p1 = new Point(10, 20);
const p2 = new Point(10, 20);
const p3 = new Point(5, 5);

console.log(p1.hashCode()); // Same hash
console.log(p2.hashCode()); // Same hash (equal values = equal hash)
console.log(p3.hashCode()); // Different hash
```

## Hash Algorithm

The generated hash function uses the following algorithm for different types:

- `number` → Integers use bitwise OR (`| 0`), floats are stringified and hashed

- `string` → Character-by-character hash: `(h * 31 + charCode) | 0`

- `boolean` → `1231` for true, `1237` for false (Java convention)

- `bigint` → Converted to string and hashed character-by-character

- `Date` → Uses `getTime() | 0` for timestamp hash

- `Array` → Combines element hashes with `h * 31 + elementHash`

- `Map/Set` → Combines all entry hashes

- `Object` → Calls `hashCode()` if available, otherwise JSON stringifies and hashes

- `null` → Returns 0

- `undefined` → Returns 1

## Field Options

### @hash(skip)

Use `@hash(skip)` to exclude a field from hash computation:

<MacroExample before={data.examples.skip.before} after={data.examples.skip.after} />

```typescript
const user1 = new User(1, "Alice", new Date("2024-01-01"));
const user2 = new User(1, "Alice", new Date("2024-12-01"));

console.log(user1.hashCode() === user2.hashCode()); // true (lastLogin is skipped)
```

## Use with PartialEq

Hash is often used together with PartialEq. Objects that are equal should have the same hash code:

<InteractiveMacro code={`/** @derive(Hash, PartialEq) */
class Product {
  sku: string;
  name: string;

  constructor(sku: string, name: string) {
    this.sku = sku;
    this.name = name;
  }
}`} />

```typescript
const p1 = new Product("ABC123", "Widget");
const p2 = new Product("ABC123", "Widget");

// Equal objects have equal hash codes
console.log(p1.equals(p2));                       // true
console.log(p1.hashCode() === p2.hashCode());     // true
```

## Interface Support

Hash also works with interfaces. For interfaces, a namespace is generated with a `hashCode` function:

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

```typescript
const p: Point = { x: 10, y: 20 };
console.log(Point.hashCode(p)); // numeric hash value
```

## Enum Support

Hash works with enums. For string enums, it hashes the string value; for numeric enums, it uses the numeric value directly:

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

```typescript
console.log(Status.hashCode(Status.Active));   // consistent hash
console.log(Status.hashCode(Status.Inactive)); // different hash
```

## Type Alias Support

Hash works with type aliases. For object types, it hashes each field:

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

```typescript
const loc: Coordinates = { lat: 40.7128, lng: -74.0060 };
console.log(Coordinates.hashCode(loc));
```

For union types, it uses JSON stringification as a fallback:

<InteractiveMacro code={`/** @derive(Hash) */
type Result = "success" | "error" | "pending";`} />

```typescript
console.log(Result.hashCode("success")); // hash of "success" string
console.log(Result.hashCode("error"));   // hash of "error" string
```