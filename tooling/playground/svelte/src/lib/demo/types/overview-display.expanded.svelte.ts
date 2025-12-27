import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";

export type OverviewDisplay = /** @default */ 'Card' | 'Table';

export function overviewDisplayDefaultValue#0#0(): OverviewDisplay {
    return 'Card';
}

export function overviewDisplaySerialize#0#0(value: OverviewDisplay): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(overviewDisplaySerializeWithContext(value, ctx));
}
export function overviewDisplaySerializeWithContext#0#0(value: OverviewDisplay, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function overviewDisplayDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: OverviewDisplay } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = overviewDisplayDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "OverviewDisplay.deserialize: root cannot be a forward reference"
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
export function overviewDisplayDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): OverviewDisplay | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as OverviewDisplay | __mf_PendingRef;
    }
    const allowedValues = [
        "'Card', 'Table'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"OverviewDisplay"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as OverviewDisplay;
}
export function overviewDisplayIs(value: unknown): value is OverviewDisplay {
    const allowedValues = [
        "'Card', 'Table'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const OverviewDisplay = {
  deserialize: overviewDisplayDeserialize,
  deserializeWithContext: overviewDisplayDeserializeWithContext,
  is: overviewDisplayIs
} as const;