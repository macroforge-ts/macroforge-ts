import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

export type RecordLink<T> = /** @default */ string | T;

export function defaultValueRecordLink<T>(): RecordLink<T> {
    return '';
}

export function toStringifiedJSONRecordLink<T>(value: RecordLink<T>): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeRecordLink<T>(value, ctx));
}
export function toObjectRecordLink<T>(value: RecordLink<T>): unknown {
    const ctx = SerializeContext.create();
    return __serializeRecordLink<T>(value, ctx);
}
export function __serializeRecordLink<T>(value: RecordLink<T>, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONRecordLink<T>(
    json: string,
    opts?: DeserializeOptions
): Result<RecordLink<T>, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectRecordLink<T>(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectRecordLink<T>(
    obj: unknown,
    opts?: DeserializeOptions
): Result<RecordLink<T>, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeRecordLink<T>(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'RecordLink.fromObject: root cannot be a forward reference'
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
export function __deserializeRecordLink<T>(
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
            message: 'RecordLink.__deserialize: value does not match any union member'
        }
    ]);
}
export function isRecordLink<T>(value: unknown): value is RecordLink<T> {
    if (typeof value === 'string') return true;
    return true;
}
