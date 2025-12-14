# Deserialize
  *The `Deserialize` macro generates JSON deserialization methods with **cycle and forward-reference support**, plus comprehensive runtime validation. This enables safe parsing of complex JSON structures including circular references.*
 ## Basic Usage
 **Before:**
```
/** @derive(Deserialize) */
class User {
    name: string;
    age: number;
    createdAt: Date;
}
```  
**After:**
```
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

class User {
    name: string;
    age: number;
    createdAt: Date;

    constructor(props: {
        name: string;
        age: number;
        createdAt: Date;
    }) {
        this.name = props.name;
        this.age = props.age;
        this.createdAt = props.createdAt;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return User.fromObject(raw, opts);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }

    static fromObject(
        obj: unknown,
        opts?: DeserializeOptions
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = User.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'User.fromObject: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return Result.ok(resultOrRef);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }

    static __deserialize(value: any, ctx: DeserializeContext): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'User.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('name' in obj)) {
            errors.push({
                field: 'name',
                message: 'missing required field'
            });
        }
        if (!('age' in obj)) {
            errors.push({
                field: 'age',
                message: 'missing required field'
            });
        }
        if (!('createdAt' in obj)) {
            errors.push({
                field: 'createdAt',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(User.prototype) as User;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_name = obj['name'] as string;
            instance.name = __raw_name;
        }
        {
            const __raw_age = obj['age'] as number;
            instance.age = __raw_age;
        }
        {
            const __raw_createdAt = obj['createdAt'] as Date;
            {
                const __dateVal =
                    typeof __raw_createdAt === 'string'
                        ? new Date(__raw_createdAt)
                        : (__raw_createdAt as Date);
                instance.createdAt = __dateVal;
            }
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static validateFields(partial: Partial<User>): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }
}
``` ```
const json = '{"name":"Alice","age":30,"createdAt":"2024-01-15T10:30:00.000Z"}';
const user = User.fromJSON(JSON.parse(json));

console.log(user.name);                    // "Alice"
console.log(user.age);                     // 30
console.log(user.createdAt instanceof Date); // true
``` ## Runtime Validation
 Deserialize validates the input data and throws descriptive errors:
 **Source:**
```
/** @derive(Deserialize) */
class User {
  name: string;
  email: string;
}
```  ```
// Missing required field
User.fromJSON({ name: "Alice" });
// Error: User.fromJSON: missing required field "email"

// Wrong type
User.fromJSON("not an object");
// Error: User.fromJSON: expected an object, got string

// Array instead of object
User.fromJSON([1, 2, 3]);
// Error: User.fromJSON: expected an object, got array
``` ## Automatic Type Conversion
 Deserialize automatically converts JSON types to their TypeScript equivalents:
 | JSON Type | TypeScript Type | Conversion |
| --- | --- | --- |
| string/number/boolean | `string`/`number`/`boolean` | Direct assignment |
| ISO string | `Date` | `new Date(string)` |
| array | `T[]` | Maps items with auto-detection |
| object | `Map<K, V>` | `new Map(Object.entries())` |
| array | `Set<T>` | `new Set(array)` |
| object | Nested class | Calls `fromJSON()` if available |
 ## Serde Options
 Use the `@serde` decorator to customize deserialization:
 ### Renaming Fields
 **Before:**
```
/** @derive(Deserialize) */
class User {
    /** @serde({ rename: "user_id" }) */
    id: string;

    /** @serde({ rename: "full_name" }) */
    name: string;
}
```  
**After:**
```
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

class User {
    id: string;

    name: string;

    constructor(props: {
        id: string;
        name: string;
    }) {
        this.id = props.id;
        this.name = props.name;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return User.fromObject(raw, opts);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }

    static fromObject(
        obj: unknown,
        opts?: DeserializeOptions
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = User.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'User.fromObject: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return Result.ok(resultOrRef);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }

    static __deserialize(value: any, ctx: DeserializeContext): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'User.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('user_id' in obj)) {
            errors.push({
                field: 'user_id',
                message: 'missing required field'
            });
        }
        if (!('full_name' in obj)) {
            errors.push({
                field: 'full_name',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(User.prototype) as User;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_id = obj['user_id'] as string;
            instance.id = __raw_id;
        }
        {
            const __raw_name = obj['full_name'] as string;
            instance.name = __raw_name;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static validateFields(partial: Partial<User>): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }
}
``` ```
const user = User.fromJSON({ user_id: "123", full_name: "Alice" });
console.log(user.id);   // "123"
console.log(user.name); // "Alice"
``` ### Default Values
 **Before:**
