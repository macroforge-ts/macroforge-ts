import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { ok as __mf_resultOk } from 'macroforge/reexports';
import { err as __mf_resultErr } from 'macroforge/reexports';
import { isOk as __mf_resultIsOk } from 'macroforge/reexports';
import type { Result as __mf_Result } from 'macroforge/reexports';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
/**
 * Comprehensive test class demonstrating all available macros.
 * Used for Playwright e2e tests to verify macro expansion works at runtime.
 */

export class AllMacrosTestClass {
    id: number;

    name: string;

    email: string;

    secretToken: string;

    isActive: boolean;

    score: number;

    static toString(value: AllMacrosTestClass): string {
        return allMacrosTestClassToString(value);
    }

    static clone(value: AllMacrosTestClass): AllMacrosTestClass {
        return allMacrosTestClassClone(value);
    }

    static equals(a: AllMacrosTestClass, b: AllMacrosTestClass): boolean {
        return allMacrosTestClassEquals(a, b);
    }
    /** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata  */

    static serialize(value: AllMacrosTestClass): string {
        return allMacrosTestClassSerialize(value);
    }
    /** @internal Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context  */

    static serializeWithContext(
        value: AllMacrosTestClass,
        ctx: __mf_SerializeContext
    ): Record<string, unknown> {
        return allMacrosTestClassSerializeWithContext(value, ctx);
    }

    constructor(props: {
        id: number;
        name: string;
        email: string;
        secretToken: string;
        isActive: boolean;
        score: number;
    }) {
        this.id = props.id;
        this.name = props.name;
        this.email = props.email;
        this.secretToken = props.secretToken;
        this.isActive = props.isActive;
        this.score = props.score;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ): __mf_Result<
        AllMacrosTestClass,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = AllMacrosTestClass.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message:
                            'AllMacrosTestClass.deserialize: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return __mf_resultOk(resultOrRef);
        } catch (e) {
            if (e instanceof __mf_DeserializeError) {
                return __mf_resultErr(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return __mf_resultErr([
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

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): AllMacrosTestClass | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'AllMacrosTestClass.deserializeWithContext: expected an object'
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
        if (!('email' in obj)) {
            errors.push({
                field: 'email',
                message: 'missing required field'
            });
        }
        if (!('secretToken' in obj)) {
            errors.push({
                field: 'secretToken',
                message: 'missing required field'
            });
        }
        if (!('isActive' in obj)) {
            errors.push({
                field: 'isActive',
                message: 'missing required field'
            });
        }
        if (!('score' in obj)) {
            errors.push({
                field: 'score',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(AllMacrosTestClass.prototype) as AllMacrosTestClass;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_id = obj['id'] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_name = obj['name'] as string;
            instance.name = __raw_name;
        }
        {
            const __raw_email = obj['email'] as string;
            instance.email = __raw_email;
        }
        {
            const __raw_secretToken = obj['secretToken'] as string;
            instance.secretToken = __raw_secretToken;
        }
        {
            const __raw_isActive = obj['isActive'] as boolean;
            instance.isActive = __raw_isActive;
        }
        {
            const __raw_score = obj['score'] as number;
            instance.score = __raw_score;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof AllMacrosTestClass>(
        field: K,
        value: AllMacrosTestClass[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static validateFields(partial: Partial<AllMacrosTestClass>): Array<{
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
            'email' in o &&
            'secretToken' in o &&
            'isActive' in o &&
            'score' in o
        );
    }

    static is(obj: unknown): obj is AllMacrosTestClass {
        if (obj instanceof AllMacrosTestClass) {
            return true;
        }
        if (!AllMacrosTestClass.hasShape(obj)) {
            return false;
        }
        const result = AllMacrosTestClass.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

export function allMacrosTestClassToString(value: AllMacrosTestClass): string {
    const parts: string[] = [];
    parts.push('identifier: ' + value.id);
    parts.push('name: ' + value.name);
    parts.push('email: ' + value.email);
    parts.push('isActive: ' + value.isActive);
    parts.push('score: ' + value.score);
    return 'AllMacrosTestClass { ' + parts.join(', ') + ' }';
}

export function allMacrosTestClassClone(value: AllMacrosTestClass): AllMacrosTestClass {
    const cloned = Object.create(Object.getPrototypeOf(value));
    cloned.id = value.id;
    cloned.name = value.name;
    cloned.email = value.email;
    cloned.secretToken = value.secretToken;
    cloned.isActive = value.isActive;
    cloned.score = value.score;
    return cloned;
}

export function allMacrosTestClassEquals(a: AllMacrosTestClass, b: AllMacrosTestClass): boolean {
    if (a === b) return true;
    return (
        a.id === b.id &&
        a.name === b.name &&
        a.email === b.email &&
        a.secretToken === b.secretToken &&
        a.isActive === b.isActive &&
        a.score === b.score
    );
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function allMacrosTestClassSerialize(
    value: AllMacrosTestClass
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(allMacrosTestClassSerializeWithContext(value, ctx));
} /** @internal Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function allMacrosTestClassSerializeWithContext(
    value: AllMacrosTestClass,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'AllMacrosTestClass', __id };
    result['id'] = value.id;
    result['name'] = value.name;
    result['email'] = value.email;
    result['secretToken'] = value.secretToken;
    result['isActive'] = value.isActive;
    result['score'] = value.score;
    return result;
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function allMacrosTestClassDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<AllMacrosTestClass, Array<{ field: string; message: string }>> {
    return AllMacrosTestClass.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function allMacrosTestClassDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): AllMacrosTestClass | __mf_PendingRef {
    return AllMacrosTestClass.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function allMacrosTestClassIs(value: unknown): value is AllMacrosTestClass {
    return AllMacrosTestClass.is(value);
}

// Pre-instantiated test instance for e2e tests
export const testInstance = new AllMacrosTestClass({
    id: 42,
    name: 'Test User',
    email: 'test@example.com',
    secretToken: 'secret-token-123',
    isActive: true,
    score: 100
});
