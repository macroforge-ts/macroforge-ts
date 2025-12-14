import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';

export type IntervalUnit = /** @default */ 'Day' | 'Week' | 'Month' | 'Year';

export function defaultValueIntervalUnit(): IntervalUnit {
    return 'Day';
}

export function toStringifiedJSONIntervalUnit(value: IntervalUnit): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeIntervalUnit(value, ctx));
}
export function toObjectIntervalUnit(value: IntervalUnit): unknown {
    const ctx = SerializeContext.create();
    return __serializeIntervalUnit(value, ctx);
}
export function __serializeIntervalUnit(value: IntervalUnit, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONIntervalUnit(
    json: string,
    opts?: DeserializeOptions
): Result<IntervalUnit, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectIntervalUnit(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectIntervalUnit(
    obj: unknown,
    opts?: DeserializeOptions
): Result<IntervalUnit, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeIntervalUnit(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'IntervalUnit.fromObject: root cannot be a forward reference'
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
export function __deserializeIntervalUnit(
    value: any,
    ctx: DeserializeContext
): IntervalUnit | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as IntervalUnit | PendingRef;
    }
    const allowedValues = ['Day', 'Week', 'Month', 'Year'] as const;
    if (!allowedValues.includes(value)) {
        throw new DeserializeError([
            {
                field: '_root',
                message:
                    'Invalid value for IntervalUnit: expected one of ' +
                    allowedValues.map((v) => JSON.stringify(v)).join(', ') +
                    ', got ' +
                    JSON.stringify(value)
            }
        ]);
    }
    return value as IntervalUnit;
}
export function isIntervalUnit(value: unknown): value is IntervalUnit {
    const allowedValues = ['Day', 'Week', 'Month', 'Year'] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type DayErrorsIntervalUnit = {
    _errors: Option<Array<string>>;
};
export type WeekErrorsIntervalUnit = { _errors: Option<Array<string>> };
export type MonthErrorsIntervalUnit = { _errors: Option<Array<string>> };
export type YearErrorsIntervalUnit = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type DayTaintedIntervalUnit = {};
export type WeekTaintedIntervalUnit = {};
export type MonthTaintedIntervalUnit = {};
export type YearTaintedIntervalUnit = {}; /** Union error type */
export type ErrorsIntervalUnit =
    | ({ _value: 'Day' } & DayErrorsIntervalUnit)
    | ({ _value: 'Week' } & WeekErrorsIntervalUnit)
    | ({ _value: 'Month' } & MonthErrorsIntervalUnit)
    | ({ _value: 'Year' } & YearErrorsIntervalUnit); /** Union tainted type */
export type TaintedIntervalUnit =
    | ({ _value: 'Day' } & DayTaintedIntervalUnit)
    | ({ _value: 'Week' } & WeekTaintedIntervalUnit)
    | ({ _value: 'Month' } & MonthTaintedIntervalUnit)
    | ({ _value: 'Year' } & YearTaintedIntervalUnit); /** Per-variant field controller types */
export interface DayFieldControllersIntervalUnit {}
export interface WeekFieldControllersIntervalUnit {}
export interface MonthFieldControllersIntervalUnit {}
export interface YearFieldControllersIntervalUnit {} /** Union Gigaform interface with variant switching */
export interface GigaformIntervalUnit {
    readonly currentVariant: 'Day' | 'Week' | 'Month' | 'Year';
    readonly data: IntervalUnit;
    readonly errors: ErrorsIntervalUnit;
    readonly tainted: TaintedIntervalUnit;
    readonly variants: VariantFieldsIntervalUnit;
    switchVariant(variant: 'Day' | 'Week' | 'Month' | 'Year'): void;
    validate(): Result<IntervalUnit, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<IntervalUnit>): void;
} /** Variant fields container */
export interface VariantFieldsIntervalUnit {
    readonly Day: { readonly fields: DayFieldControllersIntervalUnit };
    readonly Week: { readonly fields: WeekFieldControllersIntervalUnit };
    readonly Month: { readonly fields: MonthFieldControllersIntervalUnit };
    readonly Year: { readonly fields: YearFieldControllersIntervalUnit };
} /** Gets default value for a specific variant */
function getDefaultForVariantIntervalUnit(variant: string): IntervalUnit {
    switch (variant) {
        case 'Day':
            return 'Day' as IntervalUnit;
        case 'Week':
            return 'Week' as IntervalUnit;
        case 'Month':
            return 'Month' as IntervalUnit;
        case 'Year':
            return 'Year' as IntervalUnit;
        default:
            return 'Day' as IntervalUnit;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function createFormIntervalUnit(initial?: IntervalUnit): GigaformIntervalUnit {
    const initialVariant: 'Day' | 'Week' | 'Month' | 'Year' =
        (initial as 'Day' | 'Week' | 'Month' | 'Year') ?? 'Day';
    let currentVariant = $state<'Day' | 'Week' | 'Month' | 'Year'>(initialVariant);
    let data = $state<IntervalUnit>(initial ?? getDefaultForVariantIntervalUnit(initialVariant));
    let errors = $state<ErrorsIntervalUnit>({} as ErrorsIntervalUnit);
    let tainted = $state<TaintedIntervalUnit>({} as TaintedIntervalUnit);
    const variants: VariantFieldsIntervalUnit = {
        Day: {
            fields: {} as DayFieldControllersIntervalUnit
        },
        Week: {
            fields: {} as WeekFieldControllersIntervalUnit
        },
        Month: {
            fields: {} as MonthFieldControllersIntervalUnit
        },
        Year: {
            fields: {} as YearFieldControllersIntervalUnit
        }
    };
    function switchVariant(variant: 'Day' | 'Week' | 'Month' | 'Year'): void {
        currentVariant = variant;
        data = getDefaultForVariantIntervalUnit(variant);
        errors = {} as ErrorsIntervalUnit;
        tainted = {} as TaintedIntervalUnit;
    }
    function validate(): Result<IntervalUnit, Array<{ field: string; message: string }>> {
        return IntervalUnit.fromObject(data);
    }
    function reset(overrides?: Partial<IntervalUnit>): void {
        data = overrides
            ? (overrides as typeof data)
            : getDefaultForVariantIntervalUnit(currentVariant);
        errors = {} as ErrorsIntervalUnit;
        tainted = {} as TaintedIntervalUnit;
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
export function fromFormDataIntervalUnit(
    formData: FormData
): Result<IntervalUnit, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_value') as 'Day' | 'Week' | 'Month' | 'Year' | null;
    if (!discriminant) {
        return Result.err([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'Day') {
    } else if (discriminant === 'Week') {
    } else if (discriminant === 'Month') {
    } else if (discriminant === 'Year') {
    }
    return IntervalUnit.fromStringifiedJSON(JSON.stringify(obj));
}
