import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { Result } from '@playground/macro/gigaform';
import { Option } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';

export type IntervalUnit = /** @default */ 'Day' | 'Week' | 'Month' | 'Year';

export function intervalUnitDefaultValue(): IntervalUnit {
    return 'Day';
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function intervalUnitSerialize(
    value: IntervalUnit
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(intervalUnitSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function intervalUnitSerializeWithContext(
    value: IntervalUnit,
    ctx: __mf_SerializeContext
): unknown {
    if (typeof (value as any)?.serializeWithContext === 'function') {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

/** Deserializes input to this type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function intervalUnitDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, IntervalUnit> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = intervalUnitDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'IntervalUnit.deserialize: root cannot be a forward reference'
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return __mf_exitSucceed(resultOrRef);
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return __mf_exitFail(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return __mf_exitFail([{ field: '_root', message }]);
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function intervalUnitDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): IntervalUnit | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as IntervalUnit | __mf_PendingRef;
    }
    const allowedValues = ['Day', 'Week', 'Month', 'Year'] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
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
export function intervalUnitIs(value: unknown): value is IntervalUnit {
    const allowedValues = ['Day', 'Week', 'Month', 'Year'] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type IntervalUnitDayErrors = {
    _errors: Option<Array<string>>;
};
export type IntervalUnitWeekErrors = { _errors: Option<Array<string>> };
export type IntervalUnitMonthErrors = { _errors: Option<Array<string>> };
export type IntervalUnitYearErrors = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type IntervalUnitDayTainted = {};
export type IntervalUnitWeekTainted = {};
export type IntervalUnitMonthTainted = {};
export type IntervalUnitYearTainted = {}; /** Union error type */
export type IntervalUnitErrors =
    | ({ _value: 'Day' } & IntervalUnitDayErrors)
    | ({ _value: 'Week' } & IntervalUnitWeekErrors)
    | ({ _value: 'Month' } & IntervalUnitMonthErrors)
    | ({ _value: 'Year' } & IntervalUnitYearErrors); /** Union tainted type */
export type IntervalUnitTainted =
    | ({ _value: 'Day' } & IntervalUnitDayTainted)
    | ({ _value: 'Week' } & IntervalUnitWeekTainted)
    | ({ _value: 'Month' } & IntervalUnitMonthTainted)
    | ({ _value: 'Year' } & IntervalUnitYearTainted); /** Per-variant field controller types */
export interface IntervalUnitDayFieldControllers {}
export interface IntervalUnitWeekFieldControllers {}
export interface IntervalUnitMonthFieldControllers {}
export interface IntervalUnitYearFieldControllers {} /** Union Gigaform interface with variant switching */
export interface IntervalUnitGigaform {
    readonly currentVariant: 'Day' | 'Week' | 'Month' | 'Year';
    readonly data: IntervalUnit;
    readonly errors: IntervalUnitErrors;
    readonly tainted: IntervalUnitTainted;
    readonly variants: IntervalUnitVariantFields;
    switchVariant(variant: 'Day' | 'Week' | 'Month' | 'Year'): void;
    validate(): Result<IntervalUnit, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<IntervalUnit>): void;
} /** Variant fields container */
export interface IntervalUnitVariantFields {
    readonly Day: { readonly fields: IntervalUnitDayFieldControllers };
    readonly Week: { readonly fields: IntervalUnitWeekFieldControllers };
    readonly Month: { readonly fields: IntervalUnitMonthFieldControllers };
    readonly Year: { readonly fields: IntervalUnitYearFieldControllers };
} /** Gets default value for a specific variant */
function intervalUnitGetDefaultForVariant(variant: string): IntervalUnit {
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
export function intervalUnitCreateForm(initial?: IntervalUnit): IntervalUnitGigaform {
    const initialVariant: 'Day' | 'Week' | 'Month' | 'Year' =
        (initial as 'Day' | 'Week' | 'Month' | 'Year') ?? 'Day';
    let currentVariant = $state<'Day' | 'Week' | 'Month' | 'Year'>(initialVariant);
    let data = $state<IntervalUnit>(initial ?? intervalUnitGetDefaultForVariant(initialVariant));
    let errors = $state<IntervalUnitErrors>({} as IntervalUnitErrors);
    let tainted = $state<IntervalUnitTainted>({} as IntervalUnitTainted);
    const variants: IntervalUnitVariantFields = {
        Day: { fields: {} as IntervalUnitDayFieldControllers },
        Week: { fields: {} as IntervalUnitWeekFieldControllers },
        Month: { fields: {} as IntervalUnitMonthFieldControllers },
        Year: { fields: {} as IntervalUnitYearFieldControllers }
    };
    function switchVariant(variant: 'Day' | 'Week' | 'Month' | 'Year'): void {
        currentVariant = variant;
        data = intervalUnitGetDefaultForVariant(variant);
        errors = {} as IntervalUnitErrors;
        tainted = {} as IntervalUnitTainted;
    }
    function validate(): Result<IntervalUnit, Array<{ field: string; message: string }>> {
        return intervalUnitDeserialize(data);
    }
    function reset(overrides?: Partial<IntervalUnit>): void {
        data = overrides
            ? (overrides as typeof data)
            : intervalUnitGetDefaultForVariant(currentVariant);
        errors = {} as IntervalUnitErrors;
        tainted = {} as IntervalUnitTainted;
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
export function intervalUnitFromFormData(
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
    return intervalUnitDeserialize(obj);
}

export const IntervalUnit = {
    defaultValue: intervalUnitDefaultValue,
    serialize: intervalUnitSerialize,
    serializeWithContext: intervalUnitSerializeWithContext,
    deserialize: intervalUnitDeserialize,
    deserializeWithContext: intervalUnitDeserializeWithContext,
    is: intervalUnitIs,
    createForm: intervalUnitCreateForm,
    fromFormData: intervalUnitFromFormData
} as const;
