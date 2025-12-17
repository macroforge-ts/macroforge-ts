import { dailyRecurrenceRuleDefaultValue } from './daily-recurrence-rule.svelte';
import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { dailyRecurrenceRuleDeserializeWithContext } from './daily-recurrence-rule.svelte';
import { monthlyRecurrenceRuleDeserializeWithContext } from './monthly-recurrence-rule.svelte';
import { weeklyRecurrenceRuleDeserializeWithContext } from './weekly-recurrence-rule.svelte';
import { yearlyRecurrenceRuleDeserializeWithContext } from './yearly-recurrence-rule.svelte';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
import { monthlyRecurrenceRuleDefaultValue } from './monthly-recurrence-rule.svelte';
import { weeklyRecurrenceRuleDefaultValue } from './weekly-recurrence-rule.svelte';
import { yearlyRecurrenceRuleDefaultValue } from './yearly-recurrence-rule.svelte';
/** import macro {Gigaform} from "@playground/macro"; */

import type { YearlyRecurrenceRule } from './yearly-recurrence-rule.svelte';
import type { MonthlyRecurrenceRule } from './monthly-recurrence-rule.svelte';
import type { DailyRecurrenceRule } from './daily-recurrence-rule.svelte';
import type { WeeklyRecurrenceRule } from './weekly-recurrence-rule.svelte';

export type Interval =
    | /** @default */ DailyRecurrenceRule
    | WeeklyRecurrenceRule
    | MonthlyRecurrenceRule
    | YearlyRecurrenceRule;

