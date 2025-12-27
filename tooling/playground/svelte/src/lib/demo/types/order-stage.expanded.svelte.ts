import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";

export type OrderStage = /** @default */ 'Estimate' | 'Active' | 'Invoice';

export function orderStageDefaultValue#0#0(): OrderStage {
    return 'Estimate';
}

export function orderStageSerialize#0#0(value: OrderStage): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(orderStageSerializeWithContext(value, ctx));
}
export function orderStageSerializeWithContext#0#0(value: OrderStage, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function orderStageDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: OrderStage } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = orderStageDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "OrderStage.deserialize: root cannot be a forward reference"
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
export function orderStageDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): OrderStage | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as OrderStage | __mf_PendingRef;
    }
    const allowedValues = [
        "'Estimate', 'Active', 'Invoice'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"OrderStage"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as OrderStage;
}
export function orderStageIs(value: unknown): value is OrderStage {
    const allowedValues = [
        "'Estimate', 'Active', 'Invoice'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const OrderStage = {
  deserialize: orderStageDeserialize,
  deserializeWithContext: orderStageDeserializeWithContext,
  is: orderStageIs
} as const;