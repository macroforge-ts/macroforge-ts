import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";

export type IntervalUnit = /** @default */ 'Day' | 'Week' | 'Month' | 'Year';

export function intervalUnitDefaultValue#0#0(): IntervalUnit {
    return 'Day';
}

export function intervalUnitSerialize#0#0(value: IntervalUnit): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(intervalUnitSerializeWithContext(value, ctx));
}
export function intervalUnitSerializeWithContext#0#0(value: IntervalUnit, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function intervalUnitDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: IntervalUnit } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = intervalUnitDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "IntervalUnit.deserialize: root cannot be a forward reference"
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
export function intervalUnitDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): IntervalUnit | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as IntervalUnit | __mf_PendingRef;
    }
    const allowedValues = [
        "'Day', 'Week', 'Month', 'Year'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"IntervalUnit"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as IntervalUnit;
}
export function intervalUnitIs(value: unknown): value is IntervalUnit {
    const allowedValues = [
        "'Day', 'Week', 'Month', 'Year'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const IntervalUnit = {
  deserialize: intervalUnitDeserialize,
  deserializeWithContext: intervalUnitDeserializeWithContext,
  is: intervalUnitIs
} as const;