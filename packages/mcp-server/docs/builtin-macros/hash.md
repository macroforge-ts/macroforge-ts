# Hash
  *The `Hash` macro generates a `hashCode()` method for computing numeric hash codes. This is analogous to Rust's `Hash` trait and Java's `hashCode()` method, enabling objects to be used as keys in hash-based collections.*
 ## Basic Usage
 **Before:**
```
/** @derive(Hash) */
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

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.x)
                    ? this.x | 0
                    : this.x
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.y)
                    ? this.y | 0
                    : this.y
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        return hash;
    }
}
``` ```
const p1 = new Point(10, 20);
const p2 = new Point(10, 20);
const p3 = new Point(5, 5);

console.log(p1.hashCode()); // Same hash
console.log(p2.hashCode()); // Same hash (equal values = equal hash)
console.log(p3.hashCode()); // Different hash
``` ## Hash Algorithm
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
 **Before:**
```
/** @derive(Hash) */
class User {
    id: number;
    name: string;

    /** @hash(skip) */
    lastLogin: Date;

    constructor(id: number, name: string, lastLogin: Date) {
        this.id = id;
        this.name = name;
        this.lastLogin = lastLogin;
    }
}
```  
**After:**
```
class User {
    id: number;
    name: string;

    lastLogin: Date;

    constructor(id: number, name: string, lastLogin: Date) {
        this.id = id;
        this.name = name;
        this.lastLogin = lastLogin;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        return hash;
    }
}
``` ```
const user1 = new User(1, "Alice", new Date("2024-01-01"));
const user2 = new User(1, "Alice", new Date("2024-12-01"));

console.log(user1.hashCode() === user2.hashCode()); // true (lastLogin is skipped)
``` ## Use with PartialEq
 Hash is often used together with PartialEq. Objects that are equal should have the same hash code:
 **Source:**
```
/** @derive(Hash, PartialEq) */
class Product {
  sku: string;
  name: string;

  constructor(sku: string, name: string) {
    this.sku = sku;
    this.name = name;
  }
}
```  ```
const p1 = new Product("ABC123", "Widget");
const p2 = new Product("ABC123", "Widget");

// Equal objects have equal hash codes
console.log(p1.equals(p2));                       // true
console.log(p1.hashCode() === p2.hashCode());     // true
``` ## Interface Support
 Hash also works with interfaces. For interfaces, a namespace is generated with a `hashCode` function:
 **Before:**
```
/** @derive(Hash) */
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
    export function hashCode(self: Point): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(self.x)
                    ? self.x | 0
                    : self.x
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(self.y)
                    ? self.y | 0
                    : self.y
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        return hash;
    }
}
``` ```
const p: Point = { x: 10, y: 20 };
console.log(Point.hashCode(p)); // numeric hash value
``` ## Enum Support
 Hash works with enums. For string enums, it hashes the string value; for numeric enums, it uses the numeric value directly:
 **Before:**
```
/** @derive(Hash) */
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
    export function hashCode(value: Status): number {
        if (typeof value === 'string') {
            let hash = 0;
            for (let i = 0; i < value.length; i++) {
                hash = (hash * 31 + value.charCodeAt(i)) | 0;
            }
            return hash;
        }
        return value as number;
    }
}
``` ```
console.log(Status.hashCode(Status.Active));   // consistent hash
console.log(Status.hashCode(Status.Inactive)); // different hash
``` ## Type Alias Support
 Hash works with type aliases. For object types, it hashes each field:
 **Before:**
```
/** @derive(Hash) */
type Coordinates = {
    lat: number;
    lng: number;
};
```  
**After:**
```
type Coordinates = {
    lat: number;
    lng: number;
};

export namespace Coordinates {
    export function hashCode(value: Coordinates): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(value.lat)
                    ? value.lat | 0
                    : value.lat
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(value.lng)
                    ? value.lng | 0
                    : value.lng
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        return hash;
    }
}
``` ```
const loc: Coordinates = { lat: 40.7128, lng: -74.0060 };
console.log(Coordinates.hashCode(loc));
``` For union types, it uses JSON stringification as a fallback:
 **Source:**
```
/** @derive(Hash) */
type Result = "success" | "error" | "pending";
```  ```
console.log(Result.hashCode("success")); // hash of "success" string
console.log(Result.hashCode("error"));   // hash of "error" string
```