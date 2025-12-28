import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { directionHueSerializeWithContext } from "./direction-hue.svelte";
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

import type { DirectionHue } from './direction-hue.svelte';


export interface Custom {
    mappings: Array<DirectionHue>;
}

export function customDefaultValue(): Custom {
    return {
        mappings: []
    } as Custom;
}

export function customSerialize(value: Custom): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(customSerializeWithContext(value, ctx));
}
export function customSerializeWithContext(value: Custom, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: "Custom",
        __id
    };
    result.mappings = value.mappings.map((item)=>directionHueSerializeWithContext(item, ctx));
    return result;
}

export function customDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Custom } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = customDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Custom.deserialize: root cannot be a forward reference"
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
export function customDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Custom | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Custom"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("mappings" in obj)) {
        errors.push({
            field: "mappings",
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
        const __raw_mappings = obj["mappings"] as Array<DirectionHue>;
        if (Array.isArray(__raw_mappings)) {
            instance.mappings = __raw_mappings as DirectionHue[];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Custom;
}
export function customValidateField<K extends keyof Custom>(_field: K, _value: Custom[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function customValidateFields(_partial: Partial<Custom>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function customHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"mappings" in o';
}
export function customIs(obj: unknown): obj is Custom {
    if (!customHasShape(obj)) {
        return false;
    }
    const result = customDeserialize(obj);
    return result.success;
}

export type CustomErrors = {
    _errors: __gf_Option<Array<string>>;
    mappings: __gf_Option<Array<string>>;
};
export type CustomTainted = {
    mappings: __gf_Option<boolean>;
};
export interface CustomFieldControllers {
    readonly mappings: ArrayFieldController<DirectionHue>;
}
export interface CustomGigaform {
    readonly data: Custom;
    readonly errors: CustomErrors;
    readonly tainted: CustomTainted;
    readonly fields: CustomFieldControllers;
    validate(): Exit<Custom, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<Custom>): void;
}
export function customCreateForm(overrides?: Partial<Custom>): CustomGigaform {
    let data = $state({
        ...customDefaultValue(),
        ...overrides
    });
    let errors = $state<CustomErrors>({
        _errors: optionNone(),
        mappings: optionNone()
    } as CustomErrors);
    let tainted = $state<CustomTainted>({
        mappings: optionNone()
    } as CustomTainted);
    const fields = {
        mappings: {
            path: [
                "mappings"
            ] as const,
            name: "mappings",
            constraints: {
                required: true
            },
            get: ()=>data.mappings,
            set: (value: Array<DirectionHue>)=>{
                data.mappings = value;
            },
            transform: (value: Array<DirectionHue>): Array<DirectionHue> =>value,
            getError: ()=>errors.mappings,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.mappings = value;
            },
            getTainted: ()=>tainted.mappings,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.mappings = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = customValidateField("mappings", data.mappings);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            },
            at: (index: number)=>({
                    path: [
                        "mappings",
                        index
                    ] as const,
                    name: "'^mappings.${index}^'",
                    constraints: {
                        required: true
                    },
                    get: ()=>data.mappings[index]!,
                    set: (value: DirectionHue)=>{
                        data.mappings[index] = value;
                    },
                    transform: (value: DirectionHue): DirectionHue =>value,
                    getError: ()=>errors.mappings,
                    setError: (value: __gf_Option<Array<string>>)=>{
                        errors.mappings = value;
                    },
                    getTainted: ()=>tainted.mappings,
                    setTainted: (value: __gf_Option<boolean>)=>{
                        tainted.mappings = value;
                    },
                    validate: (): Array<string> =>[]
                }),
            push: (item: DirectionHue)=>{
                data.mappings.push(item);
            },
            remove: (index: number)=>{
                data.mappings.splice(index, 1);
            },
            swap: (a: number, b: number)=>{
                const tmp = data.mappings[a]!;
                data.mappings[a] = data.mappings[b]!;
                data.mappings[b] = tmp;
            }
        }
    } as CustomFieldControllers;
    const __gf_getter_hint = "get data() set data(v) get errors() set errors(v) get tainted() set tainted(v)";
    const __gf_validate_hint = ".map((e: { field: string; message: string }) => e.message)";
    function validate(): Exit<Custom, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(customDeserialize(data));
    }
    function reset(newOverrides?: Partial<Custom>): void {
        data = {
            ...customDefaultValue(),
            ...newOverrides
        };
        errors = {
            _errors: optionNone(),
            mappings: optionNone()
        };
        tainted = {
            mappings: optionNone()
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
export function customFromFormData(formData: FormData): Exit<Custom, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    const __gf_exit_hint = "Exit<Custom, Array<{ field: string; message: string }>>";
    {
        const mappingsItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while(formData.has(`${"mappings"}.` + idx + ".") || idx === 0){
            const hasAny = Array.from(formData.keys()).some((k)=>k.startsWith(`${"mappings"}.` + idx + "."));
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())){
                    if (key.startsWith(`${"mappings"}.` + idx + ".")) {
                        const fieldName = key.slice(`${"mappings"}.`.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                mappingsItems.push(item);
            }
            idx++;
            if (idx > 1000) break;
        }
        obj.mappings = mappingsItems;
    }
    return toExit(customDeserialize(obj));
}

export const Custom = {
  defaultValue: customDefaultValue,
  serialize: customSerialize,
  serializeWithContext: customSerializeWithContext,
  deserialize: customDeserialize,
  deserializeWithContext: customDeserializeWithContext,
  validateFields: customValidateFields,
  hasShape: customHasShape,
  is: customIs,
  createForm: customCreateForm,
  fromFormData: customFromFormData
} as const;