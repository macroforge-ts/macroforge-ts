import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
/** import macro {Gigaform} from "@playground/macro"; */


export interface Paid {
    amount: number | null;
    currency: string | null;
    paymentMethod: string | null;
}

export function paidDefaultValue(): Paid {
    return {
        amount: null,
        currency: null,
        paymentMethod: null
    } as Paid;
}

export function paidSerialize(value: Paid): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(paidSerializeWithContext(value, ctx));
}
export function paidSerializeWithContext(value: Paid, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Paid"}`,
        __id
    };
    result[`${"amount"}`] = value.amount;
    result[`${"currency"}`] = value.currency;
    result[`${"paymentMethod"}`] = value.paymentMethod;
    return result;
}

export function paidDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Paid } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = paidDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Paid.deserialize: root cannot be a forward reference"
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
export function paidDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Paid | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Paid"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"amount"}` in obj)) {
        errors.push({
            field: `${"amount"}`,
            message: "missing required field"
        });
    }
    if (!(`${"currency"}` in obj)) {
        errors.push({
            field: `${"currency"}`,
            message: "missing required field"
        });
    }
    if (!(`${"paymentMethod"}` in obj)) {
        errors.push({
            field: `${"paymentMethod"}`,
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_amount = obj[`${"amount"}`] as number | null;
        instance.amount = __raw_amount;
    }
    {
        const __raw_currency = obj[`${"currency"}`] as string | null;
        instance.currency = __raw_currency;
    }
    {
        const __raw_paymentMethod = obj[`${"paymentMethod"}`] as string | null;
        instance.paymentMethod = __raw_paymentMethod;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Paid;
}
export function paidValidateField<K extends keyof Paid>(_field: K, _value: Paid[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function paidValidateFields(_partial: Partial<Paid>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function paidHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"amount" in o && "currency" in o && "paymentMethod" in o';
}
export function paidIs(obj: unknown): obj is Paid {
    if (!paidHasShape(obj)) {
        return false;
    }
    const result = paidDeserialize(obj);
    return result.success;
}

export const Paid = {
  defaultValue: paidDefaultValue,
  serialize: paidSerialize,
  serializeWithContext: paidSerializeWithContext,
  deserialize: paidDeserialize,
  deserializeWithContext: paidDeserializeWithContext,
  validateFields: paidValidateFields,
  hasShape: paidHasShape,
  is: paidIs
} as const;