import { defaultValueDailyRecurrenceRule } from './daily-recurrence-rule.svelte';
import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { __deserializeDailyRecurrenceRule } from './daily-recurrence-rule.svelte';
import { __deserializeMonthlyRecurrenceRule } from './monthly-recurrence-rule.svelte';
import { __deserializeWeeklyRecurrenceRule } from './weekly-recurrence-rule.svelte';
import { __deserializeYearlyRecurrenceRule } from './yearly-recurrence-rule.svelte';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
import { defaultValueMonthlyRecurrenceRule } from './monthly-recurrence-rule.svelte';
import { defaultValueWeeklyRecurrenceRule } from './weekly-recurrence-rule.svelte';
import { defaultValueYearlyRecurrenceRule } from './yearly-recurrence-rule.svelte';
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

export function defaultValueInterval(): Interval {
    return defaultValueDailyRecurrenceRule();
}

export function toStringifiedJSONInterval(value: Interval): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeInterval(value, ctx));
}
export function toObjectInterval(value: Interval): unknown {
    const ctx = SerializeContext.create();
    return __serializeInterval(value, ctx);
}
export function __serializeInterval(value: Interval, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONInterval(
    json: string,
    opts?: DeserializeOptions
): Result<Interval, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectInterval(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectInterval(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Interval, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeInterval(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Interval.fromObject: root cannot be a forward reference'
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
}
export function __deserializeInterval(value: any, ctx: DeserializeContext): Interval | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Interval | PendingRef;
    }
    if (typeof value !== 'object' || value === null) {
        throw new DeserializeError([
            { field: '_root', message: 'Interval.__deserialize: expected an object' }
        ]);
    }
    const __typeName = (value as any).__type;
    if (typeof __typeName !== 'string') {
        throw new DeserializeError([
            {
                field: '_root',
                message: 'Interval.__deserialize: missing __type field for union dispatch'
            }
        ]);
    }
    if (__typeName === 'DailyRecurrenceRule') {
        return __deserializeDailyRecurrenceRule(value, ctx) as Interval;
    }
    if (__typeName === 'WeeklyRecurrenceRule') {
        return __deserializeWeeklyRecurrenceRule(value, ctx) as Interval;
    }
    if (__typeName === 'MonthlyRecurrenceRule') {
        return __deserializeMonthlyRecurrenceRule(value, ctx) as Interval;
    }
    if (__typeName === 'YearlyRecurrenceRule') {
        return __deserializeYearlyRecurrenceRule(value, ctx) as Interval;
    }
    throw new DeserializeError([
        {
            field: '_root',
            message:
                'Interval.__deserialize: unknown type "' +
                __typeName +
                '". Expected one of: DailyRecurrenceRule, WeeklyRecurrenceRule, MonthlyRecurrenceRule, YearlyRecurrenceRule'
        }
    ]);
}
export function isInterval(value: unknown): value is Interval {
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

/** Per-variant error types */ export type DailyRecurrenceRuleErrorsInterval = {
    _errors: Option<Array<string>>;
};
export type WeeklyRecurrenceRuleErrorsInterval = { _errors: Option<Array<string>> };
export type MonthlyRecurrenceRuleErrorsInterval = { _errors: Option<Array<string>> };
export type YearlyRecurrenceRuleErrorsInterval = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type DailyRecurrenceRuleTaintedInterval = {};
export type WeeklyRecurrenceRuleTaintedInterval = {};
export type MonthlyRecurrenceRuleTaintedInterval = {};
export type YearlyRecurrenceRuleTaintedInterval = {}; /** Union error type */
export type ErrorsInterval =
    | ({ _type: 'DailyRecurrenceRule' } & DailyRecurrenceRuleErrorsInterval)
    | ({ _type: 'WeeklyRecurrenceRule' } & WeeklyRecurrenceRuleErrorsInterval)
    | ({ _type: 'MonthlyRecurrenceRule' } & MonthlyRecurrenceRuleErrorsInterval)
    | ({
          _type: 'YearlyRecurrenceRule';
      } & YearlyRecurrenceRuleErrorsInterval); /** Union tainted type */
export type TaintedInterval =
    | ({ _type: 'DailyRecurrenceRule' } & DailyRecurrenceRuleTaintedInterval)
    | ({ _type: 'WeeklyRecurrenceRule' } & WeeklyRecurrenceRuleTaintedInterval)
    | ({ _type: 'MonthlyRecurrenceRule' } & MonthlyRecurrenceRuleTaintedInterval)
    | ({
          _type: 'YearlyRecurrenceRule';
      } & YearlyRecurrenceRuleTaintedInterval); /** Per-variant field controller types */
export interface DailyRecurrenceRuleFieldControllersInterval {}
export interface WeeklyRecurrenceRuleFieldControllersInterval {}
export interface MonthlyRecurrenceRuleFieldControllersInterval {}
export interface YearlyRecurrenceRuleFieldControllersInterval {} /** Union Gigaform interface with variant switching */
export interface GigaformInterval {
    readonly currentVariant:
        | 'DailyRecurrenceRule'
        | 'WeeklyRecurrenceRule'
        | 'MonthlyRecurrenceRule'
        | 'YearlyRecurrenceRule';
    readonly data: Interval;
    readonly errors: ErrorsInterval;
    readonly tainted: TaintedInterval;
    readonly variants: VariantFieldsInterval;
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
export interface VariantFieldsInterval {
    readonly DailyRecurrenceRule: { readonly fields: DailyRecurrenceRuleFieldControllersInterval };
    readonly WeeklyRecurrenceRule: {
        readonly fields: WeeklyRecurrenceRuleFieldControllersInterval;
    };
    readonly MonthlyRecurrenceRule: {
        readonly fields: MonthlyRecurrenceRuleFieldControllersInterval;
    };
    readonly YearlyRecurrenceRule: {
        readonly fields: YearlyRecurrenceRuleFieldControllersInterval;
    };
} /** Gets default value for a specific variant */
function getDefaultForVariantInterval(variant: string): Interval {
    switch (variant) {
        case 'DailyRecurrenceRule':
            return defaultValueDailyRecurrenceRule() as Interval;
        case 'WeeklyRecurrenceRule':
            return defaultValueWeeklyRecurrenceRule() as Interval;
        case 'MonthlyRecurrenceRule':
            return defaultValueMonthlyRecurrenceRule() as Interval;
        case 'YearlyRecurrenceRule':
            return defaultValueYearlyRecurrenceRule() as Interval;
        default:
            return defaultValueDailyRecurrenceRule() as Interval;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function createFormInterval(initial?: Interval): GigaformInterval {
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
    let data = $state<Interval>(initial ?? getDefaultForVariantInterval(initialVariant));
    let errors = $state<ErrorsInterval>({} as ErrorsInterval);
    let tainted = $state<TaintedInterval>({} as TaintedInterval);
    const variants: VariantFieldsInterval = {
        DailyRecurrenceRule: {
            fields: {} as DailyRecurrenceRuleFieldControllersInterval
        },
        WeeklyRecurrenceRule: {
            fields: {} as WeeklyRecurrenceRuleFieldControllersInterval
        },
        MonthlyRecurrenceRule: {
            fields: {} as MonthlyRecurrenceRuleFieldControllersInterval
        },
        YearlyRecurrenceRule: {
            fields: {} as YearlyRecurrenceRuleFieldControllersInterval
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
        data = getDefaultForVariantInterval(variant);
        errors = {} as ErrorsInterval;
        tainted = {} as TaintedInterval;
    }
    function validate(): Result<Interval, Array<{ field: string; message: string }>> {
        return fromObjectInterval(data);
    }
    function reset(overrides?: Partial<Interval>): void {
        data = overrides
            ? (overrides as typeof data)
            : getDefaultForVariantInterval(currentVariant);
        errors = {} as ErrorsInterval;
        tainted = {} as TaintedInterval;
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
export function fromFormDataInterval(
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
    return fromStringifiedJSONInterval(JSON.stringify(obj));
}
