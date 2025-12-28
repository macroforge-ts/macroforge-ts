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


export interface Created {
    initialData: string | null;
}

export function createdDefaultValue(): Created {
    return {
        initialData: null
    } as Created;
}

export function createdSerialize(value: Created): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(createdSerializeWithContext(value, ctx));
}
export function createdSerializeWithContext(value: Created, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: "Created",
        __id
    };
    result.initialData = value.initialData;
    return result;
}

export function createdDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Created } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = createdDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Created.deserialize: root cannot be a forward reference"
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
export function createdDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Created | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Created"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("initialData" in obj)) {
        errors.push({
            field: "initialData",
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
        const __raw_initialData = obj["initialData"] as string | null;
        instance.initialData = __raw_initialData;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Created;
}
export function createdValidateField<K extends keyof Created>(_field: K, _value: Created[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function createdValidateFields(_partial: Partial<Created>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function createdHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"initialData" in o';
}
export function createdIs(obj: unknown): obj is Created {
    if (!createdHasShape(obj)) {
        return false;
    }
    const result = createdDeserialize(obj);
    return result.success;
}

export type CreatedErrors = {
    _errors: __gf_Option<Array<string>>;
    initialData: __gf_Option<Array<string>>;
};
export type CreatedTainted = {
    initialData: __gf_Option<boolean>;
};
export interface CreatedFieldControllers {
    readonly initialData: FieldController<string | null>;
}
export interface CreatedGigaform {
    readonly data: Created;
    readonly errors: CreatedErrors;
    readonly tainted: CreatedTainted;
    readonly fields: CreatedFieldControllers;
    validate(): Exit<Created, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<Created>): void;
}
export function createdCreateForm(overrides?: Partial<Created>): CreatedGigaform {
    let data = $state({
        ...createdDefaultValue(),
        ...overrides
    });
    let errors = $state<CreatedErrors>({
        _errors: optionNone(),
        initialData: optionNone()
    } as CreatedErrors);
    let tainted = $state<CreatedTainted>({
        initialData: optionNone()
    } as CreatedTainted);
    const fields = {
        initialData: {
            path: [
                "initialData"
            ] as const,
            name: "initialData",
            constraints: {
                required: true
            },
            get: ()=>data.initialData,
            set: (value: string | null)=>{
                data.initialData = value;
            },
            transform: (value: string | null): string | null =>value,
            getError: ()=>errors.initialData,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.initialData = value;
            },
            getTainted: ()=>tainted.initialData,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.initialData = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = createdValidateField("initialData", data.initialData);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        }
    } as CreatedFieldControllers;
    const __gf_getter_hint = "get data() set data(v) get errors() set errors(v) get tainted() set tainted(v)";
    const __gf_validate_hint = ".map((e: { field: string; message: string }) => e.message)";
    function validate(): Exit<Created, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(createdDeserialize(data));
    }
    function reset(newOverrides?: Partial<Created>): void {
        data = {
            ...createdDefaultValue(),
            ...newOverrides
        };
        errors = {
            _errors: optionNone(),
            initialData: optionNone()
        };
        tainted = {
            initialData: optionNone()
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
export function createdFromFormData(formData: FormData): Exit<Created, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    const __gf_exit_hint = "Exit<Created, Array<{ field: string; message: string }>>";
    obj.initialData = formData.get(`${"initialData"}`) ?? "";
    return toExit(createdDeserialize(obj));
}

export const Created = {
  defaultValue: createdDefaultValue,
  serialize: createdSerialize,
  serializeWithContext: createdSerializeWithContext,
  deserialize: createdDeserialize,
  deserializeWithContext: createdDeserializeWithContext,
  validateFields: createdValidateFields,
  hasShape: createdHasShape,
  is: createdIs,
  createForm: createdCreateForm,
  fromFormData: createdFromFormData
} as const;