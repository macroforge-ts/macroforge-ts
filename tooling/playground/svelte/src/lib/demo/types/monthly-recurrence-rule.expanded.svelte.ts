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


export interface MonthlyRecurrenceRule {
    quantityOfMonths: number;
    day: number;
    
    name: string;
}

export function monthlyRecurrenceRuleDefaultValue(): MonthlyRecurrenceRule {
    return {
        quantityOfMonths: 0,
        day: 0,
        name: ""
    } as MonthlyRecurrenceRule;
}

export function monthlyRecurrenceRuleSerialize(value: MonthlyRecurrenceRule): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(monthlyRecurrenceRuleSerializeWithContext(value, ctx));
}
export function monthlyRecurrenceRuleSerializeWithContext(value: MonthlyRecurrenceRule, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: "MonthlyRecurrenceRule",
        __id
    };
    result.quantityOfMonths = value.quantityOfMonths;
    result.day = value.day;
    result.name = value.name;
    return result;
}

export function monthlyRecurrenceRuleDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: MonthlyRecurrenceRule } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = monthlyRecurrenceRuleDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "MonthlyRecurrenceRule.deserialize: root cannot be a forward reference"
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
export function monthlyRecurrenceRuleDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): MonthlyRecurrenceRule | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"MonthlyRecurrenceRule"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("quantityOfMonths" in obj)) {
        errors.push({
            field: "quantityOfMonths",
            message: "missing required field"
        });
    }
    if (!("day" in obj)) {
        errors.push({
            field: "day",
            message: "missing required field"
        });
    }
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
        const __raw_quantityOfMonths = obj["quantityOfMonths"] as number;
        instance.quantityOfMonths = __raw_quantityOfMonths;
    }
    {
        const __raw_day = obj["day"] as number;
        instance.day = __raw_day;
    }
    {
        const __raw_name = obj["name"] as string;
        if (__raw_name.trim().length === 0) {
            errors.push({
                field: "name",
                message: "MonthlyRecurrenceRule.name must not be empty"
            });
        }
        instance.name = __raw_name;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as MonthlyRecurrenceRule;
}
export function monthlyRecurrenceRuleValidateField<K extends keyof MonthlyRecurrenceRule>(_field: K, _value: MonthlyRecurrenceRule[K]): Array<{
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
                message: "MonthlyRecurrenceRule.name must not be empty"
            });
        }
    }
    return errors;
}
export function monthlyRecurrenceRuleValidateFields(_partial: Partial<MonthlyRecurrenceRule>): Array<{
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
                message: "MonthlyRecurrenceRule.name must not be empty"
            });
        }
    }
    return errors;
}
export function monthlyRecurrenceRuleHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"quantityOfMonths" in o && "day" in o && "name" in o';
}
export function monthlyRecurrenceRuleIs(obj: unknown): obj is MonthlyRecurrenceRule {
    if (!monthlyRecurrenceRuleHasShape(obj)) {
        return false;
    }
    const result = monthlyRecurrenceRuleDeserialize(obj);
    return result.success;
}

