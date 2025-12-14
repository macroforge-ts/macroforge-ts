# PartialEq
  *The `PartialEq` macro generates an `equals()` method for field-by-field structural equality comparison. This is analogous to Rust's `PartialEq` trait, enabling value-based equality semantics instead of reference equality.*
 ## Basic Usage
 **Before:**
```
/** @derive(PartialEq) */
class Point {
    x: number;
    y: number;

    constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
    }
}
```  
**After:**
```
class Point {
    x: number;
    y: number;

    constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
    }

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof Point)) return false;
        const typedOther = other as Point;
        return this.x === typedOther.x && this.y === typedOther.y;
    }
}
``` ```
const p1 = new Point(10, 20);
const p2 = new Point(10, 20);
const p3 = new Point(5, 5);

console.log(p1.equals(p2)); // true (same values)
console.log(p1.equals(p3)); // false (different values)
console.log(p1 === p2);     // false (different references)
``` ## How It Works
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
 **Before:**
```
/** @derive(PartialEq) */
class User {
    id: number;
    name: string;

    /** @partialEq(skip) */
    createdAt: Date;

    constructor(id: number, name: string, createdAt: Date) {
        this.id = id;
        this.name = name;
        this.createdAt = createdAt;
    }
}
```  
**After:**
```
class User {
    id: number;
    name: string;

    createdAt: Date;

    constructor(id: number, name: string, createdAt: Date) {
        this.id = id;
        this.name = name;
        this.createdAt = createdAt;
    }

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }
}
``` ```
const user1 = new User(1, "Alice", new Date("2024-01-01"));
const user2 = new User(1, "Alice", new Date("2024-12-01"));

console.log(user1.equals(user2)); // true (createdAt is skipped)
``` ## Type Safety
 The generated `equals()` method accepts `unknown` and performs runtime type checking:
 **Source:**
```
/** @derive(PartialEq) */
class User {
  name: string;
  constructor(name: string) {
    this.name = name;
  }
}
```  ```
const user = new User("Alice");

console.log(user.equals(new User("Alice"))); // true
console.log(user.equals("Alice")); // false (not a User instance)
``` ## With Nested Objects
 For objects with nested fields, PartialEq recursively calls `equals()` if available:
 **Source:**
```
/** @derive(PartialEq) */
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
}
```  ```
const addr1 = new Address("NYC", "10001");
const addr2 = new Address("NYC", "10001");

const p1 = new Person("Alice", addr1);
const p2 = new Person("Alice", addr2);

console.log(p1.equals(p2)); // true (deep equality via Address.equals)
``` ## Interface Support
 PartialEq works with interfaces. For interfaces, a namespace is generated with an `equals` function:
 **Before:**
```
/** @derive(PartialEq) */
interface Point {
    x: number;
    y: number;
}
```  
**After:**
```
interface Point {
    x: number;
    y: number;
}

export namespace Point {
    export function equals(self: Point, other: Point): boolean {
        if (self === other) return true;
        return self.x === other.x && self.y === other.y;
    }
}
``` ```
const p1: Point = { x: 10, y: 20 };
const p2: Point = { x: 10, y: 20 };
const p3: Point = { x: 5, y: 5 };

console.log(Point.equals(p1, p2)); // true
console.log(Point.equals(p1, p3)); // false
``` ## Enum Support
 PartialEq works with enums. For enums, strict equality comparison is used:
 **Before:**
```
/** @derive(PartialEq) */
enum Status {
    Active = 'active',
    Inactive = 'inactive',
    Pending = 'pending'
}
```  
**After:**
```
enum Status {
    Active = 'active',
    Inactive = 'inactive',
    Pending = 'pending'
}

export namespace Status {
    export function equals(a: Status, b: Status): boolean {
        return a === b;
    }
}
``` ```
console.log(Status.equals(Status.Active, Status.Active));   // true
console.log(Status.equals(Status.Active, Status.Inactive)); // false
``` ## Type Alias Support
 PartialEq works with type aliases. For object types, field-by-field comparison is used:
 **Before:**
```
/** @derive(PartialEq) */
type Point = {
    x: number;
    y: number;
};
```  
**After:**
```
type Point = {
    x: number;
    y: number;
};

export namespace Point {
    export function equals(a: Point, b: Point): boolean {
        if (a === b) return true;
        return a.x === b.x && a.y === b.y;
    }
}
``` ```
const p1: Point = { x: 10, y: 20 };
const p2: Point = { x: 10, y: 20 };

console.log(Point.equals(p1, p2)); // true
``` For union types, strict equality is used:
 **Source:**
```
/** @derive(PartialEq) */
type ApiStatus = "loading" | "success" | "error";
```  ```
console.log(ApiStatus.equals("success", "success")); // true
console.log(ApiStatus.equals("success", "error"));   // false
``` ## Common Patterns
 ### Finding Items in Arrays
 **Source:**
```
/** @derive(PartialEq) */
class Product {
  sku: string;
  name: string;

  constructor(sku: string, name: string) {
    this.sku = sku;
    this.name = name;
  }
}
```  ```
const products = [
  new Product("A1", "Widget"),
  new Product("B2", "Gadget"),
  new Product("C3", "Gizmo")
];

const target = new Product("B2", "Gadget");
const found = products.find(p => p.equals(target));

console.log(found); // Product { sku: "B2", name: "Gadget" }
``` ### Use with Hash
 When using objects as keys in Map-like structures, combine PartialEq with Hash:
 **Source:**
```
/** @derive(PartialEq, Hash) */
class Key {
  id: number;
  type: string;

  constructor(id: number, type: string) {
    this.id = id;
    this.type = type;
  }
}
```  ```
const k1 = new Key(1, "user");
const k2 = new Key(1, "user");

// Equal objects should have equal hash codes
console.log(k1.equals(k2));                   // true
console.log(k1.hashCode() === k2.hashCode()); // true
```