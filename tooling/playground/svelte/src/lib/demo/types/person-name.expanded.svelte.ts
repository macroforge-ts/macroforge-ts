import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
/** import macro {Gigaform} from "@playground/macro"; */


export interface PersonName {
    
    
    firstName: string;
    
    
    lastName: string;
}

export function personNameDefaultValue(): PersonName {
    return {
        firstName: "",
        lastName: ""
    } as PersonName;
}

export function personNameSerialize(value: PersonName): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(personNameSerializeWithContext(value, ctx));
}
export function personNameSerializeWithContext(value: PersonName, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"PersonName"}`,
        __id
    };
    result[`${"firstName"}`] = value.firstName;
    result[`${"lastName"}`] = value.lastName;
    return result;
}

export function personNameDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: PersonName } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = personNameDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "PersonName.deserialize: root cannot be a forward reference"
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
export function personNameDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): PersonName | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"PersonName"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"firstName"}` in obj)) {
        errors.push({
            field: `${"firstName"}`,
            message: "missing required field"
        });
    }
    if (!(`${"lastName"}` in obj)) {
        errors.push({
            field: `${"lastName"}`,
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
        const __raw_firstName = obj[`${"firstName"}`] as string;
        if (__raw_firstName.trim().length === 0) {
            errors.push({
                field: "firstName",
                message: "PersonName.firstName must not be empty"
            });
        }
        instance.firstName = __raw_firstName;
    }
    {
        const __raw_lastName = obj[`${"lastName"}`] as string;
        if (__raw_lastName.trim().length === 0) {
            errors.push({
                field: "lastName",
                message: "PersonName.lastName must not be empty"
            });
        }
        instance.lastName = __raw_lastName;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as PersonName;
}
export function personNameValidateField<K extends keyof PersonName>(_field: K, _value: PersonName[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"firstName"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "firstName",
                message: "PersonName.firstName must not be empty"
            });
        }
    }
    if (_field === `${"lastName"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "lastName",
                message: "PersonName.lastName must not be empty"
            });
        }
    }
    return errors;
}
export function personNameValidateFields(_partial: Partial<PersonName>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"firstName"}` in _partial && _partial.firstName !== undefined) {
        const __val = _partial.firstName as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "firstName",
                message: "PersonName.firstName must not be empty"
            });
        }
    }
    if (`${"lastName"}` in _partial && _partial.lastName !== undefined) {
        const __val = _partial.lastName as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "lastName",
                message: "PersonName.lastName must not be empty"
            });
        }
    }
    return errors;
}
export function personNameHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"firstName" in o && "lastName" in o';
}
export function personNameIs(obj: unknown): obj is PersonName {
    if (!personNameHasShape(obj)) {
        return false;
    }
    const result = personNameDeserialize(obj);
    return result.success;
}

export const PersonName = {
  defaultValue: personNameDefaultValue,
  serialize: personNameSerialize,
  serializeWithContext: personNameSerializeWithContext,
  deserialize: personNameDeserialize,
  deserializeWithContext: personNameDeserializeWithContext,
  validateFields: personNameValidateFields,
  hasShape: personNameHasShape,
  is: personNameIs
} as const;