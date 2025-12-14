import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';

export type WeekOfMonth = /** @default */ 'First' | 'Second' | 'Third' | 'Fourth' | 'Last';

export function defaultValueWeekOfMonth(): WeekOfMonth {
    return 'First';
}

export function toStringifiedJSONWeekOfMonth(value: WeekOfMonth): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeWeekOfMonth(value, ctx));
}
export function toObjectWeekOfMonth(value: WeekOfMonth): unknown {
    const ctx = SerializeContext.create();
    return __serializeWeekOfMonth(value, ctx);
}
export function __serializeWeekOfMonth(value: WeekOfMonth, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONWeekOfMonth(
    json: string,
    opts?: DeserializeOptions
): Result<WeekOfMonth, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectWeekOfMonth(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectWeekOfMonth(
    obj: unknown,
    opts?: DeserializeOptions
): Result<WeekOfMonth, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeWeekOfMonth(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'WeekOfMonth.fromObject: root cannot be a forward reference'
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
export function __deserializeWeekOfMonth(
    value: any,
    ctx: DeserializeContext
): WeekOfMonth | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as WeekOfMonth | PendingRef;
    }
    const allowedValues = ['First', 'Second', 'Third', 'Fourth', 'Last'] as const;
    if (!allowedValues.includes(value)) {
        throw new DeserializeError([
            {
                field: '_root',
                message:
                    'Invalid value for WeekOfMonth: expected one of ' +
                    allowedValues.map((v) => JSON.stringify(v)).join(', ') +
                    ', got ' +
                    JSON.stringify(value)
            }
        ]);
    }
    return value as WeekOfMonth;
}
export function isWeekOfMonth(value: unknown): value is WeekOfMonth {
    const allowedValues = ['First', 'Second', 'Third', 'Fourth', 'Last'] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type FirstErrorsWeekOfMonth = {
    _errors: Option<Array<string>>;
};
export type SecondErrorsWeekOfMonth = { _errors: Option<Array<string>> };
export type ThirdErrorsWeekOfMonth = { _errors: Option<Array<string>> };
export type FourthErrorsWeekOfMonth = { _errors: Option<Array<string>> };
export type LastErrorsWeekOfMonth = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type FirstTaintedWeekOfMonth = {};
export type SecondTaintedWeekOfMonth = {};
export type ThirdTaintedWeekOfMonth = {};
export type FourthTaintedWeekOfMonth = {};
export type LastTaintedWeekOfMonth = {}; /** Union error type */
export type ErrorsWeekOfMonth =
    | ({ _value: 'First' } & FirstErrorsWeekOfMonth)
    | ({ _value: 'Second' } & SecondErrorsWeekOfMonth)
    | ({ _value: 'Third' } & ThirdErrorsWeekOfMonth)
    | ({ _value: 'Fourth' } & FourthErrorsWeekOfMonth)
    | ({ _value: 'Last' } & LastErrorsWeekOfMonth); /** Union tainted type */
export type TaintedWeekOfMonth =
    | ({ _value: 'First' } & FirstTaintedWeekOfMonth)
    | ({ _value: 'Second' } & SecondTaintedWeekOfMonth)
    | ({ _value: 'Third' } & ThirdTaintedWeekOfMonth)
    | ({ _value: 'Fourth' } & FourthTaintedWeekOfMonth)
    | ({ _value: 'Last' } & LastTaintedWeekOfMonth); /** Per-variant field controller types */
export interface FirstFieldControllersWeekOfMonth {}
export interface SecondFieldControllersWeekOfMonth {}
export interface ThirdFieldControllersWeekOfMonth {}
export interface FourthFieldControllersWeekOfMonth {}
export interface LastFieldControllersWeekOfMonth {} /** Union Gigaform interface with variant switching */
export interface GigaformWeekOfMonth {
    readonly currentVariant: 'First' | 'Second' | 'Third' | 'Fourth' | 'Last';
    readonly data: WeekOfMonth;
    readonly errors: ErrorsWeekOfMonth;
    readonly tainted: TaintedWeekOfMonth;
    readonly variants: VariantFieldsWeekOfMonth;
    switchVariant(variant: 'First' | 'Second' | 'Third' | 'Fourth' | 'Last'): void;
    validate(): Result<WeekOfMonth, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<WeekOfMonth>): void;
} /** Variant fields container */
export interface VariantFieldsWeekOfMonth {
    readonly First: { readonly fields: FirstFieldControllersWeekOfMonth };
    readonly Second: { readonly fields: SecondFieldControllersWeekOfMonth };
    readonly Third: { readonly fields: ThirdFieldControllersWeekOfMonth };
    readonly Fourth: { readonly fields: FourthFieldControllersWeekOfMonth };
    readonly Last: { readonly fields: LastFieldControllersWeekOfMonth };
} /** Gets default value for a specific variant */
function getDefaultForVariantWeekOfMonth(variant: string): WeekOfMonth {
    switch (variant) {
        case 'First':
            return 'First' as WeekOfMonth;
        case 'Second':
            return 'Second' as WeekOfMonth;
        case 'Third':
            return 'Third' as WeekOfMonth;
        case 'Fourth':
            return 'Fourth' as WeekOfMonth;
        case 'Last':
            return 'Last' as WeekOfMonth;
        default:
            return 'First' as WeekOfMonth;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function createFormWeekOfMonth(initial?: WeekOfMonth): GigaformWeekOfMonth {
    const initialVariant: 'First' | 'Second' | 'Third' | 'Fourth' | 'Last' =
        (initial as 'First' | 'Second' | 'Third' | 'Fourth' | 'Last') ?? 'First';
    let currentVariant = $state<'First' | 'Second' | 'Third' | 'Fourth' | 'Last'>(initialVariant);
    let data = $state<WeekOfMonth>(initial ?? getDefaultForVariantWeekOfMonth(initialVariant));
    let errors = $state<ErrorsWeekOfMonth>({} as ErrorsWeekOfMonth);
    let tainted = $state<TaintedWeekOfMonth>({} as TaintedWeekOfMonth);
    const variants: VariantFieldsWeekOfMonth = {
        First: {
            fields: {} as FirstFieldControllersWeekOfMonth
        },
        Second: {
            fields: {} as SecondFieldControllersWeekOfMonth
        },
        Third: {
            fields: {} as ThirdFieldControllersWeekOfMonth
        },
        Fourth: {
            fields: {} as FourthFieldControllersWeekOfMonth
        },
        Last: {
            fields: {} as LastFieldControllersWeekOfMonth
        }
    };
    function switchVariant(variant: 'First' | 'Second' | 'Third' | 'Fourth' | 'Last'): void {
        currentVariant = variant;
        data = getDefaultForVariantWeekOfMonth(variant);
        errors = {} as ErrorsWeekOfMonth;
        tainted = {} as TaintedWeekOfMonth;
    }
    function validate(): Result<WeekOfMonth, Array<{ field: string; message: string }>> {
        return WeekOfMonth.fromObject(data);
    }
    function reset(overrides?: Partial<WeekOfMonth>): void {
        data = overrides
            ? (overrides as typeof data)
            : getDefaultForVariantWeekOfMonth(currentVariant);
        errors = {} as ErrorsWeekOfMonth;
        tainted = {} as TaintedWeekOfMonth;
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
export function fromFormDataWeekOfMonth(
    formData: FormData
): Result<WeekOfMonth, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_value') as
        | 'First'
        | 'Second'
        | 'Third'
        | 'Fourth'
        | 'Last'
        | null;
    if (!discriminant) {
        return Result.err([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'First') {
    } else if (discriminant === 'Second') {
    } else if (discriminant === 'Third') {
    } else if (discriminant === 'Fourth') {
    } else if (discriminant === 'Last') {
    }
    return WeekOfMonth.fromStringifiedJSON(JSON.stringify(obj));
}
