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


export interface Viewed {
    durationSeconds: number | null;
    source: string | null;
}

export function viewedDefaultValue(): Viewed {
    return {
        durationSeconds: null,
        source: null
    } as Viewed;
}

export function viewedSerialize(value: Viewed): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(viewedSerializeWithContext(value, ctx));
}
export function viewedSerializeWithContext(value: Viewed, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Viewed"}`,
        __id
    };
    result[`${"durationSeconds"}`] = value.durationSeconds;
    result[`${"source"}`] = value.source;
    return result;
}

export function viewedDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Viewed } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = viewedDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Viewed.deserialize: root cannot be a forward reference"
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
export function viewedDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Viewed | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Viewed"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"durationSeconds"}` in obj)) {
        errors.push({
            field: `${"durationSeconds"}`,
            message: "missing required field"
        });
    }
    if (!(`${"source"}` in obj)) {
        errors.push({
            field: `${"source"}`,
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
        const __raw_durationSeconds = obj[`${"durationSeconds"}`] as number | null;
        instance.durationSeconds = __raw_durationSeconds;
    }
    {
        const __raw_source = obj[`${"source"}`] as string | null;
        instance.source = __raw_source;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Viewed;
}
export function viewedValidateField<K extends keyof Viewed>(_field: K, _value: Viewed[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function viewedValidateFields(_partial: Partial<Viewed>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function viewedHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"durationSeconds" in o && "source" in o';
}
export function viewedIs(obj: unknown): obj is Viewed {
    if (!viewedHasShape(obj)) {
        return false;
    }
    const result = viewedDeserialize(obj);
    return result.success;
}

export function viewedFromFormData(formData: FormData): Exit<Viewed, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    {
        const durationSecondsStr = formData.get(`${"durationSeconds"}`);
        obj.durationSeconds = durationSecondsStr ? parseFloat(durationSecondsStr as string) : $MfPh5;
        if (obj.durationSeconds !== undefined && isNaN(obj.durationSeconds as number)) obj.durationSeconds = "0";
    }
    obj.source = formData.get(`${"source"}`) ?? "";
    return toExit("viewedDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: Viewed;
    readonly errors: ViewedErrors;
    readonly tainted: ViewedTainted;
    readonly fields: ViewedFieldControllers;
    validate(): Exit<Viewed, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<Viewed>): void;
}
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
 }; $MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
 }; export function viewedCreateForm(overrides: Partial<Viewed>): ViewedGigaform {}
let data = $state({
    ...viewedDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as ViewedErrors);
let tainted = $state<$MfPh3>({} as ViewedTainted);
const fields = {} as ViewedFieldControllers;
fields.durationSeconds = {
    label: `${"durationSeconds"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.source = {
    label: `${"source"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
function validate(): Exit<Viewed, Array<{
    field: string;
    message: string;
}>> {
    return toExit("viewedDeserialize(data)");
    data = {
        ...viewedDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const Viewed = {
  defaultValue: viewedDefaultValue,
  serialize: viewedSerialize,
  serializeWithContext: viewedSerializeWithContext,
  deserialize: viewedDeserialize,
  deserializeWithContext: viewedDeserializeWithContext,
  validateFields: viewedValidateFields,
  hasShape: viewedHasShape,
  is: viewedIs,
  fromFormData: viewedFromFormData,
  createForm: viewedCreateForm
} as const;