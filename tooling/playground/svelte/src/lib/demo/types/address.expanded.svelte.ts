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


export interface Address {
    
    street: string;
    
    city: string;
    
    state: string;
    
    zipcode: string;
}

export function addressDefaultValue(): Address {
    return {
        street: "",
        city: "",
        state: "",
        zipcode: ""
    } as Address;
}

export function addressSerialize(value: Address): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(addressSerializeWithContext(value, ctx));
}
export function addressSerializeWithContext(value: Address, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Address"}`,
        __id
    };
    result[`${"street"}`] = value.street;
    result[`${"city"}`] = value.city;
    result[`${"state"}`] = value.state;
    result[`${"zipcode"}`] = value.zipcode;
    return result;
}

export function addressDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Address } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = addressDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Address.deserialize: root cannot be a forward reference"
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
export function addressDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Address | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Address"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"street"}` in obj)) {
        errors.push({
            field: `${"street"}`,
            message: "missing required field"
        });
    }
    if (!(`${"city"}` in obj)) {
        errors.push({
            field: `${"city"}`,
            message: "missing required field"
        });
    }
    if (!(`${"state"}` in obj)) {
        errors.push({
            field: `${"state"}`,
            message: "missing required field"
        });
    }
    if (!(`${"zipcode"}` in obj)) {
        errors.push({
            field: `${"zipcode"}`,
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
        const __raw_street = obj[`${"street"}`] as string;
        if (__raw_street.trim().length === 0) {
            errors.push({
                field: "street",
                message: "Address.street must not be empty"
            });
        }
        instance.street = __raw_street;
    }
    {
        const __raw_city = obj[`${"city"}`] as string;
        if (__raw_city.trim().length === 0) {
            errors.push({
                field: "city",
                message: "Address.city must not be empty"
            });
        }
        instance.city = __raw_city;
    }
    {
        const __raw_state = obj[`${"state"}`] as string;
        if (__raw_state.trim().length === 0) {
            errors.push({
                field: "state",
                message: "Address.state must not be empty"
            });
        }
        instance.state = __raw_state;
    }
    {
        const __raw_zipcode = obj[`${"zipcode"}`] as string;
        if (__raw_zipcode.trim().length === 0) {
            errors.push({
                field: "zipcode",
                message: "Address.zipcode must not be empty"
            });
        }
        instance.zipcode = __raw_zipcode;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Address;
}
export function addressValidateField<K extends keyof Address>(_field: K, _value: Address[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"street"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "street",
                message: "Address.street must not be empty"
            });
        }
    }
    if (_field === `${"city"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "city",
                message: "Address.city must not be empty"
            });
        }
    }
    if (_field === `${"state"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "state",
                message: "Address.state must not be empty"
            });
        }
    }
    if (_field === `${"zipcode"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "zipcode",
                message: "Address.zipcode must not be empty"
            });
        }
    }
    return errors;
}
export function addressValidateFields(_partial: Partial<Address>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"street"}` in _partial && _partial.street !== undefined) {
        const __val = _partial.street as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "street",
                message: "Address.street must not be empty"
            });
        }
    }
    if (`${"city"}` in _partial && _partial.city !== undefined) {
        const __val = _partial.city as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "city",
                message: "Address.city must not be empty"
            });
        }
    }
    if (`${"state"}` in _partial && _partial.state !== undefined) {
        const __val = _partial.state as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "state",
                message: "Address.state must not be empty"
            });
        }
    }
    if (`${"zipcode"}` in _partial && _partial.zipcode !== undefined) {
        const __val = _partial.zipcode as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "zipcode",
                message: "Address.zipcode must not be empty"
            });
        }
    }
    return errors;
}
export function addressHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"street" in o && "city" in o && "state" in o && "zipcode" in o';
}
export function addressIs(obj: unknown): obj is Address {
    if (!addressHasShape(obj)) {
        return false;
    }
    const result = addressDeserialize(obj);
    return result.success;
}

export function addressFromFormData(formData: FormData): Exit<Address, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    obj.street = formData.get(`${"street"}`) ?? "";
    obj.city = formData.get(`${"city"}`) ?? "";
    obj.state = formData.get(`${"state"}`) ?? "";
    obj.zipcode = formData.get(`${"zipcode"}`) ?? "";
    return toExit("addressDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: Address;
    readonly errors: AddressErrors;
    readonly tainted: AddressTainted;
    readonly fields: AddressFieldControllers;
    validate(): Exit<Address, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<Address>): void;
}
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
 }; $MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
 }; export function addressCreateForm(overrides: Partial<Address>): AddressGigaform {}
let data = $state({
    ...addressDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as AddressErrors);
let tainted = $state<$MfPh3>({} as AddressTainted);
const fields = {} as AddressFieldControllers;
fields.street = {
    label: `${"street"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.city = {
    label: `${"city"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.state = {
    label: `${"state"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.zipcode = {
    label: `${"zipcode"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
function validate(): Exit<Address, Array<{
    field: string;
    message: string;
}>> {
    return toExit("addressDeserialize(data)");
    data = {
        ...addressDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const Address = {
  defaultValue: addressDefaultValue,
  serialize: addressSerialize,
  serializeWithContext: addressSerializeWithContext,
  deserialize: addressDeserialize,
  deserializeWithContext: addressDeserializeWithContext,
  validateFields: addressValidateFields,
  hasShape: addressHasShape,
  is: addressIs,
  fromFormData: addressFromFormData,
  createForm: addressCreateForm
} as const;