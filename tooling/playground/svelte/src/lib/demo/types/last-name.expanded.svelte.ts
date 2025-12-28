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


export interface LastName {
    
    name: string;
}

export function lastNameDefaultValue(): LastName {
    return {
        name: ""
    } as LastName;
}

export function lastNameSerialize(value: LastName): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(lastNameSerializeWithContext(value, ctx));
}
export function lastNameSerializeWithContext(value: LastName, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: "LastName",
        __id
    };
    result.name = value.name;
    return result;
}

export function lastNameDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: LastName } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = lastNameDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "LastName.deserialize: root cannot be a forward reference"
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
export function lastNameDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): LastName | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"LastName"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("name" in obj)) {
        errors.push({
            field: "name",
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
        const __raw_name = obj["name"] as string;
        if (__raw_name.trim().length === 0) {
            errors.push({
                field: "name",
                message: "LastName.name must not be empty"
            });
        }
        instance.name = __raw_name;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as LastName;
}
export function lastNameValidateField<K extends keyof LastName>(_field: K, _value: LastName[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === "name") {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "LastName.name must not be empty"
            });
        }
    }
    return errors;
}
export function lastNameValidateFields(_partial: Partial<LastName>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("name" in _partial && _partial.name !== undefined) {
        const __val = _partial.name as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "LastName.name must not be empty"
            });
        }
    }
    return errors;
}
export function lastNameHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"name" in o';
}
export function lastNameIs(obj: unknown): obj is LastName {
    if (!lastNameHasShape(obj)) {
        return false;
    }
    const result = lastNameDeserialize(obj);
    return result.success;
}

export type LastNameErrors = {
    _errors: __gf_Option<Array<string>>;
    name: __gf_Option<Array<string>>;
};
export type LastNameTainted = {
    name: __gf_Option<boolean>;
};
export interface LastNameFieldControllers {
    readonly name: FieldController<string>;
}
export interface LastNameGigaform {
    readonly data: LastName;
    readonly errors: LastNameErrors;
    readonly tainted: LastNameTainted;
    readonly fields: LastNameFieldControllers;
    validate(): Exit<LastName, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<LastName>): void;
}
export function lastNameCreateForm(overrides?: Partial<LastName>): LastNameGigaform {
    let data = $state({
        ...lastNameDefaultValue(),
        ...overrides
    });
    let errors = $state<LastNameErrors>({
        _errors: optionNone(),
        name: optionNone()
    } as LastNameErrors);
    let tainted = $state<LastNameTainted>({
        name: optionNone()
    } as LastNameTainted);
    const fields = {
        name: {
            path: [
                "name"
            ] as const,
            name: "name",
            constraints: {
                required: true
            },
            get: ()=>data.name,
            set: (value: string)=>{
                data.name = value;
            },
            transform: (value: string): string =>value,
            getError: ()=>errors.name,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.name = value;
            },
            getTainted: ()=>tainted.name,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.name = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = lastNameValidateField("name", data.name);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        }
    } as LastNameFieldControllers;
    const __gf_getter_hint = "get data() set data(v) get errors() set errors(v) get tainted() set tainted(v)";
    const __gf_validate_hint = ".map((e: { field: string; message: string }) => e.message)";
    function validate(): Exit<LastName, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(lastNameDeserialize(data));
    }
    function reset(newOverrides?: Partial<LastName>): void {
        data = {
            ...lastNameDefaultValue(),
            ...newOverrides
        };
        errors = {
            _errors: optionNone(),
            name: optionNone()
        };
        tainted = {
            name: optionNone()
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
export function lastNameFromFormData(formData: FormData): Exit<LastName, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    const __gf_exit_hint = "Exit<LastName, Array<{ field: string; message: string }>>";
    obj.name = formData.get(`${"name"}`) ?? "";
    return toExit(lastNameDeserialize(obj));
}

export const LastName = {
  defaultValue: lastNameDefaultValue,
  serialize: lastNameSerialize,
  serializeWithContext: lastNameSerializeWithContext,
  deserialize: lastNameDeserialize,
  deserializeWithContext: lastNameDeserializeWithContext,
  validateFields: lastNameValidateFields,
  hasShape: lastNameHasShape,
  is: lastNameIs,
  createForm: lastNameCreateForm,
  fromFormData: lastNameFromFormData
} as const;