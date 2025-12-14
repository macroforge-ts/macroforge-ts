# Clone
  *The `Clone` macro generates a `clone()` method for deep copying objects. This is analogous to Rust's `Clone` trait, providing a way to create independent copies of values.*
 ## Basic Usage
 **Before:**
```
/** @derive(Clone) */
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

    clone(): Point {
        const cloned = Object.create(Object.getPrototypeOf(this));
        cloned.x = this.x;
        cloned.y = this.y;
        return cloned;
    }
}
``` ```
const original = new Point(10, 20);
const copy = original.clone();

console.log(copy.x, copy.y); // 10, 20
console.log(original === copy); // false (different instances)
``` ## How It Works
 The Clone macro:
 1. Creates a new instance of the class
 2. Passes all field values to the constructor
 3. Returns the new instance
 This creates a **shallow clone** - primitive values are copied, but object references remain the same.
 ## With Nested Objects
 **Before:**
```
/** @derive(Clone) */
class User {
    name: string;
    address: { city: string; zip: string };

    constructor(name: string, address: { city: string; zip: string }) {
        this.name = name;
        this.address = address;
    }
}
```  
**After:**
```
class User {
    name: string;
    address: { city: string; zip: string };

    constructor(name: string, address: { city: string; zip: string }) {
        this.name = name;
        this.address = address;
    }

    clone(): User {
        const cloned = Object.create(Object.getPrototypeOf(this));
        cloned.name = this.name;
        cloned.address = this.address;
        return cloned;
    }
}
``` ```
const original = new User("Alice", { city: "NYC", zip: "10001" });
const copy = original.clone();

// The address object is the same reference
console.log(original.address === copy.address); // true

// Modifying the copy's address affects the original
copy.address.city = "LA";
console.log(original.address.city); // "LA"
``` For deep cloning of nested objects, you would need to implement custom clone methods or use a deep clone utility.
 ## Combining with PartialEq
 Clone works well with PartialEq for creating independent copies that compare as equal:
 **Source:**
```
/** @derive(Clone, PartialEq) */
class Point {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}
```  ```
const original = new Point(10, 20);
const copy = original.clone();

console.log(original === copy);       // false (different instances)
console.log(original.equals(copy));   // true (same values)
``` ## Interface Support
 Clone also works with interfaces. For interfaces, a namespace is generated with a `clone` function:
 **Before:**
```
/** @derive(Clone) */
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
    export function clone(self: Point): Point {
        return { x: self.x, y: self.y };
    }
}
``` ```
const original: Point = { x: 10, y: 20 };
const copy = Point.clone(original);

console.log(copy.x, copy.y);        // 10, 20
console.log(original === copy);     // false (different objects)
``` ## Enum Support
 Clone also works with enums. For enums, the clone function simply returns the value as-is, since enum values are primitives and don't need cloning:
 **Before:**
```
/** @derive(Clone) */
enum Status {
    Active = 'active',
    Inactive = 'inactive'
}
```  
**After:**
```
enum Status {
    Active = 'active',
    Inactive = 'inactive'
}

export namespace Status {
    export function clone(value: Status): Status {
        return value;
    }
}
``` ```
const original = Status.Active;
const copy = Status.clone(original);

console.log(copy);               // "active"
console.log(original === copy);  // true (same primitive value)
``` ## Type Alias Support
 Clone works with type aliases. For object types, a shallow copy is created using spread:
 **Before:**
```
/** @derive(Clone) */
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
    export function clone(value: Point): Point {
        return { x: value.x, y: value.y };
    }
}
``` ```
const original: Point = { x: 10, y: 20 };
const copy = Point.clone(original);

console.log(copy.x, copy.y);     // 10, 20
console.log(original === copy);  // false (different objects)
``` For union types, the value is returned as-is (unions of primitives don't need cloning):
 **Source:**
```
/** @derive(Clone) */
type ApiStatus = "loading" | "success" | "error";
```  ```
const status: ApiStatus = "success";
const copy = ApiStatus.clone(status);
console.log(copy); // "success"
```