```
/** @derive(Deserialize) */
class Config {
    host: string;

    /** @serde({ default: "3000" }) */
    port: string;

    /** @serde({ default: "false" }) */
    debug: boolean;
}
```  
**After:**
```
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

class Config {
    host: string;

    port: string;

    debug: boolean;

    constructor(props: {
        host: string;
        port?: string;
        debug?: boolean;
    }) {
        this.host = props.host;
        this.port = props.port as string;
        this.debug = props.debug as boolean;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        Config,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return Config.fromObject(raw, opts);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }

    static fromObject(
        obj: unknown,
        opts?: DeserializeOptions
    ): Result<
        Config,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = Config.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'Config.fromObject: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return Result.ok(resultOrRef);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }

    static __deserialize(value: any, ctx: DeserializeContext): Config | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'Config.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('host' in obj)) {
            errors.push({
                field: 'host',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(Config.prototype) as Config;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_host = obj['host'] as string;
            instance.host = __raw_host;
        }
        if ('port' in obj && obj['port'] !== undefined) {
            const __raw_port = obj['port'] as string;
            instance.port = __raw_port;
        } else {
            instance.port = 3000;
        }
        if ('debug' in obj && obj['debug'] !== undefined) {
            const __raw_debug = obj['debug'] as boolean;
            instance.debug = __raw_debug;
        } else {
            instance.debug = false;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof Config>(
        field: K,
        value: Config[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static validateFields(partial: Partial<Config>): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }
}
``` ```
const config = Config.fromJSON({ host: "localhost" });
console.log(config.port);  // "3000"
console.log(config.debug); // false
``` ### Skipping Fields
 **Source:**
```
/** @derive(Deserialize) */
class User {
  name: string;
  email: string;

  /** @serde({ skip: true }) */
  cachedData: unknown;

  /** @serde({ skip_deserializing: true }) */
  computedField: string;
}
```   **skip vs skip_deserializing Use `skip: true` to exclude from both serialization and deserialization.
Use `skip_deserializing: true` to only skip during deserialization. ### Deny Unknown Fields
 ****Source:**
```
/** @derive(Deserialize) */
/** @serde({ deny_unknown_fields: true }) */
class StrictUser {
  name: string;
  email: string;
}
```  ```
// This will throw an error
StrictUser.fromJSON({ name: "Alice", email: "a@b.com", extra: "field" });
// Error: StrictUser.fromJSON: unknown field "extra"
``` ### Flatten Nested Objects
 **Source:**
```
/** @derive(Deserialize) */
class Address {
  city: string;
  zip: string;
}

/** @derive(Deserialize) */
class User {
  name: string;

  /** @serde({ flatten: true }) */
  address: Address;
}
```  ```
// Flat JSON structure
const user = User.fromJSON({
  name: "Alice",
  city: "NYC",
  zip: "10001"
});
console.log(user.address.city); // "NYC"
``` ## All Options
 ### Container Options (on class/interface)
 | Option | Type | Description |
| --- | --- | --- |
| `rename_all` | `string` | Apply naming convention to all fields |
| `deny_unknown_fields` | `boolean` | Throw error if JSON has unknown keys |
 ### Field Options (on properties)
 | Option | Type | Description |
| --- | --- | --- |
| `rename` | `string` | Use a different JSON key |
| `skip` | `boolean` | Exclude from serialization and deserialization |
| `skip_deserializing` | `boolean` | Exclude from deserialization only |
| `default` | `boolean | string` | Use TypeScript default or custom expression if missing |
| `flatten` | `boolean` | Merge nested object fields from parent |
 ## Interface Support
 Deserialize also works with interfaces. For interfaces, a namespace is generated with `is` (type guard) and `fromJSON` functions:
 **Before:**
