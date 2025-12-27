import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
/** import macro {Gigaform} from "@playground/macro"; */


export interface Package {
    
    id: string;
    
    date: string;
}

export function packageDefaultValue(): Package {
    return {
        id: "",
        date: ""
    } as Package;
}

export function packageSerialize(value: Package): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(packageSerializeWithContext(value, ctx));
}
export function packageSerializeWithContext(value: Package, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Package"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"date"}`] = value.date;
    return result;
}

export function packageDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Package } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = packageDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Package.deserialize: root cannot be a forward reference"
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
export function packageDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Package | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Package"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"id"}` in obj)) {
        errors.push({
            field: `${"id"}`,
            message: "missing required field"
        });
    }
    if (!(`${"date"}` in obj)) {
        errors.push({
            field: `${"date"}`,
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
        const __raw_id = obj[`${"id"}`] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_date = obj[`${"date"}`] as string;
        instance.date = __raw_date;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Package;
}
export function packageValidateField<K extends keyof Package>(_field: K, _value: Package[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function packageValidateFields(_partial: Partial<Package>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function packageHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "date" in o';
}
export function packageIs(obj: unknown): obj is Package {
    if (!packageHasShape(obj)) {
        return false;
    }
    const result = packageDeserialize(obj);
    return result.success;
}

export const Package = {
  defaultValue: packageDefaultValue,
  serialize: packageSerialize,
  serializeWithContext: packageSerializeWithContext,
  deserialize: packageDeserialize,
  deserializeWithContext: packageDeserializeWithContext,
  validateFields: packageValidateFields,
  hasShape: packageHasShape,
  is: packageIs
} as const;