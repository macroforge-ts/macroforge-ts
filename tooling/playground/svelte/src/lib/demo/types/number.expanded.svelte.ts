import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
/** import macro {Gigaform} from "@playground/macro"; */


export interface Number {
    
    countryCode: string;
    
    areaCode: string;
    
    localNumber: string;
}

export function numberDefaultValue(): Number {
    return {
        countryCode: "",
        areaCode: "",
        localNumber: ""
    } as Number;
}

export function numberSerialize(value: Number): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(numberSerializeWithContext(value, ctx));
}
export function numberSerializeWithContext(value: Number, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Number"}`,
        __id
    };
    result[`${"countryCode"}`] = value.countryCode;
    result[`${"areaCode"}`] = value.areaCode;
    result[`${"localNumber"}`] = value.localNumber;
    return result;
}

export function numberDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Number } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = numberDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Number.deserialize: root cannot be a forward reference"
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
export function numberDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Number | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Number"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"countryCode"}` in obj)) {
        errors.push({
            field: `${"countryCode"}`,
            message: "missing required field"
        });
    }
    if (!(`${"areaCode"}` in obj)) {
        errors.push({
            field: `${"areaCode"}`,
            message: "missing required field"
        });
    }
    if (!(`${"localNumber"}` in obj)) {
        errors.push({
            field: `${"localNumber"}`,
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
        const __raw_countryCode = obj[`${"countryCode"}`] as string;
        if (__raw_countryCode.trim().length === 0) {
            errors.push({
                field: "countryCode",
                message: "Number.countryCode must not be empty"
            });
        }
        instance.countryCode = __raw_countryCode;
    }
    {
        const __raw_areaCode = obj[`${"areaCode"}`] as string;
        if (__raw_areaCode.trim().length === 0) {
            errors.push({
                field: "areaCode",
                message: "Number.areaCode must not be empty"
            });
        }
        instance.areaCode = __raw_areaCode;
    }
    {
        const __raw_localNumber = obj[`${"localNumber"}`] as string;
        if (__raw_localNumber.trim().length === 0) {
            errors.push({
                field: "localNumber",
                message: "Number.localNumber must not be empty"
            });
        }
        instance.localNumber = __raw_localNumber;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Number;
}
export function numberValidateField<K extends keyof Number>(_field: K, _value: Number[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"countryCode"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "countryCode",
                message: "Number.countryCode must not be empty"
            });
        }
    }
    if (_field === `${"areaCode"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "areaCode",
                message: "Number.areaCode must not be empty"
            });
        }
    }
    if (_field === `${"localNumber"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "localNumber",
                message: "Number.localNumber must not be empty"
            });
        }
    }
    return errors;
}
export function numberValidateFields(_partial: Partial<Number>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"countryCode"}` in _partial && _partial.countryCode !== undefined) {
        const __val = _partial.countryCode as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "countryCode",
                message: "Number.countryCode must not be empty"
            });
        }
    }
    if (`${"areaCode"}` in _partial && _partial.areaCode !== undefined) {
        const __val = _partial.areaCode as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "areaCode",
                message: "Number.areaCode must not be empty"
            });
        }
    }
    if (`${"localNumber"}` in _partial && _partial.localNumber !== undefined) {
        const __val = _partial.localNumber as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "localNumber",
                message: "Number.localNumber must not be empty"
            });
        }
    }
    return errors;
}
export function numberHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"countryCode" in o && "areaCode" in o && "localNumber" in o';
}
export function numberIs(obj: unknown): obj is Number {
    if (!numberHasShape(obj)) {
        return false;
    }
    const result = numberDeserialize(obj);
    return result.success;
}

export const Number = {
  defaultValue: numberDefaultValue,
  serialize: numberSerialize,
  serializeWithContext: numberSerializeWithContext,
  deserialize: numberDeserialize,
  deserializeWithContext: numberDeserializeWithContext,
  validateFields: numberValidateFields,
  hasShape: numberHasShape,
  is: numberIs
} as const;