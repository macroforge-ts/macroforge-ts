import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";

export type NextStep = /** @default */ 'InitialContact' | 'Qualified' | 'Estimate' | 'Negotiation';

export function nextStepDefaultValue#0#0(): NextStep {
    return 'InitialContact';
}

export function nextStepSerialize#0#0(value: NextStep): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(nextStepSerializeWithContext(value, ctx));
}
export function nextStepSerializeWithContext#0#0(value: NextStep, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function nextStepDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: NextStep } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = nextStepDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "NextStep.deserialize: root cannot be a forward reference"
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
export function nextStepDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): NextStep | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as NextStep | __mf_PendingRef;
    }
    const allowedValues = [
        "'InitialContact', 'Qualified', 'Estimate', 'Negotiation'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"NextStep"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as NextStep;
}
export function nextStepIs(value: unknown): value is NextStep {
    const allowedValues = [
        "'InitialContact', 'Qualified', 'Estimate', 'Negotiation'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const NextStep = {
  deserialize: nextStepDeserialize,
  deserializeWithContext: nextStepDeserializeWithContext,
  is: nextStepIs
} as const;