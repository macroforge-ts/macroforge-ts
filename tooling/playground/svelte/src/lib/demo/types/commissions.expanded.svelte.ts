import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
/** import macro {Gigaform} from "@playground/macro"; */


export interface Commissions {
    
    technician: string;
    
    salesRep: string;
}

export function commissionsDefaultValue(): Commissions {
    return {
        technician: "",
        salesRep: ""
    } as Commissions;
}

export function commissionsSerialize(value: Commissions): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(commissionsSerializeWithContext(value, ctx));
}
export function commissionsSerializeWithContext(value: Commissions, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Commissions"}`,
        __id
    };
    result[`${"technician"}`] = value.technician;
    result[`${"salesRep"}`] = value.salesRep;
    return result;
}

export function commissionsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Commissions } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = commissionsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Commissions.deserialize: root cannot be a forward reference"
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
export function commissionsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Commissions | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Commissions"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"technician"}` in obj)) {
        errors.push({
            field: `${"technician"}`,
            message: "missing required field"
        });
    }
    if (!(`${"salesRep"}` in obj)) {
        errors.push({
            field: `${"salesRep"}`,
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
        const __raw_technician = obj[`${"technician"}`] as string;
        if (__raw_technician.trim().length === 0) {
            errors.push({
                field: "technician",
                message: "Commissions.technician must not be empty"
            });
        }
        instance.technician = __raw_technician;
    }
    {
        const __raw_salesRep = obj[`${"salesRep"}`] as string;
        if (__raw_salesRep.trim().length === 0) {
            errors.push({
                field: "salesRep",
                message: "Commissions.salesRep must not be empty"
            });
        }
        instance.salesRep = __raw_salesRep;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Commissions;
}
export function commissionsValidateField<K extends keyof Commissions>(_field: K, _value: Commissions[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"technician"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "technician",
                message: "Commissions.technician must not be empty"
            });
        }
    }
    if (_field === `${"salesRep"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "salesRep",
                message: "Commissions.salesRep must not be empty"
            });
        }
    }
    return errors;
}
export function commissionsValidateFields(_partial: Partial<Commissions>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"technician"}` in _partial && _partial.technician !== undefined) {
        const __val = _partial.technician as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "technician",
                message: "Commissions.technician must not be empty"
            });
        }
    }
    if (`${"salesRep"}` in _partial && _partial.salesRep !== undefined) {
        const __val = _partial.salesRep as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "salesRep",
                message: "Commissions.salesRep must not be empty"
            });
        }
    }
    return errors;
}
export function commissionsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"technician" in o && "salesRep" in o';
}
export function commissionsIs(obj: unknown): obj is Commissions {
    if (!commissionsHasShape(obj)) {
        return false;
    }
    const result = commissionsDeserialize(obj);
    return result.success;
}

export const Commissions = {
  defaultValue: commissionsDefaultValue,
  serialize: commissionsSerialize,
  serializeWithContext: commissionsSerializeWithContext,
  deserialize: commissionsDeserialize,
  deserializeWithContext: commissionsDeserializeWithContext,
  validateFields: commissionsValidateFields,
  hasShape: commissionsHasShape,
  is: commissionsIs
} as const;