```
/** @derive(Deserialize) */
interface ApiResponse {
    status: number;
    message: string;
    timestamp: Date;
}
```  
**After:**
```
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

interface ApiResponse {
    status: number;
    message: string;
    timestamp: Date;
}

export namespace ApiResponse {
    export function fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<ApiResponse, Array<{ field: string; message: string }>> {
        try {
            const raw = JSON.parse(json);
            return fromObject(raw, opts);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([{ field: '_root', message }]);
        }
    }
    export function fromObject(
        obj: unknown,
        opts?: DeserializeOptions
    ): Result<ApiResponse, Array<{ field: string; message: string }>> {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = __deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'ApiResponse.fromObject: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return Result.ok(resultOrRef);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([{ field: '_root', message }]);
        }
    }
    export function __deserialize(value: any, ctx: DeserializeContext): ApiResponse | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                { field: '_root', message: 'ApiResponse.__deserialize: expected an object' }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{ field: string; message: string }> = [];
        if (!('status' in obj)) {
            errors.push({ field: 'status', message: 'missing required field' });
        }
        if (!('message' in obj)) {
            errors.push({ field: 'message', message: 'missing required field' });
        }
        if (!('timestamp' in obj)) {
            errors.push({ field: 'timestamp', message: 'missing required field' });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance: any = {};
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_status = obj['status'] as number;
            instance.status = __raw_status;
        }
        {
            const __raw_message = obj['message'] as string;
            instance.message = __raw_message;
        }
        {
            const __raw_timestamp = obj['timestamp'] as Date;
            {
                const __dateVal =
                    typeof __raw_timestamp === 'string'
                        ? new Date(__raw_timestamp)
                        : (__raw_timestamp as Date);
                instance.timestamp = __dateVal;
            }
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance as ApiResponse;
    }
    export function validateField<K extends keyof ApiResponse>(
        field: K,
        value: ApiResponse[K]
    ): Array<{ field: string; message: string }> {
        return [];
    }
    export function validateFields(
        partial: Partial<ApiResponse>
    ): Array<{ field: string; message: string }> {
        return [];
    }
}
``` ```
const json = { status: 200, message: "OK", timestamp: "2024-01-15T10:30:00.000Z" };

// Type guard
if (ApiResponse.is(json)) {
  console.log(json.status); // TypeScript knows this is ApiResponse
}

// Deserialize with validation
const response = ApiResponse.fromJSON(json);
console.log(response.timestamp instanceof Date); // true
``` ## Enum Support
 Deserialize also works with enums. The `fromJSON` function validates that the input matches one of the enum values:
 **Before:**
```
/** @derive(Deserialize) */
enum Status {
    Active = 'active',
    Inactive = 'inactive',
    Pending = 'pending'
}
```  
**After:**
```
import { DeserializeContext } from 'macroforge/serde';

enum Status {
    Active = 'active',
    Inactive = 'inactive',
    Pending = 'pending'
}

export namespace Status {
    export function fromStringifiedJSON(json: string): Status {
        const data = JSON.parse(json);
        return __deserialize(data);
    }
    export function __deserialize(data: unknown): Status {
        for (const key of Object.keys(Status)) {
            const enumValue = Status[key as keyof typeof Status];
            if (enumValue === data) {
                return data as Status;
            }
        }
        throw new Error('Invalid Status value: ' + JSON.stringify(data));
    }
}
``` ```
const status = Status.fromJSON("active");
console.log(status); // Status.Active

// Invalid values throw an error
try {
  Status.fromJSON("invalid");
} catch (e) {
  console.log(e.message); // "Invalid Status value: invalid"
}
``` Works with numeric enums too:
 **Source:**
