import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';

export type OrderStage = /** @default */ 'Estimate' | 'Active' | 'Invoice';

export function defaultValueOrderStage(): OrderStage {
    return 'Estimate';
}

export function toStringifiedJSONOrderStage(value: OrderStage): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeOrderStage(value, ctx));
}
export function toObjectOrderStage(value: OrderStage): unknown {
    const ctx = SerializeContext.create();
    return __serializeOrderStage(value, ctx);
}
export function __serializeOrderStage(value: OrderStage, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONOrderStage(
    json: string,
    opts?: DeserializeOptions
): Result<OrderStage, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectOrderStage(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectOrderStage(
    obj: unknown,
    opts?: DeserializeOptions
): Result<OrderStage, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeOrderStage(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'OrderStage.fromObject: root cannot be a forward reference'
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
export function __deserializeOrderStage(
    value: any,
    ctx: DeserializeContext
): OrderStage | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as OrderStage | PendingRef;
    }
    const allowedValues = ['Estimate', 'Active', 'Invoice'] as const;
    if (!allowedValues.includes(value)) {
        throw new DeserializeError([
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
export function isOrderStage(value: unknown): value is OrderStage {
    const allowedValues = ['Estimate', 'Active', 'Invoice'] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type EstimateErrorsOrderStage = {
    _errors: Option<Array<string>>;
};
export type ActiveErrorsOrderStage = { _errors: Option<Array<string>> };
export type InvoiceErrorsOrderStage = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type EstimateTaintedOrderStage = {};
export type ActiveTaintedOrderStage = {};
export type InvoiceTaintedOrderStage = {}; /** Union error type */
export type ErrorsOrderStage =
    | ({ _value: 'Estimate' } & EstimateErrorsOrderStage)
    | ({ _value: 'Active' } & ActiveErrorsOrderStage)
    | ({ _value: 'Invoice' } & InvoiceErrorsOrderStage); /** Union tainted type */
export type TaintedOrderStage =
    | ({ _value: 'Estimate' } & EstimateTaintedOrderStage)
    | ({ _value: 'Active' } & ActiveTaintedOrderStage)
    | ({ _value: 'Invoice' } & InvoiceTaintedOrderStage); /** Per-variant field controller types */
export interface EstimateFieldControllersOrderStage {}
export interface ActiveFieldControllersOrderStage {}
export interface InvoiceFieldControllersOrderStage {} /** Union Gigaform interface with variant switching */
export interface GigaformOrderStage {
    readonly currentVariant: 'Estimate' | 'Active' | 'Invoice';
    readonly data: OrderStage;
    readonly errors: ErrorsOrderStage;
    readonly tainted: TaintedOrderStage;
    readonly variants: VariantFieldsOrderStage;
    switchVariant(variant: 'Estimate' | 'Active' | 'Invoice'): void;
    validate(): Result<OrderStage, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<OrderStage>): void;
} /** Variant fields container */
export interface VariantFieldsOrderStage {
    readonly Estimate: { readonly fields: EstimateFieldControllersOrderStage };
    readonly Active: { readonly fields: ActiveFieldControllersOrderStage };
    readonly Invoice: { readonly fields: InvoiceFieldControllersOrderStage };
} /** Gets default value for a specific variant */
function getDefaultForVariantOrderStage(variant: string): OrderStage {
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
export function createFormOrderStage(initial?: OrderStage): GigaformOrderStage {
    const initialVariant: 'Estimate' | 'Active' | 'Invoice' =
        (initial as 'Estimate' | 'Active' | 'Invoice') ?? 'Estimate';
    let currentVariant = $state<'Estimate' | 'Active' | 'Invoice'>(initialVariant);
    let data = $state<OrderStage>(initial ?? getDefaultForVariantOrderStage(initialVariant));
    let errors = $state<ErrorsOrderStage>({} as ErrorsOrderStage);
    let tainted = $state<TaintedOrderStage>({} as TaintedOrderStage);
    const variants: VariantFieldsOrderStage = {
        Estimate: {
            fields: {} as EstimateFieldControllersOrderStage
        },
        Active: {
            fields: {} as ActiveFieldControllersOrderStage
        },
        Invoice: {
            fields: {} as InvoiceFieldControllersOrderStage
        }
    };
    function switchVariant(variant: 'Estimate' | 'Active' | 'Invoice'): void {
        currentVariant = variant;
        data = getDefaultForVariantOrderStage(variant);
        errors = {} as ErrorsOrderStage;
        tainted = {} as TaintedOrderStage;
    }
    function validate(): Result<OrderStage, Array<{ field: string; message: string }>> {
        return fromObjectOrderStage(data);
    }
    function reset(overrides?: Partial<OrderStage>): void {
        data = overrides
            ? (overrides as typeof data)
            : getDefaultForVariantOrderStage(currentVariant);
        errors = {} as ErrorsOrderStage;
        tainted = {} as TaintedOrderStage;
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
export function fromFormDataOrderStage(
    formData: FormData
): Result<OrderStage, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_value') as 'Estimate' | 'Active' | 'Invoice' | null;
    if (!discriminant) {
        return Result.err([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'Estimate') {
    } else if (discriminant === 'Active') {
    } else if (discriminant === 'Invoice') {
    }
    return fromStringifiedJSONOrderStage(JSON.stringify(obj));
}
