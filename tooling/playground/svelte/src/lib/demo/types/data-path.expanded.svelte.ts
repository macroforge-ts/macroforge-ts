import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
/** import macro {Gigaform} from "@playground/macro"; */


export interface DataPath {
    path: Array<string>;
    formatter: string | null;
}

export function dataPathDefaultValue(): DataPath {
    return {
        path: [],
        formatter: null
    } as DataPath;
}

export function dataPathSerialize(value: DataPath): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(dataPathSerializeWithContext(value, ctx));
}
export function dataPathSerializeWithContext(value: DataPath, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"DataPath"}`,
        __id
    };
    result[`${"path"}`] = value.path;
    result[`${"formatter"}`] = value.formatter;
    return result;
}

export function dataPathDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: DataPath } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = dataPathDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "DataPath.deserialize: root cannot be a forward reference"
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
export function dataPathDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): DataPath | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"DataPath"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"path"}` in obj)) {
        errors.push({
            field: `${"path"}`,
            message: "missing required field"
        });
    }
    if (!(`${"formatter"}` in obj)) {
        errors.push({
            field: `${"formatter"}`,
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
        const __raw_path = obj[`${"path"}`] as Array<string>;
        if (Array.isArray(__raw_path)) {
            instance.path = __raw_path as string[];
        }
    }
    {
        const __raw_formatter = obj[`${"formatter"}`] as string | null;
        instance.formatter = __raw_formatter;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as DataPath;
}
export function dataPathValidateField<K extends keyof DataPath>(_field: K, _value: DataPath[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function dataPathValidateFields(_partial: Partial<DataPath>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function dataPathHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"path" in o && "formatter" in o';
}
export function dataPathIs(obj: unknown): obj is DataPath {
    if (!dataPathHasShape(obj)) {
        return false;
    }
    const result = dataPathDeserialize(obj);
    return result.success;
}

export const DataPath = {
  defaultValue: dataPathDefaultValue,
  serialize: dataPathSerialize,
  serializeWithContext: dataPathSerializeWithContext,
  deserialize: dataPathDeserialize,
  deserializeWithContext: dataPathDeserializeWithContext,
  validateFields: dataPathValidateFields,
  hasShape: dataPathHasShape,
  is: dataPathIs
} as const;