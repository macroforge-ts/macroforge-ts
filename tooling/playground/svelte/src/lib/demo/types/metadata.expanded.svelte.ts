import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
/** import macro {Gigaform} from "@playground/macro"; */


export interface Metadata {
    createdAt: string;
    lastLogin: string | null;
    isActive: boolean;
    roles: Array<string>;
}

export function metadataDefaultValue(): Metadata {
    return {
        createdAt: "",
        lastLogin: null,
        isActive: false,
        roles: []
    } as Metadata;
}

export function metadataSerialize(value: Metadata): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(metadataSerializeWithContext(value, ctx));
}
export function metadataSerializeWithContext(value: Metadata, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Metadata"}`,
        __id
    };
    result[`${"createdAt"}`] = value.createdAt;
    result[`${"lastLogin"}`] = value.lastLogin;
    result[`${"isActive"}`] = value.isActive;
    result[`${"roles"}`] = value.roles;
    return result;
}

export function metadataDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Metadata } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = metadataDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Metadata.deserialize: root cannot be a forward reference"
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
export function metadataDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Metadata | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Metadata"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"createdAt"}` in obj)) {
        errors.push({
            field: `${"createdAt"}`,
            message: "missing required field"
        });
    }
    if (!(`${"lastLogin"}` in obj)) {
        errors.push({
            field: `${"lastLogin"}`,
            message: "missing required field"
        });
    }
    if (!(`${"isActive"}` in obj)) {
        errors.push({
            field: `${"isActive"}`,
            message: "missing required field"
        });
    }
    if (!(`${"roles"}` in obj)) {
        errors.push({
            field: `${"roles"}`,
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
        const __raw_createdAt = obj[`${"createdAt"}`] as string;
        instance.createdAt = __raw_createdAt;
    }
    {
        const __raw_lastLogin = obj[`${"lastLogin"}`] as string | null;
        instance.lastLogin = __raw_lastLogin;
    }
    {
        const __raw_isActive = obj[`${"isActive"}`] as boolean;
        instance.isActive = __raw_isActive;
    }
    {
        const __raw_roles = obj[`${"roles"}`] as Array<string>;
        if (Array.isArray(__raw_roles)) {
            instance.roles = __raw_roles as string[];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Metadata;
}
export function metadataValidateField<K extends keyof Metadata>(_field: K, _value: Metadata[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function metadataValidateFields(_partial: Partial<Metadata>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function metadataHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"createdAt" in o && "lastLogin" in o && "isActive" in o && "roles" in o';
}
export function metadataIs(obj: unknown): obj is Metadata {
    if (!metadataHasShape(obj)) {
        return false;
    }
    const result = metadataDeserialize(obj);
    return result.success;
}

export const Metadata = {
  defaultValue: metadataDefaultValue,
  serialize: metadataSerialize,
  serializeWithContext: metadataSerializeWithContext,
  deserialize: metadataDeserialize,
  deserializeWithContext: metadataDeserializeWithContext,
  validateFields: metadataValidateFields,
  hasShape: metadataHasShape,
  is: metadataIs
} as const;