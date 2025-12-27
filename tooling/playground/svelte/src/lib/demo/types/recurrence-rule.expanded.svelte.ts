import { intervalDefaultValue } from "./interval.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { intervalSerializeWithContext } from "./interval.svelte";
import { recurrenceEndSerializeWithContext } from "./recurrence-end.svelte";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { intervalDeserializeWithContext } from "./interval.svelte";
import { recurrenceEndDeserializeWithContext } from "./recurrence-end.svelte";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";
/** import macro {Gigaform} from "@playground/macro"; */

import type { Interval } from './interval.svelte';
import type { RecurrenceEnd } from './recurrence-end.svelte';


export interface RecurrenceRule {
    interval: Interval;
    recurrenceBegins: string;
    recurrenceEnds: RecurrenceEnd | null;
    cancelledInstances: Array<string> | null;
    additionalInstances: Array<string> | null;
}

export function recurrenceRuleDefaultValue(): RecurrenceRule {
    return {
        interval: intervalDefaultValue(),
        recurrenceBegins: "",
        recurrenceEnds: null,
        cancelledInstances: null,
        additionalInstances: null
    } as RecurrenceRule;
}

export function recurrenceRuleSerialize(value: RecurrenceRule): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(recurrenceRuleSerializeWithContext(value, ctx));
}
export function recurrenceRuleSerializeWithContext(value: RecurrenceRule, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"RecurrenceRule"}`,
        __id
    };
    result[`${"interval"}`] = intervalSerializeWithContext(value.interval, ctx);
    result[`${"recurrenceBegins"}`] = value.recurrenceBegins;
    if (value.recurrenceEnds !== null) {
        result[`${"recurrenceEnds"}`] = recurrenceEndSerializeWithContext(value.recurrenceEnds, ctx);
    }
    if (value.cancelledInstances !== null) {
        result[`${"cancelledInstances"}`] = value.cancelledInstances;
    }
    if (value.additionalInstances !== null) {
        result[`${"additionalInstances"}`] = value.additionalInstances;
    }
    return result;
}

export function recurrenceRuleDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: RecurrenceRule } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = recurrenceRuleDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "RecurrenceRule.deserialize: root cannot be a forward reference"
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
export function recurrenceRuleDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): RecurrenceRule | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"RecurrenceRule"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"interval"}` in obj)) {
        errors.push({
            field: `${"interval"}`,
            message: "missing required field"
        });
    }
    if (!(`${"recurrenceBegins"}` in obj)) {
        errors.push({
            field: `${"recurrenceBegins"}`,
            message: "missing required field"
        });
    }
    if (!(`${"recurrenceEnds"}` in obj)) {
        errors.push({
            field: `${"recurrenceEnds"}`,
            message: "missing required field"
        });
    }
    if (!(`${"cancelledInstances"}` in obj)) {
        errors.push({
            field: `${"cancelledInstances"}`,
            message: "missing required field"
        });
    }
    if (!(`${"additionalInstances"}` in obj)) {
        errors.push({
            field: `${"additionalInstances"}`,
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
        const __raw_interval = obj[`${"interval"}`] as Interval;
        {
            const __result = intervalDeserializeWithContext(__raw_interval, ctx);
            ctx.assignOrDefer(instance, `${"interval"}`, __result);
        }
    }
    {
        const __raw_recurrenceBegins = obj[`${"recurrenceBegins"}`] as string;
        instance.recurrenceBegins = __raw_recurrenceBegins;
    }
    {
        const __raw_recurrenceEnds = obj[`${"recurrenceEnds"}`] as RecurrenceEnd | null;
        if (__raw_recurrenceEnds === null) {
            instance.recurrenceEnds = null;
        } else {
            const __result = recurrenceEndDeserializeWithContext(__raw_recurrenceEnds, ctx);
            ctx.assignOrDefer(instance, `${"recurrenceEnds"}`, __result);
        }
    }
    {
        const __raw_cancelledInstances = obj[`${"cancelledInstances"}`] as Array<string> | null;
        if (__raw_cancelledInstances === null) {
            instance.cancelledInstances = null;
        } else {
            instance.cancelledInstances = __raw_cancelledInstances;
        }
    }
    {
        const __raw_additionalInstances = obj[`${"additionalInstances"}`] as Array<string> | null;
        if (__raw_additionalInstances === null) {
            instance.additionalInstances = null;
        } else {
            instance.additionalInstances = __raw_additionalInstances;
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as RecurrenceRule;
}
export function recurrenceRuleValidateField<K extends keyof RecurrenceRule>(_field: K, _value: RecurrenceRule[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function recurrenceRuleValidateFields(_partial: Partial<RecurrenceRule>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function recurrenceRuleHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"interval" in o && "recurrenceBegins" in o && "recurrenceEnds" in o && "cancelledInstances" in o && "additionalInstances" in o';
}
export function recurrenceRuleIs(obj: unknown): obj is RecurrenceRule {
    if (!recurrenceRuleHasShape(obj)) {
        return false;
    }
    const result = recurrenceRuleDeserialize(obj);
    return result.success;
}

export function recurrenceRuleFromFormData(formData: FormData): Exit<RecurrenceRule, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    {
        const intervalObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"interval"}.`)) {
                const fieldName = key.slice(`${"interval"}.`.length);
                const parts = fieldName.split(".");
                let current = intervalObj;
                for(let i = 0; i < parts.length - 1; i++){
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.interval = intervalObj;
    }
    obj.recurrenceBegins = formData.get(`${"recurrenceBegins"}`) ?? "";
    obj.recurrenceEnds = formData.get(`${"recurrenceEnds"}`) ?? "";
    obj.cancelledInstances = formData.get(`${"cancelledInstances"}`) ?? "";
    obj.additionalInstances = formData.get(`${"additionalInstances"}`) ?? "";
    return toExit("recurrenceRuleDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: RecurrenceRule;
    readonly errors: RecurrenceRuleErrors;
    readonly tainted: RecurrenceRuleTainted;
    readonly fields: RecurrenceRuleFieldControllers;
    validate(): Exit<RecurrenceRule, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<RecurrenceRule>): void;
}
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
 }; $MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
 }; export function recurrenceRuleCreateForm(overrides: Partial<RecurrenceRule>): RecurrenceRuleGigaform {}
let data = $state({
    ...recurrenceRuleDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as RecurrenceRuleErrors);
let tainted = $state<$MfPh3>({} as RecurrenceRuleTainted);
const fields = {} as RecurrenceRuleFieldControllers;
fields.interval = {
    label: `${"interval"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.recurrenceBegins = {
    label: `${"recurrenceBegins"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.recurrenceEnds = {
    label: `${"recurrenceEnds"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.cancelledInstances = {
    label: `${"cancelledInstances"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.additionalInstances = {
    label: `${"additionalInstances"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
function validate(): Exit<RecurrenceRule, Array<{
    field: string;
    message: string;
}>> {
    return toExit("recurrenceRuleDeserialize(data)");
    data = {
        ...recurrenceRuleDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const RecurrenceRule = {
  defaultValue: recurrenceRuleDefaultValue,
  serialize: recurrenceRuleSerialize,
  serializeWithContext: recurrenceRuleSerializeWithContext,
  deserialize: recurrenceRuleDeserialize,
  deserializeWithContext: recurrenceRuleDeserializeWithContext,
  validateFields: recurrenceRuleValidateFields,
  hasShape: recurrenceRuleHasShape,
  is: recurrenceRuleIs,
  fromFormData: recurrenceRuleFromFormData,
  createForm: recurrenceRuleCreateForm
} as const;