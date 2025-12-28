import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";

export type RecordLink<T> = /** @default */ string | T;

export function recordLinkDefaultValue#0<T>#0(): RecordLink<T> {
    return "";
}

export function recordLinkSerialize<$MfPh1>(value: RecordLink<T>): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(recordLinkSerializeWithContext(value, ctx));
}
export function recordLinkSerializeWithContext<$MfPh6>(value: RecordLink<T>, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function recordLinkDeserialize<T>(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: RecordLink<T> } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = recordLinkDeserializeWithContext<T>(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "RecordLink.deserialize: root cannot be a forward reference"
                    }
                ]
            };
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return {
            success: true,
            value: resultOrRef
        };
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return {
                success: false,
                errors: e.errors
            };
        }
        const message = e instanceof Error ? e.message : String(e);
        return {
            success: false,
            errors: [
                {
                    field: "_root",
                    message
                }
            ]
        };
    }
}
export function recordLinkDeserializeWithContext<T>(value: any, ctx: __mf_DeserializeContext): RecordLink<T> | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as RecordLink<T> | __mf_PendingRef;
    }
    if (typeof value === "string") {
        return value as RecordLink<T>;
    }
    return value as RecordLink<T>;
    throw new __mf_DeserializeError([
        {
            field: "_root",
            message: `${"RecordLink"}.deserializeWithContext: value does not match any union member`
        }
    ]);
}
export function recordLinkIs<T>(value: unknown): value is RecordLink<T> {
    if (typeof value === "string") return true;
    return true;
}