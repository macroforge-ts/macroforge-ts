# Serialize
  *The `Serialize` macro generates JSON serialization methods with **cycle detection** and object identity tracking. This enables serialization of complex object graphs including circular references.*
 ## Basic Usage
 **Before:**
```
/** @derive(Serialize) */
class User {
    name: string;
    age: number;
    createdAt: Date;

    constructor(name: string, age: number) {
        this.name = name;
        this.age = age;
        this.createdAt = new Date();
    }
}
```  
**After:**
```
import { SerializeContext } from 'macroforge/serde';

class User {
    name: string;
    age: number;
    createdAt: Date;

    constructor(name: string, age: number) {
        this.name = name;
        this.age = age;
        this.createdAt = new Date();
    }

    toStringifiedJSON(): string {
        const ctx = SerializeContext.create();
        return JSON.stringify(this.__serialize(ctx));
    }

    toObject(): Record<string, unknown> {
        const ctx = SerializeContext.create();
        return this.__serialize(ctx);
    }

    __serialize(ctx: SerializeContext): Record<string, unknown> {
        const existingId = ctx.getId(this);
        if (existingId !== undefined) {
            return {
                __ref: existingId
            };
        }
        const __id = ctx.register(this);
        const result: Record<string, unknown> = {
            __type: 'User',
            __id
        };
        result['name'] = this.name;
        result['age'] = this.age;
        result['createdAt'] = this.createdAt.toISOString();
        return result;
    }
}
``` ```
const user = new User("Alice", 30);
console.log(JSON.stringify(user));
// {"name":"Alice","age":30,"createdAt":"2024-01-15T10:30:00.000Z"}
``` ## Automatic Type Handling
 Serialize automatically handles various TypeScript types:
 | Type | Serialization |
| --- | --- |
| `string`, `number`, `boolean` | Direct copy |
| `Date` | `.toISOString()` |
| `T[]` | Maps items, calling `toJSON()` if available |
| `Map<K, V>` | `Object.fromEntries()` |
| `Set<T>` | `Array.from()` |
| Nested objects | Calls `toJSON()` if available |
 ## Serde Options
 Use the `@serde` decorator for fine-grained control over serialization:
 ### Renaming Fields
 **Before:**
```
/** @derive(Serialize) */
class User {
    /** @serde({ rename: "user_id" }) */
    id: string;

    /** @serde({ rename: "full_name" }) */
    name: string;
}
```  
**After:**
```
import { SerializeContext } from 'macroforge/serde';

class User {
    id: string;

    name: string;

    toStringifiedJSON(): string {
        const ctx = SerializeContext.create();
        return JSON.stringify(this.__serialize(ctx));
    }

    toObject(): Record<string, unknown> {
        const ctx = SerializeContext.create();
        return this.__serialize(ctx);
    }

    __serialize(ctx: SerializeContext): Record<string, unknown> {
        const existingId = ctx.getId(this);
        if (existingId !== undefined) {
            return {
                __ref: existingId
            };
        }
        const __id = ctx.register(this);
        const result: Record<string, unknown> = {
            __type: 'User',
            __id
        };
        result['user_id'] = this.id;
        result['full_name'] = this.name;
        return result;
    }
}
``` ```
const user = new User();
user.id = "123";
user.name = "Alice";
console.log(JSON.stringify(user));
// {"user_id":"123","full_name":"Alice"}
``` ### Skipping Fields
 **Before:**
```
/** @derive(Serialize) */
class User {
    name: string;
    email: string;

    /** @serde({ skip: true }) */
    password: string;

    /** @serde({ skip_serializing: true }) */
    internalId: string;
}
```  
**After:**
```
import { SerializeContext } from 'macroforge/serde';

class User {
    name: string;
    email: string;

    password: string;

    internalId: string;

    toStringifiedJSON(): string {
        const ctx = SerializeContext.create();
        return JSON.stringify(this.__serialize(ctx));
    }

    toObject(): Record<string, unknown> {
        const ctx = SerializeContext.create();
        return this.__serialize(ctx);
    }

    __serialize(ctx: SerializeContext): Record<string, unknown> {
        const existingId = ctx.getId(this);
        if (existingId !== undefined) {
            return {
                __ref: existingId
            };
        }
        const __id = ctx.register(this);
        const result: Record<string, unknown> = {
            __type: 'User',
            __id
        };
        result['name'] = this.name;
        result['email'] = this.email;
        return result;
    }
}
```  **skip vs skip_serializing Use `skip: true` to exclude from both serialization and deserialization.
Use `skip_serializing: true` to only skip during serialization. ### Rename All Fields
 Apply a naming convention to all fields at the container level:
 ****Source:**
```
/** @derive(Serialize) */
/** @serde({ rename_all: "camelCase" }) */
class ApiResponse {
  user_name: string;
  created_at: Date;
  is_active: boolean;
}
```  Supported conventions:
 - `camelCase`
 - `snake_case`
 - `PascalCase`
 - `SCREAMING_SNAKE_CASE`
 - `kebab-case`
 ### Flattening Nested Objects
 **Source:**
