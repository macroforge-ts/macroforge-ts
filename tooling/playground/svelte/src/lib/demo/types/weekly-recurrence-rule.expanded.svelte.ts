import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { weekdaySerializeWithContext } from "./weekday.svelte";
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

import type { Weekday } from './weekday.svelte';


export interface WeeklyRecurrenceRule {
    quantityOfWeeks: number;
    weekdays: Array<Weekday>;
}

export function weeklyRecurrenceRuleDefaultValue(): WeeklyRecurrenceRule {
    return {
        quantityOfWeeks: 0,
        weekdays: []
    } as WeeklyRecurrenceRule;
}

export function weeklyRecurrenceRuleSerialize(value: WeeklyRecurrenceRule): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(weeklyRecurrenceRuleSerializeWithContext(value, ctx));
}
export function weeklyRecurrenceRuleSerializeWithContext(value: WeeklyRecurrenceRule, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: "WeeklyRecurrenceRule",
        __id
    };
    result.quantityOfWeeks = value.quantityOfWeeks;
    result.weekdays = value.weekdays.map((item)=>weekdaySerializeWithContext(item, ctx));
    return result;
}

export function weeklyRecurrenceRuleDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: WeeklyRecurrenceRule } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = weeklyRecurrenceRuleDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "WeeklyRecurrenceRule.deserialize: root cannot be a forward reference"
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
export function weeklyRecurrenceRuleDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): WeeklyRecurrenceRule | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"WeeklyRecurrenceRule"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("quantityOfWeeks" in obj)) {
        errors.push({
            field: "quantityOfWeeks",
            message: "missing required field"
        });
    }
    if (!("weekdays" in obj)) {
        errors.push({
            field: "weekdays",
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
        const __raw_quantityOfWeeks = obj["quantityOfWeeks"] as number;
        instance.quantityOfWeeks = __raw_quantityOfWeeks;
    }
    {
        const __raw_weekdays = obj["weekdays"] as Array<Weekday>;
        if (Array.isArray(__raw_weekdays)) {
            instance.weekdays = __raw_weekdays as Weekday[];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as WeeklyRecurrenceRule;
}
export function weeklyRecurrenceRuleValidateField<K extends keyof WeeklyRecurrenceRule>(_field: K, _value: WeeklyRecurrenceRule[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function weeklyRecurrenceRuleValidateFields(_partial: Partial<WeeklyRecurrenceRule>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function weeklyRecurrenceRuleHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"quantityOfWeeks" in o && "weekdays" in o';
}
export function weeklyRecurrenceRuleIs(obj: unknown): obj is WeeklyRecurrenceRule {
    if (!weeklyRecurrenceRuleHasShape(obj)) {
        return false;
    }
    const result = weeklyRecurrenceRuleDeserialize(obj);
    return result.success;
}

export type WeeklyRecurrenceRuleErrors = {
    _errors: __gf_Option<Array<string>>;
    quantityOfWeeks: __gf_Option<Array<string>>;
    weekdays: __gf_Option<Array<string>>;
};
export type WeeklyRecurrenceRuleTainted = {
    quantityOfWeeks: __gf_Option<boolean>;
    weekdays: __gf_Option<boolean>;
};
export interface WeeklyRecurrenceRuleFieldControllers {
    readonly quantityOfWeeks: FieldController<number>;
    readonly weekdays: ArrayFieldController<Weekday>;
}
export interface WeeklyRecurrenceRuleGigaform {
    readonly data: WeeklyRecurrenceRule;
    readonly errors: WeeklyRecurrenceRuleErrors;
    readonly tainted: WeeklyRecurrenceRuleTainted;
    readonly fields: WeeklyRecurrenceRuleFieldControllers;
    validate(): Exit<WeeklyRecurrenceRule, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<WeeklyRecurrenceRule>): void;
}
export function weeklyRecurrenceRuleCreateForm(overrides?: Partial<WeeklyRecurrenceRule>): WeeklyRecurrenceRuleGigaform {
    let data = $state({
        ...weeklyRecurrenceRuleDefaultValue(),
        ...overrides
    });
    let errors = $state<WeeklyRecurrenceRuleErrors>({
        _errors: optionNone(),
        quantityOfWeeks: optionNone(),
        weekdays: optionNone()
    } as WeeklyRecurrenceRuleErrors);
    let tainted = $state<WeeklyRecurrenceRuleTainted>({
        quantityOfWeeks: optionNone(),
        weekdays: optionNone()
    } as WeeklyRecurrenceRuleTainted);
    const fields = {
        quantityOfWeeks: {
            path: [
                "quantityOfWeeks"
            ] as const,
            name: "quantityOfWeeks",
            constraints: {
                required: true
            },
            get: ()=>data.quantityOfWeeks,
            set: (value: number)=>{
                data.quantityOfWeeks = value;
            },
            transform: (value: number): number =>value,
            getError: ()=>errors.quantityOfWeeks,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.quantityOfWeeks = value;
            },
            getTainted: ()=>tainted.quantityOfWeeks,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.quantityOfWeeks = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = weeklyRecurrenceRuleValidateField("quantityOfWeeks", data.quantityOfWeeks);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        weekdays: {
            path: [
                "weekdays"
            ] as const,
            name: "weekdays",
            constraints: {
                required: true
            },
            get: ()=>data.weekdays,
            set: (value: Array<Weekday>)=>{
                data.weekdays = value;
            },
            transform: (value: Array<Weekday>): Array<Weekday> =>value,
            getError: ()=>errors.weekdays,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.weekdays = value;
            },
            getTainted: ()=>tainted.weekdays,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.weekdays = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = weeklyRecurrenceRuleValidateField("weekdays", data.weekdays);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            },
            at: (index: number)=>({
                    path: [
                        "weekdays",
                        index
                    ] as const,
                    name: "'^weekdays.${index}^'",
                    constraints: {
                        required: true
                    },
                    get: ()=>data.weekdays[index]!,
                    set: (value: Weekday)=>{
                        data.weekdays[index] = value;
                    },
                    transform: (value: Weekday): Weekday =>value,
                    getError: ()=>errors.weekdays,
                    setError: (value: __gf_Option<Array<string>>)=>{
                        errors.weekdays = value;
                    },
                    getTainted: ()=>tainted.weekdays,
                    setTainted: (value: __gf_Option<boolean>)=>{
                        tainted.weekdays = value;
                    },
                    validate: (): Array<string> =>[]
                }),
            push: (item: Weekday)=>{
                data.weekdays.push(item);
            },
            remove: (index: number)=>{
                data.weekdays.splice(index, 1);
            },
            swap: (a: number, b: number)=>{
                const tmp = data.weekdays[a]!;
                data.weekdays[a] = data.weekdays[b]!;
                data.weekdays[b] = tmp;
            }
        }
    } as WeeklyRecurrenceRuleFieldControllers;
    const __gf_getter_hint = "get data() set data(v) get errors() set errors(v) get tainted() set tainted(v)";
    const __gf_validate_hint = ".map((e: { field: string; message: string }) => e.message)";
    function validate(): Exit<WeeklyRecurrenceRule, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(weeklyRecurrenceRuleDeserialize(data));
    }
    function reset(newOverrides?: Partial<WeeklyRecurrenceRule>): void {
        data = {
            ...weeklyRecurrenceRuleDefaultValue(),
            ...newOverrides
        };
        errors = {
            _errors: optionNone(),
            quantityOfWeeks: optionNone(),
            weekdays: optionNone()
        };
        tainted = {
            quantityOfWeeks: optionNone(),
            weekdays: optionNone()
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
export function weeklyRecurrenceRuleFromFormData(formData: FormData): Exit<WeeklyRecurrenceRule, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    const __gf_exit_hint = "Exit<WeeklyRecurrenceRule, Array<{ field: string; message: string }>>";
    {
        const quantityOfWeeksStr = formData.get(`${"quantityOfWeeks"}`);
        obj.quantityOfWeeks = quantityOfWeeksStr ? parseFloat(quantityOfWeeksStr as string) : $MfPh5;
        if (obj.quantityOfWeeks !== undefined && isNaN(obj.quantityOfWeeks as number)) obj.quantityOfWeeks = "0";
    }
    {
        const weekdaysItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while(formData.has(`${"weekdays"}.` + idx + ".") || idx === 0){
            const hasAny = Array.from(formData.keys()).some((k)=>k.startsWith(`${"weekdays"}.` + idx + "."));
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())){
                    if (key.startsWith(`${"weekdays"}.` + idx + ".")) {
                        const fieldName = key.slice(`${"weekdays"}.`.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                weekdaysItems.push(item);
            }
            idx++;
            if (idx > 1000) break;
        }
        obj.weekdays = weekdaysItems;
    }
    return toExit(weeklyRecurrenceRuleDeserialize(obj));
}

export const WeeklyRecurrenceRule = {
  defaultValue: weeklyRecurrenceRuleDefaultValue,
  serialize: weeklyRecurrenceRuleSerialize,
  serializeWithContext: weeklyRecurrenceRuleSerializeWithContext,
  deserialize: weeklyRecurrenceRuleDeserialize,
  deserializeWithContext: weeklyRecurrenceRuleDeserializeWithContext,
  validateFields: weeklyRecurrenceRuleValidateFields,
  hasShape: weeklyRecurrenceRuleHasShape,
  is: weeklyRecurrenceRuleIs,
  createForm: weeklyRecurrenceRuleCreateForm,
  fromFormData: weeklyRecurrenceRuleFromFormData
} as const;