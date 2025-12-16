import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
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

    toString(): string {
        const parts: string[] = [];
        parts.push('identifier: ' + this.id);
        parts.push('name: ' + this.name);
        parts.push('email: ' + this.email);
        parts.push('isActive: ' + this.isActive);
        parts.push('score: ' + this.score);
        return 'AllMacrosTestClass { ' + parts.join(', ') + ' }';
    }

    clone(): AllMacrosTestClass {
        const cloned = Object.create(Object.getPrototypeOf(this));
        cloned.id = this.id;
        cloned.name = this.name;
        cloned.email = this.email;
        cloned.secretToken = this.secretToken;
        cloned.isActive = this.isActive;
        cloned.score = this.score;
        return cloned;
    }

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof AllMacrosTestClass)) return false;
        const typedOther = other as AllMacrosTestClass;
        return (
            this.id === typedOther.id &&
            this.name === typedOther.name &&
            this.email === typedOther.email &&
            this.secretToken === typedOther.secretToken &&
            this.isActive === typedOther.isActive &&
            this.score === typedOther.score
        );
    }
    /** Serializes this instance to a JSON string.
@returns JSON string representation with cycle detection metadata  */

    serialize(): string {
        const ctx = SerializeContext.create();
        return JSON.stringify(this.serializeWithContext(ctx));
    }
    /** @internal Serializes with an existing context for nested/cyclic object graphs.  */

    serializeWithContext(ctx: SerializeContext): Record<string, unknown> {
        const existingId = ctx.getId(this);
        if (existingId !== undefined) {
            return {
                __ref: existingId
            };
        }
        const __id = ctx.register(this);
        const result: Record<string, unknown> = {
            __type: 'AllMacrosTestClass',
            __id
        };
        result['id'] = this.id;
        result['name'] = this.name;
        result['email'] = this.email;
        result['secretToken'] = this.secretToken;
        result['isActive'] = this.isActive;
        result['score'] = this.score;
        return result;
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
        opts?: DeserializeOptions
    ): Result<
        AllMacrosTestClass,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = DeserializeContext.create();
            const resultOrRef = AllMacrosTestClass.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
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

    static deserializeWithContext(
        value: any,
        ctx: DeserializeContext
    ): AllMacrosTestClass | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
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
            throw new DeserializeError(errors);
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
            throw new DeserializeError(errors);
        }
        return instance;
    }
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
