## Example

```typescript before
/** @derive(Deserialize) @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    /** @serde({ validate: { email: true, maxLength: 255 } }) */
    email: string;

    /** @serde({ default: "guest" }) */
    name: string;

    /** @serde({ validate: { positive: true } }) */
    age?: number;
}
```

```typescript after
class User {
    id: number;

    email: string;

    name: string;

    age?: number;
}
```

Generated output:

```typescript
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

/** @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    email: string;

    name: string;

    age?: number;

    constructor(props: {
        id: number;
        email: string;
        name?: string;
        age?: number;
    }) {
        this.id = props.id;
        this.email = props.email;
        this.name = props.name as string;
        this.age = props.age as number;
    }

    /**
     * Deserializes input to an instance of this class.
     * Automatically detects whether input is a JSON string or object.
     * @param input - JSON string or object to deserialize
     * @param opts - Optional deserialization options
     * @returns Result containing the deserialized instance or validation errors
     */
    static deserialize(
        input: unknown,
        opts?: DeserializeOptions
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === 'string' ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'User.deserialize: root cannot be a forward reference'
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

    /** @internal */
    static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'User.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: 'unknown field'
                });
            }
        }
        if (!('id' in obj)) {
            errors.push({
                field: 'id',
                message: 'missing required field'
            });
        }
        if (!('email' in obj)) {
            errors.push({
                field: 'email',
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
            const __raw_id = obj['id'] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj['email'] as string;
            instance.email = __raw_email;
        }
        if ('name' in obj && obj['name'] !== undefined) {
            const __raw_name = obj['name'] as string;
            instance.name = __raw_name;
        } else {
            instance.name = "guest";
        }
        if ('age' in obj && obj['age'] !== undefined) {
            const __raw_age = obj['age'] as number;
            instance.age = __raw_age;
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

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'id' in o && 'email' in o;
    }

    static is(obj: unknown): obj is User {
        if (obj instanceof User) {
            return true;
        }
        if (!User.hasShape(obj)) {
            return false;
        }
        const result = User.deserialize(obj);
        return Result.isOk(result);
    }
}

// Usage:
const result = User.deserialize('{"id":1,"email":"test@example.com"}');
if (Result.isOk(result)) {
    const user = result.value;
} else {
    console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
}
```

Generated output:

```typescript
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

/** @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    email: string;

    name: string;

    age?: number;

    constructor(props: {
        id: number;
        email: string;
        name?: string;
        age?: number;
    }) {
        this.id = props.id;
        this.email = props.email;
        this.name = props.name as string;
        this.age = props.age as number;
    }

    /**
     * Deserializes input to an instance of this class.
     * Automatically detects whether input is a JSON string or object.
     * @param input - JSON string or object to deserialize
     * @param opts - Optional deserialization options
     * @returns Result containing the deserialized instance or validation errors
     */
    static deserialize(
        input: unknown,
        opts?: DeserializeOptions
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === 'string' ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'User.deserialize: root cannot be a forward reference'
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

    /** @internal */
    static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'User.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: 'unknown field'
                });
            }
        }
        if (!('id' in obj)) {
            errors.push({
                field: 'id',
                message: 'missing required field'
            });
        }
        if (!('email' in obj)) {
            errors.push({
                field: 'email',
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
            const __raw_id = obj['id'] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj['email'] as string;
            instance.email = __raw_email;
        }
        if ('name' in obj && obj['name'] !== undefined) {
            const __raw_name = obj['name'] as string;
            instance.name = __raw_name;
        } else {
            instance.name = 'guest';
        }
        if ('age' in obj && obj['age'] !== undefined) {
            const __raw_age = obj['age'] as number;
            instance.age = __raw_age;
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

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'id' in o && 'email' in o;
    }

    static is(obj: unknown): obj is User {
        if (obj instanceof User) {
            return true;
        }
        if (!User.hasShape(obj)) {
            return false;
        }
        const result = User.deserialize(obj);
        return Result.isOk(result);
    }
}

// Usage:
const result = User.deserialize('{"id":1,"email":"test@example.com"}');
if (Result.isOk(result)) {
    const user = result.value;
} else {
    console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
}
```

