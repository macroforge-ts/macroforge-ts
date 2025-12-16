# Your First Macro
 *Let's create a class that uses Macroforge's derive macros to automatically generate useful methods.*
 ## Creating a Class with Derive Macros
 Start by creating a simple `User` class. We'll use the `@derive` decorator to automatically generate methods.
 **Before:**
```
/** @derive(Debug, Clone, PartialEq) */
export class User {
    name: string;
    age: number;
    email: string;

    constructor(name: string, age: number, email: string) {
        this.name = name;
        this.age = age;
        this.email = email;
    }
}
```  
**After:**
```
export class User {
    name: string;
    age: number;
    email: string;

    constructor(name: string, age: number, email: string) {
        this.name = name;
        this.age = age;
        this.email = email;
    }

    static toString(value: User): string {
        return userToString(value);
    }

    static clone(value: User): User {
        return userClone(value);
    }

    static equals(a: User, b: User): boolean {
        return userEquals(a, b);
    }
}

export function userToString(value: User): string {
    const parts: string[] = [];
    parts.push('name: ' + value.name);
    parts.push('age: ' + value.age);
    parts.push('email: ' + value.email);
    return 'User { ' + parts.join(', ') + ' }';
}

export function userClone(value: User): User {
    const cloned = Object.create(Object.getPrototypeOf(value));
    cloned.name = value.name;
    cloned.age = value.age;
    cloned.email = value.email;
    return cloned;
}

export function userEquals(a: User, b: User): boolean {
    if (a === b) return true;
    return a.name === b.name && a.age === b.age && a.email === b.email;
}
``` ## Using the Generated Methods
 ```
const user = new User("Alice", 30, "alice@example.com");

// Debug: toString()
console.log(user.toString());
// Output: User { name: Alice, age: 30, email: alice@example.com }

// Clone: clone()
const copy = user.clone();
console.log(copy.name); // "Alice"

// Eq: equals()
console.log(user.equals(copy)); // true

const different = new User("Bob", 25, "bob@example.com");
console.log(user.equals(different)); // false
``` ## Customizing Behavior
 You can customize how macros work using field-level decorators. For example, with the Debug macro:
 **Before:**
```
/** @derive(Debug) */
export class User {
    /** @debug({ rename: "userId" }) */
    id: number;

    name: string;

    /** @debug({ skip: true }) */
    password: string;

    constructor(id: number, name: string, password: string) {
        this.id = id;
        this.name = name;
        this.password = password;
    }
}
```  
**After:**
```
export class User {
    id: number;

    name: string;

    password: string;

    constructor(id: number, name: string, password: string) {
        this.id = id;
        this.name = name;
        this.password = password;
    }

    static toString(value: User): string {
        return userToString(value);
    }
}

export function userToString(value: User): string {
    const parts: string[] = [];
    parts.push('userId: ' + value.id);
    parts.push('name: ' + value.name);
    return 'User { ' + parts.join(', ') + ' }';
}
``` ```
const user = new User(42, "Alice", "secret123");
console.log(user.toString());
// Output: User { userId: 42, name: Alice }
// Note: 'id' is renamed to 'userId', 'password' is skipped
```  **Field-level decorators Field-level decorators let you control exactly how each field is handled by the macro. ## Next Steps
 - [Learn how macros work under the hood](../../docs/concepts)
 - [Explore all Debug options](../../docs/builtin-macros/debug)
 - [Create your own custom macros](../../docs/custom-macros)
**