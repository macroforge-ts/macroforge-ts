import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
/** import macro {Gigaform} from "@playground/macro"; */


export interface Coordinates {
    lat: number;
    lng: number;
}

export function coordinatesDefaultValue(): Coordinates {
    return {
        lat: 0,
        lng: 0
    } as Coordinates;
}

export function coordinatesSerialize(value: Coordinates): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(coordinatesSerializeWithContext(value, ctx));
}
export function coordinatesSerializeWithContext(value: Coordinates, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Coordinates"}`,
        __id
    };
    result[`${"lat"}`] = value.lat;
    result[`${"lng"}`] = value.lng;
    return result;
}

export function coordinatesDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Coordinates } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = coordinatesDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Coordinates.deserialize: root cannot be a forward reference"
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
export function coordinatesDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Coordinates | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Coordinates"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"lat"}` in obj)) {
        errors.push({
            field: `${"lat"}`,
            message: "missing required field"
        });
    }
    if (!(`${"lng"}` in obj)) {
        errors.push({
            field: `${"lng"}`,
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
        const __raw_lat = obj[`${"lat"}`] as number;
        instance.lat = __raw_lat;
    }
    {
        const __raw_lng = obj[`${"lng"}`] as number;
        instance.lng = __raw_lng;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Coordinates;
}
export function coordinatesValidateField<K extends keyof Coordinates>(_field: K, _value: Coordinates[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function coordinatesValidateFields(_partial: Partial<Coordinates>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function coordinatesHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"lat" in o && "lng" in o';
}
export function coordinatesIs(obj: unknown): obj is Coordinates {
    if (!coordinatesHasShape(obj)) {
        return false;
    }
    const result = coordinatesDeserialize(obj);
    return result.success;
}

export const Coordinates = {
  defaultValue: coordinatesDefaultValue,
  serialize: coordinatesSerialize,
  serializeWithContext: coordinatesSerializeWithContext,
  deserialize: coordinatesDeserialize,
  deserializeWithContext: coordinatesDeserializeWithContext,
  validateFields: coordinatesValidateFields,
  hasShape: coordinatesHasShape,
  is: coordinatesIs
} as const;