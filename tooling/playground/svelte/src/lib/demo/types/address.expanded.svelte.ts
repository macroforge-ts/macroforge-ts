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
        __type: "Address",
        __id
    };
    result.street = value.street;
    result.city = value.city;
    result.state = value.state;
    result.zipcode = value.zipcode;
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
    if (!("street" in obj)) {
        errors.push({
            field: "street",
            message: "missing required field"
        });
    }
    if (!("city" in obj)) {
        errors.push({
            field: "city",
            message: "missing required field"
        });
    }
    if (!("state" in obj)) {
        errors.push({
            field: "state",
            message: "missing required field"
        });
    }
    if (!("zipcode" in obj)) {
        errors.push({
            field: "zipcode",
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
        const __raw_street = obj["street"] as string;
        if (__raw_street.trim().length === 0) {
            errors.push({
                field: "street",
                message: "Address.street must not be empty"
            });
        }
        instance.street = __raw_street;
    }
    {
        const __raw_city = obj["city"] as string;
        if (__raw_city.trim().length === 0) {
            errors.push({
                field: "city",
                message: "Address.city must not be empty"
            });
        }
        instance.city = __raw_city;
    }
    {
        const __raw_state = obj["state"] as string;
        if (__raw_state.trim().length === 0) {
            errors.push({
                field: "state",
                message: "Address.state must not be empty"
            });
        }
        instance.state = __raw_state;
    }
    {
        const __raw_zipcode = obj["zipcode"] as string;
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
    if (_field === "street") {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "street",
                message: "Address.street must not be empty"
            });
        }
    }
    if (_field === "city") {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "city",
                message: "Address.city must not be empty"
            });
        }
    }
    if (_field === "state") {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "state",
                message: "Address.state must not be empty"
            });
        }
    }
    if (_field === "zipcode") {
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
    if ("street" in _partial && _partial.street !== undefined) {
        const __val = _partial.street as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "street",
                message: "Address.street must not be empty"
            });
        }
    }
    if ("city" in _partial && _partial.city !== undefined) {
        const __val = _partial.city as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "city",
                message: "Address.city must not be empty"
            });
        }
    }
    if ("state" in _partial && _partial.state !== undefined) {
        const __val = _partial.state as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "state",
                message: "Address.state must not be empty"
            });
        }
    }
    if ("zipcode" in _partial && _partial.zipcode !== undefined) {
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

export type AddressErrors = {
    _errors: __gf_Option<Array<string>>;
    street: __gf_Option<Array<string>>;
    city: __gf_Option<Array<string>>;
    state: __gf_Option<Array<string>>;
    zipcode: __gf_Option<Array<string>>;
};
export type AddressTainted = {
    street: __gf_Option<boolean>;
    city: __gf_Option<boolean>;
    state: __gf_Option<boolean>;
    zipcode: __gf_Option<boolean>;
};
export interface AddressFieldControllers {
    readonly street: FieldController<string>;
    readonly city: FieldController<string>;
    readonly state: FieldController<string>;
    readonly zipcode: FieldController<string>;
}
export interface AddressGigaform {
    readonly data: Address;
    readonly errors: AddressErrors;
    readonly tainted: AddressTainted;
    readonly fields: AddressFieldControllers;
    validate(): Exit<Address, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<Address>): void;
}
export function addressCreateForm(overrides?: Partial<Address>): AddressGigaform {
    let data = $state({
        ...addressDefaultValue(),
        ...overrides
    });
    let errors = $state<AddressErrors>({
        _errors: optionNone(),
        street: optionNone(),
        city: optionNone(),
        state: optionNone(),
        zipcode: optionNone()
    } as AddressErrors);
    let tainted = $state<AddressTainted>({
        street: optionNone(),
        city: optionNone(),
        state: optionNone(),
        zipcode: optionNone()
    } as AddressTainted);
    const fields = {
        street: {
            path: [
                "street"
            ] as const,
            name: "street",
            constraints: {
                required: true
            },
            get: ()=>data.street,
            set: (value: string)=>{
                data.street = value;
            },
            transform: (value: string): string =>value,
            getError: ()=>errors.street,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.street = value;
            },
            getTainted: ()=>tainted.street,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.street = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = addressValidateField("street", data.street);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        city: {
            path: [
                "city"
            ] as const,
            name: "city",
            constraints: {
                required: true
            },
            get: ()=>data.city,
            set: (value: string)=>{
                data.city = value;
            },
            transform: (value: string): string =>value,
            getError: ()=>errors.city,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.city = value;
            },
            getTainted: ()=>tainted.city,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.city = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = addressValidateField("city", data.city);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        state: {
            path: [
                "state"
            ] as const,
            name: "state",
            constraints: {
                required: true
            },
            get: ()=>data.state,
            set: (value: string)=>{
                data.state = value;
            },
            transform: (value: string): string =>value,
            getError: ()=>errors.state,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.state = value;
            },
            getTainted: ()=>tainted.state,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.state = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = addressValidateField("state", data.state);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        zipcode: {
            path: [
                "zipcode"
            ] as const,
            name: "zipcode",
            constraints: {
                required: true
            },
            get: ()=>data.zipcode,
            set: (value: string)=>{
                data.zipcode = value;
            },
            transform: (value: string): string =>value,
            getError: ()=>errors.zipcode,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.zipcode = value;
            },
            getTainted: ()=>tainted.zipcode,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.zipcode = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = addressValidateField("zipcode", data.zipcode);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        }
    } as AddressFieldControllers;
    const __gf_getter_hint = "get data() set data(v) get errors() set errors(v) get tainted() set tainted(v)";
    const __gf_validate_hint = ".map((e: { field: string; message: string }) => e.message)";
    function validate(): Exit<Address, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(addressDeserialize(data));
    }
    function reset(newOverrides?: Partial<Address>): void {
        data = {
            ...addressDefaultValue(),
            ...newOverrides
        };
        errors = {
            _errors: optionNone(),
            street: optionNone(),
            city: optionNone(),
            state: optionNone(),
            zipcode: optionNone()
        };
        tainted = {
            street: optionNone(),
            city: optionNone(),
            state: optionNone(),
            zipcode: optionNone()
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
export function addressFromFormData(formData: FormData): Exit<Address, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    const __gf_exit_hint = "Exit<Address, Array<{ field: string; message: string }>>";
    obj.street = formData.get(`${"street"}`) ?? "";
    obj.city = formData.get(`${"city"}`) ?? "";
    obj.state = formData.get(`${"state"}`) ?? "";
    obj.zipcode = formData.get(`${"zipcode"}`) ?? "";
    return toExit(addressDeserialize(obj));
}

export const Address = {
  defaultValue: addressDefaultValue,
  serialize: addressSerialize,
  serializeWithContext: addressSerializeWithContext,
  deserialize: addressDeserialize,
  deserializeWithContext: addressDeserializeWithContext,
  validateFields: addressValidateFields,
  hasShape: addressHasShape,
  is: addressIs,
  createForm: addressCreateForm,
  fromFormData: addressFromFormData
} as const;