Generated output:

```typescript
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

/** @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    email: string;

    name: string;

    age?: number;

    constructor(props: {
        id: number;
        email: string;
        name?: string;
        age?: number;
    }) {
        this.id = props.id;
        this.email = props.email;
        this.name = props.name as string;
        this.age = props.age as number;
    }

    /**
     * Deserializes input to an instance of this class.
     * Automatically detects whether input is a JSON string or object.
     * @param input - JSON string or object to deserialize
     * @param opts - Optional deserialization options
     * @returns Result containing the deserialized instance or validation errors
     */
    static deserialize(
        input: unknown,
        opts?: DeserializeOptions
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === 'string' ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'User.deserialize: root cannot be a forward reference'
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

    /** @internal */
    static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'User.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: 'unknown field'
                });
            }
        }
        if (!('id' in obj)) {
            errors.push({
                field: 'id',
                message: 'missing required field'
            });
        }
        if (!('email' in obj)) {
            errors.push({
                field: 'email',
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
            const __raw_id = obj['id'] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj['email'] as string;
            instance.email = __raw_email;
        }
        if ('name' in obj && obj['name'] !== undefined) {
            const __raw_name = obj['name'] as string;
            instance.name = __raw_name;
        } else {
            instance.name = "guest";
        }
        if ('age' in obj && obj['age'] !== undefined) {
            const __raw_age = obj['age'] as number;
            instance.age = __raw_age;
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

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'id' in o && 'email' in o;
    }

    static is(obj: unknown): obj is User {
        if (obj instanceof User) {
            return true;
        }
        if (!User.hasShape(obj)) {
            return false;
        }
        const result = User.deserialize(obj);
        return Result.isOk(result);
    }
}

// Usage:
const result = User.deserialize('{"id":1,"email":"test@example.com"}');
if (Result.isOk(result)) {
    const user = result.value;
} else {
    console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
}
```

Generated output:

```typescript
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

/** @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    email: string;

    name: string;

    age?: number;

    constructor(props: {
        id: number;
        email: string;
        name?: string;
        age?: number;
    }) {
        this.id = props.id;
        this.email = props.email;
        this.name = props.name as string;
        this.age = props.age as number;
    }

    /**
     * Deserializes input to an instance of this class.
     * Automatically detects whether input is a JSON string or object.
     * @param input - JSON string or object to deserialize
     * @param opts - Optional deserialization options
     * @returns Result containing the deserialized instance or validation errors
     */
    static deserialize(
        input: unknown,
        opts?: DeserializeOptions
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === 'string' ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'User.deserialize: root cannot be a forward reference'
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

    /** @internal */
    static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'User.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: 'unknown field'
                });
            }
        }
        if (!('id' in obj)) {
            errors.push({
                field: 'id',
                message: 'missing required field'
            });
        }
        if (!('email' in obj)) {
            errors.push({
                field: 'email',
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
            const __raw_id = obj['id'] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj['email'] as string;
            instance.email = __raw_email;
        }
        if ('name' in obj && obj['name'] !== undefined) {
            const __raw_name = obj['name'] as string;
            instance.name = __raw_name;
        } else {
            instance.name = 'guest';
        }
        if ('age' in obj && obj['age'] !== undefined) {
            const __raw_age = obj['age'] as number;
            instance.age = __raw_age;
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

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'id' in o && 'email' in o;
    }

    static is(obj: unknown): obj is User {
        if (obj instanceof User) {
            return true;
        }
        if (!User.hasShape(obj)) {
            return false;
        }
        const result = User.deserialize(obj);
        return Result.isOk(result);
    }
}

// Usage:
const result = User.deserialize('{"id":1,"email":"test@example.com"}');
if (Result.isOk(result)) {
    const user = result.value;
} else {
    console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
}
```

