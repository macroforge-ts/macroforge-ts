import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import type { Exit } from '@playground/macro/gigaform';
import { exitFail } from '@playground/macro/gigaform';
import type { Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';

export type OrderStage = /** @default */ 'Estimate' | 'Active' | 'Invoice';

export function orderStageDefaultValue(): OrderStage {
    return 'Estimate';
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function orderStageSerialize(
    value: OrderStage
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(orderStageSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function orderStageSerializeWithContext(
    value: OrderStage,
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
@returns Result containing the deserialized value or validation errors */ export function orderStageDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, OrderStage> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = orderStageDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'OrderStage.deserialize: root cannot be a forward reference'
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
export function orderStageDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): OrderStage | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as OrderStage | __mf_PendingRef;
    }
    const allowedValues = ['Estimate', 'Active', 'Invoice'] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message:
                    'Invalid value for OrderStage: expected one of ' +
                    allowedValues.map((v) => JSON.stringify(v)).join(', ') +
                    ', got ' +
                    JSON.stringify(value)
            }
        ]);
    }
    return value as OrderStage;
}
export function orderStageIs(value: unknown): value is OrderStage {
    const allowedValues = ['Estimate', 'Active', 'Invoice'] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type OrderStageEstimateErrors = {
    _errors: Option<Array<string>>;
};
export type OrderStageActiveErrors = { _errors: Option<Array<string>> };
export type OrderStageInvoiceErrors = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type OrderStageEstimateTainted = {};
export type OrderStageActiveTainted = {};
export type OrderStageInvoiceTainted = {}; /** Union error type */
export type OrderStageErrors =
    | ({ _value: 'Estimate' } & OrderStageEstimateErrors)
    | ({ _value: 'Active' } & OrderStageActiveErrors)
    | ({ _value: 'Invoice' } & OrderStageInvoiceErrors); /** Union tainted type */
export type OrderStageTainted =
    | ({ _value: 'Estimate' } & OrderStageEstimateTainted)
    | ({ _value: 'Active' } & OrderStageActiveTainted)
    | ({ _value: 'Invoice' } & OrderStageInvoiceTainted); /** Per-variant field controller types */
export interface OrderStageEstimateFieldControllers {}
export interface OrderStageActiveFieldControllers {}
export interface OrderStageInvoiceFieldControllers {} /** Union Gigaform interface with variant switching */
export interface OrderStageGigaform {
    readonly currentVariant: 'Estimate' | 'Active' | 'Invoice';
    readonly data: OrderStage;
    readonly errors: OrderStageErrors;
    readonly tainted: OrderStageTainted;
    readonly variants: OrderStageVariantFields;
    switchVariant(variant: 'Estimate' | 'Active' | 'Invoice'): void;
    validate(): Exit<Array<{ field: string; message: string }>, OrderStage>;
    reset(overrides?: Partial<OrderStage>): void;
} /** Variant fields container */
export interface OrderStageVariantFields {
    readonly Estimate: { readonly fields: OrderStageEstimateFieldControllers };
    readonly Active: { readonly fields: OrderStageActiveFieldControllers };
    readonly Invoice: { readonly fields: OrderStageInvoiceFieldControllers };
} /** Gets default value for a specific variant */
function orderStageGetDefaultForVariant(variant: string): OrderStage {
    switch (variant) {
        case 'Estimate':
            return 'Estimate' as OrderStage;
        case 'Active':
            return 'Active' as OrderStage;
        case 'Invoice':
            return 'Invoice' as OrderStage;
        default:
            return 'Estimate' as OrderStage;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function orderStageCreateForm(initial?: OrderStage): OrderStageGigaform {
    const initialVariant: 'Estimate' | 'Active' | 'Invoice' =
        (initial as 'Estimate' | 'Active' | 'Invoice') ?? 'Estimate';
    let currentVariant = $state<'Estimate' | 'Active' | 'Invoice'>(initialVariant);
    let data = $state<OrderStage>(initial ?? orderStageGetDefaultForVariant(initialVariant));
    let errors = $state<OrderStageErrors>({} as OrderStageErrors);
    let tainted = $state<OrderStageTainted>({} as OrderStageTainted);
    const variants: OrderStageVariantFields = {
        Estimate: { fields: {} as OrderStageEstimateFieldControllers },
        Active: { fields: {} as OrderStageActiveFieldControllers },
        Invoice: { fields: {} as OrderStageInvoiceFieldControllers }
    };
    function switchVariant(variant: 'Estimate' | 'Active' | 'Invoice'): void {
        currentVariant = variant;
        data = orderStageGetDefaultForVariant(variant);
        errors = {} as OrderStageErrors;
        tainted = {} as OrderStageTainted;
    }
    function validate(): Exit<Array<{ field: string; message: string }>, OrderStage> {
        return orderStageDeserialize(data);
    }
    function reset(overrides?: Partial<OrderStage>): void {
        data = overrides
            ? (overrides as typeof data)
            : orderStageGetDefaultForVariant(currentVariant);
        errors = {} as OrderStageErrors;
        tainted = {} as OrderStageTainted;
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
export function orderStageFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, OrderStage> {
    const discriminant = formData.get('_value') as 'Estimate' | 'Active' | 'Invoice' | null;
    if (!discriminant) {
        return exitFail([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'Estimate') {
    } else if (discriminant === 'Active') {
    } else if (discriminant === 'Invoice') {
    }
    return orderStageDeserialize(obj);
}

export const OrderStage = {
    defaultValue: orderStageDefaultValue,
    serialize: orderStageSerialize,
    serializeWithContext: orderStageSerializeWithContext,
    deserialize: orderStageDeserialize,
    deserializeWithContext: orderStageDeserializeWithContext,
    is: orderStageIs,
    createForm: orderStageCreateForm,
    fromFormData: orderStageFromFormData
} as const;
