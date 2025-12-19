import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import type { Exit } from '@playground/macro/gigaform';
import { toExit } from '@playground/macro/gigaform';
import type { Option as __gf_Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';

export type WeekOfMonth = /** @default */ 'First' | 'Second' | 'Third' | 'Fourth' | 'Last';

export function weekOfMonthDefaultValue(): WeekOfMonth {
    return 'First';
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function weekOfMonthSerialize(
    value: WeekOfMonth
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(weekOfMonthSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function weekOfMonthSerializeWithContext(
    value: WeekOfMonth,
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
@returns Result containing the deserialized value or validation errors */ export function weekOfMonthDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: WeekOfMonth }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = weekOfMonthDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: '_root',
                        message: 'WeekOfMonth.deserialize: root cannot be a forward reference'
                    }
                ]
            };
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return { success: true, value: resultOrRef };
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return { success: false, errors: e.errors };
        }
        const message = e instanceof Error ? e.message : String(e);
        return { success: false, errors: [{ field: '_root', message }] };
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function weekOfMonthDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): WeekOfMonth | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as WeekOfMonth | __mf_PendingRef;
    }
    const allowedValues = ['First', 'Second', 'Third', 'Fourth', 'Last'] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
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
export function weekOfMonthIs(value: unknown): value is WeekOfMonth {
    const allowedValues = ['First', 'Second', 'Third', 'Fourth', 'Last'] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type WeekOfMonthFirstErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type WeekOfMonthSecondErrors = { _errors: __gf_Option<Array<string>> };
export type WeekOfMonthThirdErrors = { _errors: __gf_Option<Array<string>> };
export type WeekOfMonthFourthErrors = { _errors: __gf_Option<Array<string>> };
export type WeekOfMonthLastErrors = {
    _errors: __gf_Option<Array<string>>;
}; /** Per-variant tainted types */
export type WeekOfMonthFirstTainted = {};
export type WeekOfMonthSecondTainted = {};
export type WeekOfMonthThirdTainted = {};
export type WeekOfMonthFourthTainted = {};
export type WeekOfMonthLastTainted = {}; /** Union error type */
export type WeekOfMonthErrors =
    | ({ _value: 'First' } & WeekOfMonthFirstErrors)
    | ({ _value: 'Second' } & WeekOfMonthSecondErrors)
    | ({ _value: 'Third' } & WeekOfMonthThirdErrors)
    | ({ _value: 'Fourth' } & WeekOfMonthFourthErrors)
    | ({ _value: 'Last' } & WeekOfMonthLastErrors); /** Union tainted type */
export type WeekOfMonthTainted =
    | ({ _value: 'First' } & WeekOfMonthFirstTainted)
    | ({ _value: 'Second' } & WeekOfMonthSecondTainted)
    | ({ _value: 'Third' } & WeekOfMonthThirdTainted)
    | ({ _value: 'Fourth' } & WeekOfMonthFourthTainted)
    | ({ _value: 'Last' } & WeekOfMonthLastTainted); /** Per-variant field controller types */
export interface WeekOfMonthFirstFieldControllers {}
export interface WeekOfMonthSecondFieldControllers {}
export interface WeekOfMonthThirdFieldControllers {}
export interface WeekOfMonthFourthFieldControllers {}
export interface WeekOfMonthLastFieldControllers {} /** Union Gigaform interface with variant switching */
export interface WeekOfMonthGigaform {
    readonly currentVariant: 'First' | 'Second' | 'Third' | 'Fourth' | 'Last';
    readonly data: WeekOfMonth;
    readonly errors: WeekOfMonthErrors;
    readonly tainted: WeekOfMonthTainted;
    readonly variants: WeekOfMonthVariantFields;
    switchVariant(variant: 'First' | 'Second' | 'Third' | 'Fourth' | 'Last'): void;
    validate(): Exit<WeekOfMonth, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<WeekOfMonth>): void;
} /** Variant fields container */
export interface WeekOfMonthVariantFields {
    readonly First: { readonly fields: WeekOfMonthFirstFieldControllers };
    readonly Second: { readonly fields: WeekOfMonthSecondFieldControllers };
    readonly Third: { readonly fields: WeekOfMonthThirdFieldControllers };
    readonly Fourth: { readonly fields: WeekOfMonthFourthFieldControllers };
    readonly Last: { readonly fields: WeekOfMonthLastFieldControllers };
} /** Gets default value for a specific variant */
function weekOfMonthGetDefaultForVariant(variant: string): WeekOfMonth {
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
export function weekOfMonthCreateForm(initial?: WeekOfMonth): WeekOfMonthGigaform {
    const initialVariant: 'First' | 'Second' | 'Third' | 'Fourth' | 'Last' =
        (initial as 'First' | 'Second' | 'Third' | 'Fourth' | 'Last') ?? 'First';
    let currentVariant = $state<'First' | 'Second' | 'Third' | 'Fourth' | 'Last'>(initialVariant);
    let data = $state<WeekOfMonth>(initial ?? weekOfMonthGetDefaultForVariant(initialVariant));
    let errors = $state<WeekOfMonthErrors>({} as WeekOfMonthErrors);
    let tainted = $state<WeekOfMonthTainted>({} as WeekOfMonthTainted);
    const variants: WeekOfMonthVariantFields = {
        First: { fields: {} as WeekOfMonthFirstFieldControllers },
        Second: { fields: {} as WeekOfMonthSecondFieldControllers },
        Third: { fields: {} as WeekOfMonthThirdFieldControllers },
        Fourth: { fields: {} as WeekOfMonthFourthFieldControllers },
        Last: { fields: {} as WeekOfMonthLastFieldControllers }
    };
    function switchVariant(variant: 'First' | 'Second' | 'Third' | 'Fourth' | 'Last'): void {
        currentVariant = variant;
        data = weekOfMonthGetDefaultForVariant(variant);
        errors = {} as WeekOfMonthErrors;
        tainted = {} as WeekOfMonthTainted;
    }
    function validate(): Exit<WeekOfMonth, Array<{ field: string; message: string }>> {
        return toExit(weekOfMonthDeserialize(data));
    }
    function reset(overrides?: Partial<WeekOfMonth>): void {
        data = overrides
            ? (overrides as typeof data)
            : weekOfMonthGetDefaultForVariant(currentVariant);
        errors = {} as WeekOfMonthErrors;
        tainted = {} as WeekOfMonthTainted;
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
export function weekOfMonthFromFormData(
    formData: FormData
): Exit<WeekOfMonth, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_value') as
        | 'First'
        | 'Second'
        | 'Third'
        | 'Fourth'
        | 'Last'
        | null;
    if (!discriminant) {
        return toExit({
            success: false,
            errors: [{ field: '_value', message: 'Missing discriminant field' }]
        });
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'First') {
    } else if (discriminant === 'Second') {
    } else if (discriminant === 'Third') {
    } else if (discriminant === 'Fourth') {
    } else if (discriminant === 'Last') {
    }
    return toExit(weekOfMonthDeserialize(obj));
}

export const WeekOfMonth = {
    defaultValue: weekOfMonthDefaultValue,
    serialize: weekOfMonthSerialize,
    serializeWithContext: weekOfMonthSerializeWithContext,
    deserialize: weekOfMonthDeserialize,
    deserializeWithContext: weekOfMonthDeserializeWithContext,
    is: weekOfMonthIs,
    createForm: weekOfMonthCreateForm,
    fromFormData: weekOfMonthFromFormData
} as const;
