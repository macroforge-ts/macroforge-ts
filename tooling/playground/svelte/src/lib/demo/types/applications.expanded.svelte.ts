import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";

export type Applications =
    | /** @default */ 'Sales'
    | 'Accounting'
    | 'Errand'
    | 'HumanResources'
    | 'Logistics'
    | 'Marketing'
    | 'Website';

export function applicationsDefaultValue#0#0(): Applications {
    return 'Sales';
}

export function applicationsSerialize#0#0(value: Applications): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(applicationsSerializeWithContext(value, ctx));
}
export function applicationsSerializeWithContext#0#0(value: Applications, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function applicationsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Applications } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = applicationsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Applications.deserialize: root cannot be a forward reference"
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
export function applicationsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Applications | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Applications | __mf_PendingRef;
    }
    const allowedValues = [
        "'Sales', 'Accounting', 'Errand', 'HumanResources', 'Logistics', 'Marketing', 'Website'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"Applications"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as Applications;
}
export function applicationsIs(value: unknown): value is Applications {
    const allowedValues = [
        "'Sales', 'Accounting', 'Errand', 'HumanResources', 'Logistics', 'Marketing', 'Website'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const Applications = {
  deserialize: applicationsDeserialize,
  deserializeWithContext: applicationsDeserializeWithContext,
  is: applicationsIs
} as const;