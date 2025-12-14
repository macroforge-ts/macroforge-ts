# Default
  *The `Default` macro generates a static `defaultValue()` factory method that creates instances with default values. This is analogous to Rust's `Default` trait, providing a standard way to create "zero" or "empty" instances of types.*
 ## Basic Usage
 **Before:**
```
/** @derive(Default) */
class Config {
    host: string;
    port: number;
    enabled: boolean;

    constructor(host: string, port: number, enabled: boolean) {
        this.host = host;
        this.port = port;
        this.enabled = enabled;
    }
}
```  
**After:**
```
class Config {
    host: string;
    port: number;
    enabled: boolean;

    constructor(host: string, port: number, enabled: boolean) {
        this.host = host;
        this.port = port;
        this.enabled = enabled;
    }

    static defaultValue(): Config {
        const instance = new Config();
        instance.host = '';
        instance.port = 0;
        instance.enabled = false;
        return instance;
    }
}
``` ```
const config = Config.defaultValue();
console.log(config.host);    // ""
console.log(config.port);    // 0
console.log(config.enabled); // false
``` ## Automatic Default Values
 Like Rust's `Default` trait, the macro automatically determines default values for primitive types and common collections:
 | TypeScript Type | Default Value | Rust Equivalent |
| --- | --- | --- |
| `string` | `""` | `String::default()` |
| `number` | `0` | `i32::default()` |
| `boolean` | `false` | `bool::default()` |
| `bigint` | `0n` | `i64::default()` |
| `T[]` / `Array<T>` | `[]` | `Vec::default()` |
| `Map<K, V>` | `new Map()` | `HashMap::default()` |
| `Set<T>` | `new Set()` | `HashSet::default()` |
| `Date` | `new Date()` | — |
| `T | null` / `T | undefined` | `null` | `Option::default()` |
| Custom types | **Error** | **Error** (needs `impl Default`) |
 ## Nullable Types (like Rust's Option)
 Just like Rust's `Option<T>` defaults to `None`, nullable TypeScript types automatically default to `null`:
 **Before:**
```
/** @derive(Default) */
interface User {
    name: string;
    email: string | null;
    age: number;
    metadata: Record<string, unknown> | null;
}
```  
**After:**
```
interface User {
    name: string;
    email: string | null;
    age: number;
    metadata: Record<string, unknown> | null;
}

export namespace User {
    export function defaultValue(): User {
        return { name: '', email: null, age: 0, metadata: null } as User;
    }
}
``` ```
const user = User.defaultValue();
console.log(user.name);     // ""
console.log(user.email);    // null (nullable type)
console.log(user.age);      // 0
console.log(user.metadata); // null (nullable type)
``` ## Custom Types Require @default
 Just like Rust requires `impl Default` for custom types, Macroforge requires the `@default()` decorator on fields with non-primitive types:
 **Before:**
```
/** @derive(Default) */
interface AppConfig {
    name: string;
    port: number;
    /** @default(Settings.defaultValue()) */
    settings: Settings;
    /** @default(Permissions.defaultValue()) */
    permissions: Permissions;
}
```  
**After:**
```
interface AppConfig {
    name: string;
    port: number;

    settings: Settings;

    permissions: Permissions;
}

export namespace AppConfig {
    export function defaultValue(): AppConfig {
        return {
            name: '',
            port: 0,
            settings: Settings.defaultValue(),
            permissions: Permissions.defaultValue()
        } as AppConfig;
    }
}
``` <p class="text-red-500 text-sm mt-2">Without `@default` on custom type fields, the macro will emit an error: ```
// Error: @derive(Default) cannot determine default for non-primitive fields.
// Add @default(value) to: settings, permissions
``` ## Custom Default Values
 Use the `@default()` decorator to specify custom default values for any field:
 **Before:**
```
/** @derive(Default) */
class ServerConfig {
    /** @default("localhost") */
    host: string;

    /** @default(8080) */
    port: number;

    /** @default(true) */
    enabled: boolean;

    /** @default(["info", "error"]) */
    logLevels: string[];

    constructor(host: string, port: number, enabled: boolean, logLevels: string[]) {
        this.host = host;
        this.port = port;
        this.enabled = enabled;
        this.logLevels = logLevels;
    }
}
```  
**After:**
```
class ServerConfig {
    host: string;

    port: number;

    enabled: boolean;

    logLevels: string[];

    constructor(host: string, port: number, enabled: boolean, logLevels: string[]) {
        this.host = host;
        this.port = port;
        this.enabled = enabled;
        this.logLevels = logLevels;
    }

    static defaultValue(): ServerConfig {
        const instance = new ServerConfig();
        instance.host = 'localhost';
        instance.port = 8080;
        instance.enabled = true;
        instance.logLevels = ['info', 'error'];
        return instance;
    }
}
``` ```
const config = ServerConfig.defaultValue();
console.log(config.host);      // "localhost"
console.log(config.port);      // 8080
console.log(config.enabled);   // true
console.log(config.logLevels); // ["info", "error"]
``` ## Interface Support
 Default also works with interfaces. For interfaces, a namespace is generated with a `defaultValue()` function:
 **Before:**
```
/** @derive(Default) */
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
    export function defaultValue(): Point {
        return { x: 0, y: 0 } as Point;
    }
}
``` ```
const origin = Point.defaultValue();
console.log(origin); // { x: 0, y: 0 }
``` ## Enum Support
 Default works with enums. For enums, it returns the first variant as the default value:
 **Before:**
```
/** @derive(Default) */
enum Status {
    Pending = 'pending',
    Active = 'active',
    Completed = 'completed'
}
```  
**After:**
```
enum Status {
    Pending = 'pending',
    Active = 'active',
    Completed = 'completed'
}

export namespace Status {
    export function defaultValue(): Status {
        return Status.Pending;
    }
}
``` ```
const defaultStatus = Status.defaultValue();
console.log(defaultStatus); // "pending"
``` ## Type Alias Support
 Default works with type aliases. For object types, it creates an object with default field values:
 **Before:**
```
/** @derive(Default) */
type Dimensions = {
    width: number;
    height: number;
};
```  
**After:**
```
type Dimensions = {
    width: number;
    height: number;
};

export namespace Dimensions {
    export function defaultValue(): Dimensions {
        return { width: 0, height: 0 } as Dimensions;
    }
}
``` ```
const dims = Dimensions.defaultValue();
console.log(dims); // { width: 0, height: 0 }
``` ## Combining with Other Macros
 **Source:**
```
/** @derive(Default, Debug, Clone, PartialEq) */
class User {
  /** @default("Anonymous") */
  name: string;

  /** @default(0) */
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}
```  ```
const user1 = User.defaultValue();
const user2 = user1.clone();

console.log(user1.toString());    // User { name: "Anonymous", age: 0 }
console.log(user1.equals(user2)); // true
```