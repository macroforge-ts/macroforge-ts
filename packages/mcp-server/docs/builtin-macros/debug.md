# Debug
  *The `Debug` macro generates a human-readable `toString()` method for TypeScript classes, interfaces, enums, and type aliases.*
 ## Basic Usage
 **Before:**
```
/** @derive(Debug) */
class User {
    name: string;
    age: number;

    constructor(name: string, age: number) {
        this.name = name;
        this.age = age;
    }
}
```  
**After:**
```
class User {
    name: string;
    age: number;

    constructor(name: string, age: number) {
        this.name = name;
        this.age = age;
    }

    toString(): string {
        const parts: string[] = [];
        parts.push('name: ' + this.name);
        parts.push('age: ' + this.age);
        return 'User { ' + parts.join(', ') + ' }';
    }
}
``` ```
const user = new User("Alice", 30);
console.log(user.toString());
// Output: User { name: Alice, age: 30 }
``` ## Field Options
 Use the `@debug` field decorator to customize behavior:
 ### Renaming Fields
 **Before:**
```
/** @derive(Debug) */
class User {
    /** @debug({ rename: "userId" }) */
    id: number;

    name: string;

    constructor(id: number, name: string) {
        this.id = id;
        this.name = name;
    }
}
```  
**After:**
```
class User {
    id: number;

    name: string;

    constructor(id: number, name: string) {
        this.id = id;
        this.name = name;
    }

    toString(): string {
        const parts: string[] = [];
        parts.push('userId: ' + this.id);
        parts.push('name: ' + this.name);
        return 'User { ' + parts.join(', ') + ' }';
    }
}
``` ```
const user = new User(42, "Alice");
console.log(user.toString());
// Output: User { userId: 42, name: Alice }
``` ### Skipping Fields
 Use `skip: true` to exclude sensitive fields from the output:
 **Before:**
```
/** @derive(Debug) */
class User {
    name: string;
    email: string;

    /** @debug({ skip: true }) */
    password: string;

    /** @debug({ skip: true }) */
    authToken: string;

    constructor(name: string, email: string, password: string, authToken: string) {
        this.name = name;
        this.email = email;
        this.password = password;
        this.authToken = authToken;
    }
}
```  
**After:**
```
class User {
    name: string;
    email: string;

    password: string;

    authToken: string;

    constructor(name: string, email: string, password: string, authToken: string) {
        this.name = name;
        this.email = email;
        this.password = password;
        this.authToken = authToken;
    }

    toString(): string {
        const parts: string[] = [];
        parts.push('name: ' + this.name);
        parts.push('email: ' + this.email);
        return 'User { ' + parts.join(', ') + ' }';
    }
}
``` ```
const user = new User("Alice", "alice@example.com", "secret", "tok_xxx");
console.log(user.toString());
// Output: User { name: Alice, email: alice@example.com }
// Note: password and authToken are not included
```  **Security Always skip sensitive fields like passwords, tokens, and API keys to prevent accidental logging. ## Combining Options
 ****Source:**
```
/** @derive(Debug) */
class ApiResponse {
  /** @debug({ rename: "statusCode" }) */
  status: number;

  data: unknown;

  /** @debug({ skip: true }) */
  internalMetadata: Record<string, unknown>;
}
```  ## All Options
 | Option | Type | Description |
| --- | --- | --- |
| `rename` | `string` | Display a different name in the output |
| `skip` | `boolean` | Exclude this field from the output |
 ## Interface Support
 Debug also works with interfaces. For interfaces, a namespace is generated with a `toString` function:
 **Before:**
```
/** @derive(Debug) */
interface Status {
    active: boolean;
    message: string;
}
```  
**After:**
```
interface Status {
    active: boolean;
    message: string;
}

export namespace Status {
    export function toString(self: Status): string {
        const parts: string[] = [];
        parts.push('active: ' + self.active);
        parts.push('message: ' + self.message);
        return 'Status { ' + parts.join(', ') + ' }';
    }
}
``` ```
const status: Status = { active: true, message: "OK" };
console.log(Status.toString(status));
// Output: Status { active: true, message: OK }
``` ## Enum Support
 Debug also works with enums. For enums, a namespace is generated with a `toString` function that displays the enum name and variant:
 **Before:**
```
/** @derive(Debug) */
enum Priority {
    Low = 1,
    Medium = 2,
    High = 3
}
```  
**After:**
```
enum Priority {
    Low = 1,
    Medium = 2,
    High = 3
}

export namespace Priority {
    export function toString(value: Priority): string {
        const key = Priority[value as unknown as keyof typeof Priority];
        if (key !== undefined) {
            return 'Priority.' + key;
        }
        return 'Priority(' + String(value) + ')';
    }
}
``` ```
console.log(Priority.toString(Priority.High));
// Output: Priority.High

console.log(Priority.toString(Priority.Low));
// Output: Priority.Low
``` Works with both numeric and string enums:
 **Source:**
```
/** @derive(Debug) */
enum Status {
  Active = "active",
  Inactive = "inactive",
}
```  ## Type Alias Support
 Debug works with type aliases. For object types, fields are displayed similar to interfaces:
 **Before:**
```
/** @derive(Debug) */
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
    export function toString(value: Point): string {
        const parts: string[] = [];
        parts.push('x: ' + value.x);
        parts.push('y: ' + value.y);
        return 'Point { ' + parts.join(', ') + ' }';
    }
}
``` ```
const point: Point = { x: 10, y: 20 };
console.log(Point.toString(point));
// Output: Point { x: 10, y: 20 }
``` For union types, the value is displayed using JSON.stringify:
 **Source:**
```
/** @derive(Debug) */
type ApiStatus = "loading" | "success" | "error";
```  ```
console.log(ApiStatus.toString("success"));
// Output: ApiStatus("success")
```