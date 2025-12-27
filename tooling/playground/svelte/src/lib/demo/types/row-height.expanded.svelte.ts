import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";

export type RowHeight = 'ExtraSmall' | 'Small' | /** @default */ 'Medium' | 'Large';

export function rowHeightDefaultValue#0#0(): RowHeight {
    return 'Medium';
}

export function rowHeightSerialize#0#0(value: RowHeight): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(rowHeightSerializeWithContext(value, ctx));
}
export function rowHeightSerializeWithContext#0#0(value: RowHeight, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function rowHeightDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: RowHeight } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = rowHeightDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "RowHeight.deserialize: root cannot be a forward reference"
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
export function rowHeightDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): RowHeight | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as RowHeight | __mf_PendingRef;
    }
    const allowedValues = [
        "'ExtraSmall', 'Small', 'Medium', 'Large'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"RowHeight"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as RowHeight;
}
export function rowHeightIs(value: unknown): value is RowHeight {
    const allowedValues = [
        "'ExtraSmall', 'Small', 'Medium', 'Large'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const RowHeight = {
  deserialize: rowHeightDeserialize,
  deserializeWithContext: rowHeightDeserializeWithContext,
  is: rowHeightIs
} as const;