```
/** @derive(Deserialize) */
enum Priority {
  Low = 1,
  Medium = 2,
  High = 3,
}
```  ```
const priority = Priority.fromJSON(3);
console.log(priority); // Priority.High
``` ## Type Alias Support
 Deserialize works with type aliases. For object types, validation and type conversion is applied:
 **Before:**
```
/** @derive(Deserialize) */
type UserProfile = {
    id: string;
    name: string;
    createdAt: Date;
};
```  
**After:**
```
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

type UserProfile = {
    id: string;
    name: string;
    createdAt: Date;
};

export namespace UserProfile {
    export function fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<UserProfile, Array<{ field: string; message: string }>> {
        try {
            const raw = JSON.parse(json);
            return fromObject(raw, opts);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([{ field: '_root', message }]);
        }
    }
    export function fromObject(
        obj: unknown,
        opts?: DeserializeOptions
    ): Result<UserProfile, Array<{ field: string; message: string }>> {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = __deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'UserProfile.fromObject: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return Result.ok(resultOrRef);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([{ field: '_root', message }]);
        }
    }
    export function __deserialize(value: any, ctx: DeserializeContext): UserProfile | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref) as UserProfile | PendingRef;
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                { field: '_root', message: 'UserProfile.__deserialize: expected an object' }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{ field: string; message: string }> = [];
        if (!('id' in obj)) {
            errors.push({ field: 'id', message: 'missing required field' });
        }
        if (!('name' in obj)) {
            errors.push({ field: 'name', message: 'missing required field' });
        }
        if (!('createdAt' in obj)) {
            errors.push({ field: 'createdAt', message: 'missing required field' });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance: any = {};
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_id = obj['id'] as string;
            instance.id = __raw_id;
        }
        {
            const __raw_name = obj['name'] as string;
            instance.name = __raw_name;
        }
        {
            const __raw_createdAt = obj['createdAt'] as Date;
            {
                const __dateVal =
                    typeof __raw_createdAt === 'string'
                        ? new Date(__raw_createdAt)
                        : (__raw_createdAt as Date);
                instance.createdAt = __dateVal;
            }
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance as UserProfile;
    }
    export function validateField<K extends keyof UserProfile>(
        field: K,
        value: UserProfile[K]
    ): Array<{ field: string; message: string }> {
        return [];
    }
    export function validateFields(
        partial: Partial<UserProfile>
    ): Array<{ field: string; message: string }> {
        return [];
    }
}
``` ```
const json = {
  id: "123",
  name: "Alice",
  createdAt: "2024-01-15T00:00:00.000Z"
};

const profile = UserProfile.fromJSON(json);
console.log(profile.createdAt instanceof Date); // true
``` For union types, basic validation is applied:
 **Source:**
```
/** @derive(Deserialize) */
type ApiStatus = "loading" | "success" | "error";
```  ```
const status = ApiStatus.fromJSON("success");
console.log(status); // "success"
``` ## Combining with Serialize
 Use both Serialize and Deserialize for complete JSON round-trip support:
 **Source:**
```
/** @derive(Serialize, Deserialize) */
/** @serde({ rename_all: "camelCase" }) */
class UserProfile {
  user_name: string;
  created_at: Date;
  is_active: boolean;
}
```  ```
// Create and serialize
const profile = new UserProfile();
profile.user_name = "Alice";
profile.created_at = new Date();
profile.is_active = true;

const json = JSON.stringify(profile);
// {"userName":"Alice","createdAt":"2024-...","isActive":true}

// Deserialize back
const restored = UserProfile.fromJSON(JSON.parse(json));
console.log(restored.user_name);              // "Alice"
console.log(restored.created_at instanceof Date); // true
``` ## Error Handling
 Handle deserialization errors gracefully:
 **Source:**
```
/** @derive(Deserialize) */
class User {
  name: string;
  email: string;
}
```  ```
function parseUser(json: unknown): User | null {
  try {
    return User.fromJSON(json);
  } catch (error) {
    console.error("Failed to parse user:", error.message);
    return null;
  }
}

const user = parseUser({ name: "Alice" });
// Logs: Failed to parse user: User.fromJSON: missing required field "email"
// Returns: null
```