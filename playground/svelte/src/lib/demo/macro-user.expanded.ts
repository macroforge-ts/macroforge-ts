import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
/** import macro { JSON } from "@playground/macro"; */

export class MacroUser {
    id: string;
    name: string;
    role: string;
    favoriteMacro: 'Derive' | 'JsonNative';
    since: string;

    apiToken: string;

    static toString(value: MacroUser): string {
        return macroUserToString(value);
    }
    /** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata  */

    static serialize(value: MacroUser): string {
        return macroUserSerialize(value);
    }
    /** @internal Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context  */

    static serializeWithContext(value: MacroUser, ctx: SerializeContext): Record<string, unknown> {
        return macroUserSerializeWithContext(value, ctx);
    }

    constructor(props: {
        id: string;
        name: string;
        role: string;
        favoriteMacro: 'Derive' | 'JsonNative';
        since: string;
        apiToken: string;
    }) {
        this.id = props.id;
        this.name = props.name;
        this.role = props.role;
        this.favoriteMacro = props.favoriteMacro;
        this.since = props.since;
        this.apiToken = props.apiToken;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: DeserializeOptions
    ): Result<
        MacroUser,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = DeserializeContext.create();
            const resultOrRef = MacroUser.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'MacroUser.deserialize: root cannot be a forward reference'
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
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): MacroUser | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'MacroUser.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('id' in obj)) {
            errors.push({
                field: 'id',
                message: 'missing required field'
            });
        }
        if (!('name' in obj)) {
            errors.push({
                field: 'name',
                message: 'missing required field'
            });
        }
        if (!('role' in obj)) {
            errors.push({
                field: 'role',
                message: 'missing required field'
            });
        }
        if (!('favoriteMacro' in obj)) {
            errors.push({
                field: 'favoriteMacro',
                message: 'missing required field'
            });
        }
        if (!('since' in obj)) {
            errors.push({
                field: 'since',
                message: 'missing required field'
            });
        }
        if (!('apiToken' in obj)) {
            errors.push({
                field: 'apiToken',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(MacroUser.prototype) as MacroUser;
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
            const __raw_role = obj['role'] as string;
            instance.role = __raw_role;
        }
        {
            const __raw_favoriteMacro = obj['favoriteMacro'] as 'Derive' | 'JsonNative';
            instance.favoriteMacro = __raw_favoriteMacro;
        }
        {
            const __raw_since = obj['since'] as string;
            instance.since = __raw_since;
        }
        {
            const __raw_apiToken = obj['apiToken'] as string;
            instance.apiToken = __raw_apiToken;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof MacroUser>(
        field: K,
        value: MacroUser[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static validateFields(partial: Partial<MacroUser>): Array<{
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
        return (
            'id' in o &&
            'name' in o &&
            'role' in o &&
            'favoriteMacro' in o &&
            'since' in o &&
            'apiToken' in o
        );
    }

    static is(obj: unknown): obj is MacroUser {
        if (obj instanceof MacroUser) {
            return true;
        }
        if (!MacroUser.hasShape(obj)) {
            return false;
        }
        const result = MacroUser.deserialize(obj);
        return Result.isOk(result);
    }
}

export function macroUserToString(value: MacroUser): string {
    const parts: string[] = [];
    parts.push('userId: ' + value.id);
    parts.push('name: ' + value.name);
    parts.push('role: ' + value.role);
    parts.push('favoriteMacro: ' + value.favoriteMacro);
    parts.push('since: ' + value.since);
    return 'MacroUser { ' + parts.join(', ') + ' }';
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function macroUserSerialize(
    value: MacroUser
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(macroUserSerializeWithContext(value, ctx));
} /** @internal Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function macroUserSerializeWithContext(
    value: MacroUser,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'MacroUser', __id };
    result['id'] = value.id;
    result['name'] = value.name;
    result['role'] = value.role;
    result['favoriteMacro'] = value.favoriteMacro;
    result['since'] = value.since;
    result['apiToken'] = value.apiToken;
    return result;
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function macroUserDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Result<MacroUser, Array<{ field: string; message: string }>> {
    return MacroUser.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function macroUserDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): MacroUser | PendingRef {
    return MacroUser.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function macroUserIs(value: unknown): value is MacroUser {
    return MacroUser.is(value);
}

const showcaseUser = new MacroUser({
    id: 'usr_2626',
    name: 'Alya Vector',
    role: 'Macro Explorer',
    favoriteMacro: 'Derive',
    since: '2024-09-12',
    apiToken: 'svelte-secret-token'
});

export const showcaseUserSummary = MacroUser.toString(showcaseUser);
export const showcaseUserJson = JSON.parse(MacroUser.serialize(showcaseUser));
