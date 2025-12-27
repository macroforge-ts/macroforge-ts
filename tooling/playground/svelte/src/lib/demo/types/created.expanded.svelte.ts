import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
/** import macro {Gigaform} from "@playground/macro"; */


export interface Created {
    initialData: string | null;
}

export function createdDefaultValue(): Created {
    return {
        initialData: null
    } as Created;
}

export function createdSerialize(value: Created): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(createdSerializeWithContext(value, ctx));
}
export function createdSerializeWithContext(value: Created, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Created"}`,
        __id
    };
    result[`${"initialData"}`] = value.initialData;
    return result;
}

export function createdDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Created } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = createdDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Created.deserialize: root cannot be a forward reference"
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
export function createdDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Created | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Created"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"initialData"}` in obj)) {
        errors.push({
            field: `${"initialData"}`,
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
        const __raw_initialData = obj[`${"initialData"}`] as string | null;
        instance.initialData = __raw_initialData;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Created;
}
export function createdValidateField<K extends keyof Created>(_field: K, _value: Created[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function createdValidateFields(_partial: Partial<Created>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function createdHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"initialData" in o';
}
export function createdIs(obj: unknown): obj is Created {
    if (!createdHasShape(obj)) {
        return false;
    }
    const result = createdDeserialize(obj);
    return result.success;
}

export const Created = {
  defaultValue: createdDefaultValue,
  serialize: createdSerialize,
  serializeWithContext: createdSerializeWithContext,
  deserialize: createdDeserialize,
  deserializeWithContext: createdDeserializeWithContext,
  validateFields: createdValidateFields,
  hasShape: createdHasShape,
  is: createdIs
} as const;