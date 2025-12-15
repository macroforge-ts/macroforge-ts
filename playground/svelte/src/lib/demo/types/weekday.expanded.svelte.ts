import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';

export type Weekday =
    | /** @default */ 'Monday'
    | 'Tuesday'
    | 'Wednesday'
    | 'Thursday'
    | 'Friday'
    | 'Saturday'
    | 'Sunday';

export function defaultValueWeekday(): Weekday {
    return 'Monday';
}

export function toStringifiedJSONWeekday(value: Weekday): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeWeekday(value, ctx));
}
export function toObjectWeekday(value: Weekday): unknown {
    const ctx = SerializeContext.create();
    return __serializeWeekday(value, ctx);
}
export function __serializeWeekday(value: Weekday, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONWeekday(
    json: string,
    opts?: DeserializeOptions
): Result<Weekday, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectWeekday(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectWeekday(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Weekday, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeWeekday(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Weekday.fromObject: root cannot be a forward reference'
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
export function __deserializeWeekday(value: any, ctx: DeserializeContext): Weekday | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Weekday | PendingRef;
    }
    const allowedValues = [
        'Monday',
        'Tuesday',
        'Wednesday',
        'Thursday',
        'Friday',
        'Saturday',
        'Sunday'
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new DeserializeError([
            {
                field: '_root',
                message:
                    'Invalid value for Weekday: expected one of ' +
                    allowedValues.map((v) => JSON.stringify(v)).join(', ') +
                    ', got ' +
                    JSON.stringify(value)
            }
        ]);
    }
    return value as Weekday;
}
export function isWeekday(value: unknown): value is Weekday {
    const allowedValues = [
        'Monday',
        'Tuesday',
        'Wednesday',
        'Thursday',
        'Friday',
        'Saturday',
        'Sunday'
    ] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type MondayErrorsWeekday = { _errors: Option<Array<string>> };
export type TuesdayErrorsWeekday = { _errors: Option<Array<string>> };
export type WednesdayErrorsWeekday = { _errors: Option<Array<string>> };
export type ThursdayErrorsWeekday = { _errors: Option<Array<string>> };
export type FridayErrorsWeekday = { _errors: Option<Array<string>> };
export type SaturdayErrorsWeekday = { _errors: Option<Array<string>> };
export type SundayErrorsWeekday = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type MondayTaintedWeekday = {};
export type TuesdayTaintedWeekday = {};
export type WednesdayTaintedWeekday = {};
export type ThursdayTaintedWeekday = {};
export type FridayTaintedWeekday = {};
export type SaturdayTaintedWeekday = {};
export type SundayTaintedWeekday = {}; /** Union error type */
export type ErrorsWeekday =
    | ({ _value: 'Monday' } & MondayErrorsWeekday)
    | ({ _value: 'Tuesday' } & TuesdayErrorsWeekday)
    | ({ _value: 'Wednesday' } & WednesdayErrorsWeekday)
    | ({ _value: 'Thursday' } & ThursdayErrorsWeekday)
    | ({ _value: 'Friday' } & FridayErrorsWeekday)
    | ({ _value: 'Saturday' } & SaturdayErrorsWeekday)
    | ({ _value: 'Sunday' } & SundayErrorsWeekday); /** Union tainted type */
export type TaintedWeekday =
    | ({ _value: 'Monday' } & MondayTaintedWeekday)
    | ({ _value: 'Tuesday' } & TuesdayTaintedWeekday)
    | ({ _value: 'Wednesday' } & WednesdayTaintedWeekday)
    | ({ _value: 'Thursday' } & ThursdayTaintedWeekday)
    | ({ _value: 'Friday' } & FridayTaintedWeekday)
    | ({ _value: 'Saturday' } & SaturdayTaintedWeekday)
    | ({ _value: 'Sunday' } & SundayTaintedWeekday); /** Per-variant field controller types */
export interface MondayFieldControllersWeekday {}
export interface TuesdayFieldControllersWeekday {}
export interface WednesdayFieldControllersWeekday {}
export interface ThursdayFieldControllersWeekday {}
export interface FridayFieldControllersWeekday {}
export interface SaturdayFieldControllersWeekday {}
export interface SundayFieldControllersWeekday {} /** Union Gigaform interface with variant switching */
export interface GigaformWeekday {
    readonly currentVariant:
        | 'Monday'
        | 'Tuesday'
        | 'Wednesday'
        | 'Thursday'
        | 'Friday'
        | 'Saturday'
        | 'Sunday';
    readonly data: Weekday;
    readonly errors: ErrorsWeekday;
    readonly tainted: TaintedWeekday;
    readonly variants: VariantFieldsWeekday;
    switchVariant(
        variant: 'Monday' | 'Tuesday' | 'Wednesday' | 'Thursday' | 'Friday' | 'Saturday' | 'Sunday'
    ): void;
    validate(): Result<Weekday, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Weekday>): void;
} /** Variant fields container */
export interface VariantFieldsWeekday {
    readonly Monday: { readonly fields: MondayFieldControllersWeekday };
    readonly Tuesday: { readonly fields: TuesdayFieldControllersWeekday };
    readonly Wednesday: { readonly fields: WednesdayFieldControllersWeekday };
    readonly Thursday: { readonly fields: ThursdayFieldControllersWeekday };
    readonly Friday: { readonly fields: FridayFieldControllersWeekday };
    readonly Saturday: { readonly fields: SaturdayFieldControllersWeekday };
    readonly Sunday: { readonly fields: SundayFieldControllersWeekday };
} /** Gets default value for a specific variant */
function getDefaultForVariantWeekday(variant: string): Weekday {
    switch (variant) {
        case 'Monday':
            return 'Monday' as Weekday;
        case 'Tuesday':
            return 'Tuesday' as Weekday;
        case 'Wednesday':
            return 'Wednesday' as Weekday;
        case 'Thursday':
            return 'Thursday' as Weekday;
        case 'Friday':
            return 'Friday' as Weekday;
        case 'Saturday':
            return 'Saturday' as Weekday;
        case 'Sunday':
            return 'Sunday' as Weekday;
        default:
            return 'Monday' as Weekday;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function createFormWeekday(initial?: Weekday): GigaformWeekday {
    const initialVariant:
        | 'Monday'
        | 'Tuesday'
        | 'Wednesday'
        | 'Thursday'
        | 'Friday'
        | 'Saturday'
        | 'Sunday' =
        (initial as
            | 'Monday'
            | 'Tuesday'
            | 'Wednesday'
            | 'Thursday'
            | 'Friday'
            | 'Saturday'
            | 'Sunday') ?? 'Monday';
    let currentVariant = $state<
        'Monday' | 'Tuesday' | 'Wednesday' | 'Thursday' | 'Friday' | 'Saturday' | 'Sunday'
    >(initialVariant);
    let data = $state<Weekday>(initial ?? getDefaultForVariantWeekday(initialVariant));
    let errors = $state<ErrorsWeekday>({} as ErrorsWeekday);
    let tainted = $state<TaintedWeekday>({} as TaintedWeekday);
    const variants: VariantFieldsWeekday = {
        Monday: {
            fields: {} as MondayFieldControllersWeekday
        },
        Tuesday: {
            fields: {} as TuesdayFieldControllersWeekday
        },
        Wednesday: {
            fields: {} as WednesdayFieldControllersWeekday
        },
        Thursday: {
            fields: {} as ThursdayFieldControllersWeekday
        },
        Friday: {
            fields: {} as FridayFieldControllersWeekday
        },
        Saturday: {
            fields: {} as SaturdayFieldControllersWeekday
        },
        Sunday: {
            fields: {} as SundayFieldControllersWeekday
        }
    };
    function switchVariant(
        variant: 'Monday' | 'Tuesday' | 'Wednesday' | 'Thursday' | 'Friday' | 'Saturday' | 'Sunday'
    ): void {
        currentVariant = variant;
        data = getDefaultForVariantWeekday(variant);
        errors = {} as ErrorsWeekday;
        tainted = {} as TaintedWeekday;
    }
    function validate(): Result<Weekday, Array<{ field: string; message: string }>> {
        return fromObjectWeekday(data);
    }
    function reset(overrides?: Partial<Weekday>): void {
        data = overrides ? (overrides as typeof data) : getDefaultForVariantWeekday(currentVariant);
        errors = {} as ErrorsWeekday;
        tainted = {} as TaintedWeekday;
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
export function fromFormDataWeekday(
    formData: FormData
): Result<Weekday, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_value') as
        | 'Monday'
        | 'Tuesday'
        | 'Wednesday'
        | 'Thursday'
        | 'Friday'
        | 'Saturday'
        | 'Sunday'
        | null;
    if (!discriminant) {
        return Result.err([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'Monday') {
    } else if (discriminant === 'Tuesday') {
    } else if (discriminant === 'Wednesday') {
    } else if (discriminant === 'Thursday') {
    } else if (discriminant === 'Friday') {
    } else if (discriminant === 'Saturday') {
    } else if (discriminant === 'Sunday') {
    }
    return fromStringifiedJSONWeekday(JSON.stringify(obj));
}
