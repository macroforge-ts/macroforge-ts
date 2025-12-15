import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';

export type JobTitle =
    | /** @default */ 'Technician'
    | 'SalesRepresentative'
    | 'HumanResources'
    | 'InformationTechnology';

export function defaultValueJobTitle(): JobTitle {
    return 'Technician';
}

export function toStringifiedJSONJobTitle(value: JobTitle): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeJobTitle(value, ctx));
}
export function toObjectJobTitle(value: JobTitle): unknown {
    const ctx = SerializeContext.create();
    return __serializeJobTitle(value, ctx);
}
export function __serializeJobTitle(value: JobTitle, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONJobTitle(
    json: string,
    opts?: DeserializeOptions
): Result<JobTitle, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectJobTitle(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectJobTitle(
    obj: unknown,
    opts?: DeserializeOptions
): Result<JobTitle, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeJobTitle(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'JobTitle.fromObject: root cannot be a forward reference'
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
export function __deserializeJobTitle(value: any, ctx: DeserializeContext): JobTitle | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as JobTitle | PendingRef;
    }
    const allowedValues = [
        'Technician',
        'SalesRepresentative',
        'HumanResources',
        'InformationTechnology'
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new DeserializeError([
            {
                field: '_root',
                message:
                    'Invalid value for JobTitle: expected one of ' +
                    allowedValues.map((v) => JSON.stringify(v)).join(', ') +
                    ', got ' +
                    JSON.stringify(value)
            }
        ]);
    }
    return value as JobTitle;
}
export function isJobTitle(value: unknown): value is JobTitle {
    const allowedValues = [
        'Technician',
        'SalesRepresentative',
        'HumanResources',
        'InformationTechnology'
    ] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type TechnicianErrorsJobTitle = {
    _errors: Option<Array<string>>;
};
export type SalesRepresentativeErrorsJobTitle = { _errors: Option<Array<string>> };
export type HumanResourcesErrorsJobTitle = { _errors: Option<Array<string>> };
export type InformationTechnologyErrorsJobTitle = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type TechnicianTaintedJobTitle = {};
export type SalesRepresentativeTaintedJobTitle = {};
export type HumanResourcesTaintedJobTitle = {};
export type InformationTechnologyTaintedJobTitle = {}; /** Union error type */
export type ErrorsJobTitle =
    | ({ _value: 'Technician' } & TechnicianErrorsJobTitle)
    | ({ _value: 'SalesRepresentative' } & SalesRepresentativeErrorsJobTitle)
    | ({ _value: 'HumanResources' } & HumanResourcesErrorsJobTitle)
    | ({
          _value: 'InformationTechnology';
      } & InformationTechnologyErrorsJobTitle); /** Union tainted type */
export type TaintedJobTitle =
    | ({ _value: 'Technician' } & TechnicianTaintedJobTitle)
    | ({ _value: 'SalesRepresentative' } & SalesRepresentativeTaintedJobTitle)
    | ({ _value: 'HumanResources' } & HumanResourcesTaintedJobTitle)
    | ({
          _value: 'InformationTechnology';
      } & InformationTechnologyTaintedJobTitle); /** Per-variant field controller types */
export interface TechnicianFieldControllersJobTitle {}
export interface SalesRepresentativeFieldControllersJobTitle {}
export interface HumanResourcesFieldControllersJobTitle {}
export interface InformationTechnologyFieldControllersJobTitle {} /** Union Gigaform interface with variant switching */
export interface GigaformJobTitle {
    readonly currentVariant:
        | 'Technician'
        | 'SalesRepresentative'
        | 'HumanResources'
        | 'InformationTechnology';
    readonly data: JobTitle;
    readonly errors: ErrorsJobTitle;
    readonly tainted: TaintedJobTitle;
    readonly variants: VariantFieldsJobTitle;
    switchVariant(
        variant: 'Technician' | 'SalesRepresentative' | 'HumanResources' | 'InformationTechnology'
    ): void;
    validate(): Result<JobTitle, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<JobTitle>): void;
} /** Variant fields container */
export interface VariantFieldsJobTitle {
    readonly Technician: { readonly fields: TechnicianFieldControllersJobTitle };
    readonly SalesRepresentative: { readonly fields: SalesRepresentativeFieldControllersJobTitle };
    readonly HumanResources: { readonly fields: HumanResourcesFieldControllersJobTitle };
    readonly InformationTechnology: {
        readonly fields: InformationTechnologyFieldControllersJobTitle;
    };
} /** Gets default value for a specific variant */
function getDefaultForVariantJobTitle(variant: string): JobTitle {
    switch (variant) {
        case 'Technician':
            return 'Technician' as JobTitle;
        case 'SalesRepresentative':
            return 'SalesRepresentative' as JobTitle;
        case 'HumanResources':
            return 'HumanResources' as JobTitle;
        case 'InformationTechnology':
            return 'InformationTechnology' as JobTitle;
        default:
            return 'Technician' as JobTitle;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function createFormJobTitle(initial?: JobTitle): GigaformJobTitle {
    const initialVariant:
        | 'Technician'
        | 'SalesRepresentative'
        | 'HumanResources'
        | 'InformationTechnology' =
        (initial as
            | 'Technician'
            | 'SalesRepresentative'
            | 'HumanResources'
            | 'InformationTechnology') ?? 'Technician';
    let currentVariant = $state<
        'Technician' | 'SalesRepresentative' | 'HumanResources' | 'InformationTechnology'
    >(initialVariant);
    let data = $state<JobTitle>(initial ?? getDefaultForVariantJobTitle(initialVariant));
    let errors = $state<ErrorsJobTitle>({} as ErrorsJobTitle);
    let tainted = $state<TaintedJobTitle>({} as TaintedJobTitle);
    const variants: VariantFieldsJobTitle = {
        Technician: {
            fields: {} as TechnicianFieldControllersJobTitle
        },
        SalesRepresentative: {
            fields: {} as SalesRepresentativeFieldControllersJobTitle
        },
        HumanResources: {
            fields: {} as HumanResourcesFieldControllersJobTitle
        },
        InformationTechnology: {
            fields: {} as InformationTechnologyFieldControllersJobTitle
        }
    };
    function switchVariant(
        variant: 'Technician' | 'SalesRepresentative' | 'HumanResources' | 'InformationTechnology'
    ): void {
        currentVariant = variant;
        data = getDefaultForVariantJobTitle(variant);
        errors = {} as ErrorsJobTitle;
        tainted = {} as TaintedJobTitle;
    }
    function validate(): Result<JobTitle, Array<{ field: string; message: string }>> {
        return fromObjectJobTitle(data);
    }
    function reset(overrides?: Partial<JobTitle>): void {
        data = overrides
            ? (overrides as typeof data)
            : getDefaultForVariantJobTitle(currentVariant);
        errors = {} as ErrorsJobTitle;
        tainted = {} as TaintedJobTitle;
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
export function fromFormDataJobTitle(
    formData: FormData
): Result<JobTitle, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_value') as
        | 'Technician'
        | 'SalesRepresentative'
        | 'HumanResources'
        | 'InformationTechnology'
        | null;
    if (!discriminant) {
        return Result.err([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'Technician') {
    } else if (discriminant === 'SalesRepresentative') {
    } else if (discriminant === 'HumanResources') {
    } else if (discriminant === 'InformationTechnology') {
    }
    return fromStringifiedJSONJobTitle(JSON.stringify(obj));
}