export function intervalDefaultValue(): Interval {
    return dailyRecurrenceRuleDefaultValue();
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function intervalSerialize(
    value: Interval
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(intervalSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function intervalSerializeWithContext(value: Interval, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === 'function') {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

/** Deserializes input to this type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function intervalDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Result<Interval, Array<{ field: string; message: string }>> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = intervalDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Interval.deserialize: root cannot be a forward reference'
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function intervalDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): Interval | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Interval | PendingRef;
    }
    if (typeof value !== 'object' || value === null) {
        throw new DeserializeError([
            { field: '_root', message: 'Interval.deserializeWithContext: expected an object' }
        ]);
    }
    const __typeName = (value as any).__type;
    if (typeof __typeName !== 'string') {
        throw new DeserializeError([
            {
                field: '_root',
                message: 'Interval.deserializeWithContext: missing __type field for union dispatch'
            }
        ]);
    }
    if (__typeName === 'DailyRecurrenceRule') {
        return dailyRecurrenceRuleDeserializeWithContext(value, ctx) as Interval;
    }
    if (__typeName === 'WeeklyRecurrenceRule') {
        return weeklyRecurrenceRuleDeserializeWithContext(value, ctx) as Interval;
    }
    if (__typeName === 'MonthlyRecurrenceRule') {
        return monthlyRecurrenceRuleDeserializeWithContext(value, ctx) as Interval;
    }
    if (__typeName === 'YearlyRecurrenceRule') {
        return yearlyRecurrenceRuleDeserializeWithContext(value, ctx) as Interval;
    }
    throw new DeserializeError([
        {
            field: '_root',
            message:
                'Interval.deserializeWithContext: unknown type "' +
                __typeName +
                '". Expected one of: DailyRecurrenceRule, WeeklyRecurrenceRule, MonthlyRecurrenceRule, YearlyRecurrenceRule'
        }
    ]);
}
export function intervalIs(value: unknown): value is Interval {
    if (typeof value !== 'object' || value === null) {
        return false;
    }
    const __typeName = (value as any).__type;
    return (
        __typeName === 'DailyRecurrenceRule' ||
        __typeName === 'WeeklyRecurrenceRule' ||
        __typeName === 'MonthlyRecurrenceRule' ||
        __typeName === 'YearlyRecurrenceRule'
    );
}

/** Per-variant error types */ export type IntervalDailyRecurrenceRuleErrors = {
    _errors: Option<Array<string>>;
};
export type IntervalWeeklyRecurrenceRuleErrors = { _errors: Option<Array<string>> };
export type IntervalMonthlyRecurrenceRuleErrors = { _errors: Option<Array<string>> };
export type IntervalYearlyRecurrenceRuleErrors = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type IntervalDailyRecurrenceRuleTainted = {};
export type IntervalWeeklyRecurrenceRuleTainted = {};
export type IntervalMonthlyRecurrenceRuleTainted = {};
export type IntervalYearlyRecurrenceRuleTainted = {}; /** Union error type */
export type IntervalErrors =
    | ({ _type: 'DailyRecurrenceRule' } & IntervalDailyRecurrenceRuleErrors)
    | ({ _type: 'WeeklyRecurrenceRule' } & IntervalWeeklyRecurrenceRuleErrors)
    | ({ _type: 'MonthlyRecurrenceRule' } & IntervalMonthlyRecurrenceRuleErrors)
    | ({
          _type: 'YearlyRecurrenceRule';
      } & IntervalYearlyRecurrenceRuleErrors); /** Union tainted type */
export type IntervalTainted =
    | ({ _type: 'DailyRecurrenceRule' } & IntervalDailyRecurrenceRuleTainted)
    | ({ _type: 'WeeklyRecurrenceRule' } & IntervalWeeklyRecurrenceRuleTainted)
    | ({ _type: 'MonthlyRecurrenceRule' } & IntervalMonthlyRecurrenceRuleTainted)
    | ({
          _type: 'YearlyRecurrenceRule';
      } & IntervalYearlyRecurrenceRuleTainted); /** Per-variant field controller types */
export interface IntervalDailyRecurrenceRuleFieldControllers {}
export interface IntervalWeeklyRecurrenceRuleFieldControllers {}
export interface IntervalMonthlyRecurrenceRuleFieldControllers {}
export interface IntervalYearlyRecurrenceRuleFieldControllers {} /** Union Gigaform interface with variant switching */
export interface IntervalGigaform {
    readonly currentVariant:
        | 'DailyRecurrenceRule'
        | 'WeeklyRecurrenceRule'
        | 'MonthlyRecurrenceRule'
        | 'YearlyRecurrenceRule';
    readonly data: Interval;
    readonly errors: IntervalErrors;
    readonly tainted: IntervalTainted;
    readonly variants: IntervalVariantFields;
    switchVariant(
        variant:
            | 'DailyRecurrenceRule'
            | 'WeeklyRecurrenceRule'
            | 'MonthlyRecurrenceRule'
            | 'YearlyRecurrenceRule'
    ): void;
    validate(): Result<Interval, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Interval>): void;
} /** Variant fields container */
export interface IntervalVariantFields {
    readonly DailyRecurrenceRule: { readonly fields: IntervalDailyRecurrenceRuleFieldControllers };
    readonly WeeklyRecurrenceRule: {
        readonly fields: IntervalWeeklyRecurrenceRuleFieldControllers;
    };
    readonly MonthlyRecurrenceRule: {
        readonly fields: IntervalMonthlyRecurrenceRuleFieldControllers;
    };
    readonly YearlyRecurrenceRule: {
        readonly fields: IntervalYearlyRecurrenceRuleFieldControllers;
    };
} /** Gets default value for a specific variant */
function intervalGetDefaultForVariant(variant: string): Interval {
    switch (variant) {
        case 'DailyRecurrenceRule':
            return dailyRecurrenceRuleDefaultValue() as Interval;
        case 'WeeklyRecurrenceRule':
            return weeklyRecurrenceRuleDefaultValue() as Interval;
        case 'MonthlyRecurrenceRule':
            return monthlyRecurrenceRuleDefaultValue() as Interval;
        case 'YearlyRecurrenceRule':
            return yearlyRecurrenceRuleDefaultValue() as Interval;
        default:
            return dailyRecurrenceRuleDefaultValue() as Interval;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function intervalCreateForm(initial?: Interval): IntervalGigaform {
    const initialVariant:
        | 'DailyRecurrenceRule'
        | 'WeeklyRecurrenceRule'
        | 'MonthlyRecurrenceRule'
        | 'YearlyRecurrenceRule' = 'DailyRecurrenceRule';
    let currentVariant = $state<
        | 'DailyRecurrenceRule'
        | 'WeeklyRecurrenceRule'
        | 'MonthlyRecurrenceRule'
        | 'YearlyRecurrenceRule'
    >(initialVariant);
    let data = $state<Interval>(initial ?? intervalGetDefaultForVariant(initialVariant));
    let errors = $state<IntervalErrors>({} as IntervalErrors);
    let tainted = $state<IntervalTainted>({} as IntervalTainted);
    const variants: IntervalVariantFields = {
        DailyRecurrenceRule: {
            fields: {} as IntervalDailyRecurrenceRuleFieldControllers
        },
        WeeklyRecurrenceRule: {
            fields: {} as IntervalWeeklyRecurrenceRuleFieldControllers
        },
        MonthlyRecurrenceRule: {
            fields: {} as IntervalMonthlyRecurrenceRuleFieldControllers
        },
        YearlyRecurrenceRule: {
            fields: {} as IntervalYearlyRecurrenceRuleFieldControllers
        }
    };
    function switchVariant(
        variant:
            | 'DailyRecurrenceRule'
            | 'WeeklyRecurrenceRule'
            | 'MonthlyRecurrenceRule'
            | 'YearlyRecurrenceRule'
    ): void {
        currentVariant = variant;
        data = intervalGetDefaultForVariant(variant);
        errors = {} as IntervalErrors;
        tainted = {} as IntervalTainted;
    }
    function validate(): Result<Interval, Array<{ field: string; message: string }>> {
        return intervalFromObject(data);
    }
    function reset(overrides?: Partial<Interval>): void {
        data = overrides
            ? (overrides as typeof data)
            : intervalGetDefaultForVariant(currentVariant);
        errors = {} as IntervalErrors;
        tainted = {} as IntervalTainted;
    }
    return {
        get currentVariant() {
            return currentVariant;
        },
        get data() {
            return data;
        },
        set data(v) {
            data = v;
        },
        get errors() {
            return errors;
        },
        set errors(v) {
            errors = v;
        },
        get tainted() {
            return tainted;
        },
        set tainted(v) {
            tainted = v;
        },
        variants,
        switchVariant,
        validate,
        reset
    };
} /** Parses FormData for union type, determining variant from discriminant field */
export function intervalFromFormData(
    formData: FormData
): Result<Interval, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_type') as
        | 'DailyRecurrenceRule'
        | 'WeeklyRecurrenceRule'
        | 'MonthlyRecurrenceRule'
        | 'YearlyRecurrenceRule'
        | null;
    if (!discriminant) {
        return Result.err([{ field: '_type', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._type = discriminant;
    if (discriminant === 'DailyRecurrenceRule') {
    } else if (discriminant === 'WeeklyRecurrenceRule') {
    } else if (discriminant === 'MonthlyRecurrenceRule') {
    } else if (discriminant === 'YearlyRecurrenceRule') {
    }
    return intervalFromStringifiedJSON(JSON.stringify(obj));
}

export const Interval = {
    defaultValue: intervalDefaultValue,
    serialize: intervalSerialize,
    serializeWithContext: intervalSerializeWithContext,
    deserialize: intervalDeserialize,
    deserializeWithContext: intervalDeserializeWithContext,
    is: intervalIs,
    createForm: intervalCreateForm,
    fromFormData: intervalFromFormData
} as const;
