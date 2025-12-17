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

export function weekdayDefaultValue(): Weekday {
    return 'Monday';
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function weekdaySerialize(
    value: Weekday
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(weekdaySerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function weekdaySerializeWithContext(value: Weekday, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === 'function') {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

/** Deserializes input to this type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function weekdayDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Result<Weekday, Array<{ field: string; message: string }>> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = weekdayDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Weekday.deserialize: root cannot be a forward reference'
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
export function weekdayDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): Weekday | PendingRef {
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
export function weekdayIs(value: unknown): value is Weekday {
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

/** Per-variant error types */ export type WeekdayMondayErrors = { _errors: Option<Array<string>> };
export type WeekdayTuesdayErrors = { _errors: Option<Array<string>> };
export type WeekdayWednesdayErrors = { _errors: Option<Array<string>> };
export type WeekdayThursdayErrors = { _errors: Option<Array<string>> };
export type WeekdayFridayErrors = { _errors: Option<Array<string>> };
export type WeekdaySaturdayErrors = { _errors: Option<Array<string>> };
export type WeekdaySundayErrors = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type WeekdayMondayTainted = {};
export type WeekdayTuesdayTainted = {};
export type WeekdayWednesdayTainted = {};
export type WeekdayThursdayTainted = {};
export type WeekdayFridayTainted = {};
export type WeekdaySaturdayTainted = {};
export type WeekdaySundayTainted = {}; /** Union error type */
export type WeekdayErrors =
    | ({ _value: 'Monday' } & WeekdayMondayErrors)
    | ({ _value: 'Tuesday' } & WeekdayTuesdayErrors)
    | ({ _value: 'Wednesday' } & WeekdayWednesdayErrors)
    | ({ _value: 'Thursday' } & WeekdayThursdayErrors)
    | ({ _value: 'Friday' } & WeekdayFridayErrors)
    | ({ _value: 'Saturday' } & WeekdaySaturdayErrors)
    | ({ _value: 'Sunday' } & WeekdaySundayErrors); /** Union tainted type */
export type WeekdayTainted =
    | ({ _value: 'Monday' } & WeekdayMondayTainted)
    | ({ _value: 'Tuesday' } & WeekdayTuesdayTainted)
    | ({ _value: 'Wednesday' } & WeekdayWednesdayTainted)
    | ({ _value: 'Thursday' } & WeekdayThursdayTainted)
    | ({ _value: 'Friday' } & WeekdayFridayTainted)
    | ({ _value: 'Saturday' } & WeekdaySaturdayTainted)
    | ({ _value: 'Sunday' } & WeekdaySundayTainted); /** Per-variant field controller types */
export interface WeekdayMondayFieldControllers {}
export interface WeekdayTuesdayFieldControllers {}
export interface WeekdayWednesdayFieldControllers {}
export interface WeekdayThursdayFieldControllers {}
export interface WeekdayFridayFieldControllers {}
export interface WeekdaySaturdayFieldControllers {}
export interface WeekdaySundayFieldControllers {} /** Union Gigaform interface with variant switching */
export interface WeekdayGigaform {
    readonly currentVariant:
        | 'Monday'
        | 'Tuesday'
        | 'Wednesday'
        | 'Thursday'
        | 'Friday'
        | 'Saturday'
        | 'Sunday';
    readonly data: Weekday;
    readonly errors: WeekdayErrors;
    readonly tainted: WeekdayTainted;
    readonly variants: WeekdayVariantFields;
    switchVariant(
        variant: 'Monday' | 'Tuesday' | 'Wednesday' | 'Thursday' | 'Friday' | 'Saturday' | 'Sunday'
    ): void;
    validate(): Result<Weekday, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Weekday>): void;
} /** Variant fields container */
export interface WeekdayVariantFields {
    readonly Monday: { readonly fields: WeekdayMondayFieldControllers };
    readonly Tuesday: { readonly fields: WeekdayTuesdayFieldControllers };
    readonly Wednesday: { readonly fields: WeekdayWednesdayFieldControllers };
    readonly Thursday: { readonly fields: WeekdayThursdayFieldControllers };
    readonly Friday: { readonly fields: WeekdayFridayFieldControllers };
    readonly Saturday: { readonly fields: WeekdaySaturdayFieldControllers };
    readonly Sunday: { readonly fields: WeekdaySundayFieldControllers };
} /** Gets default value for a specific variant */
function weekdayGetDefaultForVariant(variant: string): Weekday {
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
export function weekdayCreateForm(initial?: Weekday): WeekdayGigaform {
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
    let data = $state<Weekday>(initial ?? weekdayGetDefaultForVariant(initialVariant));
    let errors = $state<WeekdayErrors>({} as WeekdayErrors);
    let tainted = $state<WeekdayTainted>({} as WeekdayTainted);
    const variants: WeekdayVariantFields = {
        Monday: {
            fields: {} as WeekdayMondayFieldControllers
        },
        Tuesday: {
            fields: {} as WeekdayTuesdayFieldControllers
        },
        Wednesday: {
            fields: {} as WeekdayWednesdayFieldControllers
        },
        Thursday: {
            fields: {} as WeekdayThursdayFieldControllers
        },
        Friday: {
            fields: {} as WeekdayFridayFieldControllers
        },
        Saturday: {
            fields: {} as WeekdaySaturdayFieldControllers
        },
        Sunday: {
            fields: {} as WeekdaySundayFieldControllers
        }
    };
    function switchVariant(
        variant: 'Monday' | 'Tuesday' | 'Wednesday' | 'Thursday' | 'Friday' | 'Saturday' | 'Sunday'
    ): void {
        currentVariant = variant;
        data = weekdayGetDefaultForVariant(variant);
        errors = {} as WeekdayErrors;
        tainted = {} as WeekdayTainted;
    }
    function validate(): Result<Weekday, Array<{ field: string; message: string }>> {
        return weekdayFromObject(data);
    }
    function reset(overrides?: Partial<Weekday>): void {
        data = overrides ? (overrides as typeof data) : weekdayGetDefaultForVariant(currentVariant);
        errors = {} as WeekdayErrors;
        tainted = {} as WeekdayTainted;
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
export function weekdayFromFormData(
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
    return weekdayFromStringifiedJSON(JSON.stringify(obj));
}

export const Weekday = {
    defaultValue: weekdayDefaultValue,
    serialize: weekdaySerialize,
    serializeWithContext: weekdaySerializeWithContext,
    deserialize: weekdayDeserialize,
    deserializeWithContext: weekdayDeserializeWithContext,
    is: weekdayIs,
    createForm: weekdayCreateForm,
    fromFormData: weekdayFromFormData
} as const;