```
/** @derive(Serialize) */
class Address {
  city: string;
  zip: string;
}

/** @derive(Serialize) */
class User {
  name: string;

  /** @serde({ flatten: true }) */
  address: Address;
}
```  ```
const user = new User();
user.name = "Alice";
user.address = { city: "NYC", zip: "10001" };
console.log(JSON.stringify(user));
// {"name":"Alice","city":"NYC","zip":"10001"}
``` ## All Options
 ### Container Options (on class/interface)
 | Option | Type | Description |
| --- | --- | --- |
| `rename_all` | `string` | Apply naming convention to all fields |
 ### Field Options (on properties)
 | Option | Type | Description |
| --- | --- | --- |
| `rename` | `string` | Use a different JSON key |
| `skip` | `boolean` | Exclude from serialization and deserialization |
| `skip_serializing` | `boolean` | Exclude from serialization only |
| `flatten` | `boolean` | Merge nested object fields into parent |
 ## Interface Support
 Serialize also works with interfaces. For interfaces, a namespace is generated with a `toJSON` function:
 **Before:**
```
/** @derive(Serialize) */
interface ApiResponse {
    status: number;
    message: string;
    timestamp: Date;
}
```  
**After:**
```
import { SerializeContext } from 'macroforge/serde';

interface ApiResponse {
    status: number;
    message: string;
    timestamp: Date;
}

export namespace ApiResponse {
    export function toStringifiedJSON(self: ApiResponse): string {
        const ctx = SerializeContext.create();
        return JSON.stringify(__serialize(self, ctx));
    }
    export function toObject(self: ApiResponse): Record<string, unknown> {
        const ctx = SerializeContext.create();
        return __serialize(self, ctx);
    }
    export function __serialize(self: ApiResponse, ctx: SerializeContext): Record<string, unknown> {
        const existingId = ctx.getId(self);
        if (existingId !== undefined) {
            return { __ref: existingId };
        }
        const __id = ctx.register(self);
        const result: Record<string, unknown> = { __type: 'ApiResponse', __id };
        result['status'] = self.status;
        result['message'] = self.message;
        result['timestamp'] = self.timestamp.toISOString();
        return result;
    }
}
``` ```
const response: ApiResponse = {
  status: 200,
  message: "OK",
  timestamp: new Date()
};

console.log(JSON.stringify(ApiResponse.toJSON(response)));
// {"status":200,"message":"OK","timestamp":"2024-01-15T10:30:00.000Z"}
``` ## Enum Support
 Serialize also works with enums. The `toJSON` function returns the underlying enum value (string or number):
 **Before:**
```
/** @derive(Serialize) */
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
    export function toStringifiedJSON(value: Status): string {
        return JSON.stringify(value);
    }
    export function __serialize(_ctx: SerializeContext): string | number {
        return value;
    }
}
``` ```
console.log(Status.toJSON(Status.Active));  // "active"
console.log(Status.toJSON(Status.Pending)); // "pending"
``` Works with numeric enums too:
 **Source:**
```
/** @derive(Serialize) */
enum Priority {
  Low = 1,
  Medium = 2,
  High = 3,
}
```  ```
console.log(Priority.toJSON(Priority.High)); // 3
``` ## Type Alias Support
 Serialize works with type aliases. For object types, fields are serialized with full type handling:
 **Before:**
```
/** @derive(Serialize) */
type UserProfile = {
    id: string;
    name: string;
    createdAt: Date;
};
```  
**After:**
```
import { SerializeContext } from 'macroforge/serde';

type UserProfile = {
    id: string;
    name: string;
    createdAt: Date;
};

export namespace UserProfile {
    export function toStringifiedJSON(value: UserProfile): string {
        const ctx = SerializeContext.create();
        return JSON.stringify(__serialize(value, ctx));
    }
    export function toObject(value: UserProfile): Record<string, unknown> {
        const ctx = SerializeContext.create();
        return __serialize(value, ctx);
    }
    export function __serialize(
        value: UserProfile,
        ctx: SerializeContext
    ): Record<string, unknown> {
        const existingId = ctx.getId(value);
        if (existingId !== undefined) {
            return { __ref: existingId };
        }
        const __id = ctx.register(value);
        const result: Record<string, unknown> = { __type: 'UserProfile', __id };
        result['id'] = value.id;
        result['name'] = value.name;
        result['createdAt'] = value.createdAt;
        return result;
    }
}
``` ```
const profile: UserProfile = {
  id: "123",
  name: "Alice",
  createdAt: new Date("2024-01-15")
};

console.log(JSON.stringify(UserProfile.toJSON(profile)));
// {"id":"123","name":"Alice","createdAt":"2024-01-15T00:00:00.000Z"}
``` For union types, the value is returned directly:
 **Source:**
```
/** @derive(Serialize) */
type ApiStatus = "loading" | "success" | "error";
```  ```
console.log(ApiStatus.toJSON("success")); // "success"
``` ## Combining with Deserialize
 Use both Serialize and Deserialize for complete JSON round-trip support:
 **Source:**
```
/** @derive(Serialize, Deserialize) */
class User {
  name: string;
  createdAt: Date;
}
```  ```
// Serialize
const user = new User();
user.name = "Alice";
user.createdAt = new Date();
const json = JSON.stringify(user);

// Deserialize
const parsed = User.fromJSON(JSON.parse(json));
console.log(parsed.createdAt instanceof Date); // true
```