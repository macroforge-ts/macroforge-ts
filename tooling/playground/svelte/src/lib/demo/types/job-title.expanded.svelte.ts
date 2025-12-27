import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";

export type JobTitle =
    | /** @default */ 'Technician'
    | 'SalesRepresentative'
    | 'HumanResources'
    | 'InformationTechnology';

export function jobTitleDefaultValue#0#0(): JobTitle {
    return 'Technician';
}

export function jobTitleSerialize#0#0(value: JobTitle): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(jobTitleSerializeWithContext(value, ctx));
}
export function jobTitleSerializeWithContext#0#0(value: JobTitle, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function jobTitleDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: JobTitle } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = jobTitleDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "JobTitle.deserialize: root cannot be a forward reference"
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
export function jobTitleDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): JobTitle | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as JobTitle | __mf_PendingRef;
    }
    const allowedValues = [
        "'Technician', 'SalesRepresentative', 'HumanResources', 'InformationTechnology'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"JobTitle"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as JobTitle;
}
export function jobTitleIs(value: unknown): value is JobTitle {
    const allowedValues = [
        "'Technician', 'SalesRepresentative', 'HumanResources', 'InformationTechnology'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const JobTitle = {
  deserialize: jobTitleDeserialize,
  deserializeWithContext: jobTitleDeserializeWithContext,
  is: jobTitleIs
} as const;