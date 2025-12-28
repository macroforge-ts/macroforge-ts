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


export interface DirectionHue {
    bearing: number;
    hue: number;
}

export function directionHueDefaultValue(): DirectionHue {
    return {
        bearing: 0,
        hue: 0
    } as DirectionHue;
}

export function directionHueSerialize(value: DirectionHue): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(directionHueSerializeWithContext(value, ctx));
}
export function directionHueSerializeWithContext(value: DirectionHue, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: "DirectionHue",
        __id
    };
    result.bearing = value.bearing;
    result.hue = value.hue;
    return result;
}

export function directionHueDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: DirectionHue } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = directionHueDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "DirectionHue.deserialize: root cannot be a forward reference"
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
export function directionHueDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): DirectionHue | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"DirectionHue"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("bearing" in obj)) {
        errors.push({
            field: "bearing",
            message: "missing required field"
        });
    }
    if (!("hue" in obj)) {
        errors.push({
            field: "hue",
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
        const __raw_bearing = obj["bearing"] as number;
        instance.bearing = __raw_bearing;
    }
    {
        const __raw_hue = obj["hue"] as number;
        instance.hue = __raw_hue;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as DirectionHue;
}
export function directionHueValidateField<K extends keyof DirectionHue>(_field: K, _value: DirectionHue[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function directionHueValidateFields(_partial: Partial<DirectionHue>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function directionHueHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"bearing" in o && "hue" in o';
}
export function directionHueIs(obj: unknown): obj is DirectionHue {
    if (!directionHueHasShape(obj)) {
        return false;
    }
    const result = directionHueDeserialize(obj);
    return result.success;
}

export type DirectionHueErrors = {
    _errors: __gf_Option<Array<string>>;
    bearing: __gf_Option<Array<string>>;
    hue: __gf_Option<Array<string>>;
};
export type DirectionHueTainted = {
    bearing: __gf_Option<boolean>;
    hue: __gf_Option<boolean>;
};
export interface DirectionHueFieldControllers {
    readonly bearing: FieldController<number>;
    readonly hue: FieldController<number>;
}
export interface DirectionHueGigaform {
    readonly data: DirectionHue;
    readonly errors: DirectionHueErrors;
    readonly tainted: DirectionHueTainted;
    readonly fields: DirectionHueFieldControllers;
    validate(): Exit<DirectionHue, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<DirectionHue>): void;
}
export function directionHueCreateForm(overrides?: Partial<DirectionHue>): DirectionHueGigaform {
    let data = $state({
        ...directionHueDefaultValue(),
        ...overrides
    });
    let errors = $state<DirectionHueErrors>({
        _errors: optionNone(),
        bearing: optionNone(),
        hue: optionNone()
    } as DirectionHueErrors);
    let tainted = $state<DirectionHueTainted>({
        bearing: optionNone(),
        hue: optionNone()
    } as DirectionHueTainted);
    const fields = {
        bearing: {
            path: [
                "bearing"
            ] as const,
            name: "bearing",
            constraints: {
                required: true
            },
            get: ()=>data.bearing,
            set: (value: number)=>{
                data.bearing = value;
            },
            transform: (value: number): number =>value,
            getError: ()=>errors.bearing,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.bearing = value;
            },
            getTainted: ()=>tainted.bearing,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.bearing = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = directionHueValidateField("bearing", data.bearing);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        hue: {
            path: [
                "hue"
            ] as const,
            name: "hue",
            constraints: {
                required: true
            },
            get: ()=>data.hue,
            set: (value: number)=>{
                data.hue = value;
            },
            transform: (value: number): number =>value,
            getError: ()=>errors.hue,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.hue = value;
            },
            getTainted: ()=>tainted.hue,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.hue = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = directionHueValidateField("hue", data.hue);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        }
    } as DirectionHueFieldControllers;
    const __gf_getter_hint = "get data() set data(v) get errors() set errors(v) get tainted() set tainted(v)";
    const __gf_validate_hint = ".map((e: { field: string; message: string }) => e.message)";
    function validate(): Exit<DirectionHue, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(directionHueDeserialize(data));
    }
    function reset(newOverrides?: Partial<DirectionHue>): void {
        data = {
            ...directionHueDefaultValue(),
            ...newOverrides
        };
        errors = {
            _errors: optionNone(),
            bearing: optionNone(),
            hue: optionNone()
        };
        tainted = {
            bearing: optionNone(),
            hue: optionNone()
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
export function directionHueFromFormData(formData: FormData): Exit<DirectionHue, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    const __gf_exit_hint = "Exit<DirectionHue, Array<{ field: string; message: string }>>";
    {
        const bearingStr = formData.get(`${"bearing"}`);
        obj.bearing = bearingStr ? parseFloat(bearingStr as string) : $MfPh5;
        if (obj.bearing !== undefined && isNaN(obj.bearing as number)) obj.bearing = "0";
    }
    {
        const hueStr = formData.get(`${"hue"}`);
        obj.hue = hueStr ? parseFloat(hueStr as string) : $MfPh5;
        if (obj.hue !== undefined && isNaN(obj.hue as number)) obj.hue = "0";
    }
    return toExit(directionHueDeserialize(obj));
}

export const DirectionHue = {
  defaultValue: directionHueDefaultValue,
  serialize: directionHueSerialize,
  serializeWithContext: directionHueSerializeWithContext,
  deserialize: directionHueDeserialize,
  deserializeWithContext: directionHueDeserializeWithContext,
  validateFields: directionHueValidateFields,
  hasShape: directionHueHasShape,
  is: directionHueIs,
  createForm: directionHueCreateForm,
  fromFormData: directionHueFromFormData
} as const;