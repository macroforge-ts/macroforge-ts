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


export interface EmailParts {
    
    local: string;
    
    domainName: string;
    
    topLevelDomain: string;
}

export function emailPartsDefaultValue(): EmailParts {
    return {
        local: "",
        domainName: "",
        topLevelDomain: ""
    } as EmailParts;
}

export function emailPartsSerialize(value: EmailParts): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(emailPartsSerializeWithContext(value, ctx));
}
export function emailPartsSerializeWithContext(value: EmailParts, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"EmailParts"}`,
        __id
    };
    result[`${"local"}`] = value.local;
    result[`${"domainName"}`] = value.domainName;
    result[`${"topLevelDomain"}`] = value.topLevelDomain;
    return result;
}

export function emailPartsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: EmailParts } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = emailPartsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "EmailParts.deserialize: root cannot be a forward reference"
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
export function emailPartsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): EmailParts | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"EmailParts"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"local"}` in obj)) {
        errors.push({
            field: `${"local"}`,
            message: "missing required field"
        });
    }
    if (!(`${"domainName"}` in obj)) {
        errors.push({
            field: `${"domainName"}`,
            message: "missing required field"
        });
    }
    if (!(`${"topLevelDomain"}` in obj)) {
        errors.push({
            field: `${"topLevelDomain"}`,
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
        const __raw_local = obj[`${"local"}`] as string;
        if (__raw_local.trim().length === 0) {
            errors.push({
                field: "local",
                message: "EmailParts.local must not be empty"
            });
        }
        instance.local = __raw_local;
    }
    {
        const __raw_domainName = obj[`${"domainName"}`] as string;
        if (__raw_domainName.trim().length === 0) {
            errors.push({
                field: "domainName",
                message: "EmailParts.domainName must not be empty"
            });
        }
        instance.domainName = __raw_domainName;
    }
    {
        const __raw_topLevelDomain = obj[`${"topLevelDomain"}`] as string;
        if (__raw_topLevelDomain.trim().length === 0) {
            errors.push({
                field: "topLevelDomain",
                message: "EmailParts.topLevelDomain must not be empty"
            });
        }
        instance.topLevelDomain = __raw_topLevelDomain;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as EmailParts;
}
export function emailPartsValidateField<K extends keyof EmailParts>(_field: K, _value: EmailParts[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"local"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "local",
                message: "EmailParts.local must not be empty"
            });
        }
    }
    if (_field === `${"domainName"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "domainName",
                message: "EmailParts.domainName must not be empty"
            });
        }
    }
    if (_field === `${"topLevelDomain"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "topLevelDomain",
                message: "EmailParts.topLevelDomain must not be empty"
            });
        }
    }
    return errors;
}
export function emailPartsValidateFields(_partial: Partial<EmailParts>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"local"}` in _partial && _partial.local !== undefined) {
        const __val = _partial.local as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "local",
                message: "EmailParts.local must not be empty"
            });
        }
    }
    if (`${"domainName"}` in _partial && _partial.domainName !== undefined) {
        const __val = _partial.domainName as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "domainName",
                message: "EmailParts.domainName must not be empty"
            });
        }
    }
    if (`${"topLevelDomain"}` in _partial && _partial.topLevelDomain !== undefined) {
        const __val = _partial.topLevelDomain as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "topLevelDomain",
                message: "EmailParts.topLevelDomain must not be empty"
            });
        }
    }
    return errors;
}
export function emailPartsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"local" in o && "domainName" in o && "topLevelDomain" in o';
}
export function emailPartsIs(obj: unknown): obj is EmailParts {
    if (!emailPartsHasShape(obj)) {
        return false;
    }
    const result = emailPartsDeserialize(obj);
    return result.success;
}

export function emailPartsFromFormData(formData: FormData): Exit<EmailParts, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    obj.local = formData.get(`${"local"}`) ?? "";
    obj.domainName = formData.get(`${"domainName"}`) ?? "";
    obj.topLevelDomain = formData.get(`${"topLevelDomain"}`) ?? "";
    return toExit("emailPartsDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: EmailParts;
    readonly errors: EmailPartsErrors;
    readonly tainted: EmailPartsTainted;
    readonly fields: EmailPartsFieldControllers;
    validate(): Exit<EmailParts, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<EmailParts>): void;
}
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
 }; $MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
 }; export function emailPartsCreateForm(overrides: Partial<EmailParts>): EmailPartsGigaform {}
let data = $state({
    ...emailPartsDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as EmailPartsErrors);
let tainted = $state<$MfPh3>({} as EmailPartsTainted);
const fields = {} as EmailPartsFieldControllers;
fields.local = {
    label: `${"local"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.domainName = {
    label: `${"domainName"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.topLevelDomain = {
    label: `${"topLevelDomain"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
function validate(): Exit<EmailParts, Array<{
    field: string;
    message: string;
}>> {
    return toExit("emailPartsDeserialize(data)");
    data = {
        ...emailPartsDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const EmailParts = {
  defaultValue: emailPartsDefaultValue,
  serialize: emailPartsSerialize,
  serializeWithContext: emailPartsSerializeWithContext,
  deserialize: emailPartsDeserialize,
  deserializeWithContext: emailPartsDeserializeWithContext,
  validateFields: emailPartsValidateFields,
  hasShape: emailPartsHasShape,
  is: emailPartsIs,
  fromFormData: emailPartsFromFormData,
  createForm: emailPartsCreateForm
} as const;