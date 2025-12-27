import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { directionHueSerializeWithContext } from "./direction-hue.svelte";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
/** import macro {Gigaform} from "@playground/macro"; */

import type { DirectionHue } from './direction-hue.svelte';


export interface Custom {
    mappings: Array<DirectionHue>;
}

export function customDefaultValue(): Custom {
    return {
        mappings: []
    } as Custom;
}

export function customSerialize(value: Custom): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(customSerializeWithContext(value, ctx));
}
export function customSerializeWithContext(value: Custom, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Custom"}`,
        __id
    };
    result[`${"mappings"}`] = value.mappings.map((item)=>directionHueSerializeWithContext(item, ctx));
    return result;
}

export function customDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Custom } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = customDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Custom.deserialize: root cannot be a forward reference"
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
export function customDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Custom | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Custom"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"mappings"}` in obj)) {
        errors.push({
            field: `${"mappings"}`,
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
        const __raw_mappings = obj[`${"mappings"}`] as Array<DirectionHue>;
        if (Array.isArray(__raw_mappings)) {
            instance.mappings = __raw_mappings as DirectionHue[];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Custom;
}
export function customValidateField<K extends keyof Custom>(_field: K, _value: Custom[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function customValidateFields(_partial: Partial<Custom>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function customHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"mappings" in o';
}
export function customIs(obj: unknown): obj is Custom {
    if (!customHasShape(obj)) {
        return false;
    }
    const result = customDeserialize(obj);
    return result.success;
}

export const Custom = {
  defaultValue: customDefaultValue,
  serialize: customSerialize,
  serializeWithContext: customSerializeWithContext,
  deserialize: customDeserialize,
  deserializeWithContext: customDeserializeWithContext,
  validateFields: customValidateFields,
  hasShape: customHasShape,
  is: customIs
} as const;