import { activityTypeDefaultValue } from "./activity-type.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { activityTypeSerializeWithContext } from "./activity-type.svelte";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { activityTypeDeserializeWithContext } from "./activity-type.svelte";
/** import macro {Gigaform} from "@playground/macro"; */

import type { ActivityType } from './activity-type.svelte';
import type { Actor } from './actor.svelte';
import type { Target } from './target.svelte';


export interface Did {
    
    in: string | Actor;
    
    out: string | Target;
    id: string;
    activityType: ActivityType;
    createdAt: string;
    metadata: string | null;
}

export function didDefaultValue(): Did {
    return {
        in: "",
        out: "",
        id: "",
        activityType: activityTypeDefaultValue(),
        createdAt: "",
        metadata: null
    } as Did;
}

export function didSerialize(value: Did): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(didSerializeWithContext(value, ctx));
}
export function didSerializeWithContext(value: Did, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Did"}`,
        __id
    };
    result[`${"in"}`] = value.in;
    result[`${"out"}`] = value.out;
    result[`${"id"}`] = value.id;
    result[`${"activityType"}`] = activityTypeSerializeWithContext(value.activityType, ctx);
    result[`${"createdAt"}`] = value.createdAt;
    result[`${"metadata"}`] = value.metadata;
    return result;
}

export function didDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Did } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = didDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Did.deserialize: root cannot be a forward reference"
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
export function didDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Did | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Did"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"in"}` in obj)) {
        errors.push({
            field: `${"in"}`,
            message: "missing required field"
        });
    }
    if (!(`${"out"}` in obj)) {
        errors.push({
            field: `${"out"}`,
            message: "missing required field"
        });
    }
    if (!(`${"id"}` in obj)) {
        errors.push({
            field: `${"id"}`,
            message: "missing required field"
        });
    }
    if (!(`${"activityType"}` in obj)) {
        errors.push({
            field: `${"activityType"}`,
            message: "missing required field"
        });
    }
    if (!(`${"createdAt"}` in obj)) {
        errors.push({
            field: `${"createdAt"}`,
            message: "missing required field"
        });
    }
    if (!(`${"metadata"}` in obj)) {
        errors.push({
            field: `${"metadata"}`,
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
        const __raw_in = obj[`${"in"}`] as string | Actor;
        instance.in = __raw_in;
    }
    {
        const __raw_out = obj[`${"out"}`] as string | Target;
        instance.out = __raw_out;
    }
    {
        const __raw_id = obj[`${"id"}`] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_activityType = obj[`${"activityType"}`] as ActivityType;
        {
            const __result = activityTypeDeserializeWithContext(__raw_activityType, ctx);
            ctx.assignOrDefer(instance, `${"activityType"}`, __result);
        }
    }
    {
        const __raw_createdAt = obj[`${"createdAt"}`] as string;
        instance.createdAt = __raw_createdAt;
    }
    {
        const __raw_metadata = obj[`${"metadata"}`] as string | null;
        instance.metadata = __raw_metadata;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Did;
}
export function didValidateField<K extends keyof Did>(_field: K, _value: Did[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function didValidateFields(_partial: Partial<Did>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function didHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"in" in o && "out" in o && "id" in o && "activityType" in o && "createdAt" in o && "metadata" in o';
}
export function didIs(obj: unknown): obj is Did {
    if (!didHasShape(obj)) {
        return false;
    }
    const result = didDeserialize(obj);
    return result.success;
}

export const Did = {
  defaultValue: didDefaultValue,
  serialize: didSerialize,
  serializeWithContext: didSerializeWithContext,
  deserialize: didDeserialize,
  deserializeWithContext: didDeserializeWithContext,
  validateFields: didValidateFields,
  hasShape: didHasShape,
  is: didIs
} as const;