Generated output:

```typescript
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

/** @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    email: string;

    name: string;

    age?: number;

    constructor(props: {
        id: number;
        email: string;
        name?: string;
        age?: number;
    }) {
        this.id = props.id;
        this.email = props.email;
        this.name = props.name as string;
        this.age = props.age as number;
    }

    /**
     * Deserializes input to an instance of this class.
     * Automatically detects whether input is a JSON string or object.
     * @param input - JSON string or object to deserialize
     * @param opts - Optional deserialization options
     * @returns Result containing the deserialized instance or validation errors
     */
    static deserialize(
        input: unknown,
        opts?: DeserializeOptions
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === 'string' ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'User.deserialize: root cannot be a forward reference'
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

    /** @internal */
    static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'User.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: 'unknown field'
                });
            }
        }
        if (!('id' in obj)) {
            errors.push({
                field: 'id',
                message: 'missing required field'
            });
        }
        if (!('email' in obj)) {
            errors.push({
                field: 'email',
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
            const __raw_id = obj['id'] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj['email'] as string;
            instance.email = __raw_email;
        }
        if ('name' in obj && obj['name'] !== undefined) {
            const __raw_name = obj['name'] as string;
            instance.name = __raw_name;
        } else {
            instance.name = "guest";
        }
        if ('age' in obj && obj['age'] !== undefined) {
            const __raw_age = obj['age'] as number;
            instance.age = __raw_age;
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

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'id' in o && 'email' in o;
    }

    static is(obj: unknown): obj is User {
        if (obj instanceof User) {
            return true;
        }
        if (!User.hasShape(obj)) {
            return false;
        }
        const result = User.deserialize(obj);
        return Result.isOk(result);
    }
}

// Usage:
const result = User.deserialize('{"id":1,"email":"test@example.com"}');
if (Result.isOk(result)) {
    const user = result.value;
} else {
    console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
}
```

Generated output:

```typescript
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

/** @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    email: string;

    name: string;

    age?: number;

    constructor(props: {
        id: number;
        email: string;
        name?: string;
        age?: number;
    }) {
        this.id = props.id;
        this.email = props.email;
        this.name = props.name as string;
        this.age = props.age as number;
    }

    /**
     * Deserializes input to an instance of this class.
     * Automatically detects whether input is a JSON string or object.
     * @param input - JSON string or object to deserialize
     * @param opts - Optional deserialization options
     * @returns Result containing the deserialized instance or validation errors
     */
    static deserialize(
        input: unknown,
        opts?: DeserializeOptions
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === 'string' ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'User.deserialize: root cannot be a forward reference'
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

    /** @internal */
    static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'User.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: 'unknown field'
                });
            }
        }
        if (!('id' in obj)) {
            errors.push({
                field: 'id',
                message: 'missing required field'
            });
        }
        if (!('email' in obj)) {
            errors.push({
                field: 'email',
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
            const __raw_id = obj['id'] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj['email'] as string;
            instance.email = __raw_email;
        }
        if ('name' in obj && obj['name'] !== undefined) {
            const __raw_name = obj['name'] as string;
            instance.name = __raw_name;
        } else {
            instance.name = 'guest';
        }
        if ('age' in obj && obj['age'] !== undefined) {
            const __raw_age = obj['age'] as number;
            instance.age = __raw_age;
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

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'id' in o && 'email' in o;
    }

    static is(obj: unknown): obj is User {
        if (obj instanceof User) {
            return true;
        }
        if (!User.hasShape(obj)) {
            return false;
        }
        const result = User.deserialize(obj);
        return Result.isOk(result);
    }
}

// Usage:
const result = User.deserialize('{"id":1,"email":"test@example.com"}');
if (Result.isOk(result)) {
    const user = result.value;
} else {
    console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
}
```

Generated output:

