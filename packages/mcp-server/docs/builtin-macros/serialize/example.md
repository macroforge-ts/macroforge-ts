## Example

```typescript before
/** @derive(Serialize) */
class User {
    id: number;

    /** @serde({ rename: "userName" }) */
    name: string;

    /** @serde({ skipSerializing: true }) */
    password: string;

    /** @serde({ flatten: true }) */
    metadata: UserMetadata;
}
```

```typescript after
import { SerializeContext } from 'macroforge/serde';

class User {
    id: number;

    name: string;

    password: string;

    metadata: UserMetadata;
    /** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata  */

    static serialize(value: User): string {
        return userSerialize(value);
    }
    /** @internal Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context  */

    static serializeWithContext(value: User, ctx: SerializeContext): Record<string, unknown> {
        return userSerializeWithContext(value, ctx);
    }
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function userSerialize(
    value: User
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(userSerializeWithContext(value, ctx));
} /** @internal Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function userSerializeWithContext(
    value: User,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'User', __id };
    result['id'] = value.id;
    result['userName'] = value.name;
    {
        const __flattened = userMetadataSerializeWithContext(value.metadata, ctx);
        const { __type: _, __id: __, ...rest } = __flattened as any;
        Object.assign(result, rest);
    }
    return result;
}
```

Generated output:

```typescript
import { SerializeContext } from 'macroforge/serde';

class User {
    id: number;

    name: string;

    password: string;

    metadata: UserMetadata;
    /** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata  */

    static serialize(value: User): string {
        return userSerialize(value);
    }
    /** @internal Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context  */

    static serializeWithContext(value: User, ctx: SerializeContext): Record<string, unknown> {
        return userSerializeWithContext(value, ctx);
    }
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function userSerialize(
    value: User
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(userSerializeWithContext(value, ctx));
} /** @internal Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function userSerializeWithContext(
    value: User,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'User', __id };
    result['id'] = value.id;
    result['userName'] = value.name;
    {
        const __flattened = userMetadataSerializeWithContext(value.metadata, ctx);
        const { __type: _, __id: __, ...rest } = __flattened as any;
        Object.assign(result, rest);
    }
    return result;
}
```

## Required Import

The generated code automatically imports `SerializeContext` from `macroforge/serde`.