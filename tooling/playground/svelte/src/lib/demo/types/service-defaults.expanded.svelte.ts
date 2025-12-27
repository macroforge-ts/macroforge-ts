import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
/** import macro {Gigaform} from "@playground/macro"; */


export interface ServiceDefaults {
    
    price: number;
    
    
    description: string;
}

export function serviceDefaultsDefaultValue(): ServiceDefaults {
    return {
        price: 0,
        description: ""
    } as ServiceDefaults;
}

export function serviceDefaultsSerialize(value: ServiceDefaults): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(serviceDefaultsSerializeWithContext(value, ctx));
}
export function serviceDefaultsSerializeWithContext(value: ServiceDefaults, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"ServiceDefaults"}`,
        __id
    };
    result[`${"price"}`] = value.price;
    result[`${"description"}`] = value.description;
    return result;
}

export function serviceDefaultsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: ServiceDefaults } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = serviceDefaultsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "ServiceDefaults.deserialize: root cannot be a forward reference"
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
export function serviceDefaultsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): ServiceDefaults | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"ServiceDefaults"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"price"}` in obj)) {
        errors.push({
            field: `${"price"}`,
            message: "missing required field"
        });
    }
    if (!(`${"description"}` in obj)) {
        errors.push({
            field: `${"description"}`,
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
        const __raw_price = obj[`${"price"}`] as number;
        instance.price = __raw_price;
    }
    {
        const __raw_description = obj[`${"description"}`] as string;
        if (__raw_description.trim().length === 0) {
            errors.push({
                field: "description",
                message: "ServiceDefaults.description must not be empty"
            });
        }
        instance.description = __raw_description;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as ServiceDefaults;
}
export function serviceDefaultsValidateField<K extends keyof ServiceDefaults>(_field: K, _value: ServiceDefaults[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"description"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "description",
                message: "ServiceDefaults.description must not be empty"
            });
        }
    }
    return errors;
}
export function serviceDefaultsValidateFields(_partial: Partial<ServiceDefaults>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"description"}` in _partial && _partial.description !== undefined) {
        const __val = _partial.description as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "description",
                message: "ServiceDefaults.description must not be empty"
            });
        }
    }
    return errors;
}
export function serviceDefaultsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"price" in o && "description" in o';
}
export function serviceDefaultsIs(obj: unknown): obj is ServiceDefaults {
    if (!serviceDefaultsHasShape(obj)) {
        return false;
    }
    const result = serviceDefaultsDeserialize(obj);
    return result.success;
}

export const ServiceDefaults = {
  defaultValue: serviceDefaultsDefaultValue,
  serialize: serviceDefaultsSerialize,
  serializeWithContext: serviceDefaultsSerializeWithContext,
  deserialize: serviceDefaultsDeserialize,
  deserializeWithContext: serviceDefaultsDeserializeWithContext,
  validateFields: serviceDefaultsValidateFields,
  hasShape: serviceDefaultsHasShape,
  is: serviceDefaultsIs
} as const;