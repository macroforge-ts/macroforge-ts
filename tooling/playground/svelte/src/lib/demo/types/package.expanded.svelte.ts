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


export interface Package {
    
    id: string;
    
    date: string;
}

export function packageDefaultValue(): Package {
    return {
        id: "",
        date: ""
    } as Package;
}

export function packageSerialize(value: Package): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(packageSerializeWithContext(value, ctx));
}
export function packageSerializeWithContext(value: Package, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: "Package",
        __id
    };
    result.id = value.id;
    result.date = value.date;
    return result;
}

export function packageDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Package } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = packageDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Package.deserialize: root cannot be a forward reference"
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
export function packageDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Package | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Package"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("id" in obj)) {
        errors.push({
            field: "id",
            message: "missing required field"
        });
    }
    if (!("date" in obj)) {
        errors.push({
            field: "date",
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
        const __raw_id = obj["id"] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_date = obj["date"] as string;
        instance.date = __raw_date;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Package;
}
export function packageValidateField<K extends keyof Package>(_field: K, _value: Package[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function packageValidateFields(_partial: Partial<Package>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function packageHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "date" in o';
}
export function packageIs(obj: unknown): obj is Package {
    if (!packageHasShape(obj)) {
        return false;
    }
    const result = packageDeserialize(obj);
    return result.success;
}

export type PackageErrors = {
    _errors: __gf_Option<Array<string>>;
    id: __gf_Option<Array<string>>;
    date: __gf_Option<Array<string>>;
};
export type PackageTainted = {
    id: __gf_Option<boolean>;
    date: __gf_Option<boolean>;
};
export interface PackageFieldControllers {
    readonly id: FieldController<string>;
    readonly date: FieldController<string>;
}
export interface PackageGigaform {
    readonly data: Package;
    readonly errors: PackageErrors;
    readonly tainted: PackageTainted;
    readonly fields: PackageFieldControllers;
    validate(): Exit<Package, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<Package>): void;
}
export function packageCreateForm(overrides?: Partial<Package>): PackageGigaform {
    let data = $state({
        ...packageDefaultValue(),
        ...overrides
    });
    let errors = $state<PackageErrors>({
        _errors: optionNone(),
        id: optionNone(),
        date: optionNone()
    } as PackageErrors);
    let tainted = $state<PackageTainted>({
        id: optionNone(),
        date: optionNone()
    } as PackageTainted);
    const fields = {
        id: {
            path: [
                "id"
            ] as const,
            name: "id",
            constraints: {
                required: true
            },
            get: ()=>data.id,
            set: (value: string)=>{
                data.id = value;
            },
            transform: (value: string): string =>value,
            getError: ()=>errors.id,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.id = value;
            },
            getTainted: ()=>tainted.id,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.id = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = packageValidateField("id", data.id);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        date: {
            path: [
                "date"
            ] as const,
            name: "date",
            constraints: {
                required: true
            },
            label: "Date",
            get: ()=>data.date,
            set: (value: string)=>{
                data.date = value;
            },
            transform: (value: string): string =>value,
            getError: ()=>errors.date,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.date = value;
            },
            getTainted: ()=>tainted.date,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.date = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = packageValidateField("date", data.date);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        }
    } as PackageFieldControllers;
    const __gf_getter_hint = "get data() set data(v) get errors() set errors(v) get tainted() set tainted(v)";
    const __gf_validate_hint = ".map((e: { field: string; message: string }) => e.message)";
    function validate(): Exit<Package, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(packageDeserialize(data));
    }
    function reset(newOverrides?: Partial<Package>): void {
        data = {
            ...packageDefaultValue(),
            ...newOverrides
        };
        errors = {
            _errors: optionNone(),
            id: optionNone(),
            date: optionNone()
        };
        tainted = {
            id: optionNone(),
            date: optionNone()
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
export function packageFromFormData(formData: FormData): Exit<Package, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    const __gf_exit_hint = "Exit<Package, Array<{ field: string; message: string }>>";
    obj.id = formData.get(`${"id"}`) ?? "";
    obj.date = formData.get(`${"date"}`) ?? "";
    return toExit(packageDeserialize(obj));
}

export const Package = {
  defaultValue: packageDefaultValue,
  serialize: packageSerialize,
  serializeWithContext: packageSerializeWithContext,
  deserialize: packageDeserialize,
  deserializeWithContext: packageDeserializeWithContext,
  validateFields: packageValidateFields,
  hasShape: packageHasShape,
  is: packageIs,
  createForm: packageCreateForm,
  fromFormData: packageFromFormData
} as const;