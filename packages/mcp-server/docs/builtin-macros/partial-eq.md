# PartialEq

*The `PartialEq` macro generates an `equals()` method for value-based equality comparison between objects.*

## Basic Usage

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

```typescript
const p1 = new Point(10, 20);
const p2 = new Point(10, 20);
const p3 = new Point(5, 5);

console.log(p1.equals(p2)); // true (same values)
console.log(p1.equals(p3)); // false (different values)
console.log(p1 === p2);     // false (different references)
```

## How It Works

The PartialEq macro performs field-by-field comparison using these strategies:

- **Primitives** (string, number, boolean, null, undefined) → Strict equality (`===`)

- **Date** → Compares timestamps via `getTime()`

- **Array** → Length check + element-by-element comparison

- **Map** → Size check + entry comparison

- **Set** → Size check + membership verification

- **Objects** → Calls `equals()` if available, otherwise `===`

## Field Options

### @partialEq(skip)

Use `@partialEq(skip)` to exclude a field from equality comparison:

<MacroExample before={data.examples.skip.before} after={data.examples.skip.after} />

```typescript
const user1 = new User(1, "Alice", new Date("2024-01-01"));
const user2 = new User(1, "Alice", new Date("2024-12-01"));

console.log(user1.equals(user2)); // true (createdAt is skipped)
```

## Type Safety

The generated `equals()` method accepts `unknown` and performs runtime type checking:

<InteractiveMacro code={`/** @derive(PartialEq) */
class User {
  name: string;
  constructor(name: string) {
    this.name = name;
  }
}`} />

```typescript
const user = new User("Alice");

console.log(user.equals(new User("Alice"))); // true
console.log(user.equals("Alice")); // false (not a User instance)
```

## With Nested Objects

For objects with nested fields, PartialEq recursively calls `equals()` if available:

<InteractiveMacro code={`/** @derive(PartialEq) */
class Address {
  city: string;
  zip: string;

  constructor(city: string, zip: string) {
    this.city = city;
    this.zip = zip;
  }
}

/** @derive(PartialEq) */
class Person {
  name: string;
  address: Address;

  constructor(name: string, address: Address) {
    this.name = name;
    this.address = address;
  }
}`} />

```typescript
const addr1 = new Address("NYC", "10001");
const addr2 = new Address("NYC", "10001");

const p1 = new Person("Alice", addr1);
const p2 = new Person("Alice", addr2);

console.log(p1.equals(p2)); // true (deep equality via Address.equals)
```

## Interface Support

PartialEq works with interfaces. For interfaces, a namespace is generated with an `equals` function:

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

```typescript
const p1: Point = { x: 10, y: 20 };
const p2: Point = { x: 10, y: 20 };
const p3: Point = { x: 5, y: 5 };

console.log(Point.equals(p1, p2)); // true
console.log(Point.equals(p1, p3)); // false
```

## Enum Support

PartialEq works with enums. For enums, strict equality comparison is used:

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

```typescript
console.log(Status.equals(Status.Active, Status.Active));   // true
console.log(Status.equals(Status.Active, Status.Inactive)); // false
```

## Type Alias Support

PartialEq works with type aliases. For object types, field-by-field comparison is used:

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

```typescript
const p1: Point = { x: 10, y: 20 };
const p2: Point = { x: 10, y: 20 };

console.log(Point.equals(p1, p2)); // true
```

For union types, strict equality is used:

<InteractiveMacro code={`/** @derive(PartialEq) */
type ApiStatus = "loading" | "success" | "error";`} />

```typescript
console.log(ApiStatus.equals("success", "success")); // true
console.log(ApiStatus.equals("success", "error"));   // false
```

## Common Patterns

### Finding Items in Arrays

<InteractiveMacro code={`/** @derive(PartialEq) */
class Product {
  sku: string;
  name: string;

  constructor(sku: string, name: string) {
    this.sku = sku;
    this.name = name;
  }
}`} />

```typescript
const products = [
  new Product("A1", "Widget"),
  new Product("B2", "Gadget"),
  new Product("C3", "Gizmo")
];

const target = new Product("B2", "Gadget");
const found = products.find(p => p.equals(target));

console.log(found); // Product { sku: "B2", name: "Gadget" }
```

### Use with Hash

When using objects as keys in Map-like structures, combine PartialEq with Hash:

<InteractiveMacro code={`/** @derive(PartialEq, Hash) */
class Key {
  id: number;
  type: string;

  constructor(id: number, type: string) {
    this.id = id;
    this.type = type;
  }
}`} />

```typescript
const k1 = new Key(1, "user");
const k2 = new Key(1, "user");

// Equal objects should have equal hash codes
console.log(k1.equals(k2));                   // true
console.log(k1.hashCode() === k2.hashCode()); // true
```