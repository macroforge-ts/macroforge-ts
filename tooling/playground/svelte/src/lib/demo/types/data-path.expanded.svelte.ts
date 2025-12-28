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
import type { ArrayFieldController } from "@playground/macro/gigaform";
/** import macro {Gigaform} from "@playground/macro"; */


export interface DataPath {
    path: Array<string>;
    formatter: string | null;
}

export function dataPathDefaultValue(): DataPath {
    return {
        path: [],
        formatter: null
    } as DataPath;
}

export function dataPathSerialize(value: DataPath): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(dataPathSerializeWithContext(value, ctx));
}
export function dataPathSerializeWithContext(value: DataPath, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: "DataPath",
        __id
    };
    result.path = value.path;
    result.formatter = value.formatter;
    return result;
}

export function dataPathDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: DataPath } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = dataPathDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "DataPath.deserialize: root cannot be a forward reference"
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
export function dataPathDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): DataPath | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"DataPath"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("path" in obj)) {
        errors.push({
            field: "path",
            message: "missing required field"
        });
    }
    if (!("formatter" in obj)) {
        errors.push({
            field: "formatter",
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
        const __raw_path = obj["path"] as Array<string>;
        if (Array.isArray(__raw_path)) {
            instance.path = __raw_path as string[];
        }
    }
    {
        const __raw_formatter = obj["formatter"] as string | null;
        instance.formatter = __raw_formatter;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as DataPath;
}
export function dataPathValidateField<K extends keyof DataPath>(_field: K, _value: DataPath[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function dataPathValidateFields(_partial: Partial<DataPath>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function dataPathHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"path" in o && "formatter" in o';
}
export function dataPathIs(obj: unknown): obj is DataPath {
    if (!dataPathHasShape(obj)) {
        return false;
    }
    const result = dataPathDeserialize(obj);
    return result.success;
}

export type DataPathErrors = {
    _errors: __gf_Option<Array<string>>;
    path: __gf_Option<Array<string>>;
    formatter: __gf_Option<Array<string>>;
};
export type DataPathTainted = {
    path: __gf_Option<boolean>;
    formatter: __gf_Option<boolean>;
};
export interface DataPathFieldControllers {
    readonly path: ArrayFieldController<string>;
    readonly formatter: FieldController<string | null>;
}
export interface DataPathGigaform {
    readonly data: DataPath;
    readonly errors: DataPathErrors;
    readonly tainted: DataPathTainted;
    readonly fields: DataPathFieldControllers;
    validate(): Exit<DataPath, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<DataPath>): void;
}
export function dataPathCreateForm(overrides?: Partial<DataPath>): DataPathGigaform {
    let data = $state({
        ...dataPathDefaultValue(),
        ...overrides
    });
    let errors = $state<DataPathErrors>({
        _errors: optionNone(),
        path: optionNone(),
        formatter: optionNone()
    } as DataPathErrors);
    let tainted = $state<DataPathTainted>({
        path: optionNone(),
        formatter: optionNone()
    } as DataPathTainted);
    const fields = {
        path: {
            path: [
                "path"
            ] as const,
            name: "path",
            constraints: {
                required: true
            },
            get: ()=>data.path,
            set: (value: Array<string>)=>{
                data.path = value;
            },
            transform: (value: Array<string>): Array<string> =>value,
            getError: ()=>errors.path,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.path = value;
            },
            getTainted: ()=>tainted.path,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.path = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = dataPathValidateField("path", data.path);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            },
            at: (index: number)=>({
                    path: [
                        "path",
                        index
                    ] as const,
                    name: "'^path.${index}^'",
                    constraints: {
                        required: true
                    },
                    get: ()=>data.path[index]!,
                    set: (value: string)=>{
                        data.path[index] = value;
                    },
                    transform: (value: string): string =>value,
                    getError: ()=>errors.path,
                    setError: (value: __gf_Option<Array<string>>)=>{
                        errors.path = value;
                    },
                    getTainted: ()=>tainted.path,
                    setTainted: (value: __gf_Option<boolean>)=>{
                        tainted.path = value;
                    },
                    validate: (): Array<string> =>[]
                }),
            push: (item: string)=>{
                data.path.push(item);
            },
            remove: (index: number)=>{
                data.path.splice(index, 1);
            },
            swap: (a: number, b: number)=>{
                const tmp = data.path[a]!;
                data.path[a] = data.path[b]!;
                data.path[b] = tmp;
            }
        },
        formatter: {
            path: [
                "formatter"
            ] as const,
            name: "formatter",
            constraints: {
                required: true
            },
            get: ()=>data.formatter,
            set: (value: string | null)=>{
                data.formatter = value;
            },
            transform: (value: string | null): string | null =>value,
            getError: ()=>errors.formatter,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.formatter = value;
            },
            getTainted: ()=>tainted.formatter,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.formatter = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = dataPathValidateField("formatter", data.formatter);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        }
    } as DataPathFieldControllers;
    const __gf_getter_hint = "get data() set data(v) get errors() set errors(v) get tainted() set tainted(v)";
    const __gf_validate_hint = ".map((e: { field: string; message: string }) => e.message)";
    function validate(): Exit<DataPath, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(dataPathDeserialize(data));
    }
    function reset(newOverrides?: Partial<DataPath>): void {
        data = {
            ...dataPathDefaultValue(),
            ...newOverrides
        };
        errors = {
            _errors: optionNone(),
            path: optionNone(),
            formatter: optionNone()
        };
        tainted = {
            path: optionNone(),
            formatter: optionNone()
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
export function dataPathFromFormData(formData: FormData): Exit<DataPath, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    const __gf_exit_hint = "Exit<DataPath, Array<{ field: string; message: string }>>";
    obj.path = formData.getAll(`${"path"}`) as Array<string>;
    obj.formatter = formData.get(`${"formatter"}`) ?? "";
    return toExit(dataPathDeserialize(obj));
}

export const DataPath = {
  defaultValue: dataPathDefaultValue,
  serialize: dataPathSerialize,
  serializeWithContext: dataPathSerializeWithContext,
  deserialize: dataPathDeserialize,
  deserializeWithContext: dataPathDeserializeWithContext,
  validateFields: dataPathValidateFields,
  hasShape: dataPathHasShape,
  is: dataPathIs,
  createForm: dataPathCreateForm,
  fromFormData: dataPathFromFormData
} as const;