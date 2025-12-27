import { dataPathDefaultValue } from "./data-path.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { dataPathSerializeWithContext } from "./data-path.svelte";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { dataPathDeserializeWithContext } from "./data-path.svelte";
/** import macro {Gigaform} from "@playground/macro"; */
import type { DataPath } from './data-path.svelte';


export interface ColumnConfig {
    
    heading: string;
    dataPath: DataPath;
}

export function columnConfigDefaultValue(): ColumnConfig {
    return {
        heading: "",
        dataPath: dataPathDefaultValue()
    } as ColumnConfig;
}

export function columnConfigSerialize(value: ColumnConfig): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(columnConfigSerializeWithContext(value, ctx));
}
export function columnConfigSerializeWithContext(value: ColumnConfig, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"ColumnConfig"}`,
        __id
    };
    result[`${"heading"}`] = value.heading;
    result[`${"dataPath"}`] = dataPathSerializeWithContext(value.dataPath, ctx);
    return result;
}

export function columnConfigDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: ColumnConfig } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = columnConfigDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "ColumnConfig.deserialize: root cannot be a forward reference"
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
export function columnConfigDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): ColumnConfig | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"ColumnConfig"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"heading"}` in obj)) {
        errors.push({
            field: `${"heading"}`,
            message: "missing required field"
        });
    }
    if (!(`${"dataPath"}` in obj)) {
        errors.push({
            field: `${"dataPath"}`,
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
        const __raw_heading = obj[`${"heading"}`] as string;
        if (__raw_heading.trim().length === 0) {
            errors.push({
                field: "heading",
                message: "ColumnConfig.heading must not be empty"
            });
        }
        instance.heading = __raw_heading;
    }
    {
        const __raw_dataPath = obj[`${"dataPath"}`] as DataPath;
        {
            const __result = dataPathDeserializeWithContext(__raw_dataPath, ctx);
            ctx.assignOrDefer(instance, `${"dataPath"}`, __result);
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as ColumnConfig;
}
export function columnConfigValidateField<K extends keyof ColumnConfig>(_field: K, _value: ColumnConfig[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"heading"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "heading",
                message: "ColumnConfig.heading must not be empty"
            });
        }
    }
    return errors;
}
export function columnConfigValidateFields(_partial: Partial<ColumnConfig>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"heading"}` in _partial && _partial.heading !== undefined) {
        const __val = _partial.heading as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "heading",
                message: "ColumnConfig.heading must not be empty"
            });
        }
    }
    return errors;
}
export function columnConfigHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"heading" in o && "dataPath" in o';
}
export function columnConfigIs(obj: unknown): obj is ColumnConfig {
    if (!columnConfigHasShape(obj)) {
        return false;
    }
    const result = columnConfigDeserialize(obj);
    return result.success;
}

export const ColumnConfig = {
  defaultValue: columnConfigDefaultValue,
  serialize: columnConfigSerialize,
  serializeWithContext: columnConfigSerializeWithContext,
  deserialize: columnConfigDeserialize,
  deserializeWithContext: columnConfigDeserializeWithContext,
  validateFields: columnConfigValidateFields,
  hasShape: columnConfigHasShape,
  is: columnConfigIs
} as const;