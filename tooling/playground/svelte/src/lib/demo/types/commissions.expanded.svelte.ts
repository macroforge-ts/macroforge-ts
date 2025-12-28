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


export interface Commissions {
    
    technician: string;
    
    salesRep: string;
}

export function commissionsDefaultValue(): Commissions {
    return {
        technician: "",
        salesRep: ""
    } as Commissions;
}

export function commissionsSerialize(value: Commissions): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(commissionsSerializeWithContext(value, ctx));
}
export function commissionsSerializeWithContext(value: Commissions, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: "Commissions",
        __id
    };
    result.technician = value.technician;
    result.salesRep = value.salesRep;
    return result;
}

export function commissionsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Commissions } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = commissionsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Commissions.deserialize: root cannot be a forward reference"
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
export function commissionsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Commissions | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Commissions"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("technician" in obj)) {
        errors.push({
            field: "technician",
            message: "missing required field"
        });
    }
    if (!("salesRep" in obj)) {
        errors.push({
            field: "salesRep",
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
        const __raw_technician = obj["technician"] as string;
        if (__raw_technician.trim().length === 0) {
            errors.push({
                field: "technician",
                message: "Commissions.technician must not be empty"
            });
        }
        instance.technician = __raw_technician;
    }
    {
        const __raw_salesRep = obj["salesRep"] as string;
        if (__raw_salesRep.trim().length === 0) {
            errors.push({
                field: "salesRep",
                message: "Commissions.salesRep must not be empty"
            });
        }
        instance.salesRep = __raw_salesRep;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Commissions;
}
export function commissionsValidateField<K extends keyof Commissions>(_field: K, _value: Commissions[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === "technician") {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "technician",
                message: "Commissions.technician must not be empty"
            });
        }
    }
    if (_field === "salesRep") {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "salesRep",
                message: "Commissions.salesRep must not be empty"
            });
        }
    }
    return errors;
}
export function commissionsValidateFields(_partial: Partial<Commissions>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("technician" in _partial && _partial.technician !== undefined) {
        const __val = _partial.technician as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "technician",
                message: "Commissions.technician must not be empty"
            });
        }
    }
    if ("salesRep" in _partial && _partial.salesRep !== undefined) {
        const __val = _partial.salesRep as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "salesRep",
                message: "Commissions.salesRep must not be empty"
            });
        }
    }
    return errors;
}
export function commissionsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"technician" in o && "salesRep" in o';
}
export function commissionsIs(obj: unknown): obj is Commissions {
    if (!commissionsHasShape(obj)) {
        return false;
    }
    const result = commissionsDeserialize(obj);
    return result.success;
}

export type CommissionsErrors = {
    _errors: __gf_Option<Array<string>>;
    technician: __gf_Option<Array<string>>;
    salesRep: __gf_Option<Array<string>>;
};
export type CommissionsTainted = {
    technician: __gf_Option<boolean>;
    salesRep: __gf_Option<boolean>;
};
export interface CommissionsFieldControllers {
    readonly technician: FieldController<string>;
    readonly salesRep: FieldController<string>;
}
export interface CommissionsGigaform {
    readonly data: Commissions;
    readonly errors: CommissionsErrors;
    readonly tainted: CommissionsTainted;
    readonly fields: CommissionsFieldControllers;
    validate(): Exit<Commissions, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<Commissions>): void;
}
export function commissionsCreateForm(overrides?: Partial<Commissions>): CommissionsGigaform {
    let data = $state({
        ...commissionsDefaultValue(),
        ...overrides
    });
    let errors = $state<CommissionsErrors>({
        _errors: optionNone(),
        technician: optionNone(),
        salesRep: optionNone()
    } as CommissionsErrors);
    let tainted = $state<CommissionsTainted>({
        technician: optionNone(),
        salesRep: optionNone()
    } as CommissionsTainted);
    const fields = {
        technician: {
            path: [
                "technician"
            ] as const,
            name: "technician",
            constraints: {
                required: true
            },
            get: ()=>data.technician,
            set: (value: string)=>{
                data.technician = value;
            },
            transform: (value: string): string =>value,
            getError: ()=>errors.technician,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.technician = value;
            },
            getTainted: ()=>tainted.technician,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.technician = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = commissionsValidateField("technician", data.technician);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        salesRep: {
            path: [
                "salesRep"
            ] as const,
            name: "salesRep",
            constraints: {
                required: true
            },
            get: ()=>data.salesRep,
            set: (value: string)=>{
                data.salesRep = value;
            },
            transform: (value: string): string =>value,
            getError: ()=>errors.salesRep,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.salesRep = value;
            },
            getTainted: ()=>tainted.salesRep,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.salesRep = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = commissionsValidateField("salesRep", data.salesRep);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        }
    } as CommissionsFieldControllers;
    const __gf_getter_hint = "get data() set data(v) get errors() set errors(v) get tainted() set tainted(v)";
    const __gf_validate_hint = ".map((e: { field: string; message: string }) => e.message)";
    function validate(): Exit<Commissions, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(commissionsDeserialize(data));
    }
    function reset(newOverrides?: Partial<Commissions>): void {
        data = {
            ...commissionsDefaultValue(),
            ...newOverrides
        };
        errors = {
            _errors: optionNone(),
            technician: optionNone(),
            salesRep: optionNone()
        };
        tainted = {
            technician: optionNone(),
            salesRep: optionNone()
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
export function commissionsFromFormData(formData: FormData): Exit<Commissions, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    const __gf_exit_hint = "Exit<Commissions, Array<{ field: string; message: string }>>";
    obj.technician = formData.get(`${"technician"}`) ?? "";
    obj.salesRep = formData.get(`${"salesRep"}`) ?? "";
    return toExit(commissionsDeserialize(obj));
}

export const Commissions = {
  defaultValue: commissionsDefaultValue,
  serialize: commissionsSerialize,
  serializeWithContext: commissionsSerializeWithContext,
  deserialize: commissionsDeserialize,
  deserializeWithContext: commissionsDeserializeWithContext,
  validateFields: commissionsValidateFields,
  hasShape: commissionsHasShape,
  is: commissionsIs,
  createForm: commissionsCreateForm,
  fromFormData: commissionsFromFormData
} as const;