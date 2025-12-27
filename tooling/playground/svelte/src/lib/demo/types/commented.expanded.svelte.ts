import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
/** import macro {Gigaform} from "@playground/macro"; */


export interface Commented {
    
    comment: string;
    replyTo: string | null;
}

export function commentedDefaultValue(): Commented {
    return {
        comment: "",
        replyTo: null
    } as Commented;
}

export function commentedSerialize(value: Commented): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(commentedSerializeWithContext(value, ctx));
}
export function commentedSerializeWithContext(value: Commented, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Commented"}`,
        __id
    };
    result[`${"comment"}`] = value.comment;
    result[`${"replyTo"}`] = value.replyTo;
    return result;
}

export function commentedDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Commented } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = commentedDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Commented.deserialize: root cannot be a forward reference"
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
export function commentedDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Commented | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Commented"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"comment"}` in obj)) {
        errors.push({
            field: `${"comment"}`,
            message: "missing required field"
        });
    }
    if (!(`${"replyTo"}` in obj)) {
        errors.push({
            field: `${"replyTo"}`,
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
        const __raw_comment = obj[`${"comment"}`] as string;
        if (__raw_comment.trim().length === 0) {
            errors.push({
                field: "comment",
                message: "Commented.comment must not be empty"
            });
        }
        instance.comment = __raw_comment;
    }
    {
        const __raw_replyTo = obj[`${"replyTo"}`] as string | null;
        instance.replyTo = __raw_replyTo;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Commented;
}
export function commentedValidateField<K extends keyof Commented>(_field: K, _value: Commented[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"comment"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "comment",
                message: "Commented.comment must not be empty"
            });
        }
    }
    return errors;
}
export function commentedValidateFields(_partial: Partial<Commented>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"comment"}` in _partial && _partial.comment !== undefined) {
        const __val = _partial.comment as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "comment",
                message: "Commented.comment must not be empty"
            });
        }
    }
    return errors;
}
export function commentedHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"comment" in o && "replyTo" in o';
}
export function commentedIs(obj: unknown): obj is Commented {
    if (!commentedHasShape(obj)) {
        return false;
    }
    const result = commentedDeserialize(obj);
    return result.success;
}

export const Commented = {
  defaultValue: commentedDefaultValue,
  serialize: commentedSerialize,
  serializeWithContext: commentedSerializeWithContext,
  deserialize: commentedDeserialize,
  deserializeWithContext: commentedDeserializeWithContext,
  validateFields: commentedValidateFields,
  hasShape: commentedHasShape,
  is: commentedIs
} as const;