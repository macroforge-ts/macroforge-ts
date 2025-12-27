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


export interface Gradient {
    startHue: number;
}

export function gradientDefaultValue(): Gradient {
    return {
        startHue: 0
    } as Gradient;
}

export function gradientSerialize(value: Gradient): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(gradientSerializeWithContext(value, ctx));
}
export function gradientSerializeWithContext(value: Gradient, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Gradient"}`,
        __id
    };
    result[`${"startHue"}`] = value.startHue;
    return result;
}

export function gradientDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Gradient } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = gradientDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Gradient.deserialize: root cannot be a forward reference"
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
export function gradientDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Gradient | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Gradient"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"startHue"}` in obj)) {
        errors.push({
            field: `${"startHue"}`,
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
        const __raw_startHue = obj[`${"startHue"}`] as number;
        instance.startHue = __raw_startHue;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Gradient;
}
export function gradientValidateField<K extends keyof Gradient>(_field: K, _value: Gradient[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function gradientValidateFields(_partial: Partial<Gradient>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function gradientHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"startHue" in o';
}
export function gradientIs(obj: unknown): obj is Gradient {
    if (!gradientHasShape(obj)) {
        return false;
    }
    const result = gradientDeserialize(obj);
    return result.success;
}

export function gradientFromFormData(formData: FormData): Exit<Gradient, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    {
        const startHueStr = formData.get(`${"startHue"}`);
        obj.startHue = startHueStr ? parseFloat(startHueStr as string) : $MfPh5;
        if (obj.startHue !== undefined && isNaN(obj.startHue as number)) obj.startHue = "0";
    }
    return toExit("gradientDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: Gradient;
    readonly errors: GradientErrors;
    readonly tainted: GradientTainted;
    readonly fields: GradientFieldControllers;
    validate(): Exit<Gradient, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<Gradient>): void;
}
$MfPh0: __gf_Option<Array<string>>;
 }; $MfPh0: __gf_Option<boolean>;
 }; export function gradientCreateForm(overrides: Partial<Gradient>): GradientGigaform {}
let data = $state({
    ...gradientDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as GradientErrors);
let tainted = $state<$MfPh3>({} as GradientTainted);
const fields = {} as GradientFieldControllers;
fields.startHue = {
    label: `${"startHue"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
function validate(): Exit<Gradient, Array<{
    field: string;
    message: string;
}>> {
    return toExit("gradientDeserialize(data)");
    data = {
        ...gradientDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const Gradient = {
  defaultValue: gradientDefaultValue,
  serialize: gradientSerialize,
  serializeWithContext: gradientSerializeWithContext,
  deserialize: gradientDeserialize,
  deserializeWithContext: gradientDeserializeWithContext,
  validateFields: gradientValidateFields,
  hasShape: gradientHasShape,
  is: gradientIs,
  fromFormData: gradientFromFormData,
  createForm: gradientCreateForm
} as const;