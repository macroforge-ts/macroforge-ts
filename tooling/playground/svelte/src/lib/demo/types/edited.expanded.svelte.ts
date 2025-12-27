import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
/** import macro {Gigaform} from "@playground/macro"; */


export interface Edited {
    
    fieldName: string;
    oldValue: string | null;
    newValue: string | null;
}

export function editedDefaultValue(): Edited {
    return {
        fieldName: "",
        oldValue: null,
        newValue: null
    } as Edited;
}

export function editedSerialize(value: Edited): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(editedSerializeWithContext(value, ctx));
}
export function editedSerializeWithContext(value: Edited, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Edited"}`,
        __id
    };
    result[`${"fieldName"}`] = value.fieldName;
    result[`${"oldValue"}`] = value.oldValue;
    result[`${"newValue"}`] = value.newValue;
    return result;
}

export function editedDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Edited } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = editedDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Edited.deserialize: root cannot be a forward reference"
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
export function editedDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Edited | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Edited"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"fieldName"}` in obj)) {
        errors.push({
            field: `${"fieldName"}`,
            message: "missing required field"
        });
    }
    if (!(`${"oldValue"}` in obj)) {
        errors.push({
            field: `${"oldValue"}`,
            message: "missing required field"
        });
    }
    if (!(`${"newValue"}` in obj)) {
        errors.push({
            field: `${"newValue"}`,
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
        const __raw_fieldName = obj[`${"fieldName"}`] as string;
        if (__raw_fieldName.trim().length === 0) {
            errors.push({
                field: "fieldName",
                message: "Edited.fieldName must not be empty"
            });
        }
        instance.fieldName = __raw_fieldName;
    }
    {
        const __raw_oldValue = obj[`${"oldValue"}`] as string | null;
        instance.oldValue = __raw_oldValue;
    }
    {
        const __raw_newValue = obj[`${"newValue"}`] as string | null;
        instance.newValue = __raw_newValue;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Edited;
}
export function editedValidateField<K extends keyof Edited>(_field: K, _value: Edited[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"fieldName"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "fieldName",
                message: "Edited.fieldName must not be empty"
            });
        }
    }
    return errors;
}
export function editedValidateFields(_partial: Partial<Edited>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"fieldName"}` in _partial && _partial.fieldName !== undefined) {
        const __val = _partial.fieldName as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "fieldName",
                message: "Edited.fieldName must not be empty"
            });
        }
    }
    return errors;
}
export function editedHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"fieldName" in o && "oldValue" in o && "newValue" in o';
}
export function editedIs(obj: unknown): obj is Edited {
    if (!editedHasShape(obj)) {
        return false;
    }
    const result = editedDeserialize(obj);
    return result.success;
}

export const Edited = {
  defaultValue: editedDefaultValue,
  serialize: editedSerialize,
  serializeWithContext: editedSerializeWithContext,
  deserialize: editedDeserialize,
  deserializeWithContext: editedDeserializeWithContext,
  validateFields: editedValidateFields,
  hasShape: editedHasShape,
  is: editedIs
} as const;