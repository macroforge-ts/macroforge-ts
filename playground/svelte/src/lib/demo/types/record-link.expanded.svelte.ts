import { SerializeContext } from 'macroforge/serde';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

export type RecordLink<T> = /** @default */ string | T;

export function recordLinkDefaultValue<T>(): RecordLink<T> {
    return '';
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function recordLinkSerialize<
    T
>(value: RecordLink<T>): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(recordLinkSerializeWithContext<T>(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function recordLinkSerializeWithContext<T>(
    value: RecordLink<T>,
    ctx: SerializeContext
): unknown {
    if (typeof (value as any)?.serializeWithContext === 'function') {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

/** Deserializes input to this type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function recordLinkDeserialize<
    T
>(
    input: unknown,
    opts?: DeserializeOptions
):
    | { success: true; value: RecordLink<T> }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = recordLinkDeserializeWithContext<T>(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: '_root',
                        message: 'RecordLink.deserialize: root cannot be a forward reference'
                    }
                ]
            };
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return { success: true, value: resultOrRef };
    } catch (e) {
        if (e instanceof DeserializeError) {
            return { success: false, errors: e.errors };
        }
        const message = e instanceof Error ? e.message : String(e);
        return { success: false, errors: [{ field: '_root', message }] };
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function recordLinkDeserializeWithContext<T>(
    value: any,
    ctx: DeserializeContext
): RecordLink<T> | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as RecordLink<T> | PendingRef;
    }
    if (typeof value === 'string') {
        return value as RecordLink<T>;
    }
    return value as RecordLink<T>;
    throw new DeserializeError([
        {
            field: '_root',
            message: 'RecordLink.deserializeWithContext: value does not match any union member'
        }
    ]);
}
export function recordLinkIs<T>(value: unknown): value is RecordLink<T> {
    if (typeof value === 'string') return true;
    return true;
}
