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


export interface PhoneNumber {
    
    main: boolean;
    
    
    phoneType: string;
    
    
    number: string;
    
    canText: boolean;
    
    canCall: boolean;
}

export function phoneNumberDefaultValue(): PhoneNumber {
    return {
        main: false,
        phoneType: "",
        number: "",
        canText: false,
        canCall: false
    } as PhoneNumber;
}

export function phoneNumberSerialize(value: PhoneNumber): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(phoneNumberSerializeWithContext(value, ctx));
}
export function phoneNumberSerializeWithContext(value: PhoneNumber, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"PhoneNumber"}`,
        __id
    };
    result[`${"main"}`] = value.main;
    result[`${"phoneType"}`] = value.phoneType;
    result[`${"number"}`] = value.number;
    result[`${"canText"}`] = value.canText;
    result[`${"canCall"}`] = value.canCall;
    return result;
}

export function phoneNumberDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: PhoneNumber } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = phoneNumberDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "PhoneNumber.deserialize: root cannot be a forward reference"
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
export function phoneNumberDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): PhoneNumber | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"PhoneNumber"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"main"}` in obj)) {
        errors.push({
            field: `${"main"}`,
            message: "missing required field"
        });
    }
    if (!(`${"phoneType"}` in obj)) {
        errors.push({
            field: `${"phoneType"}`,
            message: "missing required field"
        });
    }
    if (!(`${"number"}` in obj)) {
        errors.push({
            field: `${"number"}`,
            message: "missing required field"
        });
    }
    if (!(`${"canText"}` in obj)) {
        errors.push({
            field: `${"canText"}`,
            message: "missing required field"
        });
    }
    if (!(`${"canCall"}` in obj)) {
        errors.push({
            field: `${"canCall"}`,
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
        const __raw_main = obj[`${"main"}`] as boolean;
        instance.main = __raw_main;
    }
    {
        const __raw_phoneType = obj[`${"phoneType"}`] as string;
        if (__raw_phoneType.trim().length === 0) {
            errors.push({
                field: "phoneType",
                message: "PhoneNumber.phoneType must not be empty"
            });
        }
        instance.phoneType = __raw_phoneType;
    }
    {
        const __raw_number = obj[`${"number"}`] as string;
        if (__raw_number.trim().length === 0) {
            errors.push({
                field: "number",
                message: "PhoneNumber.number must not be empty"
            });
        }
        instance.number = __raw_number;
    }
    {
        const __raw_canText = obj[`${"canText"}`] as boolean;
        instance.canText = __raw_canText;
    }
    {
        const __raw_canCall = obj[`${"canCall"}`] as boolean;
        instance.canCall = __raw_canCall;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as PhoneNumber;
}
export function phoneNumberValidateField<K extends keyof PhoneNumber>(_field: K, _value: PhoneNumber[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"phoneType"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "phoneType",
                message: "PhoneNumber.phoneType must not be empty"
            });
        }
    }
    if (_field === `${"number"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "number",
                message: "PhoneNumber.number must not be empty"
            });
        }
    }
    return errors;
}
export function phoneNumberValidateFields(_partial: Partial<PhoneNumber>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"phoneType"}` in _partial && _partial.phoneType !== undefined) {
        const __val = _partial.phoneType as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "phoneType",
                message: "PhoneNumber.phoneType must not be empty"
            });
        }
    }
    if (`${"number"}` in _partial && _partial.number !== undefined) {
        const __val = _partial.number as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "number",
                message: "PhoneNumber.number must not be empty"
            });
        }
    }
    return errors;
}
export function phoneNumberHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"main" in o && "phoneType" in o && "number" in o && "canText" in o && "canCall" in o';
}
export function phoneNumberIs(obj: unknown): obj is PhoneNumber {
    if (!phoneNumberHasShape(obj)) {
        return false;
    }
    const result = phoneNumberDeserialize(obj);
    return result.success;
}

export function phoneNumberFromFormData(formData: FormData): Exit<PhoneNumber, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    {
        const mainVal = formData.get(`${"main"}`);
        obj.main = mainVal === "true" || mainVal === "on" || mainVal === "1";
    }
    obj.phoneType = formData.get(`${"phoneType"}`) ?? "";
    obj.number = formData.get(`${"number"}`) ?? "";
    {
        const canTextVal = formData.get(`${"canText"}`);
        obj.canText = canTextVal === "true" || canTextVal === "on" || canTextVal === "1";
    }
    {
        const canCallVal = formData.get(`${"canCall"}`);
        obj.canCall = canCallVal === "true" || canCallVal === "on" || canCallVal === "1";
    }
    return toExit("phoneNumberDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: PhoneNumber;
    readonly errors: PhoneNumberErrors;
    readonly tainted: PhoneNumberTainted;
    readonly fields: PhoneNumberFieldControllers;
    validate(): Exit<PhoneNumber, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<PhoneNumber>): void;
}
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
 }; $MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
 }; export function phoneNumberCreateForm(overrides: Partial<PhoneNumber>): PhoneNumberGigaform {}
let data = $state({
    ...phoneNumberDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as PhoneNumberErrors);
let tainted = $state<$MfPh3>({} as PhoneNumberTainted);
const fields = {} as PhoneNumberFieldControllers;
fields.main = {
    label: `${"main"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.phoneType = {
    label: `${"phoneType"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.number = {
    label: `${"number"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.canText = {
    label: `${"canText"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.canCall = {
    label: `${"canCall"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
function validate(): Exit<PhoneNumber, Array<{
    field: string;
    message: string;
}>> {
    return toExit("phoneNumberDeserialize(data)");
    data = {
        ...phoneNumberDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const PhoneNumber = {
  defaultValue: phoneNumberDefaultValue,
  serialize: phoneNumberSerialize,
  serializeWithContext: phoneNumberSerializeWithContext,
  deserialize: phoneNumberDeserialize,
  deserializeWithContext: phoneNumberDeserializeWithContext,
  validateFields: phoneNumberValidateFields,
  hasShape: phoneNumberHasShape,
  is: phoneNumberIs,
  fromFormData: phoneNumberFromFormData,
  createForm: phoneNumberCreateForm
} as const;