export type MonthlyRecurrenceRuleErrors = {
    _errors: __gf_Option<Array<string>>;
    quantityOfMonths: __gf_Option<Array<string>>;
    day: __gf_Option<Array<string>>;
    name: __gf_Option<Array<string>>;
};
export type MonthlyRecurrenceRuleTainted = {
    quantityOfMonths: __gf_Option<boolean>;
    day: __gf_Option<boolean>;
    name: __gf_Option<boolean>;
};
export interface MonthlyRecurrenceRuleFieldControllers {
    readonly quantityOfMonths: FieldController<number>;
    readonly day: FieldController<number>;
    readonly name: FieldController<string>;
}
export interface MonthlyRecurrenceRuleGigaform {
    readonly data: MonthlyRecurrenceRule;
    readonly errors: MonthlyRecurrenceRuleErrors;
    readonly tainted: MonthlyRecurrenceRuleTainted;
    readonly fields: MonthlyRecurrenceRuleFieldControllers;
    validate(): Exit<MonthlyRecurrenceRule, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<MonthlyRecurrenceRule>): void;
}
export function monthlyRecurrenceRuleCreateForm(overrides?: Partial<MonthlyRecurrenceRule>): MonthlyRecurrenceRuleGigaform {
    let data = $state({
        ...monthlyRecurrenceRuleDefaultValue(),
        ...overrides
    });
    let errors = $state<MonthlyRecurrenceRuleErrors>({
        _errors: optionNone(),
        quantityOfMonths: optionNone(),
        day: optionNone(),
        name: optionNone()
    } as MonthlyRecurrenceRuleErrors);
    let tainted = $state<MonthlyRecurrenceRuleTainted>({
        quantityOfMonths: optionNone(),
        day: optionNone(),
        name: optionNone()
    } as MonthlyRecurrenceRuleTainted);
    const fields = {
        quantityOfMonths: {
            path: [
                "quantityOfMonths"
            ] as const,
            name: "quantityOfMonths",
            constraints: {
                required: true
            },
            get: ()=>data.quantityOfMonths,
            set: (value: number)=>{
                data.quantityOfMonths = value;
            },
            transform: (value: number): number =>value,
            getError: ()=>errors.quantityOfMonths,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.quantityOfMonths = value;
            },
            getTainted: ()=>tainted.quantityOfMonths,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.quantityOfMonths = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = monthlyRecurrenceRuleValidateField("quantityOfMonths", data.quantityOfMonths);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        day: {
            path: [
                "day"
            ] as const,
            name: "day",
            constraints: {
                required: true
            },
            get: ()=>data.day,
            set: (value: number)=>{
                data.day = value;
            },
            transform: (value: number): number =>value,
            getError: ()=>errors.day,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.day = value;
            },
            getTainted: ()=>tainted.day,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.day = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = monthlyRecurrenceRuleValidateField("day", data.day);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
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
                const fieldErrors = monthlyRecurrenceRuleValidateField("name", data.name);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        }
    } as MonthlyRecurrenceRuleFieldControllers;
    const __gf_getter_hint = "get data() set data(v) get errors() set errors(v) get tainted() set tainted(v)";
    const __gf_validate_hint = ".map((e: { field: string; message: string }) => e.message)";
    function validate(): Exit<MonthlyRecurrenceRule, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(monthlyRecurrenceRuleDeserialize(data));
    }
    function reset(newOverrides?: Partial<MonthlyRecurrenceRule>): void {
        data = {
            ...monthlyRecurrenceRuleDefaultValue(),
            ...newOverrides
        };
        errors = {
            _errors: optionNone(),
            quantityOfMonths: optionNone(),
            day: optionNone(),
            name: optionNone()
        };
        tainted = {
            quantityOfMonths: optionNone(),
            day: optionNone(),
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
export function monthlyRecurrenceRuleFromFormData(formData: FormData): Exit<MonthlyRecurrenceRule, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    const __gf_exit_hint = "Exit<MonthlyRecurrenceRule, Array<{ field: string; message: string }>>";
    {
        const quantityOfMonthsStr = formData.get(`${"quantityOfMonths"}`);
        obj.quantityOfMonths = quantityOfMonthsStr ? parseFloat(quantityOfMonthsStr as string) : $MfPh5;
        if (obj.quantityOfMonths !== undefined && isNaN(obj.quantityOfMonths as number)) obj.quantityOfMonths = "0";
    }
    {
        const dayStr = formData.get(`${"day"}`);
        obj.day = dayStr ? parseFloat(dayStr as string) : $MfPh5;
        if (obj.day !== undefined && isNaN(obj.day as number)) obj.day = "0";
    }
    obj.name = formData.get(`${"name"}`) ?? "";
    return toExit(monthlyRecurrenceRuleDeserialize(obj));
}

export const MonthlyRecurrenceRule = {
  defaultValue: monthlyRecurrenceRuleDefaultValue,
  serialize: monthlyRecurrenceRuleSerialize,
  serializeWithContext: monthlyRecurrenceRuleSerializeWithContext,
  deserialize: monthlyRecurrenceRuleDeserialize,
  deserializeWithContext: monthlyRecurrenceRuleDeserializeWithContext,
  validateFields: monthlyRecurrenceRuleValidateFields,
  hasShape: monthlyRecurrenceRuleHasShape,
  is: monthlyRecurrenceRuleIs,
  createForm: monthlyRecurrenceRuleCreateForm,
  fromFormData: monthlyRecurrenceRuleFromFormData
} as const;