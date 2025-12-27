import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";

export type RecurrenceEnd = /** @default(0) */ number | string;

export function recurrenceEndDefaultValue#0#0(): RecurrenceEnd {
    return 0;
}

export function recurrenceEndSerialize#0#0(value: RecurrenceEnd): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(recurrenceEndSerializeWithContext(value, ctx));
}
export function recurrenceEndSerializeWithContext#0#0(value: RecurrenceEnd, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function recurrenceEndDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: RecurrenceEnd } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = recurrenceEndDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "RecurrenceEnd.deserialize: root cannot be a forward reference"
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
export function recurrenceEndDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): RecurrenceEnd | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as RecurrenceEnd | __mf_PendingRef;
    }
    if (typeof value === `${"number"}`) {
        return value as RecurrenceEnd;
    }
    if (typeof value === `${"string"}`) {
        return value as RecurrenceEnd;
    }
    throw new __mf_DeserializeError([
        {
            field: "_root",
            message: `${"RecurrenceEnd"}.deserializeWithContext: value does not match any union member`
        }
    ]);
}
export function recurrenceEndIs(value: unknown): value is RecurrenceEnd {
    if (typeof value === `${"number"}`) return true;
    if (typeof value === `${"string"}`) return true;
    return false;
}

export const RecurrenceEnd = {
  deserialize: recurrenceEndDeserialize,
  deserializeWithContext: recurrenceEndDeserializeWithContext,
  is: recurrenceEndIs
} as const;