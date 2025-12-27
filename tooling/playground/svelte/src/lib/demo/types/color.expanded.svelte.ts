import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
/** import macro {Gigaform} from "@playground/macro"; */


export interface Color {
    red: number;
    green: number;
    blue: number;
}

export function colorDefaultValue(): Color {
    return {
        red: 0,
        green: 0,
        blue: 0
    } as Color;
}

export function colorSerialize(value: Color): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(colorSerializeWithContext(value, ctx));
}
export function colorSerializeWithContext(value: Color, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Color"}`,
        __id
    };
    result[`${"red"}`] = value.red;
    result[`${"green"}`] = value.green;
    result[`${"blue"}`] = value.blue;
    return result;
}

export function colorDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Color } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = colorDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Color.deserialize: root cannot be a forward reference"
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
export function colorDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Color | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Color"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"red"}` in obj)) {
        errors.push({
            field: `${"red"}`,
            message: "missing required field"
        });
    }
    if (!(`${"green"}` in obj)) {
        errors.push({
            field: `${"green"}`,
            message: "missing required field"
        });
    }
    if (!(`${"blue"}` in obj)) {
        errors.push({
            field: `${"blue"}`,
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
        const __raw_red = obj[`${"red"}`] as number;
        instance.red = __raw_red;
    }
    {
        const __raw_green = obj[`${"green"}`] as number;
        instance.green = __raw_green;
    }
    {
        const __raw_blue = obj[`${"blue"}`] as number;
        instance.blue = __raw_blue;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Color;
}
export function colorValidateField<K extends keyof Color>(_field: K, _value: Color[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function colorValidateFields(_partial: Partial<Color>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function colorHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"red" in o && "green" in o && "blue" in o';
}
export function colorIs(obj: unknown): obj is Color {
    if (!colorHasShape(obj)) {
        return false;
    }
    const result = colorDeserialize(obj);
    return result.success;
}

export const Color = {
  defaultValue: colorDefaultValue,
  serialize: colorSerialize,
  serializeWithContext: colorSerializeWithContext,
  deserialize: colorDeserialize,
  deserializeWithContext: colorDeserializeWithContext,
  validateFields: colorValidateFields,
  hasShape: colorHasShape,
  is: colorIs
} as const;