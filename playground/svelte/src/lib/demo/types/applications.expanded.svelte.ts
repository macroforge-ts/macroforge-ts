import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';

export type Applications =
    | /** @default */ 'Sales'
    | 'Accounting'
    | 'Errand'
    | 'HumanResources'
    | 'Logistics'
    | 'Marketing'
    | 'Website';

export function defaultValueApplications(): Applications {
    return 'Sales';
}

export function toStringifiedJSONApplications(value: Applications): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeApplications(value, ctx));
}
export function toObjectApplications(value: Applications): unknown {
    const ctx = SerializeContext.create();
    return __serializeApplications(value, ctx);
}
export function __serializeApplications(value: Applications, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONApplications(
    json: string,
    opts?: DeserializeOptions
): Result<Applications, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectApplications(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectApplications(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Applications, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeApplications(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Applications.fromObject: root cannot be a forward reference'
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
export function __deserializeApplications(
    value: any,
    ctx: DeserializeContext
): Applications | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Applications | PendingRef;
    }
    const allowedValues = [
        'Sales',
        'Accounting',
        'Errand',
        'HumanResources',
        'Logistics',
        'Marketing',
        'Website'
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new DeserializeError([
            {
                field: '_root',
                message:
                    'Invalid value for Applications: expected one of ' +
                    allowedValues.map((v) => JSON.stringify(v)).join(', ') +
                    ', got ' +
                    JSON.stringify(value)
            }
        ]);
    }
    return value as Applications;
}
export function isApplications(value: unknown): value is Applications {
    const allowedValues = [
        'Sales',
        'Accounting',
        'Errand',
        'HumanResources',
        'Logistics',
        'Marketing',
        'Website'
    ] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type SalesErrorsApplications = {
    _errors: Option<Array<string>>;
};
export type AccountingErrorsApplications = { _errors: Option<Array<string>> };
export type ErrandErrorsApplications = { _errors: Option<Array<string>> };
export type HumanResourcesErrorsApplications = { _errors: Option<Array<string>> };
export type LogisticsErrorsApplications = { _errors: Option<Array<string>> };
export type MarketingErrorsApplications = { _errors: Option<Array<string>> };
export type WebsiteErrorsApplications = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type SalesTaintedApplications = {};
export type AccountingTaintedApplications = {};
export type ErrandTaintedApplications = {};
export type HumanResourcesTaintedApplications = {};
export type LogisticsTaintedApplications = {};
export type MarketingTaintedApplications = {};
export type WebsiteTaintedApplications = {}; /** Union error type */
export type ErrorsApplications =
    | ({ _value: 'Sales' } & SalesErrorsApplications)
    | ({ _value: 'Accounting' } & AccountingErrorsApplications)
    | ({ _value: 'Errand' } & ErrandErrorsApplications)
    | ({ _value: 'HumanResources' } & HumanResourcesErrorsApplications)
    | ({ _value: 'Logistics' } & LogisticsErrorsApplications)
    | ({ _value: 'Marketing' } & MarketingErrorsApplications)
    | ({ _value: 'Website' } & WebsiteErrorsApplications); /** Union tainted type */
export type TaintedApplications =
    | ({ _value: 'Sales' } & SalesTaintedApplications)
    | ({ _value: 'Accounting' } & AccountingTaintedApplications)
    | ({ _value: 'Errand' } & ErrandTaintedApplications)
    | ({ _value: 'HumanResources' } & HumanResourcesTaintedApplications)
    | ({ _value: 'Logistics' } & LogisticsTaintedApplications)
    | ({ _value: 'Marketing' } & MarketingTaintedApplications)
    | ({
          _value: 'Website';
      } & WebsiteTaintedApplications); /** Per-variant field controller types */
export interface SalesFieldControllersApplications {}
export interface AccountingFieldControllersApplications {}
export interface ErrandFieldControllersApplications {}
export interface HumanResourcesFieldControllersApplications {}
export interface LogisticsFieldControllersApplications {}
export interface MarketingFieldControllersApplications {}
export interface WebsiteFieldControllersApplications {} /** Union Gigaform interface with variant switching */
export interface GigaformApplications {
    readonly currentVariant:
        | 'Sales'
        | 'Accounting'
        | 'Errand'
        | 'HumanResources'
        | 'Logistics'
        | 'Marketing'
        | 'Website';
    readonly data: Applications;
    readonly errors: ErrorsApplications;
    readonly tainted: TaintedApplications;
    readonly variants: VariantFieldsApplications;
    switchVariant(
        variant:
            | 'Sales'
            | 'Accounting'
            | 'Errand'
            | 'HumanResources'
            | 'Logistics'
            | 'Marketing'
            | 'Website'
    ): void;
    validate(): Result<Applications, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Applications>): void;
} /** Variant fields container */
export interface VariantFieldsApplications {
    readonly Sales: { readonly fields: SalesFieldControllersApplications };
    readonly Accounting: { readonly fields: AccountingFieldControllersApplications };
    readonly Errand: { readonly fields: ErrandFieldControllersApplications };
    readonly HumanResources: { readonly fields: HumanResourcesFieldControllersApplications };
    readonly Logistics: { readonly fields: LogisticsFieldControllersApplications };
    readonly Marketing: { readonly fields: MarketingFieldControllersApplications };
    readonly Website: { readonly fields: WebsiteFieldControllersApplications };
} /** Gets default value for a specific variant */
function getDefaultForVariantApplications(variant: string): Applications {
    switch (variant) {
        case 'Sales':
            return 'Sales' as Applications;
        case 'Accounting':
            return 'Accounting' as Applications;
        case 'Errand':
            return 'Errand' as Applications;
        case 'HumanResources':
            return 'HumanResources' as Applications;
        case 'Logistics':
            return 'Logistics' as Applications;
        case 'Marketing':
            return 'Marketing' as Applications;
        case 'Website':
            return 'Website' as Applications;
        default:
            return 'Sales' as Applications;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function createFormApplications(initial?: Applications): GigaformApplications {
    const initialVariant:
        | 'Sales'
        | 'Accounting'
        | 'Errand'
        | 'HumanResources'
        | 'Logistics'
        | 'Marketing'
        | 'Website' =
        (initial as
            | 'Sales'
            | 'Accounting'
            | 'Errand'
            | 'HumanResources'
            | 'Logistics'
            | 'Marketing'
            | 'Website') ?? 'Sales';
    let currentVariant = $state<
        'Sales' | 'Accounting' | 'Errand' | 'HumanResources' | 'Logistics' | 'Marketing' | 'Website'
    >(initialVariant);
    let data = $state<Applications>(initial ?? getDefaultForVariantApplications(initialVariant));
    let errors = $state<ErrorsApplications>({} as ErrorsApplications);
    let tainted = $state<TaintedApplications>({} as TaintedApplications);
    const variants: VariantFieldsApplications = {
        Sales: {
            fields: {} as SalesFieldControllersApplications
        },
        Accounting: {
            fields: {} as AccountingFieldControllersApplications
        },
        Errand: {
            fields: {} as ErrandFieldControllersApplications
        },
        HumanResources: {
            fields: {} as HumanResourcesFieldControllersApplications
        },
        Logistics: {
            fields: {} as LogisticsFieldControllersApplications
        },
        Marketing: {
            fields: {} as MarketingFieldControllersApplications
        },
        Website: {
            fields: {} as WebsiteFieldControllersApplications
        }
    };
    function switchVariant(
        variant:
            | 'Sales'
            | 'Accounting'
            | 'Errand'
            | 'HumanResources'
            | 'Logistics'
            | 'Marketing'
            | 'Website'
    ): void {
        currentVariant = variant;
        data = getDefaultForVariantApplications(variant);
        errors = {} as ErrorsApplications;
        tainted = {} as TaintedApplications;
    }
    function validate(): Result<Applications, Array<{ field: string; message: string }>> {
        return fromObjectApplications(data);
    }
    function reset(overrides?: Partial<Applications>): void {
        data = overrides
            ? (overrides as typeof data)
            : getDefaultForVariantApplications(currentVariant);
        errors = {} as ErrorsApplications;
        tainted = {} as TaintedApplications;
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
export function fromFormDataApplications(
    formData: FormData
): Result<Applications, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_value') as
        | 'Sales'
        | 'Accounting'
        | 'Errand'
        | 'HumanResources'
        | 'Logistics'
        | 'Marketing'
        | 'Website'
        | null;
    if (!discriminant) {
        return Result.err([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'Sales') {
    } else if (discriminant === 'Accounting') {
    } else if (discriminant === 'Errand') {
    } else if (discriminant === 'HumanResources') {
    } else if (discriminant === 'Logistics') {
    } else if (discriminant === 'Marketing') {
    } else if (discriminant === 'Website') {
    }
    return fromStringifiedJSONApplications(JSON.stringify(obj));
}
