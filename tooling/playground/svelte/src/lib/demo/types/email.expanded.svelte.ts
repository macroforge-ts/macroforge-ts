import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
/** import macro {Gigaform} from "@playground/macro"; */


export interface Email {
    
    canEmail: boolean;
    
    
    emailString: string;
}

export function emailDefaultValue(): Email {
    return {
        canEmail: false,
        emailString: ""
    } as Email;
}

export function emailSerialize(value: Email): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(emailSerializeWithContext(value, ctx));
}
export function emailSerializeWithContext(value: Email, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Email"}`,
        __id
    };
    result[`${"canEmail"}`] = value.canEmail;
    result[`${"emailString"}`] = value.emailString;
    return result;
}

export function emailDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Email } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = emailDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Email.deserialize: root cannot be a forward reference"
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
export function emailDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Email | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Email"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"canEmail"}` in obj)) {
        errors.push({
            field: `${"canEmail"}`,
            message: "missing required field"
        });
    }
    if (!(`${"emailString"}` in obj)) {
        errors.push({
            field: `${"emailString"}`,
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
        const __raw_canEmail = obj[`${"canEmail"}`] as boolean;
        instance.canEmail = __raw_canEmail;
    }
    {
        const __raw_emailString = obj[`${"emailString"}`] as string;
        if (__raw_emailString.trim().length === 0) {
            errors.push({
                field: "emailString",
                message: "Email.emailString must not be empty"
            });
        }
        if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__raw_emailString)) {
            errors.push({
                field: "emailString",
                message: "Email.emailString must be a valid email"
            });
        }
        instance.emailString = __raw_emailString;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Email;
}
export function emailValidateField<K extends keyof Email>(_field: K, _value: Email[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"emailString"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "emailString",
                message: "Email.emailString must not be empty"
            });
        }
        if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__val)) {
            errors.push({
                field: "emailString",
                message: "Email.emailString must be a valid email"
            });
        }
    }
    return errors;
}
export function emailValidateFields(_partial: Partial<Email>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"emailString"}` in _partial && _partial.emailString !== undefined) {
        const __val = _partial.emailString as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "emailString",
                message: "Email.emailString must not be empty"
            });
        }
        if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__val)) {
            errors.push({
                field: "emailString",
                message: "Email.emailString must be a valid email"
            });
        }
    }
    return errors;
}
export function emailHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"canEmail" in o && "emailString" in o';
}
export function emailIs(obj: unknown): obj is Email {
    if (!emailHasShape(obj)) {
        return false;
    }
    const result = emailDeserialize(obj);
    return result.success;
}

export const Email = {
  defaultValue: emailDefaultValue,
  serialize: emailSerialize,
  serializeWithContext: emailSerializeWithContext,
  deserialize: emailDeserialize,
  deserializeWithContext: emailDeserializeWithContext,
  validateFields: emailValidateFields,
  hasShape: emailHasShape,
  is: emailIs
} as const;