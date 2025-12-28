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


export interface CompanyName {
    
    
    companyName: string;
}

export function companyNameDefaultValue(): CompanyName {
    return {
        companyName: ""
    } as CompanyName;
}

export function companyNameSerialize(value: CompanyName): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(companyNameSerializeWithContext(value, ctx));
}
export function companyNameSerializeWithContext(value: CompanyName, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: "CompanyName",
        __id
    };
    result.companyName = value.companyName;
    return result;
}

export function companyNameDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: CompanyName } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = companyNameDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "CompanyName.deserialize: root cannot be a forward reference"
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
export function companyNameDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): CompanyName | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"CompanyName"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("companyName" in obj)) {
        errors.push({
            field: "companyName",
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
        const __raw_companyName = obj["companyName"] as string;
        if (__raw_companyName.trim().length === 0) {
            errors.push({
                field: "companyName",
                message: "CompanyName.companyName must not be empty"
            });
        }
        instance.companyName = __raw_companyName;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as CompanyName;
}
export function companyNameValidateField<K extends keyof CompanyName>(_field: K, _value: CompanyName[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === "companyName") {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "companyName",
                message: "CompanyName.companyName must not be empty"
            });
        }
    }
    return errors;
}
export function companyNameValidateFields(_partial: Partial<CompanyName>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("companyName" in _partial && _partial.companyName !== undefined) {
        const __val = _partial.companyName as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "companyName",
                message: "CompanyName.companyName must not be empty"
            });
        }
    }
    return errors;
}
export function companyNameHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"companyName" in o';
}
export function companyNameIs(obj: unknown): obj is CompanyName {
    if (!companyNameHasShape(obj)) {
        return false;
    }
    const result = companyNameDeserialize(obj);
    return result.success;
}

export type CompanyNameErrors = {
    _errors: __gf_Option<Array<string>>;
    companyName: __gf_Option<Array<string>>;
};
export type CompanyNameTainted = {
    companyName: __gf_Option<boolean>;
};
export interface CompanyNameFieldControllers {
    readonly companyName: FieldController<string>;
}
export interface CompanyNameGigaform {
    readonly data: CompanyName;
    readonly errors: CompanyNameErrors;
    readonly tainted: CompanyNameTainted;
    readonly fields: CompanyNameFieldControllers;
    validate(): Exit<CompanyName, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<CompanyName>): void;
}
export function companyNameCreateForm(overrides?: Partial<CompanyName>): CompanyNameGigaform {
    let data = $state({
        ...companyNameDefaultValue(),
        ...overrides
    });
    let errors = $state<CompanyNameErrors>({
        _errors: optionNone(),
        companyName: optionNone()
    } as CompanyNameErrors);
    let tainted = $state<CompanyNameTainted>({
        companyName: optionNone()
    } as CompanyNameTainted);
    const fields = {
        companyName: {
            path: [
                "companyName"
            ] as const,
            name: "companyName",
            constraints: {
                required: true
            },
            label: "Company Name",
            get: ()=>data.companyName,
            set: (value: string)=>{
                data.companyName = value;
            },
            transform: (value: string): string =>value,
            getError: ()=>errors.companyName,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.companyName = value;
            },
            getTainted: ()=>tainted.companyName,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.companyName = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = companyNameValidateField("companyName", data.companyName);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        }
    } as CompanyNameFieldControllers;
    const __gf_getter_hint = "get data() set data(v) get errors() set errors(v) get tainted() set tainted(v)";
    const __gf_validate_hint = ".map((e: { field: string; message: string }) => e.message)";
    function validate(): Exit<CompanyName, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(companyNameDeserialize(data));
    }
    function reset(newOverrides?: Partial<CompanyName>): void {
        data = {
            ...companyNameDefaultValue(),
            ...newOverrides
        };
        errors = {
            _errors: optionNone(),
            companyName: optionNone()
        };
        tainted = {
            companyName: optionNone()
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
export function companyNameFromFormData(formData: FormData): Exit<CompanyName, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    const __gf_exit_hint = "Exit<CompanyName, Array<{ field: string; message: string }>>";
    obj.companyName = formData.get(`${"companyName"}`) ?? "";
    return toExit(companyNameDeserialize(obj));
}

export const CompanyName = {
  defaultValue: companyNameDefaultValue,
  serialize: companyNameSerialize,
  serializeWithContext: companyNameSerializeWithContext,
  deserialize: companyNameDeserialize,
  deserializeWithContext: companyNameDeserializeWithContext,
  validateFields: companyNameValidateFields,
  hasShape: companyNameHasShape,
  is: companyNameIs,
  createForm: companyNameCreateForm,
  fromFormData: companyNameFromFormData
} as const;