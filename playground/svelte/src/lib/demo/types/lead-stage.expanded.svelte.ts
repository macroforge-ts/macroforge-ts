import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';

export type LeadStage =
    | /** @default */ 'Open'
    | 'InitialContact'
    | 'Qualified'
    | 'Estimate'
    | 'Negotiation';

export function defaultValueLeadStage(): LeadStage {
    return 'Open';
}

export function toStringifiedJSONLeadStage(value: LeadStage): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeLeadStage(value, ctx));
}
export function toObjectLeadStage(value: LeadStage): unknown {
    const ctx = SerializeContext.create();
    return __serializeLeadStage(value, ctx);
}
export function __serializeLeadStage(value: LeadStage, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONLeadStage(
    json: string,
    opts?: DeserializeOptions
): Result<LeadStage, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectLeadStage(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectLeadStage(
    obj: unknown,
    opts?: DeserializeOptions
): Result<LeadStage, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeLeadStage(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'LeadStage.fromObject: root cannot be a forward reference'
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
export function __deserializeLeadStage(
    value: any,
    ctx: DeserializeContext
): LeadStage | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as LeadStage | PendingRef;
    }
    const allowedValues = [
        'Open',
        'InitialContact',
        'Qualified',
        'Estimate',
        'Negotiation'
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new DeserializeError([
            {
                field: '_root',
                message:
                    'Invalid value for LeadStage: expected one of ' +
                    allowedValues.map((v) => JSON.stringify(v)).join(', ') +
                    ', got ' +
                    JSON.stringify(value)
            }
        ]);
    }
    return value as LeadStage;
}
export function isLeadStage(value: unknown): value is LeadStage {
    const allowedValues = [
        'Open',
        'InitialContact',
        'Qualified',
        'Estimate',
        'Negotiation'
    ] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type OpenErrorsLeadStage = { _errors: Option<Array<string>> };
export type InitialContactErrorsLeadStage = { _errors: Option<Array<string>> };
export type QualifiedErrorsLeadStage = { _errors: Option<Array<string>> };
export type EstimateErrorsLeadStage = { _errors: Option<Array<string>> };
export type NegotiationErrorsLeadStage = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type OpenTaintedLeadStage = {};
export type InitialContactTaintedLeadStage = {};
export type QualifiedTaintedLeadStage = {};
export type EstimateTaintedLeadStage = {};
export type NegotiationTaintedLeadStage = {}; /** Union error type */
export type ErrorsLeadStage =
    | ({ _value: 'Open' } & OpenErrorsLeadStage)
    | ({ _value: 'InitialContact' } & InitialContactErrorsLeadStage)
    | ({ _value: 'Qualified' } & QualifiedErrorsLeadStage)
    | ({ _value: 'Estimate' } & EstimateErrorsLeadStage)
    | ({ _value: 'Negotiation' } & NegotiationErrorsLeadStage); /** Union tainted type */
export type TaintedLeadStage =
    | ({ _value: 'Open' } & OpenTaintedLeadStage)
    | ({ _value: 'InitialContact' } & InitialContactTaintedLeadStage)
    | ({ _value: 'Qualified' } & QualifiedTaintedLeadStage)
    | ({ _value: 'Estimate' } & EstimateTaintedLeadStage)
    | ({
          _value: 'Negotiation';
      } & NegotiationTaintedLeadStage); /** Per-variant field controller types */
export interface OpenFieldControllersLeadStage {}
export interface InitialContactFieldControllersLeadStage {}
export interface QualifiedFieldControllersLeadStage {}
export interface EstimateFieldControllersLeadStage {}
export interface NegotiationFieldControllersLeadStage {} /** Union Gigaform interface with variant switching */
export interface GigaformLeadStage {
    readonly currentVariant: 'Open' | 'InitialContact' | 'Qualified' | 'Estimate' | 'Negotiation';
    readonly data: LeadStage;
    readonly errors: ErrorsLeadStage;
    readonly tainted: TaintedLeadStage;
    readonly variants: VariantFieldsLeadStage;
    switchVariant(
        variant: 'Open' | 'InitialContact' | 'Qualified' | 'Estimate' | 'Negotiation'
    ): void;
    validate(): Result<LeadStage, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<LeadStage>): void;
} /** Variant fields container */
export interface VariantFieldsLeadStage {
    readonly Open: { readonly fields: OpenFieldControllersLeadStage };
    readonly InitialContact: { readonly fields: InitialContactFieldControllersLeadStage };
    readonly Qualified: { readonly fields: QualifiedFieldControllersLeadStage };
    readonly Estimate: { readonly fields: EstimateFieldControllersLeadStage };
    readonly Negotiation: { readonly fields: NegotiationFieldControllersLeadStage };
} /** Gets default value for a specific variant */
function getDefaultForVariantLeadStage(variant: string): LeadStage {
    switch (variant) {
        case 'Open':
            return 'Open' as LeadStage;
        case 'InitialContact':
            return 'InitialContact' as LeadStage;
        case 'Qualified':
            return 'Qualified' as LeadStage;
        case 'Estimate':
            return 'Estimate' as LeadStage;
        case 'Negotiation':
            return 'Negotiation' as LeadStage;
        default:
            return 'Open' as LeadStage;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function createFormLeadStage(initial?: LeadStage): GigaformLeadStage {
    const initialVariant: 'Open' | 'InitialContact' | 'Qualified' | 'Estimate' | 'Negotiation' =
        (initial as 'Open' | 'InitialContact' | 'Qualified' | 'Estimate' | 'Negotiation') ?? 'Open';
    let currentVariant = $state<
        'Open' | 'InitialContact' | 'Qualified' | 'Estimate' | 'Negotiation'
    >(initialVariant);
    let data = $state<LeadStage>(initial ?? getDefaultForVariantLeadStage(initialVariant));
    let errors = $state<ErrorsLeadStage>({} as ErrorsLeadStage);
    let tainted = $state<TaintedLeadStage>({} as TaintedLeadStage);
    const variants: VariantFieldsLeadStage = {
        Open: {
            fields: {} as OpenFieldControllersLeadStage
        },
        InitialContact: {
            fields: {} as InitialContactFieldControllersLeadStage
        },
        Qualified: {
            fields: {} as QualifiedFieldControllersLeadStage
        },
        Estimate: {
            fields: {} as EstimateFieldControllersLeadStage
        },
        Negotiation: {
            fields: {} as NegotiationFieldControllersLeadStage
        }
    };
    function switchVariant(
        variant: 'Open' | 'InitialContact' | 'Qualified' | 'Estimate' | 'Negotiation'
    ): void {
        currentVariant = variant;
        data = getDefaultForVariantLeadStage(variant);
        errors = {} as ErrorsLeadStage;
        tainted = {} as TaintedLeadStage;
    }
    function validate(): Result<LeadStage, Array<{ field: string; message: string }>> {
        return fromObjectLeadStage(data);
    }
    function reset(overrides?: Partial<LeadStage>): void {
        data = overrides
            ? (overrides as typeof data)
            : getDefaultForVariantLeadStage(currentVariant);
        errors = {} as ErrorsLeadStage;
        tainted = {} as TaintedLeadStage;
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
export function fromFormDataLeadStage(
    formData: FormData
): Result<LeadStage, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_value') as
        | 'Open'
        | 'InitialContact'
        | 'Qualified'
        | 'Estimate'
        | 'Negotiation'
        | null;
    if (!discriminant) {
        return Result.err([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'Open') {
    } else if (discriminant === 'InitialContact') {
    } else if (discriminant === 'Qualified') {
    } else if (discriminant === 'Estimate') {
    } else if (discriminant === 'Negotiation') {
    }
    return fromStringifiedJSONLeadStage(JSON.stringify(obj));
}
