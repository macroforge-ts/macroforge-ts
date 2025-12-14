import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';

export type NextStep = /** @default */ 'InitialContact' | 'Qualified' | 'Estimate' | 'Negotiation';

export function defaultValueNextStep(): NextStep {
    return 'InitialContact';
}

export function toStringifiedJSONNextStep(value: NextStep): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeNextStep(value, ctx));
}
export function toObjectNextStep(value: NextStep): unknown {
    const ctx = SerializeContext.create();
    return __serializeNextStep(value, ctx);
}
export function __serializeNextStep(value: NextStep, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONNextStep(
    json: string,
    opts?: DeserializeOptions
): Result<NextStep, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectNextStep(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectNextStep(
    obj: unknown,
    opts?: DeserializeOptions
): Result<NextStep, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeNextStep(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'NextStep.fromObject: root cannot be a forward reference'
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
export function __deserializeNextStep(value: any, ctx: DeserializeContext): NextStep | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as NextStep | PendingRef;
    }
    const allowedValues = ['InitialContact', 'Qualified', 'Estimate', 'Negotiation'] as const;
    if (!allowedValues.includes(value)) {
        throw new DeserializeError([
            {
                field: '_root',
                message:
                    'Invalid value for NextStep: expected one of ' +
                    allowedValues.map((v) => JSON.stringify(v)).join(', ') +
                    ', got ' +
                    JSON.stringify(value)
            }
        ]);
    }
    return value as NextStep;
}
export function isNextStep(value: unknown): value is NextStep {
    const allowedValues = ['InitialContact', 'Qualified', 'Estimate', 'Negotiation'] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type InitialContactErrorsNextStep = {
    _errors: Option<Array<string>>;
};
export type QualifiedErrorsNextStep = { _errors: Option<Array<string>> };
export type EstimateErrorsNextStep = { _errors: Option<Array<string>> };
export type NegotiationErrorsNextStep = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type InitialContactTaintedNextStep = {};
export type QualifiedTaintedNextStep = {};
export type EstimateTaintedNextStep = {};
export type NegotiationTaintedNextStep = {}; /** Union error type */
export type ErrorsNextStep =
    | ({ _value: 'InitialContact' } & InitialContactErrorsNextStep)
    | ({ _value: 'Qualified' } & QualifiedErrorsNextStep)
    | ({ _value: 'Estimate' } & EstimateErrorsNextStep)
    | ({ _value: 'Negotiation' } & NegotiationErrorsNextStep); /** Union tainted type */
export type TaintedNextStep =
    | ({ _value: 'InitialContact' } & InitialContactTaintedNextStep)
    | ({ _value: 'Qualified' } & QualifiedTaintedNextStep)
    | ({ _value: 'Estimate' } & EstimateTaintedNextStep)
    | ({
          _value: 'Negotiation';
      } & NegotiationTaintedNextStep); /** Per-variant field controller types */
export interface InitialContactFieldControllersNextStep {}
export interface QualifiedFieldControllersNextStep {}
export interface EstimateFieldControllersNextStep {}
export interface NegotiationFieldControllersNextStep {} /** Union Gigaform interface with variant switching */
export interface GigaformNextStep {
    readonly currentVariant: 'InitialContact' | 'Qualified' | 'Estimate' | 'Negotiation';
    readonly data: NextStep;
    readonly errors: ErrorsNextStep;
    readonly tainted: TaintedNextStep;
    readonly variants: VariantFieldsNextStep;
    switchVariant(variant: 'InitialContact' | 'Qualified' | 'Estimate' | 'Negotiation'): void;
    validate(): Result<NextStep, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<NextStep>): void;
} /** Variant fields container */
export interface VariantFieldsNextStep {
    readonly InitialContact: { readonly fields: InitialContactFieldControllersNextStep };
    readonly Qualified: { readonly fields: QualifiedFieldControllersNextStep };
    readonly Estimate: { readonly fields: EstimateFieldControllersNextStep };
    readonly Negotiation: { readonly fields: NegotiationFieldControllersNextStep };
} /** Gets default value for a specific variant */
function getDefaultForVariantNextStep(variant: string): NextStep {
    switch (variant) {
        case 'InitialContact':
            return 'InitialContact' as NextStep;
        case 'Qualified':
            return 'Qualified' as NextStep;
        case 'Estimate':
            return 'Estimate' as NextStep;
        case 'Negotiation':
            return 'Negotiation' as NextStep;
        default:
            return 'InitialContact' as NextStep;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function createFormNextStep(initial?: NextStep): GigaformNextStep {
    const initialVariant: 'InitialContact' | 'Qualified' | 'Estimate' | 'Negotiation' =
        (initial as 'InitialContact' | 'Qualified' | 'Estimate' | 'Negotiation') ??
        'InitialContact';
    let currentVariant = $state<'InitialContact' | 'Qualified' | 'Estimate' | 'Negotiation'>(
        initialVariant
    );
    let data = $state<NextStep>(initial ?? getDefaultForVariantNextStep(initialVariant));
    let errors = $state<ErrorsNextStep>({} as ErrorsNextStep);
    let tainted = $state<TaintedNextStep>({} as TaintedNextStep);
    const variants: VariantFieldsNextStep = {
        InitialContact: {
            fields: {} as InitialContactFieldControllersNextStep
        },
        Qualified: {
            fields: {} as QualifiedFieldControllersNextStep
        },
        Estimate: {
            fields: {} as EstimateFieldControllersNextStep
        },
        Negotiation: {
            fields: {} as NegotiationFieldControllersNextStep
        }
    };
    function switchVariant(
        variant: 'InitialContact' | 'Qualified' | 'Estimate' | 'Negotiation'
    ): void {
        currentVariant = variant;
        data = getDefaultForVariantNextStep(variant);
        errors = {} as ErrorsNextStep;
        tainted = {} as TaintedNextStep;
    }
    function validate(): Result<NextStep, Array<{ field: string; message: string }>> {
        return NextStep.fromObject(data);
    }
    function reset(overrides?: Partial<NextStep>): void {
        data = overrides
            ? (overrides as typeof data)
            : getDefaultForVariantNextStep(currentVariant);
        errors = {} as ErrorsNextStep;
        tainted = {} as TaintedNextStep;
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
export function fromFormDataNextStep(
    formData: FormData
): Result<NextStep, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_value') as
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
    if (discriminant === 'InitialContact') {
    } else if (discriminant === 'Qualified') {
    } else if (discriminant === 'Estimate') {
    } else if (discriminant === 'Negotiation') {
    }
    return NextStep.fromStringifiedJSON(JSON.stringify(obj));
}