```typescript
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

/** @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    email: string;

    name: string;

    age?: number;

    constructor(props: {
        id: number;
        email: string;
        name?: string;
        age?: number;
    }) {
        this.id = props.id;
        this.email = props.email;
        this.name = props.name as string;
        this.age = props.age as number;
    }

    /**
     * Deserializes input to an instance of this class.
     * Automatically detects whether input is a JSON string or object.
     * @param input - JSON string or object to deserialize
     * @param opts - Optional deserialization options
     * @returns Result containing the deserialized instance or validation errors
     */
    static deserialize(
        input: unknown,
        opts?: DeserializeOptions
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === 'string' ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'User.deserialize: root cannot be a forward reference'
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

    /** @internal */
    static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'User.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: 'unknown field'
                });
            }
        }
        if (!('id' in obj)) {
            errors.push({
                field: 'id',
                message: 'missing required field'
            });
        }
        if (!('email' in obj)) {
            errors.push({
                field: 'email',
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
            const __raw_id = obj['id'] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj['email'] as string;
            instance.email = __raw_email;
        }
        if ('name' in obj && obj['name'] !== undefined) {
            const __raw_name = obj['name'] as string;
            instance.name = __raw_name;
        } else {
            instance.name = 'guest';
        }
        if ('age' in obj && obj['age'] !== undefined) {
            const __raw_age = obj['age'] as number;
            instance.age = __raw_age;
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

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'id' in o && 'email' in o;
    }

    static is(obj: unknown): obj is User {
        if (obj instanceof User) {
            return true;
        }
        if (!User.hasShape(obj)) {
            return false;
        }
        const result = User.deserialize(obj);
        return Result.isOk(result);
    }
}

// Usage:
const result = User.deserialize('{"id":1,"email":"test@example.com"}');
if (Result.isOk(result)) {
    const user = result.value;
} else {
    console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
}
```

Generated output:

```typescript
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

/** @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    email: string;

    name: string;

    age?: number;

    constructor(props: {
        id: number;
        email: string;
        name?: string;
        age?: number;
    }) {
        this.id = props.id;
        this.email = props.email;
        this.name = props.name as string;
        this.age = props.age as number;
    }

    /**
     * Deserializes input to an instance of this class.
     * Automatically detects whether input is a JSON string or object.
     * @param input - JSON string or object to deserialize
     * @param opts - Optional deserialization options
     * @returns Result containing the deserialized instance or validation errors
     */
    static deserialize(
        input: unknown,
        opts?: DeserializeOptions
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === 'string' ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'User.deserialize: root cannot be a forward reference'
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

    /** @internal */
    static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'User.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: 'unknown field'
                });
            }
        }
        if (!('id' in obj)) {
            errors.push({
                field: 'id',
                message: 'missing required field'
            });
        }
        if (!('email' in obj)) {
            errors.push({
                field: 'email',
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
            const __raw_id = obj['id'] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj['email'] as string;
            instance.email = __raw_email;
        }
        if ('name' in obj && obj['name'] !== undefined) {
            const __raw_name = obj['name'] as string;
            instance.name = __raw_name;
        } else {
            instance.name = 'guest';
        }
        if ('age' in obj && obj['age'] !== undefined) {
            const __raw_age = obj['age'] as number;
            instance.age = __raw_age;
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

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'id' in o && 'email' in o;
    }

    static is(obj: unknown): obj is User {
        if (obj instanceof User) {
            return true;
        }
        if (!User.hasShape(obj)) {
            return false;
        }
        const result = User.deserialize(obj);
        return Result.isOk(result);
    }
}

// Usage:
const result = User.deserialize('{"id":1,"email":"test@example.com"}');
if (Result.isOk(result)) {
    const user = result.value;
} else {
    console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
}
```

## Required Imports

The generated code automatically imports:
- `Result` from `macroforge/utils`
- `DeserializeContext`, `DeserializeError`, `PendingRef` from `macroforge/serde`