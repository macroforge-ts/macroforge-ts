import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";
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
        __type: "Email",
        __id
    };
    result.canEmail = value.canEmail;
    result.emailString = value.emailString;
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
    if (!("canEmail" in obj)) {
        errors.push({
            field: "canEmail",
            message: "missing required field"
        });
    }
    if (!("emailString" in obj)) {
        errors.push({
            field: "emailString",
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
        const __raw_canEmail = obj["canEmail"] as boolean;
        instance.canEmail = __raw_canEmail;
    }
    {
        const __raw_emailString = obj["emailString"] as string;
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
    if (_field === "emailString") {
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
    if ("emailString" in _partial && _partial.emailString !== undefined) {
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

export type EmailErrors = {
    _errors: __gf_Option<Array<string>>;
    canEmail: __gf_Option<Array<string>>;
    emailString: __gf_Option<Array<string>>;
};
export type EmailTainted = {
    canEmail: __gf_Option<boolean>;
    emailString: __gf_Option<boolean>;
};
export interface EmailFieldControllers {
    readonly canEmail: FieldController<boolean>;
    readonly emailString: FieldController<string>;
}
export interface EmailGigaform {
    readonly data: Email;
    readonly errors: EmailErrors;
    readonly tainted: EmailTainted;
    readonly fields: EmailFieldControllers;
    validate(): Exit<Email, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<Email>): void;
}
export function emailCreateForm(overrides?: Partial<Email>): EmailGigaform {
    let data = $state({
        ...emailDefaultValue(),
        ...overrides
    });
    let errors = $state<EmailErrors>({
        _errors: optionNone(),
        canEmail: optionNone(),
        emailString: optionNone()
    } as EmailErrors);
    let tainted = $state<EmailTainted>({
        canEmail: optionNone(),
        emailString: optionNone()
    } as EmailTainted);
    const fields = {
        canEmail: {
            path: [
                "canEmail"
            ] as const,
            name: "canEmail",
            constraints: {
                required: true
            },
            label: "Can Email",
            get: ()=>data.canEmail,
            set: (value: boolean)=>{
                data.canEmail = value;
            },
            transform: (value: boolean): boolean =>value,
            getError: ()=>errors.canEmail,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.canEmail = value;
            },
            getTainted: ()=>tainted.canEmail,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.canEmail = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = emailValidateField("canEmail", data.canEmail);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        emailString: {
            path: [
                "emailString"
            ] as const,
            name: "emailString",
            constraints: {
                required: true,
                type: "email"
            },
            label: "Email",
            get: ()=>data.emailString,
            set: (value: string)=>{
                data.emailString = value;
            },
            transform: (value: string): string =>value,
            getError: ()=>errors.emailString,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.emailString = value;
            },
            getTainted: ()=>tainted.emailString,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.emailString = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = emailValidateField("emailString", data.emailString);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        }
    } as EmailFieldControllers;
    const __gf_getter_hint = "get data() set data(v) get errors() set errors(v) get tainted() set tainted(v)";
    const __gf_validate_hint = ".map((e: { field: string; message: string }) => e.message)";
    function validate(): Exit<Email, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(emailDeserialize(data));
    }
    function reset(newOverrides?: Partial<Email>): void {
        data = {
            ...emailDefaultValue(),
            ...newOverrides
        };
        errors = {
            _errors: optionNone(),
            canEmail: optionNone(),
            emailString: optionNone()
        };
        tainted = {
            canEmail: optionNone(),
            emailString: optionNone()
        };
    }
    return {
        get data () {
            return data;
        },
        set data (v){
            data = v;
        },
        get errors () {
            return errors;
        },
        set errors (v){
            errors = v;
        },
        get tainted () {
            return tainted;
        },
        set tainted (v){
            tainted = v;
        },
        fields,
        validate,
        reset
    };
}
export function emailFromFormData(formData: FormData): Exit<Email, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    const __gf_exit_hint = "Exit<Email, Array<{ field: string; message: string }>>";
    {
        const canEmailVal = formData.get(`${"canEmail"}`);
        obj.canEmail = canEmailVal === "true" || canEmailVal === "on" || canEmailVal === "1";
    }
    obj.emailString = formData.get(`${"emailString"}`) ?? "";
    return toExit(emailDeserialize(obj));
}

export const Email = {
  defaultValue: emailDefaultValue,
  serialize: emailSerialize,
  serializeWithContext: emailSerializeWithContext,
  deserialize: emailDeserialize,
  deserializeWithContext: emailDeserializeWithContext,
  validateFields: emailValidateFields,
  hasShape: emailHasShape,
  is: emailIs,
  createForm: emailCreateForm,
  fromFormData: emailFromFormData
} as const;